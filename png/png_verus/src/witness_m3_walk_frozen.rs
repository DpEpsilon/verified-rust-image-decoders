//! Whole-walk/public-entry error witnesses from authored frozen fixtures.
//! Exact errors come from frozen records, not an executable oracle.
use vstd::prelude::*;
use crate::{document_model as m,png::Error,alloc_guard::sites::TryError};

verus! {


// Per-record sequence prefix, with each model step checked separately.
spec fn first_sequence_state(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53,seen:s.seen.insert(crate::walk_model::Name::Actl),doc:m::Doc {actl:Some((2u32,0u32)),..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn first_sequence_actl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires 91<=b.len(),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32)),
    ensures m::step(b,m::strict(),true,false,m::initial(h,33,m::no_events()))==m::Step::Next(first_sequence_state(h)),
{
    hide(crate::critical_model::read);hide(crate::apng_model::actl);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn first_sequence_fctl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires 91<=b.len(),h.width==4,h.height==3,
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:61,len:26,next:91,crc_ok:true,synthetic:false}),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4,3,crate::apng_model::initial())==Err((crate::png::Error::SequenceError,true)),
    ensures m::step(b,m::strict(),true,false,first_sequence_state(h))==m::Step::Error(Error::SequenceError,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::apng_model::fctl);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn first_control_sequence_error(b: Seq<u8>,h: crate::model::Ihdr)
    requires 91<=b.len(),h.width==4,h.height==3,crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:61,len:26,next:91,crc_ok:true,synthetic:false}),
        crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4,3,crate::apng_model::initial())==Err((crate::png::Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::SequenceError,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(m::drive);hide(m::initial_info);
    first_sequence_actl_step(b,h);first_sequence_fctl_step(b,h);
    m::drive_step(b,m::strict(),true,false,first_sequence_state(h));
    m::drive_step(b,m::strict(),true,false,m::initial(h,33,m::no_events()));
}
#[verifier::spinoff_prover]
proof fn invalid_iccp_keyword(d: Seq<u8>,ct: u8,depth: u8,z: nat)
    requires crate::model::valid_depth(ct,depth),crate::model::first_nul(d,0)==Some(z),
        !crate::model::keyword(d.subrange(0,z as int)),
    ensures crate::metadata_model::parse(crate::walk_model::Name::Iccp,d,ct,depth,0)==crate::metadata_model::Outcome::Error(Error::InvalidIccp),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn first_iccp_error(b: Seq<u8>,h: crate::model::Ihdr,length: nat)
    requires 41+length<=b.len(),crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![105u8,67u8,67u8,80u8],data:41,len:length,next:45+length,crc_ok:true,synthetic:false}),
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(41,(41+length) as int),h.color_type,h.depth,0)==crate::metadata_model::Outcome::Error(Error::InvalidIccp),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::InvalidIccp,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::metadata_model::parse);hide(m::drive);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let s=m::initial(h,33,m::no_events());
    assert(m::step(b,m::strict(),false,false,s)==m::Step::Error(Error::InvalidIccp,m::no_events()));
    m::drive_step(b,m::strict(),false,false,s);
}
#[verifier::spinoff_prover]
proof fn idat_then_iccp_order(b: Seq<u8>,h: crate::model::Ihdr,idat_len: nat,iccp_len: nat)
    requires 57+idat_len+iccp_len<=b.len(),h.color_type!=3,crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:41,len:idat_len,next:45+idat_len,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,45+idat_len,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![105u8,67u8,67u8,80u8],data:53+idat_len,len:iccp_len,next:57+idat_len+iccp_len,crc_ok:true,synthetic:false}),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);
    let s0=m::initial(h,33,m::no_events());
    let s1=m::State {pos:45+idat_len,phase:crate::walk_model::Phase::During,
        control:crate::apng_model::begin_idat(s0.control),
        doc:m::Doc {idat:s0.doc.idat.push(crate::walk_model::Range {offset:41,len:idat_len}),..s0.doc},..s0};
    assert(m::step(b,m::strict(),false,false,s0)==m::Step::Next(s1)) by {
        reveal(m::step);reveal(m::ordered);reveal(m::own);
    }
    assert(m::step(b,m::strict(),false,false,s1)==m::Step::Error(Error::ChunkOutOfOrder,m::no_events())) by {
        reveal(m::step);reveal(m::ordered);reveal(m::own);
    }
    m::drive_step(b,m::strict(),false,false,s1);
    m::drive_step(b,m::strict(),false,false,s0);
}

// This proof never expands the large palette payload; the caller checks its wire.
#[verifier::spinoff_prover]
proof fn first_palette_error(b: Seq<u8>,h: crate::model::Ihdr,length: nat,error: Error)
    requires 41+length<=b.len(),crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![80u8,76u8,84u8,69u8],data:41,len:length,next:45+length,crc_ok:true,synthetic:false}),
        crate::metadata_model::palette(length,h.color_type,h.depth,false,false,false,false)==crate::metadata_model::Decision::Error(error),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(error,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::metadata_model::palette);hide(m::drive);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let s=m::initial(h,33,m::no_events());
    assert(m::step(b,m::strict(),false,false,s)==m::Step::Error(error,m::no_events()));
    m::drive_step(b,m::strict(),false,false,s);
}
// Symbolic prefix states prevent the fixture's byte array entering transition solving.
spec fn zero_s0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn zero_s1(h: crate::model::Ihdr) -> m::State {
    let s=zero_s0(h);
    m::State {pos:53,seen:s.seen.insert(crate::walk_model::Name::Actl),
        doc:m::Doc {actl:Some((2u32,0u32)),..s.doc},..s}
}
spec fn zero_s2(h: crate::model::Ihdr) -> m::State {
    let s=zero_s1(h);
    m::State {pos:88,phase:crate::walk_model::Phase::During,control:crate::apng_model::begin_idat(s.control),
        doc:m::Doc {idat:s.doc.idat.push(crate::walk_model::Range {offset:61,len:23}),..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn zero_actl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(41,49),0)==2,crate::model::be32(b.subrange(41,49),4)==0,
    ensures m::step(b,m::strict(),true,false,zero_s0(h))==m::Step::Next(zero_s1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::be32);hide(m::initial_info);
    assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn zero_idat_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,h.color_type==6,
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:61,len:23,next:88,crc_ok:true,synthetic:false}),
    ensures m::step(b,m::strict(),true,false,zero_s1(h))==m::Step::Next(zero_s2(h)),
{
    hide(crate::critical_model::read);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn zero_fctl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,
        crate::critical_model::read(b,88,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:96,len:26,next:126,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(96,122),0)==0,crate::model::be32(b.subrange(96,122),4)==0,
    ensures m::step(b,m::strict(),true,false,zero_s2(h))==m::Step::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::be32);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn late_zero_width_control(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,h.color_type==6,crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:61,len:23,next:88,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,88,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:96,len:26,next:126,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(41,49),0)==2,crate::model::be32(b.subrange(41,49),4)==0,
        crate::model::be32(b.subrange(96,122),0)==0,crate::model::be32(b.subrange(96,122),4)==0,
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::model::be32);hide(m::drive);hide(m::initial_info);
    zero_actl_step(b,h);zero_idat_step(b,h);zero_fctl_step(b,h);
    m::drive_step(b,m::strict(),true,false,zero_s2(h));
    m::drive_step(b,m::strict(),true,false,zero_s1(h));
    m::drive_step(b,m::strict(),true,false,zero_s0(h));
}

// container/idat-crc; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_idat_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_idat_crc()
    ensures m::MW_p(bytes_container_idat_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_idat_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_idat_crc(),16),height:crate::model::be32(bytes_container_idat_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_idat_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_idat_crc(),16),height:crate::model::be32(bytes_container_idat_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_idat_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:false,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_idat_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:false,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_idat_crc(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_idat_crc(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_idat_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
}
fn walk_container_idat_crc() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_idat_crc());model_container_idat_crc();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::CrcMismatch))));
}

// container/ancillary-crc; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_ancillary_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,7u8,116u8,69u8,88u8,116u8,84u8,105u8,116u8,108u8,101u8,0u8,120u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_ancillary_crc()
    ensures m::MW_p(bytes_container_ancillary_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_ancillary_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_ancillary_crc(),16),height:crate::model::be32(bytes_container_ancillary_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_ancillary_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_ancillary_crc(),16),height:crate::model::be32(bytes_container_ancillary_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_ancillary_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:7nat,next:52nat,crc_ok:false,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_ancillary_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:7nat,next:52nat,crc_ok:false,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_container_ancillary_crc().subrange(41,48),0)==Some(5nat)) by (compute);
        assert(bytes_container_ancillary_crc().subrange(41,48).subrange(0,5)==seq![84u8,105u8,116u8,108u8,101u8]) by (compute);
        assert(crate::model::keyword(seq![84u8,105u8,116u8,108u8,101u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_ancillary_crc(),52nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:60nat,len:22nat,next:86nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_ancillary_crc(),52nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:60nat,len:22nat,next:86nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_ancillary_crc(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_ancillary_crc(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_container_ancillary_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
}
fn walk_container_ancillary_crc() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,7u8,116u8,69u8,88u8,116u8,84u8,105u8,116u8,108u8,101u8,0u8,120u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_ancillary_crc());model_container_ancillary_crc();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::CrcMismatch))));
}

// container/unknown-ancillary-crc; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_unknown_ancillary_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,112u8,114u8,86u8,116u8,97u8,98u8,99u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_unknown_ancillary_crc()
    ensures m::MW_p(bytes_container_unknown_ancillary_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_unknown_ancillary_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_unknown_ancillary_crc(),16),height:crate::model::be32(bytes_container_unknown_ancillary_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_unknown_ancillary_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_unknown_ancillary_crc(),16),height:crate::model::be32(bytes_container_unknown_ancillary_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![112u8,114u8,86u8,116u8],data:41nat,len:3nat,next:48nat,crc_ok:false,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![112u8,114u8,86u8,116u8],data:41nat,len:3nat,next:48nat,crc_ok:false,synthetic:false})) by (compute);
        }
        assert((112u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![112u8,114u8,86u8,116u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![112u8,114u8,86u8,116u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_crc(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_crc(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_crc(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_crc(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_container_unknown_ancillary_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
}
fn walk_container_unknown_ancillary_crc() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,112u8,114u8,86u8,116u8,97u8,98u8,99u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_unknown_ancillary_crc());model_container_unknown_ancillary_crc();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::CrcMismatch))));
}

// container/iend-crc; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_iend_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,0u8,0u8,0u8,0u8]}
proof fn model_container_iend_crc()
    ensures m::MW_p(bytes_container_iend_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_iend_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_iend_crc(),16),height:crate::model::be32(bytes_container_iend_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_iend_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_iend_crc(),16),height:crate::model::be32(bytes_container_iend_crc(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_iend_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_iend_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_iend_crc(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:false,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_iend_crc(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:false,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_iend_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
}
fn walk_container_iend_crc() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,0u8,0u8,0u8,0u8];
    proof {assert(b@==bytes_container_iend_crc());model_container_iend_crc();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::CrcMismatch))));
}

// container/length-2_31; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_length_2_31() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,128u8,0u8,0u8,0u8,116u8,69u8,88u8,116u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
proof fn model_container_length_2_31()
    ensures m::MW_p(bytes_container_length_2_31(),m::strict())==m::Outcome::Error(Error::ChunkTooLong,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_length_2_31())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_2_31(),16),height:crate::model::be32(bytes_container_length_2_31(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_length_2_31())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_2_31(),16),height:crate::model::be32(bytes_container_length_2_31(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_length_2_31(),33nat,phase,false)==Err(Error::ChunkTooLong) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_length_2_31(),33nat,phase,false)==Err(Error::ChunkTooLong)) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_length_2_31(),m::strict())==m::Outcome::Error(Error::ChunkTooLong,m::no_events()));
}
fn walk_container_length_2_31() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,128u8,0u8,0u8,0u8,116u8,69u8,88u8,116u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
    proof {assert(b@==bytes_container_length_2_31());model_container_length_2_31();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkTooLong))));
}

// container/length-ffffffff; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_length_ffffffff() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,255u8,255u8,255u8,255u8,73u8,68u8,65u8,84u8]}
proof fn model_container_length_ffffffff()
    ensures m::MW_p(bytes_container_length_ffffffff(),m::strict())==m::Outcome::Error(Error::ChunkTooLong,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_length_ffffffff())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_ffffffff(),16),height:crate::model::be32(bytes_container_length_ffffffff(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_length_ffffffff())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_ffffffff(),16),height:crate::model::be32(bytes_container_length_ffffffff(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_length_ffffffff(),33nat,phase,false)==Err(Error::ChunkTooLong) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_length_ffffffff(),33nat,phase,false)==Err(Error::ChunkTooLong)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_length_ffffffff(),m::strict())==m::Outcome::Error(Error::ChunkTooLong,m::no_events()));
}
fn walk_container_length_ffffffff() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,255u8,255u8,255u8,255u8,73u8,68u8,65u8,84u8];
    proof {assert(b@==bytes_container_length_ffffffff());model_container_length_ffffffff();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkTooLong))));
}

// container/type-digit; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_type_digit() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,116u8,101u8,49u8,116u8,26u8,48u8,83u8,139u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_type_digit()
    ensures m::MW_p(bytes_container_type_digit(),m::strict())==m::Outcome::Error(Error::InvalidChunkType,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_type_digit())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_type_digit(),16),height:crate::model::be32(bytes_container_type_digit(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_type_digit())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_type_digit(),16),height:crate::model::be32(bytes_container_type_digit(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_type_digit(),33nat,phase,false)==Err(Error::InvalidChunkType) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_type_digit(),33nat,phase,false)==Err(Error::InvalidChunkType)) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,101u8,49u8,116u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,101u8,49u8,116u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_type_digit(),m::strict())==m::Outcome::Error(Error::InvalidChunkType,m::no_events()));
}
fn walk_container_type_digit() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,116u8,101u8,49u8,116u8,26u8,48u8,83u8,139u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_type_digit());model_container_type_digit();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidChunkType))));
}

// container/type-space; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_type_space() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,68u8,65u8,32u8,98u8,199u8,179u8,59u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_type_space()
    ensures m::MW_p(bytes_container_type_space(),m::strict())==m::Outcome::Error(Error::InvalidChunkType,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_type_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_type_space(),16),height:crate::model::be32(bytes_container_type_space(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_type_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_type_space(),16),height:crate::model::be32(bytes_container_type_space(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_type_space(),33nat,phase,false)==Err(Error::InvalidChunkType) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_type_space(),33nat,phase,false)==Err(Error::InvalidChunkType)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,32u8])==crate::walk_model::Name::Unknown(false)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,32u8])==crate::walk_model::Name::Unknown(false)) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_type_space(),m::strict())==m::Outcome::Error(Error::InvalidChunkType,m::no_events()));
}
fn walk_container_type_space() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,68u8,65u8,32u8,98u8,199u8,179u8,59u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_type_space());model_container_type_space();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidChunkType))));
}

// container/truncated-mid-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_truncated_mid_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8]}
proof fn model_container_truncated_mid_idat()
    ensures m::MW_p(bytes_container_truncated_mid_idat(),m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_truncated_mid_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_truncated_mid_idat(),16),height:crate::model::be32(bytes_container_truncated_mid_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_truncated_mid_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_truncated_mid_idat(),16),height:crate::model::be32(bytes_container_truncated_mid_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_truncated_mid_idat(),33nat,phase,false)==Err(Error::TruncatedChunk) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_truncated_mid_idat(),33nat,phase,false)==Err(Error::TruncatedChunk)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_truncated_mid_idat(),m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()));
}
fn walk_container_truncated_mid_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8];
    proof {assert(b@==bytes_container_truncated_mid_idat());model_container_truncated_mid_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TruncatedChunk))));
}

// container/length-beyond-eof; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_length_beyond_eof() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,3u8,232u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8]}
proof fn model_container_length_beyond_eof()
    ensures m::MW_p(bytes_container_length_beyond_eof(),m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_length_beyond_eof())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_beyond_eof(),16),height:crate::model::be32(bytes_container_length_beyond_eof(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_length_beyond_eof())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_length_beyond_eof(),16),height:crate::model::be32(bytes_container_length_beyond_eof(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_length_beyond_eof(),33nat,phase,false)==Err(Error::TruncatedChunk) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_length_beyond_eof(),33nat,phase,false)==Err(Error::TruncatedChunk)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_length_beyond_eof(),m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()));
}
fn walk_container_length_beyond_eof() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,3u8,232u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8];
    proof {assert(b@==bytes_container_length_beyond_eof());model_container_length_beyond_eof();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TruncatedChunk))));
}

// container/missing-iend; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_missing_iend() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8]}
proof fn model_container_missing_iend()
    ensures m::MW_p(bytes_container_missing_iend(),m::strict())==m::Outcome::Error(Error::MissingIend,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_missing_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_missing_iend(),16),height:crate::model::be32(bytes_container_missing_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_missing_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_missing_iend(),16),height:crate::model::be32(bytes_container_missing_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_missing_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_missing_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_missing_iend(),67nat,phase,false)==Err(Error::MissingIend) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_missing_iend(),67nat,phase,false)==Err(Error::MissingIend)) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_missing_iend(),m::strict())==m::Outcome::Error(Error::MissingIend,m::no_events()));
}
fn walk_container_missing_iend() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8];
    proof {assert(b@==bytes_container_missing_iend());model_container_missing_iend();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingIend))));
}

// container/missing-iend-after-text; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_missing_iend_after_text() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,3u8,116u8,69u8,88u8,116u8,97u8,0u8,98u8,220u8,73u8,162u8,59u8]}
proof fn model_container_missing_iend_after_text()
    ensures m::MW_p(bytes_container_missing_iend_after_text(),m::strict())==m::Outcome::Error(Error::MissingIend,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_missing_iend_after_text())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_missing_iend_after_text(),16),height:crate::model::be32(bytes_container_missing_iend_after_text(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_missing_iend_after_text())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_missing_iend_after_text(),16),height:crate::model::be32(bytes_container_missing_iend_after_text(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_missing_iend_after_text(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_missing_iend_after_text(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_missing_iend_after_text(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:75nat,len:3nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_missing_iend_after_text(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:75nat,len:3nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_container_missing_iend_after_text().subrange(75,78),0)==Some(1nat)) by (compute);
        assert(bytes_container_missing_iend_after_text().subrange(75,78).subrange(0,1)==seq![97u8]) by (compute);
        assert(crate::model::keyword(seq![97u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_missing_iend_after_text(),82nat,phase,false)==Err(Error::MissingIend) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_missing_iend_after_text(),82nat,phase,false)==Err(Error::MissingIend)) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_container_missing_iend_after_text(),m::strict())==m::Outcome::Error(Error::MissingIend,m::no_events()));
}
fn walk_container_missing_iend_after_text() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,3u8,116u8,69u8,88u8,116u8,97u8,0u8,98u8,220u8,73u8,162u8,59u8];
    proof {assert(b@==bytes_container_missing_iend_after_text());model_container_missing_iend_after_text();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingIend))));
}

// container/trailing-byte; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_trailing_byte() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8]}
proof fn model_container_trailing_byte()
    ensures m::MW_p(bytes_container_trailing_byte(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_trailing_byte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_trailing_byte(),16),height:crate::model::be32(bytes_container_trailing_byte(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_trailing_byte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_trailing_byte(),16),height:crate::model::be32(bytes_container_trailing_byte(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_trailing_byte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_trailing_byte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_trailing_byte(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_trailing_byte(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_trailing_byte(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()));
}
fn walk_container_trailing_byte() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8];
    proof {assert(b@==bytes_container_trailing_byte());model_container_trailing_byte();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TrailingBytes))));
}

// container/trailing-second-iend; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_trailing_second_iend() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_trailing_second_iend()
    ensures m::MW_p(bytes_container_trailing_second_iend(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_trailing_second_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_trailing_second_iend(),16),height:crate::model::be32(bytes_container_trailing_second_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_trailing_second_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_trailing_second_iend(),16),height:crate::model::be32(bytes_container_trailing_second_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_trailing_second_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_trailing_second_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_trailing_second_iend(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_trailing_second_iend(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_trailing_second_iend(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()));
}
fn walk_container_trailing_second_iend() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_trailing_second_iend());model_container_trailing_second_iend();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TrailingBytes))));
}

// container/iend-length-1; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_iend_length_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,1u8,73u8,69u8,78u8,68u8,0u8,209u8,26u8,79u8,225u8]}
proof fn model_container_iend_length_1()
    ensures m::MW_p(bytes_container_iend_length_1(),m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_iend_length_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_iend_length_1(),16),height:crate::model::be32(bytes_container_iend_length_1(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_iend_length_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_iend_length_1(),16),height:crate::model::be32(bytes_container_iend_length_1(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_iend_length_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_iend_length_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_iend_length_1(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:1nat,next:80nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_iend_length_1(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:1nat,next:80nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_iend_length_1(),m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()));
}
fn walk_container_iend_length_1() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,1u8,73u8,69u8,78u8,68u8,0u8,209u8,26u8,79u8,225u8];
    proof {assert(b@==bytes_container_iend_length_1());model_container_iend_length_1();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidChunkLength))));
}

// container/no-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_no_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_no_idat()
    ensures m::MW_p(bytes_container_no_idat(),m::strict())==m::Outcome::Error(Error::MissingImageData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_no_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_no_idat(),16),height:crate::model::be32(bytes_container_no_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_no_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_no_idat(),16),height:crate::model::be32(bytes_container_no_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_no_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_no_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,2);
        assert(m::MW_p(bytes_container_no_idat(),m::strict())==m::Outcome::Error(Error::MissingImageData,m::no_events()));
}
fn walk_container_no_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_no_idat());model_container_no_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingImageData))));
}

// container/unknown-critical; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_unknown_critical() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,65u8,66u8,67u8,68u8,219u8,23u8,32u8,165u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_unknown_critical()
    ensures m::MW_p(bytes_container_unknown_critical(),m::strict())==m::Outcome::Error(Error::UnknownCriticalChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_unknown_critical())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_unknown_critical(),16),height:crate::model::be32(bytes_container_unknown_critical(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_unknown_critical())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_unknown_critical(),16),height:crate::model::be32(bytes_container_unknown_critical(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_critical(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![65u8,66u8,67u8,68u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_critical(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![65u8,66u8,67u8,68u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((65u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![65u8,66u8,67u8,68u8])==crate::walk_model::Name::Unknown(false)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![65u8,66u8,67u8,68u8])==crate::walk_model::Name::Unknown(false)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_critical(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:22nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_critical(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:22nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_critical(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_critical(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_container_unknown_critical(),m::strict())==m::Outcome::Error(Error::UnknownCriticalChunk,m::no_events()));
}
fn walk_container_unknown_critical() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,65u8,66u8,67u8,68u8,219u8,23u8,32u8,165u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_unknown_critical());model_container_unknown_critical();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnknownCriticalChunk))));
}

// container/reserved-bit-on-critical; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_reserved_bit_on_critical() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,68u8,97u8,84u8,160u8,43u8,34u8,188u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_reserved_bit_on_critical()
    ensures m::MW_p(bytes_container_reserved_bit_on_critical(),m::strict())==m::Outcome::Error(Error::UnknownCriticalChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_reserved_bit_on_critical())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_reserved_bit_on_critical(),16),height:crate::model::be32(bytes_container_reserved_bit_on_critical(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_reserved_bit_on_critical())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_reserved_bit_on_critical(),16),height:crate::model::be32(bytes_container_reserved_bit_on_critical(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_on_critical(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,97u8,84u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_on_critical(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,97u8,84u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,97u8,84u8])==crate::walk_model::Name::Unknown(false)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,97u8,84u8])==crate::walk_model::Name::Unknown(false)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_on_critical(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:22nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_on_critical(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:22nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_on_critical(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_on_critical(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_container_reserved_bit_on_critical(),m::strict())==m::Outcome::Error(Error::UnknownCriticalChunk,m::no_events()));
}
fn walk_container_reserved_bit_on_critical() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,0u8,73u8,68u8,97u8,84u8,160u8,43u8,34u8,188u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_reserved_bit_on_critical());model_container_reserved_bit_on_critical();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnknownCriticalChunk))));
}

// container/duplicate-ihdr; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_duplicate_ihdr() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_container_duplicate_ihdr()
    ensures m::MW_p(bytes_container_duplicate_ihdr(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_duplicate_ihdr())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_duplicate_ihdr(),16),height:crate::model::be32(bytes_container_duplicate_ihdr(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_duplicate_ihdr())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_duplicate_ihdr(),16),height:crate::model::be32(bytes_container_duplicate_ihdr(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_duplicate_ihdr(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,72u8,68u8,82u8],data:41nat,len:13nat,next:58nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_duplicate_ihdr(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,72u8,68u8,82u8],data:41nat,len:13nat,next:58nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,72u8,68u8,82u8])==crate::walk_model::Name::Ihdr) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,72u8,68u8,82u8])==crate::walk_model::Name::Ihdr) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_duplicate_ihdr(),58nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:66nat,len:22nat,next:92nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_duplicate_ihdr(),58nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:66nat,len:22nat,next:92nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_duplicate_ihdr(),92nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:100nat,len:0nat,next:104nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_duplicate_ihdr(),92nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:100nat,len:0nat,next:104nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        let ghost s0=m::initial(crate::model::mh(bytes_container_duplicate_ihdr()).unwrap().0,33,m::no_events());
        assert(m::step(bytes_container_duplicate_ihdr(),m::strict(),false,false,s0)==m::Step::Error(Error::DuplicateChunk,m::no_events()));
        m::drive_step(bytes_container_duplicate_ihdr(),m::strict(),false,false,s0);
        assert(m::drive(bytes_container_duplicate_ihdr(),m::strict(),false,false,s0)==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
        assert(m::MW_p(bytes_container_duplicate_ihdr(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
}
fn walk_container_duplicate_ihdr() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_duplicate_ihdr());model_container_duplicate_ihdr();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::DuplicateChunk))));
}

// container/non-consecutive-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_non_consecutive_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,5u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,246u8,96u8,63u8,133u8,0u8,0u8,0u8,3u8,116u8,69u8,88u8,116u8,97u8,0u8,98u8,220u8,73u8,162u8,59u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,167u8,128u8,111u8,134u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_container_non_consecutive_idat()
    ensures m::MW_p(bytes_container_non_consecutive_idat(),m::strict())==m::Outcome::Error(Error::NonConsecutiveIdat,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_non_consecutive_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_non_consecutive_idat(),16),height:crate::model::be32(bytes_container_non_consecutive_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_non_consecutive_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_non_consecutive_idat(),16),height:crate::model::be32(bytes_container_non_consecutive_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_non_consecutive_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:5nat,next:50nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_non_consecutive_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:5nat,next:50nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_non_consecutive_idat(),50nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:58nat,len:3nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_non_consecutive_idat(),50nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:58nat,len:3nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_container_non_consecutive_idat().subrange(58,61),0)==Some(1nat)) by (compute);
        assert(bytes_container_non_consecutive_idat().subrange(58,61).subrange(0,1)==seq![97u8]) by (compute);
        assert(crate::model::keyword(seq![97u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_non_consecutive_idat(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:17nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_non_consecutive_idat(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:17nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_non_consecutive_idat(),94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_non_consecutive_idat(),94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_container_non_consecutive_idat(),m::strict())==m::Outcome::Error(Error::NonConsecutiveIdat,m::no_events()));
}
fn walk_container_non_consecutive_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,5u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,246u8,96u8,63u8,133u8,0u8,0u8,0u8,3u8,116u8,69u8,88u8,116u8,97u8,0u8,98u8,220u8,73u8,162u8,59u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,167u8,128u8,111u8,134u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_non_consecutive_idat());model_container_non_consecutive_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::NonConsecutiveIdat))));
}

// container/idat-after-iend; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_idat_after_iend() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8]}
proof fn model_container_idat_after_iend()
    ensures m::MW_p(bytes_container_idat_after_iend(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_container_idat_after_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_idat_after_iend(),16),height:crate::model::be32(bytes_container_idat_after_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_container_idat_after_iend())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_container_idat_after_iend(),16),height:crate::model::be32(bytes_container_idat_after_iend(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_idat_after_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_idat_after_iend(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_idat_after_iend(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_idat_after_iend(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_container_idat_after_iend(),m::strict())==m::Outcome::Error(Error::TrailingBytes,m::no_events()));
}
fn walk_container_idat_after_iend() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8];
    proof {assert(b@==bytes_container_idat_after_iend());model_container_idat_after_iend();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TrailingBytes))));
}

// plte/missing; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_missing() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_missing()
    ensures m::MW_p(bytes_plte_missing(),m::strict())==m::Outcome::Error(Error::MissingPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_missing())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_missing(),16),height:crate::model::be32(bytes_plte_missing(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_missing())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_missing(),16),height:crate::model::be32(bytes_plte_missing(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_missing(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_missing(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_missing(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:67nat,len:0nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_missing(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:67nat,len:0nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,3);
        assert(m::MW_p(bytes_plte_missing(),m::strict())==m::Outcome::Error(Error::MissingPalette,m::no_events()));
}
fn walk_plte_missing() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_missing());model_plte_missing();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingPalette))));
}

// plte/after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_after_idat()
    ensures m::MW_p(bytes_plte_after_idat(),m::strict())==m::Outcome::Error(Error::MissingPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_after_idat(),16),height:crate::model::be32(bytes_plte_after_idat(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_after_idat(),16),height:crate::model::be32(bytes_plte_after_idat(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:67nat,len:9nat,next:80nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:67nat,len:9nat,next:80nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,true,false,false,false)==crate::metadata_model::Decision::Error(Error::ChunkOutOfOrder)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_after_idat(),m::strict())==m::Outcome::Error(Error::MissingPalette,m::no_events()));
}
fn walk_plte_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_after_idat());model_plte_after_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingPalette))));
}

// plte/after-idat-rgb; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_after_idat_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_after_idat_rgb()
    ensures m::MW_p(bytes_plte_after_idat_rgb(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_after_idat_rgb())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_after_idat_rgb(),16),height:crate::model::be32(bytes_plte_after_idat_rgb(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_after_idat_rgb())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_after_idat_rgb(),16),height:crate::model::be32(bytes_plte_after_idat_rgb(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat_rgb(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:75nat,len:9nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat_rgb(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:75nat,len:9nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,true,false,false,false)==crate::metadata_model::Decision::Error(Error::ChunkOutOfOrder)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_after_idat_rgb(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_after_idat_rgb(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_after_idat_rgb(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_plte_after_idat_rgb() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_after_idat_rgb());model_plte_after_idat_rgb();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// plte/duplicate; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_duplicate() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_duplicate()
    ensures m::MW_p(bytes_plte_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_duplicate(),16),height:crate::model::be32(bytes_plte_duplicate(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_duplicate(),16),height:crate::model::be32(bytes_plte_duplicate(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_duplicate(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:62nat,len:9nat,next:75nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_duplicate(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:62nat,len:9nat,next:75nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_duplicate(),75nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:83nat,len:14nat,next:101nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_duplicate(),75nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:83nat,len:14nat,next:101nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_duplicate(),101nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:109nat,len:0nat,next:113nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_duplicate(),101nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:109nat,len:0nat,next:113nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_plte_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
}
fn walk_plte_duplicate() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_duplicate());model_plte_duplicate();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::DuplicateChunk))));
}

// plte/length-8; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_length_8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,8u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,103u8,65u8,50u8,6u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_length_8()
    ensures m::MW_p(bytes_plte_length_8(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_length_8())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_length_8(),16),height:crate::model::be32(bytes_plte_length_8(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_length_8())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_length_8(),16),height:crate::model::be32(bytes_plte_length_8(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(8nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::InvalidPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_8(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:14nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_8(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:14nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_8(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_8(),79nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:87nat,len:0nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_length_8(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()));
}
fn walk_plte_length_8() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,8u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,103u8,65u8,50u8,6u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_length_8());model_plte_length_8();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidPalette))));
}

// plte/length-0; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_length_0() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,0u8,80u8,76u8,84u8,69u8,75u8,168u8,137u8,85u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_length_0()
    ensures m::MW_p(bytes_plte_length_0(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_length_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_length_0(),16),height:crate::model::be32(bytes_plte_length_0(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_length_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_length_0(),16),height:crate::model::be32(bytes_plte_length_0(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:0nat,next:45nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(0nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::InvalidPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_0(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:14nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_0(),45nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:53nat,len:14nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_length_0(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:79nat,len:0nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_length_0(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:79nat,len:0nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_length_0(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()));
}
fn walk_plte_length_0() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,0u8,80u8,76u8,84u8,69u8,75u8,168u8,137u8,85u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_length_0());model_plte_length_0();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidPalette))));
}

// plte/257-entries; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_257_entries() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,3u8,3u8,80u8,76u8,84u8,69u8,88u8,219u8,80u8,119u8,189u8,35u8,68u8,7u8,121u8,236u8,61u8,5u8,168u8,40u8,54u8,164u8,203u8,163u8,237u8,40u8,209u8,141u8,231u8,74u8,246u8,148u8,122u8,127u8,181u8,67u8,115u8,180u8,110u8,126u8,41u8,114u8,185u8,133u8,248u8,105u8,22u8,233u8,15u8,212u8,189u8,243u8,209u8,19u8,140u8,21u8,236u8,102u8,125u8,64u8,248u8,202u8,96u8,79u8,135u8,61u8,206u8,9u8,44u8,237u8,4u8,202u8,8u8,75u8,72u8,10u8,193u8,88u8,66u8,92u8,250u8,6u8,94u8,156u8,184u8,168u8,43u8,251u8,67u8,228u8,136u8,61u8,253u8,1u8,47u8,11u8,202u8,127u8,232u8,26u8,230u8,212u8,246u8,214u8,31u8,244u8,90u8,137u8,225u8,78u8,73u8,36u8,6u8,184u8,220u8,223u8,71u8,46u8,34u8,241u8,206u8,96u8,162u8,51u8,199u8,168u8,120u8,20u8,78u8,103u8,93u8,139u8,134u8,182u8,148u8,71u8,82u8,241u8,10u8,199u8,202u8,1u8,230u8,53u8,113u8,16u8,114u8,80u8,149u8,67u8,12u8,68u8,38u8,50u8,223u8,217u8,162u8,149u8,147u8,6u8,108u8,15u8,227u8,29u8,194u8,33u8,20u8,229u8,211u8,194u8,139u8,66u8,165u8,107u8,187u8,189u8,166u8,21u8,45u8,116u8,173u8,193u8,200u8,212u8,140u8,88u8,192u8,97u8,70u8,221u8,164u8,164u8,130u8,96u8,196u8,102u8,217u8,79u8,55u8,121u8,102u8,249u8,147u8,174u8,124u8,184u8,29u8,102u8,151u8,88u8,27u8,193u8,183u8,95u8,160u8,239u8,251u8,242u8,205u8,21u8,245u8,33u8,151u8,210u8,90u8,95u8,241u8,7u8,38u8,57u8,192u8,132u8,74u8,245u8,4u8,125u8,200u8,226u8,184u8,0u8,67u8,253u8,221u8,116u8,223u8,165u8,1u8,180u8,12u8,184u8,26u8,236u8,50u8,35u8,93u8,123u8,107u8,47u8,71u8,117u8,55u8,149u8,62u8,207u8,2u8,13u8,103u8,116u8,130u8,250u8,237u8,252u8,234u8,246u8,141u8,203u8,60u8,11u8,7u8,91u8,3u8,96u8,147u8,100u8,207u8,206u8,169u8,45u8,64u8,118u8,174u8,141u8,30u8,183u8,113u8,211u8,76u8,203u8,64u8,250u8,144u8,129u8,22u8,194u8,239u8,13u8,44u8,187u8,231u8,60u8,216u8,166u8,108u8,118u8,228u8,121u8,117u8,210u8,237u8,81u8,52u8,183u8,163u8,179u8,135u8,82u8,32u8,54u8,44u8,181u8,23u8,209u8,149u8,237u8,65u8,28u8,214u8,141u8,146u8,206u8,107u8,153u8,52u8,118u8,64u8,223u8,160u8,211u8,167u8,237u8,93u8,76u8,61u8,140u8,56u8,63u8,32u8,181u8,32u8,233u8,132u8,32u8,201u8,14u8,104u8,126u8,128u8,11u8,228u8,50u8,121u8,55u8,66u8,119u8,50u8,4u8,233u8,114u8,36u8,22u8,42u8,86u8,181u8,68u8,109u8,224u8,38u8,19u8,183u8,234u8,4u8,55u8,18u8,231u8,133u8,33u8,208u8,153u8,125u8,44u8,54u8,105u8,97u8,34u8,120u8,178u8,139u8,71u8,229u8,107u8,86u8,13u8,117u8,70u8,20u8,96u8,212u8,71u8,170u8,51u8,166u8,223u8,119u8,221u8,80u8,90u8,0u8,149u8,222u8,173u8,237u8,134u8,87u8,234u8,81u8,182u8,198u8,44u8,71u8,114u8,156u8,72u8,2u8,122u8,148u8,151u8,23u8,196u8,132u8,182u8,110u8,76u8,143u8,134u8,222u8,44u8,168u8,150u8,170u8,79u8,90u8,205u8,225u8,48u8,223u8,22u8,134u8,129u8,62u8,238u8,38u8,159u8,224u8,182u8,41u8,23u8,248u8,155u8,42u8,211u8,217u8,29u8,239u8,35u8,76u8,78u8,201u8,244u8,15u8,114u8,82u8,16u8,153u8,107u8,42u8,24u8,137u8,162u8,226u8,242u8,242u8,206u8,195u8,104u8,201u8,231u8,143u8,26u8,105u8,28u8,220u8,189u8,57u8,104u8,197u8,41u8,10u8,178u8,40u8,63u8,31u8,55u8,94u8,174u8,177u8,92u8,43u8,221u8,170u8,17u8,148u8,191u8,221u8,106u8,90u8,245u8,95u8,112u8,181u8,98u8,59u8,60u8,50u8,252u8,36u8,152u8,172u8,71u8,96u8,172u8,147u8,144u8,211u8,181u8,246u8,192u8,161u8,231u8,77u8,123u8,39u8,129u8,185u8,107u8,239u8,87u8,61u8,91u8,11u8,244u8,108u8,194u8,208u8,218u8,71u8,91u8,32u8,14u8,252u8,184u8,62u8,40u8,135u8,241u8,157u8,15u8,10u8,154u8,249u8,28u8,23u8,161u8,174u8,82u8,111u8,122u8,31u8,66u8,194u8,77u8,184u8,223u8,117u8,246u8,42u8,28u8,14u8,12u8,137u8,60u8,190u8,226u8,214u8,138u8,78u8,160u8,65u8,127u8,147u8,176u8,73u8,67u8,134u8,78u8,107u8,71u8,68u8,222u8,46u8,38u8,244u8,231u8,188u8,91u8,42u8,77u8,15u8,180u8,127u8,238u8,81u8,190u8,84u8,160u8,106u8,154u8,48u8,61u8,106u8,203u8,245u8,90u8,193u8,132u8,181u8,144u8,5u8,142u8,233u8,206u8,238u8,70u8,166u8,69u8,178u8,98u8,41u8,157u8,162u8,48u8,29u8,143u8,237u8,71u8,109u8,185u8,118u8,21u8,147u8,117u8,169u8,135u8,28u8,106u8,112u8,122u8,169u8,228u8,99u8,105u8,178u8,198u8,118u8,93u8,95u8,135u8,197u8,21u8,216u8,23u8,168u8,155u8,164u8,204u8,237u8,199u8,180u8,2u8,173u8,105u8,219u8,28u8,72u8,243u8,2u8,88u8,38u8,35u8,207u8,112u8,23u8,168u8,27u8,5u8,247u8,97u8,37u8,148u8,65u8,47u8,86u8,224u8,180u8,254u8,2u8,29u8,56u8,179u8,33u8,229u8,39u8,209u8,2u8,205u8,211u8,139u8,126u8,208u8,17u8,182u8,102u8,210u8,87u8,69u8,251u8,27u8,133u8,41u8,161u8,8u8,38u8,81u8,97u8,204u8,122u8,118u8,177u8,206u8,62u8,214u8,211u8,210u8,113u8,49u8,135u8,216u8,73u8,6u8,45u8,64u8,129u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_257_entries()
    ensures m::MW_p(bytes_plte_257_entries(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_257_entries())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_257_entries(),16),height:crate::model::be32(bytes_plte_257_entries(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_257_entries())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_257_entries(),16),height:crate::model::be32(bytes_plte_257_entries(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_257_entries(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:771nat,next:816nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_257_entries(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:771nat,next:816nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(771nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::InvalidPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_257_entries(),816nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:824nat,len:14nat,next:842nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_257_entries(),816nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:824nat,len:14nat,next:842nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_257_entries(),842nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:850nat,len:0nat,next:854nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_257_entries(),842nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:850nat,len:0nat,next:854nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        first_palette_error(bytes_plte_257_entries(),crate::model::mh(bytes_plte_257_entries()).unwrap().0,771,Error::InvalidPalette);
}
fn walk_plte_257_entries() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,3u8,3u8,80u8,76u8,84u8,69u8,88u8,219u8,80u8,119u8,189u8,35u8,68u8,7u8,121u8,236u8,61u8,5u8,168u8,40u8,54u8,164u8,203u8,163u8,237u8,40u8,209u8,141u8,231u8,74u8,246u8,148u8,122u8,127u8,181u8,67u8,115u8,180u8,110u8,126u8,41u8,114u8,185u8,133u8,248u8,105u8,22u8,233u8,15u8,212u8,189u8,243u8,209u8,19u8,140u8,21u8,236u8,102u8,125u8,64u8,248u8,202u8,96u8,79u8,135u8,61u8,206u8,9u8,44u8,237u8,4u8,202u8,8u8,75u8,72u8,10u8,193u8,88u8,66u8,92u8,250u8,6u8,94u8,156u8,184u8,168u8,43u8,251u8,67u8,228u8,136u8,61u8,253u8,1u8,47u8,11u8,202u8,127u8,232u8,26u8,230u8,212u8,246u8,214u8,31u8,244u8,90u8,137u8,225u8,78u8,73u8,36u8,6u8,184u8,220u8,223u8,71u8,46u8,34u8,241u8,206u8,96u8,162u8,51u8,199u8,168u8,120u8,20u8,78u8,103u8,93u8,139u8,134u8,182u8,148u8,71u8,82u8,241u8,10u8,199u8,202u8,1u8,230u8,53u8,113u8,16u8,114u8,80u8,149u8,67u8,12u8,68u8,38u8,50u8,223u8,217u8,162u8,149u8,147u8,6u8,108u8,15u8,227u8,29u8,194u8,33u8,20u8,229u8,211u8,194u8,139u8,66u8,165u8,107u8,187u8,189u8,166u8,21u8,45u8,116u8,173u8,193u8,200u8,212u8,140u8,88u8,192u8,97u8,70u8,221u8,164u8,164u8,130u8,96u8,196u8,102u8,217u8,79u8,55u8,121u8,102u8,249u8,147u8,174u8,124u8,184u8,29u8,102u8,151u8,88u8,27u8,193u8,183u8,95u8,160u8,239u8,251u8,242u8,205u8,21u8,245u8,33u8,151u8,210u8,90u8,95u8,241u8,7u8,38u8,57u8,192u8,132u8,74u8,245u8,4u8,125u8,200u8,226u8,184u8,0u8,67u8,253u8,221u8,116u8,223u8,165u8,1u8,180u8,12u8,184u8,26u8,236u8,50u8,35u8,93u8,123u8,107u8,47u8,71u8,117u8,55u8,149u8,62u8,207u8,2u8,13u8,103u8,116u8,130u8,250u8,237u8,252u8,234u8,246u8,141u8,203u8,60u8,11u8,7u8,91u8,3u8,96u8,147u8,100u8,207u8,206u8,169u8,45u8,64u8,118u8,174u8,141u8,30u8,183u8,113u8,211u8,76u8,203u8,64u8,250u8,144u8,129u8,22u8,194u8,239u8,13u8,44u8,187u8,231u8,60u8,216u8,166u8,108u8,118u8,228u8,121u8,117u8,210u8,237u8,81u8,52u8,183u8,163u8,179u8,135u8,82u8,32u8,54u8,44u8,181u8,23u8,209u8,149u8,237u8,65u8,28u8,214u8,141u8,146u8,206u8,107u8,153u8,52u8,118u8,64u8,223u8,160u8,211u8,167u8,237u8,93u8,76u8,61u8,140u8,56u8,63u8,32u8,181u8,32u8,233u8,132u8,32u8,201u8,14u8,104u8,126u8,128u8,11u8,228u8,50u8,121u8,55u8,66u8,119u8,50u8,4u8,233u8,114u8,36u8,22u8,42u8,86u8,181u8,68u8,109u8,224u8,38u8,19u8,183u8,234u8,4u8,55u8,18u8,231u8,133u8,33u8,208u8,153u8,125u8,44u8,54u8,105u8,97u8,34u8,120u8,178u8,139u8,71u8,229u8,107u8,86u8,13u8,117u8,70u8,20u8,96u8,212u8,71u8,170u8,51u8,166u8,223u8,119u8,221u8,80u8,90u8,0u8,149u8,222u8,173u8,237u8,134u8,87u8,234u8,81u8,182u8,198u8,44u8,71u8,114u8,156u8,72u8,2u8,122u8,148u8,151u8,23u8,196u8,132u8,182u8,110u8,76u8,143u8,134u8,222u8,44u8,168u8,150u8,170u8,79u8,90u8,205u8,225u8,48u8,223u8,22u8,134u8,129u8,62u8,238u8,38u8,159u8,224u8,182u8,41u8,23u8,248u8,155u8,42u8,211u8,217u8,29u8,239u8,35u8,76u8,78u8,201u8,244u8,15u8,114u8,82u8,16u8,153u8,107u8,42u8,24u8,137u8,162u8,226u8,242u8,242u8,206u8,195u8,104u8,201u8,231u8,143u8,26u8,105u8,28u8,220u8,189u8,57u8,104u8,197u8,41u8,10u8,178u8,40u8,63u8,31u8,55u8,94u8,174u8,177u8,92u8,43u8,221u8,170u8,17u8,148u8,191u8,221u8,106u8,90u8,245u8,95u8,112u8,181u8,98u8,59u8,60u8,50u8,252u8,36u8,152u8,172u8,71u8,96u8,172u8,147u8,144u8,211u8,181u8,246u8,192u8,161u8,231u8,77u8,123u8,39u8,129u8,185u8,107u8,239u8,87u8,61u8,91u8,11u8,244u8,108u8,194u8,208u8,218u8,71u8,91u8,32u8,14u8,252u8,184u8,62u8,40u8,135u8,241u8,157u8,15u8,10u8,154u8,249u8,28u8,23u8,161u8,174u8,82u8,111u8,122u8,31u8,66u8,194u8,77u8,184u8,223u8,117u8,246u8,42u8,28u8,14u8,12u8,137u8,60u8,190u8,226u8,214u8,138u8,78u8,160u8,65u8,127u8,147u8,176u8,73u8,67u8,134u8,78u8,107u8,71u8,68u8,222u8,46u8,38u8,244u8,231u8,188u8,91u8,42u8,77u8,15u8,180u8,127u8,238u8,81u8,190u8,84u8,160u8,106u8,154u8,48u8,61u8,106u8,203u8,245u8,90u8,193u8,132u8,181u8,144u8,5u8,142u8,233u8,206u8,238u8,70u8,166u8,69u8,178u8,98u8,41u8,157u8,162u8,48u8,29u8,143u8,237u8,71u8,109u8,185u8,118u8,21u8,147u8,117u8,169u8,135u8,28u8,106u8,112u8,122u8,169u8,228u8,99u8,105u8,178u8,198u8,118u8,93u8,95u8,135u8,197u8,21u8,216u8,23u8,168u8,155u8,164u8,204u8,237u8,199u8,180u8,2u8,173u8,105u8,219u8,28u8,72u8,243u8,2u8,88u8,38u8,35u8,207u8,112u8,23u8,168u8,27u8,5u8,247u8,97u8,37u8,148u8,65u8,47u8,86u8,224u8,180u8,254u8,2u8,29u8,56u8,179u8,33u8,229u8,39u8,209u8,2u8,205u8,211u8,139u8,126u8,208u8,17u8,182u8,102u8,210u8,87u8,69u8,251u8,27u8,133u8,41u8,161u8,8u8,38u8,81u8,97u8,204u8,122u8,118u8,177u8,206u8,62u8,214u8,211u8,210u8,113u8,49u8,135u8,216u8,73u8,6u8,45u8,64u8,129u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_257_entries());model_plte_257_entries();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidPalette))));
}

// plte/5-entries-depth-2; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_5_entries_depth_2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,2u8,3u8,0u8,0u8,0u8,15u8,216u8,229u8,183u8,0u8,0u8,0u8,15u8,80u8,76u8,84u8,69u8,95u8,81u8,242u8,214u8,128u8,216u8,244u8,201u8,203u8,204u8,181u8,178u8,78u8,155u8,172u8,131u8,49u8,170u8,166u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,16u8,96u8,156u8,0u8,0u8,0u8,198u8,0u8,162u8,120u8,158u8,27u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_5_entries_depth_2()
    ensures m::MW_p(bytes_plte_5_entries_depth_2(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_5_entries_depth_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_5_entries_depth_2(),16),height:crate::model::be32(bytes_plte_5_entries_depth_2(),20),depth:2,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_5_entries_depth_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_5_entries_depth_2(),16),height:crate::model::be32(bytes_plte_5_entries_depth_2(),20),depth:2,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 2u8)==4u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_5_entries_depth_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:15nat,next:60nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_5_entries_depth_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:15nat,next:60nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(15nat,3u8,2u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::InvalidPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_5_entries_depth_2(),60nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:68nat,len:12nat,next:84nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_5_entries_depth_2(),60nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:68nat,len:12nat,next:84nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_5_entries_depth_2(),84nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:92nat,len:0nat,next:96nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_5_entries_depth_2(),84nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:92nat,len:0nat,next:96nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_5_entries_depth_2(),m::strict())==m::Outcome::Error(Error::InvalidPalette,m::no_events()));
}
fn walk_plte_5_entries_depth_2() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,2u8,3u8,0u8,0u8,0u8,15u8,216u8,229u8,183u8,0u8,0u8,0u8,15u8,80u8,76u8,84u8,69u8,95u8,81u8,242u8,214u8,128u8,216u8,244u8,201u8,203u8,204u8,181u8,178u8,78u8,155u8,172u8,131u8,49u8,170u8,166u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,16u8,96u8,156u8,0u8,0u8,0u8,198u8,0u8,162u8,120u8,158u8,27u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_5_entries_depth_2());model_plte_5_entries_depth_2();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidPalette))));
}

// plte/in-greyscale; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_in_greyscale() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,0u8,0u8,0u8,0u8,87u8,221u8,82u8,248u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,4u8,2u8,6u8,0u8,0u8,20u8,0u8,5u8,115u8,141u8,79u8,172u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_in_greyscale()
    ensures m::MW_p(bytes_plte_in_greyscale(),m::strict())==m::Outcome::Error(Error::UnexpectedPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_in_greyscale())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_in_greyscale(),16),height:crate::model::be32(bytes_plte_in_greyscale(),20),depth:8,color_type:0,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_in_greyscale())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_in_greyscale(),16),height:crate::model::be32(bytes_plte_in_greyscale(),20),depth:8,color_type:0,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_greyscale(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_greyscale(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,0u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::UnexpectedPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_greyscale(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:12nat,next:78nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_greyscale(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:12nat,next:78nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_greyscale(),78nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:86nat,len:0nat,next:90nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_greyscale(),78nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:86nat,len:0nat,next:90nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_in_greyscale(),m::strict())==m::Outcome::Error(Error::UnexpectedPalette,m::no_events()));
}
fn walk_plte_in_greyscale() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,0u8,0u8,0u8,0u8,87u8,221u8,82u8,248u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,4u8,2u8,6u8,0u8,0u8,20u8,0u8,5u8,115u8,141u8,79u8,172u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_in_greyscale());model_plte_in_greyscale();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnexpectedPalette))));
}

// plte/in-grey-alpha; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_in_grey_alpha() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,4u8,0u8,0u8,0u8,216u8,191u8,197u8,175u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,100u8,2u8,66u8,70u8,38u8,6u8,6u8,0u8,0u8,69u8,0u8,11u8,56u8,247u8,216u8,123u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_plte_in_grey_alpha()
    ensures m::MW_p(bytes_plte_in_grey_alpha(),m::strict())==m::Outcome::Error(Error::UnexpectedPalette,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_plte_in_grey_alpha())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_in_grey_alpha(),16),height:crate::model::be32(bytes_plte_in_grey_alpha(),20),depth:8,color_type:4,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_plte_in_grey_alpha())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_plte_in_grey_alpha(),16),height:crate::model::be32(bytes_plte_in_grey_alpha(),20),depth:8,color_type:4,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_grey_alpha(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_grey_alpha(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,4u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Error(Error::UnexpectedPalette)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_grey_alpha(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:16nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_grey_alpha(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:16nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_in_grey_alpha(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_in_grey_alpha(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_plte_in_grey_alpha(),m::strict())==m::Outcome::Error(Error::UnexpectedPalette,m::no_events()));
}
fn walk_plte_in_grey_alpha() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,4u8,0u8,0u8,0u8,216u8,191u8,197u8,175u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,100u8,2u8,66u8,70u8,38u8,6u8,6u8,0u8,0u8,69u8,0u8,11u8,56u8,247u8,216u8,123u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_in_grey_alpha());model_plte_in_grey_alpha();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnexpectedPalette))));
}

// trns/before-plte; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_before_plte() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,1u8,116u8,82u8,78u8,83u8,0u8,64u8,230u8,216u8,102u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_before_plte()
    ensures m::MW_p(bytes_trns_before_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_before_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_before_plte(),16),height:crate::model::be32(bytes_trns_before_plte(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_before_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_before_plte(),16),height:crate::model::be32(bytes_trns_before_plte(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_before_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_before_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(1nat,3u8,0nat,false)==crate::metadata_model::Decision::Error(Error::InvalidTransparency)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_before_plte(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:54nat,len:9nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_before_plte(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:54nat,len:9nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,true,false,false)==crate::metadata_model::Decision::Error(Error::ChunkOutOfOrder)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_before_plte(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:75nat,len:14nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_before_plte(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:75nat,len:14nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_before_plte(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:101nat,len:0nat,next:105nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_before_plte(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:101nat,len:0nat,next:105nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_trns_before_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_trns_before_plte() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,1u8,116u8,82u8,78u8,83u8,0u8,64u8,230u8,216u8,102u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_before_plte());model_trns_before_plte();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// trns/after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_after_idat()
    ensures m::MW_p(bytes_trns_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_after_idat(),16),height:crate::model::be32(bytes_trns_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_after_idat(),16),height:crate::model::be32(bytes_trns_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:75nat,len:6nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:75nat,len:6nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,2u8,0nat,false)==crate::metadata_model::Decision::Keep(6nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_after_idat(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_after_idat(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_trns_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_trns_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_after_idat());model_trns_after_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// trns/duplicate; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_duplicate() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_duplicate()
    ensures m::MW_p(bytes_trns_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_duplicate(),16),height:crate::model::be32(bytes_trns_duplicate(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_duplicate(),16),height:crate::model::be32(bytes_trns_duplicate(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,2u8,0nat,false)==crate::metadata_model::Decision::Keep(6nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_duplicate(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:6nat,next:69nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_duplicate(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:6nat,next:69nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,2u8,0nat,false)==crate::metadata_model::Decision::Keep(6nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_duplicate(),69nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:77nat,len:22nat,next:103nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_duplicate(),69nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:77nat,len:22nat,next:103nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_duplicate(),103nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:111nat,len:0nat,next:115nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_duplicate(),103nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:111nat,len:0nat,next:115nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_trns_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
}
fn walk_trns_duplicate() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_duplicate());model_trns_duplicate();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::DuplicateChunk))));
}

// trns/palette-too-long; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_palette_too_long() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,4u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,179u8,147u8,102u8,154u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_palette_too_long()
    ensures m::MW_p(bytes_trns_palette_too_long(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_palette_too_long())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_palette_too_long(),16),height:crate::model::be32(bytes_trns_palette_too_long(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_palette_too_long())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_palette_too_long(),16),height:crate::model::be32(bytes_trns_palette_too_long(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_too_long(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_too_long(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_too_long(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:62nat,len:4nat,next:70nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_too_long(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:62nat,len:4nat,next:70nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(4nat,3u8,3nat,false)==crate::metadata_model::Decision::Error(Error::InvalidTransparency)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_too_long(),70nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:78nat,len:14nat,next:96nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_too_long(),70nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:78nat,len:14nat,next:96nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_too_long(),96nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:104nat,len:0nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_too_long(),96nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:104nat,len:0nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_trns_palette_too_long(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()));
}
fn walk_trns_palette_too_long() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,4u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,179u8,147u8,102u8,154u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_palette_too_long());model_trns_palette_too_long();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidTransparency))));
}

// trns/rgb-length-2; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_rgb_length_2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,0u8,0u8,118u8,147u8,205u8,56u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_rgb_length_2()
    ensures m::MW_p(bytes_trns_rgb_length_2(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_rgb_length_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_rgb_length_2(),16),height:crate::model::be32(bytes_trns_rgb_length_2(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_rgb_length_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_rgb_length_2(),16),height:crate::model::be32(bytes_trns_rgb_length_2(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_length_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_rgb_length_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,2u8,0nat,false)==crate::metadata_model::Decision::Error(Error::InvalidTransparency)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_length_2(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:22nat,next:81nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_rgb_length_2(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:22nat,next:81nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_length_2(),81nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:89nat,len:0nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_rgb_length_2(),81nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:89nat,len:0nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_trns_rgb_length_2(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()));
}
fn walk_trns_rgb_length_2() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,0u8,0u8,118u8,147u8,205u8,56u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_rgb_length_2());model_trns_rgb_length_2();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidTransparency))));
}

// trns/in-rgba; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_in_rgba() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,11u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,0u8,2u8,0u8,0u8,5u8,0u8,1u8,122u8,94u8,171u8,63u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_trns_in_rgba()
    ensures m::MW_p(bytes_trns_in_rgba(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_trns_in_rgba())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_in_rgba(),16),height:crate::model::be32(bytes_trns_in_rgba(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_trns_in_rgba())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_trns_in_rgba(),16),height:crate::model::be32(bytes_trns_in_rgba(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_in_rgba(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_in_rgba(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,6u8,0nat,false)==crate::metadata_model::Decision::Error(Error::InvalidTransparency)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_in_rgba(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:11nat,next:74nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_in_rgba(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:11nat,next:74nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_in_rgba(),74nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:82nat,len:0nat,next:86nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_in_rgba(),74nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:82nat,len:0nat,next:86nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_trns_in_rgba(),m::strict())==m::Outcome::Error(Error::InvalidTransparency,m::no_events()));
}
fn walk_trns_in_rgba() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,0u8,0u8,0u8,0u8,0u8,0u8,110u8,166u8,7u8,145u8,0u8,0u8,0u8,11u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,0u8,2u8,0u8,0u8,5u8,0u8,1u8,122u8,94u8,171u8,63u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_in_rgba());model_trns_in_rgba();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidTransparency))));
}

// anc-bad/gamma-0; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_gamma_0() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,0u8,0u8,139u8,37u8,96u8,77u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_gamma_0()
    ensures m::MW_p(bytes_anc_bad_gamma_0(),m::strict())==m::Outcome::Error(Error::InvalidGamma,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_gamma_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_gamma_0(),16),height:crate::model::be32(bytes_anc_bad_gamma_0(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_gamma_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_gamma_0(),16),height:crate::model::be32(bytes_anc_bad_gamma_0(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_gamma_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_gamma_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((103u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Gama,bytes_anc_bad_gamma_0().subrange(41,45),2u8,8u8,0nat)==crate::metadata_model::Outcome::Error(Error::InvalidGamma)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_gamma_0(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:22nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_gamma_0(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:22nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_gamma_0(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_gamma_0(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_gamma_0(),m::strict())==m::Outcome::Error(Error::InvalidGamma,m::no_events()));
}
fn walk_anc_bad_gamma_0() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,0u8,0u8,139u8,37u8,96u8,77u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_gamma_0());model_anc_bad_gamma_0();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidGamma))));
}

// anc-bad/iccp-name-80; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_iccp_name_80() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,110u8,105u8,67u8,67u8,80u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,40u8,21u8,183u8,164u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_iccp_name_80()
    ensures m::MW_p(bytes_anc_bad_iccp_name_80(),m::strict())==m::Outcome::Error(Error::InvalidIccp,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_iccp_name_80())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_iccp_name_80(),16),height:crate::model::be32(bytes_anc_bad_iccp_name_80(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_iccp_name_80())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_iccp_name_80(),16),height:crate::model::be32(bytes_anc_bad_iccp_name_80(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_name_80(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:366nat,next:411nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_name_80(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:366nat,next:411nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_bad_iccp_name_80().subrange(41,407),0)==Some(80nat)) by (compute);
        assert(bytes_anc_bad_iccp_name_80().subrange(41,407).subrange(0,80)==seq![110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8]) by (compute);
        assert(!crate::model::keyword(seq![110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_name_80(),411nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:419nat,len:22nat,next:445nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_name_80(),411nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:419nat,len:22nat,next:445nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_name_80(),445nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:453nat,len:0nat,next:457nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_name_80(),445nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:453nat,len:0nat,next:457nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        invalid_iccp_keyword(bytes_anc_bad_iccp_name_80().subrange(41,407),2u8,8u8,80nat);
        first_iccp_error(bytes_anc_bad_iccp_name_80(),crate::model::mh(bytes_anc_bad_iccp_name_80()).unwrap().0,366nat);
}
fn walk_anc_bad_iccp_name_80() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,110u8,105u8,67u8,67u8,80u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,40u8,21u8,183u8,164u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_iccp_name_80());model_anc_bad_iccp_name_80();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidIccp))));
}

// anc-bad/iccp-leading-space; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_iccp_leading_space() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,32u8,105u8,67u8,67u8,80u8,32u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,6u8,100u8,201u8,135u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_iccp_leading_space()
    ensures m::MW_p(bytes_anc_bad_iccp_leading_space(),m::strict())==m::Outcome::Error(Error::InvalidIccp,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_iccp_leading_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_iccp_leading_space(),16),height:crate::model::be32(bytes_anc_bad_iccp_leading_space(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_iccp_leading_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_iccp_leading_space(),16),height:crate::model::be32(bytes_anc_bad_iccp_leading_space(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:288nat,next:333nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:288nat,next:333nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_bad_iccp_leading_space().subrange(41,329),0)==Some(2nat)) by (compute);
        assert(bytes_anc_bad_iccp_leading_space().subrange(41,329).subrange(0,2)==seq![32u8,110u8]) by (compute);
        assert(!crate::model::keyword(seq![32u8,110u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),333nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:341nat,len:22nat,next:367nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),333nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:341nat,len:22nat,next:367nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),367nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:375nat,len:0nat,next:379nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_iccp_leading_space(),367nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:375nat,len:0nat,next:379nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        invalid_iccp_keyword(bytes_anc_bad_iccp_leading_space().subrange(41,329),2u8,8u8,2nat);
        first_iccp_error(bytes_anc_bad_iccp_leading_space(),crate::model::mh(bytes_anc_bad_iccp_leading_space()).unwrap().0,288nat);
}
fn walk_anc_bad_iccp_leading_space() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,32u8,105u8,67u8,67u8,80u8,32u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,6u8,100u8,201u8,135u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_iccp_leading_space());model_anc_bad_iccp_leading_space();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidIccp))));
}

// anc-bad/cicp-matrix-1; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_cicp_matrix_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,99u8,73u8,67u8,80u8,1u8,13u8,1u8,1u8,133u8,114u8,10u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_cicp_matrix_1()
    ensures m::MW_p(bytes_anc_bad_cicp_matrix_1(),m::strict())==m::Outcome::Error(Error::InvalidCicp,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_cicp_matrix_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_cicp_matrix_1(),16),height:crate::model::be32(bytes_anc_bad_cicp_matrix_1(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_cicp_matrix_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_cicp_matrix_1(),16),height:crate::model::be32(bytes_anc_bad_cicp_matrix_1(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![99u8,73u8,67u8,80u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![99u8,73u8,67u8,80u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((99u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![99u8,73u8,67u8,80u8])==crate::walk_model::Name::Cicp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![99u8,73u8,67u8,80u8])==crate::walk_model::Name::Cicp) by (compute);
        }
        assert(bytes_anc_bad_cicp_matrix_1().subrange(41,45)[0]==1u8) by (compute);
        assert(bytes_anc_bad_cicp_matrix_1().subrange(41,45)[1]==13u8) by (compute);
        assert(bytes_anc_bad_cicp_matrix_1().subrange(41,45)[2]==1u8) by (compute);
        assert(bytes_anc_bad_cicp_matrix_1().subrange(41,45)[3]==1u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Cicp,bytes_anc_bad_cicp_matrix_1().subrange(41,45),2u8,8u8,0nat)==crate::metadata_model::Outcome::Error(Error::InvalidCicp)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:22nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:22nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_cicp_matrix_1(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_cicp_matrix_1(),m::strict())==m::Outcome::Error(Error::InvalidCicp,m::no_events()));
}
fn walk_anc_bad_cicp_matrix_1() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,99u8,73u8,67u8,80u8,1u8,13u8,1u8,1u8,133u8,114u8,10u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_cicp_matrix_1());model_anc_bad_cicp_matrix_1();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidCicp))));
}

// anc-bad/bkgd-high-bits; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_bkgd_high_bits() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,1u8,0u8,0u8,0u8,0u8,0u8,50u8,31u8,104u8,218u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_bkgd_high_bits()
    ensures m::MW_p(bytes_anc_bad_bkgd_high_bits(),m::strict())==m::Outcome::Error(Error::InvalidBackground,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_bkgd_high_bits())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_bkgd_high_bits(),16),height:crate::model::be32(bytes_anc_bad_bkgd_high_bits(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_bkgd_high_bits())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_bkgd_high_bits(),16),height:crate::model::be32(bytes_anc_bad_bkgd_high_bits(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[0]==1u8) by (compute);
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[1]==0u8) by (compute);
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[2]==0u8) by (compute);
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[3]==0u8) by (compute);
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[4]==0u8) by (compute);
        assert(bytes_anc_bad_bkgd_high_bits().subrange(41,47)[5]==0u8) by (compute);
        assert(crate::metadata_model::words16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),3nat).len()==3);
        assert(crate::model::be16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),0)==256u16) by (compute);
        assert(crate::metadata_model::words16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),3nat)[0]==256u16);
        assert(crate::model::be16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),2)==0u16) by (compute);
        assert(crate::metadata_model::words16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),3nat)[1]==0u16);
        assert(crate::model::be16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),4)==0u16) by (compute);
        assert(crate::metadata_model::words16(bytes_anc_bad_bkgd_high_bits().subrange(41,47),3nat)[2]==0u16);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_high_bits(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_bkgd_high_bits(),m::strict())==m::Outcome::Error(Error::InvalidBackground,m::no_events()));
}
fn walk_anc_bad_bkgd_high_bits() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,1u8,0u8,0u8,0u8,0u8,0u8,50u8,31u8,104u8,218u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_bkgd_high_bits());model_anc_bad_bkgd_high_bits();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidBackground))));
}

// anc-bad/exif-len-3; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_exif_len_3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,101u8,88u8,73u8,102u8,77u8,77u8,0u8,210u8,4u8,139u8,154u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_exif_len_3()
    ensures m::MW_p(bytes_anc_bad_exif_len_3(),m::strict())==m::Outcome::Error(Error::InvalidExif,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_exif_len_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_exif_len_3(),16),height:crate::model::be32(bytes_anc_bad_exif_len_3(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_exif_len_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_exif_len_3(),16),height:crate::model::be32(bytes_anc_bad_exif_len_3(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_exif_len_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![101u8,88u8,73u8,102u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_exif_len_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![101u8,88u8,73u8,102u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((101u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![101u8,88u8,73u8,102u8])==crate::walk_model::Name::Exif) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![101u8,88u8,73u8,102u8])==crate::walk_model::Name::Exif) by (compute);
        }
        assert(bytes_anc_bad_exif_len_3().subrange(41,44)[0]==77u8) by (compute);
        assert(bytes_anc_bad_exif_len_3().subrange(41,44)[1]==77u8) by (compute);
        assert(bytes_anc_bad_exif_len_3().subrange(41,44)[2]==0u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Exif,bytes_anc_bad_exif_len_3().subrange(41,44),2u8,8u8,0nat)==crate::metadata_model::Outcome::Error(Error::InvalidExif)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_exif_len_3(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_exif_len_3(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_exif_len_3(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_exif_len_3(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_exif_len_3(),m::strict())==m::Outcome::Error(Error::InvalidExif,m::no_events()));
}
fn walk_anc_bad_exif_len_3() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,101u8,88u8,73u8,102u8,77u8,77u8,0u8,210u8,4u8,139u8,154u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_exif_len_3());model_anc_bad_exif_len_3();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidExif))));
}

// anc-bad/time-second-61; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_time_second_61() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,1u8,1u8,0u8,0u8,61u8,166u8,191u8,144u8,128u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_time_second_61()
    ensures m::MW_p(bytes_anc_bad_time_second_61(),m::strict())==m::Outcome::Error(Error::InvalidTime,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_time_second_61())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_time_second_61(),16),height:crate::model::be32(bytes_anc_bad_time_second_61(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_time_second_61())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_time_second_61(),16),height:crate::model::be32(bytes_anc_bad_time_second_61(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_time_second_61(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,73u8,77u8,69u8],data:41nat,len:7nat,next:52nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_time_second_61(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,73u8,77u8,69u8],data:41nat,len:7nat,next:52nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by (compute);
        }
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[0]==7u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[1]==234u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[2]==1u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[3]==1u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[4]==0u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[5]==0u8) by (compute);
        assert(bytes_anc_bad_time_second_61().subrange(41,48)[6]==61u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Time,bytes_anc_bad_time_second_61().subrange(41,48),2u8,8u8,0nat)==crate::metadata_model::Outcome::Error(Error::InvalidTime)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_time_second_61(),52nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:60nat,len:22nat,next:86nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_time_second_61(),52nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:60nat,len:22nat,next:86nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_time_second_61(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_time_second_61(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_time_second_61(),m::strict())==m::Outcome::Error(Error::InvalidTime,m::no_events()));
}
fn walk_anc_bad_time_second_61() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,1u8,1u8,0u8,0u8,61u8,166u8,191u8,144u8,128u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_time_second_61());model_anc_bad_time_second_61();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidTime))));
}

// anc-bad/text-keyword-double-space; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_text_keyword_double_space() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,116u8,69u8,88u8,116u8,97u8,32u8,32u8,98u8,0u8,120u8,161u8,31u8,173u8,18u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_text_keyword_double_space()
    ensures m::MW_p(bytes_anc_bad_text_keyword_double_space(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_text_keyword_double_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_text_keyword_double_space(),16),height:crate::model::be32(bytes_anc_bad_text_keyword_double_space(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_text_keyword_double_space())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_text_keyword_double_space(),16),height:crate::model::be32(bytes_anc_bad_text_keyword_double_space(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_bad_text_keyword_double_space().subrange(41,47),0)==Some(4nat)) by (compute);
        assert(bytes_anc_bad_text_keyword_double_space().subrange(41,47).subrange(0,4)==seq![97u8,32u8,32u8,98u8]) by (compute);
        assert(!crate::model::keyword(seq![97u8,32u8,32u8,98u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_text_keyword_double_space(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_text_keyword_double_space(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()));
}
fn walk_anc_bad_text_keyword_double_space() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,116u8,69u8,88u8,116u8,97u8,32u8,32u8,98u8,0u8,120u8,161u8,31u8,173u8,18u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_text_keyword_double_space());model_anc_bad_text_keyword_double_space();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidText))));
}

// anc-bad/itxt-compressed-method-1; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_itxt_compressed_method_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,105u8,84u8,88u8,116u8,107u8,0u8,1u8,1u8,0u8,0u8,164u8,111u8,185u8,54u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_itxt_compressed_method_1()
    ensures m::MW_p(bytes_anc_bad_itxt_compressed_method_1(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_itxt_compressed_method_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_itxt_compressed_method_1(),16),height:crate::model::be32(bytes_anc_bad_itxt_compressed_method_1(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_itxt_compressed_method_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_itxt_compressed_method_1(),16),height:crate::model::be32(bytes_anc_bad_itxt_compressed_method_1(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,84u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,84u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,84u8,88u8,116u8])==crate::walk_model::Name::Itxt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,84u8,88u8,116u8])==crate::walk_model::Name::Itxt) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47),0)==Some(1nat)) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47).subrange(0,1)==seq![107u8]) by (compute);
        assert(crate::model::keyword(seq![107u8]));
        assert(crate::model::first_nul(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47),4nat)==Some(4nat)) by (compute);
        assert(crate::model::first_nul(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47),5nat)==Some(5nat)) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[0]==107u8) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[1]==0u8) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[2]==1u8) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[3]==1u8) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[4]==0u8) by (compute);
        assert(bytes_anc_bad_itxt_compressed_method_1().subrange(41,47)[5]==0u8) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_compressed_method_1(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_itxt_compressed_method_1(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()));
}
fn walk_anc_bad_itxt_compressed_method_1() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,105u8,84u8,88u8,116u8,107u8,0u8,1u8,1u8,0u8,0u8,164u8,111u8,185u8,54u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_itxt_compressed_method_1());model_anc_bad_itxt_compressed_method_1();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidText))));
}

// anc-bad/itxt-missing-separators; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_itxt_missing_separators() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,105u8,84u8,88u8,116u8,107u8,0u8,0u8,0u8,101u8,110u8,175u8,145u8,103u8,217u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_itxt_missing_separators()
    ensures m::MW_p(bytes_anc_bad_itxt_missing_separators(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_itxt_missing_separators())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_itxt_missing_separators(),16),height:crate::model::be32(bytes_anc_bad_itxt_missing_separators(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_itxt_missing_separators())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_itxt_missing_separators(),16),height:crate::model::be32(bytes_anc_bad_itxt_missing_separators(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,84u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,84u8,88u8,116u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,84u8,88u8,116u8])==crate::walk_model::Name::Itxt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,84u8,88u8,116u8])==crate::walk_model::Name::Itxt) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_bad_itxt_missing_separators().subrange(41,47),0)==Some(1nat)) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47).subrange(0,1)==seq![107u8]) by (compute);
        assert(crate::model::keyword(seq![107u8]));
        assert(crate::model::first_nul(bytes_anc_bad_itxt_missing_separators().subrange(41,47),4nat)==None) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[0]==107u8) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[1]==0u8) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[2]==0u8) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[3]==0u8) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[4]==101u8) by (compute);
        assert(bytes_anc_bad_itxt_missing_separators().subrange(41,47)[5]==110u8) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_itxt_missing_separators(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_itxt_missing_separators(),m::strict())==m::Outcome::Error(Error::InvalidText,m::no_events()));
}
fn walk_anc_bad_itxt_missing_separators() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,105u8,84u8,88u8,116u8,107u8,0u8,0u8,0u8,101u8,110u8,175u8,145u8,103u8,217u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_itxt_missing_separators());model_anc_bad_itxt_missing_separators();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidText))));
}

// anc-bad/hist-without-plte; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_hist_without_plte() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_hist_without_plte()
    ensures m::MW_p(bytes_anc_bad_hist_without_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_hist_without_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_hist_without_plte(),16),height:crate::model::be32(bytes_anc_bad_hist_without_plte(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_hist_without_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_hist_without_plte(),16),height:crate::model::be32(bytes_anc_bad_hist_without_plte(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_hist_without_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_hist_without_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_hist_without_plte(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_hist_without_plte(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:22nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_hist_without_plte(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_hist_without_plte(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_anc_bad_hist_without_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_anc_bad_hist_without_plte() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_hist_without_plte());model_anc_bad_hist_without_plte();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// anc-bad/bkgd-index-3-of-3; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_bkgd_index_3_of_3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,1u8,98u8,75u8,71u8,68u8,3u8,17u8,12u8,76u8,242u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_bkgd_index_3_of_3()
    ensures m::MW_p(bytes_anc_bad_bkgd_index_3_of_3(),m::strict())==m::Outcome::Error(Error::InvalidBackground,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_bkgd_index_3_of_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_bkgd_index_3_of_3(),16),height:crate::model::be32(bytes_anc_bad_bkgd_index_3_of_3(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_bkgd_index_3_of_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_bkgd_index_3_of_3(),16),height:crate::model::be32(bytes_anc_bad_bkgd_index_3_of_3(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:62nat,len:1nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:62nat,len:1nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert(bytes_anc_bad_bkgd_index_3_of_3().subrange(62,63)[0]==3u8) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:75nat,len:14nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:75nat,len:14nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:101nat,len:0nat,next:105nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_bkgd_index_3_of_3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:101nat,len:0nat,next:105nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_anc_bad_bkgd_index_3_of_3(),m::strict())==m::Outcome::Error(Error::InvalidBackground,m::no_events()));
}
fn walk_anc_bad_bkgd_index_3_of_3() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,1u8,98u8,75u8,71u8,68u8,3u8,17u8,12u8,76u8,242u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_bkgd_index_3_of_3());model_anc_bad_bkgd_index_3_of_3();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidBackground))));
}

// anc-bad/sbit-palette-9; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_bad_sbit_palette_9() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,3u8,115u8,66u8,73u8,84u8,9u8,1u8,1u8,114u8,61u8,38u8,58u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_anc_bad_sbit_palette_9()
    ensures m::MW_p(bytes_anc_bad_sbit_palette_9(),m::strict())==m::Outcome::Error(Error::InvalidSbit,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_anc_bad_sbit_palette_9())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_sbit_palette_9(),16),height:crate::model::be32(bytes_anc_bad_sbit_palette_9(),20),depth:8,color_type:3,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_anc_bad_sbit_palette_9())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_anc_bad_sbit_palette_9(),16),height:crate::model::be32(bytes_anc_bad_sbit_palette_9(),20),depth:8,color_type:3,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,66u8,73u8,84u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,66u8,73u8,84u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,66u8,73u8,84u8])==crate::walk_model::Name::Sbit) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,66u8,73u8,84u8])==crate::walk_model::Name::Sbit) by (compute);
        }
        assert(bytes_anc_bad_sbit_palette_9().subrange(41,44)[0]==9u8) by (compute);
        assert(bytes_anc_bad_sbit_palette_9().subrange(41,44)[1]==1u8) by (compute);
        assert(bytes_anc_bad_sbit_palette_9().subrange(41,44)[2]==1u8) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:56nat,len:9nat,next:69nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:56nat,len:9nat,next:69nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),69nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:77nat,len:14nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),69nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:77nat,len:14nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),95nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:103nat,len:0nat,next:107nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_bad_sbit_palette_9(),95nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:103nat,len:0nat,next:107nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_anc_bad_sbit_palette_9(),m::strict())==m::Outcome::Error(Error::InvalidSbit,m::no_events()));
}
fn walk_anc_bad_sbit_palette_9() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,3u8,115u8,66u8,73u8,84u8,9u8,1u8,1u8,114u8,61u8,38u8,58u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_bad_sbit_palette_9());model_anc_bad_sbit_palette_9();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidSbit))));
}

// order/gAMA-after-plte; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_gama_after_plte() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_order_gama_after_plte()
    ensures m::MW_p(bytes_order_gama_after_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_order_gama_after_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_after_plte(),16),height:crate::model::be32(bytes_order_gama_after_plte(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_order_gama_after_plte())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_after_plte(),16),height:crate::model::be32(bytes_order_gama_after_plte(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_plte(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_plte(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:62nat,len:4nat,next:70nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_plte(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:62nat,len:4nat,next:70nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((103u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Gama,bytes_order_gama_after_plte().subrange(62,66),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Gama(45455u32))) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_plte(),70nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:78nat,len:22nat,next:104nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_plte(),70nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:78nat,len:22nat,next:104nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_plte(),104nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:112nat,len:0nat,next:116nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_plte(),104nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:112nat,len:0nat,next:116nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_order_gama_after_plte(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_order_gama_after_plte() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_gama_after_plte());model_order_gama_after_plte();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// order/gAMA-after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_gama_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_order_gama_after_idat()
    ensures m::MW_p(bytes_order_gama_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_order_gama_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_after_idat(),16),height:crate::model::be32(bytes_order_gama_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_order_gama_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_after_idat(),16),height:crate::model::be32(bytes_order_gama_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:75nat,len:4nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:75nat,len:4nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((103u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Gama,bytes_order_gama_after_idat().subrange(75,79),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Gama(45455u32))) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_after_idat(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_after_idat(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_order_gama_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_order_gama_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_gama_after_idat());model_order_gama_after_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// order/gAMA-duplicate; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_gama_duplicate() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_order_gama_duplicate()
    ensures m::MW_p(bytes_order_gama_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_order_gama_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_duplicate(),16),height:crate::model::be32(bytes_order_gama_duplicate(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_order_gama_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_gama_duplicate(),16),height:crate::model::be32(bytes_order_gama_duplicate(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((103u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Gama,bytes_order_gama_duplicate().subrange(41,45),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Gama(45455u32))) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_duplicate(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:57nat,len:4nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_duplicate(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![103u8,65u8,77u8,65u8],data:57nat,len:4nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((103u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![103u8,65u8,77u8,65u8])==crate::walk_model::Name::Gama) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Gama,bytes_order_gama_duplicate().subrange(57,61),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Gama(45455u32))) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_duplicate(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:22nat,next:99nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_duplicate(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:22nat,next:99nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_gama_duplicate(),99nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:107nat,len:0nat,next:111nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_gama_duplicate(),99nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:107nat,len:0nat,next:111nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MW_p(bytes_order_gama_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
}
fn walk_order_gama_duplicate() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_gama_duplicate());model_order_gama_duplicate();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::DuplicateChunk))));
}

// order/iCCP-after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_iccp_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,87u8,120u8,19u8,156u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_order_iccp_after_idat()
    ensures m::MW_p(bytes_order_iccp_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_order_iccp_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_iccp_after_idat(),16),height:crate::model::be32(bytes_order_iccp_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_order_iccp_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_iccp_after_idat(),16),height:crate::model::be32(bytes_order_iccp_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_iccp_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_iccp_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_iccp_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:75nat,len:287nat,next:366nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_iccp_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:75nat,len:287nat,next:366nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::first_nul(bytes_order_iccp_after_idat().subrange(75,362),0)==Some(1nat)) by (compute);
        assert(bytes_order_iccp_after_idat().subrange(75,362).subrange(0,1)==seq![110u8]) by (compute);
        assert(crate::model::keyword(seq![110u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_iccp_after_idat(),366nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:374nat,len:0nat,next:378nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_iccp_after_idat(),366nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:374nat,len:0nat,next:378nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        idat_then_iccp_order(bytes_order_iccp_after_idat(),crate::model::mh(bytes_order_iccp_after_idat()).unwrap().0,22,287);
}
fn walk_order_iccp_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,110u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,87u8,120u8,19u8,156u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_iccp_after_idat());model_order_iccp_after_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

// order/pHYs-after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_phys_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,9u8,112u8,72u8,89u8,115u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,79u8,37u8,196u8,214u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
proof fn model_order_phys_after_idat()
    ensures m::MW_p(bytes_order_phys_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert(crate::model::mh(bytes_order_phys_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_phys_after_idat(),16),height:crate::model::be32(bytes_order_phys_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_order_phys_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_order_phys_after_idat(),16),height:crate::model::be32(bytes_order_phys_after_idat(),20),depth:8,color_type:2,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_phys_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_phys_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_phys_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![112u8,72u8,89u8,115u8],data:75nat,len:9nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_phys_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![112u8,72u8,89u8,115u8],data:75nat,len:9nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((112u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![112u8,72u8,89u8,115u8])==crate::walk_model::Name::Phys) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![112u8,72u8,89u8,115u8])==crate::walk_model::Name::Phys) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_phys_after_idat(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_phys_after_idat(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(bytes_order_phys_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}
fn walk_order_phys_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,9u8,112u8,72u8,89u8,115u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,79u8,37u8,196u8,214u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_phys_after_idat());model_order_phys_after_idat();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_zero_frames(b: Seq<u8>)
    requires b.len()==100,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==0,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Err((Error::InvalidAnimationControl,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(0u32<=crate::limits::MAX_FRAMES) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()));
}

// apng-bad/actl-zero-frames; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_actl_zero_frames() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,137u8,77u8,192u8,16u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_zero_frames()
    ensures m::MWF_p(bytes_apng_bad_actl_zero_frames(),m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_actl_zero_frames())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_zero_frames(),16),height:crate::model::be32(bytes_apng_bad_actl_zero_frames(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_zero_frames())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_zero_frames(),16),height:crate::model::be32(bytes_apng_bad_actl_zero_frames(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames().subrange(41,49),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames().subrange(41,49),4)==0) by (compute);
        assert(0u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_actl_zero_frames().subrange(41,49))==Err((Error::InvalidAnimationControl,true))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_zero_frames().subrange(41,49))==Err((Error::InvalidAnimationControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_actl_zero_frames(bytes_apng_bad_actl_zero_frames());
}
fn walk_apng_bad_actl_zero_frames() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,137u8,77u8,192u8,16u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_zero_frames());model_apng_bad_actl_zero_frames();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidAnimationControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_zero_frames_still(b: Seq<u8>)
    requires b.len()==100,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==0,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Err((Error::InvalidAnimationControl,true)),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(0u32<=crate::limits::MAX_FRAMES) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(b,m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()));
}

// apng-bad/actl-zero-frames-still; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_actl_zero_frames_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,137u8,77u8,192u8,16u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_zero_frames_still()
    ensures m::MW_p(bytes_apng_bad_actl_zero_frames_still(),m::strict())==m::Outcome::Error(Error::InvalidAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MW_p);
        assert(crate::model::mh(bytes_apng_bad_actl_zero_frames_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),16),height:crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_zero_frames_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),16),height:crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames_still().subrange(41,49),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames_still().subrange(41,49),4)==0) by (compute);
        assert(0u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_zero_frames_still(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_zero_frames_still(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_actl_zero_frames_still().subrange(41,49))==Err((Error::InvalidAnimationControl,true))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_zero_frames_still().subrange(41,49))==Err((Error::InvalidAnimationControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_actl_zero_frames_still(bytes_apng_bad_actl_zero_frames_still());
}
fn walk_apng_bad_actl_zero_frames_still() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,137u8,77u8,192u8,16u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_zero_frames_still());model_apng_bad_actl_zero_frames_still();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidAnimationControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_after_idat(b: Seq<u8>)
    requires b.len()==100,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:76nat,len:8nat,next:88nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(76,84),0)==1,
        crate::model::be32(b.subrange(76,84),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(76,84))==Ok((1u32,0u32)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}

// apng-bad/actl-after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_actl_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_after_idat()
    ensures m::MWF_p(bytes_apng_bad_actl_after_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_actl_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_after_idat(),16),height:crate::model::be32(bytes_apng_bad_actl_after_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_after_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_after_idat(),16),height:crate::model::be32(bytes_apng_bad_actl_after_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_after_idat(),68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:76nat,len:8nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_after_idat(),68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:76nat,len:8nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_after_idat().subrange(76,84),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_after_idat().subrange(76,84),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_after_idat(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_after_idat(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_after_idat(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_after_idat(),20)==3u32) by (compute);
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::actl(bytes_apng_bad_actl_after_idat().subrange(76,84))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_after_idat().subrange(76,84))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_actl_after_idat(bytes_apng_bad_actl_after_idat());
}
fn walk_apng_bad_actl_after_idat() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_after_idat());model_apng_bad_actl_after_idat();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_duplicate(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:61nat,len:8nat,next:73nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,69),0)==1,
        crate::model::be32(b.subrange(61,69),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:81nat,len:23nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::actl(b.subrange(61,69))==Ok((1u32,0u32)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()));
}

// apng-bad/actl-duplicate; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_actl_duplicate() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_duplicate()
    ensures m::MWF_p(bytes_apng_bad_actl_duplicate(),m::strict())==m::Outcome::Error(Error::DuplicateChunk,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_actl_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_duplicate(),16),height:crate::model::be32(bytes_apng_bad_actl_duplicate(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_duplicate())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_duplicate(),16),height:crate::model::be32(bytes_apng_bad_actl_duplicate(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_duplicate(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_duplicate(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:61nat,len:8nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_duplicate(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:61nat,len:8nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate().subrange(61,69),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate().subrange(61,69),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_duplicate(),73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:81nat,len:23nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_duplicate(),73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:81nat,len:23nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_duplicate(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_duplicate(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_duplicate(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_actl_duplicate().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_duplicate().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::actl(bytes_apng_bad_actl_duplicate().subrange(61,69))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_duplicate().subrange(61,69))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_actl_duplicate(bytes_apng_bad_actl_duplicate());
}
fn walk_apng_bad_actl_duplicate() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_duplicate());model_apng_bad_actl_duplicate();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::DuplicateChunk))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_len_4(b: Seq<u8>)
    requires b.len()==96,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,45),0)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:23nat,next:84nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,84nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:92nat,len:0nat,next:96nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,45))==Err((Error::InvalidChunkLength,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()));
}

// apng-bad/actl-len-4; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_actl_len_4() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,4u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,201u8,33u8,214u8,157u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_len_4()
    ensures m::MWF_p(bytes_apng_bad_actl_len_4(),m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_actl_len_4())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_len_4(),16),height:crate::model::be32(bytes_apng_bad_actl_len_4(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_len_4())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_len_4(),16),height:crate::model::be32(bytes_apng_bad_actl_len_4(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_len_4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_len_4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_len_4().subrange(41,45),0)==0) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_len_4(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:23nat,next:84nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_len_4(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:57nat,len:23nat,next:84nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_len_4(),84nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:92nat,len:0nat,next:96nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_len_4(),84nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:92nat,len:0nat,next:96nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_len_4(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_len_4(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_actl_len_4().subrange(41,45))==Err((Error::InvalidChunkLength,true))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_len_4().subrange(41,45))==Err((Error::InvalidChunkLength,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_actl_len_4(bytes_apng_bad_actl_len_4());
}
fn walk_apng_bad_actl_len_4() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,4u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,201u8,33u8,214u8,157u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_len_4());model_apng_bad_actl_len_4();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidChunkLength))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_without_actl(b: Seq<u8>)
    requires b.len()==118,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,67),0)==0,
        crate::model::be32(b.subrange(41,67),4)==4,
        crate::model::be32(b.subrange(41,67),8)==3,
        crate::model::be32(b.subrange(41,67),12)==0,
        crate::model::be32(b.subrange(41,67),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::fctl(b.subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()));
}

// apng-bad/fctl-without-actl; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_without_actl() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_without_actl()
    ensures m::MWF_p(bytes_apng_bad_fctl_without_actl(),m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_without_actl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_without_actl(),16),height:crate::model::be32(bytes_apng_bad_fctl_without_actl(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_without_actl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_without_actl(),16),height:crate::model::be32(bytes_apng_bad_fctl_without_actl(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl().subrange(41,67),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl().subrange(41,67),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl().subrange(41,67),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl().subrange(41,67),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl().subrange(41,67),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl(),20)==3u32) by (compute);
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_without_actl().subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_without_actl().subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_fctl_without_actl(bytes_apng_bad_fctl_without_actl());
}
fn walk_apng_bad_fctl_without_actl() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_without_actl());model_apng_bad_fctl_without_actl();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingAnimationControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_without_actl_still(b: Seq<u8>)
    requires b.len()==118,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,67),0)==0,
        crate::model::be32(b.subrange(41,67),4)==4,
        crate::model::be32(b.subrange(41,67),8)==3,
        crate::model::be32(b.subrange(41,67),12)==0,
        crate::model::be32(b.subrange(41,67),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::fctl(b.subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true)),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MW_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()));
}

// apng-bad/fctl-without-actl-still; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_without_actl_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_without_actl_still()
    ensures m::MW_p(bytes_apng_bad_fctl_without_actl_still(),m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MW_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_without_actl_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),16),height:crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_without_actl_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),16),height:crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:79nat,len:23nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_without_actl_still(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_without_actl_still(),20)==3u32) by (compute);
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_without_actl_still().subrange(41,67),false,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_fctl_without_actl_still(bytes_apng_bad_fctl_without_actl_still());
}
fn walk_apng_bad_fctl_without_actl_still() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_without_actl_still());model_apng_bad_fctl_without_actl_still();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingAnimationControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fdat_without_actl(b: Seq<u8>)
    requires b.len()==112,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:76nat,len:20nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(76,96),0)==0,
        crate::model::be32(b.subrange(76,96),4)==2023514976,
        crate::model::be32(b.subrange(76,96),8)==4161765772,
        crate::model::be32(b.subrange(76,96),12)==1611270144,
        crate::model::be32(b.subrange(76,96),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::fdat(b.subrange(76,96),false,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()));
}

// apng-bad/fdat-without-actl; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fdat_without_actl() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,0u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,129u8,202u8,86u8,75u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fdat_without_actl()
    ensures m::MWF_p(bytes_apng_bad_fdat_without_actl(),m::strict())==m::Outcome::Error(Error::MissingAnimationControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fdat_without_actl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_without_actl(),16),height:crate::model::be32(bytes_apng_bad_fdat_without_actl(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fdat_without_actl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_without_actl(),16),height:crate::model::be32(bytes_apng_bad_fdat_without_actl(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:23nat,next:68nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:76nat,len:20nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),68nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:76nat,len:20nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl().subrange(76,96),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl().subrange(76,96),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl().subrange(76,96),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl().subrange(76,96),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl().subrange(76,96),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_without_actl(),100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_without_actl(),20)==3u32) by (compute);
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fdat_without_actl().subrange(76,96),false,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fdat_without_actl().subrange(76,96),false,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::MissingAnimationControl,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_fdat_without_actl(bytes_apng_bad_fdat_without_actl());
}
fn walk_apng_bad_fdat_without_actl() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,0u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,129u8,202u8,86u8,75u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fdat_without_actl());model_apng_bad_fdat_without_actl();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::MissingAnimationControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_no_fctl_at_all(b: Seq<u8>)
    requires b.len()==100,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::FrameCountMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::FrameCountMismatch,m::no_events()));
}

// apng-bad/no-fctl-at-all; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_no_fctl_at_all() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_no_fctl_at_all()
    ensures m::MWF_p(bytes_apng_bad_no_fctl_at_all(),m::strict())==m::Outcome::Error(Error::FrameCountMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_no_fctl_at_all())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_no_fctl_at_all(),16),height:crate::model::be32(bytes_apng_bad_no_fctl_at_all(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_no_fctl_at_all())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_no_fctl_at_all(),16),height:crate::model::be32(bytes_apng_bad_no_fctl_at_all(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_no_fctl_at_all().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_no_fctl_at_all().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_no_fctl_at_all(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_no_fctl_at_all(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_no_fctl_at_all(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_no_fctl_at_all().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_no_fctl_at_all().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_no_fctl_at_all(bytes_apng_bad_no_fctl_at_all());
}
fn walk_apng_bad_no_fctl_at_all() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_no_fctl_at_all());model_apng_bad_no_fctl_at_all();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::FrameCountMismatch))));
}

// apng-bad/seq-starts-at-1; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_seq_starts_at_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,87u8,0u8,230u8,47u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,5u8,194u8,15u8,233u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_seq_starts_at_1()
    ensures m::MWF_p(bytes_apng_bad_seq_starts_at_1(),m::strict())==m::Outcome::Error(Error::SequenceError,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_seq_starts_at_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_seq_starts_at_1(),16),height:crate::model::be32(bytes_apng_bad_seq_starts_at_1(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_seq_starts_at_1())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_seq_starts_at_1(),16),height:crate::model::be32(bytes_apng_bad_seq_starts_at_1(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(61,87),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(134,160),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(134,160),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(134,160),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(134,160),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(134,160),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(172,192),0)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(172,192),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(172,192),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(172,192),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1().subrange(172,192),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_seq_starts_at_1(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_seq_starts_at_1(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_seq_starts_at_1().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_seq_starts_at_1().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_seq_starts_at_1().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_seq_starts_at_1().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_seq_starts_at_1().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_seq_starts_at_1().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_seq_starts_at_1().subrange(172,192),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_seq_starts_at_1().subrange(172,192),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(2u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(2u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        first_control_sequence_error(bytes_apng_bad_seq_starts_at_1(),crate::model::mh(bytes_apng_bad_seq_starts_at_1()).unwrap().0);
}
fn walk_apng_bad_seq_starts_at_1() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,87u8,0u8,230u8,47u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,5u8,194u8,15u8,233u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_seq_starts_at_1());model_apng_bad_seq_starts_at_1();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::SequenceError))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fdat_before_fctl(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:96nat,len:20nat,next:120nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,116),0)==0,
        crate::model::be32(b.subrange(96,116),4)==2023514976,
        crate::model::be32(b.subrange(96,116),8)==4161765772,
        crate::model::be32(b.subrange(96,116),12)==1611270144,
        crate::model::be32(b.subrange(96,116),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,120nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:128nat,len:26nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(128,154),0)==1,
        crate::model::be32(b.subrange(128,154),4)==2,
        crate::model::be32(b.subrange(128,154),8)==2,
        crate::model::be32(b.subrange(128,154),12)==0,
        crate::model::be32(b.subrange(128,154),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fdat(b.subrange(96,116),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::UnexpectedFrameData,true)),
        crate::apng_model::fctl(b.subrange(128,154),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
}

// apng-bad/fdat-before-fctl; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fdat_before_fctl() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,0u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,129u8,202u8,86u8,75u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fdat_before_fctl()
    ensures m::MWF_p(bytes_apng_bad_fdat_before_fctl(),m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fdat_before_fctl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_before_fctl(),16),height:crate::model::be32(bytes_apng_bad_fdat_before_fctl(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fdat_before_fctl())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_before_fctl(),16),height:crate::model::be32(bytes_apng_bad_fdat_before_fctl(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:96nat,len:20nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:96nat,len:20nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(96,116),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(96,116),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(96,116),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(96,116),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(96,116),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),120nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:128nat,len:26nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),120nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:128nat,len:26nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(128,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(128,154),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(128,154),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(128,154),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl().subrange(128,154),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_fctl(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_fctl(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fdat_before_fctl().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fdat_before_fctl().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fdat_before_fctl().subrange(96,116),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::UnexpectedFrameData,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fdat_before_fctl().subrange(96,116),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::UnexpectedFrameData,true))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fdat_before_fctl().subrange(128,154),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fdat_before_fctl().subrange(128,154),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fdat_before_fctl(bytes_apng_bad_fdat_before_fctl());
}
fn walk_apng_bad_fdat_before_fctl() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,0u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,129u8,202u8,86u8,75u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fdat_before_fctl());model_apng_bad_fdat_before_fctl();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnexpectedFrameData))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fdat_for_idat_frame(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==4,
        crate::model::be32(b.subrange(61,87),8)==3,
        crate::model::be32(b.subrange(61,87),12)==0,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        let ghost s0=m::initial(crate::model::mh(b).unwrap().0,33,m::no_events());
        let ghost s1=match m::step(b,m::strict(),true,false,s0) {m::Step::Next(next)=>next,_=>s0};
        assert(m::step(b,m::strict(),true,false,s0)==m::Step::Next(s1));
        m::step_progress(b,m::strict(),true,false,s0);
        let ghost s2=match m::step(b,m::strict(),true,false,s1) {m::Step::Next(next)=>next,_=>s1};
        assert(m::step(b,m::strict(),true,false,s1)==m::Step::Next(s2));
        m::step_progress(b,m::strict(),true,false,s1);
        let ghost s3=match m::step(b,m::strict(),true,false,s2) {m::Step::Next(next)=>next,_=>s2};
        assert(m::step(b,m::strict(),true,false,s2)==m::Step::Next(s3));
        m::step_progress(b,m::strict(),true,false,s2);
        assert(m::step(b,m::strict(),true,false,s3)==m::Step::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),true,false,s3);
        assert(m::drive(b,m::strict(),true,false,s3)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),true,false,s2);
        assert(m::drive(b,m::strict(),true,false,s2)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),true,false,s1);
        assert(m::drive(b,m::strict(),true,false,s1)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),true,false,s0);
        assert(m::drive(b,m::strict(),true,false,s0)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
}

// apng-bad/fdat-for-idat-frame; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fdat_for_idat_frame() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fdat_for_idat_frame()
    ensures m::MWF_p(bytes_apng_bad_fdat_for_idat_frame(),m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fdat_for_idat_frame())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),16),height:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fdat_for_idat_frame())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),16),height:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fdat_for_idat_frame().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fdat_for_idat_frame().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fdat_for_idat_frame().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fdat_for_idat_frame().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
        }
        symbolic_apng_bad_fdat_for_idat_frame(bytes_apng_bad_fdat_for_idat_frame());
}
fn walk_apng_bad_fdat_for_idat_frame() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fdat_for_idat_frame());model_apng_bad_fdat_for_idat_frame();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnexpectedFrameData))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fdat_for_idat_frame_still(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==4,
        crate::model::be32(b.subrange(61,87),8)==3,
        crate::model::be32(b.subrange(61,87),12)==0,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true)),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
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
        assert(m::step(b,m::strict(),false,false,s3)==m::Step::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),false,false,s3);
        assert(m::drive(b,m::strict(),false,false,s3)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),false,false,s2);
        assert(m::drive(b,m::strict(),false,false,s2)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),false,false,s1);
        assert(m::drive(b,m::strict(),false,false,s1)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        m::drive_step(b,m::strict(),false,false,s0);
        assert(m::drive(b,m::strict(),false,false,s0)==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
        assert(m::MW_p(b,m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()));
}

// apng-bad/fdat-for-idat-frame-still; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fdat_for_idat_frame_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fdat_for_idat_frame_still()
    ensures m::MW_p(bytes_apng_bad_fdat_for_idat_frame_still(),m::strict())==m::Outcome::Error(Error::UnexpectedFrameData,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MW_p);
        assert(crate::model::mh(bytes_apng_bad_fdat_for_idat_frame_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),16),height:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fdat_for_idat_frame_still())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),16),height:crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_for_idat_frame_still(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_for_idat_frame_still(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fdat_for_idat_frame_still().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fdat_for_idat_frame_still().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fdat_for_idat_frame_still().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fdat_for_idat_frame_still().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::UnexpectedFrameData,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
        }
        symbolic_apng_bad_fdat_for_idat_frame_still(bytes_apng_bad_fdat_for_idat_frame_still());
}
fn walk_apng_bad_fdat_for_idat_frame_still() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fdat_for_idat_frame_still());model_apng_bad_fdat_for_idat_frame_still();}
    let public_result=crate::png::try_decode_still(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::UnexpectedFrameData))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fdat_before_idat(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==4,
        crate::model::be32(b.subrange(61,87),8)==3,
        crate::model::be32(b.subrange(61,87),12)==0,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:99nat,len:20nat,next:123nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(99,119),0)==1,
        crate::model::be32(b.subrange(99,119),4)==2023514976,
        crate::model::be32(b.subrange(99,119),8)==4161765772,
        crate::model::be32(b.subrange(99,119),12)==1611270144,
        crate::model::be32(b.subrange(99,119),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:131nat,len:23nat,next:158nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})),
        crate::apng_model::fdat(b.subrange(99,119),true,true,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}

// apng-bad/fdat-before-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fdat_before_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fdat_before_idat()
    ensures m::MWF_p(bytes_apng_bad_fdat_before_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fdat_before_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_before_idat(),16),height:crate::model::be32(bytes_apng_bad_fdat_before_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fdat_before_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fdat_before_idat(),16),height:crate::model::be32(bytes_apng_bad_fdat_before_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:99nat,len:20nat,next:123nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:99nat,len:20nat,next:123nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(99,119),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(99,119),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(99,119),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(99,119),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat().subrange(99,119),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:131nat,len:23nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:131nat,len:23nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fdat_before_idat(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fdat_before_idat(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fdat_before_idat().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fdat_before_idat().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fdat_before_idat().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fdat_before_idat().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fdat_before_idat().subrange(99,119),true,true,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fdat_before_idat().subrange(99,119),true,true,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(1u32))==Ok(())) by (compute);
        }
        symbolic_apng_bad_fdat_before_idat(bytes_apng_bad_fdat_before_idat());
}
fn walk_apng_bad_fdat_before_idat() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fdat_before_idat());model_apng_bad_fdat_before_idat();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_two_fctl_before_idat(b: Seq<u8>)
    requires b.len()==176,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==2,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==4,
        crate::model::be32(b.subrange(61,87),8)==3,
        crate::model::be32(b.subrange(61,87),12)==0,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:99nat,len:26nat,next:129nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(99,125),0)==1,
        crate::model::be32(b.subrange(99,125),4)==4,
        crate::model::be32(b.subrange(99,125),8)==3,
        crate::model::be32(b.subrange(99,125),12)==0,
        crate::model::be32(b.subrange(99,125),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:137nat,len:23nat,next:164nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})),
        crate::apng_model::fctl(b.subrange(99,125),true,true,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(2u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(2u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()));
}

// apng-bad/two-fctl-before-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_two_fctl_before_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,87u8,0u8,230u8,47u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_two_fctl_before_idat()
    ensures m::MWF_p(bytes_apng_bad_two_fctl_before_idat(),m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_two_fctl_before_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),16),height:crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_two_fctl_before_idat())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),16),height:crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:99nat,len:26nat,next:129nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:99nat,len:26nat,next:129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:137nat,len:23nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:137nat,len:23nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_two_fctl_before_idat(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_two_fctl_before_idat(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_two_fctl_before_idat().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_two_fctl_before_idat().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_two_fctl_before_idat().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),true,true,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_two_fctl_before_idat().subrange(99,125),true,true,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Err((Error::ChunkOutOfOrder,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(2u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},Some(2u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_two_fctl_before_idat(bytes_apng_bad_two_fctl_before_idat());
}
fn walk_apng_bad_two_fctl_before_idat() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,87u8,0u8,230u8,47u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_two_fctl_before_idat());model_apng_bad_two_fctl_before_idat();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::ChunkOutOfOrder))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_len_25(b: Seq<u8>)
    requires b.len()==137,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:25nat,next:90nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,86),0)==0,
        crate::model::be32(b.subrange(61,86),4)==0,
        crate::model::be32(b.subrange(61,86),8)==0,
        crate::model::be32(b.subrange(61,86),12)==0,
        crate::model::be32(b.subrange(61,86),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:23nat,next:125nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,125nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:133nat,len:0nat,next:137nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,86),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidChunkLength,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()));
}

// apng-bad/fctl-len-25; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_len_25() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,25u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,69u8,139u8,99u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_len_25()
    ensures m::MWF_p(bytes_apng_bad_fctl_len_25(),m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_len_25())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_len_25(),16),height:crate::model::be32(bytes_apng_bad_fctl_len_25(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_len_25())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_len_25(),16),height:crate::model::be32(bytes_apng_bad_fctl_len_25(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_len_25(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_len_25(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_len_25(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:25nat,next:90nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_len_25(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:25nat,next:90nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(61,86),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(61,86),4)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(61,86),8)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(61,86),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25().subrange(61,86),16)==0) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_len_25(),90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:23nat,next:125nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_len_25(),90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:23nat,next:125nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_len_25(),125nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:133nat,len:0nat,next:137nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_len_25(),125nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:133nat,len:0nat,next:137nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_len_25(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_len_25().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_len_25().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_len_25().subrange(61,86),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidChunkLength,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_len_25().subrange(61,86),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidChunkLength,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_len_25(bytes_apng_bad_fctl_len_25());
}
fn walk_apng_bad_fctl_len_25() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,25u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,69u8,139u8,99u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_len_25());model_apng_bad_fctl_len_25();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidChunkLength))));
}

// apng-bad/fctl-width-0; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_width_0() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,156u8,224u8,218u8,241u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_width_0()
    ensures m::MWF_p(bytes_apng_bad_fctl_width_0(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert(crate::model::mh(bytes_apng_bad_fctl_width_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_width_0(),16),height:crate::model::be32(bytes_apng_bad_fctl_width_0(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_width_0())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_width_0(),16),height:crate::model::be32(bytes_apng_bad_fctl_width_0(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_width_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_width_0(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_width_0(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_width_0(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_width_0(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_width_0(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(96,122),4)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(96,122),16)==0) by (compute);
        assert((0u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_width_0(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_width_0(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_width_0(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_width_0(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_width_0(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_width_0().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_width_0().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_width_0().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_width_0().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_width_0().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_width_0().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(2u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(2u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        late_zero_width_control(bytes_apng_bad_fctl_width_0(),crate::model::mh(bytes_apng_bad_fctl_width_0()).unwrap().0);
}
fn walk_apng_bad_fctl_width_0() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,156u8,224u8,218u8,241u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_width_0());model_apng_bad_fctl_width_0();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_outside_x(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==3,
        crate::model::be32(b.subrange(96,122),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/fctl-outside-x; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_outside_x() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,113u8,182u8,186u8,1u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_outside_x()
    ensures m::MWF_p(bytes_apng_bad_fctl_outside_x(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_outside_x())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_outside_x(),16),height:crate::model::be32(bytes_apng_bad_fctl_outside_x(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_outside_x())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_outside_x(),16),height:crate::model::be32(bytes_apng_bad_fctl_outside_x(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(96,122),12)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_x(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_x(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_outside_x().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_outside_x().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_outside_x().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_outside_x().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_outside_x().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_outside_x().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_outside_x(bytes_apng_bad_fctl_outside_x());
}
fn walk_apng_bad_fctl_outside_x() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,113u8,182u8,186u8,1u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_outside_x());model_apng_bad_fctl_outside_x();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_outside_y(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==0,
        crate::model::be32(b.subrange(96,122),16)==2,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/fctl-outside-y; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_outside_y() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,10u8,0u8,0u8,127u8,203u8,205u8,41u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_outside_y()
    ensures m::MWF_p(bytes_apng_bad_fctl_outside_y(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_outside_y())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_outside_y(),16),height:crate::model::be32(bytes_apng_bad_fctl_outside_y(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_outside_y())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_outside_y(),16),height:crate::model::be32(bytes_apng_bad_fctl_outside_y(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(96,122),16)==2) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_outside_y(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_outside_y(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_outside_y().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_outside_y().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_outside_y().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_outside_y().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_outside_y().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_outside_y().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_outside_y(bytes_apng_bad_fctl_outside_y());
}
fn walk_apng_bad_fctl_outside_y() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,10u8,0u8,0u8,127u8,203u8,205u8,41u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_outside_y());model_apng_bad_fctl_outside_y();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_offset_overflow_u32(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==4294967295,
        crate::model::be32(b.subrange(96,122),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/fctl-offset-overflow-u32; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_offset_overflow_u32() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,198u8,16u8,90u8,56u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_offset_overflow_u32()
    ensures m::MWF_p(bytes_apng_bad_fctl_offset_overflow_u32(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_offset_overflow_u32())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),16),height:crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_offset_overflow_u32())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),16),height:crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),12)==4294967295) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_offset_overflow_u32(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_offset_overflow_u32(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_offset_overflow_u32().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_offset_overflow_u32().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_offset_overflow_u32().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_offset_overflow_u32().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_offset_overflow_u32(bytes_apng_bad_fctl_offset_overflow_u32());
}
fn walk_apng_bad_fctl_offset_overflow_u32() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,198u8,16u8,90u8,56u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_offset_overflow_u32());model_apng_bad_fctl_offset_overflow_u32();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_dispose_3(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==0,
        crate::model::be32(b.subrange(96,122),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/fctl-dispose-3; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_dispose_3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,3u8,0u8,195u8,121u8,143u8,195u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_dispose_3()
    ensures m::MWF_p(bytes_apng_bad_fctl_dispose_3(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_dispose_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_dispose_3(),16),height:crate::model::be32(bytes_apng_bad_fctl_dispose_3(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_dispose_3())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_dispose_3(),16),height:crate::model::be32(bytes_apng_bad_fctl_dispose_3(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_dispose_3(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_dispose_3(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_dispose_3().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_dispose_3().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_dispose_3().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_dispose_3().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_dispose_3().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_dispose_3().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_dispose_3(bytes_apng_bad_fctl_dispose_3());
}
fn walk_apng_bad_fctl_dispose_3() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,3u8,0u8,195u8,121u8,143u8,195u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_dispose_3());model_apng_bad_fctl_dispose_3();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_blend_2(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==0,
        crate::model::be32(b.subrange(96,122),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/fctl-blend-2; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_blend_2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,2u8,6u8,90u8,189u8,44u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_blend_2()
    ensures m::MWF_p(bytes_apng_bad_fctl_blend_2(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_blend_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_blend_2(),16),height:crate::model::be32(bytes_apng_bad_fctl_blend_2(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_blend_2())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_blend_2(),16),height:crate::model::be32(bytes_apng_bad_fctl_blend_2(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_blend_2(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_blend_2(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_blend_2().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_blend_2().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_blend_2().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_blend_2().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_blend_2().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_blend_2().subrange(134,154),true,false,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::SequenceError,true))) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_fctl_blend_2(bytes_apng_bad_fctl_blend_2());
}
fn walk_apng_bad_fctl_blend_2() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,2u8,6u8,90u8,189u8,44u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_blend_2());model_apng_bad_fctl_blend_2();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_default_fctl_smaller(b: Seq<u8>)
    requires b.len()==138,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==2,
        crate::model::be32(b.subrange(61,87),8)==2,
        crate::model::be32(b.subrange(61,87),12)==0,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/default-fctl-smaller; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_default_fctl_smaller() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,232u8,84u8,220u8,0u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_default_fctl_smaller()
    ensures m::MWF_p(bytes_apng_bad_default_fctl_smaller(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_default_fctl_smaller())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_default_fctl_smaller(),16),height:crate::model::be32(bytes_apng_bad_default_fctl_smaller(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_default_fctl_smaller())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_default_fctl_smaller(),16),height:crate::model::be32(bytes_apng_bad_default_fctl_smaller(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(61,87),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(61,87),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller().subrange(61,87),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_smaller(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_smaller(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_default_fctl_smaller().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_default_fctl_smaller().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_default_fctl_smaller().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_default_fctl_smaller().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_default_fctl_smaller(bytes_apng_bad_default_fctl_smaller());
}
fn walk_apng_bad_default_fctl_smaller() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,232u8,84u8,220u8,0u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_default_fctl_smaller());model_apng_bad_default_fctl_smaller();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_default_fctl_offset(b: Seq<u8>)
    requires b.len()==138,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(61,87),0)==0,
        crate::model::be32(b.subrange(61,87),4)==3,
        crate::model::be32(b.subrange(61,87),8)==3,
        crate::model::be32(b.subrange(61,87),12)==1,
        crate::model::be32(b.subrange(61,87),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((3u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,5);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()));
}

// apng-bad/default-fctl-offset; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_default_fctl_offset() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,71u8,195u8,88u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_default_fctl_offset()
    ensures m::MWF_p(bytes_apng_bad_default_fctl_offset(),m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_default_fctl_offset())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_default_fctl_offset(),16),height:crate::model::be32(bytes_apng_bad_default_fctl_offset(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_default_fctl_offset())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_default_fctl_offset(),16),height:crate::model::be32(bytes_apng_bad_default_fctl_offset(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(61,87),4)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(61,87),12)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset().subrange(61,87),16)==0) by (compute);
        assert((3u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_default_fctl_offset(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_default_fctl_offset(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_default_fctl_offset().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_default_fctl_offset().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_default_fctl_offset().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_default_fctl_offset().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Err((Error::InvalidFrameControl,true))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},Some(1u32))==Err((Error::FrameCountMismatch,true))) by (compute);
        }
        symbolic_apng_bad_default_fctl_offset(bytes_apng_bad_default_fctl_offset());
}
fn walk_apng_bad_default_fctl_offset() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,71u8,195u8,88u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_default_fctl_offset());model_apng_bad_default_fctl_offset();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::InvalidFrameControl))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_fctl_crc(b: Seq<u8>)
    requires b.len()==170,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==1,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:false,synthetic:false})),
        crate::model::be32(b.subrange(96,122),0)==0,
        crate::model::be32(b.subrange(96,122),4)==2,
        crate::model::be32(b.subrange(96,122),8)==2,
        crate::model::be32(b.subrange(96,122),12)==0,
        crate::model::be32(b.subrange(96,122),16)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(134,154),0)==1,
        crate::model::be32(b.subrange(134,154),4)==2023514976,
        crate::model::be32(b.subrange(134,154),8)==4161765772,
        crate::model::be32(b.subrange(134,154),12)==1611270144,
        crate::model::be32(b.subrange(134,154),16)==1071318524,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Ok((1u32,0u32)),
        crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})),
        crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},Some(1u32))==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,6);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
}

// apng-bad/fctl-crc; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_bad_fctl_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_fctl_crc()
    ensures m::MWF_p(bytes_apng_bad_fctl_crc(),m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_fctl_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_crc(),16),height:crate::model::be32(bytes_apng_bad_fctl_crc(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_fctl_crc())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_fctl_crc(),16),height:crate::model::be32(bytes_apng_bad_fctl_crc(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_crc(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(41,49),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(41,49),4)==0) by (compute);
        assert(1u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_crc(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_crc(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_crc(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:false,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_crc(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:false,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_crc(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_crc(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_fctl_crc(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_fctl_crc(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:166nat,len:0nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_fctl_crc(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_fctl_crc(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_fctl_crc().subrange(41,49))==Ok((1u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_fctl_crc().subrange(41,49))==Ok((1u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_bad_fctl_crc().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_bad_fctl_crc().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_bad_fctl_crc().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_bad_fctl_crc().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},Some(1u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},Some(1u32))==Ok(())) by (compute);
        }
        symbolic_apng_bad_fctl_crc(bytes_apng_bad_fctl_crc());
}
fn walk_apng_bad_fctl_crc() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_fctl_crc());model_apng_bad_fctl_crc();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::CrcMismatch))));
}

#[verifier::spinoff_prover]
proof fn symbolic_apng_bad_actl_too_many_frames(b: Seq<u8>)
    requires b.len()==100,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8,color_type:6,interlace:0},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b.subrange(41,49),0)==65537,
        crate::model::be32(b.subrange(41,49),4)==0,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})),
        crate::model::be32(b,16)==4u32,
        crate::model::be32(b,20)==3u32,
        crate::apng_model::actl(b.subrange(41,49))==Err((Error::TooManyFrames,false)),
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::TooManyFrames,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(65537u32>crate::limits::MAX_FRAMES) by (bit_vector);
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
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
        reveal_with_fuel(m::drive,4);
        assert(m::MWF_p(b,m::strict())==m::Outcome::Error(Error::TooManyFrames,m::no_events()));
}

// apng-bad/actl-too-many-frames; fixture sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_apng_bad_actl_too_many_frames() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,1u8,0u8,1u8,0u8,0u8,0u8,0u8,18u8,90u8,226u8,20u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
#[verifier::spinoff_prover]
proof fn model_apng_bad_actl_too_many_frames()
    ensures m::MWF_p(bytes_apng_bad_actl_too_many_frames(),m::strict())==m::Outcome::Error(Error::TooManyFrames,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);
    hide(m::MWF_p);
        assert(crate::model::mh(bytes_apng_bad_actl_too_many_frames())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_too_many_frames(),16),height:crate::model::be32(bytes_apng_bad_actl_too_many_frames(),20),depth:8,color_type:6,interlace:0},33nat))) by {
            reveal(crate::model::mh);
            assert(crate::model::mh(bytes_apng_bad_actl_too_many_frames())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_apng_bad_actl_too_many_frames(),16),height:crate::model::be32(bytes_apng_bad_actl_too_many_frames(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
        }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_too_many_frames().subrange(41,49),0)==65537) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_too_many_frames().subrange(41,49),4)==0) by (compute);
        assert(65537u32>crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_bad_actl_too_many_frames(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:96nat,len:0nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_bad_actl_too_many_frames(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_bad_actl_too_many_frames(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_bad_actl_too_many_frames().subrange(41,49))==Err((Error::TooManyFrames,false))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_bad_actl_too_many_frames().subrange(41,49))==Err((Error::TooManyFrames,false))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},None)==Ok(())) by (compute);
        }
        symbolic_apng_bad_actl_too_many_frames(bytes_apng_bad_actl_too_many_frames());
}
fn walk_apng_bad_actl_too_many_frames() {
    hide(m::MWF_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,1u8,0u8,1u8,0u8,0u8,0u8,0u8,18u8,90u8,226u8,20u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_bad_actl_too_many_frames());model_apng_bad_actl_too_many_frames();}
    let public_result=crate::png::Frames::try_new(&b);
    assert(matches!(public_result,Err(TryError::Format(Error::TooManyFrames))));
}

} // verus!
