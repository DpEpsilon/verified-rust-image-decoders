#!/usr/bin/env python3
"""Generate seven calibrated filter witnesses using shared canonical row proofs.

Expected values come only from frozen VECTORS. Reconstructed scanlines are
untrusted intermediate hints from existing authored raw intentions; the proof
checks every hint with the unchanged encoded_row/checked_row_hint contracts.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

NAMES=['filter/0-ct0-d1','filter/1-ct0-d8','filter/2-ct0-d16','filter/3-ct2-d8',
       'filter/4-ct2-d16','filter/1-ct4-d8','filter/4-ct6-d16']
MARKER='// E6-M4-FILT-C1 proof bridges; original M4 certificates above are unchanged.\n'
def seq(xs):return 'seq!['+','.join(str(x)+'u8' for x in xs)+']'
def digest(data):return hashlib.sha256(data).hexdigest()
def body(text,name):
    hit=re.search(r'\bfn '+re.escape(name)+r'\(',text);assert hit,name
    op=text.index('\n{\n',hit.start())+1;depth=1;at=op+1
    while depth:
        depth+=(text[at]=='{')-(text[at]=='}');at+=1
    return text[op+1:at-1]
def hints(raw,rb,bpp):
    result=[];previous=[0]*rb
    for y in range(4):
        at=y*(rb+1);ft=raw[at];source=raw[at+1:at+1+rb];row=[]
        for x,value in enumerate(source):
            left=row[x-bpp] if x>=bpp else 0;up=previous[x];ul=previous[x-bpp] if x>=bpp else 0
            if ft==0:p=0
            elif ft==1:p=left
            elif ft==2:p=up
            elif ft==3:p=(left+up)//2
            else:
                p0=left+up-ul;a=abs(p0-left);b=abs(p0-up);c=abs(p0-ul)
                p=left if a<=b and a<=c else up if b<=c else ul
            row.append((value+p)%256)
        result.append((ft,source,row));previous=row
    return result

def generate(root):
    fixture=root.parent/'spec-frozen/VECTORS.json';vectors={r['name']:r for r in json.loads(fixture.read_text())}
    intentions_path=root.parent/'inflate_verus/tools/witness_extended.json';intentions=json.loads(intentions_path.read_text())
    old_path=root/'src/witness_m4_filter_projection.rs';text=old_path.read_text();prefix=text.split(MARKER)[0]
    if MARKER not in text:
        assert prefix.endswith('} // verus!\n');prefix=prefix[:-len('} // verus!\n')]
    additions=[MARKER];out=['//! Seven complete frozen filter witnesses; calibrated shared row structure.','use vstd::prelude::*;','use crate::{still_model as sm,still_equivalence as se,alloc_guard::TryError};','verus! {'];index=[]
    for name in NAMES:
        record=vectors[name];assert record['entry']=='still';e=record['expected'];assert set(e)=={'width','height','rgba_runs'}
        expected=b''.join(bytes.fromhex(r['hex'])*r['count'] for r in e['rgba_runs']);assert len(expected)==96 and (e['width'],e['height'])==(6,4)
        ident=re.sub('[^A-Za-z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);ct,depth=data[25],data[24]
        sources=[s for s in intentions if [name,0] in s['members']];assert len(sources)==1;raw=sources[0]['raw'];channels={0:1,2:3,4:2,6:4}[ct];rb=(6*depth*channels+7)//8;bpp=max(1,(channels*depth+7)//8);assert len(raw)==4*(rb+1)
        n=int.from_bytes(data[33:37],'big');assert data[37:41]==b'IDAT' and data[45+n+4:45+n+8]==b'IEND'
        bf=f'bytes_{ident}()';af=f'a_{ident}()';rf=f'raw_{ident}()';hf=f'h_{ident}()';zf=f'z_{ident}()';colour=f'(crate::colour_model::Context {{ct:{ct}u8,depth:{depth}u8,plte:Seq::empty(),trns:None}})';ext='crate::witness_m6_m4_filter';ef=f'{ext}::expected_{ident}()';rows=hints(raw,rb,bpp)
        colour_literal=colour;colour=f'e6_colour_{ident}()'
        additions +=[f'spec fn e6_colour_{ident}() -> crate::colour_model::Context {{{colour_literal}}}']
        for y,(_,_,row) in enumerate(rows):
            additions +=[f'spec fn e6_line_{ident}_{y}() -> Seq<u8> {{{seq(row)}}}']
        out +=[f'// {name}',f'pub open spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',f'pub open spec fn expected_{ident}() -> Seq<u8> {{{seq(expected)}}}',f'pub open spec fn expected_value_{ident}(value: sm::Value) -> bool {{value.width=={e["width"]}u32 && value.height=={e["height"]}u32 && value.pixels==expected_{ident}()}}']
        additions +=['#[verifier::spinoff_prover]',f'proof fn e6_rows_{ident}()',f'    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8({colour},r::line({rf},{af},0,y),6nat)=={ef}.subrange((24*y) as int,(24*(y+1)) as int),','{',f'    hide(raw_{ident});hide(crate::colour_model::pixel);hide({ext}::expected_{ident});',f'    crate::colour_model::depth_values({depth}u8);',f'    assert(r::base({af},0)==0nat && r::rb({af},0)=={rb}nat && crate::filter_model::bpp(r::bits({af}))=={bpp}nat) by (compute);']
        previous=None
        for y,(ft,source,row) in enumerate(rows):
            pv='None' if previous is None else 'Some('+seq(previous)+')';row_expected=expected[24*y:24*(y+1)];at=y*(rb+1)
            additions +=[f'    crate::filter_model::pass_row_equation({rf},0nat,{rb}nat,{bpp}nat,{y}nat);',f'    assert({rf}[{at}]=={ft}u8) by (compute);',f'    assert(crate::filter_model::row_data({rf},0,{rb}nat,{y}nat)=={seq(source)}) by (compute);',f'    assert(crate::filter_model::previous_row({rf},0,{rb}nat,{bpp}nat,{y}nat)=={pv});',f'    assert(crate::filter_model::encoded_row({ft}u8,{seq(source)},{pv},{bpp}nat,{seq(row)},0,{rb}nat)) by (compute);',f'    crate::filter_model::checked_row_hint({ft}u8,{seq(source)},{pv},{bpp}nat,{seq(row)});',f'    assert(r::line({rf},{af},0,{y}nat)=={seq(row)});']
            converted=f'crate::witness_m6_rows_model::row_bytes8({colour},e6_line_{ident}_{y}(),6nat)'
            for j,value in enumerate(row_expected):
                additions +=[f'    assert({converted}[{j}]=={value}u8) by (compute);']
            additions +=[f'    crate::witness_m6_rows_model::bounded_row_equal({converted},{seq(row_expected)});',f'    assert({ef}.subrange({24*y},{24*(y+1)})=={seq(row_expected)}) by (compute);']
            previous=row
        additions +=['    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8('+colour+f',r::line({rf},{af},0,y),6nat)=={ef}.subrange((24*y) as int,(24*(y+1)) as int) by {{','        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}','    }','}']
        original_body=body(prefix,'model_'+ident);cut=original_body.index(f'    inflate_{ident}();scan_{ident}();');facts=original_body[:cut].replace('hide(r::data);',f'hide(r::data);hide({ext}::expected_{ident});')
        additions +=['#[verifier::spinoff_prover]',f'pub(crate) proof fn e6_complete_{ident}()',f'    ensures match crate::still_model::MS({ext}::bytes_{ident}()) {{crate::still_model::Outcome::Done(value)=>{ext}::expected_value_{ident}(value),_=>false}},','{',facts,f'    model_{ident}();',f'    inflate_{ident}();scan_{ident}();',f'    assert({bf}=={ext}::bytes_{ident}()) by (compute);',f'    assert(r::raw({af})=={len(raw)}nat) by (compute);',f'    assert(r::data({zf},{af},STRICT)==Some(Ok(()))) by {{reveal(r::data);}}',f'    assert(crate::model::valid_depth({ct}u8,{depth}u8)) by (compute);',f'    crate::witness_m6_rows_model::bare_colour({bf},{hf},{n}nat);',f'    e6_rows_{ident}();',f'    assert({ef}.len()==96nat) by (compute);',f'    crate::witness_m6_rows_model::complete_still({bf},{hf},{zf},{rf},{colour},{ef});','}']
        out +=['#[verifier::spinoff_prover]',f'pub proof fn model_complete_{ident}()',f'    ensures match sm::MS(bytes_{ident}()) {{sm::Outcome::Done(value)=>expected_value_{ident}(value),_=>false}},','{',f'    crate::witness_m4_filter_projection::e6_complete_{ident}();','}', '#[verifier::spinoff_prover]',f'pub fn public_complete_{ident}() -> (result: Result<crate::png::Still,TryError>)',f'    ensures se::oom_member(se::requests(bytes_{ident}(),false),result),',f'        se::eq(bytes_{ident}(),false,se::native8(result)),',f'        match se::native8(result) {{Ok(value)=>expected_value_{ident}(value),Err(TryError::OutOfMemory(_))=>true,_=>false}},','{','    let b=['+','.join(str(x)+'u8' for x in data)+'];',f'    proof {{assert(b@==bytes_{ident}());model_complete_{ident}();}}','    crate::png::try_decode_still(&b)','}']
        index.append(dict(name=name,entry='still',fixture_sha256=digest(fixture.read_bytes()),input_sha256=digest(data),expected=e,expected_rgba_sha256=digest(expected),certificate_source='png/png_verus/src/witness_m4_filter_projection.rs',intermediate_hint_source='png/inflate_verus/tools/witness_extended.json',intermediate_hint_source_sha256=digest(intentions_path.read_bytes()),hints_are_not_expected_values=True,model_function=f'model_complete_{ident}',public_function=f'public_complete_{ident}',bridge_function=f'e6_complete_{ident}',row_function=f'e6_rows_{ident}',fresh_pixels=24))
    out+=['} // verus!',''];old_path.write_text(prefix+'\n'.join(additions)+'\n} // verus!\n');(root/'src/witness_m6_m4_filter.rs').write_text('\n'.join(out))
    (root/'evidence/m6-e6-m4-filt-c1-witness-index.json').write_text(json.dumps(dict(status='Generated candidates; verification credit recorded separately',generator='tools/generate_m6_m4_filter_witnesses.py',batch='E6-M4-FILT-C1',original_certificate_prefix_sha256=digest(prefix.encode()),records=index),indent=2)+'\n')
    print('Generated seven complete filter candidates with one row-certificate function per record; no generation credit.')
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);a=p.parse_args();generate(a.root)
