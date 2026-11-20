#!/usr/bin/env python3
"""Generate partial M3 model/header witnesses; never credit public-entry gates.

Inputs are the project's authored frozen fixtures, not third-party corpus bytes.
Expectations come from each frozen record, never from a decoder or oracle.
The three accepted-then-truncated-zlib records use only their header projection.
"""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
PNG = ROOT.parent
PREFIX_CASES = {
    'container/empty', 'container/short-signature', 'container/bad-signature',
    'container/signature-only', 'container/truncated-ihdr-header',
    'container/truncated-ihdr-crc', 'container/first-chunk-not-ihdr',
    'container/ihdr-length-12', 'container/ihdr-crc',
}


def main():
    lines = [
        '//! Authored frozen-fixture model/header facts, not public-entry gates.',
        '//! Category (a) still needs checked public dispatch; category (b) also',
        '//! needs full checked walk preparation. No pixel/data outcome is claimed.',
        'use vstd::prelude::*;',
        'use crate::{model,header,png::Error,document_model as m,policy::STRICT,png::Events};',
        '', 'verus! {',
    ]
    rows = []
    for relative in ['spec-frozen/VECTORS.json', 'limits-draft/VECTORS-CAPS.json']:
        path = PNG / relative
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        for record in json.loads(path.read_text()):
            name = record['name']
            if not (name.startswith('ihdr/') or name in PREFIX_CASES):
                continue
            data = bytes.fromhex(record['input_hex'])
            assert 'pad_to' not in record
            ident = re.sub('[^a-zA-Z0-9]', '_', name).lower()
            values = ','.join(f'{v}u8' for v in data)
            projected = record['expected']['error'] == 'TruncatedZlib'
            if projected:
                width = int.from_bytes(data[16:20], 'big')
                height = int.from_bytes(data[20:24], 'big')
                expected = (f'Ok((model::Ihdr {{width:model::be32(bytes_{ident}(),16),'
                            f'height:model::be32(bytes_{ident}(),20),depth:{data[24]},'
                            f'color_type:{data[25]},interlace:{data[28]}}},33nat))')
            else:
                expected = f'Err(Error::{record["expected"]["error"]})'
            lines += [f'// {name.replace(chr(94), chr(95))}; {relative} sha256 {digest}',
                      f'spec fn bytes_{ident}() -> Seq<u8> {{seq![{values}]}}',
                      f'fn header_{ident}() {{',
                      f'    let b=[{values}];',
                      f'    proof {{assert(b@==bytes_{ident}());',
                      f'        assert(model::mh(bytes_{ident}())=={expected}) by (compute);',
                      '    }',
                      '    let result=header::prefix(&b);']
            if projected:
                lines += ['    assert(result.is_ok());',
                          f'    assert(result.unwrap().0.view()==model::mh(bytes_{ident}()).unwrap().0);']
            else:
                error = record['expected']['error']
                lines += [f'    assert(matches!(result,Err(Error::{error})));',
                          f'    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::{error},m::no_events()));',
                          '    let mut ev=Events {r1:false,r3:false,r4:false};',
                          '    let still=crate::callers::still(&b,&STRICT,&mut ev);',
                          f'    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::{error}))));',
                          '    let frames=crate::callers::frames(&b,&STRICT,&mut ev);',
                          f'    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::{error}))));']
            lines += ['}', '']
            rows.append(dict(name=name,input_sha256=hashlib.sha256(data).hexdigest(),source=str(path),
                             source_sha256=digest,frozen_expected=record['expected'],function='header_'+ident,
                             status='partial model/header fact; not a complete public-entry witness',
                             category='b' if projected else 'a'))
    lines += ['} // verus!', '']
    (ROOT/'src/witness_m3_frozen.rs').write_text('\n'.join(lines))
    (ROOT/'runs/m3-header-witness-inputs.json').write_text(json.dumps(rows,indent=2)+'\n')
    print(f'Generated {len(rows)} partial frozen model/header witnesses.')


if __name__ == '__main__':
    main()
