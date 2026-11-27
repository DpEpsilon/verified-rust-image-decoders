#!/usr/bin/env python3
"""Generate five complete Adam7 calibration witnesses with pass-level certificates.

Expected pixel values come only from frozen VECTORS. Raw scanlines and inverse
filtering are untrusted hints checked by the unchanged filter-model contracts.
"""
import argparse,hashlib,json,re
from pathlib import Path
NAMES=['interlace/ct0-d1-8x8','interlace/ct6-d16-8x8','interlace/ct3-d4-8x8','interlace/ct6-d16-1x9','interlace/ct3-d4-1x1']
A7=[(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
MARKER='// A7 diagnostic resource probes only; no complete/public witnesses.\n'
def seq(xs):return 'seq!['+','.join(str(x)+'u8' for x in xs)+']'
def sha(data):return hashlib.sha256(data).hexdigest()
def body(text,name):
 m=re.search(r'\bfn '+re.escape(name)+r'\(',text);assert m,name
 op=text.index('\n{\n',m.start())+1;depth=1;at=op+1
 while depth:depth+=(text[at]=='{')-(text[at]=='}');at+=1
 return text[op+1:at-1]
def reconstruct(raw,rb,bpp,height):
 result=[];previous=[0]*rb
 for y in range(height):
  at=y*(rb+1);ft=raw[at];source=raw[at+1:at+1+rb];row=[]
  for x,value in enumerate(source):
   left=row[x-bpp] if x>=bpp else 0;up=previous[x];ul=previous[x-bpp] if x>=bpp else 0
   if ft==0:p=0
   elif ft==1:p=left
   elif ft==2:p=up
   elif ft==3:p=(left+up)//2
   else:
    v=left+up-ul;a=abs(v-left);b=abs(v-up);c=abs(v-ul);p=left if a<=b and a<=c else up if b<=c else ul
   row.append((value+p)%256)
  result.append((ft,source,row));previous=row
 return result
def generate(root):
 fixture=root.parent/'spec-frozen/VECTORS.json';vectors={r['name']:r for r in json.loads(fixture.read_text())};sources=[]
 for name in ['witness_extended.json','witness_classes.json']:
  path=root.parent/'inflate_verus/tools'/name;sources +=[(s,path,sha(path.read_bytes())) for s in json.loads(path.read_text())]
 path=root/'src/witness_m4_interlace_projection.rs';text=path.read_text();prefix=text.split(MARKER)[0]
 if MARKER not in text:assert prefix.endswith('} // verus!\n');prefix=prefix[:-len('} // verus!\n')]
 add=[MARKER];out=['//! Five complete Adam7 calibration witnesses with ordinary native calls.','use vstd::prelude::*;','use crate::{still_model as sm,still_equivalence as se,alloc_guard::TryError};','verus! {'];index=[]
 for name in NAMES:
  rec=vectors[name];e=rec['expected'];assert rec['entry']=='still' and set(e)=={'width','height','rgba_runs'}
  expected=b''.join(bytes.fromhex(run['hex'])*run['count'] for run in e['rgba_runs']);width,height=e['width'],e['height'];assert len(expected)==4*width*height
  data=bytes.fromhex(rec['input_hex']);depth,ct=data[24],data[25];assert data[28]==1 and int.from_bytes(data[16:20],'big')==width and int.from_bytes(data[20:24],'big')==height
  intent=[s for s in sources if [name,0] in s[0]['members']];assert len(intent)==1;source,source_path,source_sha=intent[0];raw=source['raw']
  ident=re.sub('[^A-Za-z0-9]','_',name).lower();bf=f'bytes_{ident}()';rf=f'raw_{ident}()';af=f'a_{ident}()';hf=f'h_{ident}()';zf=f'z_{ident}()';ext='crate::witness_m6_m4_a7';ef=f'{ext}::expected_{ident}()';ctx=f'e6_a7_colour_{ident}()'
  entries=16 if ct==3 else 0;palette=data[41:89] if ct==3 else b'';idat=93 if ct==3 else 33;n=int.from_bytes(data[idat:idat+4],'big');assert data[idat+4:idat+8]==b'IDAT'
  add +=[f'spec fn e6_a7_colour_{ident}() -> crate::colour_model::Context {{crate::colour_model::Context {{ct:{ct}u8,depth:{depth}u8,plte:{seq(palette)},trns:None}}}}']
  out +=[f'// {name}',f'pub open spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',f'pub open spec fn expected_{ident}() -> Seq<u8> {{{seq(expected)}}}',f'pub open spec fn expected_value_{ident}(value: sm::Value) -> bool {{value.width=={width}u32 && value.height=={height}u32 && value.pixels==expected_{ident}()}}']
  base=0;passes=[];bits=depth*({0:1,3:1,6:4}[ct]);bpp=max(1,(bits+7)//8)
  for k,(x0,y0,dx,dy) in enumerate(A7):
   pw=max(0,(width+dx-1-x0)//dx);ph=max(0,(height+dy-1-y0)//dy);rb=(pw*bits+7)//8
   if not pw or not ph:continue
   pass_raw=raw[base:base+ph*(rb+1)];rows=reconstruct(pass_raw,rb,bpp,ph);fn=f'e6_a7_pass_{ident}_{k}';passes.append(dict(k=k,width=pw,height=ph,base=base,row_bytes=rb,function=fn))
   for y,(_,_,line) in enumerate(rows):add +=[f'spec fn e6_a7_line_{ident}_{k}_{y}() -> Seq<u8> {{{seq(line)}}}']
   add +=['#[verifier::spinoff_prover]',f'proof fn {fn}()',f'    ensures forall|y: nat| y<{ph} ==> #[trigger] crate::witness_m6_rows_model::row_bytes8({ctx},r::line({rf},{af},{k}nat,y),{pw}nat)==crate::witness_m6_a7_model::expected_pass_row({af},{ef},{k}nat,y),','{',f'    hide(raw_{ident});hide(crate::colour_model::pixel);hide({ext}::expected_{ident});',f'    crate::colour_model::depth_values({depth}u8);crate::colour_model::depth_values(8u8);',f'    assert(r::pw({af},{k}nat)=={pw}nat && r::ph({af},{k}nat)=={ph}nat && r::base({af},{k}nat)=={base}nat && r::rb({af},{k}nat)=={rb}nat && crate::filter_model::bpp(r::bits({af}))=={bpp}nat) by (compute);']
   previous=None
   for y,(ft,encoded,line) in enumerate(rows):
    pv='None' if previous is None else 'Some('+seq(previous)+')';at=base+y*(rb+1);physical=[]
    for x in range(pw):
     p=4*((y0+y*dy)*width+x0+x*dx);physical.extend(expected[p:p+4])
    add +=[f'    crate::filter_model::pass_row_equation({rf},{base}nat,{rb}nat,{bpp}nat,{y}nat);',f'    assert({rf}[{at}]=={ft}u8) by (compute);',f'    assert(crate::filter_model::row_data({rf},{base}nat,{rb}nat,{y}nat)=={seq(encoded)}) by (compute);',f'    assert(crate::filter_model::previous_row({rf},{base}nat,{rb}nat,{bpp}nat,{y}nat)=={pv});',f'    assert(crate::filter_model::encoded_row({ft}u8,{seq(encoded)},{pv},{bpp}nat,{seq(line)},0,{rb}nat)) by (compute);',f'    crate::filter_model::checked_row_hint({ft}u8,{seq(encoded)},{pv},{bpp}nat,{seq(line)});',f'    assert(r::line({rf},{af},{k}nat,{y}nat)=={seq(line)});']
    converted=f'crate::witness_m6_rows_model::row_bytes8({ctx},e6_a7_line_{ident}_{k}_{y}(),{pw}nat)'
    for j,value in enumerate(physical):add +=[f'    assert({converted}[{j}]=={value}u8) by (compute);']
    add +=[f'    crate::witness_m6_rows_model::bounded_row_equal({converted},{seq(physical)});',f'    assert(crate::witness_m6_a7_model::expected_pass_row({af},{ef},{k}nat,{y}nat)=={seq(physical)}) by (compute);'];previous=line
   add +=[f'    assert forall|y: nat| y<{ph} implies #[trigger] crate::witness_m6_rows_model::row_bytes8({ctx},r::line({rf},{af},{k}nat,y),{pw}nat)==crate::witness_m6_a7_model::expected_pass_row({af},{ef},{k}nat,y) by {{']
   add +=['        '+('if y==0 {} '+''.join(f'else if y=={j} {{}} ' for j in range(1,ph-1))+f'else {{assert(y=={ph-1});}}' if ph>1 else 'assert(y==0);'),'    }','}'];base+=ph*(rb+1)
  assert base==len(raw)
  index.append(dict(name=name,entry='still',fixture_sha256=sha(fixture.read_bytes()),input_sha256=sha(data),expected=e,expected_rgba_sha256=sha(expected),certificate_source='png/png_verus/src/witness_m4_interlace_projection.rs',intermediate_hint_source='png/'+str(source_path.relative_to(root.parent)),intermediate_hint_source_sha256=source_sha,hints_are_not_expected_values=True,model_function=f'model_complete_{ident}',public_function=f'public_complete_{ident}',bridge_function=f'e6_a7_complete_{ident}',passes=passes,pixels=width*height))
  print(name,'nonempty passes',len(passes),'pixels',width*height)
 out +=['} // verus!',''];path.write_text(prefix+'\n'.join(add)+'\n} // verus!\n');(root/'src/witness_m6_a7_resource_probe.rs').write_text('\n'.join(out));(root/'evidence/a7-resource-probe-index.json').write_text(json.dumps(dict(status='Generated candidates; verification credit recorded separately',batch='E6-M4-A7-C1',generator='tools/generate_m6_m4_a7_witnesses.py',original_certificate_prefix_sha256=sha(prefix.encode()),records=index),indent=2)+'\n')
if __name__=='resource_probe_cli_disabled':
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);a=p.parse_args();generate(a.root)
