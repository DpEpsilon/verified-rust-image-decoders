//! Seven complete frozen filter witnesses; calibrated shared row structure.
use vstd::prelude::*;
use crate::{still_model as sm,still_equivalence as se,alloc_guard::TryError};
verus! {
// filter/0-ct0-d1
pub open spec fn bytes_filter_0_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,112u8,96u8,152u8,193u8,48u8,7u8,0u8,4u8,0u8,1u8,129u8,85u8,162u8,196u8,190u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct0_d1() -> Seq<u8> {seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]}
pub open spec fn expected_value_filter_0_ct0_d1(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct0_d1()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct0_d1()
    ensures match sm::MS(bytes_filter_0_ct0_d1()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct0_d1(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct0_d1();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct0_d1() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct0_d1(),false),result),
        se::eq(bytes_filter_0_ct0_d1(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct0_d1(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,112u8,96u8,152u8,193u8,48u8,7u8,0u8,4u8,0u8,1u8,129u8,85u8,162u8,196u8,190u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d1());model_complete_filter_0_ct0_d1();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct0-d8
pub open spec fn bytes_filter_1_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,140u8,121u8,90u8,122u8,147u8,37u8,146u8,49u8,89u8,74u8,225u8,248u8,37u8,99u8,198u8,148u8,181u8,239u8,174u8,182u8,156u8,98u8,156u8,181u8,86u8,66u8,246u8,162u8,46u8,0u8,166u8,144u8,11u8,246u8,81u8,75u8,103u8,69u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct0_d8() -> Seq<u8> {seq![92u8,92u8,92u8,255u8,65u8,65u8,65u8,255u8,182u8,182u8,182u8,255u8,143u8,143u8,143u8,255u8,147u8,147u8,147u8,255u8,236u8,236u8,236u8,255u8,99u8,99u8,99u8,255u8,125u8,125u8,125u8,255u8,157u8,157u8,157u8,255u8,100u8,100u8,100u8,255u8,54u8,54u8,54u8,255u8,105u8,105u8,105u8,255u8,100u8,100u8,100u8,255u8,17u8,17u8,17u8,255u8,255u8,255u8,255u8,255u8,212u8,212u8,212u8,255u8,88u8,88u8,88u8,255u8,34u8,34u8,34u8,255u8,154u8,154u8,154u8,255u8,71u8,71u8,71u8,255u8,95u8,95u8,95u8,255u8,124u8,124u8,124u8,255u8,77u8,77u8,77u8,255u8,122u8,122u8,122u8,255u8]}
pub open spec fn expected_value_filter_1_ct0_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct0_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct0_d8()
    ensures match sm::MS(bytes_filter_1_ct0_d8()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct0_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct0_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct0_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct0_d8(),false),result),
        se::eq(bytes_filter_1_ct0_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct0_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,140u8,121u8,90u8,122u8,147u8,37u8,146u8,49u8,89u8,74u8,225u8,248u8,37u8,99u8,198u8,148u8,181u8,239u8,174u8,182u8,156u8,98u8,156u8,181u8,86u8,66u8,246u8,162u8,46u8,0u8,166u8,144u8,11u8,246u8,81u8,75u8,103u8,69u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d8());model_complete_filter_1_ct0_d8();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct0-d16
pub open spec fn bytes_filter_2_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8,57u8,156u8,20u8,131u8,167u8,250u8,95u8,158u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct0_d16() -> Seq<u8> {seq![172u8,172u8,172u8,255u8,145u8,145u8,145u8,255u8,211u8,211u8,211u8,255u8,109u8,109u8,109u8,255u8,106u8,106u8,106u8,255u8,141u8,141u8,141u8,255u8,165u8,165u8,165u8,255u8,243u8,243u8,243u8,255u8,41u8,41u8,41u8,255u8,127u8,127u8,127u8,255u8,9u8,9u8,9u8,255u8,74u8,74u8,74u8,255u8,68u8,68u8,68u8,255u8,166u8,166u8,166u8,255u8,101u8,101u8,101u8,255u8,133u8,133u8,133u8,255u8,99u8,99u8,99u8,255u8,163u8,163u8,163u8,255u8,153u8,153u8,153u8,255u8,50u8,50u8,50u8,255u8,190u8,190u8,190u8,255u8,68u8,68u8,68u8,255u8,167u8,167u8,167u8,255u8,222u8,222u8,222u8,255u8]}
pub open spec fn expected_value_filter_2_ct0_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct0_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct0_d16()
    ensures match sm::MS(bytes_filter_2_ct0_d16()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct0_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct0_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct0_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct0_d16(),false),result),
        se::eq(bytes_filter_2_ct0_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct0_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8,57u8,156u8,20u8,131u8,167u8,250u8,95u8,158u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d16());model_complete_filter_2_ct0_d16();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct2-d8
pub open spec fn bytes_filter_3_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8,7u8,34u8,38u8,61u8,42u8,102u8,146u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct2_d8() -> Seq<u8> {seq![50u8,39u8,159u8,255u8,191u8,222u8,75u8,255u8,196u8,57u8,125u8,255u8,26u8,110u8,17u8,255u8,199u8,229u8,2u8,255u8,184u8,138u8,103u8,255u8,219u8,121u8,205u8,255u8,166u8,83u8,60u8,255u8,10u8,43u8,17u8,255u8,85u8,183u8,95u8,255u8,10u8,178u8,135u8,255u8,21u8,77u8,78u8,255u8,80u8,31u8,102u8,255u8,92u8,106u8,158u8,255u8,42u8,3u8,78u8,255u8,195u8,210u8,22u8,255u8,73u8,4u8,91u8,255u8,93u8,137u8,140u8,255u8,174u8,82u8,138u8,255u8,44u8,255u8,34u8,255u8,13u8,216u8,229u8,255u8,49u8,205u8,144u8,255u8,76u8,216u8,168u8,255u8,176u8,241u8,156u8,255u8]}
pub open spec fn expected_value_filter_3_ct2_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct2_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct2_d8()
    ensures match sm::MS(bytes_filter_3_ct2_d8()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct2_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct2_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct2_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct2_d8(),false),result),
        se::eq(bytes_filter_3_ct2_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct2_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8,7u8,34u8,38u8,61u8,42u8,102u8,146u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct2_d8());model_complete_filter_3_ct2_d8();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct2-d16
pub open spec fn bytes_filter_4_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8,170u8,20u8,72u8,102u8,112u8,106u8,99u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct2_d16() -> Seq<u8> {seq![129u8,103u8,120u8,255u8,7u8,169u8,230u8,255u8,32u8,210u8,237u8,255u8,232u8,169u8,238u8,255u8,183u8,135u8,70u8,255u8,158u8,55u8,15u8,255u8,17u8,206u8,199u8,255u8,44u8,93u8,184u8,255u8,224u8,205u8,36u8,255u8,83u8,135u8,146u8,255u8,4u8,231u8,115u8,255u8,166u8,28u8,73u8,255u8,255u8,206u8,2u8,255u8,167u8,142u8,0u8,255u8,226u8,138u8,234u8,255u8,16u8,134u8,167u8,255u8,120u8,163u8,224u8,255u8,53u8,92u8,187u8,255u8,80u8,125u8,14u8,255u8,58u8,50u8,94u8,255u8,96u8,163u8,211u8,255u8,81u8,152u8,114u8,255u8,182u8,227u8,61u8,255u8,76u8,32u8,106u8,255u8]}
pub open spec fn expected_value_filter_4_ct2_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct2_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct2_d16()
    ensures match sm::MS(bytes_filter_4_ct2_d16()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct2_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct2_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct2_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct2_d16(),false),result),
        se::eq(bytes_filter_4_ct2_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct2_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8,170u8,20u8,72u8,102u8,112u8,106u8,99u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct2_d16());model_complete_filter_4_ct2_d16();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct4-d8
pub open spec fn bytes_filter_1_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8,77u8,83u8,22u8,44u8,56u8,113u8,12u8,30u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct4_d8() -> Seq<u8> {seq![79u8,79u8,79u8,65u8,104u8,104u8,104u8,158u8,10u8,10u8,10u8,151u8,105u8,105u8,105u8,222u8,71u8,71u8,71u8,82u8,43u8,43u8,43u8,193u8,164u8,164u8,164u8,112u8,226u8,226u8,226u8,125u8,185u8,185u8,185u8,51u8,95u8,95u8,95u8,72u8,7u8,7u8,7u8,175u8,91u8,91u8,91u8,120u8,80u8,80u8,80u8,71u8,17u8,17u8,17u8,132u8,115u8,115u8,115u8,65u8,220u8,220u8,220u8,251u8,223u8,223u8,223u8,161u8,131u8,131u8,131u8,152u8,37u8,37u8,37u8,69u8,39u8,39u8,39u8,198u8,136u8,136u8,136u8,234u8,13u8,13u8,13u8,255u8,77u8,77u8,77u8,218u8,130u8,130u8,130u8,203u8]}
pub open spec fn expected_value_filter_1_ct4_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct4_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct4_d8()
    ensures match sm::MS(bytes_filter_1_ct4_d8()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct4_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct4_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct4_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct4_d8(),false),result),
        se::eq(bytes_filter_1_ct4_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct4_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8,77u8,83u8,22u8,44u8,56u8,113u8,12u8,30u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct4_d8());model_complete_filter_1_ct4_d8();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct6-d16
pub open spec fn bytes_filter_4_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8,193u8,11u8,87u8,233u8,29u8,229u8,5u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct6_d16() -> Seq<u8> {seq![52u8,111u8,23u8,165u8,235u8,86u8,198u8,24u8,85u8,3u8,168u8,60u8,3u8,217u8,76u8,124u8,118u8,172u8,22u8,75u8,164u8,75u8,109u8,101u8,93u8,64u8,120u8,179u8,118u8,13u8,217u8,235u8,102u8,83u8,191u8,190u8,97u8,248u8,63u8,107u8,123u8,219u8,73u8,4u8,223u8,128u8,153u8,24u8,40u8,168u8,104u8,253u8,70u8,58u8,134u8,65u8,84u8,139u8,22u8,56u8,97u8,111u8,61u8,34u8,169u8,76u8,29u8,10u8,55u8,182u8,94u8,216u8,186u8,196u8,43u8,132u8,213u8,211u8,45u8,40u8,148u8,14u8,21u8,49u8,35u8,126u8,80u8,9u8,246u8,168u8,221u8,21u8,198u8,247u8,252u8,224u8]}
pub open spec fn expected_value_filter_4_ct6_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct6_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct6_d16()
    ensures match sm::MS(bytes_filter_4_ct6_d16()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct6_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct6_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct6_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct6_d16(),false),result),
        se::eq(bytes_filter_4_ct6_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct6_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8,193u8,11u8,87u8,233u8,29u8,229u8,5u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct6_d16());model_complete_filter_4_ct6_d16();}
    crate::png::try_decode_still(&b)
}
} // verus!
