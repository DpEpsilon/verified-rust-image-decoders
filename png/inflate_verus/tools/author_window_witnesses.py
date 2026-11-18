#!/usr/bin/env python3
"""Author synthetic exact-window, window+1 and wrapped-ring wire/effect witnesses."""
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def seq(v): return 'seq!['+', '.join(str(x)+('u8' if i==0 else '') for i,x in enumerate(v))+']'
def bits(v,n):return [(v>>i)&1 for i in range(n)]
def word(v,n):return [(v>>i)&1 for i in reversed(range(n))]
lines=['//! Synthetic independently authored window boundaries and measuring ring wrap.','use crate::{model as m,Status,InflateError as E,witnesses::{wire_word,fixed_decode,fixed_dist_decode},witness_window::*};','use vstd::prelude::*;','verus! {']
for name,n,ds,de,width in [('window_exact',256,15,63,6),('window_plus_one',257,16,0,7),('ring_wrap_exact_distance',32768,29,8191,13)]:
 cmf,flg=(0x78,1) if n==32768 else (8,29)
 header=[cmf,flg,0,n%256,n//256,(65535-n)%256,(65535-n)//256]
 tailbits=bits(1,1)+bits(1,2)+word(1,7)+word(ds,5)+bits(de,width)+word(0,7)
 tail=[sum(b<<j for j,b in enumerate(tailbits[i:i+8])) for i in range(0,len(tailbits),8)]
 checksum=[(n+3)//256,(n+3)%256,0,1];endbytes=tail+checksum;total=7+n+len(endbytes);q=8*(n+7);end=q+22+width
 z=f'Seq::new({total},|i:int| if i < 7 {{ {seq(header)}[i] }} else if i < {7+n} {{0u8}} else {{ {seq(endbytes)}[i-{7+n}] }})'
 lines += [f'// Synthetic {name}: stored {n} zeros, then length 3 at distance {n}.','#[verifier::spinoff_prover]',f'fn {name}() {{',f'    let mut wire=[0u8;{total}];']
 for i,x in enumerate(header):
  if x:lines += [f'    wire[{i}]={x};']
 for i,x in enumerate(endbytes):
  if x:lines += [f'    wire[{7+n+i}]={x};']
 lines += ['    proof {',f'        let z={z};','        assert(wire@ =~= z);',f'        assert(z.subrange(7,{7+n}) =~= zeros({n}));','        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute); assert(m::pow2(8) == 256) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute);',f'        assert(m::distance_extra({ds}) == {width}) by (compute); assert(m::distance_base({ds}) == {n-de}) by (compute);',f'        assert(wire_word({z},{q+3},7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,{q+3},257);',f'        assert(wire_word({z},{q+10},5) == m::msb_bits({ds},5)) by (compute); fixed_dist_decode(z,{q+10},{ds});',f'        assert(m::field_value({z},{q+15},{width}) == {de}) by (compute);',f'        assert(wire_word({z},{q+15+width},7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,{q+15+width},256);',f'        assert(m::field_value({z},16,1) == 0) by (compute); assert(m::field_value({z},17,2) == 0) by (compute);',f'        assert(m::field_value({z},{q},1) == 1) by (compute); assert(m::field_value({z},{q+1},2) == 1) by (compute);',f'        zero_sums({n+3});', '        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX + 1) by (compute);',f'        assert(m::be_value({z},{7+n+len(tail)},4) == {(n+3)*65536+1}) by (compute);']
 for enforce in [True,False]:
  window=32768 if not enforce or n==32768 else 256;ok=n<=window;raw=n+3 if ok else n;status='m::InflateStatus::Done' if ok else 'm::InflateStatus::Error(m::InflateFault::DistanceTooFar)'
  lines += [f'        zero_blocks(z,{n},{window},{ds},{de},{width});',f'        assert(m::Inflate(z,{n+3},{str(enforce).lower()},false).status == {status});',f'        assert(m::Inflate(z,{n+3},{str(enforce).lower()},false).out == zeros({raw}));']
 lines += [f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {total}) by (compute);','    }',f'    let mut output=[0xa5u8;{n+3}];']
 for enforce in [True,False]:
  ok=n<=256 or n==32768 or not enforce;raw=n+3 if ok else n;status='Status::Done' if ok else 'Status::Err(E::DistanceTooFar)'
  lines += [f'    let ghost raw=zeros({raw}); check_options(&wire,&mut output,{str(enforce).lower()},{status},Ghost(raw));']
 lines += ['}']
lines += ['} // verus!',''];(ROOT/'src/witness_window_cases.rs').write_text('\n'.join(lines));print('Wrote synthetic exact/window+1 and wrapped-ring public witnesses')
