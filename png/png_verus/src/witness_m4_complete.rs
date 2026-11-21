//! Authored frozen-fixture complete M4 error witnesses.
//! Model-versus-expected plus the actual public still call; OOM remains explicit.
use vstd::prelude::*;
use crate::{raster_model as r,filter_model as f,png::Error,policy::STRICT,m4_fixed_witness_model as fixed};
use inflate_verus::model as zm;
verus! {
// precedence/index-row0-before-filter-row1; frozen fixture file sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_precedence_index_row0_before_filter_row1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,12u8,80u8,76u8,84u8,69u8,126u8,240u8,116u8,184u8,57u8,97u8,203u8,4u8,217u8,68u8,9u8,86u8,59u8,166u8,158u8,64u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,224u8,100u8,103u8,96u8,0u8,0u8,0u8,63u8,0u8,17u8,245u8,163u8,6u8,133u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_precedence_index_row0_before_filter_row1() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,224u8,100u8,103u8,96u8,0u8,0u8,0u8,63u8,0u8,17u8]}
spec fn h_precedence_index_row0_before_filter_row1() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8}}
spec fn a_precedence_index_row0_before_filter_row1() -> r::Raster {r::shape(h_precedence_index_row0_before_filter_row1(),2nat,2nat,4nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_precedence_index_row0_before_filter_row1(b: Seq<u8>)
    requires b.len()==95,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:14nat,next:83nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})),
        b.subrange(65,79)==z_precedence_index_row0_before_filter_row1(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8},z_precedence_index_row0_before_filter_row1(),4nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,4);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn inflate_precedence_index_row0_before_filter_row1()
    ensures r::inflated(z_precedence_index_row0_before_filter_row1(),a_precedence_index_row0_before_filter_row1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:seq![0u8,0u8,9u8,7u8,0u8,0u8],r2:false}),
{
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MBlk);hide(zm::MOneBlock);
    hide(zm::MCompressed);hide(zm::MCompressedStep);hide(zm::MDec);hide(zm::decode_prefix);hide(zm::cw);
    assert(r::raw(a_precedence_index_row0_before_filter_row1())==6nat) by (compute);
    assert(zm::field_value(z_precedence_index_row0_before_filter_row1(),16,1)==1nat && zm::field_value(z_precedence_index_row0_before_filter_row1(),17,2)==1nat) by (compute);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),19nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),19nat,0int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),27nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),27nat,0int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),35nat,8nat)==zm::msb_bits(57nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),35nat,9int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),43nat,8nat)==zm::msb_bits(55nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),43nat,7int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),51nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),51nat,0int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),59nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),59nat,0int);
    assert(fixed::wire_word(z_precedence_index_row0_before_filter_row1(),67nat,7nat)==zm::msb_bits(0nat,7nat)) by (compute);
    fixed::fixed_decode(z_precedence_index_row0_before_filter_row1(),67nat,256int);
    fixed::end(z_precedence_index_row0_before_filter_row1(),67nat,seq![0u8,0u8,9u8,7u8,0u8,0u8],6nat,74nat);
    assert(seq![0u8,0u8,9u8,7u8,0u8].push(0u8) =~= seq![0u8,0u8,9u8,7u8,0u8,0u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),59nat,seq![0u8,0u8,9u8,7u8,0u8],0u8,6nat,67nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(seq![0u8,0u8,9u8,7u8].push(0u8) =~= seq![0u8,0u8,9u8,7u8,0u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),51nat,seq![0u8,0u8,9u8,7u8],0u8,6nat,59nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(seq![0u8,0u8,9u8].push(7u8) =~= seq![0u8,0u8,9u8,7u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),43nat,seq![0u8,0u8,9u8],7u8,6nat,51nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(seq![0u8,0u8].push(9u8) =~= seq![0u8,0u8,9u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),35nat,seq![0u8,0u8],9u8,6nat,43nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(seq![0u8].push(0u8) =~= seq![0u8,0u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),27nat,seq![0u8],0u8,6nat,35nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(seq![].push(0u8) =~= seq![0u8]);
    fixed::literal_step(z_precedence_index_row0_before_filter_row1(),19nat,seq![],0u8,6nat,27nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat});
    assert(zm::MFinish(z_precedence_index_row0_before_filter_row1(),74nat,seq![0u8,0u8,9u8,7u8,0u8,0u8],6nat,false)==zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:112nat}) by (compute);
    fixed::envelope(z_precedence_index_row0_before_filter_row1(),6nat,zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:74nat},zm::StoredEffect::End {out:seq![0u8,0u8,9u8,7u8,0u8,0u8],position:112nat});
    assert(r::inflated(z_precedence_index_row0_before_filter_row1(),a_precedence_index_row0_before_filter_row1(),STRICT)==zm::Inflated {status:zm::InflateStatus::Done,out:seq![0u8,0u8,9u8,7u8,0u8,0u8],r2:false}) by {reveal(zm::Inflate);}
}
#[verifier::spinoff_prover]
proof fn model_precedence_index_row0_before_filter_row1()
    ensures r::still(bytes_precedence_index_row0_before_filter_row1(),STRICT,false)==r::Projection::Error(Error::PaletteIndexOutOfRange),
{
    hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(crate::model::mh(bytes_precedence_index_row0_before_filter_row1())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_precedence_index_row0_before_filter_row1())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(12nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(4nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:14nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:14nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_precedence_index_row0_before_filter_row1(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_precedence_index_row0_before_filter_row1().subrange(65,79)==z_precedence_index_row0_before_filter_row1()) by (compute);
    walk_precedence_index_row0_before_filter_row1(bytes_precedence_index_row0_before_filter_row1());
    inflate_precedence_index_row0_before_filter_row1();
    let input=seq![0u8,0u8,9u8,7u8,0u8,0u8];
    assert(r::base(a_precedence_index_row0_before_filter_row1(),0)==0nat && r::rb(a_precedence_index_row0_before_filter_row1(),0)==2nat && r::raw(a_precedence_index_row0_before_filter_row1())==6nat) by (compute);
    f::pass_row_equation(input,0,2,1,0);
    f::mf_none(f::row_data(input,0,2,0),None,1,2);
    assert(r::line(input,a_precedence_index_row0_before_filter_row1(),0,0)==seq![0u8,9u8]);
    assert(r::palette_sample(r::line(input,a_precedence_index_row0_before_filter_row1(),0,0),8u8,1nat)==9u8);
    assert(!r::palette_valid(r::line(input,a_precedence_index_row0_before_filter_row1(),0,0),8u8,2nat,4nat));
    assert(r::row_result(input,a_precedence_index_row0_before_filter_row1(),0,0)==Err(Error::PaletteIndexOutOfRange));
    r::first_row_error(input,a_precedence_index_row0_before_filter_row1(),Error::PaletteIndexOutOfRange);
    assert(r::data(z_precedence_index_row0_before_filter_row1(),a_precedence_index_row0_before_filter_row1(),STRICT)==Some(Err(Error::PaletteIndexOutOfRange))) by {reveal(r::data);}
    r::default_error(bytes_precedence_index_row0_before_filter_row1(),h_precedence_index_row0_before_filter_row1(),z_precedence_index_row0_before_filter_row1(),4nat,Error::PaletteIndexOutOfRange);
}
fn public_precedence_index_row0_before_filter_row1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,12u8,80u8,76u8,84u8,69u8,126u8,240u8,116u8,184u8,57u8,97u8,203u8,4u8,217u8,68u8,9u8,86u8,59u8,166u8,158u8,64u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,224u8,100u8,103u8,96u8,0u8,0u8,0u8,63u8,0u8,17u8,245u8,163u8,6u8,133u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_precedence_index_row0_before_filter_row1());model_precedence_index_row0_before_filter_row1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::sites::TryError::Format(Error::PaletteIndexOutOfRange))});
}
// interlace/1x1-filter-bytes-for-empty-passes; frozen fixture file sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_interlace_1x1_filter_bytes_for_empty_passes() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,0u8,0u8,0u8,1u8,77u8,121u8,171u8,195u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,101u8,0u8,3u8,0u8,0u8,43u8,0u8,6u8,30u8,160u8,156u8,4u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_1x1_filter_bytes_for_empty_passes() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,101u8,0u8,3u8,0u8,0u8,43u8,0u8,6u8]}
spec fn h_interlace_1x1_filter_bytes_for_empty_passes() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_1x1_filter_bytes_for_empty_passes() -> r::Raster {r::shape(h_interlace_1x1_filter_bytes_for_empty_passes(),1nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_1x1_filter_bytes_for_empty_passes(b: Seq<u8>)
    requires b.len()==69,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})),
        b.subrange(41,53)==z_interlace_1x1_filter_bytes_for_empty_passes(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:0u8,interlace:1u8},z_interlace_1x1_filter_bytes_for_empty_passes(),0nat),
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
#[verifier::spinoff_prover]
proof fn inflate_interlace_1x1_filter_bytes_for_empty_passes()
    ensures r::inflated(z_interlace_1x1_filter_bytes_for_empty_passes(),a_interlace_1x1_filter_bytes_for_empty_passes(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Full,out:seq![0u8,5u8],r2:false}),
{
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MBlk);hide(zm::MOneBlock);
    hide(zm::MCompressed);hide(zm::MCompressedStep);hide(zm::MDec);hide(zm::decode_prefix);hide(zm::cw);
    assert(r::raw(a_interlace_1x1_filter_bytes_for_empty_passes())==2nat) by (compute);
    assert(zm::field_value(z_interlace_1x1_filter_bytes_for_empty_passes(),16,1)==1nat && zm::field_value(z_interlace_1x1_filter_bytes_for_empty_passes(),17,2)==1nat) by (compute);
    assert(fixed::wire_word(z_interlace_1x1_filter_bytes_for_empty_passes(),19nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_interlace_1x1_filter_bytes_for_empty_passes(),19nat,0int);
    assert(fixed::wire_word(z_interlace_1x1_filter_bytes_for_empty_passes(),27nat,8nat)==zm::msb_bits(53nat,8nat)) by (compute);
    fixed::fixed_decode(z_interlace_1x1_filter_bytes_for_empty_passes(),27nat,5int);
    assert(fixed::wire_word(z_interlace_1x1_filter_bytes_for_empty_passes(),35nat,8nat)==zm::msb_bits(48nat,8nat)) by (compute);
    fixed::fixed_decode(z_interlace_1x1_filter_bytes_for_empty_passes(),35nat,0int);
    fixed::full(z_interlace_1x1_filter_bytes_for_empty_passes(),35nat,seq![0u8,5u8],2nat,0u8,43nat);
    assert(seq![0u8].push(5u8) =~= seq![0u8,5u8]);
    fixed::literal_step(z_interlace_1x1_filter_bytes_for_empty_passes(),27nat,seq![0u8],5u8,2nat,35nat,zm::StoredEffect::Full {out:seq![0u8,5u8]});
    assert(seq![].push(0u8) =~= seq![0u8]);
    fixed::literal_step(z_interlace_1x1_filter_bytes_for_empty_passes(),19nat,seq![],0u8,2nat,27nat,zm::StoredEffect::Full {out:seq![0u8,5u8]});
    fixed::envelope(z_interlace_1x1_filter_bytes_for_empty_passes(),2nat,zm::StoredEffect::Full {out:seq![0u8,5u8]},zm::StoredEffect::Full {out:seq![0u8,5u8]});
    assert(r::inflated(z_interlace_1x1_filter_bytes_for_empty_passes(),a_interlace_1x1_filter_bytes_for_empty_passes(),STRICT)==zm::Inflated {status:zm::InflateStatus::Full,out:seq![0u8,5u8],r2:false}) by {reveal(zm::Inflate);}
}
#[verifier::spinoff_prover]
proof fn model_interlace_1x1_filter_bytes_for_empty_passes()
    ensures r::still(bytes_interlace_1x1_filter_bytes_for_empty_passes(),STRICT,false)==r::Projection::Error(Error::ExcessImageData),
{
    hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(crate::model::mh(bytes_interlace_1x1_filter_bytes_for_empty_passes())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_1x1_filter_bytes_for_empty_passes())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_1x1_filter_bytes_for_empty_passes(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_1x1_filter_bytes_for_empty_passes(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_1x1_filter_bytes_for_empty_passes(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_1x1_filter_bytes_for_empty_passes(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_1x1_filter_bytes_for_empty_passes().subrange(41,53)==z_interlace_1x1_filter_bytes_for_empty_passes()) by (compute);
    walk_interlace_1x1_filter_bytes_for_empty_passes(bytes_interlace_1x1_filter_bytes_for_empty_passes());
    inflate_interlace_1x1_filter_bytes_for_empty_passes();
    assert(r::raw(a_interlace_1x1_filter_bytes_for_empty_passes())==2nat) by (compute);
    assert(r::data(z_interlace_1x1_filter_bytes_for_empty_passes(),a_interlace_1x1_filter_bytes_for_empty_passes(),STRICT)==Some(Err(Error::ExcessImageData))) by {reveal(r::data);}
    r::default_error(bytes_interlace_1x1_filter_bytes_for_empty_passes(),h_interlace_1x1_filter_bytes_for_empty_passes(),z_interlace_1x1_filter_bytes_for_empty_passes(),0nat,Error::ExcessImageData);
}
fn public_interlace_1x1_filter_bytes_for_empty_passes() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,0u8,0u8,0u8,1u8,77u8,121u8,171u8,195u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,101u8,0u8,3u8,0u8,0u8,43u8,0u8,6u8,30u8,160u8,156u8,4u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_1x1_filter_bytes_for_empty_passes());model_interlace_1x1_filter_bytes_for_empty_passes();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::sites::TryError::Format(Error::ExcessImageData))});
}
} // verus!
