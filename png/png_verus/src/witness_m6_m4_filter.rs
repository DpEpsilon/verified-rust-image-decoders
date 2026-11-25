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
// filter/0-ct0-d8
pub open spec fn bytes_filter_0_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,159u8,200u8,190u8,165u8,202u8,144u8,65u8,238u8,230u8,219u8,223u8,245u8,121u8,12u8,118u8,255u8,197u8,121u8,111u8,177u8,50u8,148u8,92u8,250u8,93u8,190u8,220u8,25u8,0u8,173u8,223u8,12u8,117u8,24u8,26u8,151u8,33u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct0_d8() -> Seq<u8> {seq![255u8,255u8,255u8,255u8,97u8,97u8,97u8,255u8,7u8,7u8,7u8,255u8,180u8,180u8,180u8,255u8,122u8,122u8,122u8,255u8,49u8,49u8,49u8,255u8,30u8,30u8,30u8,255u8,217u8,217u8,217u8,255u8,237u8,237u8,237u8,255u8,251u8,251u8,251u8,255u8,127u8,127u8,127u8,255u8,110u8,110u8,110u8,255u8,62u8,62u8,62u8,255u8,255u8,255u8,255u8,255u8,23u8,23u8,23u8,255u8,13u8,13u8,13u8,255u8,218u8,218u8,218u8,255u8,5u8,5u8,5u8,255u8,116u8,116u8,116u8,255u8,210u8,210u8,210u8,255u8,251u8,251u8,251u8,255u8,119u8,119u8,119u8,255u8,167u8,167u8,167u8,255u8,67u8,67u8,67u8,255u8]}
pub open spec fn expected_value_filter_0_ct0_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct0_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct0_d8()
    ensures match sm::MS(bytes_filter_0_ct0_d8()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct0_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct0_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct0_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct0_d8(),false),result),
        se::eq(bytes_filter_0_ct0_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct0_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,159u8,200u8,190u8,165u8,202u8,144u8,65u8,238u8,230u8,219u8,223u8,245u8,121u8,12u8,118u8,255u8,197u8,121u8,111u8,177u8,50u8,148u8,92u8,250u8,93u8,190u8,220u8,25u8,0u8,173u8,223u8,12u8,117u8,24u8,26u8,151u8,33u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d8());model_complete_filter_0_ct0_d8();}
    crate::png::try_decode_still(&b)
}
// filter/0-ct0-d16
pub open spec fn bytes_filter_0_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,61u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,241u8,238u8,82u8,203u8,203u8,144u8,222u8,16u8,103u8,35u8,139u8,70u8,6u8,11u8,159u8,93u8,188u8,201u8,137u8,13u8,95u8,52u8,31u8,133u8,165u8,48u8,176u8,114u8,112u8,104u8,92u8,99u8,75u8,14u8,204u8,83u8,158u8,153u8,201u8,112u8,79u8,223u8,76u8,116u8,219u8,205u8,182u8,252u8,70u8,137u8,166u8,60u8,0u8,60u8,58u8,20u8,150u8,248u8,127u8,179u8,191u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct0_d16() -> Seq<u8> {seq![248u8,248u8,248u8,255u8,210u8,210u8,210u8,255u8,232u8,232u8,232u8,255u8,141u8,141u8,141u8,255u8,67u8,67u8,67u8,255u8,56u8,56u8,56u8,255u8,56u8,56u8,56u8,255u8,185u8,185u8,185u8,255u8,99u8,99u8,99u8,255u8,128u8,128u8,128u8,255u8,42u8,42u8,42u8,255u8,86u8,86u8,86u8,255u8,5u8,5u8,5u8,255u8,8u8,8u8,8u8,255u8,213u8,213u8,213u8,255u8,99u8,99u8,99u8,255u8,110u8,110u8,110u8,255u8,153u8,153u8,153u8,255u8,221u8,221u8,221u8,255u8,54u8,54u8,54u8,255u8,182u8,182u8,182u8,255u8,134u8,134u8,134u8,255u8,129u8,129u8,129u8,255u8,130u8,130u8,130u8,255u8]}
pub open spec fn expected_value_filter_0_ct0_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct0_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct0_d16()
    ensures match sm::MS(bytes_filter_0_ct0_d16()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct0_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct0_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct0_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct0_d16(),false),result),
        se::eq(bytes_filter_0_ct0_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct0_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,61u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,241u8,238u8,82u8,203u8,203u8,144u8,222u8,16u8,103u8,35u8,139u8,70u8,6u8,11u8,159u8,93u8,188u8,201u8,137u8,13u8,95u8,52u8,31u8,133u8,165u8,48u8,176u8,114u8,112u8,104u8,92u8,99u8,75u8,14u8,204u8,83u8,158u8,153u8,201u8,112u8,79u8,223u8,76u8,116u8,219u8,205u8,182u8,252u8,70u8,137u8,166u8,60u8,0u8,60u8,58u8,20u8,150u8,248u8,127u8,179u8,191u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d16());model_complete_filter_0_ct0_d16();}
    crate::png::try_decode_still(&b)
}
// filter/0-ct2-d8
pub open spec fn bytes_filter_0_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8,207u8,173u8,36u8,11u8,101u8,174u8,80u8,132u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct2_d8() -> Seq<u8> {seq![130u8,201u8,136u8,255u8,75u8,0u8,41u8,255u8,139u8,10u8,24u8,255u8,81u8,11u8,150u8,255u8,76u8,112u8,11u8,255u8,192u8,240u8,148u8,255u8,38u8,252u8,186u8,255u8,71u8,79u8,32u8,255u8,71u8,72u8,111u8,255u8,215u8,127u8,61u8,255u8,24u8,48u8,223u8,255u8,149u8,46u8,117u8,255u8,66u8,198u8,224u8,255u8,254u8,7u8,188u8,255u8,239u8,45u8,234u8,255u8,214u8,142u8,226u8,255u8,70u8,154u8,227u8,255u8,194u8,43u8,114u8,255u8,246u8,114u8,49u8,255u8,144u8,235u8,140u8,255u8,92u8,53u8,61u8,255u8,218u8,219u8,44u8,255u8,250u8,209u8,220u8,255u8,81u8,135u8,178u8,255u8]}
pub open spec fn expected_value_filter_0_ct2_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct2_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct2_d8()
    ensures match sm::MS(bytes_filter_0_ct2_d8()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct2_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct2_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct2_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct2_d8(),false),result),
        se::eq(bytes_filter_0_ct2_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct2_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8,207u8,173u8,36u8,11u8,101u8,174u8,80u8,132u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct2_d8());model_complete_filter_0_ct2_d8();}
    crate::png::try_decode_still(&b)
}
// filter/0-ct2-d16
pub open spec fn bytes_filter_0_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8,13u8,247u8,72u8,67u8,125u8,149u8,250u8,88u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct2_d16() -> Seq<u8> {seq![195u8,43u8,82u8,255u8,253u8,222u8,213u8,255u8,30u8,122u8,185u8,255u8,137u8,178u8,12u8,255u8,184u8,11u8,167u8,255u8,72u8,103u8,185u8,255u8,112u8,218u8,125u8,255u8,248u8,106u8,13u8,255u8,218u8,103u8,66u8,255u8,210u8,16u8,61u8,255u8,26u8,114u8,246u8,255u8,108u8,36u8,88u8,255u8,203u8,225u8,38u8,255u8,158u8,174u8,58u8,255u8,200u8,144u8,110u8,255u8,200u8,190u8,251u8,255u8,60u8,165u8,69u8,255u8,71u8,121u8,157u8,255u8,6u8,113u8,94u8,255u8,197u8,213u8,227u8,255u8,89u8,110u8,193u8,255u8,134u8,86u8,48u8,255u8,19u8,227u8,159u8,255u8,225u8,12u8,244u8,255u8]}
pub open spec fn expected_value_filter_0_ct2_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct2_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct2_d16()
    ensures match sm::MS(bytes_filter_0_ct2_d16()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct2_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct2_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct2_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct2_d16(),false),result),
        se::eq(bytes_filter_0_ct2_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct2_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8,13u8,247u8,72u8,67u8,125u8,149u8,250u8,88u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct2_d16());model_complete_filter_0_ct2_d16();}
    crate::png::try_decode_still(&b)
}
// filter/0-ct4-d8
pub open spec fn bytes_filter_0_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8,121u8,35u8,24u8,6u8,250u8,7u8,239u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct4_d8() -> Seq<u8> {seq![132u8,132u8,132u8,182u8,213u8,213u8,213u8,13u8,179u8,179u8,179u8,117u8,92u8,92u8,92u8,240u8,34u8,34u8,34u8,58u8,244u8,244u8,244u8,195u8,10u8,10u8,10u8,154u8,2u8,2u8,2u8,192u8,114u8,114u8,114u8,58u8,68u8,68u8,68u8,205u8,171u8,171u8,171u8,170u8,21u8,21u8,21u8,14u8,212u8,212u8,212u8,0u8,204u8,204u8,204u8,250u8,203u8,203u8,203u8,205u8,187u8,187u8,187u8,163u8,76u8,76u8,76u8,202u8,15u8,15u8,15u8,20u8,162u8,162u8,162u8,207u8,96u8,96u8,96u8,191u8,81u8,81u8,81u8,159u8,128u8,128u8,128u8,227u8,38u8,38u8,38u8,150u8,62u8,62u8,62u8,33u8]}
pub open spec fn expected_value_filter_0_ct4_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct4_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct4_d8()
    ensures match sm::MS(bytes_filter_0_ct4_d8()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct4_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct4_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct4_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct4_d8(),false),result),
        se::eq(bytes_filter_0_ct4_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct4_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8,121u8,35u8,24u8,6u8,250u8,7u8,239u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct4_d8());model_complete_filter_0_ct4_d8();}
    crate::png::try_decode_still(&b)
}
// filter/0-ct6-d16
pub open spec fn bytes_filter_0_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8,148u8,154u8,96u8,252u8,75u8,11u8,232u8,231u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_0_ct6_d16() -> Seq<u8> {seq![242u8,16u8,175u8,138u8,142u8,244u8,95u8,64u8,135u8,43u8,169u8,173u8,150u8,164u8,146u8,200u8,182u8,185u8,105u8,106u8,150u8,158u8,183u8,186u8,169u8,87u8,208u8,121u8,127u8,252u8,211u8,7u8,187u8,106u8,51u8,232u8,4u8,233u8,46u8,204u8,185u8,195u8,92u8,251u8,230u8,201u8,106u8,229u8,210u8,141u8,164u8,52u8,221u8,147u8,122u8,99u8,228u8,222u8,34u8,112u8,60u8,109u8,170u8,51u8,133u8,26u8,148u8,149u8,58u8,237u8,190u8,31u8,85u8,127u8,176u8,217u8,51u8,11u8,205u8,44u8,199u8,222u8,41u8,85u8,164u8,88u8,234u8,254u8,113u8,99u8,10u8,94u8,0u8,209u8,102u8,238u8]}
pub open spec fn expected_value_filter_0_ct6_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_0_ct6_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_0_ct6_d16()
    ensures match sm::MS(bytes_filter_0_ct6_d16()) {sm::Outcome::Done(value)=>expected_value_filter_0_ct6_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_0_ct6_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_0_ct6_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_0_ct6_d16(),false),result),
        se::eq(bytes_filter_0_ct6_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_0_ct6_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8,148u8,154u8,96u8,252u8,75u8,11u8,232u8,231u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct6_d16());model_complete_filter_0_ct6_d16();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct0-d1
pub open spec fn bytes_filter_1_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,156u8,195u8,120u8,134u8,241u8,8u8,35u8,15u8,0u8,10u8,180u8,2u8,61u8,133u8,32u8,85u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct0_d1() -> Seq<u8> {seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]}
pub open spec fn expected_value_filter_1_ct0_d1(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct0_d1()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct0_d1()
    ensures match sm::MS(bytes_filter_1_ct0_d1()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct0_d1(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct0_d1();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct0_d1() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct0_d1(),false),result),
        se::eq(bytes_filter_1_ct0_d1(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct0_d1(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,156u8,195u8,120u8,134u8,241u8,8u8,35u8,15u8,0u8,10u8,180u8,2u8,61u8,133u8,32u8,85u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d1());model_complete_filter_1_ct0_d1();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct0-d16
pub open spec fn bytes_filter_1_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8,27u8,183u8,21u8,213u8,163u8,179u8,160u8,217u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct0_d16() -> Seq<u8> {seq![194u8,194u8,194u8,255u8,255u8,255u8,255u8,255u8,132u8,132u8,132u8,255u8,146u8,146u8,146u8,255u8,230u8,230u8,230u8,255u8,6u8,6u8,6u8,255u8,105u8,105u8,105u8,255u8,110u8,110u8,110u8,255u8,142u8,142u8,142u8,255u8,219u8,219u8,219u8,255u8,227u8,227u8,227u8,255u8,137u8,137u8,137u8,255u8,14u8,14u8,14u8,255u8,64u8,64u8,64u8,255u8,239u8,239u8,239u8,255u8,245u8,245u8,245u8,255u8,182u8,182u8,182u8,255u8,172u8,172u8,172u8,255u8,190u8,190u8,190u8,255u8,11u8,11u8,11u8,255u8,34u8,34u8,34u8,255u8,183u8,183u8,183u8,255u8,184u8,184u8,184u8,255u8,229u8,229u8,229u8,255u8]}
pub open spec fn expected_value_filter_1_ct0_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct0_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct0_d16()
    ensures match sm::MS(bytes_filter_1_ct0_d16()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct0_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct0_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct0_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct0_d16(),false),result),
        se::eq(bytes_filter_1_ct0_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct0_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8,27u8,183u8,21u8,213u8,163u8,179u8,160u8,217u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d16());model_complete_filter_1_ct0_d16();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct2-d8
pub open spec fn bytes_filter_1_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8,50u8,96u8,34u8,157u8,121u8,4u8,237u8,249u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct2_d8() -> Seq<u8> {seq![127u8,211u8,204u8,255u8,187u8,65u8,119u8,255u8,19u8,198u8,140u8,255u8,227u8,55u8,172u8,255u8,12u8,125u8,161u8,255u8,108u8,36u8,248u8,255u8,206u8,63u8,161u8,255u8,9u8,240u8,216u8,255u8,187u8,20u8,2u8,255u8,26u8,7u8,109u8,255u8,164u8,5u8,205u8,255u8,150u8,219u8,189u8,255u8,33u8,15u8,61u8,255u8,116u8,143u8,237u8,255u8,59u8,98u8,35u8,255u8,150u8,44u8,151u8,255u8,37u8,113u8,217u8,255u8,55u8,150u8,227u8,255u8,208u8,98u8,169u8,255u8,109u8,11u8,18u8,255u8,178u8,141u8,221u8,255u8,207u8,171u8,168u8,255u8,124u8,84u8,201u8,255u8,96u8,111u8,99u8,255u8]}
pub open spec fn expected_value_filter_1_ct2_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct2_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct2_d8()
    ensures match sm::MS(bytes_filter_1_ct2_d8()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct2_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct2_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct2_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct2_d8(),false),result),
        se::eq(bytes_filter_1_ct2_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct2_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8,50u8,96u8,34u8,157u8,121u8,4u8,237u8,249u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct2_d8());model_complete_filter_1_ct2_d8();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct2-d16
pub open spec fn bytes_filter_1_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8,3u8,17u8,75u8,49u8,171u8,174u8,64u8,54u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct2_d16() -> Seq<u8> {seq![187u8,42u8,36u8,255u8,164u8,28u8,24u8,255u8,147u8,165u8,19u8,255u8,213u8,190u8,23u8,255u8,143u8,211u8,250u8,255u8,139u8,253u8,6u8,255u8,229u8,175u8,38u8,255u8,41u8,121u8,74u8,255u8,58u8,180u8,91u8,255u8,248u8,112u8,91u8,255u8,80u8,22u8,50u8,255u8,147u8,0u8,142u8,255u8,169u8,72u8,17u8,255u8,139u8,84u8,82u8,255u8,200u8,94u8,209u8,255u8,72u8,93u8,195u8,255u8,250u8,211u8,50u8,255u8,2u8,204u8,146u8,255u8,203u8,143u8,197u8,255u8,17u8,107u8,157u8,255u8,180u8,46u8,116u8,255u8,218u8,114u8,240u8,255u8,237u8,74u8,194u8,255u8,166u8,211u8,30u8,255u8]}
pub open spec fn expected_value_filter_1_ct2_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct2_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct2_d16()
    ensures match sm::MS(bytes_filter_1_ct2_d16()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct2_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct2_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct2_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct2_d16(),false),result),
        se::eq(bytes_filter_1_ct2_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct2_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8,3u8,17u8,75u8,49u8,171u8,174u8,64u8,54u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct2_d16());model_complete_filter_1_ct2_d16();}
    crate::png::try_decode_still(&b)
}
// filter/1-ct6-d16
pub open spec fn bytes_filter_1_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8,213u8,94u8,99u8,57u8,73u8,100u8,0u8,73u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_1_ct6_d16() -> Seq<u8> {seq![108u8,120u8,210u8,158u8,47u8,187u8,128u8,170u8,119u8,130u8,2u8,95u8,228u8,127u8,147u8,193u8,235u8,171u8,70u8,139u8,156u8,130u8,189u8,52u8,130u8,109u8,194u8,89u8,129u8,243u8,41u8,40u8,18u8,180u8,241u8,25u8,116u8,186u8,158u8,119u8,158u8,199u8,74u8,25u8,156u8,121u8,109u8,29u8,155u8,236u8,239u8,130u8,146u8,140u8,172u8,224u8,151u8,14u8,198u8,174u8,209u8,6u8,18u8,126u8,35u8,147u8,64u8,85u8,18u8,65u8,139u8,69u8,150u8,168u8,252u8,20u8,182u8,43u8,71u8,29u8,205u8,62u8,145u8,175u8,210u8,249u8,96u8,122u8,72u8,24u8,46u8,240u8,243u8,7u8,77u8,156u8]}
pub open spec fn expected_value_filter_1_ct6_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_1_ct6_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_1_ct6_d16()
    ensures match sm::MS(bytes_filter_1_ct6_d16()) {sm::Outcome::Done(value)=>expected_value_filter_1_ct6_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_1_ct6_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_1_ct6_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_1_ct6_d16(),false),result),
        se::eq(bytes_filter_1_ct6_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_1_ct6_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8,213u8,94u8,99u8,57u8,73u8,100u8,0u8,73u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct6_d16());model_complete_filter_1_ct6_d16();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct0-d1
pub open spec fn bytes_filter_2_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,90u8,193u8,180u8,134u8,105u8,14u8,211u8,3u8,0u8,10u8,216u8,2u8,217u8,61u8,158u8,25u8,28u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct0_d1() -> Seq<u8> {seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]}
pub open spec fn expected_value_filter_2_ct0_d1(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct0_d1()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct0_d1()
    ensures match sm::MS(bytes_filter_2_ct0_d1()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct0_d1(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct0_d1();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct0_d1() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct0_d1(),false),result),
        se::eq(bytes_filter_2_ct0_d1(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct0_d1(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,90u8,193u8,180u8,134u8,105u8,14u8,211u8,3u8,0u8,10u8,216u8,2u8,217u8,61u8,158u8,25u8,28u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d1());model_complete_filter_2_ct0_d1();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct0-d8
pub open spec fn bytes_filter_2_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,178u8,123u8,234u8,35u8,246u8,91u8,131u8,233u8,28u8,135u8,197u8,242u8,144u8,88u8,166u8,157u8,238u8,215u8,195u8,23u8,109u8,101u8,250u8,185u8,224u8,205u8,141u8,55u8,11u8,1u8,162u8,180u8,13u8,134u8,226u8,196u8,163u8,215u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct0_d8() -> Seq<u8> {seq![62u8,62u8,62u8,255u8,229u8,229u8,229u8,255u8,76u8,76u8,76u8,255u8,22u8,22u8,22u8,255u8,251u8,251u8,251u8,255u8,40u8,40u8,40u8,255u8,12u8,12u8,12u8,255u8,237u8,237u8,237u8,255u8,132u8,132u8,132u8,255u8,189u8,189u8,189u8,255u8,79u8,79u8,79u8,255u8,133u8,133u8,133u8,255u8,197u8,197u8,197u8,255u8,52u8,52u8,52u8,255u8,91u8,91u8,91u8,255u8,20u8,20u8,20u8,255u8,241u8,241u8,241u8,255u8,58u8,58u8,58u8,255u8,190u8,190u8,190u8,255u8,212u8,212u8,212u8,255u8,71u8,71u8,71u8,255u8,236u8,236u8,236u8,255u8,221u8,221u8,221u8,255u8,219u8,219u8,219u8,255u8]}
pub open spec fn expected_value_filter_2_ct0_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct0_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct0_d8()
    ensures match sm::MS(bytes_filter_2_ct0_d8()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct0_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct0_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct0_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct0_d8(),false),result),
        se::eq(bytes_filter_2_ct0_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct0_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,178u8,123u8,234u8,35u8,246u8,91u8,131u8,233u8,28u8,135u8,197u8,242u8,144u8,88u8,166u8,157u8,238u8,215u8,195u8,23u8,109u8,101u8,250u8,185u8,224u8,205u8,141u8,55u8,11u8,1u8,162u8,180u8,13u8,134u8,226u8,196u8,163u8,215u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d8());model_complete_filter_2_ct0_d8();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct2-d8
pub open spec fn bytes_filter_2_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8,143u8,224u8,32u8,127u8,97u8,105u8,89u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct2_d8() -> Seq<u8> {seq![225u8,86u8,108u8,255u8,113u8,142u8,93u8,255u8,40u8,51u8,108u8,255u8,3u8,173u8,54u8,255u8,38u8,227u8,123u8,255u8,0u8,188u8,178u8,255u8,18u8,206u8,222u8,255u8,119u8,9u8,177u8,255u8,213u8,177u8,99u8,255u8,173u8,225u8,183u8,255u8,38u8,107u8,160u8,255u8,230u8,154u8,16u8,255u8,115u8,23u8,251u8,255u8,12u8,32u8,216u8,255u8,238u8,180u8,75u8,255u8,174u8,216u8,53u8,255u8,77u8,217u8,25u8,255u8,130u8,213u8,132u8,255u8,24u8,176u8,168u8,255u8,255u8,141u8,95u8,255u8,116u8,237u8,140u8,255u8,159u8,233u8,191u8,255u8,159u8,206u8,220u8,255u8,21u8,150u8,243u8,255u8]}
pub open spec fn expected_value_filter_2_ct2_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct2_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct2_d8()
    ensures match sm::MS(bytes_filter_2_ct2_d8()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct2_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct2_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct2_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct2_d8(),false),result),
        se::eq(bytes_filter_2_ct2_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct2_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8,143u8,224u8,32u8,127u8,97u8,105u8,89u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct2_d8());model_complete_filter_2_ct2_d8();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct2-d16
pub open spec fn bytes_filter_2_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8,24u8,91u8,71u8,39u8,214u8,217u8,147u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct2_d16() -> Seq<u8> {seq![123u8,14u8,226u8,255u8,189u8,84u8,39u8,255u8,240u8,172u8,244u8,255u8,232u8,117u8,254u8,255u8,141u8,25u8,138u8,255u8,191u8,55u8,159u8,255u8,210u8,172u8,253u8,255u8,207u8,167u8,222u8,255u8,201u8,145u8,172u8,255u8,222u8,127u8,4u8,255u8,32u8,222u8,221u8,255u8,156u8,179u8,215u8,255u8,176u8,162u8,60u8,255u8,99u8,12u8,63u8,255u8,242u8,232u8,119u8,255u8,147u8,216u8,29u8,255u8,145u8,210u8,35u8,255u8,81u8,52u8,250u8,255u8,127u8,143u8,186u8,255u8,117u8,165u8,97u8,255u8,194u8,4u8,142u8,255u8,24u8,87u8,235u8,255u8,252u8,28u8,129u8,255u8,14u8,169u8,16u8,255u8]}
pub open spec fn expected_value_filter_2_ct2_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct2_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct2_d16()
    ensures match sm::MS(bytes_filter_2_ct2_d16()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct2_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct2_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct2_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct2_d16(),false),result),
        se::eq(bytes_filter_2_ct2_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct2_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8,24u8,91u8,71u8,39u8,214u8,217u8,147u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct2_d16());model_complete_filter_2_ct2_d16();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct4-d8
pub open spec fn bytes_filter_2_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8,62u8,101u8,23u8,103u8,90u8,111u8,248u8,112u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct4_d8() -> Seq<u8> {seq![26u8,26u8,26u8,146u8,68u8,68u8,68u8,149u8,181u8,181u8,181u8,33u8,132u8,132u8,132u8,45u8,142u8,142u8,142u8,77u8,18u8,18u8,18u8,143u8,200u8,200u8,200u8,129u8,74u8,74u8,74u8,173u8,54u8,54u8,54u8,180u8,17u8,17u8,17u8,177u8,51u8,51u8,51u8,111u8,239u8,239u8,239u8,7u8,178u8,178u8,178u8,83u8,170u8,170u8,170u8,134u8,22u8,22u8,22u8,155u8,71u8,71u8,71u8,131u8,67u8,67u8,67u8,184u8,247u8,247u8,247u8,25u8,36u8,36u8,36u8,196u8,71u8,71u8,71u8,0u8,36u8,36u8,36u8,95u8,44u8,44u8,44u8,208u8,255u8,255u8,255u8,143u8,118u8,118u8,118u8,172u8]}
pub open spec fn expected_value_filter_2_ct4_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct4_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct4_d8()
    ensures match sm::MS(bytes_filter_2_ct4_d8()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct4_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct4_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct4_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct4_d8(),false),result),
        se::eq(bytes_filter_2_ct4_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct4_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8,62u8,101u8,23u8,103u8,90u8,111u8,248u8,112u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct4_d8());model_complete_filter_2_ct4_d8();}
    crate::png::try_decode_still(&b)
}
// filter/2-ct6-d16
pub open spec fn bytes_filter_2_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8,109u8,25u8,99u8,226u8,254u8,245u8,154u8,188u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_2_ct6_d16() -> Seq<u8> {seq![203u8,150u8,56u8,215u8,215u8,106u8,193u8,3u8,132u8,101u8,168u8,174u8,101u8,109u8,55u8,152u8,138u8,111u8,13u8,32u8,19u8,7u8,154u8,182u8,28u8,239u8,201u8,36u8,53u8,45u8,139u8,70u8,140u8,45u8,62u8,126u8,56u8,146u8,201u8,145u8,243u8,1u8,158u8,212u8,96u8,63u8,56u8,186u8,129u8,3u8,247u8,47u8,205u8,155u8,160u8,145u8,4u8,67u8,182u8,55u8,191u8,65u8,90u8,126u8,240u8,83u8,79u8,120u8,81u8,210u8,12u8,58u8,241u8,238u8,233u8,233u8,58u8,12u8,230u8,166u8,75u8,7u8,113u8,210u8,153u8,45u8,220u8,58u8,29u8,24u8,252u8,249u8,24u8,46u8,22u8,13u8]}
pub open spec fn expected_value_filter_2_ct6_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_2_ct6_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_2_ct6_d16()
    ensures match sm::MS(bytes_filter_2_ct6_d16()) {sm::Outcome::Done(value)=>expected_value_filter_2_ct6_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_2_ct6_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_2_ct6_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_2_ct6_d16(),false),result),
        se::eq(bytes_filter_2_ct6_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_2_ct6_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8,109u8,25u8,99u8,226u8,254u8,245u8,154u8,188u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct6_d16());model_complete_filter_2_ct6_d16();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct0-d1
pub open spec fn bytes_filter_3_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,246u8,97u8,102u8,98u8,174u8,97u8,14u8,1u8,0u8,4u8,42u8,1u8,43u8,217u8,76u8,137u8,10u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct0_d1() -> Seq<u8> {seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]}
pub open spec fn expected_value_filter_3_ct0_d1(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct0_d1()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct0_d1()
    ensures match sm::MS(bytes_filter_3_ct0_d1()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct0_d1(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct0_d1();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct0_d1() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct0_d1(),false),result),
        se::eq(bytes_filter_3_ct0_d1(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct0_d1(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,246u8,97u8,102u8,98u8,174u8,97u8,14u8,1u8,0u8,4u8,42u8,1u8,43u8,217u8,76u8,137u8,10u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d1());model_complete_filter_3_ct0_d1();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct0-d8
pub open spec fn bytes_filter_3_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,142u8,252u8,213u8,157u8,183u8,200u8,138u8,249u8,238u8,245u8,226u8,73u8,19u8,115u8,152u8,89u8,5u8,142u8,151u8,174u8,248u8,205u8,188u8,190u8,209u8,110u8,206u8,252u8,16u8,0u8,184u8,100u8,12u8,220u8,35u8,200u8,177u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct0_d8() -> Seq<u8> {seq![89u8,89u8,89u8,255u8,38u8,38u8,38u8,255u8,158u8,158u8,158u8,255u8,189u8,189u8,189u8,255u8,0u8,0u8,0u8,255u8,58u8,58u8,58u8,255u8,9u8,9u8,9u8,255u8,238u8,238u8,238u8,255u8,57u8,57u8,57u8,255u8,13u8,13u8,13u8,255u8,151u8,151u8,151u8,255u8,212u8,212u8,212u8,255u8,9u8,9u8,9u8,255u8,139u8,139u8,139u8,255u8,41u8,41u8,41u8,255u8,144u8,144u8,144u8,255u8,59u8,59u8,59u8,255u8,130u8,130u8,130u8,255u8,179u8,179u8,179u8,255u8,32u8,32u8,32u8,255u8,98u8,98u8,98u8,255u8,21u8,21u8,21u8,255u8,199u8,199u8,199u8,255u8,248u8,248u8,248u8,255u8]}
pub open spec fn expected_value_filter_3_ct0_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct0_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct0_d8()
    ensures match sm::MS(bytes_filter_3_ct0_d8()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct0_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct0_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct0_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct0_d8(),false),result),
        se::eq(bytes_filter_3_ct0_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct0_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,142u8,252u8,213u8,157u8,183u8,200u8,138u8,249u8,238u8,245u8,226u8,73u8,19u8,115u8,152u8,89u8,5u8,142u8,151u8,174u8,248u8,205u8,188u8,190u8,209u8,110u8,206u8,252u8,16u8,0u8,184u8,100u8,12u8,220u8,35u8,200u8,177u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d8());model_complete_filter_3_ct0_d8();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct0-d16
pub open spec fn bytes_filter_3_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8,122u8,168u8,23u8,178u8,165u8,47u8,232u8,141u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct0_d16() -> Seq<u8> {seq![6u8,6u8,6u8,255u8,116u8,116u8,116u8,255u8,116u8,116u8,116u8,255u8,40u8,40u8,40u8,255u8,45u8,45u8,45u8,255u8,189u8,189u8,189u8,255u8,39u8,39u8,39u8,255u8,56u8,56u8,56u8,255u8,148u8,148u8,148u8,255u8,43u8,43u8,43u8,255u8,35u8,35u8,35u8,255u8,3u8,3u8,3u8,255u8,170u8,170u8,170u8,255u8,122u8,122u8,122u8,255u8,243u8,243u8,243u8,255u8,2u8,2u8,2u8,255u8,168u8,168u8,168u8,255u8,192u8,192u8,192u8,255u8,219u8,219u8,219u8,255u8,216u8,216u8,216u8,255u8,93u8,93u8,93u8,255u8,69u8,69u8,69u8,255u8,214u8,214u8,214u8,255u8,122u8,122u8,122u8,255u8]}
pub open spec fn expected_value_filter_3_ct0_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct0_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct0_d16()
    ensures match sm::MS(bytes_filter_3_ct0_d16()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct0_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct0_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct0_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct0_d16(),false),result),
        se::eq(bytes_filter_3_ct0_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct0_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8,122u8,168u8,23u8,178u8,165u8,47u8,232u8,141u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d16());model_complete_filter_3_ct0_d16();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct2-d16
pub open spec fn bytes_filter_3_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8,235u8,81u8,65u8,134u8,230u8,230u8,77u8,137u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct2_d16() -> Seq<u8> {seq![88u8,168u8,137u8,255u8,106u8,235u8,68u8,255u8,64u8,132u8,100u8,255u8,34u8,59u8,26u8,255u8,66u8,29u8,84u8,255u8,88u8,232u8,72u8,255u8,63u8,17u8,200u8,255u8,137u8,64u8,15u8,255u8,230u8,120u8,167u8,255u8,244u8,116u8,77u8,255u8,166u8,185u8,212u8,255u8,254u8,239u8,31u8,255u8,85u8,155u8,85u8,255u8,65u8,52u8,71u8,255u8,128u8,107u8,5u8,255u8,15u8,6u8,64u8,255u8,109u8,39u8,167u8,255u8,218u8,104u8,65u8,255u8,199u8,142u8,196u8,255u8,52u8,6u8,66u8,255u8,42u8,146u8,154u8,255u8,250u8,127u8,12u8,255u8,181u8,117u8,81u8,255u8,169u8,47u8,169u8,255u8]}
pub open spec fn expected_value_filter_3_ct2_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct2_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct2_d16()
    ensures match sm::MS(bytes_filter_3_ct2_d16()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct2_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct2_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct2_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct2_d16(),false),result),
        se::eq(bytes_filter_3_ct2_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct2_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8,235u8,81u8,65u8,134u8,230u8,230u8,77u8,137u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct2_d16());model_complete_filter_3_ct2_d16();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct4-d8
pub open spec fn bytes_filter_3_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8,48u8,3u8,23u8,115u8,82u8,182u8,110u8,40u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct4_d8() -> Seq<u8> {seq![54u8,54u8,54u8,146u8,35u8,35u8,35u8,74u8,56u8,56u8,56u8,182u8,14u8,14u8,14u8,85u8,203u8,203u8,203u8,79u8,166u8,166u8,166u8,98u8,64u8,64u8,64u8,225u8,232u8,232u8,232u8,238u8,150u8,150u8,150u8,233u8,134u8,134u8,134u8,38u8,57u8,57u8,57u8,18u8,224u8,224u8,224u8,53u8,170u8,170u8,170u8,79u8,49u8,49u8,49u8,189u8,204u8,204u8,204u8,189u8,253u8,253u8,253u8,65u8,86u8,86u8,86u8,168u8,175u8,175u8,175u8,247u8,212u8,212u8,212u8,21u8,233u8,233u8,233u8,156u8,143u8,143u8,143u8,159u8,33u8,33u8,33u8,161u8,14u8,14u8,14u8,30u8,92u8,92u8,92u8,216u8]}
pub open spec fn expected_value_filter_3_ct4_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct4_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct4_d8()
    ensures match sm::MS(bytes_filter_3_ct4_d8()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct4_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct4_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct4_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct4_d8(),false),result),
        se::eq(bytes_filter_3_ct4_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct4_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8,48u8,3u8,23u8,115u8,82u8,182u8,110u8,40u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct4_d8());model_complete_filter_3_ct4_d8();}
    crate::png::try_decode_still(&b)
}
// filter/3-ct6-d16
pub open spec fn bytes_filter_3_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8,237u8,8u8,98u8,251u8,81u8,136u8,38u8,157u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_3_ct6_d16() -> Seq<u8> {seq![204u8,89u8,104u8,152u8,167u8,246u8,217u8,213u8,141u8,148u8,211u8,50u8,29u8,229u8,202u8,236u8,124u8,142u8,247u8,126u8,30u8,210u8,243u8,251u8,123u8,55u8,161u8,22u8,115u8,222u8,74u8,214u8,189u8,60u8,76u8,193u8,244u8,7u8,40u8,66u8,30u8,22u8,80u8,33u8,160u8,105u8,25u8,105u8,210u8,241u8,191u8,62u8,107u8,21u8,176u8,243u8,179u8,235u8,139u8,154u8,58u8,238u8,64u8,225u8,55u8,88u8,5u8,32u8,234u8,107u8,237u8,104u8,3u8,225u8,4u8,18u8,101u8,142u8,59u8,43u8,210u8,133u8,73u8,165u8,204u8,193u8,193u8,134u8,235u8,92u8,131u8,39u8,141u8,89u8,197u8,213u8]}
pub open spec fn expected_value_filter_3_ct6_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_3_ct6_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_3_ct6_d16()
    ensures match sm::MS(bytes_filter_3_ct6_d16()) {sm::Outcome::Done(value)=>expected_value_filter_3_ct6_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_3_ct6_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_3_ct6_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_3_ct6_d16(),false),result),
        se::eq(bytes_filter_3_ct6_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_3_ct6_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8,237u8,8u8,98u8,251u8,81u8,136u8,38u8,157u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct6_d16());model_complete_filter_3_ct6_d16();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct0-d1
pub open spec fn bytes_filter_4_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,49u8,97u8,153u8,192u8,178u8,129u8,197u8,0u8,0u8,6u8,212u8,1u8,181u8,192u8,200u8,214u8,106u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct0_d1() -> Seq<u8> {seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8]}
pub open spec fn expected_value_filter_4_ct0_d1(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct0_d1()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct0_d1()
    ensures match sm::MS(bytes_filter_4_ct0_d1()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct0_d1(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct0_d1();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct0_d1() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct0_d1(),false),result),
        se::eq(bytes_filter_4_ct0_d1(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct0_d1(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,49u8,97u8,153u8,192u8,178u8,129u8,197u8,0u8,0u8,6u8,212u8,1u8,181u8,192u8,200u8,214u8,106u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d1());model_complete_filter_4_ct0_d1();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct0-d8
pub open spec fn bytes_filter_4_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,241u8,139u8,94u8,48u8,189u8,117u8,21u8,75u8,221u8,169u8,183u8,28u8,230u8,234u8,44u8,110u8,90u8,219u8,174u8,20u8,213u8,177u8,20u8,71u8,111u8,108u8,102u8,218u8,11u8,0u8,161u8,221u8,11u8,102u8,173u8,20u8,183u8,50u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct0_d8() -> Seq<u8> {seq![78u8,78u8,78u8,255u8,169u8,169u8,169u8,255u8,73u8,73u8,73u8,255u8,224u8,224u8,224u8,255u8,101u8,101u8,101u8,255u8,15u8,15u8,15u8,255u8,204u8,204u8,204u8,255u8,150u8,150u8,150u8,255u8,54u8,54u8,54u8,255u8,232u8,232u8,232u8,255u8,156u8,156u8,156u8,255u8,140u8,140u8,140u8,255u8,18u8,18u8,18u8,255u8,60u8,60u8,60u8,255u8,236u8,236u8,236u8,255u8,192u8,192u8,192u8,255u8,14u8,14u8,14u8,255u8,140u8,140u8,140u8,255u8,133u8,133u8,133u8,255u8,224u8,224u8,224u8,255u8,157u8,157u8,157u8,255u8,32u8,32u8,32u8,255u8,16u8,16u8,16u8,255u8,73u8,73u8,73u8,255u8]}
pub open spec fn expected_value_filter_4_ct0_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct0_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct0_d8()
    ensures match sm::MS(bytes_filter_4_ct0_d8()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct0_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct0_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct0_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct0_d8(),false),result),
        se::eq(bytes_filter_4_ct0_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct0_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,241u8,139u8,94u8,48u8,189u8,117u8,21u8,75u8,221u8,169u8,183u8,28u8,230u8,234u8,44u8,110u8,90u8,219u8,174u8,20u8,213u8,177u8,20u8,71u8,111u8,108u8,102u8,218u8,11u8,0u8,161u8,221u8,11u8,102u8,173u8,20u8,183u8,50u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d8());model_complete_filter_4_ct0_d8();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct0-d16
pub open spec fn bytes_filter_4_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8,20u8,98u8,23u8,38u8,193u8,105u8,106u8,83u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct0_d16() -> Seq<u8> {seq![229u8,229u8,229u8,255u8,8u8,8u8,8u8,255u8,7u8,7u8,7u8,255u8,29u8,29u8,29u8,255u8,99u8,99u8,99u8,255u8,210u8,210u8,210u8,255u8,152u8,152u8,152u8,255u8,89u8,89u8,89u8,255u8,220u8,220u8,220u8,255u8,245u8,245u8,245u8,255u8,40u8,40u8,40u8,255u8,151u8,151u8,151u8,255u8,89u8,89u8,89u8,255u8,189u8,189u8,189u8,255u8,155u8,155u8,155u8,255u8,48u8,48u8,48u8,255u8,10u8,10u8,10u8,255u8,140u8,140u8,140u8,255u8,7u8,7u8,7u8,255u8,204u8,204u8,204u8,255u8,63u8,63u8,63u8,255u8,179u8,179u8,179u8,255u8,5u8,5u8,5u8,255u8,172u8,172u8,172u8,255u8]}
pub open spec fn expected_value_filter_4_ct0_d16(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct0_d16()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct0_d16()
    ensures match sm::MS(bytes_filter_4_ct0_d16()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct0_d16(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct0_d16();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct0_d16() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct0_d16(),false),result),
        se::eq(bytes_filter_4_ct0_d16(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct0_d16(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8,20u8,98u8,23u8,38u8,193u8,105u8,106u8,83u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d16());model_complete_filter_4_ct0_d16();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct2-d8
pub open spec fn bytes_filter_4_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8,129u8,55u8,37u8,15u8,188u8,226u8,23u8,84u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct2_d8() -> Seq<u8> {seq![101u8,154u8,20u8,255u8,85u8,3u8,76u8,255u8,26u8,72u8,248u8,255u8,151u8,28u8,169u8,255u8,170u8,150u8,55u8,255u8,204u8,86u8,247u8,255u8,219u8,20u8,232u8,255u8,173u8,157u8,243u8,255u8,152u8,146u8,237u8,255u8,5u8,185u8,173u8,255u8,51u8,114u8,102u8,255u8,9u8,226u8,155u8,255u8,254u8,245u8,137u8,255u8,51u8,37u8,39u8,255u8,45u8,238u8,74u8,255u8,139u8,147u8,225u8,255u8,3u8,87u8,229u8,255u8,159u8,248u8,157u8,255u8,101u8,155u8,31u8,255u8,153u8,80u8,246u8,255u8,102u8,7u8,71u8,255u8,214u8,3u8,82u8,255u8,251u8,157u8,79u8,255u8,251u8,156u8,183u8,255u8]}
pub open spec fn expected_value_filter_4_ct2_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct2_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct2_d8()
    ensures match sm::MS(bytes_filter_4_ct2_d8()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct2_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct2_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct2_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct2_d8(),false),result),
        se::eq(bytes_filter_4_ct2_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct2_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8,129u8,55u8,37u8,15u8,188u8,226u8,23u8,84u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct2_d8());model_complete_filter_4_ct2_d8();}
    crate::png::try_decode_still(&b)
}
// filter/4-ct4-d8
pub open spec fn bytes_filter_4_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8,127u8,37u8,24u8,132u8,210u8,72u8,235u8,233u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
pub open spec fn expected_filter_4_ct4_d8() -> Seq<u8> {seq![33u8,33u8,33u8,90u8,57u8,57u8,57u8,198u8,171u8,171u8,171u8,34u8,136u8,136u8,136u8,31u8,84u8,84u8,84u8,205u8,152u8,152u8,152u8,118u8,160u8,160u8,160u8,30u8,248u8,248u8,248u8,46u8,9u8,9u8,9u8,107u8,111u8,111u8,111u8,188u8,40u8,40u8,40u8,25u8,77u8,77u8,77u8,186u8,101u8,101u8,101u8,152u8,12u8,12u8,12u8,39u8,100u8,100u8,100u8,45u8,44u8,44u8,44u8,235u8,189u8,189u8,189u8,59u8,46u8,46u8,46u8,66u8,217u8,217u8,217u8,83u8,153u8,153u8,153u8,237u8,80u8,80u8,80u8,87u8,223u8,223u8,223u8,159u8,150u8,150u8,150u8,167u8,65u8,65u8,65u8,167u8]}
pub open spec fn expected_value_filter_4_ct4_d8(value: sm::Value) -> bool {value.width==6u32 && value.height==4u32 && value.pixels==expected_filter_4_ct4_d8()}
#[verifier::spinoff_prover]
pub proof fn model_complete_filter_4_ct4_d8()
    ensures match sm::MS(bytes_filter_4_ct4_d8()) {sm::Outcome::Done(value)=>expected_value_filter_4_ct4_d8(value),_=>false},
{
    crate::witness_m4_filter_projection::e6_complete_filter_4_ct4_d8();
}
#[verifier::spinoff_prover]
pub fn public_complete_filter_4_ct4_d8() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_filter_4_ct4_d8(),false),result),
        se::eq(bytes_filter_4_ct4_d8(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_filter_4_ct4_d8(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8,127u8,37u8,24u8,132u8,210u8,72u8,235u8,233u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct4_d8());model_complete_filter_4_ct4_d8();}
    crate::png::try_decode_still(&b)
}
} // verus!
