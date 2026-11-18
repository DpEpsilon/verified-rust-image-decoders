#!/usr/bin/env python3
"""Author D4 traces from frozen generator intent, never from decoded results."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
V={v['name']:v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
def wire(name):
 b=bytes.fromhex(V[name]['input_hex']); p=8; z=b''
 while p+12<=len(b):
  n=int.from_bytes(b[p:p+4],'big')
  if b[p+4:p+8]==b'IDAT':z+=b[p+8:p+8+n]
  p+=12+n
 return z
def seq(xs,ty='u8'):
 return f'Seq::<{ty}>::empty()' if not xs else 'seq!['+', '.join(str(x)+(ty if i==0 else '') for i,x in enumerate(xs))+']'
CL=[4,5,5,5,4,4,4,4,4,4,4,4,4,5,5,5,4,4,4]
# name suffix, sparse profile, distance profile, number of preceding 144 literals,
# authored status; aligned cases use R=17, all others use R=4.
CASES=[('single-literal-code-other-bit-then-eof',0,[0],0,'InvalidSymbol'),('single-literal-code-live-prefix-eof',0,[0],3,'TruncatedZlib'),('length-with-no-distance-codes-then-eof',2,[0],0,'InvalidSymbol'),('single-distance-code-other-bit-then-eof',2,[1],0,'InvalidSymbol'),('single-distance-code-live-prefix-eof',2,[1],6,'TruncatedZlib'),('unassigned-code-in-incomplete-literal-code',0,[0],0,'InvalidSymbol'),('length-with-no-distance-codes',2,[0],0,'InvalidSymbol')]
lines=['//! All five frozen D4 and two companion dead-prefix public-call witnesses.','use crate::{model as m,Status,InflateError as E,witnesses::{wire_word,fixed_decode,check_at},witness_dynamic::*};','use vstd::prelude::*;','verus! {']
for suffix,mode,dist,j,status in CASES:
 name='deflate/'+suffix;z=wire(name);zs=seq(z);ds=seq(dist);cl=seq(CL);L=17 if j else 4
 nlen=257 if mode==0 else 286; active=[256] if mode==0 else [0,1,2,3,65,66,256,257]
 vals=[mode+1 if i in active else 0 for i in range(nlen)]+dist
 block=16+(10+9*j if j else 0); start=block+74; positions=[start]
 for v in vals:positions.append(positions[-1]+CL[v])
 ps=seq(positions,'nat');p=positions[-1];prefix=[144]*j
 lines += [f'/// {name}: authored D4 {status} trace, R={L}.','#[verifier::spinoff_prover]',f'fn {suffix.replace("-","_")}() {{','    hide(m::MCompressedStep); hide(m::MCompressed);',f'    let wire = [{", ".join(hex(x)+( "u8" if i==0 else "") for i,x in enumerate(z))}];','    proof {',f'        let z={zs}; let code={cl}; let lit=profile({mode}); let dist={ds};',f'        let values=lit+dist; let positions={ps};','        assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::NCL == 19) by (compute);',f'        assert(m::Kraft({cl}) == 32768) by (compute); assert(m::admissible(code,m::CodeKind::CodeLen,true));',f'        assert(cl_trace({zs},{block+17},Seq::new(19,|i:int|0u8),0) == {cl}) by (compute);',f'        assert(trace_words({zs},{cl},profile({mode})+{ds},{ps},0,{len(vals)})) by (compute);',f'        trace_decodes(z,code,values,positions,0,{len(vals)});',f'        expand_literals(z,code,values,positions,{nlen},1,0);',f'        assert(values.take(0) =~= Seq::<u8>::empty()); assert(positions[0] == {start}); assert(positions[values.len() as int] == {p});',f'        profile_facts({mode});',f'        assert(m::Kraft({ds}) == {16384 if dist==[1] else 0}) by (compute);',f'        assert(m::count_prefix({ds},0,1) == {dist.count(0)}) by (compute); assert(m::bl_count({ds},1) == {dist.count(1)}) by (compute); assert(m::admissible(dist,m::CodeKind::Dist,true));',f'        assert(values.take({nlen}) =~= lit); assert(values.skip({nlen}) =~= dist);',f'        assert(m::finish_dynamic(values,{nlen},1,{p},true) == (m::DynamicHeader::Tables {{lit,dist,position:{p}}}));',f'        assert(m::MDynamicLengths(z,{start},code,Seq::empty(),{nlen},1,true) == (m::DynamicHeader::Tables {{lit,dist,position:{p}}}));',f'        cl_bridge(z,{block+17},Seq::new(19,|i:int|0u8),0,{nlen},1);',f'        assert(m::MDynamicCL(z,{block+17},Seq::new(19,|i:int|0u8),0,19,{nlen},1,true) == (m::DynamicHeader::Tables {{lit,dist,position:{p}}}));',f'        assert(m::field_value({zs},{block+3},5) == {nlen-257}) by (compute); assert(m::field_value({zs},{block+8},5) == 0) by (compute); assert(m::field_value({zs},{block+13},4) == 15) by (compute);',f'        assert(m::MDynamic(z,{block+3},true) == (m::DynamicHeader::Tables {{lit,dist,position:{p}}}));','        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);']
 init=seq(prefix)
 if mode==2:
  for s,rank in [(0,0),(65,4)]:
   rs=seq(prefix);nxt=seq(prefix+[s]);lines += [f'        assert(wire_word({zs},{p},3) == m::msb_bits({rank},3)) by (compute); profile_decode(z,{p},2,{s});',f'        assert({rs}.push({s}) =~= {nxt});',f'        assert(m::MCompressedStep(z,{p},{rs},{L},32768,lit,dist) == (m::SymbolEffect::Next {{out:{nxt},position:{p+3}}})) by {{reveal(m::MCompressedStep);}}'];prefix.append(s);p+=3
  lines += [f'        assert(wire_word({zs},{p},3) == m::msb_bits(7,3)) by (compute); profile_decode(z,{p},2,257);']
  dpos=p+3
  if dist==[0]:
   lines += [f'        assert(m::MDec(dist,z,{dpos}) == (m::Decoded::InvalidSymbol {{position:{dpos}}}));'];errpos=dpos
  else:
   lines += ['        assert(m::cw(seq![1u8],0) == Some(seq![false])) by (compute); assert forall|t:int| t != 0 implies m::cw(dist,t).is_none() by {}',f'        assert(m::bit({zs},{dpos}) == {"false" if j else "true"}) by (compute);',f'        single_error(dist,z,{dpos},0);'];errpos=dpos if j else dpos+1
 else:
  lines += ['        eob_single_word();',f'        assert(m::bit({zs},{p}) == {"false" if j else "true"}) by (compute);',f'        single_error(lit,z,{p},256);'];errpos=p if j else p+1
 raw=seq(prefix); effect=f'm::StoredEffect::Error {{error:m::InflateFault::{status},out:{raw},position:{errpos}}}'
 lines += [f'        assert(m::MCompressedStep(z,{p},{raw},{L},32768,lit,dist) == (m::SymbolEffect::Error {{error:m::InflateFault::{status},out:{raw},position:{errpos}}})) by {{reveal(m::MCompressedStep);}}',f'        assert(m::MCompressed(z,{positions[-1]},{init},{L},32768,lit,dist) == ({effect})) by {{reveal_with_fuel(m::MCompressed,4);}}',f'        assert(m::field_value({zs},{block},1) == 1) by (compute); assert(m::field_value({zs},{block+1},2) == 2) by (compute);',f'        assert(m::MOneBlock(z,{block},{init},{L},32768,true) == (true,{effect}));',f'        assert(m::MBlk(z,{block},{init},{L},32768,true) == ({effect}));']
 if j:
  for k in range(j):
   q=19+9*k;rs=seq([144]*k);nxt=seq([144]*(k+1));lines += [f'        assert(wire_word({zs},{q},9) == m::msb_bits(400,9)) by (compute); fixed_decode(z,{q},144);',f'        assert({rs}.push(144) =~= {nxt});',f'        assert(m::MCompressedStep(z,{q},{rs},{L},32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {{out:{nxt},position:{q+9}}})) by {{reveal(m::MCompressedStep);}}']
  q=19+9*j;lines += [f'        assert(wire_word({zs},{q},7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,{q},256);',f'        assert(m::MCompressedStep(z,{q},{init},{L},32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::End {{out:{init},position:{block}}})) by {{reveal(m::MCompressedStep);}}',f'        assert(m::MCompressed(z,19,Seq::empty(),{L},32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End {{out:{init},position:{block}}})) by {{reveal_with_fuel(m::MCompressed,{j+2});}}',f'        assert(m::field_value({zs},16,1) == 0) by (compute); assert(m::field_value({zs},17,2) == 1) by (compute);',f'        assert(m::MOneBlock(z,16,Seq::empty(),{L},32768,true) == (false,m::StoredEffect::End {{out:{init},position:{block}}}));',f'        assert(m::MBlk(z,16,Seq::empty(),{L},32768,true) == ({effect}));']
 lines += [f'        assert(m::Inflate(z,{L},true,false).status == m::InflateStatus::Error(m::InflateFault::{status}));',f'        assert(m::Inflate(z,{L},true,false).out == {raw});',f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',f'    let mut output=[0xa5u8;{L}];',f'    let ghost raw={raw};',f'    check_at(&wire,&mut output,Status::Err(E::{status}),Ghost(raw));','}']
lines+=['} // verus!',''];(ROOT/'src/witness_d4.rs').write_text('\n'.join(lines));print('Wrote five D4 and two companion independently authored frozen witnesses')
