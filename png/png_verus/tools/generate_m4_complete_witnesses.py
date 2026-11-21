#!/usr/bin/env python3
"""Generate two complete M4 error witnesses from authored frozen fixtures.

Errors come from each frozen expected field, never from an oracle. The palette
raster certificate is the explicit rawp of spec-frozen/gen_vectors.py:701.
Every certificate is checked against the independent Verus models. This script
neither invokes a decoder nor imports the fixture generator.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT.parent / 'spec-frozen/VECTORS.json'
NAMES = {'precedence/index-row0-before-filter-row1',
         'interlace/1x1-filter-bytes-for-empty-passes'}


def seq(data):
    return 'seq![' + ','.join(f'{x}u8' for x in data) + ']'


def walk_certificate(cert,data,ident,header,znode,entries):
    """Reuse M3's checked chunk facts; never compute the entire recursive walk."""
    facts, sites = cert.read_facts(data,ident,'')
    requirements, pure, i = [], [], 0
    while i < len(facts):
        line = facts[i]
        if line.startswith('        assert forall|'):
            expression = line.strip()[len('assert '):-len(' by {')]
            requirements.append('('+expression.replace(f'bytes_{ident}()', 'b').replace(' implies ',' ==> ')+')')
            i += 1
            while facts[i] != '        }': i += 1
        elif line.startswith('        assert('):
            expression = line[len('        assert('):]
            expression = expression.rsplit(') by ',1)[0] if ') by ' in expression else expression[:-2]
            if f'bytes_{ident}()' in expression:
                requirements.append(expression.replace(f'bytes_{ident}()', 'b'))
                if line.endswith(' by {'):
                    i += 1
                    while facts[i] != '        }': i += 1
            else:
                pure.append(line)
                if line.endswith(' by {'):
                    i += 1
                    while facts[i] != '        }': pure.append(facts[i]); i += 1
                    pure.append(facts[i])
        else: raise AssertionError(line)
        i += 1
    pos = 33
    while data[pos+4:pos+8] != b'IDAT': pos += 12+int.from_bytes(data[pos:pos+4],'big')
    size = int.from_bytes(data[pos:pos+4],'big')
    payload = f'b.subrange({pos+8},{pos+8+size})=={znode}'
    requirements.append(payload)
    hidden='    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);'
    symbolic=['#[verifier::spinoff_prover]',f'proof fn walk_{ident}(b: Seq<u8>)',
              f'    requires b.len()=={len(data)},crate::model::mh(b)==Ok(({header},33nat)),',
              *['        '+x+',' for x in requirements],
              f'    ensures r::default_case(b,{header},{znode},{entries}nat),','{',hidden,
              *pure,'    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);',
              f'    reveal_with_fuel(crate::document_model::drive,{sites+1});',
              '    reveal_with_fuel(crate::document_model::concat_ranges,3);','}']
    concrete=[f'    assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by {{',
              '        reveal(crate::model::mh);',
              f'        assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by (compute);','    }',
              *facts,f'    assert({payload.replace("b.subrange",f"bytes_{ident}().subrange")}) by (compute);',
              f'    walk_{ident}(bytes_{ident}());']
    return symbolic,concrete,hidden


def inflate_certificate(ident,znode,anode,error):
    """Authored fixed-code literal words plus checked M2 model composition."""
    raw=[0,0,9,7,0,0] if error=='PaletteIndexOutOfRange' else [0,5]
    limit=len(raw)
    status='Done' if error=='PaletteIndexOutOfRange' else 'Full'
    terminal_pos=19+8*limit
    terminal_symbol=256 if status=='Done' else 0
    width=7 if terminal_symbol==256 else 8
    terminal_after=terminal_pos+width
    effect=(f'zm::StoredEffect::End {{out:{seq(raw)},position:{terminal_after}nat}}' if status=='Done'
            else f'zm::StoredEffect::Full {{out:{seq(raw)}}}')
    end=((terminal_after+7)//8+4)*8
    finished=(f'zm::StoredEffect::End {{out:{seq(raw)},position:{end}nat}}' if status=='Done' else effect)
    code=['#[verifier::spinoff_prover]',f'proof fn inflate_{ident}()',
          f'    ensures r::inflated({znode},{anode},STRICT)==(zm::Inflated {{status:zm::InflateStatus::{status},out:{seq(raw)},r2:false}}),',
          '{','    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MBlk);hide(zm::MOneBlock);',
          '    hide(zm::MCompressed);hide(zm::MCompressedStep);hide(zm::MDec);hide(zm::decode_prefix);hide(zm::cw);',
          f'    assert(r::raw({anode})=={limit}nat) by (compute);',
          f'    assert(zm::field_value({znode},16,1)==1nat && zm::field_value({znode},17,2)==1nat) by (compute);']
    for index,symbol in enumerate(raw+[terminal_symbol]):
        p=19+8*index
        n=7 if symbol==256 else 8
        word=0 if symbol==256 else 48+symbol
        code += [f'    assert(fixed::wire_word({znode},{p}nat,{n}nat)==zm::msb_bits({word}nat,{n}nat)) by (compute);',
                 f'    fixed::fixed_decode({znode},{p}nat,{symbol}int);']
    if status=='Done':code += [f'    fixed::end({znode},{terminal_pos}nat,{seq(raw)},{limit}nat,{terminal_after}nat);']
    else:code += [f'    fixed::full({znode},{terminal_pos}nat,{seq(raw)},{limit}nat,0u8,{terminal_after}nat);']
    for index in reversed(range(limit)):
        code += [f'    assert({seq(raw[:index])}.push({raw[index]}u8) =~= {seq(raw[:index+1])});',
                 f'    fixed::literal_step({znode},{19+8*index}nat,{seq(raw[:index])},{raw[index]}u8,{limit}nat,{27+8*index}nat,{effect});']
    if status=='Done':code += [f'    assert(zm::MFinish({znode},{terminal_after}nat,{seq(raw)},{limit}nat,false)=={finished}) by (compute);']
    code += [f'    fixed::envelope({znode},{limit}nat,{effect},{finished});',
             f'    assert(r::inflated({znode},{anode},STRICT)==zm::Inflated {{status:zm::InflateStatus::{status},out:{seq(raw)},r2:false}}) by {{reveal(zm::Inflate);}}','}']
    return code


def main():
    spec=importlib.util.spec_from_file_location('m3_chunk_certificates',ROOT/'tools/generate_m3_walk_witnesses.py')
    cert=importlib.util.module_from_spec(spec);spec.loader.exec_module(cert)
    sha = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    lines = ['//! Authored frozen-fixture complete M4 error witnesses.',
             '//! Model-versus-expected plus the actual public still call; OOM remains explicit.',
             'use vstd::prelude::*;',
             'use crate::{raster_model as r,filter_model as f,png::Error,policy::STRICT,m4_fixed_witness_model as fixed};',
             'use inflate_verus::model as zm;',
             'verus! {']
    rows = []
    for record in json.loads(SOURCE.read_text()):
        name = record['name']
        if name not in NAMES:
            continue
        assert record['entry'] == 'still' and 'pad_to' not in record
        data = bytes.fromhex(record['input_hex'])
        ident = re.sub('[^a-zA-Z0-9]', '_', name).lower()
        error = record['expected']['error']
        width = int.from_bytes(data[16:20], 'big')
        height = int.from_bytes(data[20:24], 'big')
        z = bytearray()
        entries = 0
        pos = 33
        while pos < len(data):
            n = int.from_bytes(data[pos:pos+4], 'big')
            tag = data[pos+4:pos+8]
            payload = data[pos+8:pos+8+n]
            assert len(payload) == n
            if tag == b'IDAT': z.extend(payload)
            if tag == b'PLTE': entries = n//3
            pos += n+12
        assert pos == len(data)
        header = (f'crate::model::Ihdr {{width:{width}u32,height:{height}u32,depth:{data[24]}u8,'
                  f'color_type:{data[25]}u8,interlace:{data[28]}u8}}')
        b, zs, h, shape = (f'bytes_{ident}()', f'z_{ident}()', f'h_{ident}()', f'a_{ident}()')
        symbolic,concrete,hidden=walk_certificate(cert,data,ident,header,zs,entries)
        lines += [f'// {name}; frozen fixture file sha256 {sha}',
                  f'spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',
                  f'spec fn z_{ident}() -> Seq<u8> {{{seq(z)}}}',
                  f'spec fn h_{ident}() -> crate::model::Ihdr {{{header}}}',
                  f'spec fn a_{ident}() -> r::Raster {{r::shape({h},{width}nat,{height}nat,{entries}nat,STRICT)}}',
                  *symbolic,*inflate_certificate(ident,zs,shape,error),'#[verifier::spinoff_prover]', f'proof fn model_{ident}()',
                  f'    ensures r::still({b},STRICT,false)==r::Projection::Error(Error::{error}),', '{',
                  '    hide(r::data);',hidden,*concrete]
        lines += [f'    inflate_{ident}();']
        if error == 'ExcessImageData':
            lines += [f'    assert(r::raw({shape})==2nat) by (compute);']
        else:
            assert error == 'PaletteIndexOutOfRange' and width == 2 and height == 2 and entries == 4
            # Authored rawp, not a decoder-derived expectation or output.
            raw = 'seq![0u8,0u8,9u8,7u8,0u8,0u8]'
            lines += [f'    let input={raw};',
                      f'    assert(r::base({shape},0)==0nat && r::rb({shape},0)==2nat && r::raw({shape})==6nat) by (compute);',
                      '    f::pass_row_equation(input,0,2,1,0);',
                      '    f::mf_none(f::row_data(input,0,2,0),None,1,2);',
                      f'    assert(r::line(input,{shape},0,0)==seq![0u8,9u8]);',
                      f'    assert(r::palette_sample(r::line(input,{shape},0,0),8u8,1nat)==9u8);',
                      f'    assert(!r::palette_valid(r::line(input,{shape},0,0),8u8,2nat,4nat));',
                      f'    assert(r::row_result(input,{shape},0,0)==Err(Error::{error}));',
                      f'    r::first_row_error(input,{shape},Error::{error});']
        lines += [f'    assert(r::data({zs},{shape},STRICT)==Some(Err(Error::{error}))) by {{reveal(r::data);}}',
                  f'    r::default_error({b},{h},{zs},{entries}nat,Error::{error});', '}',
                  f'fn public_{ident}() {{',
                  '    let b=[' + ','.join(f'{x}u8' for x in data) + '];',
                  f'    proof {{assert(b@=={b});model_{ident}();}}',
                  '    let result=crate::png::try_decode_still(&b);',
                  f'    assert(match result {{Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::sites::TryError::Format(Error::{error}))}});',
                  '}']
        rows.append({'name':name,'gate':'complete at M4','expected_error':error,
                     'source':'spec-frozen/VECTORS.json','source_sha256':sha,
                     'input_sha256':hashlib.sha256(data).hexdigest(),
                     'model_function':f'model_{ident}','public_function':f'public_{ident}'})
    assert {r['name'] for r in rows} == NAMES
    lines += ['} // verus!', '']
    (ROOT/'src/witness_m4_complete.rs').write_text('\n'.join(lines))
    (ROOT/'evidence/m4-complete-witness-index.json').write_text(json.dumps({
        'status':'Generated witnesses; verification credit is recorded separately in LEDGER.md',
        'generator':'tools/generate_m4_complete_witnesses.py','records':rows},indent=2)+'\n')
    print('Generated',len(rows),'complete-error candidates; no verification credit from generation.')


if __name__ == '__main__':
    main()
