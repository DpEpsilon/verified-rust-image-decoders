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


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--names', nargs='+')
    args = parser.parse_args()
    inventory = json.loads((ROOT/'evidence/m3-witness-status-0282.json').read_text())
    lines = ['//! Whole-walk/preparation projections from authored frozen fixtures.',
             '//! M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.',
             'use vstd::prelude::*;',
             'use crate::{document_model as m,png::Events,policy::STRICT};',
             '', 'verus! {', 'spec fn accepted(outcome: m::Outcome,h: crate::model::Ihdr) -> bool {match outcome {m::Outcome::Done(doc,ev)=>doc.ihdr==h && ev==m::no_events(),_=>false}}']
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
        assert scheduled['entry'] == 'still'
        width = str(fixture['expected']['width']) if 'width' in fixture['expected'] else f'crate::model::be32(bytes_{ident}(),16)'
        height = str(fixture['expected']['height']) if 'height' in fixture['expected'] else f'crate::model::be32(bytes_{ident}(),20)'
        header = (f'crate::model::Ihdr {{width:{width},height:{height},depth:{data[24]}u8,'
                  f'color_type:{data[25]}u8,interlace:{data[28]}u8}}')
        facts, sites = certificates.read_facts(data, ident, '')
        pos = 33
        while pos+12 <= len(data):
            size = int.from_bytes(data[pos:pos+4],'big')
            if pos+12+size > len(data): break
            tag, payload = data[pos+4:pos+8], data[pos+8:pos+8+size]
            if tag in [b'iCCP',b'zTXt',b'sPLT']:
                nul = payload.find(0)
                if nul >= 0 and nul+1 < size:
                    facts += [f'        assert(bytes_{ident}().subrange({pos+8},{pos+8+size})[{nul+1}]=={payload[nul+1]}u8) by (compute);']
            pos += size+12

        predicate = f'accepted(m::MW_p(bytes_{ident}(),m::strict()),{header})'
        requirements, pure, i = [], [], 0
        while i < len(facts):
            line = facts[i]
            if line.startswith('        assert forall|'):
                expression = line.strip()[len('assert '):-len(' by {')]
                requirements.append('('+expression.replace(f'bytes_{ident}()', 'b')+')')
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
        symbolic_header = header.replace(f'bytes_{ident}()', 'b')
        lines += ['#[verifier::spinoff_prover]',f'proof fn symbolic_{ident}(b: Seq<u8>)',
                  f'    requires b.len()=={len(data)},crate::model::mh(b)==Ok(({symbolic_header},33nat)),',
                  *[f'        {r},' for r in requirements],
                  f'    ensures accepted(m::MW_p(b,m::strict()),{symbolic_header}),','{',
                  '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
                  *pure,'    reveal(m::step);reveal(m::ordered);reveal(m::own);',
                  f'    reveal_with_fuel(m::drive,{sites+1});','}','']
        lines += [f'// {name.replace(chr(94),chr(95))}; fixture sha256 {digest}',
                  f'spec fn bytes_{ident}() -> Seq<u8> {{seq![{values}]}}',
                  f'spec fn accepted_{ident}() -> bool {{{predicate}}}',
                  '#[verifier::spinoff_prover]',
                  f'proof fn model_{ident}()',f'    ensures accepted_{ident}(),','{',
                  '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);',
                  f'    assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by {{',
                  '        reveal(crate::model::mh);',
                  f'        assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by (compute);','    }',
                  *facts,f'    symbolic_{ident}(bytes_{ident}());','}',
                  f'// {LABEL}',f'fn projection_{ident}() {{','    hide(m::MW_p);',
                  f'    let b=[{values}];',f'    proof {{assert(b@==bytes_{ident}());model_{ident}();}}',
                  '    let mut events=Events {r1:false,r3:false,r4:false};',
                  '    let prepared=crate::callers::still(&b,&STRICT,&mut events);',
                  '    match prepared {',
                  f'        Ok(doc)=>{{assert(doc.ihdr.view()=={header});',
                  '            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}',
                  '        },','        Err(_)=>{assert(false);},','    }',
                  '    // The actual public call has no asserted final data/pixel outcome.',
                  '    let _public_result=crate::png::try_decode_still(&b);','}','']
        rows.append(dict(name=name, entry=scheduled['entry'], source=str(path), source_sha256=digest,
                         input_sha256=hashlib.sha256(data).hexdigest(), frozen_expected=fixture['expected'],
                         category='b', model_function='model_'+ident, function='projection_'+ident,
                         complete_m3_projection=True, complete_final_data_witness=False, status=LABEL))
    assert rows
    lines += ['} // verus!', '']
    (ROOT/'src/witness_m3_projection.rs').write_text('\n'.join(lines))
    (ROOT/'runs/m3-projection-witness-inputs.json').write_text(json.dumps(rows,indent=2)+'\n')
    print(f'Generated {len(rows)} M3 whole-walk/preparation projections, no final-data witnesses.')


if __name__ == '__main__':
    main()
