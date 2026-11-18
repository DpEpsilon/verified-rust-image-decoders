//! Frozen byte-identical classes; membership is checked by audit_witnesses.py.
use crate::{model as m, witness_literal_class::*, witnesses::check_at, Status};
use vstd::prelude::*;
verus! {
// Class rgb_2x2: 136 independently audited frozen streams; R=14.
#[verifier::spinoff_prover]
fn rgb_2x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0xe9, 0x69, 0x7f, 0xca, 0x7b, 0x3a, 0xe3, 0x43, 0x29, 0x71, 0xe9, 0x3b, 0x59, 0x0, 0x28, 0x41, 0x5, 0x72];
    proof {
        let z = seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114]; let raw = seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106]; let positions = seq![19nat, 27, 36, 44, 52, 61, 69, 78, 86, 95, 103, 111, 119, 128, 136, 143];
        assert(literal_words(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],seq![19nat, 27, 36, 44, 52, 61, 69, 78, 86, 95, 103, 111, 119, 128, 136, 143],0,15)) by (compute);
        literal_decodes(z,raw,positions,0,15); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),14,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],position:143}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),14,32768,true) == (m::StoredEffect::End{out:seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],position:143}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],143,seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],14,false) == (m::StoredEffect::End{out:seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],position:176})) by (compute);
        assert(m::Inflate(z,14,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,14,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 22) by (compute);
    }
    let ghost raw = seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106]; let mut output = [0xa5u8;14]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class palette_2x2: 17 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn palette_2x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x60, 0x64, 0x64, 0xfa, 0xf, 0x0, 0x1, 0x10, 0x1, 0x4];
    proof {
        let z = seq![120u8, 156, 99, 96, 96, 100, 100, 250, 15, 0, 1, 16, 1, 4]; let raw = seq![0u8, 0, 1, 1, 2, 255]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 75];
        assert(literal_words(seq![120u8, 156, 99, 96, 96, 100, 100, 250, 15, 0, 1, 16, 1, 4],seq![0u8, 0, 1, 1, 2, 255],seq![19nat, 27, 35, 43, 51, 59, 68, 75],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 1, 2, 255],position:75}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 96, 100, 100, 250, 15, 0, 1, 16, 1, 4],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 96, 100, 100, 250, 15, 0, 1, 16, 1, 4],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 1, 2, 255],position:75}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 96, 100, 100, 250, 15, 0, 1, 16, 1, 4],75,seq![0u8, 0, 1, 1, 2, 255],6,false) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 1, 2, 255],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 0, 1, 1, 2, 255]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class packed_zero: 4 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn packed_zero() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x0, 0x0, 0x0, 0x2, 0x0, 0x1];
    proof {
        let z = seq![120u8, 156, 99, 96, 0, 0, 0, 2, 0, 1]; let raw = seq![0u8, 0]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 96, 0, 0, 0, 2, 0, 1],seq![0u8, 0],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 0, 0, 0, 2, 0, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 0, 0, 0, 2, 0, 1],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 0, 0, 0, 2, 0, 1],42,seq![0u8, 0],2,false) == (m::StoredEffect::End{out:seq![0u8, 0],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 0]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class packed_high: 4 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn packed_high() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x0, 0x0, 0x0, 0x82, 0x0, 0x81];
    proof {
        let z = seq![120u8, 156, 99, 104, 0, 0, 0, 130, 0, 129]; let raw = seq![0u8, 128]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 104, 0, 0, 0, 130, 0, 129],seq![0u8, 128],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 0, 0, 0, 130, 0, 129],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 0, 0, 0, 130, 0, 129],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 0, 0, 0, 130, 0, 129],42,seq![0u8, 128],2,false) == (m::StoredEffect::End{out:seq![0u8, 128],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 128]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x70, 0x60, 0x74, 0x60, 0x62, 0x0, 0x0, 0x2, 0xe, 0x0, 0x84];
    proof {
        let z = seq![120u8, 156, 99, 112, 96, 116, 96, 98, 0, 0, 2, 14, 0, 132]; let raw = seq![0u8, 64, 1, 64, 2, 0]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 74];
        assert(literal_words(seq![120u8, 156, 99, 112, 96, 116, 96, 98, 0, 0, 2, 14, 0, 132],seq![0u8, 64, 1, 64, 2, 0],seq![19nat, 27, 35, 43, 51, 59, 67, 74],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 64, 2, 0],position:74}));
        assert(m::field_value(seq![120u8, 156, 99, 112, 96, 116, 96, 98, 0, 0, 2, 14, 0, 132],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 112, 96, 116, 96, 98, 0, 0, 2, 14, 0, 132],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 64, 2, 0],position:74}));
        assert(m::MFinish(seq![120u8, 156, 99, 112, 96, 116, 96, 98, 0, 0, 2, 14, 0, 132],74,seq![0u8, 64, 1, 64, 2, 0],6,false) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 64, 2, 0],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 64, 1, 64, 2, 0]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il0_7x5: 2 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf0, 0x63, 0x9c, 0xc4, 0xf4, 0x80, 0x59, 0x9d, 0xe5, 0x1b, 0x0, 0xc, 0xb9, 0x2, 0xe8];
    proof {
        let z = seq![120u8, 156, 99, 240, 99, 156, 196, 244, 128, 89, 157, 229, 27, 0, 12, 185, 2, 232]; let raw = seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246]; let positions = seq![19nat, 27, 35, 43, 52, 60, 69, 77, 85, 93, 102, 109];
        assert(literal_words(seq![120u8, 156, 99, 240, 99, 156, 196, 244, 128, 89, 157, 229, 27, 0, 12, 185, 2, 232],seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246],seq![19nat, 27, 35, 43, 52, 60, 69, 77, 85, 93, 102, 109],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246],position:109}));
        assert(m::field_value(seq![120u8, 156, 99, 240, 99, 156, 196, 244, 128, 89, 157, 229, 27, 0, 12, 185, 2, 232],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 240, 99, 156, 196, 244, 128, 89, 157, 229, 27, 0, 12, 185, 2, 232],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246],position:109}));
        assert(m::MFinish(seq![120u8, 156, 99, 240, 99, 156, 196, 244, 128, 89, 157, 229, 27, 0, 12, 185, 2, 232],109,seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246],10,false) == (m::StoredEffect::End{out:seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 78, 1, 146, 2, 224, 3, 39, 4, 246]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il0_9x10: 1 independently audited frozen streams; R=30.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il0_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x30, 0x60, 0x60, 0xfc, 0xdd, 0xca, 0x54, 0xd4, 0xc0, 0xbc, 0x8d, 0x8b, 0xe5, 0x74, 0x3, 0x83, 0xe, 0x3, 0x63, 0x5f, 0x11, 0xd3, 0x3d, 0x6, 0xe6, 0xcc, 0x8d, 0x2c, 0xd2, 0xd, 0x0, 0x80, 0x33, 0x8, 0x81];
    proof {
        let z = seq![120u8, 156, 99, 48, 96, 96, 252, 221, 202, 84, 212, 192, 188, 141, 139, 229, 116, 3, 131, 14, 3, 99, 95, 17, 211, 61, 6, 230, 204, 141, 44, 210, 13, 0, 128, 51, 8, 129]; let raw = seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128]; let positions = seq![19nat, 27, 35, 43, 51, 60, 68, 76, 84, 92, 100, 109, 117, 125, 134, 142, 150, 158, 166, 174, 182, 190, 198, 207, 215, 223, 231, 240, 248, 256, 264, 271];
        assert(literal_words(seq![120u8, 156, 99, 48, 96, 96, 252, 221, 202, 84, 212, 192, 188, 141, 139, 229, 116, 3, 131, 14, 3, 99, 95, 17, 211, 61, 6, 230, 204, 141, 44, 210, 13, 0, 128, 51, 8, 129],seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128],seq![19nat, 27, 35, 43, 51, 60, 68, 76, 84, 92, 100, 109, 117, 125, 134, 142, 150, 158, 166, 174, 182, 190, 198, 207, 215, 223, 231, 240, 248, 256, 264, 271],0,31)) by (compute);
        literal_decodes(z,raw,positions,0,31); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),30,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128],position:271}));
        assert(m::field_value(seq![120u8, 156, 99, 48, 96, 96, 252, 221, 202, 84, 212, 192, 188, 141, 139, 229, 116, 3, 131, 14, 3, 99, 95, 17, 211, 61, 6, 230, 204, 141, 44, 210, 13, 0, 128, 51, 8, 129],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 48, 96, 96, 252, 221, 202, 84, 212, 192, 188, 141, 139, 229, 116, 3, 131, 14, 3, 99, 95, 17, 211, 61, 6, 230, 204, 141, 44, 210, 13, 0, 128, 51, 8, 129],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),30,32768,true) == (m::StoredEffect::End{out:seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128],position:271}));
        assert(m::MFinish(seq![120u8, 156, 99, 48, 96, 96, 252, 221, 202, 84, 212, 192, 188, 141, 139, 229, 116, 3, 131, 14, 3, 99, 95, 17, 211, 61, 6, 230, 204, 141, 44, 210, 13, 0, 128, 51, 8, 129],271,seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128],30,false) == (m::StoredEffect::End{out:seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128],position:304})) by (compute);
        assert(m::Inflate(z,30,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,30,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 38) by (compute);
    }
    let ghost raw = seq![0u8, 48, 0, 1, 251, 133, 2, 114, 128, 3, 182, 10, 4, 203, 128, 0, 44, 0, 1, 142, 114, 2, 222, 0, 3, 105, 177, 4, 27, 128]; let mut output = [0xa5u8;30]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il0_17x2: 1 independently audited frozen streams; R=8.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xe8, 0xf2, 0x6a, 0x60, 0x3c, 0x6e, 0xd6, 0xc, 0x0, 0xb, 0x52, 0x2, 0xd6];
    proof {
        let z = seq![120u8, 156, 99, 232, 242, 106, 96, 60, 110, 214, 12, 0, 11, 82, 2, 214]; let raw = seq![0u8, 138, 74, 128, 1, 199, 54, 131]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 91];
        assert(literal_words(seq![120u8, 156, 99, 232, 242, 106, 96, 60, 110, 214, 12, 0, 11, 82, 2, 214],seq![0u8, 138, 74, 128, 1, 199, 54, 131],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 91],0,9)) by (compute);
        literal_decodes(z,raw,positions,0,9); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),8,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 138, 74, 128, 1, 199, 54, 131],position:91}));
        assert(m::field_value(seq![120u8, 156, 99, 232, 242, 106, 96, 60, 110, 214, 12, 0, 11, 82, 2, 214],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 232, 242, 106, 96, 60, 110, 214, 12, 0, 11, 82, 2, 214],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),8,32768,true) == (m::StoredEffect::End{out:seq![0u8, 138, 74, 128, 1, 199, 54, 131],position:91}));
        assert(m::MFinish(seq![120u8, 156, 99, 232, 242, 106, 96, 60, 110, 214, 12, 0, 11, 82, 2, 214],91,seq![0u8, 138, 74, 128, 1, 199, 54, 131],8,false) == (m::StoredEffect::End{out:seq![0u8, 138, 74, 128, 1, 199, 54, 131],position:128})) by (compute);
        assert(m::Inflate(z,8,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,8,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
    }
    let ghost raw = seq![0u8, 138, 74, 128, 1, 199, 54, 131]; let mut output = [0xa5u8;8]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x6c, 0x60, 0x62, 0x60, 0x66, 0x60, 0x39, 0x0, 0x0, 0x8, 0xf2, 0x1, 0xcb];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 108, 96, 98, 96, 102, 96, 57, 0, 0, 8, 242, 1, 203]; let raw = seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 107];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 108, 96, 98, 96, 102, 96, 57, 0, 0, 8, 242, 1, 203],seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 107],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192],position:107}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 108, 96, 98, 96, 102, 96, 57, 0, 0, 8, 242, 1, 203],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 108, 96, 98, 96, 102, 96, 57, 0, 0, 8, 242, 1, 203],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192],position:107}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 108, 96, 98, 96, 102, 96, 57, 0, 0, 8, 242, 1, 203],107,seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192],10,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 128, 2, 0, 3, 0, 4, 192]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il1_7x5: 2 independently audited frozen streams; R=22.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x60, 0x6c, 0x60, 0x62, 0x60, 0x66, 0x60, 0x39, 0xc0, 0xb0, 0x80, 0xd1, 0x81, 0xe9, 0x0, 0xb3, 0x3, 0x4b, 0x8, 0xc3, 0x2e, 0x0, 0x25, 0x88, 0x4, 0x43];
    proof {
        let z = seq![120u8, 156, 99, 96, 96, 108, 96, 98, 96, 102, 96, 57, 192, 176, 128, 209, 129, 233, 0, 179, 3, 75, 8, 195, 46, 0, 37, 136, 4, 67]; let raw = seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 108, 117, 125, 133, 141, 150, 158, 166, 174, 182, 190, 199, 206];
        assert(literal_words(seq![120u8, 156, 99, 96, 96, 108, 96, 98, 96, 102, 96, 57, 192, 176, 128, 209, 129, 233, 0, 179, 3, 75, 8, 195, 46, 0, 37, 136, 4, 67],seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 108, 117, 125, 133, 141, 150, 158, 166, 174, 182, 190, 199, 206],0,23)) by (compute);
        literal_decodes(z,raw,positions,0,23); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),22,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186],position:206}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 96, 108, 96, 98, 96, 102, 96, 57, 192, 176, 128, 209, 129, 233, 0, 179, 3, 75, 8, 195, 46, 0, 37, 136, 4, 67],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 96, 108, 96, 98, 96, 102, 96, 57, 192, 176, 128, 209, 129, 233, 0, 179, 3, 75, 8, 195, 46, 0, 37, 136, 4, 67],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),22,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186],position:206}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 96, 108, 96, 98, 96, 102, 96, 57, 192, 176, 128, 209, 129, 233, 0, 179, 3, 75, 8, 195, 46, 0, 37, 136, 4, 67],206,seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186],22,false) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186],position:240})) by (compute);
        assert(m::Inflate(z,22,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,22,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8, 0, 1, 128, 2, 0, 3, 0, 4, 192, 0, 160, 1, 64, 2, 192, 3, 64, 4, 84, 0, 186]; let mut output = [0xa5u8;22]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d1_il1_17x2: 1 independently audited frozen streams; R=12.
#[verifier::spinoff_prover]
fn intent_ct0_d1_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x70, 0x60, 0x64, 0x60, 0xa, 0x60, 0x5e, 0xc8, 0x92, 0x97, 0xbe, 0x1a, 0x0, 0xb, 0x20, 0x2, 0xbc];
    proof {
        let z = seq![120u8, 156, 99, 112, 96, 100, 96, 10, 96, 94, 200, 146, 151, 190, 26, 0, 11, 32, 2, 188]; let raw = seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 124];
        assert(literal_words(seq![120u8, 156, 99, 112, 96, 100, 96, 10, 96, 94, 200, 146, 151, 190, 26, 0, 11, 32, 2, 188],seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 124],0,13)) by (compute);
        literal_decodes(z,raw,positions,0,13); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),12,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171],position:124}));
        assert(m::field_value(seq![120u8, 156, 99, 112, 96, 100, 96, 10, 96, 94, 200, 146, 151, 190, 26, 0, 11, 32, 2, 188],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 112, 96, 100, 96, 10, 96, 94, 200, 146, 151, 190, 26, 0, 11, 32, 2, 188],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),12,32768,true) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171],position:124}));
        assert(m::MFinish(seq![120u8, 156, 99, 112, 96, 100, 96, 10, 96, 94, 200, 146, 151, 190, 26, 0, 11, 32, 2, 188],124,seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171],12,false) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171],position:160})) by (compute);
        assert(m::Inflate(z,12,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,12,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 20) by (compute);
    }
    let ghost raw = seq![0u8, 64, 1, 0, 2, 80, 3, 161, 4, 110, 103, 171]; let mut output = [0xa5u8;12]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il0_1x1: 2 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x70, 0x0, 0x0, 0x0, 0x42, 0x0, 0x41];
    proof {
        let z = seq![120u8, 156, 99, 112, 0, 0, 0, 66, 0, 65]; let raw = seq![0u8, 64]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 112, 0, 0, 0, 66, 0, 65],seq![0u8, 64],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 64],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 112, 0, 0, 0, 66, 0, 65],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 112, 0, 0, 0, 66, 0, 65],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 64],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 112, 0, 0, 0, 66, 0, 65],42,seq![0u8, 64],2,false) == (m::StoredEffect::End{out:seq![0u8, 64],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 64]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0xc0, 0xd8, 0xc0, 0xd4, 0x0, 0x0, 0x5, 0xce, 0x1, 0xc4];
    proof {
        let z = seq![120u8, 156, 99, 56, 192, 216, 192, 212, 0, 0, 5, 206, 1, 196]; let raw = seq![0u8, 192, 1, 128, 2, 128]; let positions = seq![19nat, 27, 36, 44, 52, 60, 68, 75];
        assert(literal_words(seq![120u8, 156, 99, 56, 192, 216, 192, 212, 0, 0, 5, 206, 1, 196],seq![0u8, 192, 1, 128, 2, 128],seq![19nat, 27, 36, 44, 52, 60, 68, 75],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 128, 2, 128],position:75}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 192, 216, 192, 212, 0, 0, 5, 206, 1, 196],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 192, 216, 192, 212, 0, 0, 5, 206, 1, 196],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 128, 2, 128],position:75}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 192, 216, 192, 212, 0, 0, 5, 206, 1, 196],75,seq![0u8, 192, 1, 128, 2, 128],6,false) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 128, 2, 128],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 192, 1, 128, 2, 128]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il0_7x5: 2 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf0, 0xe, 0x61, 0x7c, 0xe5, 0xc6, 0xd4, 0x2c, 0xc2, 0xfc, 0x82, 0x9d, 0xa5, 0x59, 0x1e, 0x0, 0x1f, 0xe0, 0x4, 0x2];
    proof {
        let z = seq![120u8, 156, 99, 240, 14, 97, 124, 229, 198, 212, 44, 194, 252, 130, 157, 165, 89, 30, 0, 31, 224, 4, 2]; let raw = seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31]; let positions = seq![19nat, 27, 35, 43, 51, 60, 68, 76, 84, 92, 100, 109, 117, 125, 133, 141, 148];
        assert(literal_words(seq![120u8, 156, 99, 240, 14, 97, 124, 229, 198, 212, 44, 194, 252, 130, 157, 165, 89, 30, 0, 31, 224, 4, 2],seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31],seq![19nat, 27, 35, 43, 51, 60, 68, 76, 84, 92, 100, 109, 117, 125, 133, 141, 148],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31],position:148}));
        assert(m::field_value(seq![120u8, 156, 99, 240, 14, 97, 124, 229, 198, 212, 44, 194, 252, 130, 157, 165, 89, 30, 0, 31, 224, 4, 2],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 240, 14, 97, 124, 229, 198, 212, 44, 194, 252, 130, 157, 165, 89, 30, 0, 31, 224, 4, 2],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31],position:148}));
        assert(m::MFinish(seq![120u8, 156, 99, 240, 14, 97, 124, 229, 198, 212, 44, 194, 252, 130, 157, 165, 89, 30, 0, 31, 224, 4, 2],148,seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31],15,false) == (m::StoredEffect::End{out:seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31],position:184})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 23) by (compute);
    }
    let ghost raw = seq![0u8, 75, 84, 1, 234, 70, 2, 131, 20, 3, 232, 7, 4, 131, 31]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il0_9x10: 1 frozen stored streams; R=40.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il0_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x28, 0x0, 0xd7, 0xff, 0x0, 0x98, 0x64, 0x0, 0x1, 0xb7, 0xdf, 0x2a, 0x2, 0x3e, 0xc, 0x0, 0x3, 0x80, 0xff, 0xba, 0x4, 0xff, 0xa3, 0x50, 0x0, 0x9f, 0xc2, 0xc0, 0x1, 0xf3, 0xc0, 0xcd, 0x2, 0x37, 0xd, 0x40, 0x3, 0xc6, 0xfa, 0x7d, 0x4, 0x13, 0x4d, 0x58, 0x2e, 0xf9, 0xf, 0x60];
    proof {
        let z = seq![120u8, 156, 1, 40, 0, 215, 255, 0, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88, 46, 249, 15, 96]; let raw = seq![0u8, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88];
        assert(z.subrange(7,47) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88, 46, 249, 15, 96],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88, 46, 249, 15, 96],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),40) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MOneBlock(z,16,Seq::empty(),40,32768,true) == (true,m::StoredEffect::End{out:raw,position:376}));
        assert(m::MBlk(z,16,Seq::empty(),40,32768,true) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MFinish(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88, 46, 249, 15, 96],376,seq![0u8, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88],40,false) == (m::StoredEffect::End{out:seq![0u8, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88],position:408})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,40,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,40,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 51) by (compute);
    }
    let ghost raw = seq![0u8, 152, 100, 0, 1, 183, 223, 42, 2, 62, 12, 0, 3, 128, 255, 186, 4, 255, 163, 80, 0, 159, 194, 192, 1, 243, 192, 205, 2, 55, 13, 64, 3, 198, 250, 125, 4, 19, 77, 88]; let mut output = [0xa5u8;40]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il0_17x2: 1 independently audited frozen streams; R=12.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xb0, 0xac, 0x3a, 0xb4, 0x8e, 0x81, 0x31, 0xec, 0xa9, 0x7a, 0xcd, 0x23, 0x0, 0x1b, 0x1c, 0x4, 0xe5];
    proof {
        let z = seq![120u8, 156, 99, 176, 172, 58, 180, 142, 129, 49, 236, 169, 122, 205, 35, 0, 27, 28, 4, 229]; let raw = seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226]; let positions = seq![19nat, 27, 35, 43, 52, 61, 69, 77, 85, 94, 102, 110, 119, 126];
        assert(literal_words(seq![120u8, 156, 99, 176, 172, 58, 180, 142, 129, 49, 236, 169, 122, 205, 35, 0, 27, 28, 4, 229],seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226],seq![19nat, 27, 35, 43, 52, 61, 69, 77, 85, 94, 102, 110, 119, 126],0,13)) by (compute);
        literal_decodes(z,raw,positions,0,13); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),12,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226],position:126}));
        assert(m::field_value(seq![120u8, 156, 99, 176, 172, 58, 180, 142, 129, 49, 236, 169, 122, 205, 35, 0, 27, 28, 4, 229],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 176, 172, 58, 180, 142, 129, 49, 236, 169, 122, 205, 35, 0, 27, 28, 4, 229],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),12,32768,true) == (m::StoredEffect::End{out:seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226],position:126}));
        assert(m::MFinish(seq![120u8, 156, 99, 176, 172, 58, 180, 142, 129, 49, 236, 169, 122, 205, 35, 0, 27, 28, 4, 229],126,seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226],12,false) == (m::StoredEffect::End{out:seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226],position:160})) by (compute);
        assert(m::Inflate(z,12,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,12,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 20) by (compute);
    }
    let ghost raw = seq![0u8, 57, 122, 194, 174, 0, 1, 86, 229, 39, 124, 226]; let mut output = [0xa5u8;12]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il1_1x1: 2 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0x0, 0x0, 0x0, 0xc2, 0x0, 0xc1];
    proof {
        let z = seq![120u8, 156, 99, 56, 0, 0, 0, 194, 0, 193]; let raw = seq![0u8, 192]; let positions = seq![19nat, 27, 36, 43];
        assert(literal_words(seq![120u8, 156, 99, 56, 0, 0, 0, 194, 0, 193],seq![0u8, 192],seq![19nat, 27, 36, 43],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 192],position:43}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 0, 0, 0, 194, 0, 193],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 0, 0, 0, 194, 0, 193],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 192],position:43}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 0, 0, 0, 194, 0, 193],43,seq![0u8, 192],2,false) == (m::StoredEffect::End{out:seq![0u8, 192],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 192]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x60, 0x3c, 0xc0, 0xc4, 0xc0, 0x7c, 0x80, 0x45, 0x0, 0x0, 0x7, 0xc2, 0x1, 0x9b];
    proof {
        let z = seq![120u8, 156, 99, 96, 96, 60, 192, 196, 192, 124, 128, 69, 0, 0, 7, 194, 1, 155]; let raw = seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 101, 108];
        assert(literal_words(seq![120u8, 156, 99, 96, 96, 60, 192, 196, 192, 124, 128, 69, 0, 0, 7, 194, 1, 155],seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 101, 108],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16],position:108}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 96, 60, 192, 196, 192, 124, 128, 69, 0, 0, 7, 194, 1, 155],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 96, 60, 192, 196, 192, 124, 128, 69, 0, 0, 7, 194, 1, 155],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16],position:108}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 96, 60, 192, 196, 192, 124, 128, 69, 0, 0, 7, 194, 1, 155],108,seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16],10,false) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 0, 1, 192, 2, 0, 3, 192, 4, 16]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il1_7x5: 2 independently audited frozen streams; R=24.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0xc0, 0xc8, 0xc0, 0xb4, 0x81, 0xd9, 0x81, 0x25, 0x80, 0x61, 0x37, 0x63, 0xa, 0xd3, 0x15, 0xe6, 0x13, 0x2c, 0xf3, 0x84, 0x18, 0x4a, 0x42, 0x0, 0x47, 0xad, 0x6, 0x48];
    proof {
        let z = seq![120u8, 156, 99, 56, 192, 200, 192, 180, 129, 217, 129, 37, 128, 97, 55, 99, 10, 211, 21, 230, 19, 44, 243, 132, 24, 74, 66, 0, 71, 173, 6, 72]; let raw = seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84]; let positions = seq![19nat, 27, 36, 44, 52, 60, 69, 77, 85, 93, 101, 109, 118, 126, 134, 142, 151, 159, 168, 176, 185, 193, 201, 209, 217, 224];
        assert(literal_words(seq![120u8, 156, 99, 56, 192, 200, 192, 180, 129, 217, 129, 37, 128, 97, 55, 99, 10, 211, 21, 230, 19, 44, 243, 132, 24, 74, 66, 0, 71, 173, 6, 72],seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84],seq![19nat, 27, 36, 44, 52, 60, 69, 77, 85, 93, 101, 109, 118, 126, 134, 142, 151, 159, 168, 176, 185, 193, 201, 209, 217, 224],0,25)) by (compute);
        literal_decodes(z,raw,positions,0,25); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),24,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84],position:224}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 192, 200, 192, 180, 129, 217, 129, 37, 128, 97, 55, 99, 10, 211, 21, 230, 19, 44, 243, 132, 24, 74, 66, 0, 71, 173, 6, 72],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 192, 200, 192, 180, 129, 217, 129, 37, 128, 97, 55, 99, 10, 211, 21, 230, 19, 44, 243, 132, 24, 74, 66, 0, 71, 173, 6, 72],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),24,32768,true) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84],position:224}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 192, 200, 192, 180, 129, 217, 129, 37, 128, 97, 55, 99, 10, 211, 21, 230, 19, 44, 243, 132, 24, 74, 66, 0, 71, 173, 6, 72],224,seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84],24,false) == (m::StoredEffect::End{out:seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84],position:256})) by (compute);
        assert(m::Inflate(z,24,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,24,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 32) by (compute);
    }
    let ghost raw = seq![0u8, 192, 1, 0, 2, 176, 3, 64, 4, 80, 0, 187, 1, 100, 2, 212, 3, 200, 4, 158, 18, 0, 116, 84]; let mut output = [0xa5u8;24]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il1_9x10: 1 independently audited frozen streams; R=52.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il1_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0x60, 0x7c, 0xc0, 0xd4, 0xc0, 0xec, 0xc0, 0x92, 0xc3, 0xd0, 0xc0, 0xe8, 0xc0, 0xb4, 0x81, 0xb9, 0xe5, 0x1f, 0xcb, 0x74, 0x55, 0x6, 0x27, 0xc6, 0x89, 0x4c, 0x2d, 0xcc, 0xde, 0x2c, 0x76, 0xc, 0x55, 0xa9, 0xc, 0x8c, 0x3d, 0x5, 0x47, 0x98, 0x72, 0x7f, 0x37, 0x30, 0xff, 0xbd, 0x7d, 0x9d, 0xa5, 0x4b, 0x31, 0x6, 0x0, 0x6b, 0xf, 0x10, 0x20];
    proof {
        let z = seq![120u8, 156, 99, 80, 96, 124, 192, 212, 192, 236, 192, 146, 195, 208, 192, 232, 192, 180, 129, 185, 229, 31, 203, 116, 85, 6, 39, 198, 137, 76, 45, 204, 222, 44, 118, 12, 85, 169, 12, 140, 61, 5, 71, 152, 114, 127, 55, 48, 255, 189, 125, 157, 165, 75, 49, 6, 0, 107, 15, 16, 32]; let raw = seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 100, 108, 116, 124, 132, 140, 149, 157, 165, 174, 182, 191, 199, 207, 215, 223, 232, 240, 248, 256, 264, 272, 280, 288, 296, 304, 312, 320, 328, 336, 345, 353, 361, 370, 378, 386, 395, 404, 413, 421, 429, 437, 445, 452];
        assert(literal_words(seq![120u8, 156, 99, 80, 96, 124, 192, 212, 192, 236, 192, 146, 195, 208, 192, 232, 192, 180, 129, 185, 229, 31, 203, 116, 85, 6, 39, 198, 137, 76, 45, 204, 222, 44, 118, 12, 85, 169, 12, 140, 61, 5, 71, 152, 114, 127, 55, 48, 255, 189, 125, 157, 165, 75, 49, 6, 0, 107, 15, 16, 32],seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 100, 108, 116, 124, 132, 140, 149, 157, 165, 174, 182, 191, 199, 207, 215, 223, 232, 240, 248, 256, 264, 272, 280, 288, 296, 304, 312, 320, 328, 336, 345, 353, 361, 370, 378, 386, 395, 404, 413, 421, 429, 437, 445, 452],0,53)) by (compute);
        literal_decodes(z,raw,positions,0,53); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),52,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92],position:452}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 96, 124, 192, 212, 192, 236, 192, 146, 195, 208, 192, 232, 192, 180, 129, 185, 229, 31, 203, 116, 85, 6, 39, 198, 137, 76, 45, 204, 222, 44, 118, 12, 85, 169, 12, 140, 61, 5, 71, 152, 114, 127, 55, 48, 255, 189, 125, 157, 165, 75, 49, 6, 0, 107, 15, 16, 32],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 96, 124, 192, 212, 192, 236, 192, 146, 195, 208, 192, 232, 192, 180, 129, 185, 229, 31, 203, 116, 85, 6, 39, 198, 137, 76, 45, 204, 222, 44, 118, 12, 85, 169, 12, 140, 61, 5, 71, 152, 114, 127, 55, 48, 255, 189, 125, 157, 165, 75, 49, 6, 0, 107, 15, 16, 32],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),52,32768,true) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92],position:452}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 96, 124, 192, 212, 192, 236, 192, 146, 195, 208, 192, 232, 192, 180, 129, 185, 229, 31, 203, 116, 85, 6, 39, 198, 137, 76, 45, 204, 222, 44, 118, 12, 85, 169, 12, 140, 61, 5, 71, 152, 114, 127, 55, 48, 255, 189, 125, 157, 165, 75, 49, 6, 0, 107, 15, 16, 32],452,seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92],52,false) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92],position:488})) by (compute);
        assert(m::Inflate(z,52,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,52,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 61) by (compute);
    }
    let ghost raw = seq![0u8, 32, 1, 224, 2, 128, 3, 64, 4, 108, 0, 128, 1, 64, 2, 176, 3, 132, 254, 4, 151, 37, 0, 66, 1, 145, 2, 132, 3, 75, 4, 62, 0, 122, 101, 0, 1, 140, 112, 196, 2, 109, 251, 128, 3, 253, 219, 215, 4, 138, 33, 92]; let mut output = [0xa5u8;52]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d2_il1_17x2: 1 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct0_d2_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x28, 0x60, 0x2c, 0x60, 0x5a, 0xcf, 0xec, 0xa6, 0xc1, 0x52, 0xd2, 0xbd, 0xeb, 0xf1, 0x12, 0x0, 0x1e, 0xeb, 0x5, 0x48];
    proof {
        let z = seq![120u8, 156, 99, 40, 96, 44, 96, 90, 207, 236, 166, 193, 82, 210, 189, 235, 241, 18, 0, 30, 235, 5, 72]; let raw = seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 92, 100, 108, 116, 125, 134, 143, 150];
        assert(literal_words(seq![120u8, 156, 99, 40, 96, 44, 96, 90, 207, 236, 166, 193, 82, 210, 189, 235, 241, 18, 0, 30, 235, 5, 72],seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 92, 100, 108, 116, 125, 134, 143, 150],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164],position:150}));
        assert(m::field_value(seq![120u8, 156, 99, 40, 96, 44, 96, 90, 207, 236, 166, 193, 82, 210, 189, 235, 241, 18, 0, 30, 235, 5, 72],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 40, 96, 44, 96, 90, 207, 236, 166, 193, 82, 210, 189, 235, 241, 18, 0, 30, 235, 5, 72],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164],position:150}));
        assert(m::MFinish(seq![120u8, 156, 99, 40, 96, 44, 96, 90, 207, 236, 166, 193, 82, 210, 189, 235, 241, 18, 0, 30, 235, 5, 72],150,seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164],15,false) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164],position:184})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 23) by (compute);
    }
    let ghost raw = seq![0u8, 112, 1, 112, 2, 175, 3, 70, 40, 4, 116, 139, 186, 227, 164]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x30, 0x61, 0x5c, 0xc0, 0x24, 0x9, 0x0, 0x3, 0xb, 0x0, 0xf1];
    proof {
        let z = seq![120u8, 156, 99, 48, 97, 92, 192, 36, 9, 0, 3, 11, 0, 241]; let raw = seq![0u8, 52, 1, 160, 2, 25]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 75];
        assert(literal_words(seq![120u8, 156, 99, 48, 97, 92, 192, 36, 9, 0, 3, 11, 0, 241],seq![0u8, 52, 1, 160, 2, 25],seq![19nat, 27, 35, 43, 52, 60, 68, 75],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 52, 1, 160, 2, 25],position:75}));
        assert(m::field_value(seq![120u8, 156, 99, 48, 97, 92, 192, 36, 9, 0, 3, 11, 0, 241],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 48, 97, 92, 192, 36, 9, 0, 3, 11, 0, 241],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 52, 1, 160, 2, 25],position:75}));
        assert(m::MFinish(seq![120u8, 156, 99, 48, 97, 92, 192, 36, 9, 0, 3, 11, 0, 241],75,seq![0u8, 52, 1, 160, 2, 25],6,false) == (m::StoredEffect::End{out:seq![0u8, 52, 1, 160, 2, 25],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 52, 1, 160, 2, 25]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il0_7x5: 2 independently audited frozen streams; R=25.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x58, 0x96, 0x17, 0xb6, 0x80, 0xf1, 0xef, 0xe1, 0x69, 0xab, 0x98, 0xa6, 0x89, 0x6, 0x5f, 0x60, 0x8e, 0x8e, 0x48, 0x63, 0x63, 0x29, 0x4a, 0xff, 0x29, 0x3, 0x0, 0x87, 0x8b, 0x9, 0xf0];
    proof {
        let z = seq![120u8, 156, 99, 88, 150, 23, 182, 128, 241, 239, 225, 105, 171, 152, 166, 137, 6, 95, 96, 142, 142, 72, 99, 99, 41, 74, 255, 41, 3, 0, 135, 139, 9, 240]; let raw = seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28]; let positions = seq![19nat, 27, 36, 44, 52, 61, 69, 78, 87, 96, 105, 113, 122, 130, 138, 147, 155, 163, 171, 179, 187, 195, 203, 211, 220, 228, 235];
        assert(literal_words(seq![120u8, 156, 99, 88, 150, 23, 182, 128, 241, 239, 225, 105, 171, 152, 166, 137, 6, 95, 96, 142, 142, 72, 99, 99, 41, 74, 255, 41, 3, 0, 135, 139, 9, 240],seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28],seq![19nat, 27, 36, 44, 52, 61, 69, 78, 87, 96, 105, 113, 122, 130, 138, 147, 155, 163, 171, 179, 187, 195, 203, 211, 220, 228, 235],0,26)) by (compute);
        literal_decodes(z,raw,positions,0,26); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),25,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28],position:235}));
        assert(m::field_value(seq![120u8, 156, 99, 88, 150, 23, 182, 128, 241, 239, 225, 105, 171, 152, 166, 137, 6, 95, 96, 142, 142, 72, 99, 99, 41, 74, 255, 41, 3, 0, 135, 139, 9, 240],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 88, 150, 23, 182, 128, 241, 239, 225, 105, 171, 152, 166, 137, 6, 95, 96, 142, 142, 72, 99, 99, 41, 74, 255, 41, 3, 0, 135, 139, 9, 240],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),25,32768,true) == (m::StoredEffect::End{out:seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28],position:235}));
        assert(m::MFinish(seq![120u8, 156, 99, 88, 150, 23, 182, 128, 241, 239, 225, 105, 171, 152, 166, 137, 6, 95, 96, 142, 142, 72, 99, 99, 41, 74, 255, 41, 3, 0, 135, 139, 9, 240],235,seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28],25,false) == (m::StoredEffect::End{out:seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28],position:272})) by (compute);
        assert(m::Inflate(z,25,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,25,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 34) by (compute);
    }
    let ghost raw = seq![0u8, 166, 110, 86, 160, 1, 253, 195, 150, 170, 2, 150, 21, 83, 208, 3, 91, 88, 102, 6, 4, 114, 103, 249, 28]; let mut output = [0xa5u8;25]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il0_9x10: 1 frozen stored streams; R=60.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il0_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x3c, 0x0, 0xc3, 0xff, 0x0, 0x78, 0x0, 0x60, 0x72, 0x30, 0x1, 0x20, 0xa2, 0xfe, 0x9f, 0x11, 0x2, 0x1e, 0xb6, 0x64, 0x9a, 0x40, 0x3, 0xda, 0xea, 0x9f, 0xa8, 0x6, 0x4, 0xee, 0xae, 0x56, 0xd1, 0xc0, 0x0, 0x4d, 0xb1, 0x93, 0xb0, 0x50, 0x1, 0xfa, 0x7c, 0x99, 0xa4, 0x8d, 0x2, 0xd4, 0xc8, 0x35, 0x81, 0x90, 0x3, 0x87, 0xd5, 0xd2, 0x11, 0x79, 0x4, 0x1e, 0x63, 0x10, 0xf5, 0x60, 0xf2, 0x7, 0x19, 0xf6];
    proof {
        let z = seq![120u8, 156, 1, 60, 0, 195, 255, 0, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96, 242, 7, 25, 246]; let raw = seq![0u8, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96];
        assert(z.subrange(7,67) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96, 242, 7, 25, 246],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96, 242, 7, 25, 246],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),60) == (m::StoredEffect::End{out:raw,position:536}));
        assert(m::MOneBlock(z,16,Seq::empty(),60,32768,true) == (true,m::StoredEffect::End{out:raw,position:536}));
        assert(m::MBlk(z,16,Seq::empty(),60,32768,true) == (m::StoredEffect::End{out:raw,position:536}));
        assert(m::MFinish(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96, 242, 7, 25, 246],536,seq![0u8, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96],60,false) == (m::StoredEffect::End{out:seq![0u8, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96],position:568})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,60,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,60,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 71) by (compute);
    }
    let ghost raw = seq![0u8, 120, 0, 96, 114, 48, 1, 32, 162, 254, 159, 17, 2, 30, 182, 100, 154, 64, 3, 218, 234, 159, 168, 6, 4, 238, 174, 86, 209, 192, 0, 77, 177, 147, 176, 80, 1, 250, 124, 153, 164, 141, 2, 212, 200, 53, 129, 144, 3, 135, 213, 210, 17, 121, 4, 30, 99, 16, 245, 96]; let mut output = [0xa5u8;60]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il0_17x2: 1 independently audited frozen streams; R=20.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xd8, 0xf6, 0xf2, 0xba, 0xe9, 0xb3, 0x3, 0xad, 0x65, 0xc, 0x8c, 0x2d, 0x32, 0x6e, 0x4f, 0x1a, 0xa3, 0xf6, 0x27, 0x77, 0x2, 0x0, 0x68, 0x1b, 0x9, 0x9e];
    proof {
        let z = seq![120u8, 156, 99, 216, 246, 242, 186, 233, 179, 3, 173, 101, 12, 140, 45, 50, 110, 79, 26, 163, 246, 39, 119, 2, 0, 104, 27, 9, 158]; let raw = seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137]; let positions = seq![19nat, 27, 36, 45, 54, 62, 71, 80, 88, 96, 104, 112, 120, 128, 136, 145, 153, 161, 170, 178, 186, 193];
        assert(literal_words(seq![120u8, 156, 99, 216, 246, 242, 186, 233, 179, 3, 173, 101, 12, 140, 45, 50, 110, 79, 26, 163, 246, 39, 119, 2, 0, 104, 27, 9, 158],seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137],seq![19nat, 27, 36, 45, 54, 62, 71, 80, 88, 96, 104, 112, 120, 128, 136, 145, 153, 161, 170, 178, 186, 193],0,21)) by (compute);
        literal_decodes(z,raw,positions,0,21); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),20,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137],position:193}));
        assert(m::field_value(seq![120u8, 156, 99, 216, 246, 242, 186, 233, 179, 3, 173, 101, 12, 140, 45, 50, 110, 79, 26, 163, 246, 39, 119, 2, 0, 104, 27, 9, 158],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 216, 246, 242, 186, 233, 179, 3, 173, 101, 12, 140, 45, 50, 110, 79, 26, 163, 246, 39, 119, 2, 0, 104, 27, 9, 158],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),20,32768,true) == (m::StoredEffect::End{out:seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137],position:193}));
        assert(m::MFinish(seq![120u8, 156, 99, 216, 246, 242, 186, 233, 179, 3, 173, 101, 12, 140, 45, 50, 110, 79, 26, 163, 246, 39, 119, 2, 0, 104, 27, 9, 158],193,seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137],20,false) == (m::StoredEffect::End{out:seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137],position:232})) by (compute);
        assert(m::Inflate(z,20,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,20,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 29) by (compute);
    }
    let ghost raw = seq![0u8, 182, 233, 215, 53, 230, 192, 133, 118, 0, 1, 132, 28, 70, 228, 129, 90, 191, 99, 137]; let mut output = [0xa5u8;20]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il1_1x1: 1 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0x0, 0x0, 0x0, 0x22, 0x0, 0x21];
    proof {
        let z = seq![120u8, 156, 99, 80, 0, 0, 0, 34, 0, 33]; let raw = seq![0u8, 32]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 80, 0, 0, 0, 34, 0, 33],seq![0u8, 32],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 32],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 0, 0, 0, 34, 0, 33],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 0, 0, 0, 34, 0, 33],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 32],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 0, 0, 0, 34, 0, 33],42,seq![0u8, 32],2,false) == (m::StoredEffect::End{out:seq![0u8, 32],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 32]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0xfc, 0xc0, 0x64, 0xc0, 0xec, 0xc1, 0xc2, 0x4, 0x0, 0xd, 0xc, 0x1, 0xf5];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 252, 192, 100, 192, 236, 193, 194, 4, 0, 13, 12, 1, 245]; let raw = seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 100, 107];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 252, 192, 100, 192, 236, 193, 194, 4, 0, 13, 12, 1, 245],seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 100, 107],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2],position:107}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 252, 192, 100, 192, 236, 193, 194, 4, 0, 13, 12, 1, 245],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 252, 192, 100, 192, 236, 193, 194, 4, 0, 13, 12, 1, 245],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2],position:107}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 252, 192, 100, 192, 236, 193, 194, 4, 0, 13, 12, 1, 245],107,seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2],10,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 240, 2, 48, 3, 72, 4, 2]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il1_7x5: 2 independently audited frozen streams; R=32.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x60, 0x7c, 0xc0, 0x64, 0xcf, 0xbc, 0x89, 0xe5, 0x35, 0x83, 0xa8, 0x9, 0x23, 0x97, 0x19, 0xd3, 0xfc, 0x5, 0xcc, 0xbe, 0x1b, 0x58, 0xee, 0x5, 0xb1, 0xfe, 0x66, 0xd0, 0xea, 0x9a, 0x5c, 0x0, 0x0, 0x83, 0xb9, 0x9, 0x7d];
    proof {
        let z = seq![120u8, 156, 99, 96, 96, 124, 192, 100, 207, 188, 137, 229, 53, 131, 168, 9, 35, 151, 25, 211, 252, 5, 204, 190, 27, 88, 238, 5, 177, 254, 102, 208, 234, 154, 92, 0, 0, 131, 185, 9, 125]; let raw = seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 102, 110, 118, 126, 134, 142, 150, 158, 167, 176, 184, 192, 201, 209, 218, 226, 234, 243, 251, 259, 267, 276, 284, 291];
        assert(literal_words(seq![120u8, 156, 99, 96, 96, 124, 192, 100, 207, 188, 137, 229, 53, 131, 168, 9, 35, 151, 25, 211, 252, 5, 204, 190, 27, 88, 238, 5, 177, 254, 102, 208, 234, 154, 92, 0, 0, 131, 185, 9, 125],seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 102, 110, 118, 126, 134, 142, 150, 158, 167, 176, 184, 192, 201, 209, 218, 226, 234, 243, 251, 259, 267, 276, 284, 291],0,33)) by (compute);
        literal_decodes(z,raw,positions,0,33); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),32,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112],position:291}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 96, 124, 192, 100, 207, 188, 137, 229, 53, 131, 168, 9, 35, 151, 25, 211, 252, 5, 204, 190, 27, 88, 238, 5, 177, 254, 102, 208, 234, 154, 92, 0, 0, 131, 185, 9, 125],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 96, 124, 192, 100, 207, 188, 137, 229, 53, 131, 168, 9, 35, 151, 25, 211, 252, 5, 204, 190, 27, 88, 238, 5, 177, 254, 102, 208, 234, 154, 92, 0, 0, 131, 185, 9, 125],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),32,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112],position:291}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 96, 124, 192, 100, 207, 188, 137, 229, 53, 131, 168, 9, 35, 151, 25, 211, 252, 5, 204, 190, 27, 88, 238, 5, 177, 254, 102, 208, 234, 154, 92, 0, 0, 131, 185, 9, 125],291,seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112],32,false) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112],position:328})) by (compute);
        assert(m::Inflate(z,32,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,32,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 41) by (compute);
    }
    let ghost raw = seq![0u8, 0, 1, 224, 2, 63, 3, 178, 4, 235, 0, 21, 52, 1, 10, 54, 2, 159, 160, 3, 77, 176, 4, 222, 82, 5, 251, 0, 42, 138, 147, 112]; let mut output = [0xa5u8;32]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il1_9x10: 1 frozen stored streams; R=70.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il1_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x46, 0x0, 0xb9, 0xff, 0x0, 0xa5, 0x1, 0xe7, 0x2, 0x90, 0x3, 0xb8, 0x4, 0x2b, 0x65, 0x0, 0xaf, 0x1, 0xe3, 0x2, 0xd0, 0x3, 0x9c, 0x1f, 0x2a, 0x4, 0xc2, 0x7a, 0x48, 0x0, 0x19, 0x4f, 0x1, 0x82, 0x8, 0x2, 0x72, 0xdc, 0x3, 0x35, 0x57, 0x4, 0x9c, 0xbb, 0x0, 0x3, 0x7c, 0x6c, 0x3b, 0x90, 0x1, 0x1d, 0xb2, 0xe2, 0xed, 0x2, 0x2, 0x97, 0xe0, 0x96, 0x6d, 0x30, 0x3, 0xa, 0x3c, 0x69, 0xf4, 0xa0, 0x4, 0x25, 0x6a, 0x56, 0x38, 0xef, 0x18, 0xb4, 0x17, 0xca];
    proof {
        let z = seq![120u8, 156, 1, 70, 0, 185, 255, 0, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239, 24, 180, 23, 202]; let raw = seq![0u8, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239];
        assert(z.subrange(7,77) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239, 24, 180, 23, 202],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239, 24, 180, 23, 202],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),70) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MOneBlock(z,16,Seq::empty(),70,32768,true) == (true,m::StoredEffect::End{out:raw,position:616}));
        assert(m::MBlk(z,16,Seq::empty(),70,32768,true) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MFinish(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239, 24, 180, 23, 202],616,seq![0u8, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239],70,false) == (m::StoredEffect::End{out:seq![0u8, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239],position:648})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,70,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,70,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 81) by (compute);
    }
    let ghost raw = seq![0u8, 165, 1, 231, 2, 144, 3, 184, 4, 43, 101, 0, 175, 1, 227, 2, 208, 3, 156, 31, 42, 4, 194, 122, 72, 0, 25, 79, 1, 130, 8, 2, 114, 220, 3, 53, 87, 4, 156, 187, 0, 3, 124, 108, 59, 144, 1, 29, 178, 226, 237, 2, 2, 151, 224, 150, 109, 48, 3, 10, 60, 105, 244, 160, 4, 37, 106, 86, 56, 239]; let mut output = [0xa5u8;70]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d4_il1_17x2: 1 independently audited frozen streams; R=23.
#[verifier::spinoff_prover]
fn intent_ct0_d4_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xa8, 0x7f, 0xc0, 0xf8, 0x82, 0x49, 0xd0, 0x99, 0xf9, 0xa1, 0x41, 0xd0, 0x5b, 0x96, 0x4f, 0x27, 0x66, 0xaa, 0x96, 0xad, 0x95, 0x5a, 0x7e, 0x5, 0x0, 0x6c, 0x7d, 0xa, 0x26];
    proof {
        let z = seq![120u8, 156, 99, 168, 127, 192, 248, 130, 73, 208, 153, 249, 161, 65, 208, 91, 150, 79, 39, 102, 170, 150, 173, 149, 90, 126, 5, 0, 108, 125, 10, 38]; let raw = seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212]; let positions = seq![19nat, 27, 35, 44, 52, 61, 69, 77, 85, 93, 102, 110, 118, 127, 135, 144, 153, 162, 170, 178, 187, 195, 204, 213, 220];
        assert(literal_words(seq![120u8, 156, 99, 168, 127, 192, 248, 130, 73, 208, 153, 249, 161, 65, 208, 91, 150, 79, 39, 102, 170, 150, 173, 149, 90, 126, 5, 0, 108, 125, 10, 38],seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212],seq![19nat, 27, 35, 44, 52, 61, 69, 77, 85, 93, 102, 110, 118, 127, 135, 144, 153, 162, 170, 178, 187, 195, 204, 213, 220],0,24)) by (compute);
        literal_decodes(z,raw,positions,0,24); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),23,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212],position:220}));
        assert(m::field_value(seq![120u8, 156, 99, 168, 127, 192, 248, 130, 73, 208, 153, 249, 161, 65, 208, 91, 150, 79, 39, 102, 170, 150, 173, 149, 90, 126, 5, 0, 108, 125, 10, 38],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 168, 127, 192, 248, 130, 73, 208, 153, 249, 161, 65, 208, 91, 150, 79, 39, 102, 170, 150, 173, 149, 90, 126, 5, 0, 108, 125, 10, 38],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),23,32768,true) == (m::StoredEffect::End{out:seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212],position:220}));
        assert(m::MFinish(seq![120u8, 156, 99, 168, 127, 192, 248, 130, 73, 208, 153, 249, 161, 65, 208, 91, 150, 79, 39, 102, 170, 150, 173, 149, 90, 126, 5, 0, 108, 125, 10, 38],220,seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212],23,false) == (m::StoredEffect::End{out:seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212],position:256})) by (compute);
        assert(m::Inflate(z,23,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,23,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 32) by (compute);
    }
    let ghost raw = seq![0u8, 127, 224, 1, 232, 2, 17, 67, 3, 225, 48, 82, 237, 4, 242, 200, 153, 37, 118, 173, 26, 167, 212]; let mut output = [0xa5u8;23]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il0_1x1: 1 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x8, 0x4, 0x0, 0x0, 0x53, 0x0, 0x52];
    proof {
        let z = seq![120u8, 156, 99, 8, 4, 0, 0, 83, 0, 82]; let raw = seq![0u8, 81]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 8, 4, 0, 0, 83, 0, 82],seq![0u8, 81],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 81],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 8, 4, 0, 0, 83, 0, 82],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 8, 4, 0, 0, 83, 0, 82],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 81],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 8, 4, 0, 0, 83, 0, 82],42,seq![0u8, 81],2,false) == (m::StoredEffect::End{out:seq![0u8, 81],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 81]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il0_2x3: 1 independently audited frozen streams; R=9.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xc8, 0x64, 0x66, 0xfc, 0x70, 0x93, 0x29, 0xac, 0x1e, 0x0, 0xc, 0xb1, 0x3, 0xe];
    proof {
        let z = seq![120u8, 156, 99, 200, 100, 102, 252, 112, 147, 41, 172, 30, 0, 12, 177, 3, 14]; let raw = seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127]; let positions = seq![19nat, 27, 35, 43, 51, 60, 69, 77, 85, 93, 100];
        assert(literal_words(seq![120u8, 156, 99, 200, 100, 102, 252, 112, 147, 41, 172, 30, 0, 12, 177, 3, 14],seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127],seq![19nat, 27, 35, 43, 51, 60, 69, 77, 85, 93, 100],0,10)) by (compute);
        literal_decodes(z,raw,positions,0,10); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),9,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127],position:100}));
        assert(m::field_value(seq![120u8, 156, 99, 200, 100, 102, 252, 112, 147, 41, 172, 30, 0, 12, 177, 3, 14],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 200, 100, 102, 252, 112, 147, 41, 172, 30, 0, 12, 177, 3, 14],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),9,32768,true) == (m::StoredEffect::End{out:seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127],position:100}));
        assert(m::MFinish(seq![120u8, 156, 99, 200, 100, 102, 252, 112, 147, 41, 172, 30, 0, 12, 177, 3, 14],100,seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127],9,false) == (m::StoredEffect::End{out:seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127],position:136})) by (compute);
        assert(m::Inflate(z,9,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,9,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 17) by (compute);
    }
    let ghost raw = seq![0u8, 105, 3, 1, 240, 217, 2, 86, 127]; let mut output = [0xa5u8;9]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il0_7x5: 2 frozen stored streams; R=40.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il0_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x28, 0x0, 0xd7, 0xff, 0x0, 0xb3, 0x25, 0xb5, 0x5c, 0xa8, 0xd, 0x8c, 0x1, 0xc3, 0x4e, 0x92, 0x1b, 0xfc, 0xdc, 0x5f, 0x2, 0xee, 0xce, 0x6c, 0x93, 0x19, 0x50, 0xcb, 0x3, 0x7f, 0x6e, 0xbb, 0xc8, 0xb0, 0xde, 0xae, 0x4, 0x85, 0xa4, 0x5a, 0x68, 0x34, 0x47, 0x62, 0x75, 0x5e, 0x12, 0x8d];
    proof {
        let z = seq![120u8, 156, 1, 40, 0, 215, 255, 0, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98, 117, 94, 18, 141]; let raw = seq![0u8, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98];
        assert(z.subrange(7,47) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98, 117, 94, 18, 141],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98, 117, 94, 18, 141],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),40) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MOneBlock(z,16,Seq::empty(),40,32768,true) == (true,m::StoredEffect::End{out:raw,position:376}));
        assert(m::MBlk(z,16,Seq::empty(),40,32768,true) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MFinish(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98, 117, 94, 18, 141],376,seq![0u8, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98],40,false) == (m::StoredEffect::End{out:seq![0u8, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98],position:408})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,40,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,40,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 51) by (compute);
    }
    let ghost raw = seq![0u8, 179, 37, 181, 92, 168, 13, 140, 1, 195, 78, 146, 27, 252, 220, 95, 2, 238, 206, 108, 147, 25, 80, 203, 3, 127, 110, 187, 200, 176, 222, 174, 4, 133, 164, 90, 104, 52, 71, 98]; let mut output = [0xa5u8;40]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il0_9x10: 1 frozen stored streams; R=100.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il0_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x64, 0x0, 0x9b, 0xff, 0x0, 0xc2, 0x31, 0x8e, 0x53, 0x55, 0xfc, 0xd, 0x1b, 0x8c, 0x1, 0x95, 0x0, 0x3b, 0x1d, 0xfe, 0x15, 0x2b, 0x3d, 0xd, 0x2, 0xd4, 0xee, 0xac, 0x1a, 0x88, 0x88, 0x25, 0x8, 0x1d, 0x3, 0xc2, 0xdf, 0xfd, 0x2a, 0x5a, 0x37, 0x2, 0xa2, 0x56, 0x4, 0x5c, 0x18, 0xce, 0xfb, 0x8d, 0x49, 0x5e, 0x54, 0x12, 0x0, 0xa4, 0x2e, 0x93, 0x3e, 0xb7, 0x28, 0x55, 0xf7, 0xaa, 0x1, 0x31, 0x78, 0xe0, 0x86, 0xdb, 0xd8, 0xc8, 0x6d, 0xe6, 0x2, 0xae, 0x72, 0x53, 0xb3, 0x68, 0x70, 0x3, 0x89, 0x65, 0x3, 0xb9, 0x5c, 0x72, 0x38, 0x8e, 0x11, 0x51, 0xc, 0xa8, 0x4, 0x70, 0xe0, 0x14, 0x1, 0x12, 0xe8, 0x84, 0x73, 0xa5, 0x9e, 0x62, 0x28, 0x11];
    proof {
        let z = seq![120u8, 156, 1, 100, 0, 155, 255, 0, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165, 158, 98, 40, 17]; let raw = seq![0u8, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165];
        assert(z.subrange(7,107) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165, 158, 98, 40, 17],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165, 158, 98, 40, 17],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),100) == (m::StoredEffect::End{out:raw,position:856}));
        assert(m::MOneBlock(z,16,Seq::empty(),100,32768,true) == (true,m::StoredEffect::End{out:raw,position:856}));
        assert(m::MBlk(z,16,Seq::empty(),100,32768,true) == (m::StoredEffect::End{out:raw,position:856}));
        assert(m::MFinish(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165, 158, 98, 40, 17],856,seq![0u8, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165],100,false) == (m::StoredEffect::End{out:seq![0u8, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165],position:888})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,100,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,100,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 111) by (compute);
    }
    let ghost raw = seq![0u8, 194, 49, 142, 83, 85, 252, 13, 27, 140, 1, 149, 0, 59, 29, 254, 21, 43, 61, 13, 2, 212, 238, 172, 26, 136, 136, 37, 8, 29, 3, 194, 223, 253, 42, 90, 55, 2, 162, 86, 4, 92, 24, 206, 251, 141, 73, 94, 84, 18, 0, 164, 46, 147, 62, 183, 40, 85, 247, 170, 1, 49, 120, 224, 134, 219, 216, 200, 109, 230, 2, 174, 114, 83, 179, 104, 112, 3, 137, 101, 3, 185, 92, 114, 56, 142, 17, 81, 12, 168, 4, 112, 224, 20, 1, 18, 232, 132, 115, 165]; let mut output = [0xa5u8;100]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il0_17x2: 1 frozen stored streams; R=36.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il0_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x24, 0x0, 0xdb, 0xff, 0x0, 0xbd, 0x47, 0xc8, 0x9e, 0x62, 0x41, 0x13, 0xcc, 0x36, 0xd4, 0xf8, 0xd4, 0xba, 0xb1, 0x54, 0xad, 0x7e, 0x1, 0x50, 0xe7, 0xad, 0x7e, 0x37, 0x2c, 0xa1, 0xa6, 0x3f, 0xdb, 0xec, 0x90, 0xcb, 0x83, 0x8c, 0x79, 0x1b, 0x54, 0xa5, 0x12, 0xbe];
    proof {
        let z = seq![120u8, 156, 1, 36, 0, 219, 255, 0, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27, 84, 165, 18, 190]; let raw = seq![0u8, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27];
        assert(z.subrange(7,43) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 36, 0, 219, 255, 0, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27, 84, 165, 18, 190],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 36, 0, 219, 255, 0, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27, 84, 165, 18, 190],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),36) == (m::StoredEffect::End{out:raw,position:344}));
        assert(m::MOneBlock(z,16,Seq::empty(),36,32768,true) == (true,m::StoredEffect::End{out:raw,position:344}));
        assert(m::MBlk(z,16,Seq::empty(),36,32768,true) == (m::StoredEffect::End{out:raw,position:344}));
        assert(m::MFinish(seq![120u8, 156, 1, 36, 0, 219, 255, 0, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27, 84, 165, 18, 190],344,seq![0u8, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27],36,false) == (m::StoredEffect::End{out:seq![0u8, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27],position:376})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,36,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,36,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 47) by (compute);
    }
    let ghost raw = seq![0u8, 189, 71, 200, 158, 98, 65, 19, 204, 54, 212, 248, 212, 186, 177, 84, 173, 126, 1, 80, 231, 173, 126, 55, 44, 161, 166, 63, 219, 236, 144, 203, 131, 140, 121, 27]; let mut output = [0xa5u8;36]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il1_1x1: 1 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x30, 0x4, 0x0, 0x0, 0x33, 0x0, 0x32];
    proof {
        let z = seq![120u8, 156, 99, 48, 4, 0, 0, 51, 0, 50]; let raw = seq![0u8, 49]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 48, 4, 0, 0, 51, 0, 50],seq![0u8, 49],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 49],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 48, 4, 0, 0, 51, 0, 50],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 48, 4, 0, 0, 51, 0, 50],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 49],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 48, 4, 0, 0, 51, 0, 50],42,seq![0u8, 49],2,false) == (m::StoredEffect::End{out:seq![0u8, 49],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 49]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il1_2x3: 1 independently audited frozen streams; R=11.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xa8, 0x65, 0x5c, 0xc3, 0x14, 0xca, 0x7c, 0x8e, 0xa5, 0xf7, 0x39, 0x0, 0x11, 0xb6, 0x3, 0xcb];
    proof {
        let z = seq![120u8, 156, 99, 168, 101, 92, 195, 20, 202, 124, 142, 165, 247, 57, 0, 17, 182, 3, 203]; let raw = seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 101, 110, 117];
        assert(literal_words(seq![120u8, 156, 99, 168, 101, 92, 195, 20, 202, 124, 142, 165, 247, 57, 0, 17, 182, 3, 203],seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 85, 93, 101, 110, 117],0,12)) by (compute);
        literal_decodes(z,raw,positions,0,12); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),11,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231],position:117}));
        assert(m::field_value(seq![120u8, 156, 99, 168, 101, 92, 195, 20, 202, 124, 142, 165, 247, 57, 0, 17, 182, 3, 203],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 168, 101, 92, 195, 20, 202, 124, 142, 165, 247, 57, 0, 17, 182, 3, 203],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),11,32768,true) == (m::StoredEffect::End{out:seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231],position:117}));
        assert(m::MFinish(seq![120u8, 156, 99, 168, 101, 92, 195, 20, 202, 124, 142, 165, 247, 57, 0, 17, 182, 3, 203],117,seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231],11,false) == (m::StoredEffect::End{out:seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231],position:152})) by (compute);
        assert(m::Inflate(z,11,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,11,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 19) by (compute);
    }
    let ghost raw = seq![0u8, 125, 1, 172, 2, 85, 3, 206, 4, 141, 231]; let mut output = [0xa5u8;11]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il1_7x5: 2 independently audited frozen streams; R=46.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x28, 0x61, 0x74, 0x64, 0xfa, 0xfa, 0x8c, 0x79, 0x57, 0x15, 0x8b, 0xe0, 0x54, 0x6, 0x7, 0x71, 0x37, 0x57, 0xc6, 0xb7, 0x47, 0xb9, 0x99, 0x78, 0x13, 0x3f, 0x33, 0x7, 0xf1, 0xf0, 0xb2, 0x28, 0x79, 0x2a, 0x4c, 0x3a, 0xb1, 0x68, 0x19, 0xc3, 0x57, 0x97, 0xd7, 0x31, 0x3d, 0xb3, 0x5c, 0x0, 0x53, 0x22, 0x10, 0x1];
    proof {
        let z = seq![120u8, 156, 99, 40, 97, 116, 100, 250, 250, 140, 121, 87, 21, 139, 224, 84, 6, 7, 113, 55, 87, 198, 183, 71, 185, 153, 120, 19, 63, 51, 7, 241, 240, 178, 40, 121, 42, 76, 58, 177, 104, 25, 195, 87, 151, 215, 49, 61, 179, 92, 0, 83, 34, 16, 1]; let raw = seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 77, 85, 94, 102, 110, 118, 127, 135, 143, 151, 159, 167, 175, 184, 193, 201, 209, 217, 225, 234, 242, 250, 258, 266, 274, 282, 290, 298, 307, 316, 325, 334, 342, 351, 359, 368, 376, 384, 393, 401, 408];
        assert(literal_words(seq![120u8, 156, 99, 40, 97, 116, 100, 250, 250, 140, 121, 87, 21, 139, 224, 84, 6, 7, 113, 55, 87, 198, 183, 71, 185, 153, 120, 19, 63, 51, 7, 241, 240, 178, 40, 121, 42, 76, 58, 177, 104, 25, 195, 87, 151, 215, 49, 61, 179, 92, 0, 83, 34, 16, 1],seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68],seq![19nat, 27, 35, 43, 51, 59, 68, 77, 85, 94, 102, 110, 118, 127, 135, 143, 151, 159, 167, 175, 184, 193, 201, 209, 217, 225, 234, 242, 250, 258, 266, 274, 282, 290, 298, 307, 316, 325, 334, 342, 351, 359, 368, 376, 384, 393, 401, 408],0,47)) by (compute);
        literal_decodes(z,raw,positions,0,47); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),46,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68],position:408}));
        assert(m::field_value(seq![120u8, 156, 99, 40, 97, 116, 100, 250, 250, 140, 121, 87, 21, 139, 224, 84, 6, 7, 113, 55, 87, 198, 183, 71, 185, 153, 120, 19, 63, 51, 7, 241, 240, 178, 40, 121, 42, 76, 58, 177, 104, 25, 195, 87, 151, 215, 49, 61, 179, 92, 0, 83, 34, 16, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 40, 97, 116, 100, 250, 250, 140, 121, 87, 21, 139, 224, 84, 6, 7, 113, 55, 87, 198, 183, 71, 185, 153, 120, 19, 63, 51, 7, 241, 240, 178, 40, 121, 42, 76, 58, 177, 104, 25, 195, 87, 151, 215, 49, 61, 179, 92, 0, 83, 34, 16, 1],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),46,32768,true) == (m::StoredEffect::End{out:seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68],position:408}));
        assert(m::MFinish(seq![120u8, 156, 99, 40, 97, 116, 100, 250, 250, 140, 121, 87, 21, 139, 224, 84, 6, 7, 113, 55, 87, 198, 183, 71, 185, 153, 120, 19, 63, 51, 7, 241, 240, 178, 40, 121, 42, 76, 58, 177, 104, 25, 195, 87, 151, 215, 49, 61, 179, 92, 0, 83, 34, 16, 1],408,seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68],46,false) == (m::StoredEffect::End{out:seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68],position:440})) by (compute);
        assert(m::Inflate(z,46,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,46,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 55) by (compute);
    }
    let ghost raw = seq![0u8, 116, 1, 65, 2, 245, 230, 3, 186, 122, 4, 17, 149, 0, 64, 23, 70, 69, 1, 237, 197, 11, 2, 13, 97, 243, 3, 82, 12, 13, 4, 34, 73, 32, 146, 200, 162, 166, 0, 245, 68, 235, 92, 140, 154, 68]; let mut output = [0xa5u8;46]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il1_9x10: 1 frozen stored streams; R=110.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il1_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x6e, 0x0, 0x91, 0xff, 0x0, 0x76, 0x8, 0x1, 0x48, 0x39, 0x2, 0x36, 0x3, 0x22, 0x4, 0xf, 0xf8, 0x5e, 0x0, 0xac, 0x88, 0x1, 0x85, 0xd, 0x2, 0xaa, 0x33, 0x3, 0x74, 0x10, 0x8b, 0x96, 0x3f, 0x4, 0xea, 0xa3, 0xba, 0x40, 0x44, 0x0, 0x9b, 0x98, 0x3d, 0x7c, 0x1, 0x4d, 0xbb, 0xd1, 0x79, 0x2, 0x47, 0x6b, 0x6e, 0x9b, 0x3, 0xbf, 0x70, 0xa4, 0xcc, 0x4, 0x56, 0xe5, 0x8, 0x22, 0x0, 0xf0, 0x6e, 0xfe, 0xc, 0x7b, 0x53, 0xaf, 0xc9, 0x47, 0x1, 0xd4, 0xde, 0xe, 0x44, 0x50, 0xbf, 0x53, 0xe5, 0x64, 0x2, 0xf8, 0xdc, 0xc3, 0xf4, 0xd6, 0x20, 0x4a, 0xdb, 0x27, 0x3, 0x56, 0xf1, 0xfd, 0x32, 0x54, 0xf2, 0xf6, 0x27, 0x8f, 0x4, 0x3e, 0x1d, 0x50, 0xfd, 0x41, 0x77, 0x88, 0xd2, 0x55, 0x82, 0x47, 0x2d, 0x1c];
    proof {
        let z = seq![120u8, 156, 1, 110, 0, 145, 255, 0, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85, 130, 71, 45, 28]; let raw = seq![0u8, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85];
        assert(z.subrange(7,117) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85, 130, 71, 45, 28],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85, 130, 71, 45, 28],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),110) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MOneBlock(z,16,Seq::empty(),110,32768,true) == (true,m::StoredEffect::End{out:raw,position:936}));
        assert(m::MBlk(z,16,Seq::empty(),110,32768,true) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MFinish(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85, 130, 71, 45, 28],936,seq![0u8, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85],110,false) == (m::StoredEffect::End{out:seq![0u8, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85],position:968})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,110,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,110,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 121) by (compute);
    }
    let ghost raw = seq![0u8, 118, 8, 1, 72, 57, 2, 54, 3, 34, 4, 15, 248, 94, 0, 172, 136, 1, 133, 13, 2, 170, 51, 3, 116, 16, 139, 150, 63, 4, 234, 163, 186, 64, 68, 0, 155, 152, 61, 124, 1, 77, 187, 209, 121, 2, 71, 107, 110, 155, 3, 191, 112, 164, 204, 4, 86, 229, 8, 34, 0, 240, 110, 254, 12, 123, 83, 175, 201, 71, 1, 212, 222, 14, 68, 80, 191, 83, 229, 100, 2, 248, 220, 195, 244, 214, 32, 74, 219, 39, 3, 86, 241, 253, 50, 84, 242, 246, 39, 143, 4, 62, 29, 80, 253, 65, 119, 136, 210, 85]; let mut output = [0xa5u8;110]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d8_il1_17x2: 1 frozen stored streams; R=39.
#[verifier::spinoff_prover]
fn intent_ct0_d8_il1_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x27, 0x0, 0xd8, 0xff, 0x0, 0x8b, 0xd2, 0xa7, 0x1, 0xd7, 0x58, 0x2, 0xcb, 0xae, 0x88, 0x8a, 0x3, 0xe6, 0x9a, 0xab, 0x54, 0xe6, 0x11, 0xc, 0x46, 0x4, 0xe7, 0xf2, 0x60, 0x91, 0x6, 0x37, 0xb5, 0xab, 0x17, 0xad, 0xe0, 0x45, 0x45, 0xbc, 0xf3, 0xa7, 0xfb, 0x6d, 0xa, 0x13, 0x77];
    proof {
        let z = seq![120u8, 156, 1, 39, 0, 216, 255, 0, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251, 109, 10, 19, 119]; let raw = seq![0u8, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251];
        assert(z.subrange(7,46) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 39, 0, 216, 255, 0, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251, 109, 10, 19, 119],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 39, 0, 216, 255, 0, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251, 109, 10, 19, 119],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),39) == (m::StoredEffect::End{out:raw,position:368}));
        assert(m::MOneBlock(z,16,Seq::empty(),39,32768,true) == (true,m::StoredEffect::End{out:raw,position:368}));
        assert(m::MBlk(z,16,Seq::empty(),39,32768,true) == (m::StoredEffect::End{out:raw,position:368}));
        assert(m::MFinish(seq![120u8, 156, 1, 39, 0, 216, 255, 0, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251, 109, 10, 19, 119],368,seq![0u8, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251],39,false) == (m::StoredEffect::End{out:seq![0u8, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251],position:400})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,39,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,39,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 50) by (compute);
    }
    let ghost raw = seq![0u8, 139, 210, 167, 1, 215, 88, 2, 203, 174, 136, 138, 3, 230, 154, 171, 84, 230, 17, 12, 70, 4, 231, 242, 96, 145, 6, 55, 181, 171, 23, 173, 224, 69, 69, 188, 243, 167, 251]; let mut output = [0xa5u8;39]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il0_1x1: 1 independently audited frozen streams; R=3.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xb8, 0xb4, 0x19, 0x0, 0x2, 0x5a, 0x1, 0x86];
    proof {
        let z = seq![120u8, 156, 99, 184, 180, 25, 0, 2, 90, 1, 134]; let raw = seq![0u8, 210, 179]; let positions = seq![19nat, 27, 36, 45, 52];
        assert(literal_words(seq![120u8, 156, 99, 184, 180, 25, 0, 2, 90, 1, 134],seq![0u8, 210, 179],seq![19nat, 27, 36, 45, 52],0,4)) by (compute);
        literal_decodes(z,raw,positions,0,4); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),3,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 210, 179],position:52}));
        assert(m::field_value(seq![120u8, 156, 99, 184, 180, 25, 0, 2, 90, 1, 134],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 184, 180, 25, 0, 2, 90, 1, 134],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),3,32768,true) == (m::StoredEffect::End{out:seq![0u8, 210, 179],position:52}));
        assert(m::MFinish(seq![120u8, 156, 99, 184, 180, 25, 0, 2, 90, 1, 134],52,seq![0u8, 210, 179],3,false) == (m::StoredEffect::End{out:seq![0u8, 210, 179],position:88})) by (compute);
        assert(m::Inflate(z,3,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,3,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 11) by (compute);
    }
    let ghost raw = seq![0u8, 210, 179]; let mut output = [0xa5u8;3]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il0_2x3: 1 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x58, 0x73, 0xea, 0x91, 0x2d, 0x63, 0xf2, 0xf3, 0x99, 0x51, 0x4c, 0x37, 0x1d, 0xad, 0xa3, 0x1, 0x36, 0xfa, 0x6, 0x86];
    proof {
        let z = seq![120u8, 156, 99, 88, 115, 234, 145, 45, 99, 242, 243, 153, 81, 76, 55, 29, 173, 163, 1, 54, 250, 6, 134]; let raw = seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91]; let positions = seq![19nat, 27, 36, 45, 54, 62, 70, 78, 87, 96, 104, 112, 121, 129, 137, 145, 152];
        assert(literal_words(seq![120u8, 156, 99, 88, 115, 234, 145, 45, 99, 242, 243, 153, 81, 76, 55, 29, 173, 163, 1, 54, 250, 6, 134],seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91],seq![19nat, 27, 36, 45, 54, 62, 70, 78, 87, 96, 104, 112, 121, 129, 137, 145, 152],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91],position:152}));
        assert(m::field_value(seq![120u8, 156, 99, 88, 115, 234, 145, 45, 99, 242, 243, 153, 81, 76, 55, 29, 173, 163, 1, 54, 250, 6, 134],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 88, 115, 234, 145, 45, 99, 242, 243, 153, 81, 76, 55, 29, 173, 163, 1, 54, 250, 6, 134],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91],position:152}));
        assert(m::MFinish(seq![120u8, 156, 99, 88, 115, 234, 145, 45, 99, 242, 243, 153, 81, 76, 55, 29, 173, 163, 1, 54, 250, 6, 134],152,seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91],15,false) == (m::StoredEffect::End{out:seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91],position:184})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 23) by (compute);
    }
    let ghost raw = seq![0u8, 172, 202, 226, 61, 1, 99, 231, 153, 90, 2, 217, 65, 59, 91]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il0_7x5: 2 frozen stored streams; R=75.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il0_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x4b, 0x0, 0xb4, 0xff, 0x0, 0xd7, 0xd0, 0x81, 0x93, 0x75, 0x70, 0xcb, 0x45, 0x4b, 0x0, 0x47, 0x5, 0xa4, 0x14, 0x1, 0x30, 0xcd, 0x87, 0xb0, 0x5a, 0x3d, 0xf5, 0xda, 0xf, 0xac, 0x88, 0xd1, 0x9, 0xc9, 0x2, 0xfe, 0x37, 0x5, 0xac, 0x5f, 0xaf, 0x6, 0xbe, 0x8a, 0xe4, 0x95, 0x57, 0x8, 0x5e, 0x3, 0x2, 0x3f, 0x5a, 0xa5, 0x4d, 0xfe, 0x5b, 0xc7, 0xf6, 0xbf, 0x25, 0x45, 0xbf, 0xa2, 0x4, 0x1a, 0x9c, 0x7c, 0x6, 0xe7, 0x3b, 0x98, 0xb9, 0x94, 0xca, 0x14, 0x57, 0x3e, 0x5e, 0xe8, 0x80, 0x21, 0x3f];
    proof {
        let z = seq![120u8, 156, 1, 75, 0, 180, 255, 0, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94, 232, 128, 33, 63]; let raw = seq![0u8, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94];
        assert(z.subrange(7,82) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94, 232, 128, 33, 63],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94, 232, 128, 33, 63],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),75) == (m::StoredEffect::End{out:raw,position:656}));
        assert(m::MOneBlock(z,16,Seq::empty(),75,32768,true) == (true,m::StoredEffect::End{out:raw,position:656}));
        assert(m::MBlk(z,16,Seq::empty(),75,32768,true) == (m::StoredEffect::End{out:raw,position:656}));
        assert(m::MFinish(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94, 232, 128, 33, 63],656,seq![0u8, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94],75,false) == (m::StoredEffect::End{out:seq![0u8, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94],position:688})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,75,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,75,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 86) by (compute);
    }
    let ghost raw = seq![0u8, 215, 208, 129, 147, 117, 112, 203, 69, 75, 0, 71, 5, 164, 20, 1, 48, 205, 135, 176, 90, 61, 245, 218, 15, 172, 136, 209, 9, 201, 2, 254, 55, 5, 172, 95, 175, 6, 190, 138, 228, 149, 87, 8, 94, 3, 2, 63, 90, 165, 77, 254, 91, 199, 246, 191, 37, 69, 191, 162, 4, 26, 156, 124, 6, 231, 59, 152, 185, 148, 202, 20, 87, 62, 94]; let mut output = [0xa5u8;75]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il0_17x2: 1 frozen stored streams; R=70.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il0_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x46, 0x0, 0xb9, 0xff, 0x0, 0x7a, 0x20, 0x64, 0x29, 0x5a, 0xdb, 0x6d, 0x76, 0xc4, 0xa4, 0x45, 0xcc, 0x64, 0x26, 0x16, 0xd1, 0x3d, 0x35, 0xd6, 0x5a, 0xd5, 0xb7, 0xb6, 0x94, 0xd3, 0x84, 0x19, 0x4a, 0xa4, 0x13, 0x39, 0xac, 0x82, 0x11, 0x1, 0xdb, 0xbc, 0xf5, 0x24, 0x2f, 0x6f, 0x6d, 0x5e, 0x3a, 0x82, 0xba, 0x75, 0xdc, 0xf8, 0xfe, 0x80, 0x9c, 0xf7, 0xdc, 0xfc, 0xb3, 0xe, 0x78, 0xb9, 0xac, 0x2, 0xbe, 0x35, 0x5a, 0xec, 0x82, 0x70, 0x59, 0x52, 0x8e, 0x6f, 0x22, 0xc3];
    proof {
        let z = seq![120u8, 156, 1, 70, 0, 185, 255, 0, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82, 142, 111, 34, 195]; let raw = seq![0u8, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82];
        assert(z.subrange(7,77) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82, 142, 111, 34, 195],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82, 142, 111, 34, 195],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),70) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MOneBlock(z,16,Seq::empty(),70,32768,true) == (true,m::StoredEffect::End{out:raw,position:616}));
        assert(m::MBlk(z,16,Seq::empty(),70,32768,true) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MFinish(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82, 142, 111, 34, 195],616,seq![0u8, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82],70,false) == (m::StoredEffect::End{out:seq![0u8, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82],position:648})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,70,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,70,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 81) by (compute);
    }
    let ghost raw = seq![0u8, 122, 32, 100, 41, 90, 219, 109, 118, 196, 164, 69, 204, 100, 38, 22, 209, 61, 53, 214, 90, 213, 183, 182, 148, 211, 132, 25, 74, 164, 19, 57, 172, 130, 17, 1, 219, 188, 245, 36, 47, 111, 109, 94, 58, 130, 186, 117, 220, 248, 254, 128, 156, 247, 220, 252, 179, 14, 120, 185, 172, 2, 190, 53, 90, 236, 130, 112, 89, 82]; let mut output = [0xa5u8;70]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il1_1x1: 1 independently audited frozen streams; R=3.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0xb1, 0x3, 0x0, 0x1, 0x49, 0x0, 0xc3];
    proof {
        let z = seq![120u8, 156, 99, 104, 177, 3, 0, 1, 73, 0, 195]; let raw = seq![0u8, 132, 62]; let positions = seq![19nat, 27, 35, 43, 50];
        assert(literal_words(seq![120u8, 156, 99, 104, 177, 3, 0, 1, 73, 0, 195],seq![0u8, 132, 62],seq![19nat, 27, 35, 43, 50],0,4)) by (compute);
        literal_decodes(z,raw,positions,0,4); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),3,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 132, 62],position:50}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 177, 3, 0, 1, 73, 0, 195],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 177, 3, 0, 1, 73, 0, 195],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),3,32768,true) == (m::StoredEffect::End{out:seq![0u8, 132, 62],position:50}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 177, 3, 0, 1, 73, 0, 195],50,seq![0u8, 132, 62],3,false) == (m::StoredEffect::End{out:seq![0u8, 132, 62],position:88})) by (compute);
        assert(m::Inflate(z,3,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,3,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 11) by (compute);
    }
    let ghost raw = seq![0u8, 132, 62]; let mut output = [0xa5u8;3]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il1_2x3: 1 independently audited frozen streams; R=17.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0xd9, 0xc6, 0xa8, 0x39, 0x8d, 0xc9, 0x6e, 0x1b, 0xf3, 0x25, 0x67, 0x96, 0x6, 0x31, 0x5d, 0x25, 0x0, 0x29, 0x48, 0x4, 0x92];
    proof {
        let z = seq![120u8, 156, 99, 80, 217, 198, 168, 57, 141, 201, 110, 27, 243, 37, 103, 150, 6, 49, 93, 37, 0, 41, 72, 4, 146]; let raw = seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34]; let positions = seq![19nat, 27, 35, 44, 52, 60, 69, 77, 85, 94, 102, 111, 119, 127, 135, 143, 151, 159, 166];
        assert(literal_words(seq![120u8, 156, 99, 80, 217, 198, 168, 57, 141, 201, 110, 27, 243, 37, 103, 150, 6, 49, 93, 37, 0, 41, 72, 4, 146],seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34],seq![19nat, 27, 35, 44, 52, 60, 69, 77, 85, 94, 102, 111, 119, 127, 135, 143, 151, 159, 166],0,18)) by (compute);
        literal_decodes(z,raw,positions,0,18); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),17,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34],position:166}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 217, 198, 168, 57, 141, 201, 110, 27, 243, 37, 103, 150, 6, 49, 93, 37, 0, 41, 72, 4, 146],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 217, 198, 168, 57, 141, 201, 110, 27, 243, 37, 103, 150, 6, 49, 93, 37, 0, 41, 72, 4, 146],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),17,32768,true) == (m::StoredEffect::End{out:seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34],position:166}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 217, 198, 168, 57, 141, 201, 110, 27, 243, 37, 103, 150, 6, 49, 93, 37, 0, 41, 72, 4, 146],166,seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34],17,false) == (m::StoredEffect::End{out:seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34],position:200})) by (compute);
        assert(m::Inflate(z,17,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,17,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 25) by (compute);
    }
    let ghost raw = seq![0u8, 36, 182, 1, 41, 150, 2, 62, 182, 3, 210, 67, 4, 128, 22, 45, 34]; let mut output = [0xa5u8;17]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il1_7x5: 2 frozen stored streams; R=81.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il1_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x51, 0x0, 0xae, 0xff, 0x0, 0x99, 0x6c, 0x1, 0x6c, 0x6d, 0x2, 0x13, 0xd3, 0xee, 0xd5, 0x3, 0x73, 0x41, 0x35, 0xe5, 0x4, 0x7c, 0xb4, 0x72, 0x25, 0x0, 0x8b, 0x44, 0xd, 0x1, 0x33, 0xb6, 0x55, 0xba, 0x1, 0x27, 0x4f, 0xc4, 0x20, 0x40, 0x9d, 0x2, 0xd9, 0xac, 0x32, 0x6d, 0x1b, 0x5d, 0x3, 0xb5, 0xa1, 0x76, 0x45, 0x39, 0xb6, 0x4, 0x91, 0x1e, 0x7e, 0x3, 0xc0, 0xf2, 0x79, 0xe5, 0x80, 0xc7, 0x38, 0xc6, 0xd2, 0x54, 0x0, 0x6a, 0xac, 0x95, 0x97, 0x1e, 0x67, 0x2a, 0x6c, 0xb, 0x61, 0x66, 0x7d, 0x5a, 0xed, 0xe8, 0x59, 0x20, 0x77];
    proof {
        let z = seq![120u8, 156, 1, 81, 0, 174, 255, 0, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237, 232, 89, 32, 119]; let raw = seq![0u8, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237];
        assert(z.subrange(7,88) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237, 232, 89, 32, 119],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237, 232, 89, 32, 119],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),81) == (m::StoredEffect::End{out:raw,position:704}));
        assert(m::MOneBlock(z,16,Seq::empty(),81,32768,true) == (true,m::StoredEffect::End{out:raw,position:704}));
        assert(m::MBlk(z,16,Seq::empty(),81,32768,true) == (m::StoredEffect::End{out:raw,position:704}));
        assert(m::MFinish(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237, 232, 89, 32, 119],704,seq![0u8, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237],81,false) == (m::StoredEffect::End{out:seq![0u8, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237],position:736})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,81,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,81,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 92) by (compute);
    }
    let ghost raw = seq![0u8, 153, 108, 1, 108, 109, 2, 19, 211, 238, 213, 3, 115, 65, 53, 229, 4, 124, 180, 114, 37, 0, 139, 68, 13, 1, 51, 182, 85, 186, 1, 39, 79, 196, 32, 64, 157, 2, 217, 172, 50, 109, 27, 93, 3, 181, 161, 118, 69, 57, 182, 4, 145, 30, 126, 3, 192, 242, 121, 229, 128, 199, 56, 198, 210, 84, 0, 106, 172, 149, 151, 30, 103, 42, 108, 11, 97, 102, 125, 90, 237]; let mut output = [0xa5u8;81]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct0_d16_il1_17x2: 1 frozen stored streams; R=73.
#[verifier::spinoff_prover]
fn intent_ct0_d16_il1_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x49, 0x0, 0xb6, 0xff, 0x0, 0xb2, 0x18, 0x1, 0xd9, 0x4e, 0x7c, 0x1, 0xb0, 0x61, 0xb4, 0x30, 0x2, 0x54, 0xa, 0x6, 0xbf, 0x77, 0xdc, 0xf6, 0xc8, 0x3, 0xb0, 0x2a, 0x93, 0x7c, 0x28, 0x6a, 0x8d, 0x68, 0x2, 0x43, 0x4f, 0xdf, 0xf8, 0x16, 0xec, 0xe3, 0x4, 0xda, 0x2e, 0xa5, 0x90, 0xa2, 0xbc, 0xa7, 0x2f, 0xd8, 0x1e, 0x4a, 0xae, 0x64, 0x6, 0xc7, 0x4e, 0x69, 0x4e, 0x37, 0x7c, 0xa6, 0x79, 0xb7, 0xf7, 0x2e, 0x2, 0x8b, 0x5, 0xbf, 0x23, 0x63, 0xa6, 0x8c, 0x29, 0x7a, 0x3e, 0x1f, 0xd7];
    proof {
        let z = seq![120u8, 156, 1, 73, 0, 182, 255, 0, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41, 122, 62, 31, 215]; let raw = seq![0u8, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41];
        assert(z.subrange(7,80) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41, 122, 62, 31, 215],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41, 122, 62, 31, 215],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),73) == (m::StoredEffect::End{out:raw,position:640}));
        assert(m::MOneBlock(z,16,Seq::empty(),73,32768,true) == (true,m::StoredEffect::End{out:raw,position:640}));
        assert(m::MBlk(z,16,Seq::empty(),73,32768,true) == (m::StoredEffect::End{out:raw,position:640}));
        assert(m::MFinish(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41, 122, 62, 31, 215],640,seq![0u8, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41],73,false) == (m::StoredEffect::End{out:seq![0u8, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41],position:672})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,73,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,73,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 84) by (compute);
    }
    let ghost raw = seq![0u8, 178, 24, 1, 217, 78, 124, 1, 176, 97, 180, 48, 2, 84, 10, 6, 191, 119, 220, 246, 200, 3, 176, 42, 147, 124, 40, 106, 141, 104, 2, 67, 79, 223, 248, 22, 236, 227, 4, 218, 46, 165, 144, 162, 188, 167, 47, 216, 30, 74, 174, 100, 6, 199, 78, 105, 78, 55, 124, 166, 121, 183, 247, 46, 2, 139, 5, 191, 35, 99, 166, 140, 41]; let mut output = [0xa5u8;73]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il0_1x1: 1 independently audited frozen streams; R=4.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xc8, 0x60, 0x7e, 0x9, 0x0, 0x2, 0x2b, 0x1, 0x55];
    proof {
        let z = seq![120u8, 156, 99, 200, 96, 126, 9, 0, 2, 43, 1, 85]; let raw = seq![0u8, 104, 3, 233]; let positions = seq![19nat, 27, 35, 43, 52, 59];
        assert(literal_words(seq![120u8, 156, 99, 200, 96, 126, 9, 0, 2, 43, 1, 85],seq![0u8, 104, 3, 233],seq![19nat, 27, 35, 43, 52, 59],0,5)) by (compute);
        literal_decodes(z,raw,positions,0,5); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),4,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 104, 3, 233],position:59}));
        assert(m::field_value(seq![120u8, 156, 99, 200, 96, 126, 9, 0, 2, 43, 1, 85],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 200, 96, 126, 9, 0, 2, 43, 1, 85],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::End{out:seq![0u8, 104, 3, 233],position:59}));
        assert(m::MFinish(seq![120u8, 156, 99, 200, 96, 126, 9, 0, 2, 43, 1, 85],59,seq![0u8, 104, 3, 233],4,false) == (m::StoredEffect::End{out:seq![0u8, 104, 3, 233],position:96})) by (compute);
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,4,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 12) by (compute);
    }
    let ghost raw = seq![0u8, 104, 3, 233]; let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il0_2x3: 1 independently audited frozen streams; R=21.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf0, 0x59, 0x69, 0x98, 0xfe, 0xb6, 0x8b, 0x31, 0xbc, 0xfb, 0xc0, 0xdf, 0xdc, 0x6d, 0x4c, 0xb, 0x1f, 0x9f, 0x16, 0x31, 0x59, 0x8, 0x0, 0x67, 0x17, 0xa, 0x2];
    proof {
        let z = seq![120u8, 156, 99, 240, 89, 105, 152, 254, 182, 139, 49, 188, 251, 192, 223, 220, 109, 76, 11, 31, 159, 22, 49, 89, 8, 0, 103, 23, 10, 2]; let raw = seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161]; let positions = seq![19nat, 27, 35, 44, 52, 60, 69, 77, 85, 93, 101, 110, 119, 127, 136, 144, 153, 162, 171, 179, 187, 196, 203];
        assert(literal_words(seq![120u8, 156, 99, 240, 89, 105, 152, 254, 182, 139, 49, 188, 251, 192, 223, 220, 109, 76, 11, 31, 159, 22, 49, 89, 8, 0, 103, 23, 10, 2],seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161],seq![19nat, 27, 35, 44, 52, 60, 69, 77, 85, 93, 101, 110, 119, 127, 136, 144, 153, 162, 171, 179, 187, 196, 203],0,22)) by (compute);
        literal_decodes(z,raw,positions,0,22); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),21,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161],position:203}));
        assert(m::field_value(seq![120u8, 156, 99, 240, 89, 105, 152, 254, 182, 139, 49, 188, 251, 192, 223, 220, 109, 76, 11, 31, 159, 22, 49, 89, 8, 0, 103, 23, 10, 2],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 240, 89, 105, 152, 254, 182, 139, 49, 188, 251, 192, 223, 220, 109, 76, 11, 31, 159, 22, 49, 89, 8, 0, 103, 23, 10, 2],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),21,32768,true) == (m::StoredEffect::End{out:seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161],position:203}));
        assert(m::MFinish(seq![120u8, 156, 99, 240, 89, 105, 152, 254, 182, 139, 49, 188, 251, 192, 223, 220, 109, 76, 11, 31, 159, 22, 49, 89, 8, 0, 103, 23, 10, 2],203,seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161],21,false) == (m::StoredEffect::End{out:seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161],position:240})) by (compute);
        assert(m::Inflate(z,21,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,21,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8, 76, 169, 49, 103, 237, 138, 1, 87, 139, 192, 253, 109, 182, 2, 161, 227, 203, 20, 52, 161]; let mut output = [0xa5u8;21]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il0_7x5: 2 frozen stored streams; R=110.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il0_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x6e, 0x0, 0x91, 0xff, 0x0, 0xb6, 0x5b, 0x15, 0x2d, 0x5d, 0xdc, 0x9f, 0x3c, 0xff, 0xb5, 0xb8, 0x32, 0xb4, 0x48, 0x39, 0x7d, 0x59, 0xca, 0xa1, 0xbe, 0x67, 0x1, 0xcc, 0x6a, 0x98, 0xc0, 0xc9, 0x4b, 0x9b, 0xdd, 0x27, 0xcd, 0xef, 0xb3, 0xd, 0x34, 0x19, 0x67, 0x54, 0xf4, 0xe3, 0x53, 0xd1, 0x2, 0x4, 0x44, 0x21, 0xed, 0xbc, 0x99, 0xcc, 0x32, 0x29, 0x8d, 0xfd, 0xa5, 0x72, 0xf8, 0xf, 0xc2, 0x12, 0x25, 0x9c, 0xcf, 0x59, 0x3, 0x62, 0x52, 0xd2, 0x3b, 0x1d, 0xd8, 0xe5, 0x61, 0x73, 0xbd, 0x55, 0x5d, 0x6d, 0x61, 0x28, 0x6b, 0x8a, 0x56, 0xd5, 0xfb, 0xf3, 0x4, 0x94, 0x14, 0x93, 0xb, 0x49, 0xeb, 0x43, 0x88, 0x30, 0x6c, 0x86, 0x89, 0x9, 0x47, 0x4e, 0x8a, 0xdc, 0xed, 0x88, 0xb2, 0x3b, 0x90, 0x42, 0x34, 0xda];
    proof {
        let z = seq![120u8, 156, 1, 110, 0, 145, 255, 0, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59, 144, 66, 52, 218]; let raw = seq![0u8, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59];
        assert(z.subrange(7,117) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59, 144, 66, 52, 218],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59, 144, 66, 52, 218],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),110) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MOneBlock(z,16,Seq::empty(),110,32768,true) == (true,m::StoredEffect::End{out:raw,position:936}));
        assert(m::MBlk(z,16,Seq::empty(),110,32768,true) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MFinish(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59, 144, 66, 52, 218],936,seq![0u8, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59],110,false) == (m::StoredEffect::End{out:seq![0u8, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59],position:968})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,110,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,110,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 121) by (compute);
    }
    let ghost raw = seq![0u8, 182, 91, 21, 45, 93, 220, 159, 60, 255, 181, 184, 50, 180, 72, 57, 125, 89, 202, 161, 190, 103, 1, 204, 106, 152, 192, 201, 75, 155, 221, 39, 205, 239, 179, 13, 52, 25, 103, 84, 244, 227, 83, 209, 2, 4, 68, 33, 237, 188, 153, 204, 50, 41, 141, 253, 165, 114, 248, 15, 194, 18, 37, 156, 207, 89, 3, 98, 82, 210, 59, 29, 216, 229, 97, 115, 189, 85, 93, 109, 97, 40, 107, 138, 86, 213, 251, 243, 4, 148, 20, 147, 11, 73, 235, 67, 136, 48, 108, 134, 137, 9, 71, 78, 138, 220, 237, 136, 178, 59]; let mut output = [0xa5u8;110]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il0_17x2: 1 frozen stored streams; R=104.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il0_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x68, 0x0, 0x97, 0xff, 0x0, 0xce, 0x90, 0x58, 0xbb, 0x8b, 0xcb, 0x55, 0x67, 0x1a, 0xc3, 0xf4, 0x1f, 0x74, 0x3f, 0x61, 0x45, 0xfe, 0xf4, 0x92, 0x72, 0x5d, 0xfc, 0x1e, 0x3b, 0x10, 0x25, 0xb7, 0xf3, 0x46, 0xf8, 0x66, 0x13, 0xfd, 0xcb, 0x3d, 0x85, 0x66, 0x36, 0x86, 0xc4, 0xfa, 0xb7, 0x51, 0xab, 0x90, 0x89, 0x66, 0x55, 0x8f, 0x8b, 0x35, 0x1, 0xb8, 0x8f, 0x54, 0x19, 0xa3, 0x19, 0x33, 0x46, 0xc2, 0xc8, 0xe9, 0x54, 0xf3, 0x8f, 0x8, 0x5e, 0xc9, 0xfc, 0x14, 0x58, 0x15, 0x5c, 0x8d, 0x87, 0xf6, 0x33, 0x2, 0x26, 0x86, 0xfc, 0x77, 0xb8, 0xe9, 0x57, 0x90, 0x7b, 0x33, 0x9b, 0x5a, 0xe5, 0xdd, 0xb5, 0x17, 0x4a, 0xc2, 0xb0, 0xd8, 0xf, 0xb4, 0xb7, 0x24, 0x90, 0x6e, 0x33, 0xcf];
    proof {
        let z = seq![120u8, 156, 1, 104, 0, 151, 255, 0, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36, 144, 110, 51, 207]; let raw = seq![0u8, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36];
        assert(z.subrange(7,111) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 104, 0, 151, 255, 0, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36, 144, 110, 51, 207],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 104, 0, 151, 255, 0, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36, 144, 110, 51, 207],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),104) == (m::StoredEffect::End{out:raw,position:888}));
        assert(m::MOneBlock(z,16,Seq::empty(),104,32768,true) == (true,m::StoredEffect::End{out:raw,position:888}));
        assert(m::MBlk(z,16,Seq::empty(),104,32768,true) == (m::StoredEffect::End{out:raw,position:888}));
        assert(m::MFinish(seq![120u8, 156, 1, 104, 0, 151, 255, 0, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36, 144, 110, 51, 207],888,seq![0u8, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36],104,false) == (m::StoredEffect::End{out:seq![0u8, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36],position:920})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,104,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,104,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 115) by (compute);
    }
    let ghost raw = seq![0u8, 206, 144, 88, 187, 139, 203, 85, 103, 26, 195, 244, 31, 116, 63, 97, 69, 254, 244, 146, 114, 93, 252, 30, 59, 16, 37, 183, 243, 70, 248, 102, 19, 253, 203, 61, 133, 102, 54, 134, 196, 250, 183, 81, 171, 144, 137, 102, 85, 143, 139, 53, 1, 184, 143, 84, 25, 163, 25, 51, 70, 194, 200, 233, 84, 243, 143, 8, 94, 201, 252, 20, 88, 21, 92, 141, 135, 246, 51, 2, 38, 134, 252, 119, 184, 233, 87, 144, 123, 51, 155, 90, 229, 221, 181, 23, 74, 194, 176, 216, 15, 180, 183, 36]; let mut output = [0xa5u8;104]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il1_1x1: 1 independently audited frozen streams; R=4.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xd0, 0xf, 0x56, 0x7, 0x0, 0x1, 0x5e, 0x0, 0xaa];
    proof {
        let z = seq![120u8, 156, 99, 208, 15, 86, 7, 0, 1, 94, 0, 170]; let raw = seq![0u8, 47, 83, 39]; let positions = seq![19nat, 27, 35, 43, 51, 58];
        assert(literal_words(seq![120u8, 156, 99, 208, 15, 86, 7, 0, 1, 94, 0, 170],seq![0u8, 47, 83, 39],seq![19nat, 27, 35, 43, 51, 58],0,5)) by (compute);
        literal_decodes(z,raw,positions,0,5); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),4,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 47, 83, 39],position:58}));
        assert(m::field_value(seq![120u8, 156, 99, 208, 15, 86, 7, 0, 1, 94, 0, 170],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 208, 15, 86, 7, 0, 1, 94, 0, 170],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::End{out:seq![0u8, 47, 83, 39],position:58}));
        assert(m::MFinish(seq![120u8, 156, 99, 208, 15, 86, 7, 0, 1, 94, 0, 170],58,seq![0u8, 47, 83, 39],4,false) == (m::StoredEffect::End{out:seq![0u8, 47, 83, 39],position:96})) by (compute);
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,4,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 12) by (compute);
    }
    let ghost raw = seq![0u8, 47, 83, 39]; let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il1_2x3: 1 independently audited frozen streams; R=23.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0xb6, 0x36, 0x64, 0xdc, 0x25, 0xb5, 0x82, 0xa9, 0xb7, 0x33, 0x8b, 0x39, 0x55, 0x69, 0xa, 0xcb, 0xe7, 0x13, 0xbd, 0x33, 0x36, 0xb6, 0x0, 0x0, 0x52, 0x63, 0x8, 0xc6];
    proof {
        let z = seq![120u8, 156, 99, 80, 182, 54, 100, 220, 37, 181, 130, 169, 183, 51, 139, 57, 85, 105, 10, 203, 231, 19, 189, 51, 54, 182, 0, 0, 82, 99, 8, 198]; let raw = seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 85, 93, 101, 109, 117, 125, 133, 141, 150, 158, 167, 176, 184, 193, 202, 210, 217];
        assert(literal_words(seq![120u8, 156, 99, 80, 182, 54, 100, 220, 37, 181, 130, 169, 183, 51, 139, 57, 85, 105, 10, 203, 231, 19, 189, 51, 54, 182, 0, 0, 82, 99, 8, 198],seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 85, 93, 101, 109, 117, 125, 133, 141, 150, 158, 167, 176, 184, 193, 202, 210, 217],0,24)) by (compute);
        literal_decodes(z,raw,positions,0,24); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),23,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132],position:217}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 182, 54, 100, 220, 37, 181, 130, 169, 183, 51, 139, 57, 85, 105, 10, 203, 231, 19, 189, 51, 54, 182, 0, 0, 82, 99, 8, 198],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 182, 54, 100, 220, 37, 181, 130, 169, 183, 51, 139, 57, 85, 105, 10, 203, 231, 19, 189, 51, 54, 182, 0, 0, 82, 99, 8, 198],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),23,32768,true) == (m::StoredEffect::End{out:seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132],position:217}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 182, 54, 100, 220, 37, 181, 130, 169, 183, 51, 139, 57, 85, 105, 10, 203, 231, 19, 189, 51, 54, 182, 0, 0, 82, 99, 8, 198],217,seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132],23,false) == (m::StoredEffect::End{out:seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132],position:256})) by (compute);
        assert(m::Inflate(z,23,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,23,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 32) by (compute);
    }
    let ghost raw = seq![0u8, 35, 59, 49, 1, 186, 26, 168, 2, 141, 137, 106, 3, 101, 34, 148, 4, 243, 200, 141, 152, 177, 132]; let mut output = [0xa5u8;23]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il1_7x5: 2 frozen stored streams; R=116.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il1_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x74, 0x0, 0x8b, 0xff, 0x0, 0xd4, 0xd1, 0xc9, 0x1, 0x8f, 0x24, 0x86, 0x2, 0xec, 0xda, 0x1e, 0x3a, 0x6f, 0xd3, 0x3, 0xd8, 0x35, 0xb2, 0x5f, 0xb9, 0x10, 0x4, 0x14, 0x9d, 0x6b, 0x0, 0x6, 0x3, 0x0, 0x87, 0x58, 0xb8, 0x2c, 0xc1, 0x11, 0x8e, 0x33, 0x92, 0xe5, 0x81, 0x5d, 0x1, 0x45, 0xd9, 0xfe, 0x5b, 0x16, 0xfa, 0xea, 0xbe, 0xb4, 0x2, 0x9b, 0x79, 0xf, 0xe7, 0x2b, 0x6b, 0xd9, 0xe0, 0x2e, 0x3, 0xdc, 0xb0, 0xce, 0x0, 0xe4, 0xa, 0x1b, 0x6f, 0xf0, 0x4, 0xae, 0xe, 0xff, 0x4b, 0x21, 0x79, 0x56, 0x5e, 0xb2, 0xe, 0xf1, 0x58, 0xb1, 0xcf, 0xca, 0x73, 0xea, 0x94, 0x70, 0xf1, 0x71, 0x0, 0xfd, 0xfd, 0xd9, 0x94, 0xe1, 0x4d, 0xb6, 0xae, 0x24, 0x93, 0x83, 0xb8, 0x76, 0xd2, 0xe3, 0xd4, 0x87, 0x5, 0x3a, 0x73, 0xc6, 0xe3, 0x87, 0x38, 0x52];
    proof {
        let z = seq![120u8, 156, 1, 116, 0, 139, 255, 0, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198, 227, 135, 56, 82]; let raw = seq![0u8, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198];
        assert(z.subrange(7,123) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 116, 0, 139, 255, 0, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198, 227, 135, 56, 82],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 116, 0, 139, 255, 0, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198, 227, 135, 56, 82],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),116) == (m::StoredEffect::End{out:raw,position:984}));
        assert(m::MOneBlock(z,16,Seq::empty(),116,32768,true) == (true,m::StoredEffect::End{out:raw,position:984}));
        assert(m::MBlk(z,16,Seq::empty(),116,32768,true) == (m::StoredEffect::End{out:raw,position:984}));
        assert(m::MFinish(seq![120u8, 156, 1, 116, 0, 139, 255, 0, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198, 227, 135, 56, 82],984,seq![0u8, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198],116,false) == (m::StoredEffect::End{out:seq![0u8, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198],position:1016})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,116,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,116,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 127) by (compute);
    }
    let ghost raw = seq![0u8, 212, 209, 201, 1, 143, 36, 134, 2, 236, 218, 30, 58, 111, 211, 3, 216, 53, 178, 95, 185, 16, 4, 20, 157, 107, 0, 6, 3, 0, 135, 88, 184, 44, 193, 17, 142, 51, 146, 229, 129, 93, 1, 69, 217, 254, 91, 22, 250, 234, 190, 180, 2, 155, 121, 15, 231, 43, 107, 217, 224, 46, 3, 220, 176, 206, 0, 228, 10, 27, 111, 240, 4, 174, 14, 255, 75, 33, 121, 86, 94, 178, 14, 241, 88, 177, 207, 202, 115, 234, 148, 112, 241, 113, 0, 253, 253, 217, 148, 225, 77, 182, 174, 36, 147, 131, 184, 118, 210, 227, 212, 135, 5, 58, 115, 198]; let mut output = [0xa5u8;116]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d8_il1_17x2: 1 frozen stored streams; R=107.
#[verifier::spinoff_prover]
fn intent_ct2_d8_il1_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x6b, 0x0, 0x94, 0xff, 0x0, 0x18, 0xe2, 0xe7, 0xa, 0xd9, 0x3e, 0x44, 0x3b, 0x5e, 0x1, 0xfd, 0x33, 0x2d, 0xba, 0x64, 0xe3, 0x2, 0xe3, 0xd2, 0x1e, 0x35, 0x40, 0xf5, 0x2b, 0x3c, 0x64, 0xd4, 0xe8, 0x8a, 0x3, 0xc6, 0x76, 0x4, 0x14, 0x30, 0x5f, 0x10, 0xfb, 0x93, 0x45, 0x34, 0xbe, 0xc8, 0xae, 0xeb, 0x99, 0x6b, 0x8f, 0x13, 0x99, 0xe5, 0xd6, 0xd1, 0xf6, 0x4, 0xcb, 0xce, 0x25, 0x32, 0xb5, 0x55, 0xd5, 0x61, 0x1e, 0x38, 0x5d, 0x33, 0x48, 0x5e, 0x18, 0x6d, 0x32, 0xea, 0x92, 0xdd, 0xb5, 0xd9, 0x56, 0x72, 0xfa, 0x42, 0x7a, 0x23, 0x8b, 0x94, 0x7b, 0x8d, 0xfb, 0x8b, 0x5f, 0xed, 0x9d, 0xc1, 0xab, 0x39, 0x62, 0x3d, 0x78, 0x88, 0x7b, 0x84, 0x2, 0xf2, 0xa3, 0x3e, 0xde, 0x7b, 0x5c, 0x33, 0xf7];
    proof {
        let z = seq![120u8, 156, 1, 107, 0, 148, 255, 0, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222, 123, 92, 51, 247]; let raw = seq![0u8, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222];
        assert(z.subrange(7,114) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 107, 0, 148, 255, 0, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222, 123, 92, 51, 247],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 107, 0, 148, 255, 0, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222, 123, 92, 51, 247],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),107) == (m::StoredEffect::End{out:raw,position:912}));
        assert(m::MOneBlock(z,16,Seq::empty(),107,32768,true) == (true,m::StoredEffect::End{out:raw,position:912}));
        assert(m::MBlk(z,16,Seq::empty(),107,32768,true) == (m::StoredEffect::End{out:raw,position:912}));
        assert(m::MFinish(seq![120u8, 156, 1, 107, 0, 148, 255, 0, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222, 123, 92, 51, 247],912,seq![0u8, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222],107,false) == (m::StoredEffect::End{out:seq![0u8, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222],position:944})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,107,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,107,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 118) by (compute);
    }
    let ghost raw = seq![0u8, 24, 226, 231, 10, 217, 62, 68, 59, 94, 1, 253, 51, 45, 186, 100, 227, 2, 227, 210, 30, 53, 64, 245, 43, 60, 100, 212, 232, 138, 3, 198, 118, 4, 20, 48, 95, 16, 251, 147, 69, 52, 190, 200, 174, 235, 153, 107, 143, 19, 153, 229, 214, 209, 246, 4, 203, 206, 37, 50, 181, 85, 213, 97, 30, 56, 93, 51, 72, 94, 24, 109, 50, 234, 146, 221, 181, 217, 86, 114, 250, 66, 122, 35, 139, 148, 123, 141, 251, 139, 95, 237, 157, 193, 171, 57, 98, 61, 120, 136, 123, 132, 2, 242, 163, 62, 222]; let mut output = [0xa5u8;107]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d16_il0_1x1: 1 independently audited frozen streams; R=7.
#[verifier::spinoff_prover]
fn intent_ct2_d16_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf8, 0xfa, 0x2b, 0xf5, 0xea, 0x39, 0x2b, 0x0, 0x10, 0x90, 0x4, 0x32];
    proof {
        let z = seq![120u8, 156, 99, 248, 250, 43, 245, 234, 57, 43, 0, 16, 144, 4, 50]; let raw = seq![0u8, 245, 250, 101, 213, 206, 58]; let positions = seq![19nat, 27, 36, 45, 53, 62, 71, 79, 86];
        assert(literal_words(seq![120u8, 156, 99, 248, 250, 43, 245, 234, 57, 43, 0, 16, 144, 4, 50],seq![0u8, 245, 250, 101, 213, 206, 58],seq![19nat, 27, 36, 45, 53, 62, 71, 79, 86],0,8)) by (compute);
        literal_decodes(z,raw,positions,0,8); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),7,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 245, 250, 101, 213, 206, 58],position:86}));
        assert(m::field_value(seq![120u8, 156, 99, 248, 250, 43, 245, 234, 57, 43, 0, 16, 144, 4, 50],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 248, 250, 43, 245, 234, 57, 43, 0, 16, 144, 4, 50],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),7,32768,true) == (m::StoredEffect::End{out:seq![0u8, 245, 250, 101, 213, 206, 58],position:86}));
        assert(m::MFinish(seq![120u8, 156, 99, 248, 250, 43, 245, 234, 57, 43, 0, 16, 144, 4, 50],86,seq![0u8, 245, 250, 101, 213, 206, 58],7,false) == (m::StoredEffect::End{out:seq![0u8, 245, 250, 101, 213, 206, 58],position:120})) by (compute);
        assert(m::Inflate(z,7,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,7,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
    }
    let ghost raw = seq![0u8, 245, 250, 101, 213, 206, 58]; let mut output = [0xa5u8;7]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d16_il0_2x3: 1 independently audited frozen streams; R=39.
#[verifier::spinoff_prover]
fn intent_ct2_d16_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x48, 0xc, 0xf0, 0x39, 0x76, 0xf5, 0xca, 0x4c, 0x2f, 0x5, 0xd6, 0xae, 0x62, 0x46, 0x31, 0x67, 0xb9, 0xf0, 0xbf, 0x2d, 0xd6, 0xaf, 0x4b, 0x32, 0xbe, 0xea, 0x31, 0x9, 0x3a, 0xde, 0xfc, 0x7b, 0xef, 0x7d, 0xad, 0x73, 0x81, 0x95, 0xfc, 0x7a, 0x0, 0x43, 0xc, 0x11, 0x16];
    proof {
        let z = seq![120u8, 156, 99, 72, 12, 240, 57, 118, 245, 202, 76, 47, 5, 214, 174, 98, 70, 49, 103, 185, 240, 191, 45, 214, 175, 75, 50, 190, 234, 49, 9, 58, 222, 252, 123, 239, 125, 173, 115, 129, 149, 252, 122, 0, 67, 12, 17, 22]; let raw = seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175]; let positions = seq![19nat, 27, 35, 43, 51, 60, 69, 78, 87, 95, 103, 111, 119, 127, 135, 143, 151, 159, 167, 176, 184, 192, 201, 209, 217, 226, 234, 242, 250, 258, 267, 276, 285, 294, 302, 310, 318, 326, 334, 343, 350];
        assert(literal_words(seq![120u8, 156, 99, 72, 12, 240, 57, 118, 245, 202, 76, 47, 5, 214, 174, 98, 70, 49, 103, 185, 240, 191, 45, 214, 175, 75, 50, 190, 234, 49, 9, 58, 222, 252, 123, 239, 125, 173, 115, 129, 149, 252, 122, 0, 67, 12, 17, 22],seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175],seq![19nat, 27, 35, 43, 51, 60, 69, 78, 87, 95, 103, 111, 119, 127, 135, 143, 151, 159, 167, 176, 184, 192, 201, 209, 217, 226, 234, 242, 250, 258, 267, 276, 285, 294, 302, 310, 318, 326, 334, 343, 350],0,40)) by (compute);
        literal_decodes(z,raw,positions,0,40); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),39,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175],position:350}));
        assert(m::field_value(seq![120u8, 156, 99, 72, 12, 240, 57, 118, 245, 202, 76, 47, 5, 214, 174, 98, 70, 49, 103, 185, 240, 191, 45, 214, 175, 75, 50, 190, 234, 49, 9, 58, 222, 252, 123, 239, 125, 173, 115, 129, 149, 252, 122, 0, 67, 12, 17, 22],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 72, 12, 240, 57, 118, 245, 202, 76, 47, 5, 214, 174, 98, 70, 49, 103, 185, 240, 191, 45, 214, 175, 75, 50, 190, 234, 49, 9, 58, 222, 252, 123, 239, 125, 173, 115, 129, 149, 252, 122, 0, 67, 12, 17, 22],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),39,32768,true) == (m::StoredEffect::End{out:seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175],position:350}));
        assert(m::MFinish(seq![120u8, 156, 99, 72, 12, 240, 57, 118, 245, 202, 76, 47, 5, 214, 174, 98, 70, 49, 103, 185, 240, 191, 45, 214, 175, 75, 50, 190, 234, 49, 9, 58, 222, 252, 123, 239, 125, 173, 115, 129, 149, 252, 122, 0, 67, 12, 17, 22],350,seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175],39,false) == (m::StoredEffect::End{out:seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175],position:384})) by (compute);
        assert(m::Inflate(z,39,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,39,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 48) by (compute);
    }
    let ghost raw = seq![0u8, 97, 80, 76, 198, 213, 212, 153, 74, 32, 5, 138, 115, 1, 22, 67, 30, 87, 253, 132, 59, 235, 116, 104, 245, 46, 2, 17, 65, 217, 253, 222, 239, 125, 67, 112, 58, 31, 175]; let mut output = [0xa5u8;39]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d16_il1_1x1: 1 independently audited frozen streams; R=7.
#[verifier::spinoff_prover]
fn intent_ct2_d16_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x58, 0xcc, 0x96, 0x2a, 0xf4, 0x73, 0x6, 0x0, 0x8, 0x4b, 0x2, 0xb2];
    proof {
        let z = seq![120u8, 156, 99, 88, 204, 150, 42, 244, 115, 6, 0, 8, 75, 2, 178]; let raw = seq![0u8, 163, 6, 101, 18, 249, 152]; let positions = seq![19nat, 27, 36, 44, 52, 60, 69, 78, 85];
        assert(literal_words(seq![120u8, 156, 99, 88, 204, 150, 42, 244, 115, 6, 0, 8, 75, 2, 178],seq![0u8, 163, 6, 101, 18, 249, 152],seq![19nat, 27, 36, 44, 52, 60, 69, 78, 85],0,8)) by (compute);
        literal_decodes(z,raw,positions,0,8); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),7,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 163, 6, 101, 18, 249, 152],position:85}));
        assert(m::field_value(seq![120u8, 156, 99, 88, 204, 150, 42, 244, 115, 6, 0, 8, 75, 2, 178],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 88, 204, 150, 42, 244, 115, 6, 0, 8, 75, 2, 178],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),7,32768,true) == (m::StoredEffect::End{out:seq![0u8, 163, 6, 101, 18, 249, 152],position:85}));
        assert(m::MFinish(seq![120u8, 156, 99, 88, 204, 150, 42, 244, 115, 6, 0, 8, 75, 2, 178],85,seq![0u8, 163, 6, 101, 18, 249, 152],7,false) == (m::StoredEffect::End{out:seq![0u8, 163, 6, 101, 18, 249, 152],position:120})) by (compute);
        assert(m::Inflate(z,7,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,7,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
    }
    let ghost raw = seq![0u8, 163, 6, 101, 18, 249, 152]; let mut output = [0xa5u8;7]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct2_d16_il1_2x3: 1 frozen stored streams; R=41.
#[verifier::spinoff_prover]
fn intent_ct2_d16_il1_2x3() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x29, 0x0, 0xd6, 0xff, 0x0, 0x23, 0x50, 0xd0, 0xca, 0xa, 0xae, 0x1, 0x59, 0x4d, 0x4f, 0x86, 0x79, 0xf7, 0x2, 0xfe, 0xad, 0x91, 0x4c, 0x11, 0x5a, 0x3, 0xc5, 0x45, 0xf9, 0x87, 0x79, 0x73, 0x4, 0x90, 0x93, 0x35, 0x67, 0x44, 0x8c, 0x89, 0x9c, 0xf5, 0xbb, 0xa8, 0x47, 0x67, 0x5f, 0x12, 0x77];
    proof {
        let z = seq![120u8, 156, 1, 41, 0, 214, 255, 0, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71, 103, 95, 18, 119]; let raw = seq![0u8, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71];
        assert(z.subrange(7,48) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 41, 0, 214, 255, 0, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71, 103, 95, 18, 119],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 41, 0, 214, 255, 0, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71, 103, 95, 18, 119],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),41) == (m::StoredEffect::End{out:raw,position:384}));
        assert(m::MOneBlock(z,16,Seq::empty(),41,32768,true) == (true,m::StoredEffect::End{out:raw,position:384}));
        assert(m::MBlk(z,16,Seq::empty(),41,32768,true) == (m::StoredEffect::End{out:raw,position:384}));
        assert(m::MFinish(seq![120u8, 156, 1, 41, 0, 214, 255, 0, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71, 103, 95, 18, 119],384,seq![0u8, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71],41,false) == (m::StoredEffect::End{out:seq![0u8, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71],position:416})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,41,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,41,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 52) by (compute);
    }
    let ghost raw = seq![0u8, 35, 80, 208, 202, 10, 174, 1, 89, 77, 79, 134, 121, 247, 2, 254, 173, 145, 76, 17, 90, 3, 197, 69, 249, 135, 121, 115, 4, 144, 147, 53, 103, 68, 140, 137, 156, 245, 187, 168, 71]; let mut output = [0xa5u8;41]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x60, 0x74, 0x60, 0x6a, 0x0, 0x0, 0x1, 0x4e, 0x0, 0xc4];
    proof {
        let z = seq![120u8, 156, 99, 96, 96, 116, 96, 106, 0, 0, 1, 78, 0, 196]; let raw = seq![0u8, 0, 1, 64, 2, 128]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 74];
        assert(literal_words(seq![120u8, 156, 99, 96, 96, 116, 96, 106, 0, 0, 1, 78, 0, 196],seq![0u8, 0, 1, 64, 2, 128],seq![19nat, 27, 35, 43, 51, 59, 67, 74],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 64, 2, 128],position:74}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 96, 116, 96, 106, 0, 0, 1, 78, 0, 196],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 96, 116, 96, 106, 0, 0, 1, 78, 0, 196],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 64, 2, 128],position:74}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 96, 116, 96, 106, 0, 0, 1, 78, 0, 196],74,seq![0u8, 0, 1, 64, 2, 128],6,false) == (m::StoredEffect::End{out:seq![0u8, 0, 1, 64, 2, 128],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 0, 1, 64, 2, 128]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il0_7x5: 2 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x58, 0xc4, 0x38, 0x83, 0xc9, 0x81, 0xd9, 0x82, 0xa5, 0x2, 0x0, 0xc, 0x6c, 0x2, 0x35];
    proof {
        let z = seq![120u8, 156, 99, 88, 196, 56, 131, 201, 129, 217, 130, 165, 2, 0, 12, 108, 2, 53]; let raw = seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120]; let positions = seq![19nat, 27, 36, 44, 53, 61, 69, 77, 85, 93, 101, 108];
        assert(literal_words(seq![120u8, 156, 99, 88, 196, 56, 131, 201, 129, 217, 130, 165, 2, 0, 12, 108, 2, 53],seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120],seq![19nat, 27, 36, 44, 53, 61, 69, 77, 85, 93, 101, 108],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120],position:108}));
        assert(m::field_value(seq![120u8, 156, 99, 88, 196, 56, 131, 201, 129, 217, 130, 165, 2, 0, 12, 108, 2, 53],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 88, 196, 56, 131, 201, 129, 217, 130, 165, 2, 0, 12, 108, 2, 53],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120],position:108}));
        assert(m::MFinish(seq![120u8, 156, 99, 88, 196, 56, 131, 201, 129, 217, 130, 165, 2, 0, 12, 108, 2, 53],108,seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120],10,false) == (m::StoredEffect::End{out:seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 162, 1, 152, 2, 64, 3, 56, 4, 120]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il0_9x10: 1 independently audited frozen streams; R=30.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il0_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xd8, 0xc2, 0xc0, 0xe8, 0xbc, 0x97, 0x69, 0x26, 0x3, 0xb3, 0xba, 0x19, 0x4b, 0x12, 0x27, 0xc3, 0xc4, 0x6, 0x46, 0xcf, 0xed, 0x4c, 0xbc, 0xd, 0xcc, 0x5, 0xc5, 0x2c, 0x67, 0x1b, 0x0, 0x6d, 0x67, 0x7, 0xf8];
    proof {
        let z = seq![120u8, 156, 99, 216, 194, 192, 232, 188, 151, 105, 38, 3, 179, 186, 25, 75, 18, 39, 195, 196, 6, 70, 207, 237, 76, 188, 13, 204, 5, 197, 44, 103, 27, 0, 109, 103, 7, 248]; let raw = seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128]; let positions = seq![19nat, 27, 36, 44, 52, 60, 69, 77, 86, 94, 102, 110, 118, 126, 134, 142, 150, 159, 167, 175, 183, 192, 200, 208, 216, 224, 232, 240, 248, 257, 265, 272];
        assert(literal_words(seq![120u8, 156, 99, 216, 194, 192, 232, 188, 151, 105, 38, 3, 179, 186, 25, 75, 18, 39, 195, 196, 6, 70, 207, 237, 76, 188, 13, 204, 5, 197, 44, 103, 27, 0, 109, 103, 7, 248],seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128],seq![19nat, 27, 36, 44, 52, 60, 69, 77, 86, 94, 102, 110, 118, 126, 134, 142, 150, 159, 167, 175, 183, 192, 200, 208, 216, 224, 232, 240, 248, 257, 265, 272],0,31)) by (compute);
        literal_decodes(z,raw,positions,0,31); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),30,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128],position:272}));
        assert(m::field_value(seq![120u8, 156, 99, 216, 194, 192, 232, 188, 151, 105, 38, 3, 179, 186, 25, 75, 18, 39, 195, 196, 6, 70, 207, 237, 76, 188, 13, 204, 5, 197, 44, 103, 27, 0, 109, 103, 7, 248],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 216, 194, 192, 232, 188, 151, 105, 38, 3, 179, 186, 25, 75, 18, 39, 195, 196, 6, 70, 207, 237, 76, 188, 13, 204, 5, 197, 44, 103, 27, 0, 109, 103, 7, 248],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),30,32768,true) == (m::StoredEffect::End{out:seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128],position:272}));
        assert(m::MFinish(seq![120u8, 156, 99, 216, 194, 192, 232, 188, 151, 105, 38, 3, 179, 186, 25, 75, 18, 39, 195, 196, 6, 70, 207, 237, 76, 188, 13, 204, 5, 197, 44, 103, 27, 0, 109, 103, 7, 248],272,seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128],30,false) == (m::StoredEffect::End{out:seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128],position:304})) by (compute);
        assert(m::Inflate(z,30,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,30,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 38) by (compute);
    }
    let ghost raw = seq![0u8, 180, 0, 1, 67, 189, 2, 153, 0, 3, 39, 54, 4, 98, 9, 0, 145, 128, 1, 73, 183, 2, 13, 128, 3, 112, 115, 4, 205, 128]; let mut output = [0xa5u8;30]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il0_17x2: 1 independently audited frozen streams; R=8.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0x1a, 0xca, 0xc0, 0x58, 0x2c, 0xf6, 0x1d, 0x0, 0x9, 0xe9, 0x2, 0x9c];
    proof {
        let z = seq![120u8, 156, 99, 56, 26, 202, 192, 88, 44, 246, 29, 0, 9, 233, 2, 156]; let raw = seq![0u8, 197, 85, 0, 1, 115, 22, 247]; let positions = seq![19nat, 27, 36, 44, 52, 60, 68, 76, 85, 92];
        assert(literal_words(seq![120u8, 156, 99, 56, 26, 202, 192, 88, 44, 246, 29, 0, 9, 233, 2, 156],seq![0u8, 197, 85, 0, 1, 115, 22, 247],seq![19nat, 27, 36, 44, 52, 60, 68, 76, 85, 92],0,9)) by (compute);
        literal_decodes(z,raw,positions,0,9); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),8,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 197, 85, 0, 1, 115, 22, 247],position:92}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 26, 202, 192, 88, 44, 246, 29, 0, 9, 233, 2, 156],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 26, 202, 192, 88, 44, 246, 29, 0, 9, 233, 2, 156],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),8,32768,true) == (m::StoredEffect::End{out:seq![0u8, 197, 85, 0, 1, 115, 22, 247],position:92}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 26, 202, 192, 88, 44, 246, 29, 0, 9, 233, 2, 156],92,seq![0u8, 197, 85, 0, 1, 115, 22, 247],8,false) == (m::StoredEffect::End{out:seq![0u8, 197, 85, 0, 1, 115, 22, 247],position:128})) by (compute);
        assert(m::Inflate(z,8,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,8,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
    }
    let ghost raw = seq![0u8, 197, 85, 0, 1, 115, 22, 247]; let mut output = [0xa5u8;8]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x64, 0x60, 0x62, 0x60, 0x66, 0x60, 0x71, 0x0, 0x0, 0x4, 0xf2, 0x0, 0xcb];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 100, 96, 98, 96, 102, 96, 113, 0, 0, 4, 242, 0, 203]; let raw = seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 99, 106];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 100, 96, 98, 96, 102, 96, 113, 0, 0, 4, 242, 0, 203],seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 99, 106],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64],position:106}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 100, 96, 98, 96, 102, 96, 113, 0, 0, 4, 242, 0, 203],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 100, 96, 98, 96, 102, 96, 113, 0, 0, 4, 242, 0, 203],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64],position:106}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 100, 96, 98, 96, 102, 96, 113, 0, 0, 4, 242, 0, 203],106,seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64],10,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 0, 2, 0, 3, 0, 4, 64]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il1_7x5: 2 independently audited frozen streams; R=22.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x6c, 0x60, 0x6a, 0x60, 0x3e, 0xc0, 0xe2, 0xc0, 0xb0, 0x81, 0xf1, 0x1, 0xd3, 0x2, 0xe6, 0x5, 0x2c, 0xb3, 0x18, 0xb4, 0x0, 0x44, 0xda, 0x6, 0x29];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 108, 96, 106, 96, 62, 192, 226, 192, 176, 129, 241, 1, 211, 2, 230, 5, 44, 179, 24, 180, 0, 68, 218, 6, 41]; let raw = seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 125, 134, 142, 151, 159, 168, 176, 185, 193, 201, 208];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 108, 96, 106, 96, 62, 192, 226, 192, 176, 129, 241, 1, 211, 2, 230, 5, 44, 179, 24, 180, 0, 68, 218, 6, 41],seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 125, 134, 142, 151, 159, 168, 176, 185, 193, 201, 208],0,23)) by (compute);
        literal_decodes(z,raw,positions,0,23); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),22,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42],position:208}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 108, 96, 106, 96, 62, 192, 226, 192, 176, 129, 241, 1, 211, 2, 230, 5, 44, 179, 24, 180, 0, 68, 218, 6, 41],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 108, 96, 106, 96, 62, 192, 226, 192, 176, 129, 241, 1, 211, 2, 230, 5, 44, 179, 24, 180, 0, 68, 218, 6, 41],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),22,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42],position:208}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 108, 96, 106, 96, 62, 192, 226, 192, 176, 129, 241, 1, 211, 2, 230, 5, 44, 179, 24, 180, 0, 68, 218, 6, 41],208,seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42],22,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42],position:240})) by (compute);
        assert(m::Inflate(z,22,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,22,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 128, 2, 128, 3, 192, 4, 64, 0, 176, 1, 224, 2, 160, 3, 160, 4, 154, 0, 42]; let mut output = [0xa5u8;22]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il1_9x10: 1 independently audited frozen streams; R=45.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il1_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x74, 0x60, 0x6a, 0x60, 0x3e, 0xc0, 0xf2, 0x80, 0x81, 0x81, 0xf1, 0x0, 0xd3, 0x1, 0xe6, 0x2, 0x96, 0xa, 0x86, 0xb, 0x8c, 0x1, 0x4c, 0x9, 0xcc, 0x2f, 0x58, 0xe, 0x30, 0xd8, 0x36, 0x30, 0x16, 0x8, 0x30, 0xe9, 0x35, 0x30, 0xf7, 0x4f, 0x64, 0x11, 0x6b, 0x0, 0x0, 0x13, 0x6f, 0xc, 0x3a];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 116, 96, 106, 96, 62, 192, 242, 128, 129, 129, 241, 0, 211, 1, 230, 2, 150, 10, 134, 11, 140, 1, 76, 9, 204, 47, 88, 14, 48, 216, 54, 48, 22, 8, 48, 233, 53, 48, 247, 79, 100, 17, 107, 0, 0, 19, 111, 12, 58]; let raw = seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 101, 109, 117, 125, 134, 142, 151, 159, 167, 175, 183, 191, 200, 208, 216, 224, 232, 240, 249, 257, 266, 274, 282, 290, 298, 306, 314, 322, 330, 338, 346, 354, 363, 371, 379, 387, 394];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 116, 96, 106, 96, 62, 192, 242, 128, 129, 129, 241, 0, 211, 1, 230, 2, 150, 10, 134, 11, 140, 1, 76, 9, 204, 47, 88, 14, 48, 216, 54, 48, 22, 8, 48, 233, 53, 48, 247, 79, 100, 17, 107, 0, 0, 19, 111, 12, 58],seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 101, 109, 117, 125, 134, 142, 151, 159, 167, 175, 183, 191, 200, 208, 216, 224, 232, 240, 249, 257, 266, 274, 282, 290, 298, 306, 314, 322, 330, 338, 346, 354, 363, 371, 379, 387, 394],0,46)) by (compute);
        literal_decodes(z,raw,positions,0,46); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),45,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128],position:394}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 116, 96, 106, 96, 62, 192, 242, 128, 129, 129, 241, 0, 211, 1, 230, 2, 150, 10, 134, 11, 140, 1, 76, 9, 204, 47, 88, 14, 48, 216, 54, 48, 22, 8, 48, 233, 53, 48, 247, 79, 100, 17, 107, 0, 0, 19, 111, 12, 58],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 116, 96, 106, 96, 62, 192, 242, 128, 129, 129, 241, 0, 211, 1, 230, 2, 150, 10, 134, 11, 140, 1, 76, 9, 204, 47, 88, 14, 48, 216, 54, 48, 22, 8, 48, 233, 53, 48, 247, 79, 100, 17, 107, 0, 0, 19, 111, 12, 58],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),45,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128],position:394}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 116, 96, 106, 96, 62, 192, 242, 128, 129, 129, 241, 0, 211, 1, 230, 2, 150, 10, 134, 11, 140, 1, 76, 9, 204, 47, 88, 14, 48, 216, 54, 48, 22, 8, 48, 233, 53, 48, 247, 79, 100, 17, 107, 0, 0, 19, 111, 12, 58],394,seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128],45,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128],position:432})) by (compute);
        assert(m::Inflate(z,45,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,45,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 54) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 64, 2, 128, 3, 192, 4, 224, 0, 0, 1, 192, 2, 192, 3, 112, 4, 120, 0, 208, 1, 80, 2, 96, 3, 232, 4, 192, 0, 61, 128, 1, 112, 16, 2, 46, 128, 3, 143, 145, 4, 22, 128]; let mut output = [0xa5u8;45]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d1_il1_17x2: 1 independently audited frozen streams; R=12.
#[verifier::spinoff_prover]
fn intent_ct3_d1_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x48, 0x60, 0x6c, 0x60, 0x32, 0x60, 0xe6, 0x64, 0xf9, 0xb8, 0xfb, 0xa, 0x0, 0xf, 0x82, 0x3, 0xa4];
    proof {
        let z = seq![120u8, 156, 99, 72, 96, 108, 96, 50, 96, 230, 100, 249, 184, 251, 10, 0, 15, 130, 3, 164]; let raw = seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 109, 118, 125];
        assert(literal_words(seq![120u8, 156, 99, 72, 96, 108, 96, 50, 96, 230, 100, 249, 184, 251, 10, 0, 15, 130, 3, 164],seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 100, 109, 118, 125],0,13)) by (compute);
        literal_decodes(z,raw,positions,0,13); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),12,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212],position:125}));
        assert(m::field_value(seq![120u8, 156, 99, 72, 96, 108, 96, 50, 96, 230, 100, 249, 184, 251, 10, 0, 15, 130, 3, 164],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 72, 96, 108, 96, 50, 96, 230, 100, 249, 184, 251, 10, 0, 15, 130, 3, 164],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),12,32768,true) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212],position:125}));
        assert(m::MFinish(seq![120u8, 156, 99, 72, 96, 108, 96, 50, 96, 230, 100, 249, 184, 251, 10, 0, 15, 130, 3, 164],125,seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212],12,false) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212],position:160})) by (compute);
        assert(m::Inflate(z,12,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,12,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 20) by (compute);
    }
    let ghost raw = seq![0u8, 96, 1, 128, 2, 48, 3, 9, 4, 241, 187, 212]; let mut output = [0xa5u8;12]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x28, 0x60, 0xbc, 0xc0, 0x64, 0x0, 0x0, 0x4, 0xde, 0x1, 0x74];
    proof {
        let z = seq![120u8, 156, 99, 40, 96, 188, 192, 100, 0, 0, 4, 222, 1, 116]; let raw = seq![0u8, 112, 1, 208, 2, 48]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 75];
        assert(literal_words(seq![120u8, 156, 99, 40, 96, 188, 192, 100, 0, 0, 4, 222, 1, 116],seq![0u8, 112, 1, 208, 2, 48],seq![19nat, 27, 35, 43, 52, 60, 68, 75],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 208, 2, 48],position:75}));
        assert(m::field_value(seq![120u8, 156, 99, 40, 96, 188, 192, 100, 0, 0, 4, 222, 1, 116],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 40, 96, 188, 192, 100, 0, 0, 4, 222, 1, 116],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 208, 2, 48],position:75}));
        assert(m::MFinish(seq![120u8, 156, 99, 40, 96, 188, 192, 100, 0, 0, 4, 222, 1, 116],75,seq![0u8, 112, 1, 208, 2, 48],6,false) == (m::StoredEffect::End{out:seq![0u8, 112, 1, 208, 2, 48],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 112, 1, 208, 2, 48]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il0_7x5: 2 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x48, 0x6b, 0x61, 0x9c, 0x91, 0xc0, 0xb4, 0xc5, 0x84, 0xf9, 0x59, 0x8, 0x4b, 0xb2, 0x5, 0x0, 0x24, 0xb3, 0x4, 0xaa];
    proof {
        let z = seq![120u8, 156, 99, 72, 107, 97, 156, 145, 192, 180, 197, 132, 249, 89, 8, 75, 178, 5, 0, 36, 179, 4, 170]; let raw = seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56]; let positions = seq![19nat, 27, 35, 43, 51, 60, 68, 76, 85, 93, 101, 110, 118, 126, 134, 142, 149];
        assert(literal_words(seq![120u8, 156, 99, 72, 107, 97, 156, 145, 192, 180, 197, 132, 249, 89, 8, 75, 178, 5, 0, 36, 179, 4, 170],seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56],seq![19nat, 27, 35, 43, 51, 60, 68, 76, 85, 93, 101, 110, 118, 126, 134, 142, 149],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56],position:149}));
        assert(m::field_value(seq![120u8, 156, 99, 72, 107, 97, 156, 145, 192, 180, 197, 132, 249, 89, 8, 75, 178, 5, 0, 36, 179, 4, 170],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 72, 107, 97, 156, 145, 192, 180, 197, 132, 249, 89, 8, 75, 178, 5, 0, 36, 179, 4, 170],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56],position:149}));
        assert(m::MFinish(seq![120u8, 156, 99, 72, 107, 97, 156, 145, 192, 180, 197, 132, 249, 89, 8, 75, 178, 5, 0, 36, 179, 4, 170],149,seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56],15,false) == (m::StoredEffect::End{out:seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56],position:184})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 23) by (compute);
    }
    let ghost raw = seq![0u8, 102, 132, 1, 152, 96, 2, 180, 52, 3, 230, 84, 4, 99, 56]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il0_9x10: 1 independently audited frozen streams; R=40.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il0_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0x6a, 0x3c, 0xc0, 0x68, 0x5a, 0xe3, 0xcf, 0xb4, 0xed, 0xfd, 0x1, 0xe6, 0xed, 0x3f, 0x3e, 0xb2, 0xe8, 0xe6, 0x99, 0x30, 0x3c, 0x12, 0x3a, 0xc0, 0x68, 0xfc, 0x42, 0x95, 0x69, 0x43, 0x92, 0x3, 0xb3, 0x75, 0x9f, 0x3e, 0xcb, 0x9a, 0x4f, 0xc, 0x0, 0x38, 0xe, 0xf, 0x8];
    proof {
        let z = seq![120u8, 156, 99, 96, 106, 60, 192, 104, 90, 227, 207, 180, 237, 253, 1, 230, 237, 63, 62, 178, 232, 230, 153, 48, 60, 18, 58, 192, 104, 252, 66, 149, 105, 67, 146, 3, 179, 117, 159, 62, 203, 154, 79, 12, 0, 56, 14, 15, 8]; let raw = seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 101, 110, 119, 127, 136, 145, 154, 162, 170, 178, 186, 194, 203, 211, 220, 228, 236, 245, 253, 261, 270, 278, 286, 294, 302, 310, 318, 326, 335, 344, 352, 359];
        assert(literal_words(seq![120u8, 156, 99, 96, 106, 60, 192, 104, 90, 227, 207, 180, 237, 253, 1, 230, 237, 63, 62, 178, 232, 230, 153, 48, 60, 18, 58, 192, 104, 252, 66, 149, 105, 67, 146, 3, 179, 117, 159, 62, 203, 154, 79, 12, 0, 56, 14, 15, 8],seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 101, 110, 119, 127, 136, 145, 154, 162, 170, 178, 186, 194, 203, 211, 220, 228, 236, 245, 253, 261, 270, 278, 286, 294, 302, 310, 318, 326, 335, 344, 352, 359],0,41)) by (compute);
        literal_decodes(z,raw,positions,0,41); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),40,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0],position:359}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 106, 60, 192, 104, 90, 227, 207, 180, 237, 253, 1, 230, 237, 63, 62, 178, 232, 230, 153, 48, 60, 18, 58, 192, 104, 252, 66, 149, 105, 67, 146, 3, 179, 117, 159, 62, 203, 154, 79, 12, 0, 56, 14, 15, 8],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 106, 60, 192, 104, 90, 227, 207, 180, 237, 253, 1, 230, 237, 63, 62, 178, 232, 230, 153, 48, 60, 18, 58, 192, 104, 252, 66, 149, 105, 67, 146, 3, 179, 117, 159, 62, 203, 154, 79, 12, 0, 56, 14, 15, 8],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),40,32768,true) == (m::StoredEffect::End{out:seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0],position:359}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 106, 60, 192, 104, 90, 227, 207, 180, 237, 253, 1, 230, 237, 63, 62, 178, 232, 230, 153, 48, 60, 18, 58, 192, 104, 252, 66, 149, 105, 67, 146, 3, 179, 117, 159, 62, 203, 154, 79, 12, 0, 56, 14, 15, 8],359,seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0],40,false) == (m::StoredEffect::End{out:seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0],position:392})) by (compute);
        assert(m::Inflate(z,40,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,40,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 49) by (compute);
    }
    let ghost raw = seq![0u8, 2, 129, 192, 1, 53, 124, 79, 2, 182, 239, 192, 3, 183, 248, 241, 4, 45, 110, 52, 0, 226, 18, 192, 1, 51, 232, 37, 2, 176, 98, 64, 3, 59, 142, 47, 4, 172, 242, 0]; let mut output = [0xa5u8;40]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il0_17x2: 1 independently audited frozen streams; R=12.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x90, 0x6a, 0x55, 0x53, 0x75, 0x60, 0x3c, 0xb1, 0xd0, 0x62, 0xbd, 0x1, 0x0, 0x13, 0x42, 0x3, 0xac];
    proof {
        let z = seq![120u8, 156, 99, 144, 106, 85, 83, 117, 96, 60, 177, 208, 98, 189, 1, 0, 19, 66, 3, 172]; let raw = seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 93, 101, 110, 118, 125];
        assert(literal_words(seq![120u8, 156, 99, 144, 106, 85, 83, 117, 96, 60, 177, 208, 98, 189, 1, 0, 19, 66, 3, 172],seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 93, 101, 110, 118, 125],0,13)) by (compute);
        literal_decodes(z,raw,positions,0,13); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),12,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48],position:125}));
        assert(m::field_value(seq![120u8, 156, 99, 144, 106, 85, 83, 117, 96, 60, 177, 208, 98, 189, 1, 0, 19, 66, 3, 172],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 144, 106, 85, 83, 117, 96, 60, 177, 208, 98, 189, 1, 0, 19, 66, 3, 172],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),12,32768,true) == (m::StoredEffect::End{out:seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48],position:125}));
        assert(m::MFinish(seq![120u8, 156, 99, 144, 106, 85, 83, 117, 96, 60, 177, 208, 98, 189, 1, 0, 19, 66, 3, 172],125,seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48],12,false) == (m::StoredEffect::End{out:seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48],position:160})) by (compute);
        assert(m::Inflate(z,12,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,12,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 20) by (compute);
    }
    let ghost raw = seq![0u8, 26, 133, 38, 37, 64, 1, 200, 161, 56, 175, 48]; let mut output = [0xa5u8;12]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x3c, 0xc0, 0xc4, 0xc0, 0xcc, 0xc0, 0x32, 0x1, 0x0, 0xa, 0x82, 0x1, 0xdb];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 60, 192, 196, 192, 204, 192, 50, 1, 0, 10, 130, 1, 219]; let raw = seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 101, 108];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 60, 192, 196, 192, 204, 192, 50, 1, 0, 10, 130, 1, 219],seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144],seq![19nat, 27, 35, 43, 52, 60, 68, 76, 84, 92, 101, 108],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144],position:108}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 60, 192, 196, 192, 204, 192, 50, 1, 0, 10, 130, 1, 219],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 60, 192, 196, 192, 204, 192, 50, 1, 0, 10, 130, 1, 219],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144],position:108}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 60, 192, 196, 192, 204, 192, 50, 1, 0, 10, 130, 1, 219],108,seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144],10,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 192, 2, 0, 3, 0, 4, 144]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il1_7x5: 2 independently audited frozen streams; R=24.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x68, 0x60, 0x64, 0x60, 0xba, 0xc0, 0x3c, 0x81, 0xe5, 0x2, 0x43, 0x3a, 0x23, 0xf, 0x93, 0xd, 0xf3, 0xb, 0x16, 0x93, 0x1e, 0x86, 0xe3, 0x2e, 0x0, 0x45, 0x5, 0x6, 0x27];
    proof {
        let z = seq![120u8, 156, 99, 104, 96, 100, 96, 186, 192, 60, 129, 229, 2, 67, 58, 35, 15, 147, 13, 243, 11, 22, 147, 30, 134, 227, 46, 0, 69, 5, 6, 39]; let raw = seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 85, 93, 102, 110, 118, 126, 134, 142, 150, 158, 167, 175, 183, 191, 199, 208, 216, 223];
        assert(literal_words(seq![120u8, 156, 99, 104, 96, 100, 96, 186, 192, 60, 129, 229, 2, 67, 58, 35, 15, 147, 13, 243, 11, 22, 147, 30, 134, 227, 46, 0, 69, 5, 6, 39],seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 85, 93, 102, 110, 118, 126, 134, 142, 150, 158, 167, 175, 183, 191, 199, 208, 216, 223],0,25)) by (compute);
        literal_decodes(z,raw,positions,0,25); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),24,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68],position:223}));
        assert(m::field_value(seq![120u8, 156, 99, 104, 96, 100, 96, 186, 192, 60, 129, 229, 2, 67, 58, 35, 15, 147, 13, 243, 11, 22, 147, 30, 134, 227, 46, 0, 69, 5, 6, 39],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 104, 96, 100, 96, 186, 192, 60, 129, 229, 2, 67, 58, 35, 15, 147, 13, 243, 11, 22, 147, 30, 134, 227, 46, 0, 69, 5, 6, 39],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),24,32768,true) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68],position:223}));
        assert(m::MFinish(seq![120u8, 156, 99, 104, 96, 100, 96, 186, 192, 60, 129, 229, 2, 67, 58, 35, 15, 147, 13, 243, 11, 22, 147, 30, 134, 227, 46, 0, 69, 5, 6, 39],223,seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68],24,false) == (m::StoredEffect::End{out:seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68],position:256})) by (compute);
        assert(m::Inflate(z,24,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,24,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 32) by (compute);
    }
    let ghost raw = seq![0u8, 128, 1, 0, 2, 208, 3, 144, 4, 208, 0, 103, 1, 12, 2, 60, 3, 232, 4, 52, 140, 0, 199, 68]; let mut output = [0xa5u8;24]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il1_9x10: 1 independently audited frozen streams; R=52.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il1_9x10() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xd8, 0xc0, 0x78, 0x81, 0xe9, 0x0, 0xf3, 0x2, 0x16, 0xe, 0x6, 0x1, 0x46, 0x6, 0xa6, 0xb, 0xcc, 0xd5, 0xce, 0x2c, 0xb1, 0x19, 0xc, 0x73, 0x18, 0x77, 0x33, 0xed, 0x63, 0xd6, 0x67, 0x69, 0x61, 0xa8, 0x58, 0xc7, 0xc0, 0xb8, 0x65, 0xef, 0x79, 0x26, 0xd6, 0xf, 0xd, 0xcc, 0x6a, 0x2d, 0xcf, 0x58, 0x98, 0x2b, 0x84, 0x1, 0x76, 0x26, 0xf, 0x35];
    proof {
        let z = seq![120u8, 156, 99, 216, 192, 120, 129, 233, 0, 243, 2, 22, 14, 6, 1, 70, 6, 166, 11, 204, 213, 206, 44, 177, 25, 12, 115, 24, 119, 51, 237, 99, 214, 103, 105, 97, 168, 88, 199, 192, 184, 101, 239, 121, 38, 214, 15, 13, 204, 106, 45, 207, 88, 152, 43, 132, 1, 118, 38, 15, 53]; let raw = seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19]; let positions = seq![19nat, 27, 36, 44, 53, 61, 70, 78, 87, 95, 103, 111, 119, 127, 135, 143, 152, 160, 168, 176, 184, 192, 200, 208, 217, 225, 234, 242, 251, 259, 267, 275, 283, 291, 299, 308, 316, 324, 333, 342, 351, 359, 367, 376, 384, 392, 400, 408, 417, 425, 433, 441, 449, 456];
        assert(literal_words(seq![120u8, 156, 99, 216, 192, 120, 129, 233, 0, 243, 2, 22, 14, 6, 1, 70, 6, 166, 11, 204, 213, 206, 44, 177, 25, 12, 115, 24, 119, 51, 237, 99, 214, 103, 105, 97, 168, 88, 199, 192, 184, 101, 239, 121, 38, 214, 15, 13, 204, 106, 45, 207, 88, 152, 43, 132, 1, 118, 38, 15, 53],seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19],seq![19nat, 27, 36, 44, 53, 61, 70, 78, 87, 95, 103, 111, 119, 127, 135, 143, 152, 160, 168, 176, 184, 192, 200, 208, 217, 225, 234, 242, 251, 259, 267, 275, 283, 291, 299, 308, 316, 324, 333, 342, 351, 359, 367, 376, 384, 392, 400, 408, 417, 425, 433, 441, 449, 456],0,53)) by (compute);
        literal_decodes(z,raw,positions,0,53); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),52,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19],position:456}));
        assert(m::field_value(seq![120u8, 156, 99, 216, 192, 120, 129, 233, 0, 243, 2, 22, 14, 6, 1, 70, 6, 166, 11, 204, 213, 206, 44, 177, 25, 12, 115, 24, 119, 51, 237, 99, 214, 103, 105, 97, 168, 88, 199, 192, 184, 101, 239, 121, 38, 214, 15, 13, 204, 106, 45, 207, 88, 152, 43, 132, 1, 118, 38, 15, 53],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 216, 192, 120, 129, 233, 0, 243, 2, 22, 14, 6, 1, 70, 6, 166, 11, 204, 213, 206, 44, 177, 25, 12, 115, 24, 119, 51, 237, 99, 214, 103, 105, 97, 168, 88, 199, 192, 184, 101, 239, 121, 38, 214, 15, 13, 204, 106, 45, 207, 88, 152, 43, 132, 1, 118, 38, 15, 53],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),52,32768,true) == (m::StoredEffect::End{out:seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19],position:456}));
        assert(m::MFinish(seq![120u8, 156, 99, 216, 192, 120, 129, 233, 0, 243, 2, 22, 14, 6, 1, 70, 6, 166, 11, 204, 213, 206, 44, 177, 25, 12, 115, 24, 119, 51, 237, 99, 214, 103, 105, 97, 168, 88, 199, 192, 184, 101, 239, 121, 38, 214, 15, 13, 204, 106, 45, 207, 88, 152, 43, 132, 1, 118, 38, 15, 53],456,seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19],52,false) == (m::StoredEffect::End{out:seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19],position:488})) by (compute);
        assert(m::Inflate(z,52,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,52,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 61) by (compute);
    }
    let ghost raw = seq![0u8, 176, 1, 208, 2, 192, 3, 160, 4, 8, 0, 16, 1, 0, 2, 208, 3, 123, 67, 4, 93, 104, 0, 156, 1, 187, 2, 190, 3, 47, 4, 132, 0, 120, 174, 0, 1, 180, 189, 207, 2, 5, 240, 128, 3, 38, 132, 230, 4, 3, 120, 19]; let mut output = [0xa5u8;52]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d2_il1_17x2: 1 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct3_d2_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x48, 0x60, 0x74, 0x60, 0xfa, 0xcf, 0xcc, 0xb1, 0x90, 0xe5, 0x9b, 0xee, 0xdc, 0xbd, 0x87, 0x1, 0x20, 0xd8, 0x5, 0x93];
    proof {
        let z = seq![120u8, 156, 99, 72, 96, 116, 96, 250, 207, 204, 177, 144, 229, 155, 238, 220, 189, 135, 1, 32, 216, 5, 147]; let raw = seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 93, 101, 110, 118, 127, 136, 145, 152];
        assert(literal_words(seq![120u8, 156, 99, 72, 96, 116, 96, 250, 207, 204, 177, 144, 229, 155, 238, 220, 189, 135, 1, 32, 216, 5, 147],seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 93, 101, 110, 118, 127, 136, 145, 152],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195],position:152}));
        assert(m::field_value(seq![120u8, 156, 99, 72, 96, 116, 96, 250, 207, 204, 177, 144, 229, 155, 238, 220, 189, 135, 1, 32, 216, 5, 147],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 72, 96, 116, 96, 250, 207, 204, 177, 144, 229, 155, 238, 220, 189, 135, 1, 32, 216, 5, 147],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195],position:152}));
        assert(m::MFinish(seq![120u8, 156, 99, 72, 96, 116, 96, 250, 207, 204, 177, 144, 229, 155, 238, 220, 189, 135, 1, 32, 216, 5, 147],152,seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195],15,false) == (m::StoredEffect::End{out:seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195],position:184})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 23) by (compute);
    }
    let ghost raw = seq![0u8, 96, 1, 64, 2, 255, 3, 8, 161, 4, 246, 45, 157, 189, 195]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il0_2x3: 1 independently audited frozen streams; R=6.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf0, 0x67, 0x94, 0x61, 0x32, 0x2, 0x0, 0x2, 0x1f, 0x0, 0xa1];
    proof {
        let z = seq![120u8, 156, 99, 240, 103, 148, 97, 50, 2, 0, 2, 31, 0, 161]; let raw = seq![0u8, 79, 1, 28, 2, 50]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 74];
        assert(literal_words(seq![120u8, 156, 99, 240, 103, 148, 97, 50, 2, 0, 2, 31, 0, 161],seq![0u8, 79, 1, 28, 2, 50],seq![19nat, 27, 35, 43, 51, 59, 67, 74],0,7)) by (compute);
        literal_decodes(z,raw,positions,0,7); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),6,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 79, 1, 28, 2, 50],position:74}));
        assert(m::field_value(seq![120u8, 156, 99, 240, 103, 148, 97, 50, 2, 0, 2, 31, 0, 161],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 240, 103, 148, 97, 50, 2, 0, 2, 31, 0, 161],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),6,32768,true) == (m::StoredEffect::End{out:seq![0u8, 79, 1, 28, 2, 50],position:74}));
        assert(m::MFinish(seq![120u8, 156, 99, 240, 103, 148, 97, 50, 2, 0, 2, 31, 0, 161],74,seq![0u8, 79, 1, 28, 2, 50],6,false) == (m::StoredEffect::End{out:seq![0u8, 79, 1, 28, 2, 50],position:112})) by (compute);
        assert(m::Inflate(z,6,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,6,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 14) by (compute);
    }
    let ghost raw = seq![0u8, 79, 1, 28, 2, 50]; let mut output = [0xa5u8;6]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il0_7x5: 2 independently audited frozen streams; R=25.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il0_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xb8, 0xa7, 0x7d, 0xc8, 0x81, 0xf1, 0xcb, 0x93, 0xb3, 0xd2, 0x4c, 0xda, 0x25, 0x57, 0xe, 0x30, 0xbf, 0x71, 0x77, 0x59, 0xc6, 0xc2, 0x39, 0x43, 0xb3, 0x1, 0x0, 0x8d, 0xf7, 0xa, 0x70];
    proof {
        let z = seq![120u8, 156, 99, 184, 167, 125, 200, 129, 241, 203, 147, 179, 210, 76, 218, 37, 87, 14, 48, 191, 113, 119, 89, 198, 194, 57, 67, 179, 1, 0, 141, 247, 10, 112]; let raw = seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128]; let positions = seq![19nat, 27, 36, 44, 53, 61, 69, 78, 87, 96, 104, 112, 120, 128, 137, 146, 154, 163, 171, 179, 188, 196, 204, 213, 221, 229, 236];
        assert(literal_words(seq![120u8, 156, 99, 184, 167, 125, 200, 129, 241, 203, 147, 179, 210, 76, 218, 37, 87, 14, 48, 191, 113, 119, 89, 198, 194, 57, 67, 179, 1, 0, 141, 247, 10, 112],seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128],seq![19nat, 27, 36, 44, 53, 61, 69, 78, 87, 96, 104, 112, 120, 128, 137, 146, 154, 163, 171, 179, 188, 196, 204, 213, 221, 229, 236],0,26)) by (compute);
        literal_decodes(z,raw,positions,0,26); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),25,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128],position:236}));
        assert(m::field_value(seq![120u8, 156, 99, 184, 167, 125, 200, 129, 241, 203, 147, 179, 210, 76, 218, 37, 87, 14, 48, 191, 113, 119, 89, 198, 194, 57, 67, 179, 1, 0, 141, 247, 10, 112],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 184, 167, 125, 200, 129, 241, 203, 147, 179, 210, 76, 218, 37, 87, 14, 48, 191, 113, 119, 89, 198, 194, 57, 67, 179, 1, 0, 141, 247, 10, 112],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),25,32768,true) == (m::StoredEffect::End{out:seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128],position:236}));
        assert(m::MFinish(seq![120u8, 156, 99, 184, 167, 125, 200, 129, 241, 203, 147, 179, 210, 76, 218, 37, 87, 14, 48, 191, 113, 119, 89, 198, 194, 57, 67, 179, 1, 0, 141, 247, 10, 112],236,seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128],25,false) == (m::StoredEffect::End{out:seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128],position:272})) by (compute);
        assert(m::Inflate(z,25,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,25,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 34) by (compute);
    }
    let ghost raw = seq![0u8, 222, 43, 194, 64, 1, 244, 228, 205, 27, 2, 43, 116, 212, 192, 3, 236, 71, 68, 166, 4, 9, 152, 41, 128]; let mut output = [0xa5u8;25]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il0_9x10: 1 frozen stored streams; R=60.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il0_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x3c, 0x0, 0xc3, 0xff, 0x0, 0xef, 0xb5, 0x44, 0x8d, 0xd0, 0x1, 0x86, 0x12, 0xee, 0x41, 0xf9, 0x2, 0x8e, 0x2, 0x62, 0x15, 0x70, 0x3, 0xe3, 0x36, 0xe0, 0x99, 0xa1, 0x4, 0x96, 0x24, 0x63, 0xac, 0x10, 0x0, 0x33, 0xa1, 0x92, 0xfc, 0x40, 0x1, 0xbc, 0xac, 0x61, 0x47, 0x90, 0x2, 0x57, 0x7a, 0x43, 0xe7, 0xf0, 0x3, 0x8d, 0xa7, 0xe4, 0xe2, 0xf3, 0x4, 0xe5, 0xa1, 0xa3, 0xc9, 0x30, 0x1a, 0x84, 0x1b, 0xd9];
    proof {
        let z = seq![120u8, 156, 1, 60, 0, 195, 255, 0, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48, 26, 132, 27, 217]; let raw = seq![0u8, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48];
        assert(z.subrange(7,67) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48, 26, 132, 27, 217],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48, 26, 132, 27, 217],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),60) == (m::StoredEffect::End{out:raw,position:536}));
        assert(m::MOneBlock(z,16,Seq::empty(),60,32768,true) == (true,m::StoredEffect::End{out:raw,position:536}));
        assert(m::MBlk(z,16,Seq::empty(),60,32768,true) == (m::StoredEffect::End{out:raw,position:536}));
        assert(m::MFinish(seq![120u8, 156, 1, 60, 0, 195, 255, 0, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48, 26, 132, 27, 217],536,seq![0u8, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48],60,false) == (m::StoredEffect::End{out:seq![0u8, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48],position:568})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,60,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,60,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 71) by (compute);
    }
    let ghost raw = seq![0u8, 239, 181, 68, 141, 208, 1, 134, 18, 238, 65, 249, 2, 142, 2, 98, 21, 112, 3, 227, 54, 224, 153, 161, 4, 150, 36, 99, 172, 16, 0, 51, 161, 146, 252, 64, 1, 188, 172, 97, 71, 144, 2, 87, 122, 67, 231, 240, 3, 141, 167, 228, 226, 243, 4, 229, 161, 163, 201, 48]; let mut output = [0xa5u8;60]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il0_17x2: 1 independently audited frozen streams; R=20.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x98, 0x55, 0xf1, 0x32, 0x46, 0x47, 0xf4, 0x48, 0xab, 0x0, 0x63, 0xc4, 0xa3, 0x86, 0x77, 0x9, 0x71, 0x31, 0x2c, 0x52, 0x0, 0x55, 0x99, 0x7, 0xd3];
    proof {
        let z = seq![120u8, 156, 99, 152, 85, 241, 50, 70, 71, 244, 72, 171, 0, 99, 196, 163, 134, 119, 9, 113, 49, 44, 82, 0, 85, 153, 7, 211]; let raw = seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26]; let positions = seq![19nat, 27, 36, 44, 53, 61, 69, 77, 86, 94, 102, 110, 118, 127, 135, 144, 152, 160, 168, 176, 184, 191];
        assert(literal_words(seq![120u8, 156, 99, 152, 85, 241, 50, 70, 71, 244, 72, 171, 0, 99, 196, 163, 134, 119, 9, 113, 49, 44, 82, 0, 85, 153, 7, 211],seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26],seq![19nat, 27, 36, 44, 53, 61, 69, 77, 86, 94, 102, 110, 118, 127, 135, 144, 152, 160, 168, 176, 184, 191],0,21)) by (compute);
        literal_decodes(z,raw,positions,0,21); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),20,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26],position:191}));
        assert(m::field_value(seq![120u8, 156, 99, 152, 85, 241, 50, 70, 71, 244, 72, 171, 0, 99, 196, 163, 134, 119, 9, 113, 49, 44, 82, 0, 85, 153, 7, 211],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 152, 85, 241, 50, 70, 71, 244, 72, 171, 0, 99, 196, 163, 134, 119, 9, 113, 49, 44, 82, 0, 85, 153, 7, 211],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),20,32768,true) == (m::StoredEffect::End{out:seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26],position:191}));
        assert(m::MFinish(seq![120u8, 156, 99, 152, 85, 241, 50, 70, 71, 244, 72, 171, 0, 99, 196, 163, 134, 119, 9, 113, 49, 44, 82, 0, 85, 153, 7, 211],191,seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26],20,false) == (m::StoredEffect::End{out:seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26],position:224})) by (compute);
        assert(m::Inflate(z,20,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,20,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 28) by (compute);
    }
    let ghost raw = seq![0u8, 154, 120, 233, 92, 44, 21, 196, 133, 16, 1, 88, 226, 128, 238, 96, 94, 92, 4, 26]; let mut output = [0xa5u8;20]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il1_2x3: 1 independently audited frozen streams; R=10.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0x60, 0x54, 0x60, 0x6a, 0x60, 0xbe, 0xc0, 0x22, 0x5, 0x0, 0x7, 0x3c, 0x1, 0xb5];
    proof {
        let z = seq![120u8, 156, 99, 80, 96, 84, 96, 106, 96, 190, 192, 34, 5, 0, 7, 60, 1, 181]; let raw = seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 107];
        assert(literal_words(seq![120u8, 156, 99, 80, 96, 84, 96, 106, 96, 190, 192, 34, 5, 0, 7, 60, 1, 181],seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 107],0,11)) by (compute);
        literal_decodes(z,raw,positions,0,11); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),10,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26],position:107}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 96, 84, 96, 106, 96, 190, 192, 34, 5, 0, 7, 60, 1, 181],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 96, 84, 96, 106, 96, 190, 192, 34, 5, 0, 7, 60, 1, 181],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),10,32768,true) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26],position:107}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 96, 84, 96, 106, 96, 190, 192, 34, 5, 0, 7, 60, 1, 181],107,seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26],10,false) == (m::StoredEffect::End{out:seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26],position:144})) by (compute);
        assert(m::Inflate(z,10,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,10,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 18) by (compute);
    }
    let ghost raw = seq![0u8, 32, 1, 32, 2, 128, 3, 208, 4, 26]; let mut output = [0xa5u8;10]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il1_7x5: 2 independently audited frozen streams; R=32.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x70, 0x60, 0x14, 0x60, 0xca, 0x66, 0x3e, 0xc5, 0x12, 0xc2, 0x30, 0x75, 0xb, 0xe3, 0x8a, 0x19, 0x4c, 0xae, 0x1, 0xcc, 0x72, 0xf3, 0x58, 0x18, 0xd8, 0x32, 0x37, 0x32, 0xb8, 0xe8, 0x8a, 0x5, 0x0, 0x0, 0x7b, 0xdb, 0x7, 0xbf];
    proof {
        let z = seq![120u8, 156, 99, 112, 96, 20, 96, 202, 102, 62, 197, 18, 194, 48, 117, 11, 227, 138, 25, 76, 174, 1, 204, 114, 243, 88, 24, 216, 50, 55, 50, 184, 232, 138, 5, 0, 0, 123, 219, 7, 191]; let raw = seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 126, 134, 143, 152, 160, 168, 176, 184, 192, 201, 209, 217, 225, 233, 242, 250, 258, 266, 274, 282, 289];
        assert(literal_words(seq![120u8, 156, 99, 112, 96, 20, 96, 202, 102, 62, 197, 18, 194, 48, 117, 11, 227, 138, 25, 76, 174, 1, 204, 114, 243, 88, 24, 216, 50, 55, 50, 184, 232, 138, 5, 0, 0, 123, 219, 7, 191],seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 117, 126, 134, 143, 152, 160, 168, 176, 184, 192, 201, 209, 217, 225, 233, 242, 250, 258, 266, 274, 282, 289],0,33)) by (compute);
        literal_decodes(z,raw,positions,0,33); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),32,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80],position:289}));
        assert(m::field_value(seq![120u8, 156, 99, 112, 96, 20, 96, 202, 102, 62, 197, 18, 194, 48, 117, 11, 227, 138, 25, 76, 174, 1, 204, 114, 243, 88, 24, 216, 50, 55, 50, 184, 232, 138, 5, 0, 0, 123, 219, 7, 191],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 112, 96, 20, 96, 202, 102, 62, 197, 18, 194, 48, 117, 11, 227, 138, 25, 76, 174, 1, 204, 114, 243, 88, 24, 216, 50, 55, 50, 184, 232, 138, 5, 0, 0, 123, 219, 7, 191],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),32,32768,true) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80],position:289}));
        assert(m::MFinish(seq![120u8, 156, 99, 112, 96, 20, 96, 202, 102, 62, 197, 18, 194, 48, 117, 11, 227, 138, 25, 76, 174, 1, 204, 114, 243, 88, 24, 216, 50, 55, 50, 184, 232, 138, 5, 0, 0, 123, 219, 7, 191],289,seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80],32,false) == (m::StoredEffect::End{out:seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80],position:328})) by (compute);
        assert(m::Inflate(z,32,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,32,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 41) by (compute);
    }
    let ghost raw = seq![0u8, 64, 1, 16, 2, 107, 3, 202, 4, 84, 0, 149, 180, 1, 168, 152, 2, 69, 80, 3, 30, 158, 4, 0, 6, 105, 177, 0, 68, 45, 22, 80]; let mut output = [0xa5u8;32]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il1_9x10: 1 frozen stored streams; R=70.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il1_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x46, 0x0, 0xb9, 0xff, 0x0, 0x35, 0x1, 0x67, 0x2, 0xa0, 0x3, 0x80, 0x4, 0xe1, 0x5f, 0x0, 0x34, 0x1, 0xb7, 0x2, 0xa1, 0x3, 0xbb, 0x50, 0xea, 0x4, 0xbc, 0x53, 0xf0, 0x0, 0x1, 0x94, 0x1, 0xcc, 0x29, 0x2, 0x10, 0x36, 0x3, 0xf8, 0x74, 0x4, 0x64, 0x80, 0x0, 0xa0, 0xaa, 0x69, 0x32, 0x80, 0x1, 0x8f, 0x1f, 0x28, 0xbf, 0x3b, 0x2, 0xc3, 0x16, 0x65, 0xa7, 0xc0, 0x3, 0x66, 0x32, 0xd9, 0x63, 0x8f, 0x4, 0xc4, 0x7a, 0xfd, 0x9, 0x90, 0x1f, 0x9d, 0x18, 0xd6];
    proof {
        let z = seq![120u8, 156, 1, 70, 0, 185, 255, 0, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144, 31, 157, 24, 214]; let raw = seq![0u8, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144];
        assert(z.subrange(7,77) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144, 31, 157, 24, 214],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144, 31, 157, 24, 214],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),70) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MOneBlock(z,16,Seq::empty(),70,32768,true) == (true,m::StoredEffect::End{out:raw,position:616}));
        assert(m::MBlk(z,16,Seq::empty(),70,32768,true) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MFinish(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144, 31, 157, 24, 214],616,seq![0u8, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144],70,false) == (m::StoredEffect::End{out:seq![0u8, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144],position:648})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,70,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,70,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 81) by (compute);
    }
    let ghost raw = seq![0u8, 53, 1, 103, 2, 160, 3, 128, 4, 225, 95, 0, 52, 1, 183, 2, 161, 3, 187, 80, 234, 4, 188, 83, 240, 0, 1, 148, 1, 204, 41, 2, 16, 54, 3, 248, 116, 4, 100, 128, 0, 160, 170, 105, 50, 128, 1, 143, 31, 40, 191, 59, 2, 195, 22, 101, 167, 192, 3, 102, 50, 217, 99, 143, 4, 196, 122, 253, 9, 144]; let mut output = [0xa5u8;70]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d4_il1_17x2: 1 independently audited frozen streams; R=23.
#[verifier::spinoff_prover]
fn intent_ct3_d4_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xe8, 0x11, 0x60, 0x6c, 0x61, 0x72, 0x5d, 0xc6, 0x5c, 0xe2, 0x98, 0x34, 0x87, 0xe5, 0x4f, 0x8c, 0x92, 0xcf, 0xac, 0x6b, 0xbe, 0x77, 0x5, 0x0, 0x53, 0x2b, 0x8, 0x39];
    proof {
        let z = seq![120u8, 156, 99, 232, 17, 96, 108, 97, 114, 93, 198, 92, 226, 152, 52, 135, 229, 79, 140, 146, 207, 172, 107, 190, 119, 5, 0, 83, 43, 8, 57]; let raw = seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 116, 125, 133, 142, 150, 158, 166, 175, 184, 192, 201, 209, 216];
        assert(literal_words(seq![120u8, 156, 99, 232, 17, 96, 108, 97, 114, 93, 198, 92, 226, 152, 52, 135, 229, 79, 140, 146, 207, 172, 107, 190, 119, 5, 0, 83, 43, 8, 57],seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 84, 92, 100, 108, 116, 125, 133, 142, 150, 158, 166, 175, 184, 192, 201, 209, 216],0,24)) by (compute);
        literal_decodes(z,raw,positions,0,24); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),23,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16],position:216}));
        assert(m::field_value(seq![120u8, 156, 99, 232, 17, 96, 108, 97, 114, 93, 198, 92, 226, 152, 52, 135, 229, 79, 140, 146, 207, 172, 107, 190, 119, 5, 0, 83, 43, 8, 57],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 232, 17, 96, 108, 97, 114, 93, 198, 92, 226, 152, 52, 135, 229, 79, 140, 146, 207, 172, 107, 190, 119, 5, 0, 83, 43, 8, 57],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),23,32768,true) == (m::StoredEffect::End{out:seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16],position:216}));
        assert(m::MFinish(seq![120u8, 156, 99, 232, 17, 96, 108, 97, 114, 93, 198, 92, 226, 152, 52, 135, 229, 79, 140, 146, 207, 172, 107, 190, 119, 5, 0, 83, 43, 8, 57],216,seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16],23,false) == (m::StoredEffect::End{out:seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16],position:248})) by (compute);
        assert(m::Inflate(z,23,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,23,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 31) by (compute);
    }
    let ghost raw = seq![0u8, 140, 16, 1, 132, 2, 69, 166, 3, 116, 65, 98, 156, 4, 252, 92, 34, 76, 154, 214, 77, 221, 16]; let mut output = [0xa5u8;23]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il0_1x1: 1 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x10, 0x5, 0x0, 0x0, 0x17, 0x0, 0x16];
    proof {
        let z = seq![120u8, 156, 99, 16, 5, 0, 0, 23, 0, 22]; let raw = seq![0u8, 21]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 16, 5, 0, 0, 23, 0, 22],seq![0u8, 21],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 21],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 16, 5, 0, 0, 23, 0, 22],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 16, 5, 0, 0, 23, 0, 22],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 21],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 16, 5, 0, 0, 23, 0, 22],42,seq![0u8, 21],2,false) == (m::StoredEffect::End{out:seq![0u8, 21],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 21]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il0_2x3: 1 independently audited frozen streams; R=9.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x28, 0x67, 0x67, 0xf4, 0xbb, 0xc0, 0x14, 0xea, 0x1, 0x0, 0x9, 0xb6, 0x2, 0x3d];
    proof {
        let z = seq![120u8, 156, 99, 40, 103, 103, 244, 187, 192, 20, 234, 1, 0, 9, 182, 2, 61]; let raw = seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72]; let positions = seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 92, 99];
        assert(literal_words(seq![120u8, 156, 99, 40, 103, 103, 244, 187, 192, 20, 234, 1, 0, 9, 182, 2, 61],seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72],seq![19nat, 27, 35, 43, 51, 59, 68, 76, 84, 92, 99],0,10)) by (compute);
        literal_decodes(z,raw,positions,0,10); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),9,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72],position:99}));
        assert(m::field_value(seq![120u8, 156, 99, 40, 103, 103, 244, 187, 192, 20, 234, 1, 0, 9, 182, 2, 61],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 40, 103, 103, 244, 187, 192, 20, 234, 1, 0, 9, 182, 2, 61],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),9,32768,true) == (m::StoredEffect::End{out:seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72],position:99}));
        assert(m::MFinish(seq![120u8, 156, 99, 40, 103, 103, 244, 187, 192, 20, 234, 1, 0, 9, 182, 2, 61],99,seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72],9,false) == (m::StoredEffect::End{out:seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72],position:136})) by (compute);
        assert(m::Inflate(z,9,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,9,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 17) by (compute);
    }
    let ghost raw = seq![0u8, 119, 7, 1, 78, 208, 2, 85, 72]; let mut output = [0xa5u8;9]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il0_7x5: 2 frozen stored streams; R=40.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il0_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x28, 0x0, 0xd7, 0xff, 0x0, 0xe9, 0xdd, 0x4, 0xbf, 0xf0, 0x72, 0xd5, 0x1, 0x19, 0x39, 0x2b, 0x63, 0xfa, 0xa7, 0x35, 0x2, 0x50, 0x11, 0xf1, 0xc8, 0x44, 0x42, 0xfe, 0x3, 0xb6, 0xfa, 0xb9, 0x4b, 0xd2, 0x11, 0xd3, 0x4, 0x1f, 0xd3, 0x7b, 0x14, 0x1a, 0x34, 0x35, 0x7c, 0x46, 0x11, 0x8d];
    proof {
        let z = seq![120u8, 156, 1, 40, 0, 215, 255, 0, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53, 124, 70, 17, 141]; let raw = seq![0u8, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53];
        assert(z.subrange(7,47) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53, 124, 70, 17, 141],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53, 124, 70, 17, 141],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),40) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MOneBlock(z,16,Seq::empty(),40,32768,true) == (true,m::StoredEffect::End{out:raw,position:376}));
        assert(m::MBlk(z,16,Seq::empty(),40,32768,true) == (m::StoredEffect::End{out:raw,position:376}));
        assert(m::MFinish(seq![120u8, 156, 1, 40, 0, 215, 255, 0, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53, 124, 70, 17, 141],376,seq![0u8, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53],40,false) == (m::StoredEffect::End{out:seq![0u8, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53],position:408})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,40,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,40,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 51) by (compute);
    }
    let ghost raw = seq![0u8, 233, 221, 4, 191, 240, 114, 213, 1, 25, 57, 43, 99, 250, 167, 53, 2, 80, 17, 241, 200, 68, 66, 254, 3, 182, 250, 185, 75, 210, 17, 211, 4, 31, 211, 123, 20, 26, 52, 53]; let mut output = [0xa5u8;40]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il0_9x10: 1 frozen stored streams; R=100.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il0_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x64, 0x0, 0x9b, 0xff, 0x0, 0x5c, 0x21, 0x32, 0x7, 0x4f, 0x5a, 0x22, 0x2e, 0x44, 0x1, 0x47, 0xbc, 0x44, 0x2, 0xc5, 0x44, 0xfe, 0x6, 0xf8, 0x2, 0xba, 0x44, 0xbe, 0xd, 0xb, 0xd6, 0xe1, 0xfb, 0xc7, 0x3, 0x35, 0xd, 0x34, 0xf6, 0xe6, 0x17, 0xd4, 0x5, 0xed, 0x4, 0xed, 0x20, 0xa5, 0x27, 0x42, 0xe5, 0x44, 0x7, 0xf4, 0x0, 0x49, 0x54, 0x1d, 0x4b, 0xc, 0x43, 0x31, 0x4c, 0x45, 0x1, 0xf, 0x4a, 0xc2, 0x19, 0x1c, 0xb6, 0x15, 0x38, 0xf6, 0x2, 0x28, 0xde, 0x17, 0x5, 0xfd, 0x2a, 0x5, 0xbc, 0xc, 0x3, 0x1e, 0x1a, 0xb, 0xc0, 0x9, 0x22, 0xf, 0xfd, 0xd0, 0x4, 0xdf, 0xff, 0xea, 0x2e, 0xd0, 0xd2, 0x4e, 0x7, 0x37, 0xe, 0x79, 0x24, 0xb4];
    proof {
        let z = seq![120u8, 156, 1, 100, 0, 155, 255, 0, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55, 14, 121, 36, 180]; let raw = seq![0u8, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55];
        assert(z.subrange(7,107) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55, 14, 121, 36, 180],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55, 14, 121, 36, 180],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),100) == (m::StoredEffect::End{out:raw,position:856}));
        assert(m::MOneBlock(z,16,Seq::empty(),100,32768,true) == (true,m::StoredEffect::End{out:raw,position:856}));
        assert(m::MBlk(z,16,Seq::empty(),100,32768,true) == (m::StoredEffect::End{out:raw,position:856}));
        assert(m::MFinish(seq![120u8, 156, 1, 100, 0, 155, 255, 0, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55, 14, 121, 36, 180],856,seq![0u8, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55],100,false) == (m::StoredEffect::End{out:seq![0u8, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55],position:888})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,100,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,100,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 111) by (compute);
    }
    let ghost raw = seq![0u8, 92, 33, 50, 7, 79, 90, 34, 46, 68, 1, 71, 188, 68, 2, 197, 68, 254, 6, 248, 2, 186, 68, 190, 13, 11, 214, 225, 251, 199, 3, 53, 13, 52, 246, 230, 23, 212, 5, 237, 4, 237, 32, 165, 39, 66, 229, 68, 7, 244, 0, 73, 84, 29, 75, 12, 67, 49, 76, 69, 1, 15, 74, 194, 25, 28, 182, 21, 56, 246, 2, 40, 222, 23, 5, 253, 42, 5, 188, 12, 3, 30, 26, 11, 192, 9, 34, 15, 253, 208, 4, 223, 255, 234, 46, 208, 210, 78, 7, 55]; let mut output = [0xa5u8;100]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il0_17x2: 1 independently audited frozen streams; R=36.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il0_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xa8, 0xf3, 0x98, 0xb2, 0x48, 0x3b, 0x6d, 0xb6, 0xb5, 0xfc, 0xe2, 0xf2, 0x4a, 0x9d, 0xea, 0xc0, 0xc9, 0x3a, 0x8c, 0x1b, 0xa7, 0xbb, 0x2e, 0x63, 0x36, 0xb1, 0xfe, 0xb0, 0xe1, 0xf4, 0xb7, 0xe2, 0x17, 0x12, 0xec, 0x7a, 0xb5, 0x0, 0x6, 0xde, 0xe, 0xf9];
    proof {
        let z = seq![120u8, 156, 99, 168, 243, 152, 178, 72, 59, 109, 182, 181, 252, 226, 242, 74, 157, 234, 192, 201, 58, 140, 27, 167, 187, 46, 99, 54, 177, 254, 176, 225, 244, 183, 226, 23, 18, 236, 122, 181, 0, 6, 222, 14, 249]; let raw = seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125]; let positions = seq![19nat, 27, 35, 43, 52, 61, 69, 77, 86, 94, 102, 111, 119, 127, 135, 143, 151, 160, 168, 176, 185, 194, 202, 211, 219, 227, 235, 244, 253, 262, 271, 279, 288, 296, 304, 312, 320, 327];
        assert(literal_words(seq![120u8, 156, 99, 168, 243, 152, 178, 72, 59, 109, 182, 181, 252, 226, 242, 74, 157, 234, 192, 201, 58, 140, 27, 167, 187, 46, 99, 54, 177, 254, 176, 225, 244, 183, 226, 23, 18, 236, 122, 181, 0, 6, 222, 14, 249],seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125],seq![19nat, 27, 35, 43, 52, 61, 69, 77, 86, 94, 102, 111, 119, 127, 135, 143, 151, 160, 168, 176, 185, 194, 202, 211, 219, 227, 235, 244, 253, 262, 271, 279, 288, 296, 304, 312, 320, 327],0,37)) by (compute);
        literal_decodes(z,raw,positions,0,37); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),36,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125],position:327}));
        assert(m::field_value(seq![120u8, 156, 99, 168, 243, 152, 178, 72, 59, 109, 182, 181, 252, 226, 242, 74, 157, 234, 192, 201, 58, 140, 27, 167, 187, 46, 99, 54, 177, 254, 176, 225, 244, 183, 226, 23, 18, 236, 122, 181, 0, 6, 222, 14, 249],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 168, 243, 152, 178, 72, 59, 109, 182, 181, 252, 226, 242, 74, 157, 234, 192, 201, 58, 140, 27, 167, 187, 46, 99, 54, 177, 254, 176, 225, 244, 183, 226, 23, 18, 236, 122, 181, 0, 6, 222, 14, 249],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),36,32768,true) == (m::StoredEffect::End{out:seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125],position:327}));
        assert(m::MFinish(seq![120u8, 156, 99, 168, 243, 152, 178, 72, 59, 109, 182, 181, 252, 226, 242, 74, 157, 234, 192, 201, 58, 140, 27, 167, 187, 46, 99, 54, 177, 254, 176, 225, 244, 183, 226, 23, 18, 236, 122, 181, 0, 6, 222, 14, 249],327,seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125],36,false) == (m::StoredEffect::End{out:seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125],position:360})) by (compute);
        assert(m::Inflate(z,36,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,36,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 45) by (compute);
    }
    let ghost raw = seq![0u8, 126, 72, 148, 162, 43, 102, 155, 59, 31, 163, 119, 121, 44, 123, 81, 147, 44, 1, 177, 151, 69, 166, 3, 52, 59, 240, 176, 203, 246, 115, 232, 24, 7, 46, 125]; let mut output = [0xa5u8;36]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il1_1x1: 1 independently audited frozen streams; R=2.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x50, 0x7, 0x0, 0x0, 0x29, 0x0, 0x28];
    proof {
        let z = seq![120u8, 156, 99, 80, 7, 0, 0, 41, 0, 40]; let raw = seq![0u8, 39]; let positions = seq![19nat, 27, 35, 42];
        assert(literal_words(seq![120u8, 156, 99, 80, 7, 0, 0, 41, 0, 40],seq![0u8, 39],seq![19nat, 27, 35, 42],0,3)) by (compute);
        literal_decodes(z,raw,positions,0,3); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),2,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 39],position:42}));
        assert(m::field_value(seq![120u8, 156, 99, 80, 7, 0, 0, 41, 0, 40],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 80, 7, 0, 0, 41, 0, 40],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),2,32768,true) == (m::StoredEffect::End{out:seq![0u8, 39],position:42}));
        assert(m::MFinish(seq![120u8, 156, 99, 80, 7, 0, 0, 41, 0, 40],42,seq![0u8, 39],2,false) == (m::StoredEffect::End{out:seq![0u8, 39],position:80})) by (compute);
        assert(m::Inflate(z,2,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,2,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 10) by (compute);
    }
    let ghost raw = seq![0u8, 39]; let mut output = [0xa5u8;2]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il1_2x3: 1 independently audited frozen streams; R=11.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x90, 0x63, 0xf4, 0x60, 0x32, 0x65, 0xb6, 0x60, 0x49, 0xdf, 0x3, 0x0, 0x7, 0x51, 0x2, 0x1];
    proof {
        let z = seq![120u8, 156, 99, 144, 99, 244, 96, 50, 101, 182, 96, 73, 223, 3, 0, 7, 81, 2, 1]; let raw = seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188]; let positions = seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 99, 108, 115];
        assert(literal_words(seq![120u8, 156, 99, 144, 99, 244, 96, 50, 101, 182, 96, 73, 223, 3, 0, 7, 81, 2, 1],seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188],seq![19nat, 27, 35, 43, 51, 59, 67, 75, 83, 91, 99, 108, 115],0,12)) by (compute);
        literal_decodes(z,raw,positions,0,12); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),11,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188],position:115}));
        assert(m::field_value(seq![120u8, 156, 99, 144, 99, 244, 96, 50, 101, 182, 96, 73, 223, 3, 0, 7, 81, 2, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 144, 99, 244, 96, 50, 101, 182, 96, 73, 223, 3, 0, 7, 81, 2, 1],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),11,32768,true) == (m::StoredEffect::End{out:seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188],position:115}));
        assert(m::MFinish(seq![120u8, 156, 99, 144, 99, 244, 96, 50, 101, 182, 96, 73, 223, 3, 0, 7, 81, 2, 1],115,seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188],11,false) == (m::StoredEffect::End{out:seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188],position:152})) by (compute);
        assert(m::Inflate(z,11,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,11,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 19) by (compute);
    }
    let ghost raw = seq![0u8, 30, 1, 72, 2, 53, 3, 56, 4, 103, 188]; let mut output = [0xa5u8;11]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il1_7x5: 2 independently audited frozen streams; R=46.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il1_7x5() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x98, 0xcf, 0xe8, 0xc3, 0x64, 0xa6, 0xcc, 0xec, 0x6d, 0xcc, 0x12, 0x38, 0x8d, 0x21, 0x7e, 0x75, 0x8d, 0x26, 0x63, 0xcf, 0xcc, 0x97, 0x4c, 0x4d, 0x4f, 0x13, 0x98, 0xd5, 0xca, 0x2f, 0xb2, 0xe8, 0x7c, 0x95, 0xf8, 0x72, 0x3b, 0xd4, 0x9c, 0xc1, 0x9f, 0xcd, 0x60, 0x91, 0x55, 0xaa, 0x0, 0x0, 0x46, 0x96, 0xf, 0x1a];
    proof {
        let z = seq![120u8, 156, 99, 152, 207, 232, 195, 100, 166, 204, 236, 109, 204, 18, 56, 141, 33, 126, 117, 141, 38, 99, 207, 204, 151, 76, 77, 79, 19, 152, 213, 202, 47, 178, 232, 124, 149, 248, 114, 59, 212, 156, 193, 159, 205, 96, 145, 85, 170, 0, 0, 70, 150, 15, 26]; let raw = seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16]; let positions = seq![19nat, 27, 36, 44, 52, 60, 68, 76, 84, 92, 100, 108, 116, 125, 133, 141, 150, 158, 166, 174, 182, 191, 200, 208, 216, 225, 233, 241, 249, 257, 266, 274, 282, 291, 299, 308, 317, 325, 333, 341, 349, 357, 365, 374, 382, 390, 398, 405];
        assert(literal_words(seq![120u8, 156, 99, 152, 207, 232, 195, 100, 166, 204, 236, 109, 204, 18, 56, 141, 33, 126, 117, 141, 38, 99, 207, 204, 151, 76, 77, 79, 19, 152, 213, 202, 47, 178, 232, 124, 149, 248, 114, 59, 212, 156, 193, 159, 205, 96, 145, 85, 170, 0, 0, 70, 150, 15, 26],seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16],seq![19nat, 27, 36, 44, 52, 60, 68, 76, 84, 92, 100, 108, 116, 125, 133, 141, 150, 158, 166, 174, 182, 191, 200, 208, 216, 225, 233, 241, 249, 257, 266, 274, 282, 291, 299, 308, 317, 325, 333, 341, 349, 357, 365, 374, 382, 390, 398, 405],0,47)) by (compute);
        literal_decodes(z,raw,positions,0,47); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),46,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16],position:405}));
        assert(m::field_value(seq![120u8, 156, 99, 152, 207, 232, 195, 100, 166, 204, 236, 109, 204, 18, 56, 141, 33, 126, 117, 141, 38, 99, 207, 204, 151, 76, 77, 79, 19, 152, 213, 202, 47, 178, 232, 124, 149, 248, 114, 59, 212, 156, 193, 159, 205, 96, 145, 85, 170, 0, 0, 70, 150, 15, 26],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 152, 207, 232, 195, 100, 166, 204, 236, 109, 204, 18, 56, 141, 33, 126, 117, 141, 38, 99, 207, 204, 151, 76, 77, 79, 19, 152, 213, 202, 47, 178, 232, 124, 149, 248, 114, 59, 212, 156, 193, 159, 205, 96, 145, 85, 170, 0, 0, 70, 150, 15, 26],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),46,32768,true) == (m::StoredEffect::End{out:seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16],position:405}));
        assert(m::MFinish(seq![120u8, 156, 99, 152, 207, 232, 195, 100, 166, 204, 236, 109, 204, 18, 56, 141, 33, 126, 117, 141, 38, 99, 207, 204, 151, 76, 77, 79, 19, 152, 213, 202, 47, 178, 232, 124, 149, 248, 114, 59, 212, 156, 193, 159, 205, 96, 145, 85, 170, 0, 0, 70, 150, 15, 26],405,seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16],46,false) == (m::StoredEffect::End{out:seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16],position:440})) by (compute);
        assert(m::Inflate(z,46,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,46,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 55) by (compute);
    }
    let ghost raw = seq![0u8, 159, 1, 76, 2, 54, 35, 3, 75, 51, 4, 81, 150, 0, 95, 171, 124, 41, 1, 140, 153, 233, 2, 130, 229, 96, 3, 38, 119, 209, 4, 44, 245, 24, 244, 219, 85, 55, 0, 79, 6, 48, 162, 58, 101, 16]; let mut output = [0xa5u8;46]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il1_9x10: 1 frozen stored streams; R=110.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il1_9x10() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x6e, 0x0, 0x91, 0xff, 0x0, 0x42, 0x20, 0x1, 0xd, 0xf5, 0x2, 0x3c, 0x3, 0xfa, 0x4, 0x28, 0xea, 0x23, 0x0, 0xb, 0x3c, 0x1, 0x41, 0xfa, 0x2, 0x4, 0xd7, 0x3, 0x2b, 0x26, 0xb, 0x19, 0x11, 0x4, 0xfd, 0xdb, 0x33, 0xcf, 0x32, 0x0, 0x2f, 0x35, 0x19, 0x31, 0x1, 0x4a, 0xdd, 0xf0, 0x2a, 0x2, 0xdc, 0x20, 0x34, 0xcd, 0x3, 0x5, 0x18, 0xe4, 0xf0, 0x4, 0x1f, 0xec, 0x1e, 0x1d, 0x0, 0x13, 0x12, 0x48, 0x4, 0x2, 0x24, 0x7, 0x36, 0x21, 0x1, 0x2b, 0x17, 0xee, 0x1a, 0xd7, 0xc, 0x17, 0xd9, 0xee, 0x2, 0xf6, 0xfc, 0x4, 0xf3, 0x1, 0xde, 0xc1, 0xf1, 0xfb, 0x3, 0x3c, 0xf7, 0xdb, 0xe, 0xf6, 0xf, 0x13, 0x1, 0xb, 0x4, 0xdc, 0xfd, 0x15, 0xcf, 0xf, 0x25, 0xf3, 0xfe, 0xf3, 0x1e, 0x8b, 0x26, 0x94];
    proof {
        let z = seq![120u8, 156, 1, 110, 0, 145, 255, 0, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243, 30, 139, 38, 148]; let raw = seq![0u8, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243];
        assert(z.subrange(7,117) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243, 30, 139, 38, 148],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243, 30, 139, 38, 148],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),110) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MOneBlock(z,16,Seq::empty(),110,32768,true) == (true,m::StoredEffect::End{out:raw,position:936}));
        assert(m::MBlk(z,16,Seq::empty(),110,32768,true) == (m::StoredEffect::End{out:raw,position:936}));
        assert(m::MFinish(seq![120u8, 156, 1, 110, 0, 145, 255, 0, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243, 30, 139, 38, 148],936,seq![0u8, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243],110,false) == (m::StoredEffect::End{out:seq![0u8, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243],position:968})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,110,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,110,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 121) by (compute);
    }
    let ghost raw = seq![0u8, 66, 32, 1, 13, 245, 2, 60, 3, 250, 4, 40, 234, 35, 0, 11, 60, 1, 65, 250, 2, 4, 215, 3, 43, 38, 11, 25, 17, 4, 253, 219, 51, 207, 50, 0, 47, 53, 25, 49, 1, 74, 221, 240, 42, 2, 220, 32, 52, 205, 3, 5, 24, 228, 240, 4, 31, 236, 30, 29, 0, 19, 18, 72, 4, 2, 36, 7, 54, 33, 1, 43, 23, 238, 26, 215, 12, 23, 217, 238, 2, 246, 252, 4, 243, 1, 222, 193, 241, 251, 3, 60, 247, 219, 14, 246, 15, 19, 1, 11, 4, 220, 253, 21, 207, 15, 37, 243, 254, 243]; let mut output = [0xa5u8;110]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct3_d8_il1_17x2: 1 independently audited frozen streams; R=39.
#[verifier::spinoff_prover]
fn intent_ct3_d8_il1_17x2() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x58, 0xea, 0xbd, 0x96, 0x91, 0xbd, 0x8f, 0x69, 0xfa, 0x9e, 0x7c, 0x9, 0x66, 0xe3, 0x96, 0xcf, 0x5b, 0x16, 0x94, 0xf5, 0xa8, 0xb1, 0x6c, 0xcd, 0xae, 0x5d, 0x57, 0x1a, 0x1d, 0x6e, 0x97, 0xd6, 0x5a, 0xe2, 0xea, 0xf6, 0x7e, 0x89, 0xd9, 0x26, 0x0, 0x2e, 0x67, 0x10, 0x52];
    proof {
        let z = seq![120u8, 156, 99, 88, 234, 189, 150, 145, 189, 143, 105, 250, 158, 124, 9, 102, 227, 150, 207, 91, 22, 148, 245, 168, 177, 108, 205, 174, 93, 87, 26, 29, 110, 151, 214, 90, 226, 234, 246, 126, 137, 217, 38, 0, 46, 103, 16, 82]; let raw = seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178]; let positions = seq![19nat, 27, 36, 44, 53, 61, 69, 77, 85, 94, 103, 111, 119, 127, 135, 143, 152, 161, 170, 178, 186, 194, 202, 211, 219, 227, 236, 244, 252, 260, 268, 276, 284, 292, 300, 308, 317, 326, 334, 343, 350];
        assert(literal_words(seq![120u8, 156, 99, 88, 234, 189, 150, 145, 189, 143, 105, 250, 158, 124, 9, 102, 227, 150, 207, 91, 22, 148, 245, 168, 177, 108, 205, 174, 93, 87, 26, 29, 110, 151, 214, 90, 226, 234, 246, 126, 137, 217, 38, 0, 46, 103, 16, 82],seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178],seq![19nat, 27, 36, 44, 53, 61, 69, 77, 85, 94, 103, 111, 119, 127, 135, 143, 152, 161, 170, 178, 186, 194, 202, 211, 219, 227, 236, 244, 252, 260, 268, 276, 284, 292, 300, 308, 317, 326, 334, 343, 350],0,40)) by (compute);
        literal_decodes(z,raw,positions,0,40); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),39,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178],position:350}));
        assert(m::field_value(seq![120u8, 156, 99, 88, 234, 189, 150, 145, 189, 143, 105, 250, 158, 124, 9, 102, 227, 150, 207, 91, 22, 148, 245, 168, 177, 108, 205, 174, 93, 87, 26, 29, 110, 151, 214, 90, 226, 234, 246, 126, 137, 217, 38, 0, 46, 103, 16, 82],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 88, 234, 189, 150, 145, 189, 143, 105, 250, 158, 124, 9, 102, 227, 150, 207, 91, 22, 148, 245, 168, 177, 108, 205, 174, 93, 87, 26, 29, 110, 151, 214, 90, 226, 234, 246, 126, 137, 217, 38, 0, 46, 103, 16, 82],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),39,32768,true) == (m::StoredEffect::End{out:seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178],position:350}));
        assert(m::MFinish(seq![120u8, 156, 99, 88, 234, 189, 150, 145, 189, 143, 105, 250, 158, 124, 9, 102, 227, 150, 207, 91, 22, 148, 245, 168, 177, 108, 205, 174, 93, 87, 26, 29, 110, 151, 214, 90, 226, 234, 246, 126, 137, 217, 38, 0, 46, 103, 16, 82],350,seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178],39,false) == (m::StoredEffect::End{out:seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178],position:384})) by (compute);
        assert(m::Inflate(z,39,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,39,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 48) by (compute);
    }
    let ghost raw = seq![0u8, 165, 75, 173, 1, 7, 142, 2, 151, 188, 111, 24, 3, 51, 132, 243, 180, 160, 118, 140, 38, 4, 181, 107, 125, 174, 117, 91, 87, 62, 102, 133, 116, 69, 70, 239, 164, 54, 178]; let mut output = [0xa5u8;39]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il0_1x1: 1 independently audited frozen streams; R=3.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x60, 0xbd, 0x0, 0x0, 0x0, 0xdd, 0x0, 0xd6];
    proof {
        let z = seq![120u8, 156, 99, 96, 189, 0, 0, 0, 221, 0, 214]; let raw = seq![0u8, 5, 208]; let positions = seq![19nat, 27, 35, 44, 51];
        assert(literal_words(seq![120u8, 156, 99, 96, 189, 0, 0, 0, 221, 0, 214],seq![0u8, 5, 208],seq![19nat, 27, 35, 44, 51],0,4)) by (compute);
        literal_decodes(z,raw,positions,0,4); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),3,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 5, 208],position:51}));
        assert(m::field_value(seq![120u8, 156, 99, 96, 189, 0, 0, 0, 221, 0, 214],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 189, 0, 0, 0, 221, 0, 214],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),3,32768,true) == (m::StoredEffect::End{out:seq![0u8, 5, 208],position:51}));
        assert(m::MFinish(seq![120u8, 156, 99, 96, 189, 0, 0, 0, 221, 0, 214],51,seq![0u8, 5, 208],3,false) == (m::StoredEffect::End{out:seq![0u8, 5, 208],position:88})) by (compute);
        assert(m::Inflate(z,3,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,3,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 11) by (compute);
    }
    let ghost raw = seq![0u8, 5, 208]; let mut output = [0xa5u8;3]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il0_2x3: 1 independently audited frozen streams; R=15.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x10, 0x99, 0x7d, 0x38, 0x88, 0x31, 0xd0, 0xf9, 0xd5, 0x71, 0x26, 0xa1, 0x45, 0x2f, 0xaf, 0x2, 0x0, 0x2a, 0x9a, 0x6, 0x7f];
    proof {
        let z = seq![120u8, 156, 99, 16, 153, 125, 56, 136, 49, 208, 249, 213, 113, 38, 161, 69, 47, 175, 2, 0, 42, 154, 6, 127]; let raw = seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213]; let positions = seq![19nat, 27, 35, 44, 53, 61, 69, 77, 85, 94, 103, 111, 119, 128, 137, 146, 153];
        assert(literal_words(seq![120u8, 156, 99, 16, 153, 125, 56, 136, 49, 208, 249, 213, 113, 38, 161, 69, 47, 175, 2, 0, 42, 154, 6, 127],seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213],seq![19nat, 27, 35, 44, 53, 61, 69, 77, 85, 94, 103, 111, 119, 128, 137, 146, 153],0,16)) by (compute);
        literal_decodes(z,raw,positions,0,16); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),15,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213],position:153}));
        assert(m::field_value(seq![120u8, 156, 99, 16, 153, 125, 56, 136, 49, 208, 249, 213, 113, 38, 161, 69, 47, 175, 2, 0, 42, 154, 6, 127],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 16, 153, 125, 56, 136, 49, 208, 249, 213, 113, 38, 161, 69, 47, 175, 2, 0, 42, 154, 6, 127],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),15,32768,true) == (m::StoredEffect::End{out:seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213],position:153}));
        assert(m::MFinish(seq![120u8, 156, 99, 16, 153, 125, 56, 136, 49, 208, 249, 213, 113, 38, 161, 69, 47, 175, 2, 0, 42, 154, 6, 127],153,seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213],15,false) == (m::StoredEffect::End{out:seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213],position:192})) by (compute);
        assert(m::Inflate(z,15,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,15,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 24) by (compute);
    }
    let ghost raw = seq![0u8, 20, 155, 195, 82, 1, 81, 67, 234, 199, 2, 18, 162, 233, 213]; let mut output = [0xa5u8;15]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il0_7x5: 2 frozen stored streams; R=75.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il0_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x4b, 0x0, 0xb4, 0xff, 0x0, 0x81, 0x88, 0xcc, 0xe4, 0xb6, 0x79, 0xd2, 0x6c, 0x8e, 0xdc, 0x58, 0x5b, 0x62, 0x14, 0x1, 0x2e, 0x2b, 0x3b, 0xd, 0x36, 0xd1, 0xfc, 0x30, 0x58, 0xf6, 0xf4, 0xc2, 0x17, 0xd, 0x2, 0x71, 0x5d, 0xb8, 0x10, 0xe2, 0x13, 0x57, 0xd, 0xf9, 0x5e, 0xe, 0xae, 0xfa, 0xa2, 0x3, 0x31, 0xb7, 0xd2, 0xdb, 0x41, 0xd2, 0xea, 0xbc, 0x73, 0xbe, 0x23, 0x5c, 0x98, 0x41, 0x4, 0x72, 0x8a, 0xa2, 0xf4, 0x8f, 0xff, 0x42, 0x4e, 0xd7, 0x37, 0x4e, 0x24, 0xfb, 0xfb, 0x2c, 0xa0, 0x24, 0x5b];
    proof {
        let z = seq![120u8, 156, 1, 75, 0, 180, 255, 0, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251, 44, 160, 36, 91]; let raw = seq![0u8, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251];
        assert(z.subrange(7,82) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251, 44, 160, 36, 91],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251, 44, 160, 36, 91],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),75) == (m::StoredEffect::End{out:raw,position:656}));
        assert(m::MOneBlock(z,16,Seq::empty(),75,32768,true) == (true,m::StoredEffect::End{out:raw,position:656}));
        assert(m::MBlk(z,16,Seq::empty(),75,32768,true) == (m::StoredEffect::End{out:raw,position:656}));
        assert(m::MFinish(seq![120u8, 156, 1, 75, 0, 180, 255, 0, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251, 44, 160, 36, 91],656,seq![0u8, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251],75,false) == (m::StoredEffect::End{out:seq![0u8, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251],position:688})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,75,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,75,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 86) by (compute);
    }
    let ghost raw = seq![0u8, 129, 136, 204, 228, 182, 121, 210, 108, 142, 220, 88, 91, 98, 20, 1, 46, 43, 59, 13, 54, 209, 252, 48, 88, 246, 244, 194, 23, 13, 2, 113, 93, 184, 16, 226, 19, 87, 13, 249, 94, 14, 174, 250, 162, 3, 49, 183, 210, 219, 65, 210, 234, 188, 115, 190, 35, 92, 152, 65, 4, 114, 138, 162, 244, 143, 255, 66, 78, 215, 55, 78, 36, 251, 251]; let mut output = [0xa5u8;75]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il0_17x2: 1 frozen stored streams; R=70.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il0_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x46, 0x0, 0xb9, 0xff, 0x0, 0xeb, 0x75, 0xd3, 0x15, 0x56, 0x7d, 0xa3, 0x9, 0x50, 0xaa, 0x63, 0xd8, 0x69, 0xea, 0x6e, 0xc5, 0xbb, 0x50, 0xa6, 0x7, 0x78, 0xa9, 0x63, 0x9b, 0x91, 0xc0, 0xbe, 0xcc, 0x72, 0x8, 0x48, 0xc0, 0x3c, 0x3a, 0x1, 0xd2, 0x39, 0xdf, 0x7f, 0xed, 0x21, 0x66, 0x94, 0x80, 0xe0, 0x44, 0x8, 0xd7, 0xc8, 0x59, 0xbc, 0xe3, 0x9e, 0xb, 0x3b, 0xb6, 0xfc, 0x70, 0xea, 0x52, 0x3b, 0xba, 0x97, 0x40, 0x35, 0xc0, 0xbc, 0x88, 0x87, 0xd6, 0xfb, 0x23, 0xb0];
    proof {
        let z = seq![120u8, 156, 1, 70, 0, 185, 255, 0, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135, 214, 251, 35, 176]; let raw = seq![0u8, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135];
        assert(z.subrange(7,77) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135, 214, 251, 35, 176],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135, 214, 251, 35, 176],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),70) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MOneBlock(z,16,Seq::empty(),70,32768,true) == (true,m::StoredEffect::End{out:raw,position:616}));
        assert(m::MBlk(z,16,Seq::empty(),70,32768,true) == (m::StoredEffect::End{out:raw,position:616}));
        assert(m::MFinish(seq![120u8, 156, 1, 70, 0, 185, 255, 0, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135, 214, 251, 35, 176],616,seq![0u8, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135],70,false) == (m::StoredEffect::End{out:seq![0u8, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135],position:648})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,70,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,70,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 81) by (compute);
    }
    let ghost raw = seq![0u8, 235, 117, 211, 21, 86, 125, 163, 9, 80, 170, 99, 216, 105, 234, 110, 197, 187, 80, 166, 7, 120, 169, 99, 155, 145, 192, 190, 204, 114, 8, 72, 192, 60, 58, 1, 210, 57, 223, 127, 237, 33, 102, 148, 128, 224, 68, 8, 215, 200, 89, 188, 227, 158, 11, 59, 182, 252, 112, 234, 82, 59, 186, 151, 64, 53, 192, 188, 136, 135]; let mut output = [0xa5u8;70]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il1_1x1: 1 independently audited frozen streams; R=3.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x78, 0x9b, 0x7, 0x0, 0x2, 0x4b, 0x1, 0x5c];
    proof {
        let z = seq![120u8, 156, 99, 120, 155, 7, 0, 2, 75, 1, 92]; let raw = seq![0u8, 237, 110]; let positions = seq![19nat, 27, 36, 44, 51];
        assert(literal_words(seq![120u8, 156, 99, 120, 155, 7, 0, 2, 75, 1, 92],seq![0u8, 237, 110],seq![19nat, 27, 36, 44, 51],0,4)) by (compute);
        literal_decodes(z,raw,positions,0,4); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),3,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 237, 110],position:51}));
        assert(m::field_value(seq![120u8, 156, 99, 120, 155, 7, 0, 2, 75, 1, 92],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 120, 155, 7, 0, 2, 75, 1, 92],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),3,32768,true) == (m::StoredEffect::End{out:seq![0u8, 237, 110],position:51}));
        assert(m::MFinish(seq![120u8, 156, 99, 120, 155, 7, 0, 2, 75, 1, 92],51,seq![0u8, 237, 110],3,false) == (m::StoredEffect::End{out:seq![0u8, 237, 110],position:88})) by (compute);
        assert(m::Inflate(z,3,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,3,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 11) by (compute);
    }
    let ghost raw = seq![0u8, 237, 110]; let mut output = [0xa5u8;3]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il1_2x3: 1 independently audited frozen streams; R=17.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf8, 0x76, 0x9c, 0xd1, 0xee, 0x32, 0xd3, 0x8c, 0x3a, 0xe6, 0xc4, 0x68, 0x96, 0x26, 0xeb, 0x98, 0x4e, 0x0, 0x3b, 0x95, 0x6, 0x4d];
    proof {
        let z = seq![120u8, 156, 99, 248, 118, 156, 209, 238, 50, 211, 140, 58, 230, 196, 104, 150, 38, 235, 152, 78, 0, 59, 149, 6, 77]; let raw = seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137]; let positions = seq![19nat, 27, 36, 45, 53, 61, 70, 78, 87, 95, 103, 111, 119, 127, 135, 143, 151, 159, 166];
        assert(literal_words(seq![120u8, 156, 99, 248, 118, 156, 209, 238, 50, 211, 140, 58, 230, 196, 104, 150, 38, 235, 152, 78, 0, 59, 149, 6, 77],seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137],seq![19nat, 27, 36, 45, 53, 61, 70, 78, 87, 95, 103, 111, 119, 127, 135, 143, 151, 159, 166],0,18)) by (compute);
        literal_decodes(z,raw,positions,0,18); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),17,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137],position:166}));
        assert(m::field_value(seq![120u8, 156, 99, 248, 118, 156, 209, 238, 50, 211, 140, 58, 230, 196, 104, 150, 38, 235, 152, 78, 0, 59, 149, 6, 77],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 248, 118, 156, 209, 238, 50, 211, 140, 58, 230, 196, 104, 150, 38, 235, 152, 78, 0, 59, 149, 6, 77],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),17,32768,true) == (m::StoredEffect::End{out:seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137],position:166}));
        assert(m::MFinish(seq![120u8, 156, 99, 248, 118, 156, 209, 238, 50, 211, 140, 58, 230, 196, 104, 150, 38, 235, 152, 78, 0, 59, 149, 6, 77],166,seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137],17,false) == (m::StoredEffect::End{out:seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137],position:200})) by (compute);
        assert(m::Inflate(z,17,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,17,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 25) by (compute);
    }
    let ghost raw = seq![0u8, 246, 199, 1, 62, 211, 2, 152, 126, 3, 97, 91, 4, 130, 59, 92, 137]; let mut output = [0xa5u8;17]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il1_7x5: 2 frozen stored streams; R=81.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il1_7x5() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x51, 0x0, 0xae, 0xff, 0x0, 0x4c, 0xce, 0x1, 0x1a, 0xcb, 0x2, 0xa9, 0x28, 0x6b, 0x36, 0x3, 0xd4, 0x54, 0xe, 0x28, 0x4, 0x59, 0xb9, 0xc8, 0x27, 0x0, 0x9e, 0xb, 0x48, 0x25, 0x9, 0xab, 0x28, 0xe, 0x1, 0x84, 0xfc, 0xb4, 0xfc, 0xca, 0xac, 0x2, 0xb1, 0xa5, 0x81, 0xb1, 0x22, 0x7a, 0x3, 0xf2, 0xd0, 0xca, 0x2c, 0x3d, 0xb5, 0x4, 0x6b, 0x69, 0x50, 0x4e, 0x56, 0xe7, 0xda, 0x87, 0xa5, 0x17, 0x80, 0x0, 0x2a, 0x5d, 0x0, 0x9, 0x60, 0x61, 0x18, 0x2a, 0x8f, 0x22, 0x61, 0x35, 0x0, 0xf6, 0x1f, 0xd, 0xdd, 0xd0, 0x6b, 0x1e, 0x7e];
    proof {
        let z = seq![120u8, 156, 1, 81, 0, 174, 255, 0, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221, 208, 107, 30, 126]; let raw = seq![0u8, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221];
        assert(z.subrange(7,88) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221, 208, 107, 30, 126],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221, 208, 107, 30, 126],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),81) == (m::StoredEffect::End{out:raw,position:704}));
        assert(m::MOneBlock(z,16,Seq::empty(),81,32768,true) == (true,m::StoredEffect::End{out:raw,position:704}));
        assert(m::MBlk(z,16,Seq::empty(),81,32768,true) == (m::StoredEffect::End{out:raw,position:704}));
        assert(m::MFinish(seq![120u8, 156, 1, 81, 0, 174, 255, 0, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221, 208, 107, 30, 126],704,seq![0u8, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221],81,false) == (m::StoredEffect::End{out:seq![0u8, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221],position:736})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,81,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,81,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 92) by (compute);
    }
    let ghost raw = seq![0u8, 76, 206, 1, 26, 203, 2, 169, 40, 107, 54, 3, 212, 84, 14, 40, 4, 89, 185, 200, 39, 0, 158, 11, 72, 37, 9, 171, 40, 14, 1, 132, 252, 180, 252, 202, 172, 2, 177, 165, 129, 177, 34, 122, 3, 242, 208, 202, 44, 61, 181, 4, 107, 105, 80, 78, 86, 231, 218, 135, 165, 23, 128, 0, 42, 93, 0, 9, 96, 97, 24, 42, 143, 34, 97, 53, 0, 246, 31, 13, 221]; let mut output = [0xa5u8;81]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d8_il1_17x2: 1 frozen stored streams; R=73.
#[verifier::spinoff_prover]
fn intent_ct4_d8_il1_17x2() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x49, 0x0, 0xb6, 0xff, 0x0, 0xb8, 0xa3, 0xe, 0x92, 0x26, 0xd7, 0x1, 0xb4, 0xe8, 0x5c, 0x88, 0x2, 0x9c, 0xcb, 0x33, 0x75, 0xe, 0x2d, 0xc, 0x63, 0x3, 0xb2, 0x59, 0x4b, 0x44, 0xd5, 0x98, 0x37, 0xf9, 0x9b, 0xc1, 0x3f, 0xb8, 0x27, 0xee, 0x2f, 0x57, 0x4, 0xaf, 0xd8, 0x97, 0xe1, 0xa9, 0xa0, 0xf8, 0x3d, 0xc9, 0xf6, 0x2a, 0xf, 0x68, 0xe3, 0xb9, 0x61, 0x7b, 0x82, 0x72, 0xf9, 0x98, 0x74, 0xbb, 0x9b, 0x13, 0x8f, 0xca, 0x85, 0xe3, 0xb9, 0x82, 0xb4, 0x8b, 0xc7, 0xdf, 0x7b, 0x25, 0x1a];
    proof {
        let z = seq![120u8, 156, 1, 73, 0, 182, 255, 0, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199, 223, 123, 37, 26]; let raw = seq![0u8, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199];
        assert(z.subrange(7,80) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199, 223, 123, 37, 26],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199, 223, 123, 37, 26],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),73) == (m::StoredEffect::End{out:raw,position:640}));
        assert(m::MOneBlock(z,16,Seq::empty(),73,32768,true) == (true,m::StoredEffect::End{out:raw,position:640}));
        assert(m::MBlk(z,16,Seq::empty(),73,32768,true) == (m::StoredEffect::End{out:raw,position:640}));
        assert(m::MFinish(seq![120u8, 156, 1, 73, 0, 182, 255, 0, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199, 223, 123, 37, 26],640,seq![0u8, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199],73,false) == (m::StoredEffect::End{out:seq![0u8, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199],position:672})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,73,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,73,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 84) by (compute);
    }
    let ghost raw = seq![0u8, 184, 163, 14, 146, 38, 215, 1, 180, 232, 92, 136, 2, 156, 203, 51, 117, 14, 45, 12, 99, 3, 178, 89, 75, 68, 213, 152, 55, 249, 155, 193, 63, 184, 39, 238, 47, 87, 4, 175, 216, 151, 225, 169, 160, 248, 61, 201, 246, 42, 15, 104, 227, 185, 97, 123, 130, 114, 249, 152, 116, 187, 155, 19, 143, 202, 133, 227, 185, 130, 180, 139, 199]; let mut output = [0xa5u8;73]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d16_il0_1x1: 1 independently audited frozen streams; R=5.
#[verifier::spinoff_prover]
fn intent_ct4_d16_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xf0, 0x9e, 0x7b, 0xe4, 0x1c, 0x0, 0x5, 0x5e, 0x2, 0x7b];
    proof {
        let z = seq![120u8, 156, 99, 240, 158, 123, 228, 28, 0, 5, 94, 2, 123]; let raw = seq![0u8, 75, 157, 196, 206]; let positions = seq![19nat, 27, 35, 44, 53, 62, 69];
        assert(literal_words(seq![120u8, 156, 99, 240, 158, 123, 228, 28, 0, 5, 94, 2, 123],seq![0u8, 75, 157, 196, 206],seq![19nat, 27, 35, 44, 53, 62, 69],0,6)) by (compute);
        literal_decodes(z,raw,positions,0,6); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),5,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 75, 157, 196, 206],position:69}));
        assert(m::field_value(seq![120u8, 156, 99, 240, 158, 123, 228, 28, 0, 5, 94, 2, 123],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 240, 158, 123, 228, 28, 0, 5, 94, 2, 123],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),5,32768,true) == (m::StoredEffect::End{out:seq![0u8, 75, 157, 196, 206],position:69}));
        assert(m::MFinish(seq![120u8, 156, 99, 240, 158, 123, 228, 28, 0, 5, 94, 2, 123],69,seq![0u8, 75, 157, 196, 206],5,false) == (m::StoredEffect::End{out:seq![0u8, 75, 157, 196, 206],position:104})) by (compute);
        assert(m::Inflate(z,5,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,5,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 13) by (compute);
    }
    let ghost raw = seq![0u8, 75, 157, 196, 206]; let mut output = [0xa5u8;5]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d16_il0_2x3: 1 independently audited frozen streams; R=27.
#[verifier::spinoff_prover]
fn intent_ct4_d16_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x78, 0xf4, 0x26, 0x58, 0xa8, 0xf9, 0xd1, 0x77, 0x57, 0x46, 0xff, 0xe2, 0xde, 0xa6, 0xc9, 0xdf, 0x96, 0x5d, 0x64, 0xfa, 0xba, 0xb6, 0xeb, 0xc2, 0xa5, 0xab, 0x95, 0xe2, 0x0, 0xc6, 0x74, 0xe, 0xdc];
    proof {
        let z = seq![120u8, 156, 99, 120, 244, 38, 88, 168, 249, 209, 119, 87, 70, 255, 226, 222, 166, 201, 223, 150, 93, 100, 250, 186, 182, 235, 194, 165, 171, 149, 226, 0, 198, 116, 14, 220]; let raw = seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23]; let positions = seq![19nat, 27, 36, 45, 53, 61, 69, 78, 87, 95, 103, 111, 119, 127, 135, 144, 153, 162, 171, 179, 188, 197, 205, 214, 223, 232, 240, 248, 255];
        assert(literal_words(seq![120u8, 156, 99, 120, 244, 38, 88, 168, 249, 209, 119, 87, 70, 255, 226, 222, 166, 201, 223, 150, 93, 100, 250, 186, 182, 235, 194, 165, 171, 149, 226, 0, 198, 116, 14, 220],seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23],seq![19nat, 27, 36, 45, 53, 61, 69, 78, 87, 95, 103, 111, 119, 127, 135, 144, 153, 162, 171, 179, 188, 197, 205, 214, 223, 232, 240, 248, 255],0,28)) by (compute);
        literal_decodes(z,raw,positions,0,28); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),27,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23],position:255}));
        assert(m::field_value(seq![120u8, 156, 99, 120, 244, 38, 88, 168, 249, 209, 119, 87, 70, 255, 226, 222, 166, 201, 223, 150, 93, 100, 250, 186, 182, 235, 194, 165, 171, 149, 226, 0, 198, 116, 14, 220],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 120, 244, 38, 88, 168, 249, 209, 119, 87, 70, 255, 226, 222, 166, 201, 223, 150, 93, 100, 250, 186, 182, 235, 194, 165, 171, 149, 226, 0, 198, 116, 14, 220],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),27,32768,true) == (m::StoredEffect::End{out:seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23],position:255}));
        assert(m::MFinish(seq![120u8, 156, 99, 120, 244, 38, 88, 168, 249, 209, 119, 87, 70, 255, 226, 222, 166, 201, 223, 150, 93, 100, 250, 186, 182, 235, 194, 165, 171, 149, 226, 0, 198, 116, 14, 220],255,seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23],27,false) == (m::StoredEffect::End{out:seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23],position:288})) by (compute);
        assert(m::Inflate(z,27,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,27,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 36) by (compute);
    }
    let ghost raw = seq![0u8, 226, 236, 83, 18, 131, 226, 247, 69, 1, 79, 115, 141, 130, 147, 246, 166, 209, 2, 245, 173, 138, 208, 210, 213, 121, 23]; let mut output = [0xa5u8;27]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d16_il1_1x1: 1 independently audited frozen streams; R=5.
#[verifier::spinoff_prover]
fn intent_ct4_d16_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x10, 0x2d, 0x6c, 0x5c, 0x6, 0x0, 0x3, 0x54, 0x1, 0xae];
    proof {
        let z = seq![120u8, 156, 99, 16, 45, 108, 92, 6, 0, 3, 84, 1, 174]; let raw = seq![0u8, 21, 113, 129, 166]; let positions = seq![19nat, 27, 35, 43, 51, 60, 67];
        assert(literal_words(seq![120u8, 156, 99, 16, 45, 108, 92, 6, 0, 3, 84, 1, 174],seq![0u8, 21, 113, 129, 166],seq![19nat, 27, 35, 43, 51, 60, 67],0,6)) by (compute);
        literal_decodes(z,raw,positions,0,6); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),5,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 21, 113, 129, 166],position:67}));
        assert(m::field_value(seq![120u8, 156, 99, 16, 45, 108, 92, 6, 0, 3, 84, 1, 174],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 16, 45, 108, 92, 6, 0, 3, 84, 1, 174],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),5,32768,true) == (m::StoredEffect::End{out:seq![0u8, 21, 113, 129, 166],position:67}));
        assert(m::MFinish(seq![120u8, 156, 99, 16, 45, 108, 92, 6, 0, 3, 84, 1, 174],67,seq![0u8, 21, 113, 129, 166],5,false) == (m::StoredEffect::End{out:seq![0u8, 21, 113, 129, 166],position:104})) by (compute);
        assert(m::Inflate(z,5,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,5,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 13) by (compute);
    }
    let ghost raw = seq![0u8, 21, 113, 129, 166]; let mut output = [0xa5u8;5]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct4_d16_il1_2x3: 1 independently audited frozen streams; R=29.
#[verifier::spinoff_prover]
fn intent_ct4_d16_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xc8, 0x7d, 0x35, 0x35, 0x80, 0xf1, 0x23, 0xb7, 0x71, 0xa, 0xd3, 0xd2, 0xc4, 0x1d, 0x97, 0x99, 0x97, 0xba, 0x1f, 0x3e, 0xca, 0xb2, 0x39, 0xd4, 0xa1, 0xae, 0xf4, 0xcd, 0xbd, 0xbd, 0x0, 0xb7, 0x6f, 0xd, 0xa1];
    proof {
        let z = seq![120u8, 156, 99, 200, 125, 53, 53, 128, 241, 35, 183, 113, 10, 211, 210, 196, 29, 151, 153, 151, 186, 31, 62, 202, 178, 57, 212, 161, 174, 244, 205, 189, 189, 0, 183, 111, 13, 161]; let raw = seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189]; let positions = seq![19nat, 27, 35, 44, 53, 61, 69, 78, 86, 94, 102, 110, 119, 127, 136, 145, 153, 162, 170, 179, 188, 196, 205, 213, 221, 229, 237, 246, 255, 264, 271];
        assert(literal_words(seq![120u8, 156, 99, 200, 125, 53, 53, 128, 241, 35, 183, 113, 10, 211, 210, 196, 29, 151, 153, 151, 186, 31, 62, 202, 178, 57, 212, 161, 174, 244, 205, 189, 189, 0, 183, 111, 13, 161],seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189],seq![19nat, 27, 35, 44, 53, 61, 69, 78, 86, 94, 102, 110, 119, 127, 136, 145, 153, 162, 170, 179, 188, 196, 205, 213, 221, 229, 237, 246, 255, 264, 271],0,30)) by (compute);
        literal_decodes(z,raw,positions,0,30); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),29,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189],position:271}));
        assert(m::field_value(seq![120u8, 156, 99, 200, 125, 53, 53, 128, 241, 35, 183, 113, 10, 211, 210, 196, 29, 151, 153, 151, 186, 31, 62, 202, 178, 57, 212, 161, 174, 244, 205, 189, 189, 0, 183, 111, 13, 161],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 200, 125, 53, 53, 128, 241, 35, 183, 113, 10, 211, 210, 196, 29, 151, 153, 151, 186, 31, 62, 202, 178, 57, 212, 161, 174, 244, 205, 189, 189, 0, 183, 111, 13, 161],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),29,32768,true) == (m::StoredEffect::End{out:seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189],position:271}));
        assert(m::MFinish(seq![120u8, 156, 99, 200, 125, 53, 53, 128, 241, 35, 183, 113, 10, 211, 210, 196, 29, 151, 153, 151, 186, 31, 62, 202, 178, 57, 212, 161, 174, 244, 205, 189, 189, 0, 183, 111, 13, 161],271,seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189],29,false) == (m::StoredEffect::End{out:seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189],position:304})) by (compute);
        assert(m::Inflate(z,29,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,29,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 38) by (compute);
    }
    let ghost raw = seq![0u8, 109, 234, 149, 80, 1, 241, 11, 51, 100, 2, 165, 97, 184, 211, 3, 165, 71, 195, 197, 4, 179, 85, 64, 126, 117, 236, 222, 189]; let mut output = [0xa5u8;29]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d8_il0_1x1: 1 independently audited frozen streams; R=5.
#[verifier::spinoff_prover]
fn intent_ct6_d8_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xd0, 0xba, 0x1e, 0x2b, 0x0, 0x0, 0x3, 0xfc, 0x1, 0x6f];
    proof {
        let z = seq![120u8, 156, 99, 208, 186, 30, 43, 0, 0, 3, 252, 1, 111]; let raw = seq![0u8, 42, 215, 93, 16]; let positions = seq![19nat, 27, 35, 44, 52, 60, 67];
        assert(literal_words(seq![120u8, 156, 99, 208, 186, 30, 43, 0, 0, 3, 252, 1, 111],seq![0u8, 42, 215, 93, 16],seq![19nat, 27, 35, 44, 52, 60, 67],0,6)) by (compute);
        literal_decodes(z,raw,positions,0,6); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),5,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 42, 215, 93, 16],position:67}));
        assert(m::field_value(seq![120u8, 156, 99, 208, 186, 30, 43, 0, 0, 3, 252, 1, 111],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 208, 186, 30, 43, 0, 0, 3, 252, 1, 111],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),5,32768,true) == (m::StoredEffect::End{out:seq![0u8, 42, 215, 93, 16],position:67}));
        assert(m::MFinish(seq![120u8, 156, 99, 208, 186, 30, 43, 0, 0, 3, 252, 1, 111],67,seq![0u8, 42, 215, 93, 16],5,false) == (m::StoredEffect::End{out:seq![0u8, 42, 215, 93, 16],position:104})) by (compute);
        assert(m::Inflate(z,5,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,5,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 13) by (compute);
    }
    let ghost raw = seq![0u8, 42, 215, 93, 16]; let mut output = [0xa5u8;5]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d8_il0_2x3: 1 independently audited frozen streams; R=27.
#[verifier::spinoff_prover]
fn intent_ct6_d8_il0_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x10, 0x50, 0x7a, 0x9d, 0x9f, 0x3d, 0x71, 0xb6, 0x2c, 0xe3, 0xa3, 0x1b, 0x89, 0x81, 0xfb, 0xf6, 0x3e, 0xd, 0x64, 0xa, 0xee, 0x79, 0x3f, 0x9d, 0xf5, 0x93, 0xb3, 0x39, 0x0, 0xa1, 0x4e, 0xc, 0x37];
    proof {
        let z = seq![120u8, 156, 99, 16, 80, 122, 157, 159, 61, 113, 182, 44, 227, 163, 27, 137, 129, 251, 246, 62, 13, 100, 10, 238, 121, 63, 157, 245, 147, 179, 57, 0, 161, 78, 12, 55]; let raw = seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55]; let positions = seq![19nat, 27, 35, 43, 52, 60, 68, 77, 86, 94, 102, 111, 120, 128, 136, 145, 154, 163, 171, 179, 187, 195, 204, 213, 221, 230, 238, 246, 253];
        assert(literal_words(seq![120u8, 156, 99, 16, 80, 122, 157, 159, 61, 113, 182, 44, 227, 163, 27, 137, 129, 251, 246, 62, 13, 100, 10, 238, 121, 63, 157, 245, 147, 179, 57, 0, 161, 78, 12, 55],seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55],seq![19nat, 27, 35, 43, 52, 60, 68, 77, 86, 94, 102, 111, 120, 128, 136, 145, 154, 163, 171, 179, 187, 195, 204, 213, 221, 230, 238, 246, 253],0,28)) by (compute);
        literal_decodes(z,raw,positions,0,28); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),27,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55],position:253}));
        assert(m::field_value(seq![120u8, 156, 99, 16, 80, 122, 157, 159, 61, 113, 182, 44, 227, 163, 27, 137, 129, 251, 246, 62, 13, 100, 10, 238, 121, 63, 157, 245, 147, 179, 57, 0, 161, 78, 12, 55],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 16, 80, 122, 157, 159, 61, 113, 182, 44, 227, 163, 27, 137, 129, 251, 246, 62, 13, 100, 10, 238, 121, 63, 157, 245, 147, 179, 57, 0, 161, 78, 12, 55],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),27,32768,true) == (m::StoredEffect::End{out:seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55],position:253}));
        assert(m::MFinish(seq![120u8, 156, 99, 16, 80, 122, 157, 159, 61, 113, 182, 44, 227, 163, 27, 137, 129, 251, 246, 62, 13, 100, 10, 238, 121, 63, 157, 245, 147, 179, 57, 0, 161, 78, 12, 55],253,seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55],27,false) == (m::StoredEffect::End{out:seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55],position:288})) by (compute);
        assert(m::Inflate(z,27,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,27,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 36) by (compute);
    }
    let ghost raw = seq![0u8, 16, 34, 235, 111, 107, 145, 155, 29, 1, 226, 216, 97, 81, 190, 189, 229, 81, 2, 83, 140, 239, 151, 5, 242, 67, 55]; let mut output = [0xa5u8;27]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d8_il1_1x1: 1 independently audited frozen streams; R=5.
#[verifier::spinoff_prover]
fn intent_ct6_d8_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xa8, 0x4c, 0x8e, 0x7c, 0x2, 0x0, 0x4, 0xa8, 0x2, 0x1a];
    proof {
        let z = seq![120u8, 156, 99, 168, 76, 142, 124, 2, 0, 4, 168, 2, 26]; let raw = seq![0u8, 121, 99, 89, 228]; let positions = seq![19nat, 27, 35, 43, 51, 60, 67];
        assert(literal_words(seq![120u8, 156, 99, 168, 76, 142, 124, 2, 0, 4, 168, 2, 26],seq![0u8, 121, 99, 89, 228],seq![19nat, 27, 35, 43, 51, 60, 67],0,6)) by (compute);
        literal_decodes(z,raw,positions,0,6); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),5,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 121, 99, 89, 228],position:67}));
        assert(m::field_value(seq![120u8, 156, 99, 168, 76, 142, 124, 2, 0, 4, 168, 2, 26],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 168, 76, 142, 124, 2, 0, 4, 168, 2, 26],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),5,32768,true) == (m::StoredEffect::End{out:seq![0u8, 121, 99, 89, 228],position:67}));
        assert(m::MFinish(seq![120u8, 156, 99, 168, 76, 142, 124, 2, 0, 4, 168, 2, 26],67,seq![0u8, 121, 99, 89, 228],5,false) == (m::StoredEffect::End{out:seq![0u8, 121, 99, 89, 228],position:104})) by (compute);
        assert(m::Inflate(z,5,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,5,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 13) by (compute);
    }
    let ghost raw = seq![0u8, 121, 99, 89, 228]; let mut output = [0xa5u8;5]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d8_il1_2x3: 1 independently audited frozen streams; R=29.
#[verifier::spinoff_prover]
fn intent_ct6_d8_il1_2x3() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x38, 0x3c, 0xab, 0x75, 0x5, 0xa3, 0x4e, 0x65, 0x6b, 0x36, 0x93, 0x61, 0xb9, 0x7c, 0x4, 0xf3, 0xf3, 0x60, 0xef, 0xcb, 0x2c, 0xdf, 0x1a, 0xd, 0x97, 0xf4, 0x58, 0xfd, 0xe5, 0x1, 0x0, 0xa7, 0xc0, 0xb, 0xbc];
    proof {
        let z = seq![120u8, 156, 99, 56, 60, 171, 117, 5, 163, 78, 101, 107, 54, 147, 97, 185, 124, 4, 243, 243, 96, 239, 203, 44, 223, 26, 13, 151, 244, 88, 253, 229, 1, 0, 167, 192, 11, 188]; let raw = seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12]; let positions = seq![19nat, 27, 36, 45, 53, 62, 70, 78, 86, 94, 102, 110, 118, 126, 134, 142, 150, 159, 167, 175, 184, 192, 201, 209, 217, 226, 234, 242, 251, 259, 266];
        assert(literal_words(seq![120u8, 156, 99, 56, 60, 171, 117, 5, 163, 78, 101, 107, 54, 147, 97, 185, 124, 4, 243, 243, 96, 239, 203, 44, 223, 26, 13, 151, 244, 88, 253, 229, 1, 0, 167, 192, 11, 188],seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12],seq![19nat, 27, 36, 45, 53, 62, 70, 78, 86, 94, 102, 110, 118, 126, 134, 142, 150, 159, 167, 175, 184, 192, 201, 209, 217, 226, 234, 242, 251, 259, 266],0,30)) by (compute);
        literal_decodes(z,raw,positions,0,30); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),29,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12],position:266}));
        assert(m::field_value(seq![120u8, 156, 99, 56, 60, 171, 117, 5, 163, 78, 101, 107, 54, 147, 97, 185, 124, 4, 243, 243, 96, 239, 203, 44, 223, 26, 13, 151, 244, 88, 253, 229, 1, 0, 167, 192, 11, 188],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 60, 171, 117, 5, 163, 78, 101, 107, 54, 147, 97, 185, 124, 4, 243, 243, 96, 239, 203, 44, 223, 26, 13, 151, 244, 88, 253, 229, 1, 0, 167, 192, 11, 188],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),29,32768,true) == (m::StoredEffect::End{out:seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12],position:266}));
        assert(m::MFinish(seq![120u8, 156, 99, 56, 60, 171, 117, 5, 163, 78, 101, 107, 54, 147, 97, 185, 124, 4, 243, 243, 96, 239, 203, 44, 223, 26, 13, 151, 244, 88, 253, 229, 1, 0, 167, 192, 11, 188],266,seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12],29,false) == (m::StoredEffect::End{out:seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12],position:304})) by (compute);
        assert(m::Inflate(z,29,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,29,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 38) by (compute);
    }
    let ghost raw = seq![0u8, 195, 154, 133, 168, 1, 44, 121, 133, 107, 2, 49, 119, 31, 88, 3, 231, 83, 75, 211, 4, 246, 129, 49, 164, 140, 58, 253, 12]; let mut output = [0xa5u8;29]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d16_il0_1x1: 1 independently audited frozen streams; R=9.
#[verifier::spinoff_prover]
fn intent_ct6_d16_il0_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0x90, 0xff, 0xf0, 0x7e, 0x7f, 0x6a, 0x42, 0x29, 0x27, 0x0, 0x14, 0x8d, 0x4, 0x1];
    proof {
        let z = seq![120u8, 156, 99, 144, 255, 240, 126, 127, 106, 66, 41, 39, 0, 20, 141, 4, 1]; let raw = seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9]; let positions = seq![19nat, 27, 35, 44, 53, 62, 70, 78, 86, 94, 101];
        assert(literal_words(seq![120u8, 156, 99, 144, 255, 240, 126, 127, 106, 66, 41, 39, 0, 20, 141, 4, 1],seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9],seq![19nat, 27, 35, 44, 53, 62, 70, 78, 86, 94, 101],0,10)) by (compute);
        literal_decodes(z,raw,positions,0,10); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),9,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9],position:101}));
        assert(m::field_value(seq![120u8, 156, 99, 144, 255, 240, 126, 127, 106, 66, 41, 39, 0, 20, 141, 4, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 144, 255, 240, 126, 127, 106, 66, 41, 39, 0, 20, 141, 4, 1],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),9,32768,true) == (m::StoredEffect::End{out:seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9],position:101}));
        assert(m::MFinish(seq![120u8, 156, 99, 144, 255, 240, 126, 127, 106, 66, 41, 39, 0, 20, 141, 4, 1],101,seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9],9,false) == (m::StoredEffect::End{out:seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9],position:136})) by (compute);
        assert(m::Inflate(z,9,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,9,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 17) by (compute);
    }
    let ghost raw = seq![0u8, 31, 240, 239, 191, 101, 96, 117, 9]; let mut output = [0xa5u8;9]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d16_il0_2x3: 1 frozen stored streams; R=51.
#[verifier::spinoff_prover]
fn intent_ct6_d16_il0_2x3() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x33, 0x0, 0xcc, 0xff, 0x0, 0xf3, 0x35, 0x38, 0xff, 0x7c, 0xce, 0x1, 0x7d, 0x67, 0x63, 0xc5, 0xa, 0x8f, 0x78, 0xbd, 0xf5, 0x1, 0x7a, 0xca, 0x77, 0xab, 0xa0, 0x58, 0x38, 0x4, 0xb6, 0x62, 0x91, 0x36, 0x8a, 0xa6, 0x83, 0x7d, 0x2, 0x89, 0xec, 0xef, 0x4c, 0x94, 0x1c, 0x90, 0xff, 0x92, 0x32, 0x82, 0x8a, 0xff, 0xc6, 0xa6, 0xf7, 0x7e, 0x13, 0x1a, 0x47];
    proof {
        let z = seq![120u8, 156, 1, 51, 0, 204, 255, 0, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247, 126, 19, 26, 71]; let raw = seq![0u8, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247];
        assert(z.subrange(7,58) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 51, 0, 204, 255, 0, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247, 126, 19, 26, 71],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 51, 0, 204, 255, 0, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247, 126, 19, 26, 71],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),51) == (m::StoredEffect::End{out:raw,position:464}));
        assert(m::MOneBlock(z,16,Seq::empty(),51,32768,true) == (true,m::StoredEffect::End{out:raw,position:464}));
        assert(m::MBlk(z,16,Seq::empty(),51,32768,true) == (m::StoredEffect::End{out:raw,position:464}));
        assert(m::MFinish(seq![120u8, 156, 1, 51, 0, 204, 255, 0, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247, 126, 19, 26, 71],464,seq![0u8, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247],51,false) == (m::StoredEffect::End{out:seq![0u8, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247],position:496})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,51,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,51,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 62) by (compute);
    }
    let ghost raw = seq![0u8, 243, 53, 56, 255, 124, 206, 1, 125, 103, 99, 197, 10, 143, 120, 189, 245, 1, 122, 202, 119, 171, 160, 88, 56, 4, 182, 98, 145, 54, 138, 166, 131, 125, 2, 137, 236, 239, 76, 148, 28, 144, 255, 146, 50, 130, 138, 255, 198, 166, 247]; let mut output = [0xa5u8;51]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d16_il1_1x1: 1 independently audited frozen streams; R=9.
#[verifier::spinoff_prover]
fn intent_ct6_d16_il1_1x1() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    let wire = [0x78u8, 0x9c, 0x63, 0xb8, 0xb9, 0xb5, 0xb1, 0xfa, 0xc1, 0xb4, 0x2f, 0x8f, 0x1, 0x19, 0x3e, 0x5, 0xd8];
    proof {
        let z = seq![120u8, 156, 99, 184, 185, 181, 177, 250, 193, 180, 47, 143, 1, 25, 62, 5, 216]; let raw = seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227]; let positions = seq![19nat, 27, 36, 45, 53, 61, 70, 79, 88, 97, 104];
        assert(literal_words(seq![120u8, 156, 99, 184, 185, 181, 177, 250, 193, 180, 47, 143, 1, 25, 62, 5, 216],seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227],seq![19nat, 27, 36, 45, 53, 61, 70, 79, 88, 97, 104],0,10)) by (compute);
        literal_decodes(z,raw,positions,0,10); literal_expand(z,raw,positions,0);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(m::MCompressed(z,19,Seq::empty(),9,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227],position:104}));
        assert(m::field_value(seq![120u8, 156, 99, 184, 185, 181, 177, 250, 193, 180, 47, 143, 1, 25, 62, 5, 216],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 184, 185, 181, 177, 250, 193, 180, 47, 143, 1, 25, 62, 5, 216],17,2) == 1) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),9,32768,true) == (m::StoredEffect::End{out:seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227],position:104}));
        assert(m::MFinish(seq![120u8, 156, 99, 184, 185, 181, 177, 250, 193, 180, 47, 143, 1, 25, 62, 5, 216],104,seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227],9,false) == (m::StoredEffect::End{out:seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227],position:136})) by (compute);
        assert(m::Inflate(z,9,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,9,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 17) by (compute);
    }
    let ghost raw = seq![0u8, 217, 181, 129, 123, 224, 150, 244, 227]; let mut output = [0xa5u8;9]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
// Class intent_ct6_d16_il1_2x3: 1 frozen stored streams; R=53.
#[verifier::spinoff_prover]
fn intent_ct6_d16_il1_2x3() {
    hide(m::MStoredPayload);
    let wire = [0x78u8, 0x9c, 0x1, 0x35, 0x0, 0xca, 0xff, 0x0, 0x58, 0xe3, 0xf9, 0x1a, 0x34, 0x26, 0xb0, 0x3, 0x1, 0xfc, 0xd0, 0x98, 0x7f, 0x30, 0x3a, 0x24, 0x53, 0x2, 0xd5, 0x7c, 0x5b, 0xd2, 0x84, 0xe4, 0x76, 0xb9, 0x3, 0xa6, 0xda, 0x33, 0xe8, 0xf3, 0x4f, 0xd6, 0xa, 0x4, 0xb3, 0x15, 0x6f, 0x3d, 0xa3, 0xae, 0xa4, 0xf1, 0xea, 0x6f, 0x9b, 0x92, 0x7e, 0x48, 0x49, 0xe9, 0x8b, 0xb9, 0x19, 0xd4];
    proof {
        let z = seq![120u8, 156, 1, 53, 0, 202, 255, 0, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233, 139, 185, 25, 212]; let raw = seq![0u8, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233];
        assert(z.subrange(7,60) =~= raw);
        stored_payload(z,raw);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 1, 53, 0, 202, 255, 0, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233, 139, 185, 25, 212],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 1, 53, 0, 202, 255, 0, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233, 139, 185, 25, 212],17,2) == 0) by (compute);
        assert(m::MStoredBlock(z,19,Seq::empty(),53) == (m::StoredEffect::End{out:raw,position:480}));
        assert(m::MOneBlock(z,16,Seq::empty(),53,32768,true) == (true,m::StoredEffect::End{out:raw,position:480}));
        assert(m::MBlk(z,16,Seq::empty(),53,32768,true) == (m::StoredEffect::End{out:raw,position:480}));
        assert(m::MFinish(seq![120u8, 156, 1, 53, 0, 202, 255, 0, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233, 139, 185, 25, 212],480,seq![0u8, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233],53,false) == (m::StoredEffect::End{out:seq![0u8, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233],position:512})) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::Inflate(z,53,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,53,true,false).out == raw);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 64) by (compute);
    }
    let ghost raw = seq![0u8, 88, 227, 249, 26, 52, 38, 176, 3, 1, 252, 208, 152, 127, 48, 58, 36, 83, 2, 213, 124, 91, 210, 132, 228, 118, 185, 3, 166, 218, 51, 232, 243, 79, 214, 10, 4, 179, 21, 111, 61, 163, 174, 164, 241, 234, 111, 155, 146, 126, 72, 73, 233]; let mut output = [0xa5u8;53]; check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
} // verus!
