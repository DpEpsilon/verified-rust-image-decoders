#!/usr/bin/env python3
"""Generate M5 APNG data/control projections; no complete canvas claim.

Every expected count/end/shape comes from the frozen expected field. Inflate
hints come from M2 authored intentions or its checked forward-encoder rules.
No oracle or executable decoder supplies an expected result.
"""
import argparse,hashlib,importlib.util,json,re,sys
from pathlib import Path
sys.dont_write_bytecode=True

def load(n,p):
 sp=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m

def raw_size(w,h,ct,d,il,a7):
 bits={0:1,2:3,3:1,4:2,6:4}[ct]*d;total=0
 for x,y,dx,dy in (a7 if il else [(0,0,1,1)]):
  pw=max(0,(w+dx-1-x)//dx);ph=max(0,(h+dy-1-y)//dy)
  if pw and ph:total+=ph*(1+(pw*bits+7)//8)
 return total

def extra_source(base,root,records,name,z,index,fw):
 try:return base.source_for(root,records,name,z,fw)
 except AssertionError:pass
 if name=='apng/interlaced-subframe' and index==1:
  red=(255,0,0,255);green=(0,255,0,255);blue=(0,0,255,255);half=(0,0,255,128);px=([red,green,blue,half]*3)[:9];raw=fw['scan_intent'](px,3,3,6,8,1)
 elif name=='apng/frame1-truncated-data-ends' and index==1:
  raw=fw['scan_intent']([(0,255,0,255)]*4,2,2,6,8,0)[:-1]
 else:raise AssertionError(('Missing authored stream',name,index))
 if fw['stored_body'](raw)==z[2:-4]:return dict(raw=raw,steps=[],encoding='stored',source_path='spec-frozen/gen_vectors.py:1022-1025',source_sha256=base.digest(root.parent/'spec-frozen/gen_vectors.py'))
 for prefix in range(min(24,len(raw))+1):
  for lazy in [False,True]:
   for oldest in [False,True]:
    for origin in [0,1]:
     tokens=fw['greedy'](raw,prefix,lazy,oldest,origin)
     if fw['encode'](tokens)==z[2:-4]:return dict(raw=raw,steps=base.steps_for(tokens,fw),encoding='fixed-matches',source_path='spec-frozen/gen_vectors.py:1022-1031',source_sha256=base.digest(root.parent/'spec-frozen/gen_vectors.py'))
 raise AssertionError(('No forward token certificate',name,index))

def walk_certificate(base,walk,data,ident,hf,frames_entry=True):
 facts,sites=walk.read_facts(data,ident,None);facts+=walk.apng_facts(data,ident)
 symbolic,composition=walk.split_apng_steps(data,ident,facts,frames_entry,None)
 last=max(int(x) for l in symbolic for x in re.findall(r'spec fn split_'+ident+r'_state_(\d+)\(',l));state=f'split_{ident}_state_{last}({hf})'
 doc=f'doc_{ident}()';definition=f'spec fn doc_{ident}() -> m::Doc {{let s={state};m::Doc {{info:m::Info {{frame_count:s.control.frame_count,..s.doc.info}},..s.doc}}}}'
 model='MWF_p' if frames_entry else 'MW_p'
 old='match m::MWF_p(b,m::strict()) {m::Outcome::Done(doc,ev)=>doc.ihdr==h && doc.idat==seq![crate::walk_model::Range {offset:61nat,len:2nat}] && ev==m::no_events(),_=>false}'
 new=f'm::MWF_p(b,m::strict())==m::Outcome::Done({doc},m::no_events())';old=old.replace('m::MWF_p(',f'm::{model}(');count=sum(l.count(old) for l in symbolic);assert count>=1;symbolic=[l.replace(old,new) for l in symbolic]
 root=['#[verifier::spinoff_prover]',f'proof fn walk_{ident}() ensures m::MWF_p(bytes_{ident}(),m::strict())==m::Outcome::Done({doc},m::no_events()),','{',f'    hide(bytes_{ident});hide(m::MWF_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);',f'    assert(bytes_{ident}().len()=={len(data)}nat) by (compute);',f'    assert(crate::model::mh(bytes_{ident}())==Ok(({hf},33nat))) by {{reveal(crate::model::mh);assert(crate::model::mh(bytes_{ident}())==Ok(({hf},33nat))) by (compute);}}',*facts,f'    split_{ident}_compose(bytes_{ident}(),{hf});','}']
 symbolic=[l.replace('m::MWF_p(',f'm::{model}(') for l in symbolic]
 root=[l.replace('m::MWF_p(',f'm::{model}(').replace('hide(m::MWF_p)',f'hide(m::{model})') for l in root]
 check='    assert(actual.doc.frames =~= expected.doc.frames);'
 expanded='    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {\n        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {\n            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}\n        }\n    }\n'
 symbolic=[l.replace(check,expanded+check) for l in symbolic]
 return symbolic+[definition]+root

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);p.add_argument('--names',nargs='*');a=p.parse_args();root=a.root;base=load('m5_still_helpers',root/'tools/generate_m5_still_witnesses.py');walk=load('m5_apng_walk',root/'tools/generate_m3_walk_witnesses.py');fixture=root.parent/'spec-frozen/VECTORS.json';records={r['name']:r for r in json.loads(fixture.read_text())};selected=[r for r in records.values() if r['name'].startswith('apng/') and r['entry']=='frames' and (not a.names or r['name'] in a.names)];fw,author=base.forward_library(root);sha=base.digest(fixture)
 out=['//! M5 frame data/control projections; complete canvas witnesses remain due at M6.','use vstd::prelude::*;','use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,iterator_data_model as id,alloc_guard::TryError,','    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};','use inflate_verus::model as zm;','verus! {'];index=[]
 for record in selected:
  name=record['name'];ident=re.sub('[^a-zA-Z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);ih=data[16:29];w,h=int.from_bytes(ih[:4],'big'),int.from_bytes(ih[4:8],'big');ct,depth,il=ih[9],ih[8],ih[12];expected=record['expected'];assert ct==6 and depth==8
  bf,hf,df,cf=f'bytes_{ident}()',f'h_{ident}()',f'doc_{ident}()',f'id::constructor_doc(doc_{ident}())';ss=base.streams(data);controls=[];before_idat=True;stream_index=0
  for pos,tag,d in base.chunks(data):
   if tag==b'IDAT':before_idat=False
   if tag==b'fcTL':
    k=0 if before_idat else stream_index+1
    if not before_idat:stream_index+=1
    controls.append((k,int.from_bytes(d[4:8],'big'),int.from_bytes(d[8:12],'big')))
  if not controls:controls=[(0,w,h)]
  shapes=[(w,h)]+[(x,y) for k,x,y in controls if k>0];assert len(shapes)==len(ss)
  out +=[f'// {name}; frozen SHA256 {sha}',f'spec fn bytes_{ident}() -> Seq<u8> {{{base.seq(data)}}}',f'spec fn h_{ident}() -> crate::model::Ihdr {{crate::model::Ihdr {{width:{w}u32,height:{h}u32,depth:8u8,color_type:6u8,interlace:{il}u8}}}}',*walk_certificate(base,walk,data,ident,hf)]
  stream_status=[];sources=[]
  for j,z in enumerate(ss):
   sid=f'{ident}_s{j}';af=f'a_{sid}()';zf=f'z_{sid}()';rf=f'raw_{sid}()';sw,sh=shapes[j];limit=raw_size(sw,sh,ct,depth,il,base.A7)
   out +=[f'spec fn a_{sid}() -> r::Raster {{r::shape({hf},{sw}nat,{sh}nat,0nat,STRICT)}}']
   if not z:
    out +=[f'spec fn z_{sid}() -> Seq<u8> {{Seq::empty()}}',f'spec fn raw_{sid}() -> Seq<u8> {{Seq::empty()}}', '#[verifier::spinoff_prover]',f'proof fn inflate_{sid}() ensures zm::Inflate({zf},{limit}nat,true,false)==(zm::Inflated {{status:zm::InflateStatus::Error(zm::InflateFault::TruncatedZlib),out:{rf},r2:false}}),','{',f'    assert(zm::Inflate({zf},{limit}nat,true,false)==(zm::Inflated {{status:zm::InflateStatus::Error(zm::InflateFault::TruncatedZlib),out:{rf},r2:false}})) by (compute);','}'];status='TruncatedZlib';source=dict(source_path='P9 empty zlib stream',source_sha256=sha)
   else:
    source=extra_source(base,root,records,name,z,j,fw);out+=base.stream_certificate(sid,z,source,limit)
    n=len(source['raw']);checksum=((n+sum((n-i)*v for i,v in enumerate(source['raw'])))%65521)*65536+(1+sum(source['raw']))%65521
    status='AdlerMismatch' if int.from_bytes(z[-4:],'big')!=checksum else 'TruncatedImageData' if n<limit else None
    if status is None:out+=base.scan_certificate(sid,source['raw'],sw,sh,ct,depth,il,0,None)
   sources.append(source);stream_status.append(status);goal='Ok(())' if status is None else f'Err(Error::{status})'
   out +=['#[verifier::spinoff_prover]',f'proof fn data_{sid}() ensures r::data({zf},{af},STRICT)==Some({goal}),','{',f'    hide(z_{sid});hide(raw_{sid});hide(r::data);inflate_{sid}();',f'    assert(r::raw({af})=={limit}nat) by (compute);']
   if status is None:out +=[f'    scan_{sid}();']
   out +=[f'    assert(r::data({zf},{af},STRICT)==Some({goal})) by {{reveal(r::data);}}','}']
  n=len(expected['frames']);end=expected['end'];error=end['error'] if isinstance(end,dict) else None;assert stream_status[0] is None
  statuses=[stream_status[k] for k,sw,sh in controls];assert all(x is None for x in statuses[:n]) and (statuses[n]==error if error else len(statuses)==n)
  model_end=f'id::End::Format(Error::{error})' if error else 'id::End::Iend';proj=f'id::Projection {{frames:{n}nat,end:{model_end}}}'
  framefacts=[]
  for k,(j,sw,sh) in enumerate(controls):
   goal='Ok(())' if statuses[k] is None else f'Err(Error::{statuses[k]})';framefacts.append(f'id::frame_data({bf},{cf},STRICT,{k}nat)==Some({goal})')
  defaultfact=f'r::document_data({bf},{cf},STRICT)==Some(Ok(()))'
  out +=['#[verifier::spinoff_prover]',f'proof fn model_{ident}()',f'    ensures m::MWF_p({bf},m::strict())==m::Outcome::Done({df},m::no_events()),id::MFr_data_p({bf},STRICT)==id::Outcome::Frames({proj}),id::from_walk({bf},{cf},STRICT),id::default_success({bf},{cf},STRICT),',f'        {defaultfact},'+','.join(framefacts)+',','{',f'    hide(bytes_{ident});hide(r::data);hide(m::MWF_p);','    crate::walk_policy::profiles_project_exactly();',f'    walk_{ident}();id::from_walk_rule({bf},{cf},STRICT);']
  for j,z in enumerate(ss):out +=[f'    data_{ident}_s{j}();']
  out +=[f'    reveal_with_fuel(m::concat_ranges,{max(4,len(ss)+2)});',f'    assert(m::default_stream({bf},{cf})==Some(z_{ident}_s0())) by (compute);',f'    assert(r::document_shape({cf},STRICT)==a_{ident}_s0()) by (compute);',f'    assert({defaultfact});id::default_intro({bf},{cf},STRICT);']
  for k,(j,sw,sh) in enumerate(controls):
   out +=[f'    id::frame_rule({bf},{cf},STRICT,{k}nat);']
   if len(ss)>1 or any(tag==b'acTL' for pos,tag,d in base.chunks(data)):
    out +=[f'    assert(m::frame_stream({bf},{cf},{k}nat)==Some(z_{ident}_s{j}())) by (compute);',f'    assert(id::frame_shape({cf},STRICT,{k}nat)==a_{ident}_s{j}()) by (compute);']
   out +=[f'    assert({framefacts[k]});']
  for k in reversed(range(n+1)):out +=[f'    id::suffix_step({bf},{cf},STRICT,{k}nat);']
  out +=[f'    assert(id::MFr_data({bf},{cf},STRICT)==({proj}));',f'    id::canonical_saved_at({bf},{cf},STRICT);','}',f'fn public_{ident}() {{','    let b=['+','.join(f'{v}u8' for v in data)+'];',f'    proof {{assert(b@=={bf});model_{ident}();}}','    let mut dec=match crate::png::Frames::try_new(&b) {Ok(dec)=>dec,Err(TryError::OutOfMemory(_))=>return,_=>{assert(false);return;}};','    proof {','        crate::walk_policy::profiles_project_exactly();dec.allocation_history();',f'        id::from_walk_exact(b@,dec.model_document(),STRICT,{df},m::no_events());',f'        assert(dec.model_document()=={cf});','    }','    let info=dec.info();','    proof {info.public_observations(b.len() as nat);}',f'    assert(info.width=={expected["width"]}u32 && info.height=={expected["height"]}u32);']
  for k in range(n):
   out +=[f'    assert(dec.history_view().returned=={k}nat);','    let ghost before=dec.history_view();','    let next=dec.try_next();',f'    proof {{id::call_context(b@,{cf},STRICT,before,id::kind(next));}}','    match next {Ok(Some(_))=>{},Err(TryError::OutOfMemory(_))=>return,_=>{assert(false);return;}}',f'    assert(dec.history_view().returned=={k+1}nat);','    proof {dec.allocation_history();id::history_context(dec.model_input(),dec.model_document(),dec.model_policy(),dec.history_view());}']
  out +=['    let ghost before=dec.history_view();','    let next=dec.try_next();',f'    proof {{id::call_context(b@,{cf},STRICT,before,id::kind(next));}}']
  if error:out +=[f'    match next {{Err(TryError::Format(e))=>assert(e==Error::{error}),Err(TryError::OutOfMemory(_))=>return,_=>{{assert(false);return;}}}}']
  else:out +=['    match next {Ok(None)=>{},Err(TryError::OutOfMemory(_))=>return,_=>{assert(false);return;}}']
  out +=['}'];index.append(dict(name=name,entry='frames',gate_assignment='projection at M5, complete at M6',projection='constructor/data success, dimensions, returned-frame count and terminal error/IEND; no canvas/control-payload equality',expected_projection={'width':expected['width'],'height':expected['height'],'returned_frames':n,'end':end},fixture_sha256=sha,input_sha256=hashlib.sha256(data).hexdigest(),model_function=f'model_{ident}',public_function=f'public_{ident}',certificate_sources=[{'source':q['source_path'],'sha256':q['source_sha256']} for q in sources]))
 out+=['} // verus!',''];(root/'src/witness_m5_frames.rs').write_text('\n'.join(out));(root/'evidence/m5-frames-witness-index.json').write_text(json.dumps({'status':'Generated candidates; verification credit recorded separately','generator':'tools/generate_m5_frames_witnesses.py','records':index},indent=2)+'\n');print('Generated',len(index),'M5 frame candidates; no proof credit from generation.')
if __name__=='__main__':main()
