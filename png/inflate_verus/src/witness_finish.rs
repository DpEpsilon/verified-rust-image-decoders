//! Authored fixed frame/checksum/tail witnesses at independent per-stream R.
use crate::{model as m, witness_match as t, witness_checksum as c, witness_policy::check_policy_parts, Status, InflateError as E};
use vstd::prelude::*;
verus! {
#[verifier::spinoff_prover]
fn frame_adler_error() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0: [u8;16] = [0x78u8, 0x9c, 0x63, 0x60, 0xf8, 0xf, 0x81, 0x8c, 0x60, 0xa, 0x8, 0x0, 0x3f, 0xdb, 0x5, 0x0];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        let z = seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 0]; let raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0]; let ts = seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:89, before:15, after:18, symbol:257, le:0, ds:0, de:0, length:3, distance:1}, t::Step {pos:89, next:96, before:18, after:18, symbol:256, le:0, ds:0, de:0, length:0, distance:0}];
        assert(t::trace(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 0],seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:89, before:15, after:18, symbol:257, le:0, ds:0, de:0, length:3, distance:1}, t::Step {pos:89, next:96, before:18, after:18, symbol:256, le:0, ds:0, de:0, length:0, distance:0}],0,8)) by (compute);
        t::effects_at(z,raw,ts,0,8,18); t::expand_at(z,raw,ts,0,18);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 0],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 0],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),18,32768,true) == (m::StoredEffect::End {out:raw,position:96}));
        assert(c::sums(seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],0,18) == (1531nat,16347nat)) by (compute);
        c::sums_correct(raw,0,18); assert(raw.subrange(0,18) =~= raw);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);
        assert((16347nat%65521)*(u16::MAX as nat+1)+(1+1531nat)%65521 == 1071318524) by (compute); assert(m::Adler(raw) == 1071318524);
        assert(m::MFinish(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 0],96,seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],18,false) == (m::StoredEffect::Error {out:seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],position:128,error:m::InflateFault::AdlerMismatch})) by (compute);
        assert(m::Inflate(z,18,true,false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch));
        assert(m::Inflate(z,18,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
    }
    let ghost raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0]; let mut output = [0xa5u8;18]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::AdlerMismatch),Ghost(raw));
}
#[verifier::spinoff_prover]
fn green_canvas_r() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0: [u8;16] = [0x78u8, 0x9c, 0x63, 0x60, 0xf8, 0xf, 0x81, 0x8c, 0x60, 0xa, 0x8, 0x0, 0x3f, 0xdb, 0x5, 0xfc];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        let z = seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 252]; let raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0]; let ts = seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:89, before:15, after:18, symbol:257, le:0, ds:0, de:0, length:3, distance:1}, t::Step {pos:89, next:96, before:18, after:18, symbol:256, le:0, ds:0, de:0, length:0, distance:0}];
        assert(t::trace(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 252],seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:89, before:15, after:18, symbol:257, le:0, ds:0, de:0, length:3, distance:1}, t::Step {pos:89, next:96, before:18, after:18, symbol:256, le:0, ds:0, de:0, length:0, distance:0}],0,8)) by (compute);
        t::effects_at(z,raw,ts,0,8,51); t::expand_at(z,raw,ts,0,51);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 252],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 252],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),51,32768,true) == (m::StoredEffect::End {out:raw,position:96}));
        assert(c::sums(seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],0,18) == (1531nat,16347nat)) by (compute);
        c::sums_correct(raw,0,18); assert(raw.subrange(0,18) =~= raw);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);
        assert((16347nat%65521)*(u16::MAX as nat+1)+(1+1531nat)%65521 == 1071318524) by (compute); assert(m::Adler(raw) == 1071318524);
        assert(m::MFinish(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 10, 8, 0, 63, 219, 5, 252],96,seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],51,false) == (m::StoredEffect::End {out:seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0],position:128})) by (compute);
        assert(m::Inflate(z,51,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,51,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
    }
    let ghost raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0, 0]; let mut output = [0xa5u8;51]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));
}
#[verifier::spinoff_prover]
fn short_successful_frame() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0: [u8;17] = [0x78u8, 0x9c, 0x63, 0x60, 0xf8, 0xf, 0x81, 0x8c, 0x60, 0x8a, 0x81, 0x1, 0x0, 0x39, 0xdf, 0x5, 0xfc];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        let z = seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 138, 129, 1, 0, 57, 223, 5, 252]; let raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0]; let ts = seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:85, before:15, after:16, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:85, next:93, before:16, after:17, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:93, next:100, before:17, after:17, symbol:256, le:0, ds:0, de:0, length:0, distance:0}];
        assert(t::trace(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 138, 129, 1, 0, 57, 223, 5, 252],seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0],seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:44, before:2, after:3, symbol:255, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:56, before:3, after:9, symbol:260, le:0, ds:1, de:0, length:6, distance:2}, t::Step {pos:56, next:64, before:9, after:10, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:64, next:77, before:10, after:15, symbol:259, le:0, ds:5, de:0, length:5, distance:7}, t::Step {pos:77, next:85, before:15, after:16, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:85, next:93, before:16, after:17, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:93, next:100, before:17, after:17, symbol:256, le:0, ds:0, de:0, length:0, distance:0}],0,9)) by (compute);
        t::effects_at(z,raw,ts,0,9,18); t::expand_at(z,raw,ts,0,18);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 138, 129, 1, 0, 57, 223, 5, 252],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 138, 129, 1, 0, 57, 223, 5, 252],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),18,32768,true) == (m::StoredEffect::End {out:raw,position:100}));
        assert(c::sums(seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0],0,17) == (1531nat,14815nat)) by (compute);
        c::sums_correct(raw,0,17); assert(raw.subrange(0,17) =~= raw);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);
        assert((14815nat%65521)*(u16::MAX as nat+1)+(1+1531nat)%65521 == 970917372) by (compute); assert(m::Adler(raw) == 970917372);
        assert(m::MFinish(seq![120u8, 156, 99, 96, 248, 15, 129, 140, 96, 138, 129, 1, 0, 57, 223, 5, 252],100,seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0],18,false) == (m::StoredEffect::End {out:seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0],position:136})) by (compute);
        assert(m::Inflate(z,18,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,18,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 17) by (compute);
    }
    let ghost raw = seq![0u8, 0, 255, 0, 255, 0, 255, 0, 255, 1, 0, 255, 0, 255, 0, 0, 0]; let mut output = [0xa5u8;18]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));
}
#[verifier::spinoff_prover]
fn palette_five_entries() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0: [u8;12] = [0x78u8, 0x9c, 0x63, 0x10, 0x60, 0x9c, 0x0, 0x0, 0x0, 0xc6, 0x0, 0xa2];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        let z = seq![120u8, 156, 99, 16, 96, 156, 0, 0, 0, 198, 0, 162]; let raw = seq![0u8, 16, 1, 144]; let ts = seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:16, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:43, before:2, after:3, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:43, next:52, before:3, after:4, symbol:144, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:52, next:59, before:4, after:4, symbol:256, le:0, ds:0, de:0, length:0, distance:0}];
        assert(t::trace(seq![120u8, 156, 99, 16, 96, 156, 0, 0, 0, 198, 0, 162],seq![0u8, 16, 1, 144],seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:35, before:1, after:2, symbol:16, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:35, next:43, before:2, after:3, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:43, next:52, before:3, after:4, symbol:144, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:52, next:59, before:4, after:4, symbol:256, le:0, ds:0, de:0, length:0, distance:0}],0,5)) by (compute);
        t::effects_at(z,raw,ts,0,5,4); t::expand_at(z,raw,ts,0,4);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 99, 16, 96, 156, 0, 0, 0, 198, 0, 162],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 16, 96, 156, 0, 0, 0, 198, 0, 162],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::End {out:raw,position:59}));
        assert(c::sums(seq![0u8, 16, 1, 144],0,4) == (161nat,198nat)) by (compute);
        c::sums_correct(raw,0,4); assert(raw.subrange(0,4) =~= raw);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);
        assert((198nat%65521)*(u16::MAX as nat+1)+(1+161nat)%65521 == 12976290) by (compute); assert(m::Adler(raw) == 12976290);
        assert(m::MFinish(seq![120u8, 156, 99, 16, 96, 156, 0, 0, 0, 198, 0, 162],59,seq![0u8, 16, 1, 144],4,false) == (m::StoredEffect::End {out:seq![0u8, 16, 1, 144],position:96})) by (compute);
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,4,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 12) by (compute);
    }
    let ghost raw = seq![0u8, 16, 1, 144]; let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));
}
#[verifier::spinoff_prover]
fn idat_after_iend_tail() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0: [u8;22] = [0x78u8, 0x9c, 0x63, 0x38, 0xe9, 0x69, 0x7f, 0xca, 0x7b, 0x3a, 0xe3, 0x43, 0x29, 0x71, 0xe9, 0x3b, 0x59, 0x0, 0x28, 0x41, 0x5, 0x72];
    let segment_1: [u8;22] = [0x78u8, 0x9c, 0x63, 0x38, 0xe9, 0x69, 0x7f, 0xca, 0x7b, 0x3a, 0xe3, 0x43, 0x29, 0x71, 0xe9, 0x3b, 0x59, 0x0, 0x28, 0x41, 0x5, 0x72];
    let parts: [&[u8];2] = [&segment_0, &segment_1];
    proof {
        let z = seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114, 120, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114]; let raw = seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106]; let ts = seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:36, before:1, after:2, symbol:201, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:36, next:44, before:2, after:3, symbol:73, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:52, before:3, after:4, symbol:63, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:52, next:61, before:4, after:5, symbol:202, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:61, next:69, before:5, after:6, symbol:75, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:69, next:78, before:6, after:7, symbol:151, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:78, next:86, before:7, after:8, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:86, next:95, before:8, after:9, symbol:225, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:95, next:103, before:9, after:10, symbol:26, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:103, next:111, before:10, after:11, symbol:23, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:111, next:119, before:11, after:12, symbol:27, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:119, next:128, before:12, after:13, symbol:220, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:128, next:136, before:13, after:14, symbol:106, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:136, next:143, before:14, after:14, symbol:256, le:0, ds:0, de:0, length:0, distance:0}];
        assert(t::trace(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114, 120, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],seq![t::Step {pos:19, next:27, before:0, after:1, symbol:0, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:27, next:36, before:1, after:2, symbol:201, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:36, next:44, before:2, after:3, symbol:73, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:44, next:52, before:3, after:4, symbol:63, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:52, next:61, before:4, after:5, symbol:202, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:61, next:69, before:5, after:6, symbol:75, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:69, next:78, before:6, after:7, symbol:151, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:78, next:86, before:7, after:8, symbol:1, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:86, next:95, before:8, after:9, symbol:225, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:95, next:103, before:9, after:10, symbol:26, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:103, next:111, before:10, after:11, symbol:23, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:111, next:119, before:11, after:12, symbol:27, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:119, next:128, before:12, after:13, symbol:220, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:128, next:136, before:13, after:14, symbol:106, le:0, ds:0, de:0, length:0, distance:0}, t::Step {pos:136, next:143, before:14, after:14, symbol:256, le:0, ds:0, de:0, length:0, distance:0}],0,15)) by (compute);
        t::effects_at(z,raw,ts,0,15,14); t::expand_at(z,raw,ts,0,14);
        assert(raw.take(0) =~= Seq::<u8>::empty());
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114, 120, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114, 120, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),14,32768,true) == (m::StoredEffect::End {out:raw,position:143}));
        assert(c::sums(seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],0,14) == (1393nat,10305nat)) by (compute);
        c::sums_correct(raw,0,14); assert(raw.subrange(0,14) =~= raw);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);
        assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521 == 675349874) by (compute); assert(m::Adler(raw) == 675349874);
        assert(m::MFinish(seq![120u8, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114, 120, 156, 99, 56, 233, 105, 127, 202, 123, 58, 227, 67, 41, 113, 233, 59, 89, 0, 40, 65, 5, 114],143,seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],14,false) == (m::StoredEffect::Error {out:seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106],position:184,error:m::InflateFault::TrailingZlibData})) by (compute);
        assert(m::Inflate(z,14,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData));
        assert(m::Inflate(z,14,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,3); assert(crate::segments::parts_prefix(parts@,2) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 44) by (compute);
    }
    let ghost raw = seq![0u8, 201, 73, 63, 202, 75, 151, 1, 225, 26, 23, 27, 220, 106]; let mut output = [0xa5u8;14]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
}
} // verus!
