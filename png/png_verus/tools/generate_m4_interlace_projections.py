#!/usr/bin/env python3
"""Generate all 36 Adam7 M4 format/shape projections, not complete pixels.

Expectations are the frozen dimensions and run byte counts. Checked inflate
certificates use M2's authored raw/token intentions, never decoder outcomes.
Empty passes contribute no filter byte. Full four-bit palettes use mask bounds.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT.parent / 'spec-frozen/VECTORS.json'
INTENT_FILES = [ROOT.parent/'inflate_verus/tools/witness_extended.json',
                ROOT.parent/'inflate_verus/tools/witness_classes.json']
A7 = [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]


def load(name, path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def seq(xs):
    return 'seq!['+','.join(f'{x}u8' for x in xs)+']'


def literal_steps(raw):
    """Forward RFC fixed-code word positions for the authored tiny literal class."""
    p=19;before=0;steps=[]
    for symbol in raw+[256]:
        n=8 if symbol<144 else 9 if symbol<256 else 7
        after=before+(symbol<256)
        steps.append(dict(pos=p,next=p+n,before=before,after=after,symbol=symbol,
                          le=0,ds=0,de=0,length=0,distance=0))
        p+=n;before=after
    return steps


def scan_certificate(ident,raw,width,height,ct,depth,entries):
    rf,af=f'raw_{ident}()',f'a_{ident}()'
    channels={0:1,3:1,6:4}[ct]
    bits=channels*depth;base=0;passes=[]
    for x0,y0,dx,dy in A7:
        pw=(width+dx-1-x0)//dx;ph=(height+dy-1-y0)//dy
        rb=(pw*bits+7)//8
        passes.append((pw,ph,rb,base))
        if pw and ph:base+=ph*(1+rb)
    assert base==len(raw)
    lines=['#[verifier::spinoff_prover]',f'proof fn scan_{ident}()',
           f'    ensures r::scan({rf},{af},0)==Ok(()),','{',f'    hide(raw_{ident});',
           f'    assert(r::count({af})==7nat) by (compute);',
           f'    r::scan_step({rf},{af},7);']
    for k in reversed(range(7)):
        pw,ph,rb,base=passes[k]
        lines += [f'    assert(r::pw({af},{k}nat)=={pw}nat && r::ph({af},{k}nat)=={ph}nat && r::rb({af},{k}nat)=={rb}nat && r::base({af},{k}nat)=={base}nat) by (compute);']
        if pw and ph:
            for row in range(ph):
                pos=base+row*(1+rb);ft=raw[pos];assert ft<=4
                lines += [f'    assert({rf}[{pos}]=={ft}u8) by (compute);']
                if ct==3:
                    assert depth==4 and entries==16
                    lines += [f'    r::palette_full4(r::line({rf},{af},{k}nat,{row}nat),{pw}nat);']
                lines += [f'    assert(r::row_result({rf},{af},{k}nat,{row}nat)==Ok(()));']
            lines += [f'    r::pass_step({rf},{af},{k}nat,{row}nat);' for row in reversed(range(ph+1))]
        else:
            lines += [f'    r::pass_step({rf},{af},{k}nat,0);']
        lines += [f'    r::scan_step({rf},{af},{k}nat);']
    lines += ['}']
    return lines


def main():
    shared=load('filter_projection_certificates',ROOT/'tools/generate_m4_filter_projections.py')
    cert=load('complete_m4_certificates',ROOT/'tools/generate_m4_complete_witnesses.py')
    walk=load('m3_walk_certificates',ROOT/'tools/generate_m3_walk_witnesses.py')
    schedule=json.loads((ROOT/'evidence/m4-witness-schedule.json').read_text())['records']
    names={x['name'] for x in schedule if x['name'].startswith('interlace/') and x['gate_assignment'].startswith('projection')}
    sources=[]
    for path in INTENT_FILES:
        sha=hashlib.sha256(path.read_bytes()).hexdigest()
        sources += [(x,path,sha) for x in json.loads(path.read_text())]
    fixture_sha=hashlib.sha256(FIXTURES.read_bytes()).hexdigest()
    lines=['//! Adam7 M4 format/shape projections; complete pixel witnesses remain due at M6.',
           'use vstd::prelude::*;',
           'use crate::{raster_model as r,png::Error,policy::STRICT,m4_trace_witness_model as t,',
           '    m4_checksum_witness_model as c,m4_stream_witness_model as stream,m4_crc_witness_model as crc_cert};',
           'use inflate_verus::model as zm;','verus! {']
    rows=[]
    for record in json.loads(FIXTURES.read_text()):
        name=record['name']
        if name not in names:continue
        assert record['entry']=='still' and 'pad_to' not in record
        candidates=[x for x in sources if [name,0] in x[0]['members']]
        assert len(candidates)==1
        intent,path,intent_sha=candidates[0]
        source=dict(intent)
        if not source.get('steps') and source['encoding']=='fixed-literals':
            source['steps']=literal_steps(source['raw'])
        assert all(type(x) is int and 0<=x<=255 for x in source['raw'])
        data=bytes.fromhex(record['input_hex']);ih=data[16:29]
        width,height=int.from_bytes(ih[:4],'big'),int.from_bytes(ih[4:8],'big')
        depth,ct=ih[8],ih[9]
        assert ih[12]==1 and ct in (0,3,6)
        z=bytearray();entries=0;pos=33
        while pos<len(data):
            n=int.from_bytes(data[pos:pos+4],'big');tag=data[pos+4:pos+8]
            if tag==b'IDAT':z.extend(data[pos+8:pos+8+n])
            if tag==b'PLTE':entries=n//3
            pos+=12+n
        assert pos==len(data)
        expected=record['expected']
        assert all(len(run['hex'])==8 and run['count']>0 for run in expected['rgba_runs'])
        expected_bytes=sum(run['count']*(len(run['hex'])//2) for run in expected['rgba_runs'])
        ident=re.sub('[^a-zA-Z0-9]','_',name).lower()
        b,zf,h,a=f'bytes_{ident}()',f'z_{ident}()',f'h_{ident}()',f'a_{ident}()'
        header=f'crate::model::Ihdr {{width:{width}u32,height:{height}u32,depth:{depth}u8,color_type:{ct}u8,interlace:1u8}}'
        symbolic,concrete,hidden=cert.walk_certificate(walk,data,ident,header,zf,entries)
        ready=f'(r::Projection::Ready {{width:{expected["width"]}u32,height:{expected["height"]}u32,bytes:{expected_bytes}nat}})'
        lines += [f'// {name}; frozen sha256 {fixture_sha}; authored M2 intent sha256 {intent_sha}',
                  f'spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',
                  f'spec fn z_{ident}() -> Seq<u8> {{{seq(z)}}}',
                  f'spec fn h_{ident}() -> crate::model::Ihdr {{{header}}}',
                  f'spec fn a_{ident}() -> r::Raster {{r::shape({h},{width}nat,{height}nat,{entries}nat,STRICT)}}',
                  *symbolic,*shared.stream_certificate(ident,z,source,include_scan=False),
                  *scan_certificate(ident,source['raw'],width,height,ct,depth,entries),
                  '#[verifier::spinoff_prover]',f'proof fn model_{ident}()',
                  f'    ensures r::still({b},STRICT,false)=={ready},','{',
                  f'    hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(r::data);',hidden,
                  f'    assert({b}.len()=={len(data)}nat) by (compute);',*concrete,
                  f'    inflate_{ident}();scan_{ident}();',
                  f'    assert(r::raw({a})=={len(source["raw"])}nat) by (compute);',
                  f'    assert(r::data({zf},{a},STRICT)==Some(Ok(()))) by {{reveal(r::data);}}',
                  f'    r::default_ready({b},{h},{zf},{entries}nat,false);','}',
                  f'fn public_{ident}() {{','    let b=['+','.join(f'{x}u8' for x in data)+'];',
                  f'    proof {{assert(b@=={b});model_{ident}();}}',
                  '    let result=crate::png::try_decode_still(&b);',
                  f'    assert(match result {{Ok(s)=>s.width=={expected["width"]}u32 && s.height=={expected["height"]}u32 && s.rgba@.len()=={expected_bytes}nat,',
                  '        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});','}']
        rows.append({'name':name,'gate':'M4 projection; complete pixel witness due at M6 (E6)',
                     'fixture_sha256':fixture_sha,'input_sha256':hashlib.sha256(data).hexdigest(),
                     'authored_inflate_intent_file':str(path.relative_to(ROOT.parent)),
                     'authored_inflate_intent_sha256':intent_sha,'encoding':source['encoding'],
                     'expected_projection':{'success':True,'width':expected['width'],'height':expected['height'],'bytes':expected_bytes},
                     'palette_bound':'all four-bit samples fit sixteen entries' if ct==3 else None,
                     'model_function':f'model_{ident}','public_function':f'public_{ident}'})
    assert len(rows)==36 and {x['name'] for x in rows}==names
    lines += ['} // verus!','']
    (ROOT/'src/witness_m4_interlace_projection.rs').write_text('\n'.join(lines))
    (ROOT/'evidence/m4-interlace-projection-index.json').write_text(json.dumps({
        'status':'Generated M4 projection candidates; verification credit recorded separately',
        'complete_pixel_witnesses_proved':0,'generator':'tools/generate_m4_interlace_projections.py','records':rows},indent=2)+'\n')
    print('Generated 36 Adam7 M4 projection candidates; zero complete pixel witnesses; no credit from generation.')


if __name__=='__main__':main()
