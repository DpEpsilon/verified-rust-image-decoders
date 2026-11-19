#!/usr/bin/env python3
"""Author fixed-stream window/limit gates from independent sample and encoder intent.

The imported authoring module replays samples and forward filters only; importing
it does not write generated files. Complete forward encodings select eligible
traces. Neither raw bytes nor effects come from reading compressed symbols.
"""
import json
import re
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import author_extended_classes as a
V={v['name']:v for v in a.V}
P={v['name']:v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}
def parts(data):
    return [b for k,b in a.chunks(data) if k==b'IDAT']
def candidate(raw,lazy=False,minimum=3,origin=1):
    # Independent slow-encoder choice: shortest-distance longest match; optional
    # one-position lazy lookahead and filtered-strategy short-match suppression.
    ts=[];p=0
    def match(p):
        n,d=a.match(raw,p,False,origin) if p>=origin else (0,0)
        return (n,d) if n>=minimum else (0,0)
    while p<len(raw):
        n,d=match(p)
        if n and lazy and match(p+1)[0]>n:n=0
        if n:ts.append((n,d));p+=n
        else:ts.append(raw[p]);p+=1
    return ts

def fast_candidate(raw):
    # Level-one forward policy: stop at a match of eight bytes, inspect at most
    # four equal three-byte keys, and insert skipped positions only for matches
    # no longer than four bytes. The search never consults compressed input.
    p=0;tokens=[];inserted=[]
    while p<len(raw):
        n=d=0;seen=0
        for q in reversed(inserted):
            if raw[q:q+3]!=raw[p:p+3]:continue
            seen+=1;length=0
            while length<258 and p+length<len(raw) and raw[q+length]==raw[p+length]:length+=1
            if length>=3 and length>n:n,d=length,p-q
            if n>=8 or seen>=4:break
        if p:inserted.append(p)
        if n:
            tokens.append((n,d))
            if n<=4:inserted.extend(range(p+1,p+n))
            p+=n
        else:tokens.append(raw[p]);p+=1
    return tokens

def steps(tokens):
    out=[];p=19;before=0
    for token in tokens+[256]:
        if isinstance(token,int):
            s=token;le=ds=de=n=d=0;nxt=p+(8 if s<144 else 9 if s<256 else 7);after=before+(s<256)
        else:
            n,d=token;s,le,ln,ds,de,dn=a.codes(n,d);nxt=p+(7 if s<280 else 8)+ln+5+dn;after=before+n
        out.append(dict(pos=p,next=nxt,before=before,after=after,symbol=s,le=le,ds=ds,de=de,length=n,distance=d));p=nxt;before=after
    return out

def effect(raw,ss,L,W):
    # Effects follow authored tokens, never compressed-input interpretation.
    for s in ss:
        if s['symbol']==256:return 'Done',raw,s['next']
        if s['symbol']>256 and s['distance']>W:return 'DistanceTooFar',raw[:s['before']],s['next']
        if s['after']>L:return 'Full',raw[:L],None
    raise AssertionError('authored trace has no end')
def model_effect(status,raw,pos):
    rs=a.seq(raw)
    if status=='Full':return 'm::StoredEffect::Full {out:'+rs+'}'
    if status=='Done':return 'm::StoredEffect::End {out:'+rs+',position:'+str(pos)+'}'
    return 'm::StoredEffect::Error {error:m::InflateFault::'+status+',out:'+rs+',position:'+str(pos)+'}'
def ms(s):return 'm::InflateStatus::'+s if s in ('Done','Full') else 'm::InflateStatus::Error(m::InflateFault::'+s+')'
def es(s):return 'Status::'+s if s in ('Done','Full') else 'Status::Err(E::'+s+')'

claims=[]
def add(ident,name,raw,tokens=None,domain='core',records=None,finish='Done'):
    vectors=V if domain=='core' else P;data=bytes.fromhex(vectors[name]['input_hex']);ps=parts(data);z=b''.join(ps)
    if tokens is None:
        for lazy in (False,True):
            for minimum in (3,6):
                trial=candidate(raw,lazy,minimum)
                if a.encode(trial)==z[2:-4]:tokens=trial;break
            if tokens is not None:break
    assert tokens is not None, name
    body=a.encode(tokens)
    assert z[2:2+len(body)]==body, name
    r=a.image_r(data);assert r is not None
    ss=steps(tokens);n=len(raw);adler=(sum((n-i)*v for i,v in enumerate(raw))+n)%65521*65536+(1+sum(raw))%65521
    calls=[]
    for L,e,t in ([(L,e,t) for L in (r,r+1) for e in (True,False) for t in (False,True)] if domain=='policy' else [(r,True,False)]):
        W=2**(z[0]//16+8) if e else 32768;status,output,pos=effect(raw,ss,L,W)
        if status=='Done':
            status=finish
            if status=='Done' and (ss[-1]['next']+7)//8+4<len(z):status='Done' if t and n==L else 'TrailingZlibData'
        calls.append(dict(limit=L,enforce=e,ignore_tail=t,status=status,raw=output,window=W))
    claims.append(dict(class_=ident,domain=domain,members=[[x,0] for x in (records or [name])],r=r,raw=raw,steps=ss,tokens=tokens,segment_lengths=[len(x) for x in ps],calls=calls,intent={'schedule':'base-or-window-authored','record':name},adler=adler,wire=z,parts=ps))

for suffix in ('level1-default','level1-fixed','level1-filtered'):
    add(suffix.replace('-','_'),'zlib/'+suffix,a.base_raw,fast_candidate(a.base_raw))
for suffix in ('level6-fixed','level9-fixed','level6-filtered','level9-filtered','window-9','window-10','window-12'):
    add(suffix.replace('-','_'),'zlib/'+suffix,a.base_raw)
add('short_image','image/truncated-data',a.base_raw[:-5])
for suffix,value in [('5',5),('255',255)]:
    raw=a.base_raw.copy();raw[93*3]=value;add('filter_type_'+suffix,'image/filter-type-'+suffix,raw)
raw=a.base_raw.copy();raw[0]=9
add('adler_before_filter','precedence/adler-before-filter',raw,candidate(raw),finish='AdlerMismatch')
for suffix,raw in [('data',a.base_raw+[0]),('row',a.base_raw+a.base_raw[:93])]:add('excess_'+suffix,'image/excess-'+suffix,raw)
add('empty_adam7_passes','interlace/1x1-filter-bytes-for-empty-passes',[0,5]+[0]*6)
raw=a.window_raw[:400]+a.window_raw[100:103];tokens=a.window_raw[:400]+[(3,300)]
for suffix in ('beyond','within'):add('declared_'+suffix,'deflate/distance-'+suffix+'-declared-window',raw,tokens)
for suffix,raw in [('excess-output-stays-an-error',a.base_raw+[0]),('excess-row-stays-an-error',a.base_raw+a.base_raw[:93]),('excess-output-and-trailing-byte',a.base_raw+[0])]:
    prefix='policy/zlib-tail/'+suffix
    add('policy_'+suffix.replace('-','_'),prefix+'/strict',raw,candidate(raw),domain='policy',records=[prefix+'/strict',prefix+'/recommended-beta'])
prefix='policy/window/distance-beyond-declared-window'
add('policy_declared_beyond',prefix+'/strict',a.window_raw[:400]+a.window_raw[100:103],a.window_raw[:400]+[(3,300)],domain='policy',records=[prefix+'/strict',prefix+'/recommended-beta'])

# Reuse one representative only when its complete input, independent R, raw
# intent and option/result descriptors agree; retain every frozen member.
merged=[]
for claim in claims:
    equal=next((c for c in merged if all(c[key] == claim[key] for key in
               ('domain','wire','parts','r','raw','steps','calls'))),None)
    if equal is None:merged.append(claim)
    else:equal['members'].extend(claim['members'])
claims=merged

lines=['//! Independently authored fixed-window, image-limit and policy gates.',
'use crate::{model as m, witness_match as t, witness_checksum as c, witness_config as g, witness_policy::check_policy_parts, Status, InflateError as E};','use vstd::prelude::*;','verus! {']
def step_text(s):return 't::Step {'+', '.join(f'{k}:{v}' for k,v in s.items())+'}'
def halves(a,b,depth):
    if not depth:return [(a,b)]
    mid=a+(b-a)//2
    return halves(a,mid,depth-1)+halves(mid,b,depth-1)
def symbolic_effect(status,out,pos):
    if status=='Full':return f'm::StoredEffect::Full {{out:{out}}}'
    if status=='Done':return f'm::StoredEffect::End {{out:{out},position:{pos}}}'
    return f'm::StoredEffect::Error {{error:m::InflateFault::{status},out:{out},position:{pos}}}'
for cl in claims:
    ident=cl.pop('class_');cl['class']=ident;z=cl['wire'];raw=cl['raw'];ss=cl['steps'];ps=cl['parts'];n=len(raw);assert len(ps)==1
    zs=a.seq(z);rs=a.seq(raw);ts='seq!['+', '.join(step_text(s) for s in ss)+']';ranges=halves(0,len(ss),3);cl['trace_chunks']=ranges
    lines += [f'spec fn {ident}_z() -> Seq<u8> {{ {zs} }}',f'spec fn {ident}_raw() -> Seq<u8> {{ {rs} }}',f'spec fn {ident}_ts() -> Seq<t::Step> {{ {ts} }}']
    for j,call in enumerate(cl['calls']):lines += [f'spec fn {ident}_out_{j}() -> Seq<u8> {{ {a.seq(call["raw"])} }}']
    lines += [f'fn {ident}_part_0() -> (wire: [u8;{len(z)}])',f'    ensures wire@ == {ident}_z(),','{','    ['+', '.join(hex(x)+('u8' if j==0 else '') for j,x in enumerate(z))+']','}',
        '#[verifier::spinoff_prover]',f'proof fn {ident}_shape()',f'    ensures {ident}_z().len() == {len(z)}, {ident}_raw().len() == {n}, {ident}_ts().len() == {len(ss)},',f'        {ident}_ts()[0].pos == 19, {ident}_ts()[0].before == 0,','{}']
    for j,(lo,hi) in enumerate(ranges):
        lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_trace_chunk_{j}()',f'    ensures t::trace({ident}_z(),{ident}_raw(),{ident}_ts(),{lo},{hi}),','{','    hide(t::valid_step);',f'    assert(t::trace({ident}_z(),{ident}_raw(),{ident}_ts(),{lo},{hi})) by (compute);','}']
    lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_trace()',f'    ensures t::trace({ident}_z(),{ident}_raw(),{ident}_ts(),0,{len(ss)}),','{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_ts); hide(t::valid_step); hide(t::trace);']
    lines += [f'    {ident}_trace_chunk_{j}();' for j in range(len(ranges))]
    for depth in (2,1,0):
        for lo,hi in halves(0,len(ss),depth):lines += [f'    t::join_trace({ident}_z(),{ident}_raw(),{ident}_ts(),{lo},{hi});']
    lines += ['}']
    sb=sum(raw);sp=sum((n-i)*v for i,v in enumerate(raw))+n;adler=cl['adler'];end=ss[-1]['next'];cursor=(end+7)//8;stored_adler=int.from_bytes(z[cursor:cursor+4],'big')
    lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_checksum()',f'    ensures m::Adler({ident}_raw()) == {adler},','{','    hide(m::sum_bytes); hide(m::sum_prefixes);',f'    assert(c::sums({rs},0,{n}) == ({sb}nat,{sp}nat)) by (compute);',f'    c::sums_correct({ident}_raw(),0,{n}); assert({ident}_raw().subrange(0,{n}) =~= {ident}_raw());','    assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute);','}',
        '#[verifier::spinoff_prover]',f'proof fn {ident}_header()',f'    ensures {ident}_z()[0] == {z[0]}, {ident}_z()[1] == {z[1]},',f'        m::field_value({ident}_z(),16,1) == 1, m::field_value({ident}_z(),17,2) == 1,',f'        m::be_value({ident}_z(),{cursor},4) == {stored_adler},','{',f'    assert(m::field_value({ident}_z(),16,1) == 1) by (compute); assert(m::field_value({ident}_z(),17,2) == 1) by (compute);',f'    assert(m::be_value({ident}_z(),{cursor},4) == {stored_adler}) by (compute);','}']
    for j,call in enumerate(cl['calls']):
        L,W=call['limit'],call['window'];e,t=str(call['enforce']).lower(),str(call['ignore_tail']).lower();s,out,pos=effect(raw,ss,L,W);k=next(i for i,q in enumerate(ss) if q['symbol']==256 or q['symbol']>256 and q['distance']>W or q['after']>L)
        outfn=f'{ident}_out_{j}()';beff=symbolic_effect(s,outfn,pos);fp=8*(cursor+4) if call['status']=='Done' else 8*(cursor+(5 if call['status']=='TrailingZlibData' else 4));feff=symbolic_effect(call['status'],outfn,fp)
        lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_output_{j}()',f'    ensures {outfn} == {ident}_raw().take({len(out)}), {outfn}.len() == {len(out)},'+(f' {outfn} == {ident}_raw(),' if len(out)==n else ''),'{',f'    assert({outfn} == {ident}_raw().take({len(out)})) by (compute);','}',
            '#[verifier::spinoff_prover]',f'proof fn {ident}_walk_{j}()',f'    ensures g::walk({ident}_ts(),0,{k},{L},{W}), {ident}_ts()[{k}] == ({step_text(ss[k])}),','{',f'    assert(g::walk({ident}_ts(),0,{k},{L},{W})) by (compute);','}',
            '#[verifier::spinoff_prover]',f'proof fn {ident}_result_{j}()',f'    ensures g::result({ident}_raw(),{ident}_ts(),0,{L},{W}) == ({beff}),','{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_ts); hide({ident}_out_{j});',f'    {ident}_shape(); {ident}_output_{j}(); {ident}_walk_{j}();',f'    g::walk_quant({ident}_ts(),0,{k},{L},{W}); g::result_prefix({ident}_raw(),{ident}_ts(),0,{k},{L},{W});',f'    assert({ident}_raw().take({n}) =~= {ident}_raw());','}']
        if s=='Done':
            lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_finish_{j}()',f'    ensures m::MFinish({ident}_z(),{end},{ident}_raw(),{L},{t}) == ({feff}),','{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_out_{j}); hide(m::sum_bytes); hide(m::sum_prefixes);',f'    {ident}_shape(); {ident}_output_{j}(); {ident}_header(); {ident}_checksum();',f'    assert({ident}_raw().take({n}) =~= {ident}_raw());','}']
        lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_model_{j}()',f'    ensures m::Inflate({ident}_z(),{L},{e},{t}).status == {ms(call["status"])},',f'        m::Inflate({ident}_z(),{L},{e},{t}).out == {outfn}, {outfn}.len() == {len(out)}, {ident}_z().len() == {len(z)},','{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_ts); hide({ident}_out_{j}); hide(g::result); hide(g::step); hide(m::MBlk); hide(m::MFinish); hide(m::MEnvelope);',f'    {ident}_shape(); {ident}_trace(); {ident}_header(); {ident}_result_{j}(); {ident}_output_{j}();']
        if s=='Done':lines += [f'    {ident}_finish_{j}();']
        lines += [f'    g::block({ident}_z(),{ident}_raw(),{ident}_ts(),{L},{W});',f'    assert(m::window_size({z[0]//16},{e}) == {W}) by (compute);',f'    g::envelope({ident}_z(),{L},{e},{t},{z[0]},{z[1]},{W},{beff},'+(feff if s=='Done' else beff)+');','}']
    lines += ['#[verifier::spinoff_prover]',f'fn {ident}() {{',f'    hide({ident}_z); '+''.join(f'hide({ident}_out_{j}); ' for j in range(len(cl['calls']))),'    let segment_0 = '+ident+'_part_0();','    let parts: [&[u8];1] = [&segment_0];','    proof {',f'        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= {ident}_z());',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }']
    for j,call in enumerate(cl['calls']):
        L=call['limit'];e,t=str(call['enforce']).lower(),str(call['ignore_tail']).lower()
        lines += [f'    proof {{ {ident}_model_{j}(); }}',f'    let ghost expected = {ident}_out_{j}(); let mut output = [0xa5u8;{L}]; check_policy_parts(&parts,&mut output,{e},{t},{es(call["status"])},Ghost(expected));']
    lines += ['}'];del cl['wire'];del cl['parts']
lines += ['} // verus!','']
text='\n'.join(lines)
def drop_helper(text,name):
    pattern=r'#\[verifier::spinoff_prover\]\nproof fn '+re.escape(name)+r'\(\)\n.*?\n\}\n'
    text,count=re.subn(pattern,'',text,flags=re.S)
    assert count==1,(name,count)
    return text
# Share equal output prefixes and equal block-result subqueries, while retaining
# every public call and its independently checked limit/window/tail descriptor.
for claim in claims:
    ident=claim['class'];outputs={};walks={};finishes={}
    for j,call in enumerate(claim['calls']):
        key=tuple(call['raw'])
        if key in outputs:
            ref=outputs[key]
            pattern=r'^spec fn '+re.escape(f'{ident}_out_{j}')+r'\(\) -> Seq<u8> \{[^\n]*\}\n'
            text,count=re.subn(pattern,'',text,flags=re.M);assert count==1
            text=drop_helper(text,f'{ident}_output_{j}')
            text=text.replace(f'{ident}_out_{j}()',f'{ident}_out_{ref}()')
            text=text.replace(f'{ident}_output_{j}()',f'{ident}_output_{ref}()')
            text=text.replace(f'hide({ident}_out_{j});',f'hide({ident}_out_{ref});')
        else:outputs[key]=j
        key=(call['limit'],call['window'])
        if key in walks:
            ref=walks[key]
            for suffix in ('walk','result'):
                text=drop_helper(text,f'{ident}_{suffix}_{j}')
                text=text.replace(f'{ident}_{suffix}_{j}()',f'{ident}_{suffix}_{ref}()')
        else:walks[key]=j
        if f'proof fn {ident}_finish_{j}()' in text:
            key=(call['limit'],call['ignore_tail'],call['status'])
            if key in finishes:
                ref=finishes[key];text=drop_helper(text,f'{ident}_finish_{j}')
                text=text.replace(f'{ident}_finish_{j}()',f'{ident}_finish_{ref}()')
            else:finishes[key]=j
    if text.count(f'{ident}_checksum()')==1:text=drop_helper(text,f'{ident}_checksum')
clean=[]
for line in text.splitlines():
    seen=set()
    def keep_hide(match):
        name=match[1]
        if name in seen:return ''
        seen.add(name);return match[0]
    clean.append(re.sub(r'hide\(([^)]+)\);',keep_hide,line))
(ROOT/'src/witness_config_cases.rs').write_text('\n'.join(clean)+'\n')
(ROOT/'tools/witness_config.json').write_text(json.dumps(claims,indent=2)+'\n')
print('Authored',len(claims),'chunked fixed configuration classes')
