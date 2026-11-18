#!/usr/bin/env python3
"""Author fixed frame operands at their own R, checksum errors and later tails.

Raw scanlines and token choices below are authored from sample/filter intent.
Forward RFC fixed encoding checks eligibility; it never decodes the input.
"""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
V={v['name']:v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
def extract(data):
    p=8;idat=None;frames=[]
    while p+12<=len(data):
        n=int.from_bytes(data[p:p+4],'big');k=data[p+4:p+8];b=data[p+8:p+8+n];p+=n+12
        if k==b'IDAT':
            if idat is None:idat=[]
            idat.append(b)
        elif k==b'fcTL':frames.append(None)
        elif k==b'fdAT' and frames and len(b)>=4:
            if frames[-1] is None:frames[-1]=[]
            frames[-1].append(b[4:])
    return ([] if idat is None else [idat])+[f for f in frames if f is not None]
def seq(xs,ty='u8'):
    return 'seq!['+', '.join(str(x)+(ty if i==0 else '') for i,x in enumerate(xs))+']'
LBASE=[3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258]
LEXTRA=[0]*8+[1]*4+[2]*4+[3]*4+[4]*4+[5]*4+[0]
DBASE=[1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577]
DEXTRA=[0]*4+[i for i in range(1,14) for _ in range(2)]
def fields(n,d):
    li=max(i for i,b in enumerate(LBASE) if b<=n);di=max(i for i,b in enumerate(DBASE) if b<=d)
    return 257+li,n-LBASE[li],LEXTRA[li],di,d-DBASE[di],DEXTRA[di]
def width(s):return 8 if s<144 else 9 if s<256 else 7 if s<280 else 8
def word(s):return 48+s if s<144 else 256+s if s<256 else s-256 if s<280 else s-88
GREEN=[0,0,255,0,255,0,255,0,255,1,0,255,0,255,0,0,0,0]
GT=[0,0,255,(6,2),1,(5,7),(3,1)]
RGB=[0,201,73,63,202,75,151,1,225,26,23,27,220,106]
claims=[
 {'class':'frame_adler_error','members':[['apng/frame1-adler-error-ends',1],['apng/frame1-error-still-unaffected',1]],'r':18,'raw':GREEN,'tokens':GT,'status':'AdlerMismatch'},
 {'class':'green_canvas_r','members':[['apng-bad/fdat-for-idat-frame',1],['apng-bad/fdat-for-idat-frame-still',1],['apng-bad/fdat-before-idat',1]],'r':51,'raw':GREEN,'tokens':GT,'status':'Done'},
 {'class':'short_successful_frame','members':[['apng/frame1-truncated-data-ends',1]],'r':18,'raw':GREEN[:-1],'tokens':[0,0,255,(6,2),1,(5,7),0,0],'status':'Done'},
 {'class':'palette_five_entries','members':[['plte/5-entries-depth-2',0]],'r':4,'raw':[0,16,1,144],'tokens':[0,16,1,144],'status':'Done'},
 {'class':'idat_after_iend_tail','members':[['container/idat-after-iend',0]],'r':14,'raw':RGB,'tokens':RGB,'status':'TrailingZlibData'},
]
lines=['//! Authored fixed frame/checksum/tail witnesses at independent per-stream R.',
'use crate::{model as m, witness_match as t, witness_checksum as c, witness_policy::check_policy_parts, Status, InflateError as E};',
'use vstd::prelude::*;','verus! {']
for claim in claims:
    ident=claim['class'];raw=claim['raw'];tokens=claim['tokens'];r=claim['r'];name,index=claim['members'][0]
    parts=extract(bytes.fromhex(V[name]['input_hex']))[index];z=b''.join(parts);zs=seq(z);rs=seq(raw)
    bits=[1,1,0];steps=[];p=19;before=0
    def field(v,n):bits.extend((v>>i)&1 for i in range(n))
    def code(s):bits.extend((word(s)>>i)&1 for i in range(width(s)-1,-1,-1))
    for token in tokens+[256]:
        if isinstance(token,int):
            s=token;le=ds=de=length=distance=0;after=before+(s<256);nxt=p+width(s);code(s)
        else:
            length,distance=token;s,le,ln,ds,de,dn=fields(*token);after=before+length;nxt=p+width(s)+ln+5+dn
            code(s);field(le,ln);bits.extend((ds>>i)&1 for i in range(4,-1,-1));field(de,dn)
        steps.append(dict(pos=p,next=nxt,before=before,after=after,symbol=s,le=le,ds=ds,de=de,length=length,distance=distance));p=nxt;before=after
    body=bytes(sum(v<<j for j,v in enumerate(bits[i:i+8])) for i in range(0,len(bits),8))
    assert z[2:2+len(body)]==body, ident
    ts='seq!['+', '.join('t::Step {'+', '.join(f'{k}:{v}' for k,v in s.items())+'}' for s in steps)+']'
    claim['steps']=steps;claim['segment_lengths']=[len(x) for x in parts]
    sb=sum(raw);sp=sum((len(raw)-i)*v for i,v in enumerate(raw))+len(raw);adler=(sp%65521)*65536+(1+sb)%65521
    lines+=['#[verifier::spinoff_prover]',f'fn {ident}() {{',
        '    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);']
    for i,part in enumerate(parts):lines+=[f'    let segment_{i}: [u8;{len(part)}] = ['+', '.join(hex(x)+('u8' if j==0 else '') for j,x in enumerate(part))+'];']
    lines+=[f'    let parts: [&[u8];{len(parts)}] = ['+', '.join('&segment_'+str(i) for i in range(len(parts)))+'];',
        '    proof {',f'        let z = {zs}; let raw = {rs}; let ts = {ts};',
        f'        assert(t::trace({zs},{rs},{ts},0,{len(steps)})) by (compute);',
        f'        t::effects_at(z,raw,ts,0,{len(steps)},{r}); t::expand_at(z,raw,ts,0,{r});',
        '        assert(raw.take(0) =~= Seq::<u8>::empty());',
        '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);',
        f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 1) by (compute);',
        f'        assert(m::MBlk(z,16,Seq::empty(),{r},32768,true) == (m::StoredEffect::End {{out:raw,position:{p}}}));',
        f'        assert(c::sums({rs},0,{len(raw)}) == ({sb}nat,{sp}nat)) by (compute);',
        f'        c::sums_correct(raw,0,{len(raw)}); assert(raw.subrange(0,{len(raw)}) =~= raw);',
        '        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',
        f'        assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute); assert(m::Adler(raw) == {adler});',
        f'        assert(m::MFinish({zs},{p},{rs},{r},false) == (m::StoredEffect::'+('End' if claim['status']=='Done' else 'Error')+' {out:'+rs+',position:'+str(8*min(len(z),(p+7)//8+(4 if claim['status']!='TrailingZlibData' else 5)))+('' if claim['status']=='Done' else ',error:m::InflateFault::'+claim['status'])+'})) by (compute);',
        f'        assert(m::Inflate(z,{r},true,false).status == '+('m::InflateStatus::Done' if claim['status']=='Done' else 'm::InflateStatus::Error(m::InflateFault::'+claim['status']+')')+');',
        f'        assert(m::Inflate(z,{r},true,false).out == raw);',
        f'        reveal_with_fuel(crate::segments::parts_prefix,{len(parts)+1}); assert(crate::segments::parts_prefix(parts@,{len(parts)}) =~= z);',
        f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',
        f'    let ghost raw = {rs}; let mut output = [0xa5u8;{r}]; check_policy_parts(&parts,&mut output,true,false,'+('Status::Done' if claim['status']=='Done' else 'Status::Err(E::'+claim['status']+')')+',Ghost(raw));','}']
lines+=['} // verus!','']
(ROOT/'src/witness_finish.rs').write_text('\n'.join(lines))
(ROOT/'tools/witness_finish.json').write_text(json.dumps(claims,indent=2)+'\n')
print('Authored',len(claims),'fixed finishing classes covering',sum(len(x['members']) for x in claims),'streams')
