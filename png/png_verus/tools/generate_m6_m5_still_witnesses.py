#!/usr/bin/env python3
"""Complete M5 still witnesses from frozen expectations and existing certificates.

The original M5 proof prefix is retained verbatim. Only thin proof bridges are
appended there, so no large literal certificate needs to be regenerated/copied.
"""
import argparse,hashlib,importlib.util,json,re,sys
from pathlib import Path
sys.dont_write_bytecode=True
MARKER='// E6-M5-S generated proof bridges; original M5 certificates above are unchanged.\n'
OVERLAP='plte/suggested-palette-in-rgb'
def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);a=p.parse_args();root=a.root
 base=load('m6_existing_still_certificates',root/'tools/generate_m5_still_witnesses.py');fixture=root.parent/'spec-frozen/VECTORS.json';records=json.loads(fixture.read_text());fixture_sha=hashlib.sha256(fixture.read_bytes()).hexdigest()
 selected=[r for r in records if r['entry'] in ['still','still16'] and (r['name'] in base.NAMES or r['name'].startswith(('trns16/','apng/'))) and 'error' not in r['expected']];assert len(selected)==22
 old_path=root/'src/witness_m5_still.rs';old=old_path.read_text();prefix=old.split(MARKER,1)[0] if MARKER in old else old.rsplit('} // verus!',1)[0];prefix_sha=hashlib.sha256(prefix.encode()).hexdigest()
 bridges=[MARKER];out=['//! Complete E6 M5 still/still16 witnesses; frozen expected values only.','use vstd::prelude::*;','use crate::{still_model as sm,still_equivalence as se,alloc_guard::TryError};','verus! {'];index=[]
 for record in selected:
  name=record['name'];ident=re.sub('[^a-zA-Z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);wide=record['entry']=='still16';word=str(wide).lower();expected=record['expected'];rgba=base.expanded_expected(expected,wide);w,h=expected['width'],expected['height'];assert len(rgba)==(8 if wide else 4)*w*h
  if name==OVERLAP:
   index.append({'name':name,'entry':record['entry'],'gate_assignment':'complete at M6; cross-checked E6-M3 overlap, not regenerated','fixture_source':'spec-frozen/VECTORS.json','fixture_sha256':fixture_sha,'input_sha256':hashlib.sha256(data).hexdigest(),'expected_rgba_sha256':hashlib.sha256(rgba).hexdigest(),'model_function':'model_complete_plte_suggested_palette_in_rgb','public_function':'public_complete_plte_suggested_palette_in_rgb','witness_source':'src/witness_m6_m3.rs','fresh_witness':False});continue
  seq_input,seq_expected=base.seq(data),base.seq(rgba);bf,hf,zf,af,cf=[f'{x}_{ident}()' for x in ['bytes','h','z','a','colour']];entries=next((len(d)//3 for _,tag,d in base.chunks(data) if tag==b'PLTE'),0)
  bridges +=['#[verifier::spinoff_prover]',f'pub(crate) proof fn e6_bridge_{ident}(b: Seq<u8>,expected: Seq<u8>)',f'    requires b=={seq_input},expected=={seq_expected},',f'    ensures match crate::still_model::MS_wide(b,{word}) {{crate::still_model::Outcome::Done(value)=>value.width=={w}u32 && value.height=={h}u32 && value.pixels==expected,_=>false}},','{',f'    hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(expected_{ident});hide(m::MW_p);hide(zm::Inflate);',f'    assert({bf}=={seq_input}) by (compute);',f'    assert(expected_{ident}()=={seq_expected}) by (compute);',f'    model_{ident}();',f'    inflate_{ident}();scan_{ident}();',f'    assert(r::raw({af})==raw_{ident}().len()) by (compute);',f'    assert(r::data({zf},{af},STRICT)==Some(Ok(()))) by {{reveal(r::data);}};']
  for pos,tag,d in base.chunks(data):
   if tag in [b'PLTE',b'tRNS']:bridges +=[f'    assert({bf}.subrange({pos+8},{pos+8+len(d)})=={base.seq(d)}) by (compute);']
  bridges +=[f'    assert(crate::paint_model::domain({af})) by (compute);',f'    assert(crate::model::valid_depth({cf}.ct,{cf}.depth)) by (compute);',f'    assert({hf}.color_type=={cf}.ct && {hf}.depth=={cf}.depth) by (compute);',f'    assert({cf}.plte.len()/3=={entries}nat) by (compute);',f'    assert(match m::MW_p({bf},m::strict()) {{m::Outcome::Done(doc,_)=>crate::colour_model::from_doc(doc,{bf})=={cf},_=>false}});',f'    crate::witness_m6_still_model::canonical_still({bf},{hf},{zf},{entries}nat,{cf},{word});',f'    match crate::still_model::MS_wide({bf},{word}) {{crate::still_model::Outcome::Done(value)=>{{',f'        content_{ident}(value.pixels);','    },_=>{}}','}']
  nb,ne=f'e6_bytes_{ident}()',f'e6_expected_{ident}()'
  out +=[f'// {name}; frozen SHA256 {fixture_sha}',f'pub open spec fn e6_bytes_{ident}() -> Seq<u8> {{{seq_input}}}',f'pub open spec fn e6_expected_{ident}() -> Seq<u8> {{{seq_expected}}}',f'pub open spec fn expected_value_{ident}(value: sm::Value) -> bool {{value.width=={w}u32 && value.height=={h}u32 && value.pixels=={ne}}}', '#[verifier::spinoff_prover]',f'pub proof fn model_complete_{ident}()',f'    ensures match sm::MS_wide({nb},{word}) {{sm::Outcome::Done(value)=>expected_value_{ident}(value),_=>false}},','{',f'    crate::witness_m5_still::e6_bridge_{ident}({nb},{ne});','}',f'pub fn public_complete_{ident}() -> (result: Result<crate::png::{"Still16" if wide else "Still"},TryError>)',f'    ensures se::oom_member(se::requests({nb},{word}),result),se::eq({nb},{word},se::native{"16" if wide else "8"}(result)),',f'        match se::native{"16" if wide else "8"}(result) {{Ok(value)=>expected_value_{ident}(value),Err(TryError::OutOfMemory(_))=>true,_=>false}},','{','    let b=['+','.join(f'{x}u8' for x in data)+'];',f'    proof {{assert(b@=={nb});model_complete_{ident}();}}','    crate::png::'+('try_decode_still16' if wide else 'try_decode_still')+'(&b)','}']
  index.append({'name':name,'entry':record['entry'],'gate_assignment':'complete at M6','fixture_source':'spec-frozen/VECTORS.json','fixture_sha256':fixture_sha,'input_sha256':hashlib.sha256(data).hexdigest(),'expected_rgba_sha256':hashlib.sha256(rgba).hexdigest(),'expected_keys':list(expected),'model_function':f'model_complete_{ident}','public_function':f'public_complete_{ident}','bridge_function':f'e6_bridge_{ident}','witness_source':'src/witness_m6_m5_still.rs','certificate_source':'png/png_verus/src/witness_m5_still.rs','certificate_prefix_sha256':prefix_sha,'fresh_witness':True})
 out+=['} // verus!',''];old_path.write_text(prefix+'\n'.join(bridges)+'\n} // verus!\n');(root/'src/witness_m6_m5_still.rs').write_text('\n'.join(out));(root/'evidence/m6-e6-m5-s-witness-index.json').write_text(json.dumps({'status':'Generated candidates; generation earns no proof credit','generator':'tools/generate_m6_m5_still_witnesses.py','records':index},indent=2)+'\n');print('Generated 21 new complete still witnesses and cross-checked one committed M3 overlap; no proof credit from generation.')
if __name__=='__main__':main()
