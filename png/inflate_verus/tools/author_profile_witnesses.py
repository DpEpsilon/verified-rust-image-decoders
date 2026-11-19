#!/usr/bin/env python3
"""Author both remaining small dynamic streams from forward Huffman intent."""
import json
import re
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import author_extended_classes as a
import author_huffman_intent as h
V={v['name']:v for v in a.V}
def extract(data):
    idat=None;frames=[]
    for k,b in a.chunks(data):
        if k==b'IDAT':
            if idat is None:idat=[]
            idat.append(b)
        elif k==b'fcTL':frames.append(None)
        elif k==b'fdAT' and frames and len(b)>=4:
            if frames[-1] is None:frames[-1]=[]
            frames[-1].append(b[4:])
    return ([] if idat is None else [idat])+[f for f in frames if f is not None]
def sparse(xs):
    return f'Seq::new({len(xs)},|i:int| '+''.join(f'if i == {i} {{{n}u8}} else ' for i,n in enumerate(xs) if n)+'{0u8})'
vals=[0,127,128,383,384,32767,32768,32895,32896,65407,65408,65535]
raw0=a.scan_intent([(v,) for v in vals],12,1,0,16,0,0)
px=[(255,0,0,255),(0,255,0,255),(0,0,255,255),(0,0,255,128)]*2+[(255,0,0,255)]
raw1=a.scan_intent(px,3,3,6,8,1)
lines=['//! Authored dynamic profiles, repeat fields and sequence-copy effects.',
'use crate::{model as m, witness_dynamic as d, witness_profile_bridge as b, witnesses::{follow_word,wire_word}, witness_match as t, witness_policy::check_policy_parts, Status};','use vstd::prelude::*;','verus! {']
claims=[]
for ident,name,index,raw,r in [('rounding','depth16/rounding-boundaries',0,raw0,25),('subframe','apng/interlaced-subframe',1,raw1,42)]:
    ps=extract(bytes.fromhex(V[name]['input_hex']))[index];z=b''.join(ps);zs=a.seq(z);rs=a.seq(raw)
    tokens=a.greedy(raw,1,False,False,1);body,lit,dist,cl,rr,start,end=h.encode(tokens)
    assert z[2:-4]==body,name
    assert len(raw)==r,(name,len(raw),r)
    order=[16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15];nc=max(4,max(i for i,s in enumerate(order) if cl[s])+1)
    values=lit+dist;symbols=[s for s,_,_ in rr];positions=[33+3*nc];offsets=[0]
    for s,ex,n in rr:
        positions.append(positions[-1]+cl[s]+n)
        offsets.append(offsets[-1]+(1 if s<16 else ex+(11 if s==18 else 3)))
    assert positions[-1]==start and offsets[-1]==len(values)
    ls,ds,cs=sparse(lit),sparse(dist),a.seq(cl);ss=a.seq(symbols);poss=a.seq(positions,'nat');offs=a.seq(offsets,'nat')
    lines += [f'spec fn {ident}_z() -> Seq<u8> {{ {zs} }}',f'spec fn {ident}_raw() -> Seq<u8> {{ {rs} }}',f'spec fn {ident}_lit() -> Seq<u8> {{ {ls} }}',f'spec fn {ident}_dist() -> Seq<u8> {{ {ds} }}',f'spec fn {ident}_cl() -> Seq<u8> {{ {cs} }}']
    for j,part in enumerate(ps):lines += [f'fn {ident}_part_{j}() -> (wire: [u8;{len(part)}])',f'    ensures wire@ == {a.seq(part)},','{','    ['+', '.join(hex(x)+('u8' if k==0 else '') for k,x in enumerate(part))+']','}']
    lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_header()',f'    ensures m::MDynamic({ident}_z(),19,true) == (m::DynamicHeader::Tables {{lit:{ident}_lit(),dist:{ident}_dist(),position:{start}}}),','{',
        '    hide(m::MDec); hide(m::decode_prefix); hide(m::MDynamicLengths);',f'    let z = {ident}_z(); let lit = {ident}_lit(); let dist = {ident}_dist(); let code = {ident}_cl();',
        '    assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);']
    for fn in ('lit','dist','cl'):lines += [f'    assert(m::Kraft({ident}_{fn}()) == 32768) by (compute);']
    lines += ['    assert(m::admissible(lit,m::CodeKind::LitLen,true)); assert(m::admissible(dist,m::CodeKind::Dist,true)); assert(m::admissible(code,m::CodeKind::CodeLen,true));',f'    let values = lit+dist;',f'    assert(values =~= {a.seq(values)});',
        f'    assert(d::repeat_trace({zs},{cs},{ss},{poss},{offs},{a.seq(values)},0,{len(rr)})) by (compute);',f'    d::expand_repeats(z,code,{ss},{poss},{offs},values,{len(lit)},{len(dist)},0);',
        f'    assert(values.take(0) =~= Seq::<u8>::empty()); assert(values.take({len(lit)}) =~= lit); assert(values.skip({len(lit)}) =~= dist);',
        f'    assert(m::MDynamicLengths(z,{start},code,values,{len(lit)},{len(dist)},true) == (m::DynamicHeader::Tables {{lit,dist,position:{start}}})) by {{ reveal(m::MDynamicLengths); }}',
        f'    assert(b::fields({zs},33,Seq::new(19,|i:int|0u8),0,{nc}) == {cs}) by (compute);',f'    b::bridge(z,33,Seq::new(19,|i:int|0u8),0,{nc},{len(lit)},{len(dist)});',
        f'    assert(m::field_value({zs},19,5) == {len(lit)-257}) by (compute); assert(m::field_value({zs},24,5) == {len(dist)-1}) by (compute); assert(m::field_value({zs},29,4) == {nc-4}) by (compute);','}',
        '#[verifier::spinoff_prover]',f'proof fn {ident}_payload()',f'    ensures m::MCompressed({ident}_z(),{start},Seq::empty(),{r},32768,{ident}_lit(),{ident}_dist()) == (m::StoredEffect::End {{out:{ident}_raw(),position:{end}}}),','{',
        '    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::copy_distance);',f'    let z = {ident}_z(); let raw = {ident}_raw(); let lit = {ident}_lit(); let dist = {ident}_dist();',
        '    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);',f'    assert(m::Kraft({ident}_lit()) == 32768) by (compute); assert(m::Kraft({ident}_dist()) == 32768) by (compute);']
    p=start;before=0;steps=[]
    for token in tokens+[256]:
        if isinstance(token,int):s=token;le=dsym=de=n=distance=0;dp=ep=0;nxt=p+lit[s];after=before+(s<256)
        else:
            n,distance=token;s,le,ln,dsym,de,dn=a.codes(n,distance);dp=p+lit[s]+ln;ep=dp+dist[dsym];nxt=ep+dn;after=before+n
        steps.append(dict(pos=p,next=nxt,before=before,after=after,symbol=s,le=le,ds=dsym,de=de,length=n,distance=distance))
        lines += [f'    assert(wire_word({zs},{p},{lit[s]}) == m::cw({ident}_lit(),{s}).unwrap()) by (compute);',f'    follow_word(lit,z,{p},{s},0);',f'    assert(m::MDec(lit,z,{p}) == (m::Decoded::Symbol {{symbol:{s},position:{p+lit[s]}}})) by {{reveal(m::MDec);}}']
        if s>256:
            lines += [f'    assert(m::field_value({zs},{p+lit[s]},{a.LEXTRA[s-257]}) == {le}) by (compute);',f'    assert(wire_word({zs},{dp},{dist[dsym]}) == m::cw({ident}_dist(),{dsym}).unwrap()) by (compute);',f'    follow_word(dist,z,{dp},{dsym},0);',f'    assert(m::MDec(dist,z,{dp}) == (m::Decoded::Symbol {{symbol:{dsym},position:{ep}}})) by {{reveal(m::MDec);}}',f'    assert(m::field_value({zs},{ep},{a.DEXTRA[dsym]}) == {de}) by (compute);',f'    assert(t::copy_bytes({rs},{before},{distance},0,{n})) by (compute);',f'    t::copy_quant(raw,{before},{distance},0,{n}); t::copy_prefix(raw,{before},{distance},{n});',f'    assert(m::MCopy(raw.take({before}),{r},32768,{distance},{n}) == (m::OutputEffect::Next {{out:raw.take({after})}}));']
        if s<256:lines += [f'    assert(raw.take({before}).push({s}u8) =~= raw.take({after}));']
        if s==256:lines += [f'    assert(raw.take({r}) =~= raw);']
        if s>256:
            lines += [f'    assert(m::length_extra({s}) == {a.LEXTRA[s-257]}) by (compute); assert(m::length_base({s}) == {n-le}) by (compute);',f'    assert(m::distance_extra({dsym}) == {a.DEXTRA[dsym]}) by (compute); assert(m::distance_base({dsym}) == {distance-de}) by (compute);', '    assert(crate::limits::NDIST_MAX == 30) by (compute);']
        effect=('End {out:raw' if s==256 else f'Next {{out:raw.take({after})')+f',position:{nxt}'+'}'
        lines += [f'    assert(m::MCompressedStep(z,{p},raw.take({before}),{r},32768,lit,dist) == (m::SymbolEffect::{effect})) by {{reveal(m::MCompressedStep);}}']
        p=nxt;before=after
    lines += ['    assert(raw.take(0) =~= Seq::<u8>::empty());',f'    assert(m::MCompressed(z,{start},Seq::empty(),{r},32768,lit,dist) == (m::StoredEffect::End {{out:raw,position:{end}}})) by {{reveal_with_fuel(m::MCompressed,{len(steps)+1});}}','}',
        '#[verifier::spinoff_prover]',f'fn {ident}() {{','    hide(m::MDec); hide(m::MDynamic); hide(m::MCompressed); hide(m::sum_bytes); hide(m::sum_prefixes);']
    for j in range(len(ps)):lines += [f'    let segment_{j} = {ident}_part_{j}();']
    lines += [f'    let parts: [&[u8];{len(ps)}] = ['+', '.join('&segment_'+str(j) for j in range(len(ps)))+'];',
        '    proof {',f'        {ident}_header(); {ident}_payload();',f'        let z = {ident}_z(); let raw = {ident}_raw();',
        '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);',
        f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 2) by (compute);',
        f'        assert(m::MBlk(z,16,Seq::empty(),{r},32768,true) == (m::StoredEffect::End {{out:raw,position:{end}}}));',f'        assert(m::MFinish({zs},{end},{rs},{r},false) == (m::StoredEffect::End {{out:{rs},position:{8*len(z)}}})) by (compute);',f'        assert(m::Inflate(z,{r},true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,{r},true,false).out == raw);',f'        reveal_with_fuel(crate::segments::parts_prefix,{len(ps)+1}); assert(crate::segments::parts_prefix(parts@,{len(ps)}) =~= z);',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',f'    let ghost raw = {ident}_raw(); let mut output = [0xa5u8;{r}]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));','}']
    claims.append({'class':ident,'members':[[name,index]],'domain':'core','r':r,'raw':raw,'tokens':tokens,'lit':lit,'dist':dist,'cl':cl,'repeat_symbols':symbols,'repeat_positions':positions,'repeat_offsets':offsets,'steps':steps,'segment_lengths':[len(x) for x in ps],'calls':[{'limit':r,'enforce':True,'ignore_tail':False,'status':'Done','raw':raw}]})
lines += ['} // verus!','']
text='\n'.join(lines)
for claim in claims:
    ident=claim['class']
    pattern=r'proof fn '+ident+r'_payload\(\)\n.*?\n\{(.*?)\n\}'
    match=re.search(pattern,text,re.S);assert match
    body=match[1].splitlines();starts=[i for i,line in enumerate(body) if 'assert(wire_word(' in line and ident+'_lit()' in line]
    tail_start=next(i for i,line in enumerate(body) if 'assert(raw.take(0) =~=' in line)
    common=body[:starts[0]];kraft=[line for line in common if 'assert(m::Kraft(' in line]
    # Bind the small sequence shapes explicitly rather than unfolding every
    # nested push while the solver composes a match or end-of-block effect.
    common_steps=[line.replace('hide(m::MCompressed); ','') for line in common if 'assert(m::Kraft(' not in line]+[f'    {ident}_profiles();', f'    assert({ident}_z().len() == {sum(claim["segment_lengths"])}) by (compute); assert({ident}_raw().len() == {claim["r"]}) by (compute);']
    helpers=['#[verifier::spinoff_prover]',f'proof fn {ident}_profiles()',f'    ensures m::lengths_ok({ident}_lit()), m::lengths_ok({ident}_dist()),',f'        m::Kraft({ident}_lit()) == 32768, m::Kraft({ident}_dist()) == 32768,','{']+[line for line in common if not line.strip().startswith('hide(')]+['}']
    for j,start in enumerate(starts):
        end=starts[j+1] if j+1<len(starts) else tail_start;chunk=body[start:end]
        line=next(line for line in reversed(chunk) if 'assert(m::MCompressedStep(' in line)
        expr=re.search(r'assert\((.*)\) by \{reveal\(m::MCompressedStep\);\}',line)[1]
        for var in ('z','raw','lit','dist'):expr=re.sub(r'\b'+var+r'\b',ident+'_'+var+'()',expr)
        helpers+=['#[verifier::spinoff_prover]',f'proof fn {ident}_step_{j}()',f'    ensures {expr},','{']+common_steps+chunk+['}']
    signature=match[0][:match[0].index('\n{')]
    final_body=['    hide(m::MCompressedStep); hide(m::MCompressed);',f'    let z = {ident}_z(); let raw = {ident}_raw(); let lit = {ident}_lit(); let dist = {ident}_dist();']+[f'    {ident}_step_{j}();' for j in range(len(starts))]+body[tail_start:]
    replacement='\n'.join(helpers)+'\n#[verifier::spinoff_prover]\n'+signature+'\n{\n'+'\n'.join(final_body)+'\n}'
    text=text[:match.start()]+replacement+text[match.end():]
text=text.replace('#[verifier::spinoff_prover]\n#[verifier::spinoff_prover]','#[verifier::spinoff_prover]')
(ROOT/'src/witness_profiles.rs').write_text(text)
(ROOT/'tools/witness_profiles.json').write_text(json.dumps(claims,indent=2)+'\n')
print('Authored two complete dynamic profiles and exact public-call descriptors')
