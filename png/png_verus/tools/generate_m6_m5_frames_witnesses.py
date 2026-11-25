#!/usr/bin/env python3
"""Complete E6 M5 frame witnesses; all expected fields come only from VECTORS."""
import argparse
import hashlib
import json
from pathlib import Path
import re

MARKER = '// E6-M5-F generated proof bridges; original M5 certificates above are unchanged.\n'
def sha(b):
    return hashlib.sha256(b).hexdigest()
def seq(b):
    return 'seq![' + ','.join(str(x)+'u8' for x in b) + ']'
def pixels(frame):
    return b''.join(bytes.fromhex(run['hex'])*run['count'] for run in frame['rgba_runs'])
def option(v, suffix='u32'):
    return 'None' if v is None else f'Some({v}{suffix})'
def frame_value(ident,k,frame):
    x,y,w,h=frame['rect']
    delay='None' if frame['delay'] is None else f'Some(({frame["delay"][0]}u16,{frame["delay"][1]}u16))'
    return f'fm::Frame {{rgba:e6_expected_{ident}_{k}(),rect:crate::composite_model::rect({x}u32,{y}u32,{w}u32,{h}u32),delay:{delay},dispose:{frame["dispose"]}u8,blend:{frame["blend"]}u8}}'
def end_value(expected):
    return 'id::End::Iend' if expected['end']=='iend' else f'id::End::Format(Error::{expected["end"]["error"]})'
def streams_for_frames(data):
    result=[]; before=True; stream_index=0;pos=8
    while pos<len(data):
        n=int.from_bytes(data[pos:pos+4],'big');tag=data[pos+4:pos+8]
        if tag==b'IDAT':before=False
        if tag==b'fcTL':
            if before:result.append(0)
            else:stream_index+=1;result.append(stream_index)
        pos+=12+n
    return result or [0]
def public_witness(ident,data,e):
    n=len(e['frames']);bf=f'e6_bytes_{ident}()';req=f'id::requests_p({bf},STRICT,{n+1}nat)'
    return ['#[verifier::spinoff_prover]',f'pub fn public_complete_{ident}() -> (result: (Result<(),TryError>,Ghost<crate::witness_m6_frame_model::Observation>))',
            f'    ensures crate::still_equivalence::oom_member({req},result.0),',
            f'        match fm::MFr({bf}) {{fm::Outcome::Done(value)=>expected_value_{ident}(value)',
            '            && crate::witness_m6_frame_model::observed_result(value,result.0,result.1@),_=>false},',
            '{','    hide(fm::MFr);',
            '    let b=['+','.join(str(v)+'u8' for v in data)+'];',
            f'    proof {{assert(b@=={bf});model_complete_{ident}();}}',
            '    let ghost value=choose|value: fm::Value| fm::MFr(b@)==fm::Outcome::Done(value);',
            f'    crate::witness_m6_frame_model::exercise(&b,{n}usize,Ghost(value))','}']

def generate(root):
    fixture=root.parent/'spec-frozen/VECTORS.json'
    records=[r for r in json.loads(fixture.read_text()) if r['name'].startswith('apng/') and r['entry']=='frames']
    assert len(records)==13
    original=root/'src/witness_m5_frames.rs';text=original.read_text();prefix=text.split(MARKER)[0]
    if MARKER not in text:
        assert prefix.endswith('} // verus!\n');prefix=prefix[:-len('} // verus!\n')]
    bridges=[MARKER]
    out=['//! Complete frozen frame results and real public iterator calls.','use vstd::prelude::*;','use crate::{frame_model as fm,frame_equivalence as fe,iterator_data_model as id,','    document_model as d,png::Error,alloc_guard::{TryError,AllocSite},policy::STRICT};','verus! {']
    index=[]
    for record in records:
        name=record['name'];ident=re.sub('[^a-zA-Z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);e=record['expected'];n=len(e['frames']);w=e['width'];h=e['height'];N=4*w*h
        b=f'bytes_{ident}()';doc=f'id::constructor_doc(doc_{ident}())';ext='crate::witness_m6_m5_frames'
        out +=[f'// {name}',f'pub open spec fn e6_bytes_{ident}() -> Seq<u8> {{{seq(data)}}}']
        for k,f in enumerate(e['frames']):
            expected=pixels(f);assert len(expected)==N
            out +=[f'pub open spec fn e6_expected_{ident}_{k}() -> Seq<u8> {{{seq(expected)}}}']
        frames='seq!['+','.join(frame_value(ident,k,f) for k,f in enumerate(e['frames']))+']'
        out +=[f'pub open spec fn expected_frames_{ident}() -> Seq<fm::Frame> {{{frames}}}',f'pub open spec fn expected_value_{ident}(value: fm::Value) -> bool {{',f'    value.info.header.width=={w}u32 && value.info.header.height=={h}u32',f'    && value.num_plays=={option(e["num_plays"])} && value.info.num_plays=={option(e["num_plays"])}',f'    && value.frames==expected_frames_{ident}() && value.end=={end_value(e)}','}']
        streams=streams_for_frames(data)
        for k,f in enumerate(e['frames']):
            sid=f'{ident}_s{streams[k]}';a=f'a_{sid}()';z=f'z_{sid}()';raw=f'raw_{sid}()';fc='crate::frame_canvas_model';old='crate::iterator_canvas_model';fold=f'{fc}::fold({b},{doc},{k+1}nat)'
            before=f'{fc}::zero({doc})' if k==0 else f'{fc}::fold({b},{doc},{k}nat).canvas'
            ref='None' if k==0 else f'Some({fc}::fold({b},{doc},{k}nat).before_draw)'
            bridges +=['#[verifier::spinoff_prover]',f'proof fn e6_canvas_{ident}_{k}()',f'    ensures {fold}.canvas==expected_{ident}_{k}(),{fold}.before_draw==before_draw_{ident}_{k}(),','{',f'    hide(bytes_{ident});hide(r::inflated);hide(crate::colour_model::pixel);',f'    model_{ident}();data_{sid}();inflate_{sid}();scan_{sid}();']
            if k:bridges +=[f'    e6_canvas_{ident}_{k-1}();']
            bridges +=[f'    assert({doc}.ihdr==h_{ident}()) by (compute);',f'    assert(h_{ident}().width=={w}u32 && h_{ident}().height=={h}u32) by (compute);',f'    assert({fc}::bytes({doc})=={N}nat);']
            if k==0:
                bridges +=[f'    assert({before}=~=Seq::new({N}nat,|j: int|0u8));']
            bridges +=[f'    assert({old}::stream({b},{doc},{k}nat)==Some({z})) by (compute);',f'    assert({old}::shape({doc},STRICT,{k}nat)=={a}) by (compute);',f'    assert(r::raw({a})=={raw}.len()) by (compute);',f'    assert({fc}::geometry({doc},{k}nat)) by (compute);',f'    assert(crate::colour_model::from_doc({doc},{b})==crate::colour_model::Context {{ct:6u8,depth:8u8,plte:Seq::empty(),trns:None}}) by (compute);',f'    assert(crate::model::valid_depth(6u8,8u8)) by (compute);',f'    crate::witness_m6_frame_model::canonical_step({b},{doc},{k}nat);',f'    {fc}::lengths({b},{doc},{k}nat);',f'    content_{ident}_{k}({before},{ref},{fold}.canvas);',f'    {fc}::fold_step({b},{doc},{k+1}nat);','}']
        bridges +=['#[verifier::spinoff_prover]',f'pub(crate) proof fn e6_complete_{ident}()',f'    ensures match crate::frame_model::MFr({ext}::e6_bytes_{ident}()) {{crate::frame_model::Outcome::Done(value)=>{ext}::expected_value_{ident}(value),_=>false}},',f'        crate::frame_equivalence::requests({ext}::e6_bytes_{ident}()).is_prefix_of(id::requests_p({ext}::e6_bytes_{ident}(),STRICT,{n+1}nat)),','{',f'    hide(bytes_{ident});',f'    model_{ident}();',f'    crate::frame_equivalence::canonical_requests({b},{doc},{n+1}nat);',f'    assert({b}=={ext}::e6_bytes_{ident}()) by (compute);']
        for k in reversed(range(n+1)):
            bridges +=[f'    id::suffix_step({b},{doc},STRICT,{k}nat);']
        for k in range(n):
            bridges +=[f'    e6_canvas_{ident}_{k}();',f'    assert(expected_{ident}_{k}()=={ext}::e6_expected_{ident}_{k}()) by (compute);']
            for field in ['rect','delay','dispose','blend']:
                bridges +=[f'    assert(crate::frame_model::record({b},{doc},{k}nat).{field}=={ext}::expected_frames_{ident}()[{k}].{field}) by (compute);']
            bridges +=[f'    assert(crate::frame_model::record({b},{doc},{k}nat)=={ext}::expected_frames_{ident}()[{k}]);']
        bridges +=[f'    assert(id::MFr_data({b},{doc},STRICT)==(id::Projection {{frames:{n}nat,end:{end_value(e)}}}));',f'    let value=match crate::frame_model::from_document({b},{doc}) {{crate::frame_model::Outcome::Done(value)=>value,_=>{{assert(false);return;}}}};',f'    assert(value.frames.len()=={n}nat);',f'    assert forall|k: int| 0<=k<{n} implies #[trigger] value.frames[k]=={ext}::expected_frames_{ident}()[k] by {{',*[f'        if k=={k} {{}}' for k in range(n)],'    }',f'    assert(value.frames=~={ext}::expected_frames_{ident}());','}']
        out +=['#[verifier::spinoff_prover]',f'pub proof fn model_complete_{ident}()',f'    ensures match fm::MFr(e6_bytes_{ident}()) {{fm::Outcome::Done(value)=>expected_value_{ident}(value),_=>false}},',f'        fe::requests(e6_bytes_{ident}()).is_prefix_of(id::requests_p(e6_bytes_{ident}(),STRICT,{n+1}nat)),','{',f'    crate::witness_m5_frames::e6_complete_{ident}();','}']
        out +=public_witness(ident,data,e)
        index.append(dict(name=name,entry='frames',model_function=f'model_complete_{ident}',public_function=f'public_complete_{ident}',fixture_sha256=sha(fixture.read_bytes()),input_sha256=sha(data),expected=e,expected_canvas_sha256=[sha(pixels(f)) for f in e['frames']],certificate_source='png/png_verus/src/witness_m5_frames.rs'))
    out+=['} // verus!','']
    original.write_text(prefix+'\n'.join(bridges)+'\n} // verus!\n')
    (root/'src/witness_m6_m5_frames.rs').write_text('\n'.join(out))
    (root/'evidence/m6-e6-m5-f-witness-index.json').write_text(json.dumps(dict(status='Generated; verification credit recorded separately',generator='tools/generate_m6_m5_frames_witnesses.py',original_certificate_prefix_sha256=sha(prefix.encode()),records=index),indent=2)+'\n')
    print(f'Generated {len(records)} complete frame model candidates; no generation credit.')
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);a=p.parse_args();generate(a.root)
