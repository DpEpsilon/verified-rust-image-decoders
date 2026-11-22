#!/usr/bin/env python3
"""Reverify the two existing M2 authored dynamic certificates in the PNG crate.

Copy proof bodies, not imported private theorems; retain independent public M2
models and reuse the already checked PNG wire-word/copy bridges.
"""
import argparse,re
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);root=p.parse_args().root;src=root.parent/'inflate_verus/src'
 dynamic=(src/'witness_dynamic.rs').read_text();dynamic=dynamic[dynamic.index('pub(crate) open spec fn repeat_step'):dynamic.index('pub(crate) open spec fn cl_trace')]
 bridge=(src/'witness_profile_bridge.rs').read_text();bridge=bridge[bridge.index('pub(crate) open spec fn fields'):bridge.rindex('} // verus!')]
 profiles=(src/'witness_profiles.rs').read_text();profiles=profiles[profiles.index('spec fn rounding_z()'):profiles.rindex('} // verus!')]
 # Remove only executable input-array and executable entry-point functions.
 for name in ['rounding_part_0','subframe_part_0','rounding','subframe']:
  start=profiles.index('fn '+name+'(');a=profiles.index('{',start);depth=1;i=a+1
  while depth:
   depth+=(profiles[i]=='{')-(profiles[i]=='}');i+=1
  before=profiles[:start]
  if before.endswith('#[verifier::spinoff_prover]\n'):before=before[:-len('#[verifier::spinoff_prover]\n')]
  profiles=before+profiles[i:]
 out=['//! M2 authored dynamic certificates reverified against public independent models.','use inflate_verus::model as m;','use vstd::prelude::*;','use crate::m4_fixed_witness_model::{follow_word,wire_word};','use crate::m4_trace_witness_model as t;','mod d {use super::*;verus! {',dynamic,'}}','mod b {use super::*;verus! {',bridge,'}}','verus! {',profiles]
 for name,n,position in [('rounding',25,200),('subframe',42,256)]:
  z=re.search(r'spec fn '+name+r'_z\(\).*?\{ (seq!\[.*?\]) \}',profiles).group(1);raw=re.search(r'spec fn '+name+r'_raw\(\).*?\{ (seq!\[.*?\]) \}',profiles).group(1)
  out += ['#[verifier::spinoff_prover]',f'pub(crate) proof fn checked_{name}()',f'    ensures m::Inflate({z},{n}nat,true,false)==(m::Inflated {{status:m::InflateStatus::Done,out:{raw},r2:false}}),','{','    hide(m::MDec);hide(m::MDynamic);hide(m::MCompressed);hide(m::sum_bytes);hide(m::sum_prefixes);',f'    {name}_header();{name}_payload();',f'    let z={name}_z();let raw={name}_raw();','    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);assert(m::pow2(15)==32768) by (compute);',f'    assert(m::field_value({z},16,1)==1 && m::field_value({z},17,2)==2) by (compute);',f'    assert(m::MBlk(z,16,Seq::empty(),{n},32768,true)==(m::StoredEffect::End {{out:raw,position:{position}}}));',f'    assert(m::MFinish({z},{position},{raw},{n},false)==(m::StoredEffect::End {{out:{raw},position:{8*(position//8+4)}}})) by (compute);','}']
 out+=['} // verus!',''];text='\n'.join(out).replace('crate::limits::','inflate_verus::limits::');(root/'src/m5_dynamic_witness_model.rs').write_text(text);print('Generated two existing authored dynamic certificates; no verification credit.')
if __name__=='__main__':main()
