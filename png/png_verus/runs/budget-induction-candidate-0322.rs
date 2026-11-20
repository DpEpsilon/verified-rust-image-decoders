//! Symbolic-index APNG budget attempt; certificates come from authored frozen bytes.
use vstd::prelude::*;
use crate::{document_model as m,apng_model as a,walk_model as w,critical_model as c,png::Error,alloc_guard::sites::TryError};
verus! {
spec fn header() -> crate::model::Ihdr {crate::model::Ihdr {width:8192,height:2048,depth:8,color_type:6,interlace:0}}
spec fn prefix_bytes(count: u32) -> Seq<u8> {if count==64 {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,8u8,6u8,0u8,0u8,0u8,142u8,248u8,189u8,162u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,64u8,0u8,0u8,0u8,0u8,209u8,190u8,152u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8]} else {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,8u8,6u8,0u8,0u8,0u8,142u8,248u8,189u8,162u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,65u8,0u8,0u8,0u8,0u8,236u8,222u8,177u8,233u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8]}}
spec fn pair(k: nat) -> Seq<u8> {
if k==0 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,137u8,164u8,36u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,1u8,157u8,114u8,106u8,97u8]}
else if k==1 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,31u8,119u8,205u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,1u8,158u8,246u8,190u8,15u8]}
else if k==2 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,4u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,213u8,5u8,183u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,120u8,1u8,154u8,123u8,194u8,189u8]}
else if k==3 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,6u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,67u8,214u8,94u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,7u8,120u8,1u8,153u8,255u8,22u8,211u8]}
else if k==4 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,8u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,48u8,231u8,2u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,9u8,120u8,1u8,147u8,97u8,59u8,217u8]}
else if k==5 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,10u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,166u8,52u8,235u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,11u8,120u8,1u8,144u8,229u8,239u8,183u8]}
else if k==6 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,12u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,108u8,70u8,145u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,13u8,120u8,1u8,148u8,104u8,147u8,5u8]}
else if k==7 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,14u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,250u8,149u8,120u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,15u8,120u8,1u8,151u8,236u8,71u8,107u8]}
else if k==8 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,16u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,251u8,34u8,104u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,17u8,120u8,1u8,129u8,84u8,201u8,17u8]}
else if k==9 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,18u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,109u8,241u8,129u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,19u8,120u8,1u8,130u8,208u8,29u8,127u8]}
else if k==10 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,20u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,167u8,131u8,251u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,21u8,120u8,1u8,134u8,93u8,97u8,205u8]}
else if k==11 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,22u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,49u8,80u8,18u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,23u8,120u8,1u8,133u8,217u8,181u8,163u8]}
else if k==12 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,24u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,66u8,97u8,78u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,25u8,120u8,1u8,143u8,71u8,152u8,169u8]}
else if k==13 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,26u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,212u8,178u8,167u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,27u8,120u8,1u8,140u8,195u8,76u8,199u8]}
else if k==14 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,28u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,30u8,192u8,221u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,29u8,120u8,1u8,136u8,78u8,48u8,117u8]}
else if k==15 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,30u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,136u8,19u8,52u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,31u8,120u8,1u8,139u8,202u8,228u8,27u8]}
else if k==16 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,32u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,108u8,168u8,188u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,33u8,120u8,1u8,165u8,63u8,44u8,129u8]}
else if k==17 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,34u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,250u8,123u8,85u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,35u8,120u8,1u8,166u8,187u8,248u8,239u8]}
else if k==18 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,36u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,48u8,9u8,47u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,37u8,120u8,1u8,162u8,54u8,132u8,93u8]}
else if k==19 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,38u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,166u8,218u8,198u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,39u8,120u8,1u8,161u8,178u8,80u8,51u8]}
else if k==20 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,40u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,213u8,235u8,154u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,41u8,120u8,1u8,171u8,44u8,125u8,57u8]}
else if k==21 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,42u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,67u8,56u8,115u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,43u8,120u8,1u8,168u8,168u8,169u8,87u8]}
else if k==22 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,44u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,137u8,74u8,9u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,45u8,120u8,1u8,172u8,37u8,213u8,229u8]}
else if k==23 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,46u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,31u8,153u8,224u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,47u8,120u8,1u8,175u8,161u8,1u8,139u8]}
else if k==24 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,48u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,30u8,46u8,240u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,49u8,120u8,1u8,185u8,25u8,143u8,241u8]}
else if k==25 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,50u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,136u8,253u8,25u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,51u8,120u8,1u8,186u8,157u8,91u8,159u8]}
else if k==26 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,52u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,66u8,143u8,99u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,53u8,120u8,1u8,190u8,16u8,39u8,45u8]}
else if k==27 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,54u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,212u8,92u8,138u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,55u8,120u8,1u8,189u8,148u8,243u8,67u8]}
else if k==28 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,56u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,167u8,109u8,214u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,57u8,120u8,1u8,183u8,10u8,222u8,73u8]}
else if k==29 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,58u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,49u8,190u8,63u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,59u8,120u8,1u8,180u8,142u8,10u8,39u8]}
else if k==30 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,60u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,251u8,204u8,69u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,61u8,120u8,1u8,176u8,3u8,118u8,149u8]}
else if k==31 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,62u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,109u8,31u8,172u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,63u8,120u8,1u8,179u8,135u8,162u8,251u8]}
else if k==32 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,64u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,67u8,189u8,20u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,65u8,120u8,1u8,237u8,232u8,231u8,161u8]}
else if k==33 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,66u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,213u8,110u8,253u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,67u8,120u8,1u8,238u8,108u8,51u8,207u8]}
else if k==34 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,68u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,31u8,28u8,135u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,69u8,120u8,1u8,234u8,225u8,79u8,125u8]}
else if k==35 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,70u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,137u8,207u8,110u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,71u8,120u8,1u8,233u8,101u8,155u8,19u8]}
else if k==36 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,72u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,250u8,254u8,50u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,73u8,120u8,1u8,227u8,251u8,182u8,25u8]}
else if k==37 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,74u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,108u8,45u8,219u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,75u8,120u8,1u8,224u8,127u8,98u8,119u8]}
else if k==38 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,76u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,166u8,95u8,161u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,77u8,120u8,1u8,228u8,242u8,30u8,197u8]}
else if k==39 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,78u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,48u8,140u8,72u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,79u8,120u8,1u8,231u8,118u8,202u8,171u8]}
else if k==40 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,80u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,49u8,59u8,88u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,81u8,120u8,1u8,241u8,206u8,68u8,209u8]}
else if k==41 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,167u8,232u8,177u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,83u8,120u8,1u8,242u8,74u8,144u8,191u8]}
else if k==42 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,84u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,109u8,154u8,203u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,85u8,120u8,1u8,246u8,199u8,236u8,13u8]}
else if k==43 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,86u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,251u8,73u8,34u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,87u8,120u8,1u8,245u8,67u8,56u8,99u8]}
else if k==44 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,88u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,136u8,120u8,126u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,89u8,120u8,1u8,255u8,221u8,21u8,105u8]}
else if k==45 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,90u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,30u8,171u8,151u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,91u8,120u8,1u8,252u8,89u8,193u8,7u8]}
else if k==46 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,92u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,212u8,217u8,237u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,93u8,120u8,1u8,248u8,212u8,189u8,181u8]}
else if k==47 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,94u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,66u8,10u8,4u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,95u8,120u8,1u8,251u8,80u8,105u8,219u8]}
else if k==48 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,96u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,166u8,177u8,140u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,97u8,120u8,1u8,213u8,165u8,161u8,65u8]}
else if k==49 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,98u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,48u8,98u8,101u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,99u8,120u8,1u8,214u8,33u8,117u8,47u8]}
else if k==50 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,100u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,250u8,16u8,31u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,101u8,120u8,1u8,210u8,172u8,9u8,157u8]}
else if k==51 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,102u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,108u8,195u8,246u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,103u8,120u8,1u8,209u8,40u8,221u8,243u8]}
else if k==52 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,104u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,31u8,242u8,170u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,105u8,120u8,1u8,219u8,182u8,240u8,249u8]}
else if k==53 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,106u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,137u8,33u8,67u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,107u8,120u8,1u8,216u8,50u8,36u8,151u8]}
else if k==54 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,108u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,67u8,83u8,57u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,109u8,120u8,1u8,220u8,191u8,88u8,37u8]}
else if k==55 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,110u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,213u8,128u8,208u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,111u8,120u8,1u8,223u8,59u8,140u8,75u8]}
else if k==56 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,112u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,212u8,55u8,192u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,113u8,120u8,1u8,201u8,131u8,2u8,49u8]}
else if k==57 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,114u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,66u8,228u8,41u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,115u8,120u8,1u8,202u8,7u8,214u8,95u8]}
else if k==58 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,116u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,136u8,150u8,83u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,117u8,120u8,1u8,206u8,138u8,170u8,237u8]}
else if k==59 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,118u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,30u8,69u8,186u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,119u8,120u8,1u8,205u8,14u8,126u8,131u8]}
else if k==60 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,120u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,109u8,116u8,230u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,121u8,120u8,1u8,199u8,144u8,83u8,137u8]}
else if k==61 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,122u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,251u8,167u8,15u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,123u8,120u8,1u8,196u8,20u8,135u8,231u8]}
else if k==62 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,124u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,49u8,213u8,117u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,125u8,120u8,1u8,192u8,153u8,251u8,85u8]}
else if k==63 {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,126u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,167u8,6u8,156u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,127u8,120u8,1u8,195u8,29u8,47u8,59u8]}
else {seq![0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,128u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,86u8,29u8,150u8,68u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,129u8,120u8,1u8,124u8,71u8,113u8,225u8]}}
spec fn ending() -> Seq<u8> {seq![0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}

spec fn bytes(count: u32) -> Seq<u8> {
    Seq::new(79+56*(count as nat), |i: int|
        if i<67 {prefix_bytes(count)[i]}
        else if i<67+56*(count as int) {pair(((i-67)/56) as nat)[(i-67)%56]}
        else {ending()[i-67-56*(count as int)]})
}
spec fn control(k: nat) -> a::Control {
    a::Control {sequence:2*k,current:if k==0 {a::Current::None}else{a::Current::Frame(true)},
        frame_count:k,fctl_before_idat:false,pixel_total:k*(8192nat*2048nat)}
}
spec fn middle_control(k: nat) -> a::Control {
    a::Control {sequence:2*k+1,current:a::Current::Frame(false),frame_count:k+1,
        fctl_before_idat:false,pixel_total:(k+1)*(8192nat*2048nat)}
}
spec fn frame() -> a::FrameControl {
    a::FrameControl {width:8192,height:2048,x:0,y:0,delay_num:1,delay_den:10,dispose:0,blend:0}
}
#[verifier::opaque]
spec fn wire_good(b: Seq<u8>,k: nat) -> bool {
    let p=67+56*k;
    (forall|phase: w::Phase| #[trigger] c::read(b,p,phase,false)==Ok(c::Candidate {
        ty:seq![102u8,99u8,84u8,76u8],data:p+8,len:26,next:p+38,crc_ok:true,synthetic:false}))
    && (forall|phase: w::Phase| #[trigger] c::read(b,p+38,phase,false)==Ok(c::Candidate {
        ty:seq![102u8,100u8,65u8,84u8],data:p+46,len:6,next:p+56,crc_ok:true,synthetic:false}))
    && a::fctl(b.subrange((p+8) as int,(p+34) as int),true,false,8192,2048,control(k))
        == (if k<64 {Ok((frame(),middle_control(k)))}else{Err((Error::PixelBudgetExceeded,false))})
    && (k<64 ==> a::fdat(b.subrange((p+46) as int,(p+52) as int),true,false,middle_control(k))==Ok(control(k+1)))
}
spec fn prefix_state(s: m::State,k: nat,count: u32) -> bool {
    s.pos==67+56*k && s.doc.ihdr==header() && s.doc.plte==None
    && s.doc.actl==Some((count,0u32)) && s.doc.frames.len()==k
    && s.seen==set![w::Name::Ihdr,w::Name::Actl]
    && s.phase==(if k==0 {w::Phase::During}else{w::Phase::After})
    && s.control==control(k) && s.events==m::no_events()
}
spec fn middle(s: m::State,k: nat) -> m::State {
    m::State {pos:s.pos+38,phase:w::Phase::After,control:middle_control(k),
        doc:m::Doc {frames:s.doc.frames.push(w::Frame {control_offset:s.pos,width:8192,height:2048,
            x:0,y:0,delay_num:1,delay_den:10,dispose:0,blend:0,source:w::Source::Fdat(Seq::empty())}),..s.doc},..s}
}
spec fn after_pair(s: m::State,k: nat) -> m::State {
    let t=middle(s,k);
    m::State {pos:s.pos+56,control:control(k+1),doc:m::Doc {frames:t.doc.frames.update(k as int,
        w::Frame {source:w::Source::Fdat(seq![w::Range {offset:s.pos+50,len:2}]),..t.doc.frames[k as int]}),..t.doc},..t}
}
#[verifier::spinoff_prover]
proof fn tags()
    ensures w::classify(seq![102u8,99u8,84u8,76u8])==w::Name::Fctl,
        w::classify(seq![102u8,100u8,65u8,84u8])==w::Name::Fdat,
        w::classify(seq![73u8,69u8,78u8,68u8])==w::Name::Iend,
{
    assert(w::classify(seq![102u8,99u8,84u8,76u8])==w::Name::Fctl) by (compute);
    assert(w::classify(seq![102u8,100u8,65u8,84u8])==w::Name::Fdat) by (compute);
    assert(w::classify(seq![73u8,69u8,78u8,68u8])==w::Name::Iend) by (compute);
}
#[verifier::spinoff_prover]
proof fn fctl_step(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires k<64, count==64 || count==65, b.len()==79+56*(count as nat),
        prefix_state(s,k,count),wire_good(b,k),
    ensures m::step(b,m::strict(),true,false,s)==m::Step::Next(middle(s,k)),
{
    hide(c::read);hide(a::fctl);hide(a::fdat);hide(w::classify);
    tags();reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn fdat_step(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires k<64, count==64 || count==65,b.len()==79+56*(count as nat),
        prefix_state(s,k,count),wire_good(b,k),
    ensures m::step(b,m::strict(),true,false,middle(s,k))==m::Step::Next(after_pair(s,k)),
        prefix_state(after_pair(s,k),k+1,count),
{
    hide(c::read);hide(a::fctl);hide(a::fdat);hide(w::classify);
    tags();reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    let t=middle(s,k);
    assert(t.doc.frames.len()==k+1);
    assert(t.doc.frames[k as int].source==w::Source::Fdat(Seq::empty()));
    assert(t.doc.frames[k as int].source==w::Source::Fdat(Seq::empty()));
}
spec fn budget_outcome(out: m::Outcome,count: u32) -> bool {
    if count==65 {out==m::Outcome::Error(Error::PixelBudgetExceeded,m::no_events())}
    else {match out {m::Outcome::Done(doc,ev)=>doc.ihdr==header() && ev==m::no_events(),_=>false}}
}
#[verifier::opaque]
spec fn end_good(b: Seq<u8>,count: u32) -> bool {
    if count==65 {wire_good(b,64)}else{
        c::read(b,3651,w::Phase::After,false)==Ok(c::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:3659,len:0,next:3663,crc_ok:true,synthetic:false})}
}
#[verifier::spinoff_prover]
proof fn terminal(b: Seq<u8>,s: m::State,count: u32)
    requires count==64 || count==65,b.len()==79+56*(count as nat),prefix_state(s,64,count),end_good(b,count),
    ensures budget_outcome(m::drive(b,m::strict(),true,false,s),count),
{
    hide(m::drive);hide(c::read);hide(a::fctl);hide(w::classify);
    tags();reveal(end_good);reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    if count==65 {
        assert(m::step(b,m::strict(),true,false,s)==m::Step::Error(Error::PixelBudgetExceeded,m::no_events()));
    } else {
        assert(a::finish(control(64),Some(64u32))==Ok(())) by (compute);
        assert(match m::step(b,m::strict(),true,false,s) {m::Step::Done(doc,ev)=>doc.ihdr==header() && ev==m::no_events(),_=>false});
    }
    m::drive_step(b,m::strict(),true,false,s);
}
#[verifier::spinoff_prover]
proof fn finish(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires count==64 || count==65,k<=64,b.len()==79+56*(count as nat),
        prefix_state(s,k,count),end_good(b,count),
        forall|j: nat| j<64 ==> #[trigger] wire_good(b,j),
    ensures budget_outcome(m::drive(b,m::strict(),true,false,s),count),
    decreases 64-k,
{
    hide(m::drive);
    if k==64 {terminal(b,s,count);}else{
        fctl_step(b,s,k,count);fdat_step(b,s,k,count);
        finish(b,after_pair(s,k),k+1,count);
        m::drive_step(b,m::strict(),true,false,middle(s,k));
        m::drive_step(b,m::strict(),true,false,s);
    }
}
#[verifier::opaque]
spec fn start_good(b: Seq<u8>,count: u32) -> bool {
    crate::model::mh(b)==Ok((header(),33nat))
    && c::read(b,33,w::Phase::Before,false)==Ok(c::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false})
    && a::actl(b.subrange(41,49))==Ok((count,0u32))
    && c::read(b,53,w::Phase::Before,false)==Ok(c::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61,len:2,next:67,crc_ok:true,synthetic:false})
}
spec fn start_one(count: u32) -> m::State {
    let s=m::initial(header(),33,m::no_events());
    m::State {pos:53,seen:s.seen.insert(w::Name::Actl),doc:m::Doc {actl:Some((count,0u32)),..s.doc},..s}
}
spec fn start_two(count: u32) -> m::State {
    let s=start_one(count);
    m::State {pos:67,phase:w::Phase::During,doc:m::Doc {idat:seq![w::Range {offset:61,len:2}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn start(b: Seq<u8>,count: u32)
    requires count==64 || count==65,b.len()==79+56*(count as nat),start_good(b,count),
        end_good(b,count),forall|j: nat| j<64 ==> #[trigger] wire_good(b,j),
    ensures budget_outcome(m::MWF_p(b,m::strict()),count),
{
    hide(m::drive);hide(c::read);hide(a::actl);hide(crate::model::mh);hide(m::initial_info);
    reveal(start_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(m::step(b,m::strict(),true,false,m::initial(header(),33,m::no_events()))==m::Step::Next(start_one(count)));
    assert(m::step(b,m::strict(),true,false,start_one(count))==m::Step::Next(start_two(count)));
    assert(prefix_state(start_two(count),0,count));
    finish(b,start_two(count),0,count);
    m::drive_step(b,m::strict(),true,false,start_one(count));
    m::drive_step(b,m::strict(),true,false,m::initial(header(),33,m::no_events()));
}
#[verifier::spinoff_prover]
proof fn wire_0(count: u32)
    requires (count==64 || count==65), 0nat<count as nat,
    ensures wire_good(bytes(count),0nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),67nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:75nat,len:26nat,next:105nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),67nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:75nat,len:26nat,next:105nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),105nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:113nat,len:6nat,next:123nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),105nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:113nat,len:6nat,next:123nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(75,101),true,false,8192,2048,control(0nat))==Ok((frame(),middle_control(0nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(113,119),true,false,middle_control(0nat))==Ok(control(1nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),67nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:75nat,len:26nat,next:105nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),67nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:75nat,len:26nat,next:105nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),105nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:113nat,len:6nat,next:123nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),105nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:113nat,len:6nat,next:123nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(75,101),true,false,8192,2048,control(0nat))==Ok((frame(),middle_control(0nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(113,119),true,false,middle_control(0nat))==Ok(control(1nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_1(count: u32)
    requires (count==64 || count==65), 1nat<count as nat,
    ensures wire_good(bytes(count),1nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),123nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:131nat,len:26nat,next:161nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),123nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:131nat,len:26nat,next:161nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),161nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:169nat,len:6nat,next:179nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),161nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:169nat,len:6nat,next:179nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(131,157),true,false,8192,2048,control(1nat))==Ok((frame(),middle_control(1nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(169,175),true,false,middle_control(1nat))==Ok(control(2nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),123nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:131nat,len:26nat,next:161nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),123nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:131nat,len:26nat,next:161nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),161nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:169nat,len:6nat,next:179nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),161nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:169nat,len:6nat,next:179nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(131,157),true,false,8192,2048,control(1nat))==Ok((frame(),middle_control(1nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(169,175),true,false,middle_control(1nat))==Ok(control(2nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_2(count: u32)
    requires (count==64 || count==65), 2nat<count as nat,
    ensures wire_good(bytes(count),2nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),179nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:187nat,len:26nat,next:217nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),179nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:187nat,len:26nat,next:217nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),217nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:225nat,len:6nat,next:235nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),217nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:225nat,len:6nat,next:235nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(187,213),true,false,8192,2048,control(2nat))==Ok((frame(),middle_control(2nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(225,231),true,false,middle_control(2nat))==Ok(control(3nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),179nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:187nat,len:26nat,next:217nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),179nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:187nat,len:26nat,next:217nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),217nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:225nat,len:6nat,next:235nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),217nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:225nat,len:6nat,next:235nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(187,213),true,false,8192,2048,control(2nat))==Ok((frame(),middle_control(2nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(225,231),true,false,middle_control(2nat))==Ok(control(3nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_3(count: u32)
    requires (count==64 || count==65), 3nat<count as nat,
    ensures wire_good(bytes(count),3nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),235nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:243nat,len:26nat,next:273nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),235nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:243nat,len:26nat,next:273nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),273nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:281nat,len:6nat,next:291nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),273nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:281nat,len:6nat,next:291nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(243,269),true,false,8192,2048,control(3nat))==Ok((frame(),middle_control(3nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(281,287),true,false,middle_control(3nat))==Ok(control(4nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),235nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:243nat,len:26nat,next:273nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),235nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:243nat,len:26nat,next:273nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),273nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:281nat,len:6nat,next:291nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),273nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:281nat,len:6nat,next:291nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(243,269),true,false,8192,2048,control(3nat))==Ok((frame(),middle_control(3nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(281,287),true,false,middle_control(3nat))==Ok(control(4nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_4(count: u32)
    requires (count==64 || count==65), 4nat<count as nat,
    ensures wire_good(bytes(count),4nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),291nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:299nat,len:26nat,next:329nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),291nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:299nat,len:26nat,next:329nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),329nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:337nat,len:6nat,next:347nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),329nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:337nat,len:6nat,next:347nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(299,325),true,false,8192,2048,control(4nat))==Ok((frame(),middle_control(4nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(337,343),true,false,middle_control(4nat))==Ok(control(5nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),291nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:299nat,len:26nat,next:329nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),291nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:299nat,len:26nat,next:329nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),329nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:337nat,len:6nat,next:347nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),329nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:337nat,len:6nat,next:347nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(299,325),true,false,8192,2048,control(4nat))==Ok((frame(),middle_control(4nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(337,343),true,false,middle_control(4nat))==Ok(control(5nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_5(count: u32)
    requires (count==64 || count==65), 5nat<count as nat,
    ensures wire_good(bytes(count),5nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),347nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:355nat,len:26nat,next:385nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),347nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:355nat,len:26nat,next:385nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),385nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:393nat,len:6nat,next:403nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),385nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:393nat,len:6nat,next:403nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(355,381),true,false,8192,2048,control(5nat))==Ok((frame(),middle_control(5nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(393,399),true,false,middle_control(5nat))==Ok(control(6nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),347nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:355nat,len:26nat,next:385nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),347nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:355nat,len:26nat,next:385nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),385nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:393nat,len:6nat,next:403nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),385nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:393nat,len:6nat,next:403nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(355,381),true,false,8192,2048,control(5nat))==Ok((frame(),middle_control(5nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(393,399),true,false,middle_control(5nat))==Ok(control(6nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_6(count: u32)
    requires (count==64 || count==65), 6nat<count as nat,
    ensures wire_good(bytes(count),6nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),403nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:411nat,len:26nat,next:441nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),403nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:411nat,len:26nat,next:441nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),441nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:449nat,len:6nat,next:459nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),441nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:449nat,len:6nat,next:459nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(411,437),true,false,8192,2048,control(6nat))==Ok((frame(),middle_control(6nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(449,455),true,false,middle_control(6nat))==Ok(control(7nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),403nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:411nat,len:26nat,next:441nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),403nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:411nat,len:26nat,next:441nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),441nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:449nat,len:6nat,next:459nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),441nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:449nat,len:6nat,next:459nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(411,437),true,false,8192,2048,control(6nat))==Ok((frame(),middle_control(6nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(449,455),true,false,middle_control(6nat))==Ok(control(7nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_7(count: u32)
    requires (count==64 || count==65), 7nat<count as nat,
    ensures wire_good(bytes(count),7nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),459nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:467nat,len:26nat,next:497nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),459nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:467nat,len:26nat,next:497nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),497nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:505nat,len:6nat,next:515nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),497nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:505nat,len:6nat,next:515nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(467,493),true,false,8192,2048,control(7nat))==Ok((frame(),middle_control(7nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(505,511),true,false,middle_control(7nat))==Ok(control(8nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),459nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:467nat,len:26nat,next:497nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),459nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:467nat,len:26nat,next:497nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),497nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:505nat,len:6nat,next:515nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),497nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:505nat,len:6nat,next:515nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(467,493),true,false,8192,2048,control(7nat))==Ok((frame(),middle_control(7nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(505,511),true,false,middle_control(7nat))==Ok(control(8nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_8(count: u32)
    requires (count==64 || count==65), 8nat<count as nat,
    ensures wire_good(bytes(count),8nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),515nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:523nat,len:26nat,next:553nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),515nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:523nat,len:26nat,next:553nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),553nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:561nat,len:6nat,next:571nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),553nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:561nat,len:6nat,next:571nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(523,549),true,false,8192,2048,control(8nat))==Ok((frame(),middle_control(8nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(561,567),true,false,middle_control(8nat))==Ok(control(9nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),515nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:523nat,len:26nat,next:553nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),515nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:523nat,len:26nat,next:553nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),553nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:561nat,len:6nat,next:571nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),553nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:561nat,len:6nat,next:571nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(523,549),true,false,8192,2048,control(8nat))==Ok((frame(),middle_control(8nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(561,567),true,false,middle_control(8nat))==Ok(control(9nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_9(count: u32)
    requires (count==64 || count==65), 9nat<count as nat,
    ensures wire_good(bytes(count),9nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),571nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:579nat,len:26nat,next:609nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),571nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:579nat,len:26nat,next:609nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),609nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:617nat,len:6nat,next:627nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),609nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:617nat,len:6nat,next:627nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(579,605),true,false,8192,2048,control(9nat))==Ok((frame(),middle_control(9nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(617,623),true,false,middle_control(9nat))==Ok(control(10nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),571nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:579nat,len:26nat,next:609nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),571nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:579nat,len:26nat,next:609nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),609nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:617nat,len:6nat,next:627nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),609nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:617nat,len:6nat,next:627nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(579,605),true,false,8192,2048,control(9nat))==Ok((frame(),middle_control(9nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(617,623),true,false,middle_control(9nat))==Ok(control(10nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_10(count: u32)
    requires (count==64 || count==65), 10nat<count as nat,
    ensures wire_good(bytes(count),10nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),627nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:635nat,len:26nat,next:665nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),627nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:635nat,len:26nat,next:665nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),665nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:673nat,len:6nat,next:683nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),665nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:673nat,len:6nat,next:683nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(635,661),true,false,8192,2048,control(10nat))==Ok((frame(),middle_control(10nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(673,679),true,false,middle_control(10nat))==Ok(control(11nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),627nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:635nat,len:26nat,next:665nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),627nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:635nat,len:26nat,next:665nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),665nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:673nat,len:6nat,next:683nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),665nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:673nat,len:6nat,next:683nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(635,661),true,false,8192,2048,control(10nat))==Ok((frame(),middle_control(10nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(673,679),true,false,middle_control(10nat))==Ok(control(11nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_11(count: u32)
    requires (count==64 || count==65), 11nat<count as nat,
    ensures wire_good(bytes(count),11nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),683nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:691nat,len:26nat,next:721nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),683nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:691nat,len:26nat,next:721nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),721nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:729nat,len:6nat,next:739nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),721nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:729nat,len:6nat,next:739nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(691,717),true,false,8192,2048,control(11nat))==Ok((frame(),middle_control(11nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(729,735),true,false,middle_control(11nat))==Ok(control(12nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),683nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:691nat,len:26nat,next:721nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),683nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:691nat,len:26nat,next:721nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),721nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:729nat,len:6nat,next:739nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),721nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:729nat,len:6nat,next:739nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(691,717),true,false,8192,2048,control(11nat))==Ok((frame(),middle_control(11nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(729,735),true,false,middle_control(11nat))==Ok(control(12nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_12(count: u32)
    requires (count==64 || count==65), 12nat<count as nat,
    ensures wire_good(bytes(count),12nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),739nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:747nat,len:26nat,next:777nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),739nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:747nat,len:26nat,next:777nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),777nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:785nat,len:6nat,next:795nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),777nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:785nat,len:6nat,next:795nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(747,773),true,false,8192,2048,control(12nat))==Ok((frame(),middle_control(12nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(785,791),true,false,middle_control(12nat))==Ok(control(13nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),739nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:747nat,len:26nat,next:777nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),739nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:747nat,len:26nat,next:777nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),777nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:785nat,len:6nat,next:795nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),777nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:785nat,len:6nat,next:795nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(747,773),true,false,8192,2048,control(12nat))==Ok((frame(),middle_control(12nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(785,791),true,false,middle_control(12nat))==Ok(control(13nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_13(count: u32)
    requires (count==64 || count==65), 13nat<count as nat,
    ensures wire_good(bytes(count),13nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),795nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:803nat,len:26nat,next:833nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),795nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:803nat,len:26nat,next:833nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),833nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:841nat,len:6nat,next:851nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),833nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:841nat,len:6nat,next:851nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(803,829),true,false,8192,2048,control(13nat))==Ok((frame(),middle_control(13nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(841,847),true,false,middle_control(13nat))==Ok(control(14nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),795nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:803nat,len:26nat,next:833nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),795nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:803nat,len:26nat,next:833nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),833nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:841nat,len:6nat,next:851nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),833nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:841nat,len:6nat,next:851nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(803,829),true,false,8192,2048,control(13nat))==Ok((frame(),middle_control(13nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(841,847),true,false,middle_control(13nat))==Ok(control(14nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_14(count: u32)
    requires (count==64 || count==65), 14nat<count as nat,
    ensures wire_good(bytes(count),14nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),851nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:859nat,len:26nat,next:889nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),851nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:859nat,len:26nat,next:889nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),889nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:897nat,len:6nat,next:907nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),889nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:897nat,len:6nat,next:907nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(859,885),true,false,8192,2048,control(14nat))==Ok((frame(),middle_control(14nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(897,903),true,false,middle_control(14nat))==Ok(control(15nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),851nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:859nat,len:26nat,next:889nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),851nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:859nat,len:26nat,next:889nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),889nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:897nat,len:6nat,next:907nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),889nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:897nat,len:6nat,next:907nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(859,885),true,false,8192,2048,control(14nat))==Ok((frame(),middle_control(14nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(897,903),true,false,middle_control(14nat))==Ok(control(15nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_15(count: u32)
    requires (count==64 || count==65), 15nat<count as nat,
    ensures wire_good(bytes(count),15nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),907nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:915nat,len:26nat,next:945nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),907nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:915nat,len:26nat,next:945nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),945nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:953nat,len:6nat,next:963nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),945nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:953nat,len:6nat,next:963nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(915,941),true,false,8192,2048,control(15nat))==Ok((frame(),middle_control(15nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(953,959),true,false,middle_control(15nat))==Ok(control(16nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),907nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:915nat,len:26nat,next:945nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),907nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:915nat,len:26nat,next:945nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),945nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:953nat,len:6nat,next:963nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),945nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:953nat,len:6nat,next:963nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(915,941),true,false,8192,2048,control(15nat))==Ok((frame(),middle_control(15nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(953,959),true,false,middle_control(15nat))==Ok(control(16nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_16(count: u32)
    requires (count==64 || count==65), 16nat<count as nat,
    ensures wire_good(bytes(count),16nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),963nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:971nat,len:26nat,next:1001nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),963nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:971nat,len:26nat,next:1001nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1001nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1009nat,len:6nat,next:1019nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1001nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1009nat,len:6nat,next:1019nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(971,997),true,false,8192,2048,control(16nat))==Ok((frame(),middle_control(16nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1009,1015),true,false,middle_control(16nat))==Ok(control(17nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),963nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:971nat,len:26nat,next:1001nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),963nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:971nat,len:26nat,next:1001nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1001nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1009nat,len:6nat,next:1019nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1001nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1009nat,len:6nat,next:1019nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(971,997),true,false,8192,2048,control(16nat))==Ok((frame(),middle_control(16nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1009,1015),true,false,middle_control(16nat))==Ok(control(17nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_17(count: u32)
    requires (count==64 || count==65), 17nat<count as nat,
    ensures wire_good(bytes(count),17nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1019nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1027nat,len:26nat,next:1057nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1019nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1027nat,len:26nat,next:1057nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1057nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1065nat,len:6nat,next:1075nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1057nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1065nat,len:6nat,next:1075nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1027,1053),true,false,8192,2048,control(17nat))==Ok((frame(),middle_control(17nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1065,1071),true,false,middle_control(17nat))==Ok(control(18nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1019nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1027nat,len:26nat,next:1057nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1019nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1027nat,len:26nat,next:1057nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1057nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1065nat,len:6nat,next:1075nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1057nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1065nat,len:6nat,next:1075nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1027,1053),true,false,8192,2048,control(17nat))==Ok((frame(),middle_control(17nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1065,1071),true,false,middle_control(17nat))==Ok(control(18nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_18(count: u32)
    requires (count==64 || count==65), 18nat<count as nat,
    ensures wire_good(bytes(count),18nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1075nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1083nat,len:26nat,next:1113nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1075nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1083nat,len:26nat,next:1113nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1113nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1121nat,len:6nat,next:1131nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1113nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1121nat,len:6nat,next:1131nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1083,1109),true,false,8192,2048,control(18nat))==Ok((frame(),middle_control(18nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1121,1127),true,false,middle_control(18nat))==Ok(control(19nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1075nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1083nat,len:26nat,next:1113nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1075nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1083nat,len:26nat,next:1113nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1113nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1121nat,len:6nat,next:1131nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1113nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1121nat,len:6nat,next:1131nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1083,1109),true,false,8192,2048,control(18nat))==Ok((frame(),middle_control(18nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1121,1127),true,false,middle_control(18nat))==Ok(control(19nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_19(count: u32)
    requires (count==64 || count==65), 19nat<count as nat,
    ensures wire_good(bytes(count),19nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1131nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1139nat,len:26nat,next:1169nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1131nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1139nat,len:26nat,next:1169nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1169nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1177nat,len:6nat,next:1187nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1169nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1177nat,len:6nat,next:1187nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1139,1165),true,false,8192,2048,control(19nat))==Ok((frame(),middle_control(19nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1177,1183),true,false,middle_control(19nat))==Ok(control(20nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1131nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1139nat,len:26nat,next:1169nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1131nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1139nat,len:26nat,next:1169nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1169nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1177nat,len:6nat,next:1187nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1169nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1177nat,len:6nat,next:1187nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1139,1165),true,false,8192,2048,control(19nat))==Ok((frame(),middle_control(19nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1177,1183),true,false,middle_control(19nat))==Ok(control(20nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_20(count: u32)
    requires (count==64 || count==65), 20nat<count as nat,
    ensures wire_good(bytes(count),20nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1187nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1195nat,len:26nat,next:1225nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1187nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1195nat,len:26nat,next:1225nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1225nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1233nat,len:6nat,next:1243nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1225nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1233nat,len:6nat,next:1243nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1195,1221),true,false,8192,2048,control(20nat))==Ok((frame(),middle_control(20nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1233,1239),true,false,middle_control(20nat))==Ok(control(21nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1187nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1195nat,len:26nat,next:1225nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1187nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1195nat,len:26nat,next:1225nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1225nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1233nat,len:6nat,next:1243nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1225nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1233nat,len:6nat,next:1243nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1195,1221),true,false,8192,2048,control(20nat))==Ok((frame(),middle_control(20nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1233,1239),true,false,middle_control(20nat))==Ok(control(21nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_21(count: u32)
    requires (count==64 || count==65), 21nat<count as nat,
    ensures wire_good(bytes(count),21nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1243nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1251nat,len:26nat,next:1281nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1243nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1251nat,len:26nat,next:1281nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1281nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1289nat,len:6nat,next:1299nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1281nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1289nat,len:6nat,next:1299nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1251,1277),true,false,8192,2048,control(21nat))==Ok((frame(),middle_control(21nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1289,1295),true,false,middle_control(21nat))==Ok(control(22nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1243nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1251nat,len:26nat,next:1281nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1243nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1251nat,len:26nat,next:1281nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1281nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1289nat,len:6nat,next:1299nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1281nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1289nat,len:6nat,next:1299nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1251,1277),true,false,8192,2048,control(21nat))==Ok((frame(),middle_control(21nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1289,1295),true,false,middle_control(21nat))==Ok(control(22nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_22(count: u32)
    requires (count==64 || count==65), 22nat<count as nat,
    ensures wire_good(bytes(count),22nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1299nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1307nat,len:26nat,next:1337nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1299nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1307nat,len:26nat,next:1337nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1337nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1345nat,len:6nat,next:1355nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1337nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1345nat,len:6nat,next:1355nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1307,1333),true,false,8192,2048,control(22nat))==Ok((frame(),middle_control(22nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1345,1351),true,false,middle_control(22nat))==Ok(control(23nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1299nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1307nat,len:26nat,next:1337nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1299nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1307nat,len:26nat,next:1337nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1337nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1345nat,len:6nat,next:1355nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1337nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1345nat,len:6nat,next:1355nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1307,1333),true,false,8192,2048,control(22nat))==Ok((frame(),middle_control(22nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1345,1351),true,false,middle_control(22nat))==Ok(control(23nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_23(count: u32)
    requires (count==64 || count==65), 23nat<count as nat,
    ensures wire_good(bytes(count),23nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1355nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1363nat,len:26nat,next:1393nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1355nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1363nat,len:26nat,next:1393nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1393nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1401nat,len:6nat,next:1411nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1393nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1401nat,len:6nat,next:1411nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1363,1389),true,false,8192,2048,control(23nat))==Ok((frame(),middle_control(23nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1401,1407),true,false,middle_control(23nat))==Ok(control(24nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1355nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1363nat,len:26nat,next:1393nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1355nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1363nat,len:26nat,next:1393nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1393nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1401nat,len:6nat,next:1411nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1393nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1401nat,len:6nat,next:1411nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1363,1389),true,false,8192,2048,control(23nat))==Ok((frame(),middle_control(23nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1401,1407),true,false,middle_control(23nat))==Ok(control(24nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_24(count: u32)
    requires (count==64 || count==65), 24nat<count as nat,
    ensures wire_good(bytes(count),24nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1411nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1419nat,len:26nat,next:1449nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1411nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1419nat,len:26nat,next:1449nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1449nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1457nat,len:6nat,next:1467nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1449nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1457nat,len:6nat,next:1467nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1419,1445),true,false,8192,2048,control(24nat))==Ok((frame(),middle_control(24nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1457,1463),true,false,middle_control(24nat))==Ok(control(25nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1411nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1419nat,len:26nat,next:1449nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1411nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1419nat,len:26nat,next:1449nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1449nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1457nat,len:6nat,next:1467nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1449nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1457nat,len:6nat,next:1467nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1419,1445),true,false,8192,2048,control(24nat))==Ok((frame(),middle_control(24nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1457,1463),true,false,middle_control(24nat))==Ok(control(25nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_25(count: u32)
    requires (count==64 || count==65), 25nat<count as nat,
    ensures wire_good(bytes(count),25nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1467nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1475nat,len:26nat,next:1505nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1467nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1475nat,len:26nat,next:1505nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1505nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1513nat,len:6nat,next:1523nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1505nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1513nat,len:6nat,next:1523nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1475,1501),true,false,8192,2048,control(25nat))==Ok((frame(),middle_control(25nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1513,1519),true,false,middle_control(25nat))==Ok(control(26nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1467nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1475nat,len:26nat,next:1505nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1467nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1475nat,len:26nat,next:1505nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1505nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1513nat,len:6nat,next:1523nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1505nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1513nat,len:6nat,next:1523nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1475,1501),true,false,8192,2048,control(25nat))==Ok((frame(),middle_control(25nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1513,1519),true,false,middle_control(25nat))==Ok(control(26nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_26(count: u32)
    requires (count==64 || count==65), 26nat<count as nat,
    ensures wire_good(bytes(count),26nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1523nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1531nat,len:26nat,next:1561nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1523nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1531nat,len:26nat,next:1561nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1561nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1569nat,len:6nat,next:1579nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1561nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1569nat,len:6nat,next:1579nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1531,1557),true,false,8192,2048,control(26nat))==Ok((frame(),middle_control(26nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1569,1575),true,false,middle_control(26nat))==Ok(control(27nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1523nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1531nat,len:26nat,next:1561nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1523nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1531nat,len:26nat,next:1561nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1561nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1569nat,len:6nat,next:1579nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1561nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1569nat,len:6nat,next:1579nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1531,1557),true,false,8192,2048,control(26nat))==Ok((frame(),middle_control(26nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1569,1575),true,false,middle_control(26nat))==Ok(control(27nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_27(count: u32)
    requires (count==64 || count==65), 27nat<count as nat,
    ensures wire_good(bytes(count),27nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1579nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1587nat,len:26nat,next:1617nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1579nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1587nat,len:26nat,next:1617nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1617nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1625nat,len:6nat,next:1635nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1617nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1625nat,len:6nat,next:1635nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1587,1613),true,false,8192,2048,control(27nat))==Ok((frame(),middle_control(27nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1625,1631),true,false,middle_control(27nat))==Ok(control(28nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1579nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1587nat,len:26nat,next:1617nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1579nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1587nat,len:26nat,next:1617nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1617nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1625nat,len:6nat,next:1635nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1617nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1625nat,len:6nat,next:1635nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1587,1613),true,false,8192,2048,control(27nat))==Ok((frame(),middle_control(27nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1625,1631),true,false,middle_control(27nat))==Ok(control(28nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_28(count: u32)
    requires (count==64 || count==65), 28nat<count as nat,
    ensures wire_good(bytes(count),28nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1635nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1643nat,len:26nat,next:1673nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1635nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1643nat,len:26nat,next:1673nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1673nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1681nat,len:6nat,next:1691nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1673nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1681nat,len:6nat,next:1691nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1643,1669),true,false,8192,2048,control(28nat))==Ok((frame(),middle_control(28nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1681,1687),true,false,middle_control(28nat))==Ok(control(29nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1635nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1643nat,len:26nat,next:1673nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1635nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1643nat,len:26nat,next:1673nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1673nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1681nat,len:6nat,next:1691nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1673nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1681nat,len:6nat,next:1691nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1643,1669),true,false,8192,2048,control(28nat))==Ok((frame(),middle_control(28nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1681,1687),true,false,middle_control(28nat))==Ok(control(29nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_29(count: u32)
    requires (count==64 || count==65), 29nat<count as nat,
    ensures wire_good(bytes(count),29nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1691nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1699nat,len:26nat,next:1729nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1691nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1699nat,len:26nat,next:1729nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1729nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1737nat,len:6nat,next:1747nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1729nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1737nat,len:6nat,next:1747nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1699,1725),true,false,8192,2048,control(29nat))==Ok((frame(),middle_control(29nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1737,1743),true,false,middle_control(29nat))==Ok(control(30nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1691nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1699nat,len:26nat,next:1729nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1691nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1699nat,len:26nat,next:1729nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1729nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1737nat,len:6nat,next:1747nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1729nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1737nat,len:6nat,next:1747nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1699,1725),true,false,8192,2048,control(29nat))==Ok((frame(),middle_control(29nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1737,1743),true,false,middle_control(29nat))==Ok(control(30nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_30(count: u32)
    requires (count==64 || count==65), 30nat<count as nat,
    ensures wire_good(bytes(count),30nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1747nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1755nat,len:26nat,next:1785nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1747nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1755nat,len:26nat,next:1785nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1785nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1793nat,len:6nat,next:1803nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1785nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1793nat,len:6nat,next:1803nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1755,1781),true,false,8192,2048,control(30nat))==Ok((frame(),middle_control(30nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1793,1799),true,false,middle_control(30nat))==Ok(control(31nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1747nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1755nat,len:26nat,next:1785nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1747nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1755nat,len:26nat,next:1785nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1785nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1793nat,len:6nat,next:1803nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1785nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1793nat,len:6nat,next:1803nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1755,1781),true,false,8192,2048,control(30nat))==Ok((frame(),middle_control(30nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1793,1799),true,false,middle_control(30nat))==Ok(control(31nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_31(count: u32)
    requires (count==64 || count==65), 31nat<count as nat,
    ensures wire_good(bytes(count),31nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1803nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1811nat,len:26nat,next:1841nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1803nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1811nat,len:26nat,next:1841nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1841nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1849nat,len:6nat,next:1859nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1841nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1849nat,len:6nat,next:1859nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1811,1837),true,false,8192,2048,control(31nat))==Ok((frame(),middle_control(31nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1849,1855),true,false,middle_control(31nat))==Ok(control(32nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1803nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1811nat,len:26nat,next:1841nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1803nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1811nat,len:26nat,next:1841nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1841nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1849nat,len:6nat,next:1859nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1841nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1849nat,len:6nat,next:1859nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1811,1837),true,false,8192,2048,control(31nat))==Ok((frame(),middle_control(31nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1849,1855),true,false,middle_control(31nat))==Ok(control(32nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_32(count: u32)
    requires (count==64 || count==65), 32nat<count as nat,
    ensures wire_good(bytes(count),32nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1859nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1867nat,len:26nat,next:1897nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1859nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1867nat,len:26nat,next:1897nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1897nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1905nat,len:6nat,next:1915nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1897nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1905nat,len:6nat,next:1915nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1867,1893),true,false,8192,2048,control(32nat))==Ok((frame(),middle_control(32nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1905,1911),true,false,middle_control(32nat))==Ok(control(33nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1859nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1867nat,len:26nat,next:1897nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1859nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1867nat,len:26nat,next:1897nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1897nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1905nat,len:6nat,next:1915nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1897nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1905nat,len:6nat,next:1915nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1867,1893),true,false,8192,2048,control(32nat))==Ok((frame(),middle_control(32nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1905,1911),true,false,middle_control(32nat))==Ok(control(33nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_33(count: u32)
    requires (count==64 || count==65), 33nat<count as nat,
    ensures wire_good(bytes(count),33nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1915nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1923nat,len:26nat,next:1953nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1915nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1923nat,len:26nat,next:1953nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),1953nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1961nat,len:6nat,next:1971nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1953nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1961nat,len:6nat,next:1971nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1923,1949),true,false,8192,2048,control(33nat))==Ok((frame(),middle_control(33nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(1961,1967),true,false,middle_control(33nat))==Ok(control(34nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1915nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1923nat,len:26nat,next:1953nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1915nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1923nat,len:26nat,next:1953nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),1953nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1961nat,len:6nat,next:1971nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1953nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:1961nat,len:6nat,next:1971nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1923,1949),true,false,8192,2048,control(33nat))==Ok((frame(),middle_control(33nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(1961,1967),true,false,middle_control(33nat))==Ok(control(34nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_34(count: u32)
    requires (count==64 || count==65), 34nat<count as nat,
    ensures wire_good(bytes(count),34nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),1971nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1979nat,len:26nat,next:2009nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),1971nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1979nat,len:26nat,next:2009nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2009nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2017nat,len:6nat,next:2027nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2009nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2017nat,len:6nat,next:2027nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(1979,2005),true,false,8192,2048,control(34nat))==Ok((frame(),middle_control(34nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2017,2023),true,false,middle_control(34nat))==Ok(control(35nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),1971nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1979nat,len:26nat,next:2009nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),1971nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:1979nat,len:26nat,next:2009nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2009nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2017nat,len:6nat,next:2027nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2009nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2017nat,len:6nat,next:2027nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(1979,2005),true,false,8192,2048,control(34nat))==Ok((frame(),middle_control(34nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2017,2023),true,false,middle_control(34nat))==Ok(control(35nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_35(count: u32)
    requires (count==64 || count==65), 35nat<count as nat,
    ensures wire_good(bytes(count),35nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2027nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2035nat,len:26nat,next:2065nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2027nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2035nat,len:26nat,next:2065nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2065nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2073nat,len:6nat,next:2083nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2065nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2073nat,len:6nat,next:2083nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2035,2061),true,false,8192,2048,control(35nat))==Ok((frame(),middle_control(35nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2073,2079),true,false,middle_control(35nat))==Ok(control(36nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2027nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2035nat,len:26nat,next:2065nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2027nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2035nat,len:26nat,next:2065nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2065nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2073nat,len:6nat,next:2083nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2065nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2073nat,len:6nat,next:2083nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2035,2061),true,false,8192,2048,control(35nat))==Ok((frame(),middle_control(35nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2073,2079),true,false,middle_control(35nat))==Ok(control(36nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_36(count: u32)
    requires (count==64 || count==65), 36nat<count as nat,
    ensures wire_good(bytes(count),36nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2083nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2091nat,len:26nat,next:2121nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2083nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2091nat,len:26nat,next:2121nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2121nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2129nat,len:6nat,next:2139nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2121nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2129nat,len:6nat,next:2139nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2091,2117),true,false,8192,2048,control(36nat))==Ok((frame(),middle_control(36nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2129,2135),true,false,middle_control(36nat))==Ok(control(37nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2083nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2091nat,len:26nat,next:2121nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2083nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2091nat,len:26nat,next:2121nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2121nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2129nat,len:6nat,next:2139nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2121nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2129nat,len:6nat,next:2139nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2091,2117),true,false,8192,2048,control(36nat))==Ok((frame(),middle_control(36nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2129,2135),true,false,middle_control(36nat))==Ok(control(37nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_37(count: u32)
    requires (count==64 || count==65), 37nat<count as nat,
    ensures wire_good(bytes(count),37nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2139nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2147nat,len:26nat,next:2177nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2139nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2147nat,len:26nat,next:2177nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2177nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2185nat,len:6nat,next:2195nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2177nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2185nat,len:6nat,next:2195nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2147,2173),true,false,8192,2048,control(37nat))==Ok((frame(),middle_control(37nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2185,2191),true,false,middle_control(37nat))==Ok(control(38nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2139nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2147nat,len:26nat,next:2177nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2139nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2147nat,len:26nat,next:2177nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2177nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2185nat,len:6nat,next:2195nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2177nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2185nat,len:6nat,next:2195nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2147,2173),true,false,8192,2048,control(37nat))==Ok((frame(),middle_control(37nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2185,2191),true,false,middle_control(37nat))==Ok(control(38nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_38(count: u32)
    requires (count==64 || count==65), 38nat<count as nat,
    ensures wire_good(bytes(count),38nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2195nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2203nat,len:26nat,next:2233nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2195nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2203nat,len:26nat,next:2233nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2233nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2241nat,len:6nat,next:2251nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2233nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2241nat,len:6nat,next:2251nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2203,2229),true,false,8192,2048,control(38nat))==Ok((frame(),middle_control(38nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2241,2247),true,false,middle_control(38nat))==Ok(control(39nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2195nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2203nat,len:26nat,next:2233nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2195nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2203nat,len:26nat,next:2233nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2233nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2241nat,len:6nat,next:2251nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2233nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2241nat,len:6nat,next:2251nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2203,2229),true,false,8192,2048,control(38nat))==Ok((frame(),middle_control(38nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2241,2247),true,false,middle_control(38nat))==Ok(control(39nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_39(count: u32)
    requires (count==64 || count==65), 39nat<count as nat,
    ensures wire_good(bytes(count),39nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2251nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2259nat,len:26nat,next:2289nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2251nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2259nat,len:26nat,next:2289nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2289nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2297nat,len:6nat,next:2307nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2289nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2297nat,len:6nat,next:2307nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2259,2285),true,false,8192,2048,control(39nat))==Ok((frame(),middle_control(39nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2297,2303),true,false,middle_control(39nat))==Ok(control(40nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2251nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2259nat,len:26nat,next:2289nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2251nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2259nat,len:26nat,next:2289nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2289nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2297nat,len:6nat,next:2307nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2289nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2297nat,len:6nat,next:2307nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2259,2285),true,false,8192,2048,control(39nat))==Ok((frame(),middle_control(39nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2297,2303),true,false,middle_control(39nat))==Ok(control(40nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_40(count: u32)
    requires (count==64 || count==65), 40nat<count as nat,
    ensures wire_good(bytes(count),40nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2307nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2315nat,len:26nat,next:2345nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2307nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2315nat,len:26nat,next:2345nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2345nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2353nat,len:6nat,next:2363nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2345nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2353nat,len:6nat,next:2363nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2315,2341),true,false,8192,2048,control(40nat))==Ok((frame(),middle_control(40nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2353,2359),true,false,middle_control(40nat))==Ok(control(41nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2307nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2315nat,len:26nat,next:2345nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2307nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2315nat,len:26nat,next:2345nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2345nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2353nat,len:6nat,next:2363nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2345nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2353nat,len:6nat,next:2363nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2315,2341),true,false,8192,2048,control(40nat))==Ok((frame(),middle_control(40nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2353,2359),true,false,middle_control(40nat))==Ok(control(41nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_41(count: u32)
    requires (count==64 || count==65), 41nat<count as nat,
    ensures wire_good(bytes(count),41nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2363nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2371nat,len:26nat,next:2401nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2363nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2371nat,len:26nat,next:2401nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2401nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2409nat,len:6nat,next:2419nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2401nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2409nat,len:6nat,next:2419nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2371,2397),true,false,8192,2048,control(41nat))==Ok((frame(),middle_control(41nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2409,2415),true,false,middle_control(41nat))==Ok(control(42nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2363nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2371nat,len:26nat,next:2401nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2363nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2371nat,len:26nat,next:2401nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2401nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2409nat,len:6nat,next:2419nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2401nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2409nat,len:6nat,next:2419nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2371,2397),true,false,8192,2048,control(41nat))==Ok((frame(),middle_control(41nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2409,2415),true,false,middle_control(41nat))==Ok(control(42nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_42(count: u32)
    requires (count==64 || count==65), 42nat<count as nat,
    ensures wire_good(bytes(count),42nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2419nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2427nat,len:26nat,next:2457nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2419nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2427nat,len:26nat,next:2457nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2457nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2465nat,len:6nat,next:2475nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2457nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2465nat,len:6nat,next:2475nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2427,2453),true,false,8192,2048,control(42nat))==Ok((frame(),middle_control(42nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2465,2471),true,false,middle_control(42nat))==Ok(control(43nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2419nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2427nat,len:26nat,next:2457nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2419nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2427nat,len:26nat,next:2457nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2457nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2465nat,len:6nat,next:2475nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2457nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2465nat,len:6nat,next:2475nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2427,2453),true,false,8192,2048,control(42nat))==Ok((frame(),middle_control(42nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2465,2471),true,false,middle_control(42nat))==Ok(control(43nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_43(count: u32)
    requires (count==64 || count==65), 43nat<count as nat,
    ensures wire_good(bytes(count),43nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2475nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2483nat,len:26nat,next:2513nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2475nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2483nat,len:26nat,next:2513nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2513nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2521nat,len:6nat,next:2531nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2513nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2521nat,len:6nat,next:2531nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2483,2509),true,false,8192,2048,control(43nat))==Ok((frame(),middle_control(43nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2521,2527),true,false,middle_control(43nat))==Ok(control(44nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2475nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2483nat,len:26nat,next:2513nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2475nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2483nat,len:26nat,next:2513nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2513nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2521nat,len:6nat,next:2531nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2513nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2521nat,len:6nat,next:2531nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2483,2509),true,false,8192,2048,control(43nat))==Ok((frame(),middle_control(43nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2521,2527),true,false,middle_control(43nat))==Ok(control(44nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_44(count: u32)
    requires (count==64 || count==65), 44nat<count as nat,
    ensures wire_good(bytes(count),44nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2531nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2539nat,len:26nat,next:2569nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2531nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2539nat,len:26nat,next:2569nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2569nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2577nat,len:6nat,next:2587nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2569nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2577nat,len:6nat,next:2587nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2539,2565),true,false,8192,2048,control(44nat))==Ok((frame(),middle_control(44nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2577,2583),true,false,middle_control(44nat))==Ok(control(45nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2531nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2539nat,len:26nat,next:2569nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2531nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2539nat,len:26nat,next:2569nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2569nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2577nat,len:6nat,next:2587nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2569nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2577nat,len:6nat,next:2587nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2539,2565),true,false,8192,2048,control(44nat))==Ok((frame(),middle_control(44nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2577,2583),true,false,middle_control(44nat))==Ok(control(45nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_45(count: u32)
    requires (count==64 || count==65), 45nat<count as nat,
    ensures wire_good(bytes(count),45nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2587nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2595nat,len:26nat,next:2625nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2587nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2595nat,len:26nat,next:2625nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2625nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2633nat,len:6nat,next:2643nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2625nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2633nat,len:6nat,next:2643nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2595,2621),true,false,8192,2048,control(45nat))==Ok((frame(),middle_control(45nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2633,2639),true,false,middle_control(45nat))==Ok(control(46nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2587nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2595nat,len:26nat,next:2625nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2587nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2595nat,len:26nat,next:2625nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2625nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2633nat,len:6nat,next:2643nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2625nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2633nat,len:6nat,next:2643nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2595,2621),true,false,8192,2048,control(45nat))==Ok((frame(),middle_control(45nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2633,2639),true,false,middle_control(45nat))==Ok(control(46nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_46(count: u32)
    requires (count==64 || count==65), 46nat<count as nat,
    ensures wire_good(bytes(count),46nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2643nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2651nat,len:26nat,next:2681nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2643nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2651nat,len:26nat,next:2681nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2681nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2689nat,len:6nat,next:2699nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2681nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2689nat,len:6nat,next:2699nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2651,2677),true,false,8192,2048,control(46nat))==Ok((frame(),middle_control(46nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2689,2695),true,false,middle_control(46nat))==Ok(control(47nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2643nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2651nat,len:26nat,next:2681nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2643nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2651nat,len:26nat,next:2681nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2681nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2689nat,len:6nat,next:2699nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2681nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2689nat,len:6nat,next:2699nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2651,2677),true,false,8192,2048,control(46nat))==Ok((frame(),middle_control(46nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2689,2695),true,false,middle_control(46nat))==Ok(control(47nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_47(count: u32)
    requires (count==64 || count==65), 47nat<count as nat,
    ensures wire_good(bytes(count),47nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2699nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2707nat,len:26nat,next:2737nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2699nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2707nat,len:26nat,next:2737nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2737nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2745nat,len:6nat,next:2755nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2737nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2745nat,len:6nat,next:2755nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2707,2733),true,false,8192,2048,control(47nat))==Ok((frame(),middle_control(47nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2745,2751),true,false,middle_control(47nat))==Ok(control(48nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2699nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2707nat,len:26nat,next:2737nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2699nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2707nat,len:26nat,next:2737nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2737nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2745nat,len:6nat,next:2755nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2737nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2745nat,len:6nat,next:2755nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2707,2733),true,false,8192,2048,control(47nat))==Ok((frame(),middle_control(47nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2745,2751),true,false,middle_control(47nat))==Ok(control(48nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_48(count: u32)
    requires (count==64 || count==65), 48nat<count as nat,
    ensures wire_good(bytes(count),48nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2755nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2763nat,len:26nat,next:2793nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2755nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2763nat,len:26nat,next:2793nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2793nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2801nat,len:6nat,next:2811nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2793nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2801nat,len:6nat,next:2811nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2763,2789),true,false,8192,2048,control(48nat))==Ok((frame(),middle_control(48nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2801,2807),true,false,middle_control(48nat))==Ok(control(49nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2755nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2763nat,len:26nat,next:2793nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2755nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2763nat,len:26nat,next:2793nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2793nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2801nat,len:6nat,next:2811nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2793nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2801nat,len:6nat,next:2811nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2763,2789),true,false,8192,2048,control(48nat))==Ok((frame(),middle_control(48nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2801,2807),true,false,middle_control(48nat))==Ok(control(49nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_49(count: u32)
    requires (count==64 || count==65), 49nat<count as nat,
    ensures wire_good(bytes(count),49nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2811nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2819nat,len:26nat,next:2849nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2811nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2819nat,len:26nat,next:2849nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2849nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2857nat,len:6nat,next:2867nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2849nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2857nat,len:6nat,next:2867nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2819,2845),true,false,8192,2048,control(49nat))==Ok((frame(),middle_control(49nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2857,2863),true,false,middle_control(49nat))==Ok(control(50nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2811nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2819nat,len:26nat,next:2849nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2811nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2819nat,len:26nat,next:2849nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2849nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2857nat,len:6nat,next:2867nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2849nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2857nat,len:6nat,next:2867nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2819,2845),true,false,8192,2048,control(49nat))==Ok((frame(),middle_control(49nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2857,2863),true,false,middle_control(49nat))==Ok(control(50nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_50(count: u32)
    requires (count==64 || count==65), 50nat<count as nat,
    ensures wire_good(bytes(count),50nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2867nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2875nat,len:26nat,next:2905nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2867nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2875nat,len:26nat,next:2905nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2905nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2913nat,len:6nat,next:2923nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2905nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2913nat,len:6nat,next:2923nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2875,2901),true,false,8192,2048,control(50nat))==Ok((frame(),middle_control(50nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2913,2919),true,false,middle_control(50nat))==Ok(control(51nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2867nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2875nat,len:26nat,next:2905nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2867nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2875nat,len:26nat,next:2905nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2905nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2913nat,len:6nat,next:2923nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2905nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2913nat,len:6nat,next:2923nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2875,2901),true,false,8192,2048,control(50nat))==Ok((frame(),middle_control(50nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2913,2919),true,false,middle_control(50nat))==Ok(control(51nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_51(count: u32)
    requires (count==64 || count==65), 51nat<count as nat,
    ensures wire_good(bytes(count),51nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2923nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2931nat,len:26nat,next:2961nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2923nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2931nat,len:26nat,next:2961nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),2961nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2969nat,len:6nat,next:2979nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2961nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2969nat,len:6nat,next:2979nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2931,2957),true,false,8192,2048,control(51nat))==Ok((frame(),middle_control(51nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(2969,2975),true,false,middle_control(51nat))==Ok(control(52nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2923nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2931nat,len:26nat,next:2961nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2923nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2931nat,len:26nat,next:2961nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),2961nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2969nat,len:6nat,next:2979nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2961nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:2969nat,len:6nat,next:2979nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2931,2957),true,false,8192,2048,control(51nat))==Ok((frame(),middle_control(51nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(2969,2975),true,false,middle_control(51nat))==Ok(control(52nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_52(count: u32)
    requires (count==64 || count==65), 52nat<count as nat,
    ensures wire_good(bytes(count),52nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),2979nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2987nat,len:26nat,next:3017nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),2979nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2987nat,len:26nat,next:3017nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3017nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3025nat,len:6nat,next:3035nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3017nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3025nat,len:6nat,next:3035nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(2987,3013),true,false,8192,2048,control(52nat))==Ok((frame(),middle_control(52nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3025,3031),true,false,middle_control(52nat))==Ok(control(53nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),2979nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2987nat,len:26nat,next:3017nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),2979nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:2987nat,len:26nat,next:3017nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3017nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3025nat,len:6nat,next:3035nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3017nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3025nat,len:6nat,next:3035nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(2987,3013),true,false,8192,2048,control(52nat))==Ok((frame(),middle_control(52nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3025,3031),true,false,middle_control(52nat))==Ok(control(53nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_53(count: u32)
    requires (count==64 || count==65), 53nat<count as nat,
    ensures wire_good(bytes(count),53nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3035nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3043nat,len:26nat,next:3073nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3035nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3043nat,len:26nat,next:3073nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3073nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3081nat,len:6nat,next:3091nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3073nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3081nat,len:6nat,next:3091nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3043,3069),true,false,8192,2048,control(53nat))==Ok((frame(),middle_control(53nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3081,3087),true,false,middle_control(53nat))==Ok(control(54nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3035nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3043nat,len:26nat,next:3073nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3035nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3043nat,len:26nat,next:3073nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3073nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3081nat,len:6nat,next:3091nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3073nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3081nat,len:6nat,next:3091nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3043,3069),true,false,8192,2048,control(53nat))==Ok((frame(),middle_control(53nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3081,3087),true,false,middle_control(53nat))==Ok(control(54nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_54(count: u32)
    requires (count==64 || count==65), 54nat<count as nat,
    ensures wire_good(bytes(count),54nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3091nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3099nat,len:26nat,next:3129nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3091nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3099nat,len:26nat,next:3129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3129nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3137nat,len:6nat,next:3147nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3129nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3137nat,len:6nat,next:3147nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3099,3125),true,false,8192,2048,control(54nat))==Ok((frame(),middle_control(54nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3137,3143),true,false,middle_control(54nat))==Ok(control(55nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3091nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3099nat,len:26nat,next:3129nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3091nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3099nat,len:26nat,next:3129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3129nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3137nat,len:6nat,next:3147nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3129nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3137nat,len:6nat,next:3147nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3099,3125),true,false,8192,2048,control(54nat))==Ok((frame(),middle_control(54nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3137,3143),true,false,middle_control(54nat))==Ok(control(55nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_55(count: u32)
    requires (count==64 || count==65), 55nat<count as nat,
    ensures wire_good(bytes(count),55nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3147nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3155nat,len:26nat,next:3185nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3147nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3155nat,len:26nat,next:3185nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3185nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3193nat,len:6nat,next:3203nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3185nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3193nat,len:6nat,next:3203nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3155,3181),true,false,8192,2048,control(55nat))==Ok((frame(),middle_control(55nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3193,3199),true,false,middle_control(55nat))==Ok(control(56nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3147nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3155nat,len:26nat,next:3185nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3147nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3155nat,len:26nat,next:3185nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3185nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3193nat,len:6nat,next:3203nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3185nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3193nat,len:6nat,next:3203nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3155,3181),true,false,8192,2048,control(55nat))==Ok((frame(),middle_control(55nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3193,3199),true,false,middle_control(55nat))==Ok(control(56nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_56(count: u32)
    requires (count==64 || count==65), 56nat<count as nat,
    ensures wire_good(bytes(count),56nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3203nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3211nat,len:26nat,next:3241nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3203nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3211nat,len:26nat,next:3241nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3241nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3249nat,len:6nat,next:3259nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3241nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3249nat,len:6nat,next:3259nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3211,3237),true,false,8192,2048,control(56nat))==Ok((frame(),middle_control(56nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3249,3255),true,false,middle_control(56nat))==Ok(control(57nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3203nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3211nat,len:26nat,next:3241nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3203nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3211nat,len:26nat,next:3241nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3241nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3249nat,len:6nat,next:3259nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3241nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3249nat,len:6nat,next:3259nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3211,3237),true,false,8192,2048,control(56nat))==Ok((frame(),middle_control(56nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3249,3255),true,false,middle_control(56nat))==Ok(control(57nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_57(count: u32)
    requires (count==64 || count==65), 57nat<count as nat,
    ensures wire_good(bytes(count),57nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3259nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3267nat,len:26nat,next:3297nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3259nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3267nat,len:26nat,next:3297nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3297nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3305nat,len:6nat,next:3315nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3297nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3305nat,len:6nat,next:3315nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3267,3293),true,false,8192,2048,control(57nat))==Ok((frame(),middle_control(57nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3305,3311),true,false,middle_control(57nat))==Ok(control(58nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3259nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3267nat,len:26nat,next:3297nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3259nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3267nat,len:26nat,next:3297nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3297nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3305nat,len:6nat,next:3315nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3297nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3305nat,len:6nat,next:3315nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3267,3293),true,false,8192,2048,control(57nat))==Ok((frame(),middle_control(57nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3305,3311),true,false,middle_control(57nat))==Ok(control(58nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_58(count: u32)
    requires (count==64 || count==65), 58nat<count as nat,
    ensures wire_good(bytes(count),58nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3315nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3323nat,len:26nat,next:3353nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3315nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3323nat,len:26nat,next:3353nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3353nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3361nat,len:6nat,next:3371nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3353nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3361nat,len:6nat,next:3371nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3323,3349),true,false,8192,2048,control(58nat))==Ok((frame(),middle_control(58nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3361,3367),true,false,middle_control(58nat))==Ok(control(59nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3315nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3323nat,len:26nat,next:3353nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3315nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3323nat,len:26nat,next:3353nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3353nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3361nat,len:6nat,next:3371nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3353nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3361nat,len:6nat,next:3371nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3323,3349),true,false,8192,2048,control(58nat))==Ok((frame(),middle_control(58nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3361,3367),true,false,middle_control(58nat))==Ok(control(59nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_59(count: u32)
    requires (count==64 || count==65), 59nat<count as nat,
    ensures wire_good(bytes(count),59nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3371nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3379nat,len:26nat,next:3409nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3371nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3379nat,len:26nat,next:3409nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3409nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3417nat,len:6nat,next:3427nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3409nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3417nat,len:6nat,next:3427nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3379,3405),true,false,8192,2048,control(59nat))==Ok((frame(),middle_control(59nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3417,3423),true,false,middle_control(59nat))==Ok(control(60nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3371nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3379nat,len:26nat,next:3409nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3371nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3379nat,len:26nat,next:3409nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3409nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3417nat,len:6nat,next:3427nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3409nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3417nat,len:6nat,next:3427nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3379,3405),true,false,8192,2048,control(59nat))==Ok((frame(),middle_control(59nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3417,3423),true,false,middle_control(59nat))==Ok(control(60nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_60(count: u32)
    requires (count==64 || count==65), 60nat<count as nat,
    ensures wire_good(bytes(count),60nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3427nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3435nat,len:26nat,next:3465nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3427nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3435nat,len:26nat,next:3465nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3465nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3473nat,len:6nat,next:3483nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3465nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3473nat,len:6nat,next:3483nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3435,3461),true,false,8192,2048,control(60nat))==Ok((frame(),middle_control(60nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3473,3479),true,false,middle_control(60nat))==Ok(control(61nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3427nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3435nat,len:26nat,next:3465nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3427nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3435nat,len:26nat,next:3465nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3465nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3473nat,len:6nat,next:3483nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3465nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3473nat,len:6nat,next:3483nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3435,3461),true,false,8192,2048,control(60nat))==Ok((frame(),middle_control(60nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3473,3479),true,false,middle_control(60nat))==Ok(control(61nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_61(count: u32)
    requires (count==64 || count==65), 61nat<count as nat,
    ensures wire_good(bytes(count),61nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3483nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3491nat,len:26nat,next:3521nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3483nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3491nat,len:26nat,next:3521nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3521nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3529nat,len:6nat,next:3539nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3521nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3529nat,len:6nat,next:3539nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3491,3517),true,false,8192,2048,control(61nat))==Ok((frame(),middle_control(61nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3529,3535),true,false,middle_control(61nat))==Ok(control(62nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3483nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3491nat,len:26nat,next:3521nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3483nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3491nat,len:26nat,next:3521nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3521nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3529nat,len:6nat,next:3539nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3521nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3529nat,len:6nat,next:3539nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3491,3517),true,false,8192,2048,control(61nat))==Ok((frame(),middle_control(61nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3529,3535),true,false,middle_control(61nat))==Ok(control(62nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_62(count: u32)
    requires (count==64 || count==65), 62nat<count as nat,
    ensures wire_good(bytes(count),62nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3539nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3547nat,len:26nat,next:3577nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3539nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3547nat,len:26nat,next:3577nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3577nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3585nat,len:6nat,next:3595nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3577nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3585nat,len:6nat,next:3595nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3547,3573),true,false,8192,2048,control(62nat))==Ok((frame(),middle_control(62nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3585,3591),true,false,middle_control(62nat))==Ok(control(63nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3539nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3547nat,len:26nat,next:3577nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3539nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3547nat,len:26nat,next:3577nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3577nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3585nat,len:6nat,next:3595nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3577nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3585nat,len:6nat,next:3595nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3547,3573),true,false,8192,2048,control(62nat))==Ok((frame(),middle_control(62nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3585,3591),true,false,middle_control(62nat))==Ok(control(63nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_63(count: u32)
    requires (count==64 || count==65), 63nat<count as nat,
    ensures wire_good(bytes(count),63nat),
{
    reveal(wire_good);
    if count==64u32 {
        assert forall|phase: w::Phase| c::read(bytes(64u32),3595nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3603nat,len:26nat,next:3633nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3595nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3603nat,len:26nat,next:3633nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(64u32),3633nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3641nat,len:6nat,next:3651nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(64u32),3633nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3641nat,len:6nat,next:3651nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(64u32).subrange(3603,3629),true,false,8192,2048,control(63nat))==Ok((frame(),middle_control(63nat)))) by (compute);
        assert(a::fdat(bytes(64u32).subrange(3641,3647),true,false,middle_control(63nat))==Ok(control(64nat))) by (compute);
    }
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3595nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3603nat,len:26nat,next:3633nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3595nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3603nat,len:26nat,next:3633nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3633nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3641nat,len:6nat,next:3651nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3633nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3641nat,len:6nat,next:3651nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3603,3629),true,false,8192,2048,control(63nat))==Ok((frame(),middle_control(63nat)))) by (compute);
        assert(a::fdat(bytes(65u32).subrange(3641,3647),true,false,middle_control(63nat))==Ok(control(64nat))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn wire_64(count: u32)
    requires (count==64 || count==65), 64nat<count as nat,
    ensures wire_good(bytes(count),64nat),
{
    reveal(wire_good);
    if count==65u32 {
        assert forall|phase: w::Phase| c::read(bytes(65u32),3651nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3659nat,len:26nat,next:3689nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3651nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:3659nat,len:26nat,next:3689nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert forall|phase: w::Phase| c::read(bytes(65u32),3689nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3697nat,len:6nat,next:3707nat,crc_ok:true,synthetic:false}) by {
            assert(c::read(bytes(65u32),3689nat,phase,false)==Ok(c::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:3697nat,len:6nat,next:3707nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert(a::fctl(bytes(65u32).subrange(3659,3685),true,false,8192,2048,control(64nat))==Err((Error::PixelBudgetExceeded,false))) by (compute);
    }
}
#[verifier::spinoff_prover]
proof fn all_wire(count: u32)
    requires count==64 || count==65,
    ensures forall|k: nat| k<count as nat ==> #[trigger] wire_good(bytes(count),k),
{
    // wire_good is already opaque; no explicit hide fuel is needed.
    assert forall|k: nat| k<count as nat implies #[trigger] wire_good(bytes(count),k) by {
        if k==0 {wire_0(count);}
        else if k==1 {wire_1(count);}
        else if k==2 {wire_2(count);}
        else if k==3 {wire_3(count);}
        else if k==4 {wire_4(count);}
        else if k==5 {wire_5(count);}
        else if k==6 {wire_6(count);}
        else if k==7 {wire_7(count);}
        else if k==8 {wire_8(count);}
        else if k==9 {wire_9(count);}
        else if k==10 {wire_10(count);}
        else if k==11 {wire_11(count);}
        else if k==12 {wire_12(count);}
        else if k==13 {wire_13(count);}
        else if k==14 {wire_14(count);}
        else if k==15 {wire_15(count);}
        else if k==16 {wire_16(count);}
        else if k==17 {wire_17(count);}
        else if k==18 {wire_18(count);}
        else if k==19 {wire_19(count);}
        else if k==20 {wire_20(count);}
        else if k==21 {wire_21(count);}
        else if k==22 {wire_22(count);}
        else if k==23 {wire_23(count);}
        else if k==24 {wire_24(count);}
        else if k==25 {wire_25(count);}
        else if k==26 {wire_26(count);}
        else if k==27 {wire_27(count);}
        else if k==28 {wire_28(count);}
        else if k==29 {wire_29(count);}
        else if k==30 {wire_30(count);}
        else if k==31 {wire_31(count);}
        else if k==32 {wire_32(count);}
        else if k==33 {wire_33(count);}
        else if k==34 {wire_34(count);}
        else if k==35 {wire_35(count);}
        else if k==36 {wire_36(count);}
        else if k==37 {wire_37(count);}
        else if k==38 {wire_38(count);}
        else if k==39 {wire_39(count);}
        else if k==40 {wire_40(count);}
        else if k==41 {wire_41(count);}
        else if k==42 {wire_42(count);}
        else if k==43 {wire_43(count);}
        else if k==44 {wire_44(count);}
        else if k==45 {wire_45(count);}
        else if k==46 {wire_46(count);}
        else if k==47 {wire_47(count);}
        else if k==48 {wire_48(count);}
        else if k==49 {wire_49(count);}
        else if k==50 {wire_50(count);}
        else if k==51 {wire_51(count);}
        else if k==52 {wire_52(count);}
        else if k==53 {wire_53(count);}
        else if k==54 {wire_54(count);}
        else if k==55 {wire_55(count);}
        else if k==56 {wire_56(count);}
        else if k==57 {wire_57(count);}
        else if k==58 {wire_58(count);}
        else if k==59 {wire_59(count);}
        else if k==60 {wire_60(count);}
        else if k==61 {wire_61(count);}
        else if k==62 {wire_62(count);}
        else if k==63 {wire_63(count);}
        else if k==64 {wire_64(count);}
    }
}
#[verifier::spinoff_prover]
proof fn bookends(count: u32)
    requires count==64 || count==65,
    ensures start_good(bytes(count),count),end_good(bytes(count),count),
{
    reveal(start_good);reveal(end_good);
    if count==64 {
        assert(start_good(bytes(64u32),64u32)) by (compute);
        assert(end_good(bytes(64u32),64u32)) by (compute);
    }
    if count==65 {
        assert(start_good(bytes(65u32),65u32)) by (compute);
        wire_64(65u32);
    }
}

#[verifier::spinoff_prover]
proof fn full_model(count: u32)
    requires count==64 || count==65,
    ensures budget_outcome(m::MWF_p(bytes(count),m::strict()),count),
{
    hide(bytes);hide(m::MWF_p);
    byte_length(count);all_wire(count);bookends(count);start(bytes(count),count);
}
#[verifier::spinoff_prover]
proof fn byte_length(count: u32)
    ensures bytes(count).len()==79+56*(count as nat),
{}
#[verifier::spinoff_prover]
proof fn short_lengths(count: u32)
    requires count==64 || count==65,
    ensures prefix_bytes(count).len()==67,ending().len()==12,
{}
#[verifier::spinoff_prover]
proof fn pair_length(k: nat)
    ensures pair(k).len()==56,
{}
#[verifier::spinoff_prover]
proof fn byte_slices(count: u32,k: nat)
    requires count==64 || count==65,k<count as nat,
    ensures bytes(count).len()==79+56*(count as nat),
        bytes(count).subrange(0,67)==prefix_bytes(count),
        bytes(count).subrange((67+56*k) as int,(123+56*k) as int)==pair(k),
        bytes(count).subrange((67+56*(count as nat)) as int,bytes(count).len() as int)==ending(),
{
    hide(pair);hide(prefix_bytes);hide(ending);
    pair_length(k);short_lengths(count);
    assert(prefix_bytes(count).len()==67);
    assert(ending().len()==12);
    assert(bytes(count).subrange(0,67) =~= prefix_bytes(count));
    assert(bytes(count).subrange((67+56*k) as int,(123+56*k) as int) =~= pair(k)) by {
        assert forall|j: int| 0<=j<56 implies bytes(count)[67+56*(k as int)+j]==pair(k)[j] by {
            assert((56*(k as int)+j)/56==k as int);
            assert((56*(k as int)+j)%56==j);
        }
    }
    assert(bytes(count).subrange((67+56*(count as nat)) as int,bytes(count).len() as int) =~= ending());
}
#[verifier::spinoff_prover]
proof fn join(s: Seq<u8>,lo: int,hi: int)
    requires 0<=lo<=hi<=s.len(),
    ensures s.subrange(0,lo)+s.subrange(lo,hi)==s.subrange(0,hi),
{
    assert(s.subrange(0,lo)+s.subrange(lo,hi) =~= s.subrange(0,hi));
}
fn append(out: &mut [u8],pos: usize,chunk: &[u8])
    requires pos<=old(out).len(),chunk.len()<=old(out).len()-pos,
    ensures final(out).len()==old(out).len(),
        final(out)@.subrange(0,(pos+chunk.len()) as int)==old(out)@.subrange(0,pos as int)+chunk@,
{
    let ghost before=out@;let n=out.len();let mut j=0usize;
    while j<chunk.len()
        invariant out.len()==n,n==before.len(),before==old(out)@,pos<=n,chunk.len()<=n-pos,j<=chunk.len(),
            forall|i: int| 0<=i<n ==> out@[i]==(if pos as int<=i<pos as int+j as int {chunk@[(i-pos as int) as int]}else{before[i]}),
        decreases chunk.len()-j,
    {
        out[pos+j]=chunk[j];j+=1;
    }
    assert(out@.subrange(0,(pos+chunk.len()) as int) =~= before.subrange(0,pos as int)+chunk@);
}
fn literal_pair(k: usize) -> (out: [u8;56])
    requires k<65,
    ensures out@==pair(k as nat),
{
    match k {
        0=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,137u8,164u8,36u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,1u8,157u8,114u8,106u8,97u8],
        1=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,31u8,119u8,205u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,1u8,158u8,246u8,190u8,15u8],
        2=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,4u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,213u8,5u8,183u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,120u8,1u8,154u8,123u8,194u8,189u8],
        3=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,6u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,67u8,214u8,94u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,7u8,120u8,1u8,153u8,255u8,22u8,211u8],
        4=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,8u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,48u8,231u8,2u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,9u8,120u8,1u8,147u8,97u8,59u8,217u8],
        5=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,10u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,166u8,52u8,235u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,11u8,120u8,1u8,144u8,229u8,239u8,183u8],
        6=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,12u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,93u8,108u8,70u8,145u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,13u8,120u8,1u8,148u8,104u8,147u8,5u8],
        7=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,14u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,176u8,250u8,149u8,120u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,15u8,120u8,1u8,151u8,236u8,71u8,107u8],
        8=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,16u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,251u8,34u8,104u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,17u8,120u8,1u8,129u8,84u8,201u8,17u8],
        9=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,18u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,109u8,241u8,129u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,19u8,120u8,1u8,130u8,208u8,29u8,127u8],
        10=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,20u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,167u8,131u8,251u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,21u8,120u8,1u8,134u8,93u8,97u8,205u8],
        11=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,22u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,49u8,80u8,18u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,23u8,120u8,1u8,133u8,217u8,181u8,163u8],
        12=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,24u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,66u8,97u8,78u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,25u8,120u8,1u8,143u8,71u8,152u8,169u8],
        13=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,26u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,212u8,178u8,167u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,27u8,120u8,1u8,140u8,195u8,76u8,199u8],
        14=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,28u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,92u8,30u8,192u8,221u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,29u8,120u8,1u8,136u8,78u8,48u8,117u8],
        15=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,30u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,177u8,136u8,19u8,52u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,31u8,120u8,1u8,139u8,202u8,228u8,27u8],
        16=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,32u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,108u8,168u8,188u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,33u8,120u8,1u8,165u8,63u8,44u8,129u8],
        17=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,34u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,250u8,123u8,85u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,35u8,120u8,1u8,166u8,187u8,248u8,239u8],
        18=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,36u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,48u8,9u8,47u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,37u8,120u8,1u8,162u8,54u8,132u8,93u8],
        19=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,38u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,166u8,218u8,198u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,39u8,120u8,1u8,161u8,178u8,80u8,51u8],
        20=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,40u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,213u8,235u8,154u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,41u8,120u8,1u8,171u8,44u8,125u8,57u8],
        21=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,42u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,67u8,56u8,115u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,43u8,120u8,1u8,168u8,168u8,169u8,87u8],
        22=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,44u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,95u8,137u8,74u8,9u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,45u8,120u8,1u8,172u8,37u8,213u8,229u8],
        23=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,46u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,178u8,31u8,153u8,224u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,47u8,120u8,1u8,175u8,161u8,1u8,139u8],
        24=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,48u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,30u8,46u8,240u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,49u8,120u8,1u8,185u8,25u8,143u8,241u8],
        25=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,50u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,136u8,253u8,25u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,51u8,120u8,1u8,186u8,157u8,91u8,159u8],
        26=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,52u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,66u8,143u8,99u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,53u8,120u8,1u8,190u8,16u8,39u8,45u8],
        27=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,54u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,212u8,92u8,138u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,55u8,120u8,1u8,189u8,148u8,243u8,67u8],
        28=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,56u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,167u8,109u8,214u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,57u8,120u8,1u8,183u8,10u8,222u8,73u8],
        29=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,58u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,49u8,190u8,63u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,59u8,120u8,1u8,180u8,142u8,10u8,39u8],
        30=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,60u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,94u8,251u8,204u8,69u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,61u8,120u8,1u8,176u8,3u8,118u8,149u8],
        31=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,62u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,179u8,109u8,31u8,172u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,63u8,120u8,1u8,179u8,135u8,162u8,251u8],
        32=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,64u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,67u8,189u8,20u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,65u8,120u8,1u8,237u8,232u8,231u8,161u8],
        33=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,66u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,213u8,110u8,253u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,67u8,120u8,1u8,238u8,108u8,51u8,207u8],
        34=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,68u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,31u8,28u8,135u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,69u8,120u8,1u8,234u8,225u8,79u8,125u8],
        35=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,70u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,137u8,207u8,110u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,71u8,120u8,1u8,233u8,101u8,155u8,19u8],
        36=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,72u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,250u8,254u8,50u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,73u8,120u8,1u8,227u8,251u8,182u8,25u8],
        37=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,74u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,108u8,45u8,219u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,75u8,120u8,1u8,224u8,127u8,98u8,119u8],
        38=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,76u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,88u8,166u8,95u8,161u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,77u8,120u8,1u8,228u8,242u8,30u8,197u8],
        39=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,78u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,181u8,48u8,140u8,72u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,79u8,120u8,1u8,231u8,118u8,202u8,171u8],
        40=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,80u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,49u8,59u8,88u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,81u8,120u8,1u8,241u8,206u8,68u8,209u8],
        41=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,167u8,232u8,177u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,83u8,120u8,1u8,242u8,74u8,144u8,191u8],
        42=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,84u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,109u8,154u8,203u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,85u8,120u8,1u8,246u8,199u8,236u8,13u8],
        43=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,86u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,251u8,73u8,34u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,87u8,120u8,1u8,245u8,67u8,56u8,99u8],
        44=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,88u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,136u8,120u8,126u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,89u8,120u8,1u8,255u8,221u8,21u8,105u8],
        45=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,90u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,30u8,171u8,151u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,91u8,120u8,1u8,252u8,89u8,193u8,7u8],
        46=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,92u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,89u8,212u8,217u8,237u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,93u8,120u8,1u8,248u8,212u8,189u8,181u8],
        47=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,94u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,180u8,66u8,10u8,4u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,95u8,120u8,1u8,251u8,80u8,105u8,219u8],
        48=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,96u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,166u8,177u8,140u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,97u8,120u8,1u8,213u8,165u8,161u8,65u8],
        49=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,98u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,48u8,98u8,101u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,99u8,120u8,1u8,214u8,33u8,117u8,47u8],
        50=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,100u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,250u8,16u8,31u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,101u8,120u8,1u8,210u8,172u8,9u8,157u8],
        51=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,102u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,108u8,195u8,246u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,103u8,120u8,1u8,209u8,40u8,221u8,243u8],
        52=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,104u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,31u8,242u8,170u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,105u8,120u8,1u8,219u8,182u8,240u8,249u8],
        53=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,106u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,137u8,33u8,67u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,107u8,120u8,1u8,216u8,50u8,36u8,151u8],
        54=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,108u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,67u8,83u8,57u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,109u8,120u8,1u8,220u8,191u8,88u8,37u8],
        55=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,110u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,183u8,213u8,128u8,208u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,111u8,120u8,1u8,223u8,59u8,140u8,75u8],
        56=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,112u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,212u8,55u8,192u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,113u8,120u8,1u8,201u8,131u8,2u8,49u8],
        57=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,114u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,66u8,228u8,41u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,115u8,120u8,1u8,202u8,7u8,214u8,95u8],
        58=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,116u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,136u8,150u8,83u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,117u8,120u8,1u8,206u8,138u8,170u8,237u8],
        59=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,118u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,30u8,69u8,186u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,119u8,120u8,1u8,205u8,14u8,126u8,131u8],
        60=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,120u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,109u8,116u8,230u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,121u8,120u8,1u8,199u8,144u8,83u8,137u8],
        61=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,122u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,251u8,167u8,15u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,123u8,120u8,1u8,196u8,20u8,135u8,231u8],
        62=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,124u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,91u8,49u8,213u8,117u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,125u8,120u8,1u8,192u8,153u8,251u8,85u8],
        63=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,126u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,182u8,167u8,6u8,156u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,127u8,120u8,1u8,195u8,29u8,47u8,59u8],
        _=>[0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,128u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,86u8,29u8,150u8,68u8,0u8,0u8,0u8,6u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,129u8,120u8,1u8,124u8,71u8,113u8,225u8],
    }
}
fn literal_prefix(count: u32) -> (out: [u8;67])
    requires count==64 || count==65,
    ensures out@==prefix_bytes(count),
{
    if count==64 {[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,8u8,6u8,0u8,0u8,0u8,142u8,248u8,189u8,162u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,64u8,0u8,0u8,0u8,0u8,209u8,190u8,152u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8]} else {[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,8u8,0u8,8u8,6u8,0u8,0u8,0u8,142u8,248u8,189u8,162u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,65u8,0u8,0u8,0u8,0u8,236u8,222u8,177u8,233u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8]}
}

fn build(out: &mut [u8],count: u32)
    requires count==64 || count==65,old(out).len()==79+56*(count as nat),
    ensures final(out).len()==old(out).len(),final(out)@==bytes(count),
{
    hide(bytes);hide(pair);hide(prefix_bytes);hide(ending);
    let n=out.len();let p=literal_prefix(count);
    proof {byte_slices(count,0);}
    append(out,0,&p);
    let mut k=0usize;let mut pos=67usize;
    while k<(count as usize)
        invariant count==64 || count==65,out.len()==n,n==79+56*(count as nat),
            k<=count as usize,pos==67+56*k,pos<=n,
            bytes(count).len()==n,
            out@.subrange(0,pos as int)==bytes(count).subrange(0,pos as int),
        decreases count as usize-k,
    {
        proof {byte_slices(count,k as nat);}
        let p=literal_pair(k);
        append(out,pos,&p);
        proof {join(bytes(count),pos as int,(pos+56) as int);}
        pos+=56;k+=1;
    }
    let end=[0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {byte_slices(count,0);}
    append(out,pos,&end);
    proof {join(bytes(count),pos as int,n as int);assert(out@ =~= bytes(count));}
}

fn budget_overflow_witness() {
    hide(bytes);hide(m::MWF_p);
    let mut b=[0u8;3719];build(&mut b,65u32);
    proof {full_model(65u32);}
    let result=crate::png::Frames::try_new(&b);
    assert(matches!(result,Err(TryError::Format(Error::PixelBudgetExceeded))));
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.
fn budget_exact_projection_witness() {
    hide(bytes);hide(m::MWF_p);
    let mut b=[0u8;3663];build(&mut b,64u32);
    proof {full_model(64u32);}
    let mut ev=crate::png::Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::frames(&b,&crate::policy::STRICT,&mut ev);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==header());assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}},
        Err(_)=>{assert(false);},
    }
    let _public_result=crate::png::Frames::try_new(&b);
}
} // verus!
