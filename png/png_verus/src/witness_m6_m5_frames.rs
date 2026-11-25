//! Complete frozen frame results and real public iterator calls.
use vstd::prelude::*;
use crate::{frame_model as fm,frame_equivalence as fe,iterator_data_model as id,
    document_model as d,png::Error,alloc_guard::{TryError,AllocSite},policy::STRICT};
verus! {
// apng/default-is-frame0-dispose-bg-blend-over
pub open spec fn e6_bytes_apng_default_is_frame0_dispose_bg_blend_over() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,32u8,227u8,219u8,236u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,0u8,52u8,100u8,47u8,237u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,1u8,0u8,13u8,197u8,211u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,100u8,0u8,1u8,192u8,66u8,10u8,152u8,0u8,0u8,0u8,7u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,207u8,160u8,248u8,174u8,0u8,0u8,0u8,15u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,120u8,231u8,82u8,174u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_default_is_frame0_dispose_bg_blend_over_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn e6_expected_apng_default_is_frame0_dispose_bg_blend_over_1() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn e6_expected_apng_default_is_frame0_dispose_bg_blend_over_2() -> Seq<u8> {seq![127u8,0u8,128u8,255u8,127u8,0u8,128u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_default_is_frame0_dispose_bg_blend_over() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_default_is_frame0_dispose_bg_blend_over_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((5u16,0u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_default_is_frame0_dispose_bg_blend_over_1(),rect:crate::composite_model::rect(1u32,1u32,2u32,2u32),delay:Some((1u16,10u16)),dispose:1u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_default_is_frame0_dispose_bg_blend_over_2(),rect:crate::composite_model::rect(0u32,0u32,2u32,1u32),delay:Some((3u16,100u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_default_is_frame0_dispose_bg_blend_over(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(2u32) && value.info.num_plays==Some(2u32)
    && value.frames==expected_frames_apng_default_is_frame0_dispose_bg_blend_over() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_default_is_frame0_dispose_bg_blend_over()
    ensures match fm::MFr(e6_bytes_apng_default_is_frame0_dispose_bg_blend_over()) {fm::Outcome::Done(value)=>expected_value_apng_default_is_frame0_dispose_bg_blend_over(value),_=>false},
        fe::requests(e6_bytes_apng_default_is_frame0_dispose_bg_blend_over()).is_prefix_of(id::requests_p(e6_bytes_apng_default_is_frame0_dispose_bg_blend_over(),STRICT,4nat)),
{
    crate::witness_m5_frames::e6_complete_apng_default_is_frame0_dispose_bg_blend_over();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_default_is_frame0_dispose_bg_blend_over() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_default_is_frame0_dispose_bg_blend_over(),STRICT,4nat),result.0),
        match fm::MFr(e6_bytes_apng_default_is_frame0_dispose_bg_blend_over()) {fm::Outcome::Done(value)=>expected_value_apng_default_is_frame0_dispose_bg_blend_over(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,32u8,227u8,219u8,236u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,0u8,52u8,100u8,47u8,237u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,1u8,0u8,13u8,197u8,211u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,100u8,0u8,1u8,192u8,66u8,10u8,152u8,0u8,0u8,0u8,7u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,207u8,160u8,248u8,174u8,0u8,0u8,0u8,15u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,120u8,231u8,82u8,174u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_default_is_frame0_dispose_bg_blend_over());model_complete_apng_default_is_frame0_dispose_bg_blend_over();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,3usize,Ghost(value))
}
// apng/default-not-in-animation
pub open spec fn e6_bytes_apng_default_not_in_animation() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,228u8,1u8,51u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,1u8,0u8,0u8,215u8,25u8,34u8,153u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,241u8,195u8,124u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_default_not_in_animation_0() -> Seq<u8> {seq![0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn e6_expected_apng_default_not_in_animation_1() -> Seq<u8> {seq![0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,128u8,0u8,0u8,255u8,128u8]}
pub open spec fn expected_frames_apng_default_not_in_animation() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_default_not_in_animation_0(),rect:crate::composite_model::rect(0u32,0u32,2u32,2u32),delay:Some((1u16,1u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_default_not_in_animation_1(),rect:crate::composite_model::rect(2u32,2u32,2u32,1u32),delay:Some((1u16,1u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_default_not_in_animation(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_default_not_in_animation() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_default_not_in_animation()
    ensures match fm::MFr(e6_bytes_apng_default_not_in_animation()) {fm::Outcome::Done(value)=>expected_value_apng_default_not_in_animation(value),_=>false},
        fe::requests(e6_bytes_apng_default_not_in_animation()).is_prefix_of(id::requests_p(e6_bytes_apng_default_not_in_animation(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_default_not_in_animation();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_default_not_in_animation() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_default_not_in_animation(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_default_not_in_animation()) {fm::Outcome::Done(value)=>expected_value_apng_default_not_in_animation(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,228u8,1u8,51u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,1u8,0u8,0u8,215u8,25u8,34u8,153u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,241u8,195u8,124u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_default_not_in_animation());model_complete_apng_default_not_in_animation();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/dispose-previous-first-frame-and-later
pub open spec fn e6_bytes_apng_dispose_previous_first_frame_and_later() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,206u8,237u8,186u8,192u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,2u8,0u8,242u8,16u8,129u8,152u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,2u8,0u8,77u8,68u8,187u8,183u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,1u8,0u8,1u8,147u8,103u8,132u8,71u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,106u8,194u8,115u8,68u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_dispose_previous_first_frame_and_later_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn e6_expected_apng_dispose_previous_first_frame_and_later_1() -> Seq<u8> {seq![0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn e6_expected_apng_dispose_previous_first_frame_and_later_2() -> Seq<u8> {seq![0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,255u8,128u8,0u8,0u8,255u8,128u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn expected_frames_apng_dispose_previous_first_frame_and_later() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_dispose_previous_first_frame_and_later_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,1u16)),dispose:2u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_dispose_previous_first_frame_and_later_1(),rect:crate::composite_model::rect(0u32,0u32,2u32,2u32),delay:Some((1u16,1u16)),dispose:2u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_dispose_previous_first_frame_and_later_2(),rect:crate::composite_model::rect(1u32,1u32,2u32,1u32),delay:Some((1u16,1u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_dispose_previous_first_frame_and_later(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_dispose_previous_first_frame_and_later() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_dispose_previous_first_frame_and_later()
    ensures match fm::MFr(e6_bytes_apng_dispose_previous_first_frame_and_later()) {fm::Outcome::Done(value)=>expected_value_apng_dispose_previous_first_frame_and_later(value),_=>false},
        fe::requests(e6_bytes_apng_dispose_previous_first_frame_and_later()).is_prefix_of(id::requests_p(e6_bytes_apng_dispose_previous_first_frame_and_later(),STRICT,4nat)),
{
    crate::witness_m5_frames::e6_complete_apng_dispose_previous_first_frame_and_later();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_dispose_previous_first_frame_and_later() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_dispose_previous_first_frame_and_later(),STRICT,4nat),result.0),
        match fm::MFr(e6_bytes_apng_dispose_previous_first_frame_and_later()) {fm::Outcome::Done(value)=>expected_value_apng_dispose_previous_first_frame_and_later(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,206u8,237u8,186u8,192u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,2u8,0u8,242u8,16u8,129u8,152u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,2u8,0u8,77u8,68u8,187u8,183u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,1u8,0u8,1u8,147u8,103u8,132u8,71u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,106u8,194u8,115u8,68u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_dispose_previous_first_frame_and_later());model_complete_apng_dispose_previous_first_frame_and_later();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,3usize,Ghost(value))
}
// apng/blend-over-arithmetic
pub open spec fn e6_bytes_apng_blend_over_arithmetic() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,63u8,182u8,92u8,88u8,0u8,0u8,0u8,32u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,248u8,197u8,234u8,203u8,0u8,196u8,255u8,129u8,184u8,129u8,17u8,72u8,48u8,51u8,48u8,48u8,28u8,5u8,226u8,73u8,0u8,124u8,99u8,8u8,11u8,28u8,215u8,217u8,211u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,0u8,1u8,194u8,222u8,107u8,83u8,0u8,0u8,0u8,33u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,224u8,58u8,33u8,199u8,0u8,196u8,140u8,64u8,92u8,15u8,34u8,254u8,237u8,115u8,114u8,117u8,178u8,252u8,177u8,160u8,3u8,0u8,98u8,147u8,9u8,32u8,249u8,187u8,207u8,86u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_blend_over_arithmetic_0() -> Seq<u8> {seq![250u8,5u8,77u8,0u8,250u8,5u8,77u8,255u8,250u8,5u8,77u8,128u8,0u8,0u8,0u8,0u8,250u8,5u8,77u8,3u8,250u8,5u8,77u8,200u8,250u8,5u8,77u8,90u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn e6_expected_apng_blend_over_arithmetic_1() -> Seq<u8> {seq![250u8,5u8,77u8,0u8,250u8,5u8,77u8,255u8,250u8,5u8,77u8,128u8,0u8,0u8,0u8,0u8,250u8,5u8,77u8,3u8,250u8,5u8,77u8,200u8,247u8,7u8,76u8,91u8,10u8,200u8,30u8,127u8,0u8,0u8,0u8,0u8,10u8,200u8,30u8,254u8,200u8,10u8,99u8,64u8,1u8,2u8,3u8,200u8]}
pub open spec fn expected_frames_apng_blend_over_arithmetic() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_blend_over_arithmetic_0(),rect:crate::composite_model::rect(0u32,0u32,3u32,2u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_blend_over_arithmetic_1(),rect:crate::composite_model::rect(1u32,1u32,3u32,2u32),delay:Some((1u16,10u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_blend_over_arithmetic(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_blend_over_arithmetic() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_blend_over_arithmetic()
    ensures match fm::MFr(e6_bytes_apng_blend_over_arithmetic()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_arithmetic(value),_=>false},
        fe::requests(e6_bytes_apng_blend_over_arithmetic()).is_prefix_of(id::requests_p(e6_bytes_apng_blend_over_arithmetic(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_blend_over_arithmetic();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_blend_over_arithmetic() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_blend_over_arithmetic(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_blend_over_arithmetic()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_arithmetic(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,63u8,182u8,92u8,88u8,0u8,0u8,0u8,32u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,248u8,197u8,234u8,203u8,0u8,196u8,255u8,129u8,184u8,129u8,17u8,72u8,48u8,51u8,48u8,48u8,28u8,5u8,226u8,73u8,0u8,124u8,99u8,8u8,11u8,28u8,215u8,217u8,211u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,0u8,1u8,194u8,222u8,107u8,83u8,0u8,0u8,0u8,33u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,224u8,58u8,33u8,199u8,0u8,196u8,140u8,64u8,92u8,15u8,34u8,254u8,237u8,115u8,114u8,117u8,178u8,252u8,177u8,160u8,3u8,0u8,98u8,147u8,9u8,32u8,249u8,187u8,207u8,86u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_blend_over_arithmetic());model_complete_apng_blend_over_arithmetic();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/blend-over-transparent-source
pub open spec fn e6_bytes_apng_blend_over_transparent_source() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,41u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,196u8,13u8,64u8,204u8,248u8,31u8,8u8,24u8,65u8,2u8,12u8,12u8,12u8,141u8,32u8,252u8,245u8,245u8,195u8,127u8,76u8,12u8,104u8,0u8,0u8,124u8,86u8,12u8,48u8,25u8,48u8,82u8,42u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,40u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,145u8,98u8,196u8,0u8,195u8,32u8,192u8,200u8,0u8,5u8,255u8,255u8,255u8,7u8,211u8,76u8,32u8,6u8,35u8,19u8,51u8,3u8,19u8,51u8,11u8,24u8,3u8,0u8,47u8,251u8,10u8,48u8,80u8,149u8,138u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_blend_over_transparent_source_0() -> Seq<u8> {seq![10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8,10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8,10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8]}
pub open spec fn e6_expected_apng_blend_over_transparent_source_1() -> Seq<u8> {seq![10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8,10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8,10u8,20u8,30u8,255u8,10u8,20u8,30u8,128u8,10u8,20u8,30u8,1u8,255u8,255u8,255u8,255u8]}
pub open spec fn expected_frames_apng_blend_over_transparent_source() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_blend_over_transparent_source_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_blend_over_transparent_source_1(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_blend_over_transparent_source(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_blend_over_transparent_source() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_blend_over_transparent_source()
    ensures match fm::MFr(e6_bytes_apng_blend_over_transparent_source()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_transparent_source(value),_=>false},
        fe::requests(e6_bytes_apng_blend_over_transparent_source()).is_prefix_of(id::requests_p(e6_bytes_apng_blend_over_transparent_source(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_blend_over_transparent_source();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_blend_over_transparent_source() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_blend_over_transparent_source(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_blend_over_transparent_source()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_transparent_source(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,41u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,196u8,13u8,64u8,204u8,248u8,31u8,8u8,24u8,65u8,2u8,12u8,12u8,12u8,141u8,32u8,252u8,245u8,245u8,195u8,127u8,76u8,12u8,104u8,0u8,0u8,124u8,86u8,12u8,48u8,25u8,48u8,82u8,42u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,40u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,145u8,98u8,196u8,0u8,195u8,32u8,192u8,200u8,0u8,5u8,255u8,255u8,255u8,7u8,211u8,76u8,32u8,6u8,35u8,19u8,51u8,3u8,19u8,51u8,11u8,24u8,3u8,0u8,47u8,251u8,10u8,48u8,80u8,149u8,138u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_blend_over_transparent_source());model_complete_apng_blend_over_transparent_source();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/blend-over-transparent-destination
pub open spec fn e6_bytes_apng_blend_over_transparent_destination() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,45u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,74u8,102u8,96u8,16u8,73u8,98u8,103u8,144u8,75u8,228u8,99u8,208u8,72u8,16u8,101u8,96u8,52u8,138u8,151u8,97u8,224u8,250u8,207u8,14u8,199u8,76u8,26u8,127u8,100u8,24u8,144u8,49u8,0u8,232u8,60u8,10u8,245u8,165u8,218u8,222u8,165u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,56u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,97u8,196u8,192u8,120u8,220u8,152u8,153u8,233u8,152u8,9u8,155u8,195u8,81u8,83u8,206u8,122u8,198u8,35u8,102u8,60u8,13u8,255u8,25u8,153u8,25u8,129u8,216u8,29u8,136u8,77u8,153u8,254u8,176u8,240u8,212u8,1u8,113u8,3u8,16u8,239u8,0u8,98u8,70u8,0u8,136u8,19u8,15u8,179u8,166u8,184u8,187u8,59u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_blend_over_transparent_destination_0() -> Seq<u8> {seq![10u8,99u8,0u8,0u8,20u8,98u8,7u8,0u8,30u8,97u8,14u8,0u8,40u8,96u8,21u8,0u8,50u8,95u8,28u8,0u8,60u8,94u8,35u8,0u8,70u8,93u8,42u8,0u8,80u8,92u8,49u8,0u8,90u8,91u8,56u8,0u8,100u8,90u8,63u8,0u8,110u8,89u8,70u8,0u8,120u8,88u8,77u8,0u8]}
pub open spec fn e6_expected_apng_blend_over_transparent_destination_1() -> Seq<u8> {seq![200u8,50u8,0u8,1u8,199u8,51u8,3u8,2u8,198u8,52u8,6u8,64u8,197u8,53u8,9u8,127u8,196u8,54u8,12u8,128u8,195u8,55u8,15u8,129u8,194u8,56u8,18u8,200u8,193u8,57u8,21u8,253u8,192u8,58u8,24u8,254u8,191u8,59u8,27u8,1u8,190u8,60u8,30u8,128u8,189u8,61u8,33u8,254u8]}
pub open spec fn expected_frames_apng_blend_over_transparent_destination() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_blend_over_transparent_destination_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_blend_over_transparent_destination_1(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_blend_over_transparent_destination(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_blend_over_transparent_destination() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_blend_over_transparent_destination()
    ensures match fm::MFr(e6_bytes_apng_blend_over_transparent_destination()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_transparent_destination(value),_=>false},
        fe::requests(e6_bytes_apng_blend_over_transparent_destination()).is_prefix_of(id::requests_p(e6_bytes_apng_blend_over_transparent_destination(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_blend_over_transparent_destination();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_blend_over_transparent_destination() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_blend_over_transparent_destination(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_blend_over_transparent_destination()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_transparent_destination(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,45u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,74u8,102u8,96u8,16u8,73u8,98u8,103u8,144u8,75u8,228u8,99u8,208u8,72u8,16u8,101u8,96u8,52u8,138u8,151u8,97u8,224u8,250u8,207u8,14u8,199u8,76u8,26u8,127u8,100u8,24u8,144u8,49u8,0u8,232u8,60u8,10u8,245u8,165u8,218u8,222u8,165u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,56u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,97u8,196u8,192u8,120u8,220u8,152u8,153u8,233u8,152u8,9u8,155u8,195u8,81u8,83u8,206u8,122u8,198u8,35u8,102u8,60u8,13u8,255u8,25u8,153u8,25u8,129u8,216u8,29u8,136u8,77u8,153u8,254u8,176u8,240u8,212u8,1u8,113u8,3u8,16u8,239u8,0u8,98u8,70u8,0u8,136u8,19u8,15u8,179u8,166u8,184u8,187u8,59u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_blend_over_transparent_destination());model_complete_apng_blend_over_transparent_destination();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/blend-over-zero-result-alpha
pub open spec fn e6_bytes_apng_blend_over_zero_result_alpha() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,57u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,74u8,102u8,96u8,16u8,73u8,98u8,103u8,144u8,75u8,228u8,99u8,208u8,72u8,16u8,101u8,96u8,52u8,138u8,151u8,97u8,224u8,250u8,207u8,14u8,199u8,76u8,26u8,127u8,100u8,24u8,142u8,44u8,186u8,203u8,176u8,115u8,209u8,85u8,134u8,13u8,75u8,206u8,51u8,0u8,0u8,252u8,30u8,13u8,203u8,206u8,181u8,136u8,102u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,57u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,97u8,196u8,192u8,112u8,220u8,152u8,153u8,225u8,152u8,9u8,27u8,195u8,81u8,83u8,78u8,6u8,198u8,35u8,102u8,60u8,12u8,255u8,25u8,153u8,225u8,152u8,233u8,15u8,11u8,15u8,131u8,237u8,201u8,143u8,12u8,182u8,199u8,223u8,50u8,120u8,92u8,248u8,194u8,0u8,0u8,83u8,128u8,15u8,13u8,88u8,79u8,148u8,62u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_blend_over_zero_result_alpha_0() -> Seq<u8> {seq![10u8,99u8,0u8,0u8,20u8,98u8,7u8,0u8,30u8,97u8,14u8,0u8,40u8,96u8,21u8,0u8,50u8,95u8,28u8,0u8,60u8,94u8,35u8,0u8,70u8,93u8,42u8,0u8,80u8,92u8,49u8,0u8,90u8,91u8,56u8,0u8,0u8,0u8,0u8,0u8,255u8,255u8,255u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn e6_expected_apng_blend_over_zero_result_alpha_1() -> Seq<u8> {seq![10u8,99u8,0u8,0u8,20u8,98u8,7u8,0u8,30u8,97u8,14u8,0u8,40u8,96u8,21u8,0u8,50u8,95u8,28u8,0u8,60u8,94u8,35u8,0u8,70u8,93u8,42u8,0u8,80u8,92u8,49u8,0u8,90u8,91u8,56u8,0u8,0u8,0u8,0u8,0u8,255u8,255u8,255u8,0u8,0u8,0u8,0u8,0u8]}
pub open spec fn expected_frames_apng_blend_over_zero_result_alpha() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_blend_over_zero_result_alpha_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_blend_over_zero_result_alpha_1(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:1u8}]}
pub open spec fn expected_value_apng_blend_over_zero_result_alpha(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_blend_over_zero_result_alpha() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_blend_over_zero_result_alpha()
    ensures match fm::MFr(e6_bytes_apng_blend_over_zero_result_alpha()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_zero_result_alpha(value),_=>false},
        fe::requests(e6_bytes_apng_blend_over_zero_result_alpha()).is_prefix_of(id::requests_p(e6_bytes_apng_blend_over_zero_result_alpha(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_blend_over_zero_result_alpha();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_blend_over_zero_result_alpha() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_blend_over_zero_result_alpha(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_blend_over_zero_result_alpha()) {fm::Outcome::Done(value)=>expected_value_apng_blend_over_zero_result_alpha(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,57u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,224u8,74u8,102u8,96u8,16u8,73u8,98u8,103u8,144u8,75u8,228u8,99u8,208u8,72u8,16u8,101u8,96u8,52u8,138u8,151u8,97u8,224u8,250u8,207u8,14u8,199u8,76u8,26u8,127u8,100u8,24u8,142u8,44u8,186u8,203u8,176u8,115u8,209u8,85u8,134u8,13u8,75u8,206u8,51u8,0u8,0u8,252u8,30u8,13u8,203u8,206u8,181u8,136u8,102u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,1u8,86u8,226u8,239u8,132u8,0u8,0u8,0u8,57u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,56u8,97u8,196u8,192u8,112u8,220u8,152u8,153u8,225u8,152u8,9u8,27u8,195u8,81u8,83u8,78u8,6u8,198u8,35u8,102u8,60u8,12u8,255u8,25u8,153u8,225u8,152u8,233u8,15u8,11u8,15u8,131u8,237u8,201u8,143u8,12u8,182u8,199u8,223u8,50u8,120u8,92u8,248u8,194u8,0u8,0u8,83u8,128u8,15u8,13u8,88u8,79u8,148u8,62u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_blend_over_zero_result_alpha());model_complete_apng_blend_over_zero_result_alpha();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/interlaced-subframe
pub open spec fn e6_bytes_apng_interlaced_subframe() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,1u8,195u8,243u8,158u8,80u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,33u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,159u8,17u8,68u8,48u8,129u8,8u8,16u8,102u8,6u8,17u8,13u8,12u8,12u8,13u8,44u8,12u8,48u8,0u8,147u8,130u8,97u8,0u8,3u8,71u8,18u8,249u8,191u8,111u8,158u8,154u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,220u8,176u8,178u8,164u8,0u8,0u8,0u8,40u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,21u8,136u8,129u8,13u8,0u8,48u8,8u8,131u8,112u8,238u8,240u8,237u8,114u8,180u8,105u8,32u8,161u8,8u8,214u8,226u8,137u8,82u8,141u8,120u8,41u8,63u8,91u8,47u8,87u8,54u8,91u8,121u8,16u8,253u8,172u8,226u8,175u8,232u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_interlaced_subframe_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn e6_expected_apng_interlaced_subframe_1() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,0u8,255u8,255u8,255u8,0u8,0u8,255u8,0u8,0u8,255u8,128u8,255u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,128u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_interlaced_subframe() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_interlaced_subframe_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_interlaced_subframe_1(),rect:crate::composite_model::rect(1u32,0u32,3u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_interlaced_subframe(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_interlaced_subframe() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_interlaced_subframe()
    ensures match fm::MFr(e6_bytes_apng_interlaced_subframe()) {fm::Outcome::Done(value)=>expected_value_apng_interlaced_subframe(value),_=>false},
        fe::requests(e6_bytes_apng_interlaced_subframe()).is_prefix_of(id::requests_p(e6_bytes_apng_interlaced_subframe(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_interlaced_subframe();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_interlaced_subframe() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_interlaced_subframe(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_interlaced_subframe()) {fm::Outcome::Done(value)=>expected_value_apng_interlaced_subframe(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,1u8,195u8,243u8,158u8,80u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,33u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,159u8,17u8,68u8,48u8,129u8,8u8,16u8,102u8,6u8,17u8,13u8,12u8,12u8,13u8,44u8,12u8,48u8,0u8,147u8,130u8,97u8,0u8,3u8,71u8,18u8,249u8,191u8,111u8,158u8,154u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,220u8,176u8,178u8,164u8,0u8,0u8,0u8,40u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,21u8,136u8,129u8,13u8,0u8,48u8,8u8,131u8,112u8,238u8,240u8,237u8,114u8,180u8,105u8,32u8,161u8,8u8,214u8,226u8,137u8,82u8,141u8,120u8,41u8,63u8,91u8,47u8,87u8,54u8,91u8,121u8,16u8,253u8,172u8,226u8,175u8,232u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_interlaced_subframe());model_complete_apng_interlaced_subframe();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
// apng/static-file-as-frames
pub open spec fn e6_bytes_apng_static_file_as_frames() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_static_file_as_frames_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_static_file_as_frames() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_static_file_as_frames_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:None,dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_static_file_as_frames(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==None && value.info.num_plays==None
    && value.frames==expected_frames_apng_static_file_as_frames() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_static_file_as_frames()
    ensures match fm::MFr(e6_bytes_apng_static_file_as_frames()) {fm::Outcome::Done(value)=>expected_value_apng_static_file_as_frames(value),_=>false},
        fe::requests(e6_bytes_apng_static_file_as_frames()).is_prefix_of(id::requests_p(e6_bytes_apng_static_file_as_frames(),STRICT,2nat)),
{
    crate::witness_m5_frames::e6_complete_apng_static_file_as_frames();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_static_file_as_frames() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_static_file_as_frames(),STRICT,2nat),result.0),
        match fm::MFr(e6_bytes_apng_static_file_as_frames()) {fm::Outcome::Done(value)=>expected_value_apng_static_file_as_frames(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_static_file_as_frames());model_complete_apng_static_file_as_frames();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,1usize,Ghost(value))
}
// apng/frame1-adler-error-ends
pub open spec fn e6_bytes_apng_frame1_adler_error_ends() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,0u8,225u8,253u8,120u8,187u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_frame1_adler_error_ends_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_frame1_adler_error_ends() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_frame1_adler_error_ends_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_frame1_adler_error_ends(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_frame1_adler_error_ends() && value.end==id::End::Format(Error::AdlerMismatch)
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_frame1_adler_error_ends()
    ensures match fm::MFr(e6_bytes_apng_frame1_adler_error_ends()) {fm::Outcome::Done(value)=>expected_value_apng_frame1_adler_error_ends(value),_=>false},
        fe::requests(e6_bytes_apng_frame1_adler_error_ends()).is_prefix_of(id::requests_p(e6_bytes_apng_frame1_adler_error_ends(),STRICT,2nat)),
{
    crate::witness_m5_frames::e6_complete_apng_frame1_adler_error_ends();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_frame1_adler_error_ends() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_frame1_adler_error_ends(),STRICT,2nat),result.0),
        match fm::MFr(e6_bytes_apng_frame1_adler_error_ends()) {fm::Outcome::Done(value)=>expected_value_apng_frame1_adler_error_ends(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,0u8,225u8,253u8,120u8,187u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_frame1_adler_error_ends());model_complete_apng_frame1_adler_error_ends();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,1usize,Ghost(value))
}
// apng/frame1-truncated-data-ends
pub open spec fn e6_bytes_apng_frame1_truncated_data_ends() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,21u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,138u8,129u8,1u8,0u8,57u8,223u8,5u8,252u8,29u8,220u8,221u8,223u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_frame1_truncated_data_ends_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_frame1_truncated_data_ends() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_frame1_truncated_data_ends_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_frame1_truncated_data_ends(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_frame1_truncated_data_ends() && value.end==id::End::Format(Error::TruncatedImageData)
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_frame1_truncated_data_ends()
    ensures match fm::MFr(e6_bytes_apng_frame1_truncated_data_ends()) {fm::Outcome::Done(value)=>expected_value_apng_frame1_truncated_data_ends(value),_=>false},
        fe::requests(e6_bytes_apng_frame1_truncated_data_ends()).is_prefix_of(id::requests_p(e6_bytes_apng_frame1_truncated_data_ends(),STRICT,2nat)),
{
    crate::witness_m5_frames::e6_complete_apng_frame1_truncated_data_ends();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_frame1_truncated_data_ends() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_frame1_truncated_data_ends(),STRICT,2nat),result.0),
        match fm::MFr(e6_bytes_apng_frame1_truncated_data_ends()) {fm::Outcome::Done(value)=>expected_value_apng_frame1_truncated_data_ends(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,21u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,138u8,129u8,1u8,0u8,57u8,223u8,5u8,252u8,29u8,220u8,221u8,223u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_frame1_truncated_data_ends());model_complete_apng_frame1_truncated_data_ends();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,1usize,Ghost(value))
}
// apng/empty-fdat-payload
pub open spec fn e6_bytes_apng_empty_fdat_payload() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,127u8,48u8,208u8,0u8,0u8,0u8,13u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,0u8,1u8,164u8,1u8,60u8,147u8,139u8,14u8,183u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,193u8,12u8,218u8,4u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_empty_fdat_payload_0() -> Seq<u8> {seq![10u8,20u8,30u8,255u8]}
pub open spec fn expected_frames_apng_empty_fdat_payload() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_empty_fdat_payload_0(),rect:crate::composite_model::rect(0u32,0u32,1u32,1u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_empty_fdat_payload(value: fm::Value) -> bool {
    value.info.header.width==1u32 && value.info.header.height==1u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_empty_fdat_payload() && value.end==id::End::Format(Error::TruncatedZlib)
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_empty_fdat_payload()
    ensures match fm::MFr(e6_bytes_apng_empty_fdat_payload()) {fm::Outcome::Done(value)=>expected_value_apng_empty_fdat_payload(value),_=>false},
        fe::requests(e6_bytes_apng_empty_fdat_payload()).is_prefix_of(id::requests_p(e6_bytes_apng_empty_fdat_payload(),STRICT,2nat)),
{
    crate::witness_m5_frames::e6_complete_apng_empty_fdat_payload();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_empty_fdat_payload() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_empty_fdat_payload(),STRICT,2nat),result.0),
        match fm::MFr(e6_bytes_apng_empty_fdat_payload()) {fm::Outcome::Done(value)=>expected_value_apng_empty_fdat_payload(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,127u8,48u8,208u8,0u8,0u8,0u8,13u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,0u8,1u8,164u8,1u8,60u8,147u8,139u8,14u8,183u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,193u8,12u8,218u8,4u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_empty_fdat_payload());model_complete_apng_empty_fdat_payload();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,1usize,Ghost(value))
}
// apng/empty-fdat-then-data
pub open spec fn e6_bytes_apng_empty_fdat_then_data() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn e6_expected_apng_empty_fdat_then_data_0() -> Seq<u8> {seq![255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn e6_expected_apng_empty_fdat_then_data_1() -> Seq<u8> {seq![0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8]}
pub open spec fn expected_frames_apng_empty_fdat_then_data() -> Seq<fm::Frame> {seq![fm::Frame {rgba:e6_expected_apng_empty_fdat_then_data_0(),rect:crate::composite_model::rect(0u32,0u32,4u32,3u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8},fm::Frame {rgba:e6_expected_apng_empty_fdat_then_data_1(),rect:crate::composite_model::rect(0u32,0u32,2u32,2u32),delay:Some((1u16,10u16)),dispose:0u8,blend:0u8}]}
pub open spec fn expected_value_apng_empty_fdat_then_data(value: fm::Value) -> bool {
    value.info.header.width==4u32 && value.info.header.height==3u32
    && value.num_plays==Some(0u32) && value.info.num_plays==Some(0u32)
    && value.frames==expected_frames_apng_empty_fdat_then_data() && value.end==id::End::Iend
}
#[verifier::spinoff_prover]
pub proof fn model_complete_apng_empty_fdat_then_data()
    ensures match fm::MFr(e6_bytes_apng_empty_fdat_then_data()) {fm::Outcome::Done(value)=>expected_value_apng_empty_fdat_then_data(value),_=>false},
        fe::requests(e6_bytes_apng_empty_fdat_then_data()).is_prefix_of(id::requests_p(e6_bytes_apng_empty_fdat_then_data(),STRICT,3nat)),
{
    crate::witness_m5_frames::e6_complete_apng_empty_fdat_then_data();
}
#[verifier::spinoff_prover]
pub fn public_complete_apng_empty_fdat_then_data() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))
    ensures crate::still_equivalence::oom_member(id::requests_p(e6_bytes_apng_empty_fdat_then_data(),STRICT,3nat),result.0),
        match fm::MFr(e6_bytes_apng_empty_fdat_then_data()) {fm::Outcome::Done(value)=>expected_value_apng_empty_fdat_then_data(value)
            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},
{
    hide(fm::MFr);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==e6_bytes_apng_empty_fdat_then_data());model_complete_apng_empty_fdat_then_data();}
    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);
    crate::witness_m6_frame_model::exercise(&b,2usize,Ghost(value))
}
} // verus!
