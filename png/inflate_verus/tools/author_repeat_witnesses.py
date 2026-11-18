#!/usr/bin/env python3
"""Author frozen repeat rejection traces without decoding the input.

The frozen generator emits symbol 16 at offset 90 with no prior length,
and three symbol-18/extra-127 runs of 138 zeros. The third run exceeds
286+2 slots. CL profiles, symbols, positions, offsets and zero lengths
below are authored constants; input_hex is used only for transcription.
"""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
V = {v['name']: v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
def wire(name):
    data = bytes.fromhex(V[name]['input_hex']); p = 8; z = b''
    while p+12 <= len(data):
        n = int.from_bytes(data[p:p+4], 'big')
        if data[p+4:p+8] == b'IDAT': z += data[p+8:p+8+n]
        p += n+12
    return z
def seq(xs, ty='u8'):
    return 'Seq::<'+ty+'>::empty()' if not xs else 'seq!['+', '.join(str(x)+(ty if i == 0 else '') for i,x in enumerate(xs))+']'
CL = [4,5,5,5,4,4,4,4,4,4,4,4,4,5,5,5,4,4,4]
lines = ['//! Authored frozen repeat/admission rejections; no decode oracle.', 'use crate::{model as m, witness_dynamic::*, witnesses::{follow_word,wire_word,check_at}, Status, InflateError as E};', 'use vstd::prelude::*;', 'verus! {']
for suffix in ['repeat-with-no-previous', 'repeat-overflows-count', 'code-length-code-oversubscribed', 'code-length-code-incomplete']:
    name = 'deflate/'+suffix; z = wire(name); zs = seq(z)
    lines += [f'/// {name}: authored CL/repeat rejection, R=4.', '#[verifier::spinoff_prover]', f'fn {suffix.replace("-","_")}() {{', '    hide(m::MDynamicLengths); hide(m::MDec); hide(m::decode_prefix);', f'    let wire = [{", ".join(hex(x)+("u8" if i == 0 else "") for i,x in enumerate(z))}];', '    proof {', f'        let z = {zs};', '        assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute);']
    if suffix.startswith('repeat'):
        cl = seq(CL)
        lines += [f'        let code = {cl};', f'        assert(m::Kraft({cl}) == 32768) by (compute); assert(m::admissible(code,m::CodeKind::CodeLen,true));', f'        assert(cl_trace({zs},33,Seq::new(19,|i:int|0u8),0) == {cl}) by (compute);']
        if suffix == 'repeat-overflows-count':
            lines += ['        let values = Seq::new(276,|i:int|0u8);', f'        assert(repeat_trace({zs},{cl},seq![18u8,18],seq![90nat,101,112],seq![0nat,138,276],Seq::new(276,|i:int|0u8),0,2)) by (compute);', '        expand_repeats(z,code,seq![18u8,18],seq![90nat,101,112],seq![0nat,138,276],values,286,2,0);', '        assert(values.take(0) =~= Seq::<u8>::empty());', f'        assert(wire_word({zs},112,4) == m::cw({cl},18).unwrap()) by (compute);', '        follow_word(code,z,112,18,0);', '        assert(m::MDec(code,z,112) == (m::Decoded::Symbol{symbol:18,position:116})) by { reveal(m::MDec); }', f'        assert(m::field_value({zs},116,7) == 127) by (compute);', '        assert(m::MDynamicLengths(z,112,code,values,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:123})) by { reveal(m::MDynamicLengths); }']
            end = 123
        else:
            lines += [f'        assert(wire_word({zs},90,4) == m::cw({cl},16).unwrap()) by (compute);', '        follow_word(code,z,90,16,0);', '        assert(m::MDec(code,z,90) == (m::Decoded::Symbol{symbol:16,position:94})) by { reveal(m::MDec); }', '        assert(m::MDynamicLengths(z,90,code,Seq::empty(),286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:94})) by { reveal(m::MDynamicLengths); }']
            end = 94
        lines += ['        cl_bridge(z,33,Seq::new(19,|i:int|0u8),0,286,2);', f'        assert(m::MDynamicCL(z,33,Seq::new(19,|i:int|0u8),0,19,286,2,true) == (m::DynamicHeader::Error{{error:m::InflateFault::InvalidCodeLengths,position:{end}}}));']
    else:
        end = 90
        width = 3 if suffix.endswith('oversubscribed') else 5
        code = f'Seq::new(19,|i:int|{width}u8)'
        lines += [f'        let code = {code};', f'        assert(cl_trace({zs},33,Seq::new(19,|i:int|0u8),0) == {code}) by (compute);', f'        assert(m::Kraft({code}) == {19*(32768>>width)}) by (compute);', '        assert(!m::admissible(code,m::CodeKind::CodeLen,true));', '        cl_fields(z,33,Seq::new(19,|i:int|0u8),0,286,2);', '        assert(m::MDynamicCL(z,90,code,19,19,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:90}));']
    lines += [f'        assert(m::field_value({zs},19,5) == 29) by (compute); assert(m::field_value({zs},24,5) == 1) by (compute); assert(m::field_value({zs},29,4) == 15) by (compute);', f'        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Error{{error:m::InflateFault::InvalidCodeLengths,position:{end}}}));', '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);', f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 2) by (compute);', f'        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::Error{{error:m::InflateFault::InvalidCodeLengths,out:Seq::empty(),position:{end}}}));', '        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Error(m::InflateFault::InvalidCodeLengths)); assert(m::Inflate(z,4,true,false).out == Seq::<u8>::empty());', f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);', '    }', '    let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));', '}']
lines += ['} // verus!', '']
(ROOT/'src/witness_repeats.rs').write_text('\n'.join(lines))
print('Wrote four independently authored repeat/admission witnesses')
