#!/usr/bin/env python3
"""Transcribe frozen inputs; author dynamic length profiles and symbol effects.

No executable table or oracle is imported. The CL profile, sparse profiles,
positions and output expectations follow the generator's stated wire intent.
"""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
V = {v['name']:v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
def wire(name):
    b=bytes.fromhex(V[name]['input_hex']); p=8; z=b''
    while p+12<=len(b):
        n=int.from_bytes(b[p:p+4],'big')
        if b[p+4:p+8]==b'IDAT': z+=b[p+8:p+8+n]
        p+=n+12
    return z
def seq(xs,ty='u8'):
    return f'Seq::<{ty}>::empty()' if not xs else 'seq!['+', '.join(str(x)+(ty if i==0 else '') for i,x in enumerate(xs))+']'
CL=[4,5,5,5,4,4,4,4,4,4,4,4,4,5,5,5,4,4,4]
CASES=[('deflate/dynamic-literals-two-dist-codes',[1,1]),('deflate/dynamic-single-dist-code-len1',[1]),('deflate/dynamic-no-dist-codes',[0])]
lines=['//! Authored dynamic wire profiles and public-call witnesses.','use crate::{model as m, Status, witnesses::{wire_word,check_at}, witness_dynamic::*};','use vstd::prelude::*;','verus! {']
for name,dist in CASES:
    z=wire(name); zs=seq(z); ds=seq(dist); cl=seq(CL)
    values=[2 if i in (0,65,66,256) else 0 for i in range(286)]+dist
    pos=[90]
    for v in values: pos.append(pos[-1]+CL[v])
    ps=seq(pos,'nat'); end=pos[-1]+10
    lines += [f'/// {name}: authored lengths and literal [0,65,0,66] effects, R=4.','#[verifier::spinoff_prover]',f'fn {name.replace("/","_").replace("-","_")}() {{',
        '    hide(m::MCompressedStep); hide(m::MCompressed);',f'    let wire = [{", ".join(hex(x)+( "u8" if i==0 else "") for i,x in enumerate(z))}];','    proof {',f'        let z = {zs};',f'        let code = {cl};',f'        let dist = {ds};','        let lit = profile(1);', '        assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::NCL == 19) by (compute);',f'        let values = lit + dist;',f'        let positions = {ps};',
        f'        assert(m::Kraft({cl}) == 32768) by (compute);',f'        assert(m::admissible(code,m::CodeKind::CodeLen,true));',
        f'        assert(cl_trace({zs},33,Seq::new(19,|i:int|0u8),0) == {cl}) by (compute);',
        f'        assert(trace_words({zs},{cl},profile(1)+{ds},{ps},0,{len(values)})) by (compute);',
        f'        trace_decodes(z,code,values,positions,0,{len(values)});',
        f'        expand_literals(z,code,values,positions,286,{len(dist)},0);', f'        assert(values.take(0) =~= Seq::<u8>::empty()); assert(positions[0] == 90); assert(positions[values.len() as int] == {pos[-1]});',
        '        profile_facts(1);',f'        assert(m::Kraft({ds}) == {32768 if dist==[1,1] else 16384 if dist==[1] else 0}) by (compute);', f'        assert(m::count_prefix({ds},0,{len(dist)}) == {dist.count(0)}) by (compute);', f'        assert(m::bl_count({ds},1) == {dist.count(1)}) by (compute);', f'        assert(m::admissible(dist,m::CodeKind::Dist,true));',
        f'        assert(values.take(286) =~= lit); assert(values.skip(286) =~= dist);',
        f'        assert(m::finish_dynamic(values,286,{len(dist)},{pos[-1]},true) == (m::DynamicHeader::Tables {{ lit, dist, position: {pos[-1]} }}));',
        f'        assert(m::MDynamicLengths(z,90,code,Seq::empty(),286,{len(dist)},true) == (m::DynamicHeader::Tables {{ lit, dist, position: {pos[-1]} }}));',
        f'        cl_bridge(z,33,Seq::new(19,|i:int|0u8),0,286,{len(dist)});',
        f'        assert(m::MDynamicCL(z,33,Seq::new(19,|i:int|0u8),0,19,286,{len(dist)},true) == (m::DynamicHeader::Tables {{ lit, dist, position: {pos[-1]} }}));',
        f'        assert(m::field_value({zs},19,5) == 29) by (compute);',f'        assert(m::field_value({zs},24,5) == {len(dist)-1}) by (compute);',f'        assert(m::field_value({zs},29,4) == 15) by (compute);',
        f'        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Tables {{ lit, dist, position: {pos[-1]} }}));',
        '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);']
    prefix=[]
    for j,s in enumerate([0,65,0,66,256]):
        p=pos[-1]+2*j; rank={0:0,65:1,66:2,256:3}[s]; rs=seq(prefix)
        lines += [f'        assert(wire_word({zs},{p},2) == m::msb_bits({rank},2)) by (compute);',f'        profile_decode(z,{p},1,{s});']
        if s!=256:
            nxt=seq(prefix+[s]); effect=f'm::SymbolEffect::Next {{ out: {nxt}, position: {p+2} }}'
            lines += [f'        assert({rs}.push({s}) =~= {nxt});']; prefix.append(s)
        else: effect=f'm::SymbolEffect::End {{ out: {rs}, position: {p+2} }}'
        lines += [f'        assert(m::MCompressedStep(z,{p},{rs},4,32768,lit,dist) == ({effect})) by {{ reveal(m::MCompressedStep); }}']
    raw=seq([0,65,0,66]); effect=f'm::StoredEffect::End {{ out: {raw}, position: {end} }}'
    lines += [f'        assert(m::MCompressed(z,{pos[-1]},Seq::empty(),4,32768,lit,dist) == ({effect})) by {{ reveal_with_fuel(m::MCompressed,6); }}',f'        assert(m::field_value({zs},16,1) == 1) by (compute);',f'        assert(m::field_value({zs},17,2) == 2) by (compute);',f'        assert(m::MOneBlock(z,16,Seq::empty(),4,32768,true) == (true,{effect}));',f'        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == ({effect}));',f'        assert(m::MFinish({zs},{end},{raw},4,false) == (m::StoredEffect::End {{ out: {raw}, position: {8*len(z)} }})) by (compute);',f'        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Done);',f'        assert(m::Inflate(z,4,true,false).out == {raw});',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }', '    let mut output = [0xa5u8;4];',f'    let ghost raw = {raw};','    check_at(&wire,&mut output,Status::Done,Ghost(raw));','}']
lines += ['} // verus!','']
(ROOT/'src/witness_dynamic_cases.rs').write_text('\n'.join(lines))
print(f'Wrote {len(CASES)} authored dynamic witnesses without an oracle')
