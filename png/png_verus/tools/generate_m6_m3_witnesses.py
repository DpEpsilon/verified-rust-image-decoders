#!/usr/bin/env python3
"""Complete E6 clause-2 witnesses; frozen VECTORS alone supplies expectations.

Reuse authored inflate certificates and checked walk/filter helpers. Certificate
hints are not outcomes: Verus checks every hint against the independent models.
No decoder or fixture generator supplies an expected value.
"""
import argparse, hashlib, importlib.util, json, re, sys
from pathlib import Path
sys.dont_write_bytecode=True
NAMES=['container/minimal-ok','container/unknown-ancillary-everywhere',
 'container/reserved-bit-ancillary','plte/ok','plte/suggested-palette-in-rgb',
 'anc-ok/iCCP-303','anc-ok/tIME-7-after-idat','anc-ok/srgb-and-iccp-both',
 'anc-ok/many-text','anc-ok/ztxt-with-corrupt-zlib-not-inflated',
 'order/splt-twice','order/bkgd-hist-after-plte-before-idat-rgb']
def load(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def complete_wire_facts(walk,data,ident,oracle):
 """Reuse M3's literal ancillary-parser certificates, not an executable."""
 facts,sites=walk.read_facts(data,ident,oracle);entries=0
 for pos,tag,payload in walk_chunks(data):
  size=len(payload);d=f'bytes_{ident}().subrange({pos+8},{pos+8+size})'
  if tag==b'PLTE':entries=size//3
  if tag in [b'iCCP',b'zTXt',b'sPLT']:
   nul=payload.find(0)
   if 0<=nul<size-1:facts +=[f'        assert({d}[{nul+1}]=={payload[nul+1]}u8) by (compute);']
  value=helper=None;compute=False
  if tag==b'iCCP':
   nul=payload.index(0);value=f'Iccp {{name_len:{nul},profile_start:{nul+2},profile_len:{size-nul-2}}}';helper=f'iccp_success({d},{nul}nat);'
  elif tag in [b'tEXt',b'zTXt',b'sPLT',b'hIST']:
   value='Ignored'
   if tag==b'tEXt':helper=f'text_success({d},{payload.index(0)}nat);'
   if tag==b'zTXt':helper=f'ztxt_success({d},{payload.index(0)}nat);'
   if tag==b'hIST':compute=True
  elif tag==b'tIME':
   value='Time('+','.join([f'{int.from_bytes(payload[:2],"big")}u16']+[f'{v}u8' for v in payload[2:]])+')';compute=True
  elif tag==b'sRGB':value=f'Srgb({payload[0]}u8)';compute=True
  elif tag==b'bKGD' and data[25]==2:
   value=f'Bkgd(crate::metadata_model::words16({d},3))';helper=f'rgb_background_success({d},{entries}nat);'
   facts +=[f'        assert forall|i: int| 0<=i<3 implies (#[trigger] crate::metadata_model::words16({d},3)[i] as nat)<(1u32<<8u8) as nat by {{']
   for i in range(3):facts +=[f'            {"if" if i==0 else "else if"} i=={i} {{assert(crate::metadata_model::words16({d},3)[i]=={int.from_bytes(payload[2*i:2*i+2],"big")}u16);}}']
   facts +=['        }']
  if value is not None:
   expression=f'crate::metadata_model::parse(crate::walk_model::Name::{walk.NAMES[tag]},{d},2u8,8u8,{entries}nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::{value})'
   facts +=[f'        assert({expression}) by {{','            reveal(crate::metadata_model::parse);']
   if helper:facts +=[f'            {helper}']
   elif compute:facts +=[f'            assert({expression}) by (compute);']
   facts +=['        }']
 return facts,sites

def walk_chunks(data):
 pos=33
 while pos<len(data):
  size=int.from_bytes(data[pos:pos+4],'big');yield pos,data[pos+4:pos+8],data[pos+8:pos+8+size];pos+=size+12

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);p.add_argument('--names',nargs='+');args=p.parse_args();root=args.root
 fixture=root.parent/'spec-frozen/VECTORS.json';records={r['name']:r for r in json.loads(fixture.read_text())};fixture_sha=hashlib.sha256(fixture.read_bytes()).hexdigest()
 base=load('e6_existing_still_certificates',root/'tools/generate_m5_still_witnesses.py');walk=load('e6_existing_walk_certificates',root/'tools/generate_m3_walk_witnesses.py');crc=load('e6_existing_crc_certificates',root/'tools/generate_m4_complete_witnesses.py');fw,_=base.forward_library(root)
 m3=load('e6_existing_metadata_certificates',root/'tools/generate_m3_projection_witnesses.py')
 import types
 walk_proxy=types.SimpleNamespace(read_facts=lambda b,i,o:complete_wire_facts(walk,b,i,o),apng_facts=walk.apng_facts)
 names=NAMES if args.names is None else args.names;assert set(names)<=set(NAMES)
 out=['//! Complete E6 clause-2 Strict witnesses; no projection credit is claimed.','use vstd::prelude::*;',
  'use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,',
  '    still_model as sm,still_equivalence as se,colour_model as colour,alloc_guard::TryError,',
  '    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};',
  'use inflate_verus::model as zm;','verus! {',m3.METADATA_HELPERS,r'''
// Only compose the accepted independent definitions; no executable is called.
#[verifier::spinoff_prover]
proof fn compose_complete(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat,col: colour::Context,pixels: Seq<u8>)
    requires r::default_case(b,h,z,entries),
        r::data(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT)==Some(Ok(())),
        match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>colour::from_doc(doc,b)==col,_=>false},
        sm::image(r::inflated(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT).out,
            r::shape(h,h.width as nat,h.height as nat,entries,STRICT),col,false)==pixels,
    ensures match sm::MS(b) {sm::Outcome::Done(value)=>value.width==h.width && value.height==h.height && value.pixels==pixels,_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);
    match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(r::document_shape(doc,STRICT)==r::shape(h,h.width as nat,h.height as nat,entries,STRICT));
        assert(r::document_data(b,doc,STRICT)==Some(Ok(())));
        assert(crate::paint_model::default_raster(doc,STRICT)==r::shape(h,h.width as nat,h.height as nat,entries,STRICT));
    },_=>{}}
}
'''];index=[]
 for name in names:
  record=records[name];assert record['entry']=='still';expected=record['expected'];assert set(expected)=={'width','height','rgba_runs'}
  ident=re.sub('[^a-zA-Z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);w,h=expected['width'],expected['height'];ct,depth,il=data[25],data[24],data[28];assert depth==8 and il==0
  entries=next((len(d)//3 for _,tag,d in base.chunks(data) if tag==b'PLTE'),0);z=base.streams(data)[0];source=base.source_for(root,records,name,z,fw);raw=source['raw'];rgba=base.expanded_expected(expected);assert len(rgba)==4*w*h
  bf,hf,af,zf,rf,cf,ef=[f'{x}_{ident}()' for x in ['bytes','h','a','z','raw','colour','expected']]
  header=f'crate::model::Ihdr {{width:{w}u32,height:{h}u32,depth:{depth}u8,color_type:{ct}u8,interlace:{il}u8}}'
  symbolic,concrete,hidden=base.walk_certificate(root,walk_proxy,crc,data,ident,hf,zf,entries)
  store_facts=['    hide(m::store);hide(crate::metadata_model::parse);',
   '    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|',
   '        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte',
   '        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl',
   '        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {',
   '        stored_structure(doc,value,base);','    }']
  # Keep proof-local hide at each body start, including split palette steps.
  patched=[]
  for line in symbolic:
   patched.append(line)
   if line.startswith('    hide(crate::critical_model::read);') or line.startswith('    hide(m::drive);hide(crate::critical_model::read);'):patched+=store_facts
  symbolic=patched
  if name=='order/bkgd-hist-after-plte-before-idat-rgb':
   start=next(i for i,line in enumerate(symbolic) if line.startswith(f'proof fn walk_{ident}('));fn=symbolic[start:];post=next(i for i,line in enumerate(fn) if line.strip().startswith('ensures '));body=fn.index('{')
   shape=fn.copy();shape[0]=shape[0].replace(f'walk_{ident}',f'walk_shape_{ident}');shape[post]=f'    ensures r::default_case(b,{hf},{zf},{entries}nat),'
   requirements=shape[1][len('    requires '):-1].split(',\n        ')
   needed=[q for q in requirements if q.startswith('b.len()') or 'crate::model::mh(b)' in q or 'crate::critical_model::read(' in q or 'crate::metadata_model::parse(' in q or q.endswith('=='+zf)]
   shape[1]='    requires '+',\n        '.join(needed)+','
   ranges=fn.copy();ranges[0]=ranges[0].replace(f'walk_{ident}',f'walk_ranges_{ident}');ranges[post]=f'    ensures {base.range_goal("b",data)},'
   common=shape[1:post]
   prefix='e6_bkgd_hist'
   ps=list(base.chunks(data));assert [tag for _,tag,_ in ps]==[b'PLTE',b'hIST',b'bKGD',b'IDAT',b'IEND']
   p0,t0,d0=ps[0];p1,t1,d1=ps[1];p2,t2,d2=ps[2];p3,t3,d3=ps[3]
   states=[f'm::initial({hf},33,m::no_events())',
    f'let s={prefix}_state_0(b);m::State {{pos:{p0+12+len(d0)}nat,seen:s.seen.insert(crate::walk_model::Name::Plte),doc:m::Doc {{plte:Some(crate::walk_model::Range {{offset:{p0+8}nat,len:{len(d0)}nat}}),info:m::Info {{palette_entries:{entries}nat,..s.doc.info}},..s.doc}},..s}}',
    f'let s={prefix}_state_1(b);m::State {{pos:{p1+12+len(d1)}nat,seen:s.seen.insert(crate::walk_model::Name::Hist),doc:m::store(s.doc,crate::metadata_model::Value::Ignored,{p1+8}nat),..s}}',
    f'let s={prefix}_state_2(b);m::State {{pos:{p2+12+len(d2)}nat,seen:s.seen.insert(crate::walk_model::Name::Bkgd),doc:m::store(s.doc,crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange({p2+8},{p2+8+len(d2)}),3)),{p2+8}nat),..s}}',
    f'let s={prefix}_state_3(b);m::State {{pos:{p3+12+len(d3)}nat,phase:crate::walk_model::Phase::During,doc:m::Doc {{idat:seq![crate::walk_model::Range {{offset:{p3+8}nat,len:{len(d3)}nat}}],..s.doc}},..s}}']
   step_code=[f'spec fn {prefix}_state_{j}(b: Seq<u8>) -> m::State {{{value}}}' for j,value in enumerate(states)]
   step_body=[line.replace('hide(m::drive);','') for line in shape[body+1:-1] if 'reveal_with_fuel(m::drive' not in line and 'reveal_with_fuel(m::concat_ranges' not in line]
   for j in range(5):
    result=f'm::Step::Next({prefix}_state_{j+1}(b))' if j<4 else f'm::Step::Done(m::Doc {{info:m::Info {{frame_count:0nat,..{prefix}_state_4(b).doc.info}},..{prefix}_state_4(b).doc}},m::no_events())'
    step_code+=['#[verifier::spinoff_prover]',f'proof fn {prefix}_step_{j}(b: Seq<u8>)',*common,f'    ensures m::step(b,m::strict(),false,false,{prefix}_state_{j}(b))=={result},','{',*step_body,'}']
   shape=shape[:body+1]+['    hide(m::drive);hide(crate::critical_model::read);hide(crate::metadata_model::parse);hide(crate::model::mh);']+[f'    {prefix}_step_{j}(b);' for j in range(5)]+[f'    m::drive_step(b,m::strict(),false,false,{prefix}_state_{j}(b));' for j in reversed(range(5))]+['    reveal_with_fuel(m::concat_ranges,3);','}']
   wrapper=fn[:body+1]+[f'    walk_shape_{ident}(b);walk_ranges_{ident}(b);','}']
   symbolic=symbolic[:start]+step_code+['#[verifier::spinoff_prover]']+shape+['#[verifier::spinoff_prover]']+ranges+['#[verifier::spinoff_prover]']+wrapper
  rows,points=base.row_hints(ident,raw,w,h,ct,depth,il)
  out +=[f'// {name}; frozen fixture SHA256 {fixture_sha}',f'pub open spec fn bytes_{ident}() -> Seq<u8> {{{base.seq(data)}}}',
   f'spec fn h_{ident}() -> crate::model::Ihdr {{{header}}}',f'spec fn a_{ident}() -> r::Raster {{r::shape({hf},{w}nat,{h}nat,{entries}nat,STRICT)}}',
   base.colour_definition(ident,data,ct,depth),f'pub open spec fn expected_{ident}() -> Seq<u8> {{{base.seq(rgba)}}}',
   *base.stream_certificate(ident,z,source,len(raw)),*symbolic,*base.scan_certificate(ident,raw,w,h,ct,depth,il,entries,None),*rows]
  out +=['#[verifier::spinoff_prover]',f'proof fn raster_{ident}()',f'    ensures r::default_case({bf},{hf},{zf},{entries}nat),{base.range_goal(bf,data)},r::data({zf},{af},STRICT)==Some(Ok(())),','{',
   f'    hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(r::data);',hidden,f'    assert({bf}.len()=={len(data)}nat) by (compute);',*concrete,f'    inflate_{ident}();scan_{ident}();',
   f'    assert(r::raw({af})=={len(raw)}nat) by (compute);',f'    assert(r::data({zf},{af},STRICT)==Some(Ok(()))) by {{reveal(r::data);}};','}']
  image=f'sm::image({rf},{af},{cf},false)'
  for i in range(w*h):
   x,y=i%w,i//w;k,col,row,hint,rname=points[(x,y)];px='seq!['+','.join(f'{int(v)*257}u16' for v in rgba[4*i:4*i+4])+']'
   goals=','.join(f'{image}[{4*i+j}]=={int(rgba[4*i+j])}u8' for j in range(4))
   out +=['#[verifier::spinoff_prover]',f'proof fn pixel_{ident}_{i}() ensures {goals},','{','    colour::depth_values(8u8);',f'    {rname}();',f'    assert(crate::paint_model::source_cell({af},{x}nat,{y}nat)==({k}nat,{col}nat,{row}nat)) by (compute);',
    f'    assert(colour::pixel({cf},{base.seq(hint)},{col}nat,true,false)==Ok({px})) by (compute);',
    f'    assert(crate::paint_model::source_pixel({rf},{af},{cf},{x}nat,{y}nat,true,false)==Ok({px}));']
   for j in range(4):out +=[f'    assert((({px}[{j}] as nat+128)/257) as u8=={int(rgba[4*i+j])}u8) by (compute);']
   out +=['}']
  out +=['#[verifier::spinoff_prover]',f'proof fn pixels_{ident}() ensures {image}=={ef},','{']
  out +=[f'    pixel_{ident}_{i}();' for i in range(w*h)]
  out +=[f'    assert(a_{ident}().width=={w}nat && a_{ident}().height=={h}nat) by (compute);',f'    assert({image}.len()=={len(rgba)}nat);',f'    assert({ef}.len()=={len(rgba)}nat) by (compute);',f'    assert forall|j: int| 0<=j<{len(rgba)} implies (#[trigger] {image}[j])=={ef}[j] by {{']
  out +=['        '+('if' if j==0 else 'else if')+f' j=={j} {{assert({ef}[{j}]=={int(rgba[j])}u8) by (compute);}}' for j in range(len(rgba)-1)]
  out +=[f'        else {{assert(j=={len(rgba)-1});assert({ef}[j]=={int(rgba[-1])}u8) by (compute);}}','    }',f'    assert({image}=~={ef});','}']
  pred=f'value.width=={w}u32 && value.height=={h}u32 && value.pixels=={ef}'
  out +=[f'pub open spec fn expected_value_{ident}(value: sm::Value) -> bool {{{pred}}}',
   '#[verifier::spinoff_prover]',f'pub proof fn model_complete_{ident}()',f'    ensures match sm::MS({bf}) {{sm::Outcome::Done(value)=>expected_value_{ident}(value),_=>false}},','{',
   f'    hide(m::MW_p);hide(zm::Inflate);hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(expected_{ident});',f'    assert({bf}.len()=={len(data)}nat) by (compute);',f'    raster_{ident}();inflate_{ident}();pixels_{ident}();']
  for pos,tag,d in base.chunks(data):
   if tag in [b'PLTE',b'tRNS']:out +=[f'    assert({bf}.subrange({pos+8},{pos+8+len(d)})=={base.seq(d)}) by (compute);']
  out +=[f'    match m::MW_p({bf},m::strict()) {{m::Outcome::Done(doc,_)=>{{',f'        assert(colour::from_doc(doc,{bf})=={cf});',f'        assert(crate::paint_model::default_raster(doc,STRICT)=={af});','    },_=>{}}',f'    assert(r::raw({af})=={len(raw)}nat) by (compute);',f'    assert(r::inflated({zf},{af},STRICT).out=={rf});',f'    compose_complete({bf},{hf},{zf},{entries}nat,{cf},{ef});','}']
  out +=[f'pub fn public_complete_{ident}() -> (result: Result<crate::png::Still,TryError>)',
   f'    ensures se::oom_member(se::requests({bf},false),result),se::eq({bf},false,se::native8(result)),',
   f'        match se::native8(result) {{Ok(value)=>expected_value_{ident}(value),Err(TryError::OutOfMemory(_))=>true,_=>false}},','{',
   '    let b=['+','.join(f'{v}u8' for v in data)+'];',f'    proof {{assert(b@=={bf});model_complete_{ident}();}}',
   '    crate::png::try_decode_still(&b)','}']
  index.append({'name':name,'entry':'still','gate_assignment':'complete at M6','expected_keys':list(expected),'fixture_source':'spec-frozen/VECTORS.json','fixture_sha256':fixture_sha,'input_sha256':hashlib.sha256(data).hexdigest(),'expected_rgba_sha256':hashlib.sha256(rgba).hexdigest(),'model_function':f'model_complete_{ident}','public_function':f'public_complete_{ident}','certificate_source':str(Path(source['source_path']).resolve().relative_to(Path(__file__).resolve().parents[3])),'certificate_source_sha256':source['source_sha256'],'allocation_exception':'actual ordinary-entry OOM membership in Req; every non-OOM result equals the complete frozen width, height and RGBA bytes'})
 out +=['} // verus!',''];(root/'src/witness_m6_m3.rs').write_text('\n'.join(out));(root/'evidence/m6-e6-m3-witness-index.json').write_text(json.dumps({'status':'Generated candidates; no proof credit from generation','generator':'tools/generate_m6_m3_witnesses.py','records':index},indent=2)+'\n');print('Generated',len(index),'complete E6-M3 candidates; no proof credit from generation.')
if __name__=='__main__':main()
