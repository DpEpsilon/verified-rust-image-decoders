#!/usr/bin/env python3
"""Generate the 35 filter-only M4 format/shape projections, never pixel equality.

Frozen expected dimensions and run byte counts supply the restricted result.
M2's authored forward-encoder raw/token intentions supply checked certificates,
not expected outcomes. No decoder, oracle or fixture generator is invoked.
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
INTENTS = ROOT.parent / 'inflate_verus/tools/witness_extended.json'


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def seq(xs):
    return 'seq![' + ','.join(f'{x}u8' for x in xs) + ']'


def stream_certificate(ident, z, source):
    raw, steps = source['raw'], source['steps']
    n = len(raw)
    zf, rf, tf = f'z_{ident}()', f'raw_{ident}()', f'ts_{ident}()'
    encoding = source['encoding']
    assert encoding in ('stored', 'fixed-literals')
    sb = sum(raw)
    sp = n + sum((n-j)*v for j, v in enumerate(raw))
    checksum = (sp % 65521) * 65536 + (1+sb) % 65521
    cursor = 7+n if encoding == 'stored' else (steps[-1]['next']+7)//8
    assert cursor+4 == len(z)
    lines = [f'spec fn raw_{ident}() -> Seq<u8> {{{seq(raw)}}}']
    if encoding == 'fixed-literals':
        ts = 'seq![' + ','.join('t::Step {' + ','.join(f'{k}:{v}nat' for k,v in s.items()) + '}' for s in steps) + ']'
        lines += [f'spec fn ts_{ident}() -> Seq<t::Step> {{{ts}}}',
                  '#[verifier::spinoff_prover]', f'proof fn trace_{ident}()',
                  f'    ensures t::trace({zf},{rf},{tf},0,{len(steps)}nat),', '{',
                  f'    hide(z_{ident});hide(raw_{ident});hide(ts_{ident});hide(t::valid_step);',
                  f'    assert(t::trace({zf},{rf},{tf},0,{len(steps)}nat)) by (compute);', '}']
    facts = [f'{rf}.len()=={n}nat', f'{zf}.len()=={len(z)}nat',
             f'{zf}[0]==120u8', f'{zf}[1]=={z[1]}u8',
             f'zm::field_value({zf},16,1)==1nat',
             f'zm::field_value({zf},17,2)=={0 if encoding=="stored" else 1}nat',
             f'zm::be_value({zf},{cursor}nat,4)=={checksum}nat']
    if encoding == 'stored':
        facts += [f'{zf}.subrange(7,{7+n})=={rf}',
                  f'{zf}[3] as nat+256*{zf}[4] as nat=={n}nat',
                  f'{zf}[5] as nat+256*{zf}[6] as nat==u16::MAX as nat-{n}nat',
                  f'zm::be_value({zf},0,1)==120nat', f'zm::be_value({zf},1,1)=={z[1]}nat']
    else:
        facts += [f'{tf}.len()=={len(steps)}nat', f'{tf}[0].pos==19nat',
                  f'{tf}[0].before==0nat', f'{tf}.last().next=={steps[-1]["next"]}nat']
    lines += ['#[verifier::spinoff_prover]', f'proof fn shape_{ident}()',
              '    ensures '+',\n        '.join(facts)+',', '{',
              f'    hide(z_{ident});hide(raw_{ident});'+(f'hide(ts_{ident});' if steps else ''),
              *[f'    assert({fact}) by (compute);' for fact in facts], '}',
              '#[verifier::spinoff_prover]', f'proof fn checksum_{ident}()',
              f'    ensures zm::Adler({rf})=={checksum}nat,', '{',
              f'    hide(raw_{ident});hide(zm::sum_bytes);hide(zm::sum_prefixes);',
              f'    assert({rf}.len()=={n}nat) by (compute);',
              f'    assert(c::sums({rf},0,{n}nat)==({sb}nat,{sp}nat)) by (compute);',
              f'    c::sums_correct({rf},0,{n}nat);', f'    assert({rf}.subrange(0,{n}) =~= {rf});',
              '    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);',
              '    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);',
              f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521=={checksum}nat) by (compute);', '}',
              '#[verifier::spinoff_prover]', f'proof fn inflate_{ident}()',
              f'    ensures r::inflated({zf},a_{ident}(),STRICT)==(zm::Inflated {{status:zm::InflateStatus::Done,out:{rf},r2:false}}),',
              f'        {rf}.len()=={n}nat,', '{',
              f'    hide(z_{ident});hide(raw_{ident});'+(f'hide(ts_{ident});' if steps else ''),
              '    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);',
              f'    shape_{ident}();checksum_{ident}();',
              f'    assert(r::raw(a_{ident}())=={n}nat) by (compute);',
              f'    assert((120nat*256+{z[1]}nat)%31==0nat && ({z[1]}nat/32)%2==0nat) by (compute);']
    if encoding == 'stored':
        lines += [f'    stream::stored_stream({zf},{rf},{n}nat,120nat,{z[1]}nat,{checksum}nat);']
    else:
        lines += [f'    trace_{ident}();', f'    stream::fixed_stream({zf},{rf},{tf},{n}nat,{z[1]}nat,{checksum}nat);']
    lines += ['}', '#[verifier::spinoff_prover]', f'proof fn scan_{ident}()',
              f'    ensures r::scan({rf},a_{ident}(),0)==Ok(()),', '{',
              f'    hide(raw_{ident});',
              f'    assert(r::base(a_{ident}(),0)==0nat && r::ph(a_{ident}(),0)==4nat && r::count(a_{ident}())==1nat) by (compute);',
              f'    assert(r::rb(a_{ident}(),0)=={n//4-1}nat) by (compute);']
    for row in range(4):
        pos = (n//4)*row
        assert raw[pos] <= 4
        lines += [f'    assert({rf}[{pos}]=={raw[pos]}u8) by (compute);',
                  f'    assert(r::row_result({rf},a_{ident}(),0,{row}nat)==Ok(()));']
    lines += [f'    r::pass_step({rf},a_{ident}(),0,{row}nat);' for row in reversed(range(5))]
    lines += [f'    r::scan_step({rf},a_{ident}(),1);', f'    r::scan_step({rf},a_{ident}(),0);', '}']
    return lines


def main():
    cert = load('complete_m4_certificates', ROOT/'tools/generate_m4_complete_witnesses.py')
    walk = load('m3_walk_certificates', ROOT/'tools/generate_m3_walk_witnesses.py')
    sources = json.loads(INTENTS.read_text())
    fixture_sha = hashlib.sha256(FIXTURES.read_bytes()).hexdigest()
    intent_sha = hashlib.sha256(INTENTS.read_bytes()).hexdigest()
    lines = ['//! M4 filter format/shape projections; complete pixel witnesses remain due at M6.',
             'use vstd::prelude::*;',
             'use crate::{raster_model as r, png::Error, policy::STRICT, m4_trace_witness_model as t,',
             '    m4_checksum_witness_model as c, m4_stream_witness_model as stream, m4_crc_witness_model as crc_cert};',
             'use inflate_verus::model as zm;', 'verus! {']
    index = []
    for record in json.loads(FIXTURES.read_text()):
        name = record['name']
        if not name.startswith('filter/'):
            continue
        assert record['entry']=='still' and 'pad_to' not in record
        candidates = [x for x in sources if [name,0] in x['members']]
        assert len(candidates)==1
        source = candidates[0]
        data = bytes.fromhex(record['input_hex'])
        ih = data[16:29]
        width, height = int.from_bytes(ih[:4],'big'), int.from_bytes(ih[4:8],'big')
        assert (width,height)==(6,4) and ih[9]!=3 and ih[12]==0
        z = bytearray();pos=33
        while pos<len(data):
            n=int.from_bytes(data[pos:pos+4],'big')
            if data[pos+4:pos+8]==b'IDAT':z.extend(data[pos+8:pos+8+n])
            pos+=12+n
        assert pos==len(data)
        expected=record['expected']
        expected_bytes=sum(run['count']*(len(run['hex'])//2) for run in expected['rgba_runs'])
        assert all(len(run['hex'])==8 and run['count']>0 for run in expected['rgba_runs'])
        ident=re.sub('[^a-zA-Z0-9]','_',name).lower()
        b,zf,h,a=(f'bytes_{ident}()',f'z_{ident}()',f'h_{ident}()',f'a_{ident}()')
        header=f'crate::model::Ihdr {{width:{width}u32,height:{height}u32,depth:{ih[8]}u8,color_type:{ih[9]}u8,interlace:0u8}}'
        symbolic,concrete,hidden=cert.walk_certificate(walk,data,ident,header,zf,0)
        ready=f'(r::Projection::Ready {{width:{expected["width"]}u32,height:{expected["height"]}u32,bytes:{expected_bytes}nat}})'
        lines += [f'// {name}; frozen sha256 {fixture_sha}; authored M2 intent sha256 {intent_sha}',
                  f'spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',
                  f'spec fn z_{ident}() -> Seq<u8> {{{seq(z)}}}',
                  f'spec fn h_{ident}() -> crate::model::Ihdr {{{header}}}',
                  f'spec fn a_{ident}() -> r::Raster {{r::shape({h},{width}nat,{height}nat,0nat,STRICT)}}',
                  *symbolic,*stream_certificate(ident,z,source),
                  '#[verifier::spinoff_prover]', f'proof fn model_{ident}()',
                  f'    ensures r::still({b},STRICT,false)=={ready},', '{',
                  f'    hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(r::data);', hidden,
                  f'    assert({b}.len()=={len(data)}nat) by (compute);',*concrete,
                  f'    inflate_{ident}();scan_{ident}();',
                  f'    assert(r::raw({a})=={len(source["raw"])}nat) by (compute);',
                  f'    assert(r::data({zf},{a},STRICT)==Some(Ok(()))) by {{reveal(r::data);}}',
                  f'    r::default_ready({b},{h},{zf},0nat,false);', '}',
                  f'fn public_{ident}() {{',
                  '    let b=['+','.join(f'{v}u8' for v in data)+'];',
                  f'    proof {{assert(b@=={b});model_{ident}();}}',
                  '    let result=crate::png::try_decode_still(&b);',
                  f'    assert(match result {{Ok(s)=>s.width=={expected["width"]}u32 && s.height=={expected["height"]}u32 && s.rgba@.len()=={expected_bytes}nat,',
                  '        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});','}']
        index.append({'name':name,'gate':'M4 projection; complete pixel witness due at M6 (E6)',
                      'fixture_sha256':fixture_sha,'input_sha256':hashlib.sha256(data).hexdigest(),
                      'authored_inflate_intent_sha256':intent_sha,'encoding':source['encoding'],
                      'expected_projection':{'success':True,'width':expected['width'],'height':expected['height'],'bytes':expected_bytes},
                      'model_function':f'model_{ident}','public_function':f'public_{ident}'})
    assert len(index)==35
    lines += ['} // verus!','']
    (ROOT/'src/witness_m4_filter_projection.rs').write_text('\n'.join(lines))
    (ROOT/'evidence/m4-filter-projection-index.json').write_text(json.dumps({
        'status':'Generated M4 projection candidates; verification credit recorded separately',
        'complete_pixel_witnesses_proved':0,'generator':'tools/generate_m4_filter_projections.py','records':index},indent=2)+'\n')
    print('Generated',len(index),'M4 filter projection candidates, zero complete pixel witnesses; no credit from generation.')


if __name__=='__main__':main()
