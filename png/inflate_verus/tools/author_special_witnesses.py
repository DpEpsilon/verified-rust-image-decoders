#!/usr/bin/env python3
"""Author small error outcomes from explicit token/scanline intent, not an oracle."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
CORE={v['name']:v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
POLICY={v['name']:v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}
def extract(data):
    p=8;idat=None;frames=[]
    while p+12<=len(data):
        n=int.from_bytes(data[p:p+4],'big');k=data[p+4:p+8];b=data[p+8:p+8+n];p+=n+12
        if k==b'IDAT':
            if idat is None:idat=[]
            idat.append(b)
        elif k==b'fcTL':frames.append(None)
        elif k==b'fdAT' and len(b)>=4 and frames:
            if frames[-1] is None:frames[-1]=[]
            frames[-1].append(b[4:])
    return ([] if idat is None else [idat])+[f for f in frames if f is not None]
def seq(xs):return 'seq!['+', '.join(str(x)+('u8' if i==0 else '') for i,x in enumerate(xs))+']' if xs else 'Seq::<u8>::empty()'
claims=[
 {'class':'empty_fdat','domain':'core','members':[['apng/empty-fdat-payload',1],['apng/empty-fdat-payload-still',1]],'r':5,'raw':[],'status':'TruncatedZlib'},
 {'class':'core_tail_later_idat','domain':'core','members':[['zlib/trailing-in-later-idat',0]],'r':4,'raw':[0,65,0,66],'status':'TrailingZlibData'},
 {'class':'policy_distance_before','domain':'policy','members':[['policy/window/distance-before-output-stays-an-error/strict',0],['policy/window/distance-before-output-stays-an-error/recommended-beta',0]],'r':4,'raw':[],'status':'DistanceTooFar'},
]
lines=['//! Explicit small error gates, with original IDAT/fdAT payload boundaries.',
'use crate::{model as m, witness_policy::check_policy_parts, witnesses::{fixed_decode,fixed_dist_decode,wire_word}, Status, InflateError as E};','use vstd::prelude::*;','verus! {',
'proof fn before_output(z: Seq<u8>, limit: nat, enforce: bool, tail: bool)',
'    requires z == seq![120u8,1,3,2,0,0,0,1],',
'    ensures m::Inflate(z,limit,enforce,tail).status == m::InflateStatus::Error(m::InflateFault::DistanceTooFar),',
'        m::Inflate(z,limit,enforce,tail).out == Seq::<u8>::empty(),',
'{','    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressedStep); hide(m::MCompressed);',
'    assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);',
'    assert(wire_word(seq![120u8,1,3,2,0,0,0,1],19,7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,19,257);',
'    assert(wire_word(seq![120u8,1,3,2,0,0,0,1],26,5) == m::msb_bits(0,5)) by (compute); fixed_dist_decode(z,26,0);',
'    assert(m::MCopy(Seq::<u8>::empty(),limit,32768,1,3) == (m::OutputEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty()}));',
'    assert(m::MCompressedStep(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31})) by {reveal(m::MCompressedStep);}',
'    assert(m::MCompressed(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31})) by {reveal_with_fuel(m::MCompressed,2);}',
'    assert(m::field_value(seq![120u8,1,3,2,0,0,0,1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8,1,3,2,0,0,0,1],17,2) == 1) by (compute);',
'    assert(m::MBlk(z,16,Seq::empty(),limit,32768,true) == (m::StoredEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31}));','}']
for c in claims:
    name,index=c['members'][0];v=(CORE if c['domain']=='core' else POLICY)[name];ps=extract(bytes.fromhex(v['input_hex']))[index];zs=seq(b''.join(ps));rs=seq(c['raw']);r=c['r']
    c['segment_lengths']=[len(p) for p in ps]
    lines+=['#[verifier::spinoff_prover]',f"fn {c['class']}() {{"]
    for i,p in enumerate(ps):lines += [f'    let segment_{i}: [u8;{len(p)}] = ['+', '.join(hex(x)+('u8' if j==0 else '') for j,x in enumerate(p))+'];']
    lines += [f'    let parts: [&[u8];{len(ps)}] = ['+', '.join('&segment_'+str(i) for i in range(len(ps)))+'];',
              '    proof {',f'        reveal_with_fuel(crate::segments::parts_prefix,{len(ps)+1});',f'        assert(crate::segments::parts_prefix(parts@,{len(ps)}) =~= {zs});',
              f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {sum(len(p) for p in ps)}) by (compute);','    }',f'    let ghost raw = {rs};']
    options=[(L,e,t) for L in (r,r+1) for e in (True,False) for t in (False,True)] if c['domain']=='policy' else [(r,True,False)]
    for L,e,t in options:
        ee,tt=str(e).lower(),str(t).lower()
        if c['domain']=='policy':lines += [f'    proof {{before_output({zs},{L},{ee},{tt});}}']
        else:lines += ['    proof {',f'        assert(m::Inflate({zs},{L},{ee},{tt}).status == m::InflateStatus::Error(m::InflateFault::{c["status"]})) by (compute);',f'        assert(m::Inflate({zs},{L},{ee},{tt}).out == {rs}) by (compute);','    }']
        lines += [f'    let mut output = [0xa5u8;{L}]; check_policy_parts(&parts,&mut output,{ee},{tt},Status::Err(E::{c["status"]}),Ghost(raw));']
    lines+=['}']
lines+=['} // verus!','']
(ROOT/'src/witness_special.rs').write_text('\n'.join(lines));(ROOT/'tools/witness_special.json').write_text(json.dumps(claims,indent=2)+'\n')
print('Authored three small error families: three core streams and two policy records')
