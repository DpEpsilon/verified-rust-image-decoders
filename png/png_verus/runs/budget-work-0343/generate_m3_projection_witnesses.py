#!/usr/bin/env python3
"""Generate category-b whole-walk/preparation projections, not pixel witnesses.

Public expected dimensions come from the authored fixture. For records whose
frozen final result is a data error, only the independent header is projected.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('walk_certificates', ROOT/'tools/generate_m3_walk_witnesses.py')
certificates = importlib.util.module_from_spec(spec)
spec.loader.exec_module(certificates)
LABEL = 'M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile'



METADATA_HELPERS = r"""

#[verifier::spinoff_prover]
proof fn stored_structure(doc: m::Doc,value: crate::metadata_model::Value,base: nat)
    ensures m::store(doc,value,base).ihdr==doc.ihdr,m::store(doc,value,base).plte==doc.plte,
        m::store(doc,value,base).trns==doc.trns,m::store(doc,value,base).actl==doc.actl,
        m::store(doc,value,base).idat==doc.idat,m::store(doc,value,base).frames==doc.frames,
{}
#[verifier::spinoff_prover]
proof fn iccp_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
        z+1<d.len(),d[z as int+1]==0,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Iccp,d,2,8,0)==crate::metadata_model::Outcome::Value(
        crate::metadata_model::Value::Iccp {name_len:z,profile_start:z+2,profile_len:(d.len()-z-2) as nat}),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn text_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
    ensures crate::metadata_model::parse(crate::walk_model::Name::Text,d,2,8,0)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn ztxt_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
        z+1<d.len(),d[z as int+1]==0,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Ztxt,d,2,8,0)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn rgb_background_success(d: Seq<u8>,entries: nat)
    requires d.len()==6,entries<=256,
        forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(d,3)[i] as nat)<(1u32<<8u8) as nat,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Bkgd,d,2,8,entries)==crate::metadata_model::Outcome::Value(
        crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(d,3))),
{}
"""


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--names', nargs='+')
    parser.add_argument('--split-budget', action='store_true')
    args = parser.parse_args()
    inventory = json.loads((ROOT/'evidence/m3-witness-status-0282.json').read_text())
    lines = ['//! Whole-walk/preparation projections from authored frozen fixtures.',
             '//! M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.',
             'use vstd::prelude::*;',
             'use crate::{document_model as m,png::Events,policy::STRICT};',
             '', 'verus! {', METADATA_HELPERS, 'spec fn accepted(outcome: m::Outcome,h: crate::model::Ihdr) -> bool {match outcome {m::Outcome::Done(doc,ev)=>doc.ihdr==h && ev==m::no_events(),_=>false}}']
    rows = []
    for scheduled in inventory['records']:
        if scheduled['category'] != 'b':
            continue
        name = scheduled['name']
        if args.names is not None and name not in args.names:
            continue
        if args.names is None and name == 'apng-bad/pixel-budget-exactly-2^30-then-zlib':
            continue
        path = Path(scheduled['source'])
        fixture = next(x for x in json.loads(path.read_text()) if x['name'] == name)
        assert 'pad_to' not in fixture
        data = bytes.fromhex(fixture['input_hex'])
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        ident = re.sub('[^a-zA-Z0-9]', '_', name).lower()
        values = ','.join(f'{x}u8' for x in data)
        frames = scheduled['entry'] == 'frames'
        model = 'MWF_p' if frames else 'MW_p'
        preparation = 'frames' if frames else 'still'
        public = 'crate::png::Frames::try_new' if frames else 'crate::png::try_decode_still'
        strong_error = (name.startswith('ihdr/') and name.endswith('accepted-then-truncated-zlib')) or (frames and args.split_budget)
        data_offset = 61 if frames else 41
        width = str(fixture['expected']['width']) if 'width' in fixture['expected'] else f'crate::model::be32(bytes_{ident}(),16)'
        height = str(fixture['expected']['height']) if 'height' in fixture['expected'] else f'crate::model::be32(bytes_{ident}(),20)'
        header = (f'crate::model::Ihdr {{width:{width},height:{height},depth:{data[24]}u8,'
                  f'color_type:{data[25]}u8,interlace:{data[28]}u8}}')
        facts, sites = certificates.read_facts(data, ident, '')
        if frames:
            facts += certificates.apng_facts(data, ident)
        if strong_error:
            facts += [f'        assert(bytes_{ident}().subrange({data_offset},{data_offset+2})==seq![120u8,1u8]) by (compute);']
        pos = 33
        entries = 0
        while pos+12 <= len(data):
            size = int.from_bytes(data[pos:pos+4],'big')
            if pos+12+size > len(data): break
            tag, payload = data[pos+4:pos+8], data[pos+8:pos+8+size]
            if tag in [b'iCCP',b'zTXt',b'sPLT']:
                nul = payload.find(0)
                if nul >= 0 and nul+1 < size:
                    facts += [f'        assert(bytes_{ident}().subrange({pos+8},{pos+8+size})[{nul+1}]=={payload[nul+1]}u8) by (compute);']
            d = f'bytes_{ident}().subrange({pos+8},{pos+8+size})'
            if tag == b'PLTE':entries = size//3
            value, helper, compute = None, None, False
            if tag == b'iCCP':
                z = payload.index(0)
                value = f'Iccp {{name_len:{z},profile_start:{z+2},profile_len:{size-z-2}}}'
                helper = f'iccp_success({d},{z}nat);'
            elif tag in [b'tEXt',b'zTXt',b'sPLT',b'hIST']:
                value = 'Ignored'
                if tag == b'tEXt':helper = f'text_success({d},{payload.index(0)}nat);'
                if tag == b'zTXt':helper = f'ztxt_success({d},{payload.index(0)}nat);'
                if tag == b'hIST':compute = True
            elif tag == b'tIME':
                year = int.from_bytes(payload[:2],'big')
                value = 'Time('+','.join([f'{year}u16']+[f'{x}u8' for x in payload[2:]])+')'
                compute = True
            elif tag == b'sRGB':
                value = f'Srgb({payload[0]}u8)'
                compute = True
            elif tag == b'bKGD' and data[25] == 2:
                value = f'Bkgd(crate::metadata_model::words16({d},3))'
                helper = f'rgb_background_success({d},{entries}nat);'
                facts += [f'        assert forall|i: int| 0<=i<3 implies (#[trigger] crate::metadata_model::words16({d},3)[i] as nat)<(1u32<<8u8) as nat by {{']
                for index in range(3):
                    number = int.from_bytes(payload[2*index:2*index+2],'big')
                    branch = 'if' if index == 0 else 'else if'
                    facts += [f'            {branch} i=={index} {{assert(crate::metadata_model::words16({d},3)[i]=={number}u16);}}']
                facts += ['        }']
            if value is not None:
                expression = (f'crate::metadata_model::parse(crate::walk_model::Name::{certificates.NAMES[tag]},'
                              f'{d},2u8,8u8,{entries}nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::{value})')
                facts += [f'        assert({expression}) by {{', '            reveal(crate::metadata_model::parse);']
                if helper:facts += [f'            {helper}']
                elif compute:facts += [f'            assert({expression}) by (compute);']
                facts += ['        }']
            pos += size+12

        parse_hide = '    hide(crate::metadata_model::parse);' if any('crate::metadata_model::parse(' in line for line in facts) else ''
        control_hide = '    '+''.join(f'hide(crate::apng_model::{method});' for method in ['actl','fctl','fdat','finish']
                                     if any(f'crate::apng_model::{method}(' in line for line in facts))
        predicate = f'accepted(m::{model}(bytes_{ident}(),m::strict()),{header})'
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
                        while facts[i] != '        }':pure.append(facts[i]);i += 1
                        pure.append(facts[i])
            else:raise AssertionError(line)
            i += 1
        structural_store = []
        if name in ['plte/suggested-palette-in-rgb','order/bkgd-hist-after-plte-before-idat-rgb']:
            structural_store = [
                '    hide(m::store);',
                '    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|',
                '        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte',
                '        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl',
                '        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {',
                '        stored_structure(doc,value,base);','    }']
        split_helpers = []
        if frames and args.split_budget:
            split_helpers, split_composition = certificates.split_apng_steps(data,ident,facts,True,None)
        symbolic_header = header.replace(f'bytes_{ident}()', 'b')
        composition = [f'    reveal_with_fuel(m::drive,{sites+1});']
        if name == 'plte/suggested-palette-in-rgb':
            composition = [f'    let ghost s0=m::initial(crate::model::mh(b).unwrap().0,33,m::no_events());']
            for n in range(4):
                step = f'm::step(b,m::strict(),false,false,s{n})'
                composition += [f'    let ghost s{n+1}=match {step} {{m::Step::Next(next)=>next,_=>s{n}}};',
                                f'    assert({step}==m::Step::Next(s{n+1}));',
                                f'    m::step_progress(b,m::strict(),false,false,s{n});']
            composition += [f'    assert(match m::step(b,m::strict(),false,false,s4) {{m::Step::Done(doc,ev)=>doc.ihdr=={symbolic_header} && ev==m::no_events(),_=>false}});']
            for n in reversed(range(5)):
                composition += [f'    m::drive_step(b,m::strict(),false,false,s{n});',
                                f'    assert(accepted(m::drive(b,m::strict(),false,false,s{n}),{symbolic_header}));']
        if split_helpers:
            lines += split_helpers
            composition = [x.replace(f'bytes_{ident}()', 'b') for x in split_composition]
        if strong_error:
            composition += ['    crate::walk_policy::profiles_project_exactly();',
                f'    match m::{model}(b,m::strict()) {{',
                f'        m::Outcome::Done(doc,ev)=>{{crate::allocation_model::short_default_error(b,doc,STRICT,{data_offset}nat);crate::allocation_model::walk_inflate_error(b,STRICT,{str(frames).lower()},doc,ev);}},',
                '        _=>{assert(false);},','    }']
        error_post = f'crate::allocation_model::first_inflate_error(b,STRICT,{str(frames).lower()})==Some(crate::png::Error::TruncatedZlib)'
        lines += ['#[verifier::spinoff_prover]',f'proof fn symbolic_{ident}(b: Seq<u8>)',
                  f'    requires b.len()=={len(data)},crate::model::mh(b)==Ok(({symbolic_header},33nat)),',
                  *[f'        {r},' for r in requirements],
                  f'    ensures accepted(m::{model}(b,m::strict()),{symbolic_header}),',*([f'        {error_post},'] if strong_error else []),'{',
                  '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
                  parse_hide,control_hide,*structural_store,*pure,'    reveal(m::step);reveal(m::ordered);reveal(m::own);',
                  *composition,'}','']
        entry_prefix = [f'fn projection_{ident}() {{',f'    hide(m::{model});',f'    let b=[{values}];',
                        f'    proof {{assert(b@==bytes_{ident}());model_{ident}();}}']
        entry_suffix = []
        if frames and args.split_budget:
            entry_prefix = [f'fn public_preparation_{ident}(b: &[u8])',f'    requires b@==bytes_{ident}(),','{',
                            f'    hide(m::{model});hide(bytes_{ident});',f'    proof {{model_{ident}();}}']
            entry_suffix = [f'fn projection_{ident}() {{',f'    let b=[{values}];',f'    public_preparation_{ident}(&b);','}','']
        lines += [f'// {name.replace(chr(94),chr(95))}; fixture sha256 {digest}',
                  f'spec fn bytes_{ident}() -> Seq<u8> {{seq![{values}]}}',
                  f'spec fn accepted_{ident}() -> bool {{{predicate}}}',
                  '#[verifier::spinoff_prover]',
                  f'proof fn model_{ident}()',f'    ensures accepted_{ident}(),',*([f'        {error_post.replace("b,STRICT",f"bytes_{ident}(),STRICT")},'] if strong_error else []),'{',
                  '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
                  f'    hide(m::{model});',
                  parse_hide,control_hide,
                  f'    assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by {{',
                  '        reveal(crate::model::mh);',
                  f'        assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by (compute);','    }',
                  *facts,f'    symbolic_{ident}(bytes_{ident}());','}',
                  f'// {LABEL}',*entry_prefix,
                  '    let mut events=Events {r1:false,r3:false,r4:false};',
                  f'    let prepared=crate::callers::{preparation}(&b,&STRICT,&mut events);',
                  '    match prepared {',
                  f'        Ok(doc)=>{{assert(doc.ihdr.view()=={header});',
                  '            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}',
                  '        },','        Err(_)=>{assert(false);},','    }',
                  *(['    // The actual public call has no asserted final data/pixel outcome.'] if not strong_error else []),
                  f'    let _public_result={public}(&b);',
                  *(['    match _public_result {',
                     '        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>{},',
                     '        Err(crate::alloc_guard::sites::TryError::Format(e))=>{assert(e==crate::png::Error::TruncatedZlib);},',
                     '        Ok(_)=>{assert(false);},','    }'] if strong_error else []),
                  '}','',*entry_suffix]
        rows.append(dict(name=name, entry=scheduled['entry'], source=str(path), source_sha256=digest,
                         input_sha256=hashlib.sha256(data).hexdigest(), frozen_expected=fixture['expected'],
                         category='b', model_function='model_'+ident, function='projection_'+ident,
                         complete_m3_projection=True, complete_final_data_witness=False, status=LABEL))
    assert rows
    lines += ['} // verus!', '']
    (ROOT/'src/witness_m3_projection.rs').write_text('\n'.join(line.rstrip() for line in lines))
    (ROOT/'runs/m3-projection-witness-inputs.json').write_text(json.dumps(rows,indent=2)+'\n')
    print(f'Generated {len(rows)} M3 whole-walk/preparation projections, no final-data witnesses.')


if __name__ == '__main__':
    main()
