#!/usr/bin/env python3
"""Generate M5 per-position APNG canvas projections; no complete frame-record claim.

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

def split_frame_content(base,out,ident,data,sources,controls,meta,w,h):
 result=[];i=0;N=4*w*h;bf=f'bytes_{ident}()';cf=f'id::constructor_doc(doc_{ident}())';colour='crate::colour_model::Context {ct:6u8,depth:8u8,plte:Seq::empty(),trns:None}'
 while i<len(out):
  line=out[i];match=re.match(r'proof fn content_'+re.escape(ident)+r'_(\d+)\(',line)
  if not match:result.append(line);i+=1;continue
  k=int(match.group(1));end=next(j for j in range(i+1,len(out)) if out[j]=='}');old=out[i:end+1];f=meta[k];sid=f'{ident}_s{controls[k][0]}';sw,sh=controls[k][1:];af,rf=f'a_{sid}()',f'raw_{sid}()';hf=f'before_draw_{ident}_{k}()';ef=f'expected_{ident}_{k}()';shape=f'(2nat,{N}nat,{w}nat,{f["x"]}nat,{f["y"]}nat,{str(f["blend"]==1).lower()})';body=old.index('{');cut=next(j for j,l in enumerate(old) if '::drawing_frame_context(' in l)+1;requires=old[1];post=old[2];prefix=old[body+1:cut];ctx=f'frame_observations_{ident}_{k}'
  # The one MK query exports its per-position equations; small pixel queries only consume them.
  observation=f'forall|x: nat,y: nat| x<{sw} && y<{sh} ==> (#[trigger] crate::paint_model::source_pixel({rf},{af},{colour},x,y,false,false)).is_ok() && crate::colour_model::observe({shape},actual,{sw}nat,x,y)==crate::colour_model::converted({shape},crate::paint_model::source_pixel({rf},{af},{colour},x,y,false,false).get_Ok_0(),crate::colour_model::observe({shape},{hf},{sw}nat,x,y))'
  footprint=f'forall|j: int| 0<=j<{N} && !crate::composite_model::drawing_region({shape},{sw}nat,{sh}nat,j) ==> #[trigger] actual[j]=={hf}[j]'
  ctxpost=post.replace(f'actual=={ef},',f'actual.len()=={N}nat,',1).rstrip(',')+f',crate::paint_model::progress({rf},{af},{colour},{shape},{hf},actual,r::count({af}),0,false),{observation},{footprint},'
  prefix.insert(1,f'    assert(r::raw({af})=={len(sources[controls[k][0]]["raw"])}nat) by (compute);')
  result +=[f'proof fn {ctx}(before: Seq<u8>,reference: Option<Seq<u8>>,actual: Seq<u8>)',requires,ctxpost,'{',*prefix,'}']
  rowcalls=list(dict.fromkeys(l for l in old[cut:] if l.startswith('    row_') and l.endswith('();')));groups=[];group=[]
  for l in old[cut:]:
   group.append(l)
   if re.match(r'    assert\(actual.subrange\(\d+,\d+\)==expected_',l):groups.append(group);group=[]
   if len(groups)==w*h:break
  assert len(groups)==w*h
  for pixel,group in enumerate(groups):
   name=f'frame_pixel_{ident}_{k}_{pixel}';cname=f'frame_colour_{ident}_{k}_{pixel}';checks=[l for l in group if l.startswith('    assert(crate::colour_model::pixel(') and l.endswith(' by (compute);')]
   assert len(checks)<=1
   if checks:
    check=checks[0];goal=check[len('    assert('):-len(') by (compute);')]
    px=goal.split('.is_ok() && ',1)[0];converted=goal.split('.is_ok() && ',1)[1].rsplit('==seq![',1)[0];piece='seq!['+goal.rsplit('==seq![',1)[1];m=re.search(r'seq!\[(.*?)\],(\d+)nat,false,false\)',px);wire=[int(v.strip().removesuffix('u8')) for v in m.group(1).split(',')];col=int(m.group(2));v='seq!['+','.join(f'{v}u16' for v in wire[4*col:4*col+4])+']';fixed=converted.replace(px+'.get_Ok_0()',v)
    result+=['#[verifier::spinoff_prover]',f'proof fn {cname}() ensures {goal},','{','    crate::colour_model::depth_values(8u8);',f'    assert({px}==Ok({v})) by (compute);']
    result +=[f'    assert({fixed}[{j}]=={piece}[{j}]) by (compute);' for j in range(4)]
    result +=[f'    let computed_bytes={fixed};let expected_bytes={piece};',f'    assert forall|j: int| 0<=j<4 implies #[trigger] computed_bytes[j]==expected_bytes[j] by {{','        if j==0 {}else if j==1 {}else if j==2 {}else {assert(j==3);}','    }',f'    assert({fixed}=~={piece});','}'];group=[f'    {cname}();' if l==check else l for l in group]
   if checks:
    cell=next(l for l in group if '::source_cell(' in l);m=re.search(r',([0-9]+)nat,([0-9]+)nat\)==',cell);x,y=m.groups();group.insert(0,f'    crate::paint_model::pixel_observe({rf},{af},{colour},{shape},{hf},actual,false,{x}nat,{y}nat);')
    src=f'crate::paint_model::source_pixel({rf},{af},{colour},{x}nat,{y}nat,false,false)';obs=f'crate::colour_model::observe({shape},actual,{sw}nat,{x}nat,{y}nat)';dest=f'crate::colour_model::observe({shape},{hf},{sw}nat,{x}nat,{y}nat)';converted=f'crate::colour_model::converted({shape},{src}.get_Ok_0(),{dest})';at=next(j for j,l in enumerate(group) if re.match(r'    assert\(actual.subrange\(\d+,\d+\)==seq!',l));goal=re.search(r'assert\(actual.subrange\((\d+),(\d+)\)==(seq!\[.*\])\);',group[at]);lo,hi,piece=goal.groups();group[at:at]=[f'    assert(crate::colour_model::destination_offset({shape},{sw}nat,{x}nat,{y}nat)=={lo}nat && crate::colour_model::pixel_bytes({shape})==4nat) by (compute);',f'    crate::colour_model::observed_slice({shape},actual,{sw}nat,{x}nat,{y}nat);',f'    assert({af}.width=={sw}nat);',f'    assert({src}.is_ok());',f'    assert({obs}=={converted});',f'    assert({converted}=={piece});',f'    assert({obs}==actual.subrange({lo},{hi}));'];group.insert(0,'    hide(crate::colour_model::pixel);')
   result +=['#[verifier::spinoff_prover]',f'proof fn {name}(before: Seq<u8>,reference: Option<Seq<u8>>,actual: Seq<u8>)',requires,f'    ensures actual.len()=={N}nat,actual.subrange({4*pixel},{4*pixel+4})=={ef}.subrange({4*pixel},{4*pixel+4}),','{',*([group[0]] if checks else []),f'    {ctx}(before,reference,actual);',*rowcalls,*[l for l in (group[1:] if checks else group) if l not in rowcalls],'}']
  tail=old[next(j for j,l in enumerate(old) if l.startswith(f'    assert({ef}.len()')):]
  result +=['#[verifier::spinoff_prover]',old[0],requires,post,'{',f'    hint_{ident}_{k}(before,reference);',*[f'    frame_pixel_{ident}_{k}_{pixel}(before,reference,actual);' for pixel in range(w*h)],*tail]
  i=end+1
 return result

def frame_content_certificates(base,ident,data,ss,sources,controls,expected,il,w,h):
 bf,df,cf=f'bytes_{ident}()',f'doc_{ident}()',f'id::constructor_doc(doc_{ident}())';N=4*w*h
 # These are intermediate disposal hints, not computed expected outcomes.
 meta=[];before_idat=True
 for _,tag,d in base.chunks(data):
  if tag==b'IDAT':before_idat=False
  if tag==b'fcTL':meta.append(dict(w=int.from_bytes(d[4:8],'big'),h=int.from_bytes(d[8:12],'big'),x=int.from_bytes(d[12:16],'big'),y=int.from_bytes(d[16:20],'big'),dispose=d[24],blend=d[25],reuse=before_idat))
 if not meta:meta=[dict(w=w,h=h,x=0,y=0,dispose=0,blend=0,reuse=True)]
 after=[base.expanded_expected(f) for f in expected['frames']];hints=[];out=[];points={}
 for j,_,_ in controls[:len(after)]:
  if j in points:continue
  sid=f'{ident}_s{j}';sw,sh=(w,h) if j==0 else next((fw,fh) for stream,fw,fh in controls if stream==j)
  rows,pts=base.row_hints(sid,sources[j]['raw'],sw,sh,6,8,il);out+=rows;points[j]=pts
 for k,rgba in enumerate(after):
  assert len(rgba)==N;f=meta[k];hint=bytearray(N) if k==0 else bytearray(after[k-1]);prev=meta[k-1] if k else None
  if prev:
   op=1 if k-1==0 and prev['dispose']==2 else prev['dispose']
   if op==1:
    for y in range(prev['y'],prev['y']+prev['h']):hint[4*(y*w+prev['x']):4*(y*w+prev['x']+prev['w'])]=bytes(4*prev['w'])
   elif op==2:hint=bytearray(hints[k-1])
  hints.append(bytes(hint));ef=f'expected_{ident}_{k}()';hf=f'before_draw_{ident}_{k}()';j,sw,sh=controls[k];sid=f'{ident}_s{j}';af,rf,zf=f'a_{sid}()',f'raw_{sid}()',f'z_{sid}()';shape=f'(2nat,{N}nat,{w}nat,{f["x"]}nat,{f["y"]}nat,{str(f["blend"]==1).lower()})';colour='crate::colour_model::Context {ct:6u8,depth:8u8,plte:Seq::empty(),trns:None}'
  init=f'if crate::iterator_canvas_model::reuse({cf},{k}nat) {{Seq::new(before.len(),|i: int|0u8)}}else{{crate::paint_model::disposed_reference(before,reference,{w}nat,crate::iterator_canvas_model::regions(crate::iterator_canvas_model::previous({cf},{k}nat)))}}'
  pres=[f'before.len()=={N}nat']
  if k:pres +=[f'before==expected_{ident}_{k-1}()']
  elif not f['reuse']:pres +=[f'before==Seq::new({N}nat,|i: int|0u8)']
  if prev and prev['dispose']==2 and k-1>0:pres +=[f'reference==Some(before_draw_{ident}_{k-1}())']
  out +=[f'spec fn expected_{ident}_{k}() -> Seq<u8> {{{base.seq(rgba)}}}',f'spec fn before_draw_{ident}_{k}() -> Seq<u8> {{'+(base.seq(hint) if any(hint) else f'Seq::new({N}nat,|i: int|0u8)')+'}','#[verifier::spinoff_prover]',f'proof fn hint_{ident}_{k}(before: Seq<u8>,reference: Option<Seq<u8>>)', '    requires '+','.join(pres)+',',f'    ensures ({{{init}}})=={hf},','{',f'    assert(crate::iterator_canvas_model::reuse({cf},{k}nat)=={str(f["reuse"]).lower()}) by (compute);',f'    assert(crate::iterator_canvas_model::previous({cf},{k}nat)=='+('None' if k==0 else f'Some((({prev["x"]}u32,{prev["y"]}u32,{prev["w"]}u32,{prev["h"]}u32),{1 if k-1==0 and prev["dispose"]==2 else prev["dispose"]}u8))')+') by (compute);',f'    let initial={init};',f'    assert(initial.len()=={N}nat);',f'    assert(initial=~={hf});','}']
  out +=['#[verifier::spinoff_prover]',f'proof fn content_{ident}_{k}(before: Seq<u8>,reference: Option<Seq<u8>>,actual: Seq<u8>)','    requires '+','.join(pres)+f',crate::iterator_canvas_model::step({bf},{cf},STRICT,{k}nat,before,reference,actual),',f'    ensures actual=={ef},({{{init}}})=={hf},','{',f'    hide(bytes_{ident});hide(z_{sid});hide(raw_{sid});hide(crate::colour_model::pixel);',f'    model_{ident}();data_{sid}();inflate_{sid}();hint_{ident}_{k}(before,reference);',f'    assert(crate::iterator_canvas_model::stream({bf},{cf},{k}nat)==Some({zf})) by (compute);',f'    crate::iterator_canvas_model::step_context({bf},{cf},STRICT,{k}nat,{zf},before,reference,actual);',f'    assert(crate::iterator_canvas_model::shape({cf},STRICT,{k}nat)=={af}) by (compute);',f'    assert(crate::iterator_canvas_model::rectangle({cf},{k}nat)==crate::composite_model::rect({f["x"]}u32,{f["y"]}u32,{sw}u32,{sh}u32)) by (compute);',f'    assert(crate::colour_model::from_doc({cf},{bf})==({colour})) by (compute);',f'    crate::paint_model::mk_context({zf},{af},{colour},STRICT,{w}nat,crate::composite_model::rect({f["x"]}u32,{f["y"]}u32,{sw}u32,{sh}u32),{str(f["blend"]==1).lower()},crate::iterator_canvas_model::regions(crate::iterator_canvas_model::previous({cf},{k}nat)),reference,before,actual,{str(f["reuse"]).lower()});',f'    crate::paint_model::complete_pixels({rf},{af},{colour},{shape},{hf},actual,false);',f'    crate::composite_model::drawing_frame_context({hf},actual,{shape},{sw}nat,{sh}nat);']
  used=set()
  for pidx in range(w*h):
   X,Y=pidx%w,pidx//w;piece=base.seq(rgba[4*pidx:4*pidx+4])
   if f['x']<=X<f['x']+sw and f['y']<=Y<f['y']+sh:
    x,y=X-f['x'],Y-f['y'];pk,col,row,line,rname=points[j][(x,y)]
    if rname not in used:out +=[f'    {rname}();'];used.add(rname)
    src=f'crate::paint_model::source_pixel({rf},{af},{colour},{x}nat,{y}nat,false,false)';px=f'crate::colour_model::pixel({colour},{base.seq(line)},{col}nat,false,false)'
    out +=[f'    assert(crate::paint_model::source_cell({af},{x}nat,{y}nat)==({pk}nat,{col}nat,{row}nat)) by (compute);',f'    assert({src}=={px});',f'    assert({px}.is_ok() && crate::colour_model::converted({shape},{px}.get_Ok_0(),crate::colour_model::observe({shape},{hf},{sw}nat,{x}nat,{y}nat))=={piece}) by (compute);',f'    assert(actual.subrange({4*pidx},{4*pidx+4})=={piece});']
   else:
    out +=[f'    assert({hf}.subrange({4*pidx},{4*pidx+4})=={piece}) by (compute);']
    for q in range(4):out +=[f'    assert(!crate::composite_model::drawing_region({shape},{sw}nat,{sh}nat,{4*pidx+q}int)) by (compute);',f'    assert(actual[{4*pidx+q}]=={hf}[{4*pidx+q}]);']
    out +=[f'    assert(actual.subrange({4*pidx},{4*pidx+4})=~={piece});']
   out +=[f'    assert({ef}.subrange({4*pidx},{4*pidx+4})=={piece}) by (compute);',f'    assert(actual.subrange({4*pidx},{4*pidx+4})=={ef}.subrange({4*pidx},{4*pidx+4}));']
  out +=[f'    assert({ef}.len()=={N}nat) by (compute);',f'    assert forall|i: nat| i<{w*h} implies #[trigger] actual.subrange((4*i) as int,(4*(i+1)) as int)=={ef}.subrange((4*i) as int,(4*(i+1)) as int) by {{']
  out +=['        '+('if' if i==0 else 'else if')+f' i=={i} {{}}' for i in range(w*h-1)];out +=[f'        else {{assert(i=={w*h-1});}}' if w*h>1 else '        assert(i==0);','    }',f'    crate::paint_model::pixel_blocks_equal(actual,{ef},4nat,{w*h}nat);','}']
 return split_frame_content(base,out,ident,data,sources,controls,meta,w,h),meta,hints

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);p.add_argument('--names',nargs='*');a=p.parse_args();root=a.root;base=load('m5_still_helpers',root/'tools/generate_m5_still_witnesses.py');walk=load('m5_apng_walk',root/'tools/generate_m3_walk_witnesses.py');fixture=root.parent/'spec-frozen/VECTORS.json';records={r['name']:r for r in json.loads(fixture.read_text())};selected=[r for r in records.values() if r['name'].startswith('apng/') and r['entry']=='frames' and (not a.names or r['name'] in a.names)];fw,author=base.forward_library(root);sha=base.digest(fixture)
 out=['//! M5 per-position canvas projections; complete frame records remain due at M6.','use vstd::prelude::*;','use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,iterator_data_model as id,alloc_guard::TryError,','    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};','use inflate_verus::model as zm;','verus! {'];index=[]
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
  out +=[f'    assert(id::MFr_data({bf},{cf},STRICT)==({proj}));',f'    id::canonical_saved_at({bf},{cf},STRICT);','}',*frame_content_certificates(base,ident,data,ss,sources,controls,expected,il,w,h)[0],f'fn public_{ident}() {{','    let b=['+','.join(f'{v}u8' for v in data)+'];',f'    proof {{assert(b@=={bf});model_{ident}();}}','    let mut dec=match crate::png::Frames::try_new(&b) {Ok(dec)=>dec,Err(TryError::OutOfMemory(_))=>return,_=>{assert(false);return;}};','    proof {','        crate::walk_policy::profiles_project_exactly();dec.allocation_history();',f'        id::from_walk_exact(b@,dec.model_document(),STRICT,{df},m::no_events());',f'        assert(dec.model_document()=={cf});','    }','    let info=dec.info();','    proof {info.public_observations(b.len() as nat);}',f'    assert(info.width=={expected["width"]}u32 && info.height=={expected["height"]}u32);']
  _,meta,hints=frame_content_certificates(base,ident,data,ss,sources,controls,expected,il,w,h)
  out +=['    proof {',f'        assert({cf}.ihdr=={hf}) by (compute);',f'        assert({hf}.width=={w}u32 && {hf}.height=={h}u32) by (compute);','        dec.canvas_geometry();',f'        assert(dec.canvas_pixels().len()=={4*w*h}nat);','    }']
  if not meta[0]['reuse']:
   out +=['    proof {dec.pixel_context();crate::iterator_canvas_model::context(dec.model_input(),dec.model_document(),dec.model_policy(),dec.canvas_trace());',f'        crate::paint_model::initial_transparent_context(b@,{cf},STRICT,dec.canvas_pixels());',f'        assert({cf}.ihdr=={hf}) by (compute);',f'        assert({hf}.width=={w}u32 && {hf}.height=={h}u32) by (compute);','        dec.canvas_geometry();',f'        assert(dec.model_document().ihdr.width=={w}u32 && dec.model_document().ihdr.height=={h}u32);',f'        assert(dec.canvas_pixels().len()==4*(dec.model_document().ihdr.width as nat)*(dec.model_document().ihdr.height as nat));',f'        assert(dec.canvas_pixels().len()=={4*w*h}nat);',f'        assert forall|j: int| 0<=j<{4*w*h} implies #[trigger] dec.canvas_pixels()[j]==0u8 by {{}};',f'        assert(dec.canvas_pixels()=~=Seq::new({4*w*h}nat,|i: int|0u8));','    }']
  for k in range(n):
   out +=[f'    assert(dec.history_view().returned=={k}nat);','    let ghost before=dec.history_view();','    proof {dec.pixel_context();crate::iterator_canvas_model::context(dec.model_input(),dec.model_document(),dec.model_policy(),dec.canvas_trace());}', '    let ghost prior_canvas=dec.canvas_pixels();','    let ghost prior_trace=dec.canvas_trace();',f'    let ghost reference=crate::iterator_canvas_model::reference(prior_trace,{k}nat);', '    let next=dec.try_next();',f'    proof {{id::call_context(b@,{cf},STRICT,before,id::kind(next));}}',f'    match next {{Ok(Some(view))=>{{proof {{content_{ident}_{k}(prior_canvas,reference,view.pixels());}}assert(view.pixels()==expected_{ident}_{k}());}},Err(TryError::OutOfMemory(_))=>return,_=>{{assert(false);return;}}}}',f'    assert(dec.canvas_pixels()==expected_{ident}_{k}());',f'    assert(dec.canvas_trace().before[{k}]==before_draw_{ident}_{k}());',f'    assert(dec.history_view().returned=={k+1}nat);','    proof {dec.allocation_history();id::history_context(dec.model_input(),dec.model_document(),dec.model_policy(),dec.history_view());}']
  out +=['    let ghost before=dec.history_view();','    let next=dec.try_next();',f'    proof {{id::call_context(b@,{cf},STRICT,before,id::kind(next));}}']
  if error:out +=[f'    match next {{Err(TryError::Format(e))=>assert(e==Error::{error}),Err(TryError::OutOfMemory(_))=>return,_=>{{assert(false);return;}}}}']
  else:out +=['    match next {Ok(None)=>{},Err(TryError::OutOfMemory(_))=>return,_=>{assert(false);return;}}']
  out +=['}'];index.append(dict(name=name,entry='frames',gate_assignment='projection at M5, complete at M6',projection='constructor/data success, dimensions, returned-frame count, every frozen canvas byte through MK and terminal error/IEND; full frame records remain due at M6',expected_projection={'width':expected['width'],'height':expected['height'],'returned_frames':n,'end':end,'canvas_sha256_per_returned_frame':[hashlib.sha256(base.expanded_expected(f)).hexdigest() for f in expected['frames']]},fixture_sha256=sha,input_sha256=hashlib.sha256(data).hexdigest(),model_function=f'model_{ident}',public_function=f'public_{ident}',certificate_sources=[{'source':q['source_path'],'sha256':q['source_sha256']} for q in sources]))
 out+=['} // verus!',''];(root/'src/witness_m5_frames.rs').write_text('\n'.join(out));(root/'evidence/m5-frames-witness-index.json').write_text(json.dumps({'status':'Generated candidates; verification credit recorded separately','generator':'tools/generate_m5_frames_witnesses.py','records':index},indent=2)+'\n');print('Generated',len(index),'M5 frame candidates; no proof credit from generation.')
if __name__=='__main__':main()
