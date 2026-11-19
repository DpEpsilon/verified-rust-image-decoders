#!/usr/bin/env python3
"""Author the five-block flush schedule and a fixed live-prefix cut at EOF."""
import json
import re
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import author_extended_classes as a
V={v['name']:v for v in a.V}
def steps(tokens):
    out=[];pos=19;before=0
    for token in tokens+[256]:
        if isinstance(token,int):s=token;le=ds=de=n=d=0;nxt=pos+(8 if s<144 else 9 if s<256 else 7);after=before+(s<256)
        else:n,d=token;s,le,ln,ds,de,dn=a.codes(n,d);nxt=pos+(7 if s<280 else 8)+ln+5+dn;after=before+n
        out.append(dict(pos=pos,next=nxt,before=before,after=after,symbol=s,le=le,ds=ds,de=de,length=n,distance=d));pos=nxt;before=after
    return out
lines=['//! Authored five-block flush and fixed live-prefix EOF witnesses.',
'use crate::{model as m, witness_match as t, witness_checksum as c, witness_config as g, witness_block_bridge as b, witness_policy::check_policy_parts, witnesses::{wire_word,fixed_decode}, Status, InflateError as E};','use vstd::prelude::*;','verus! {']
claims=[]
for ident,name in [('flush','zlib/sync-and-full-flush-empty-stored-blocks'),('cut','zlib/cut-mid-block')]:
    data=bytes.fromhex(V[name]['input_hex']);ps=[p for k,p in a.chunks(data) if k==b'IDAT'];z=b''.join(ps);zs=a.seq(z)
    if ident=='flush':
        source=a.base_raw[:300];raw=a.base_raw;tokens=a.greedy(source,1,False,False,1);ss=steps(tokens);k=len(ss)
        first=a.encode(tokens);first=bytes([first[0]&254])+first[1:]
        body=first+b'\x00\x00\xff\xff'+b'\x00'+(723).to_bytes(2,'little')+(65535-723).to_bytes(2,'little')+bytes(raw[300:])+b'\x00\x00\x00\xff\xff'+b'\x03\x00'
        assert body==z[2:-4]
    else:
        source=a.base_raw;tokens=a.greedy(source,1,False,False,1);ss=steps(tokens)
        full=a.streams(bytes.fromhex(V['zlib/level6-default']['input_hex']))[0]
        assert full[2:-4]==a.encode(tokens) and z==full[:len(z)]
        k=next(i for i,s in enumerate(ss) if s['next']>8*len(z));assert ss[k]['symbol']==118 and ss[k]['before']==552 and ss[k]['pos']==4011
        raw=source[:552]
    rs=a.seq(raw);src=a.seq(source);ts='seq!['+', '.join('t::Step {'+', '.join(f'{kk}:{vv}' for kk,vv in s.items())+'}' for s in ss)+']'
    lines += [f'spec fn {ident}_z() -> Seq<u8> {{ {zs} }}',f'spec fn {ident}_raw() -> Seq<u8> {{ {rs} }}',f'spec fn {ident}_source() -> Seq<u8> {{ {src} }}',f'spec fn {ident}_ts() -> Seq<t::Step> {{ {ts} }}']
    for j,part in enumerate(ps):lines += [f'fn {ident}_part_{j}() -> (wire: [u8;{len(part)}])',f'    ensures wire@ == {ident}_z(),','{','    ['+', '.join(hex(x)+('u8' if i==0 else '') for i,x in enumerate(part))+']','}']
    lines += ['#[verifier::spinoff_prover]',f'proof fn {ident}_trace()',f'    ensures {ident}_z().len() == {len(z)}, {ident}_source().len() == {len(source)}, {ident}_raw().len() == {len(raw)}, {ident}_ts().len() == {len(ss)},',f'        {ident}_z()[0] == {z[0]}, {ident}_z()[1] == {z[1]}, {ident}_ts()[0].pos == 19, {ident}_ts()[0].before == 0,',f'        t::trace({ident}_z(),{ident}_source(),{ident}_ts(),0,{k}),',f'        {ident}_ts()[{k-1}].next == {ss[k-1]["next"]}, {ident}_ts()[{k-1}].after == {ss[k-1]["after"]},']
    if ident=='cut':lines += [f'        cut_ts()[{k}].pos == 4011, cut_ts()[{k}].before == 552, cut_source().take(552) == cut_raw(),']
    else:lines += ['        flush_source() == flush_raw().take(300), flush_z().subrange(269,992) == flush_raw().skip(300),']
    lines += ['{',f'    assert(t::trace({zs},{src},{ts},0,{k})) by (compute);']
    if ident=='flush':lines += [f'    assert({zs}.subrange(269,992) == {rs}.skip(300)) by (compute);']
    lines += ['}']
    if ident=='flush':
        sb=sum(raw);sp=sum((len(raw)-i)*v for i,v in enumerate(raw))+len(raw);adler=(sp%65521)*65536+(1+sb)%65521
        lines += ['#[verifier::spinoff_prover]','proof fn flush_checksum()',f'    ensures m::Adler(flush_raw()) == {adler},','{','    hide(m::sum_bytes); hide(m::sum_prefixes);',f'    assert(c::sums({rs},0,1023) == ({sb}nat,{sp}nat)) by (compute);','    c::sums_correct(flush_raw(),0,1023); assert(flush_raw().subrange(0,1023) =~= flush_raw());','    assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute);','}']
        fields=[(16,1,0),(17,2,1),(2077,1,0),(2078,2,0),(2112,1,0),(2113,2,0),(7936,1,0),(7937,2,0),(7976,1,1),(7977,2,1)]
        lines += ['#[verifier::spinoff_prover]','proof fn flush_fields()','    ensures']+[f'        m::field_value(flush_z(),{p},{n}) == {v},' for p,n,v in fields]
        for pos,count in [(260,0),(265,723),(993,0)]:
            lines += [f'        flush_z()[{pos}] as nat+256*flush_z()[{pos+1}] as nat == {count},',f'        flush_z()[{pos+2}] as nat+256*flush_z()[{pos+3}] as nat == {65535-count},']
        lines += [f'        m::be_value(flush_z(),999,4) == {adler},','        wire_word(flush_z(),7979,7) == m::msb_bits(0,7),','{']+[f'    assert(m::field_value({zs},{p},{n}) == {v}) by (compute);' for p,n,v in fields]+[f'    assert(m::be_value({zs},999,4) == {adler}) by (compute);',f'    assert(wire_word({zs},7979,7) == m::msb_bits(0,7)) by (compute);','}']
    else:
        lines += ['#[verifier::spinoff_prover]','proof fn cut_terminal()','    ensures m::MCompressed(cut_z(),4011,cut_raw(),1023,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::Error {error:m::InflateFault::TruncatedZlib,out:cut_raw(),position:4016}),','{','    hide(m::MDec); hide(m::MCompressedStep); hide(m::decode_prefix);','    m::lemma_fixed_codes(true); crate::witnesses::fixed_word(118);',f'    assert(wire_word({zs},4011,5) == m::msb_bits(166,8).take(5)) by (compute);','    b::live_eof(m::fixed_lit_lengths(),cut_z(),4011,118,0,5);','    assert(wire_word(cut_z(),4011,0) =~= Seq::<bool>::empty());','    assert(m::MDec(m::fixed_lit_lengths(),cut_z(),4011) == (m::Decoded::TruncatedZlib {position:4016})) by {reveal(m::MDec);}','    assert(crate::limits::WINDOW_MAX == 32768) by (compute);','    reveal(m::MCompressedStep); reveal(m::MCompressed);','}']
    lines += ['#[verifier::spinoff_prover]',f'fn {ident}() {{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_source); hide({ident}_ts); hide(m::MCompressedStep); hide(m::sum_bytes); hide(m::sum_prefixes);']
    for j in range(len(ps)):lines += [f'    let segment_{j} = {ident}_part_{j}();']
    lines += [f'    let parts: [&[u8];{len(ps)}] = ['+', '.join('&segment_'+str(j) for j in range(len(ps)))+'];','    proof {',f'        {ident}_trace();',f'        let z = {ident}_z(); let raw = {ident}_raw();', '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);']
    if ident=='flush':
        lines += ['        flush_fields(); flush_checksum();',f'        t::effects_at(z,flush_source(),flush_ts(),0,{k},1023); t::expand_at(z,flush_source(),flush_ts(),0,1023);','        assert(flush_source().take(0) =~= Seq::<u8>::empty());',
            '        assert(z.subrange(264,264) =~= Seq::<u8>::empty()); b::stored(z,2077,flush_source(),Seq::empty(),1023); assert(flush_source()+Seq::<u8>::empty() =~= flush_source());',
            '        b::stored(z,2112,flush_source(),raw.skip(300),1023); assert(flush_source()+raw.skip(300) =~= raw);',
            '        assert(z.subrange(997,997) =~= Seq::<u8>::empty()); b::stored(z,7936,raw,Seq::empty(),1023); assert(raw+Seq::<u8>::empty() =~= raw);',
            '        fixed_decode(z,7979,256);',
            '        assert(m::MCompressedStep(z,7979,raw,1023,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::End {out:raw,position:7986})) by {reveal(m::MCompressedStep);}',
            '        assert(m::MCompressed(z,7979,raw,1023,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End {out:raw,position:7986})) by {reveal(m::MCompressed);}',
            '        assert(m::MBlk(z,16,Seq::empty(),1023,32768,true) == (m::StoredEffect::End {out:raw,position:7986})) by {reveal_with_fuel(m::MBlk,6);}',
            '        assert(m::MFinish(z,7986,raw,1023,false) == (m::StoredEffect::End {out:raw,position:8024}));']
        status='Done'
    else:
        lines += ['        cut_terminal();',f'        g::effects(z,cut_source(),cut_ts(),0,{k},1023,32768);',f'        b::prefix(z,cut_source(),cut_ts(),0,{k},1023,m::StoredEffect::Error {{error:m::InflateFault::TruncatedZlib,out:raw,position:4016}});','        assert(cut_source().take(0) =~= Seq::<u8>::empty());',f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 1) by (compute);','        assert(m::MBlk(z,16,Seq::empty(),1023,32768,true) == (m::StoredEffect::Error {error:m::InflateFault::TruncatedZlib,out:raw,position:4016}));']
        status='TruncatedZlib'
    ms='m::InflateStatus::Done' if status=='Done' else 'm::InflateStatus::Error(m::InflateFault::TruncatedZlib)'
    es='Status::Done' if status=='Done' else 'Status::Err(E::TruncatedZlib)'
    lines += [f'        assert(m::Inflate(z,1023,true,false).status == {ms}); assert(m::Inflate(z,1023,true,false).out == raw);',f'        reveal_with_fuel(crate::segments::parts_prefix,{len(ps)+1}); assert(crate::segments::parts_prefix(parts@,{len(ps)}) =~= z);',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',f'    let ghost raw = {ident}_raw(); let mut output = [0xa5u8;1023]; check_policy_parts(&parts,&mut output,true,false,{es},Ghost(raw));','}']
    claims.append({'class':ident,'domain':'core','members':[[name,0]],'r':1023,'raw':raw,'model_raw':source,'steps':ss,'trace_prefix':k,'tokens':tokens,'segment_lengths':[len(p) for p in ps],'calls':[{'limit':1023,'enforce':True,'ignore_tail':False,'status':status,'raw':raw}]})
lines+=['} // verus!','']
text='\n'.join(lines)
def split(a,b,depth):
    if not depth:return [(a,b)]
    mid=a+(b-a)//2
    return split(a,mid,depth-1)+split(mid,b,depth-1)
for claim in claims:
    ident=claim['class'];count=claim['trace_prefix'];ranges=split(0,count,3);claim['trace_chunks']=ranges
    pattern=r'proof fn '+ident+r'_trace\(\)\n.*?\n\{(.*?)\n\}'
    match=re.search(pattern,text,re.S);assert match
    shape=match[0].replace('proof fn '+ident+'_trace()','proof fn '+ident+'_shape()')
    shape='\n'.join(line for line in shape.splitlines() if 't::trace(' not in line)
    shape=shape[:-1]+f'    assert({ident}_source().take({300 if ident=="flush" else 552}) == '+('flush_raw().take(300)' if ident=='flush' else 'cut_raw()')+') by (compute);\n}'
    extra=[shape]
    for j,(lo,hi) in enumerate(ranges):extra += ['#[verifier::spinoff_prover]',f'proof fn {ident}_trace_chunk_{j}()',f'    ensures t::trace({ident}_z(),{ident}_source(),{ident}_ts(),{lo},{hi}),','{',f'    assert(t::trace({ident}_z(),{ident}_source(),{ident}_ts(),{lo},{hi})) by (compute);','}']
    extra+=['#[verifier::spinoff_prover]',f'proof fn {ident}_trace()',f'    ensures t::trace({ident}_z(),{ident}_source(),{ident}_ts(),0,{count}),','{',f'    hide({ident}_z); hide({ident}_source); hide({ident}_ts); hide(t::trace); hide(t::valid_step);']+[f'    {ident}_trace_chunk_{j}();' for j in range(len(ranges))]
    for depth in (2,1,0):
        for lo,hi in split(0,count,depth):extra += [f'    t::join_trace({ident}_z(),{ident}_source(),{ident}_ts(),{lo},{hi});']
    extra+=['}']
    if ident=='flush':
        extra+=['#[verifier::spinoff_prover]','proof fn flush_first()','    ensures m::MOneBlock(flush_z(),16,Seq::empty(),1023,32768,true) == (false,m::StoredEffect::End {out:flush_source(),position:2077}),','{','    hide(flush_z); hide(flush_raw); hide(flush_source); hide(flush_ts);','    flush_shape(); flush_trace(); flush_fields();','    b::fixed_first(flush_z(),flush_source(),flush_ts(),1023);','}']
    else:
        extra+=['#[verifier::spinoff_prover]','proof fn cut_header()','    ensures m::field_value(cut_z(),16,1) == 1, m::field_value(cut_z(),17,2) == 1,','{','    assert(m::field_value(cut_z(),16,1) == 1) by (compute); assert(m::field_value(cut_z(),17,2) == 1) by (compute);','}']
    text=text[:match.start()]+'\n'.join(extra)+text[match.end():]
    pattern=r'fn '+ident+r'\(\) \{(.*?)\n\}'
    match=re.search(pattern,text,re.S);assert match
    body=match[1];proof=re.search(r'    proof \{\n(.*?)\n    \}',body,re.S)[1]
    proof_lines=[line[4:] for line in proof.splitlines() if 'parts_prefix' not in line]
    proof_lines[0]=f'    {ident}_shape();'+(' flush_first();' if ident=='flush' else ' cut_trace(); cut_header();')
    proof_lines=[line for line in proof_lines if 't::effects_at(' not in line and 'flush_source().take(0)' not in line and 'g::effects(' not in line and 'b::prefix(' not in line and 'cut_source().take(0)' not in line and not(ident=='cut' and 'assert(m::field_value(' in line)]
    if ident=='cut':
        i=next(i for i,line in enumerate(proof_lines) if 'cut_terminal();' in line)
        proof_lines.insert(i+1,'    b::truncated_block(z,cut_source(),cut_ts(),470,1023,m::StoredEffect::Error {error:m::InflateFault::TruncatedZlib,out:raw,position:4016});')
    status='m::InflateStatus::Done' if ident=='flush' else 'm::InflateStatus::Error(m::InflateFault::TruncatedZlib)'
    model=['#[verifier::spinoff_prover]',f'proof fn {ident}_model()',f'    ensures m::Inflate({ident}_z(),1023,true,false).status == {status},',f'        m::Inflate({ident}_z(),1023,true,false).out == {ident}_raw(), {ident}_raw().len() == {len(claim["raw"])}, {ident}_z().len() == {sum(claim["segment_lengths"])},','{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_source); '+(f'hide({ident}_ts); ' if ident=='cut' else '')+'hide(m::MCompressedStep); hide(m::MCompressed); hide(m::MDec); hide(m::sum_bytes); hide(m::sum_prefixes);']+proof_lines+['}']
    runtime_prefix=body[:body.index('    proof {')]
    runtime_prefix=re.sub(r'    hide[^\n]*\n',f'    hide({ident}_z); hide({ident}_raw);\n',runtime_prefix,count=1)
    runtime_tail=body[body.index('    let ghost raw ='):]
    caller=['#[verifier::spinoff_prover]',f'fn {ident}() {{',runtime_prefix.rstrip(),'    proof {',f'        {ident}_model();',f'        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= {ident}_z());',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {sum(claim["segment_lengths"])}) by (compute);','    }',runtime_tail.rstrip(),'}']
    text=text[:match.start()]+'\n'.join(model+caller)+text[match.end():]
text=text.replace('#[verifier::spinoff_prover]\n#[verifier::spinoff_prover]','#[verifier::spinoff_prover]')
(ROOT/'src/witness_blocks.rs').write_text(text)
(ROOT/'tools/witness_blocks.json').write_text(json.dumps(claims,indent=2)+'\n')
print('Authored complete five-block flush and fixed live-prefix EOF gates')
