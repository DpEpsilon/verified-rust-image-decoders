#!/usr/bin/env python3
"""Author stored tiny-image tail-policy outcomes and original IDAT segments.

All raw bytes are the generator's [0,65,0,66]. A matching checksum followed
by data is ignored only at exactly R; an incorrect checksum always fails.
No expected fields, reference inflate or executable decode table is read.
"""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
V = {v['name']:v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}
def parts(v):
    data = bytes.fromhex(v['input_hex']); p = 8; out = []
    while p+12 <= len(data):
        n = int.from_bytes(data[p:p+4],'big')
        if data[p+4:p+8] == b'IDAT': out.append(data[p+8:p+8+n])
        p += n+12
    return out
def seq(v): return 'seq!['+', '.join(str(x)+('u8' if i == 0 else '') for i,x in enumerate(v))+']'
lines = ['//! Independently authored tail outcomes, including original later-IDAT boundaries.', 'use crate::{model as m, Status, InflateError as E, witness_policy::check_policy_parts};', 'use vstd::prelude::*;', 'verus! {']
manifest = []
for suffix in ['trailing-byte','second-stream','trailing-in-later-idat','bad-adler-with-trailing-byte-stays-an-error']:
    name = 'policy/zlib-tail/'+suffix; ps = parts(V[name+'/strict']); z = b''.join(ps); zs = seq(z)
    ident = suffix.replace('-','_')
    lines += [f'// Policy {name}: authored raw [0,65,0,66], exact R and non-exact R+1.', '#[verifier::spinoff_prover]', f'fn {ident}() {{']
    for i,p in enumerate(ps): lines += [f'    let segment_{i} = [{", ".join(hex(x)+("u8" if j == 0 else "") for j,x in enumerate(p))}];']
    lines += [f'    let parts: [&[u8];{len(ps)}] = [{", ".join("&segment_"+str(i) for i in range(len(ps)))}];', '    proof {', f'        reveal_with_fuel(crate::segments::parts_prefix,{len(ps)+1});', f'        assert(crate::segments::parts_prefix(parts@,{len(ps)}) =~= {zs});', '        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);', '    }', '    let ghost raw = seq![0u8,65,0,66];']
    for L in (4,5):
        for enforce in (True,False):
            for ignore in (False,True):
                status = 'AdlerMismatch' if suffix.startswith('bad-adler') else 'Done' if ignore and L == 4 else 'TrailingZlibData'
                model_status = 'm::InflateStatus::Done' if status == 'Done' else f'm::InflateStatus::Error(m::InflateFault::{status})'
                actual_status = 'Status::Done' if status == 'Done' else f'Status::Err(E::{status})'
                e,t = str(enforce).lower(),str(ignore).lower()
                lines += ['    proof {', f'        assert(m::Inflate({zs},{L},{e},{t}).status == {model_status}) by (compute);', f'        assert(m::Inflate({zs},{L},{e},{t}).out == seq![0u8,65,0,66]) by (compute);', '    }', f'    let mut output = [0xa5u8;{L}]; check_policy_parts(&parts,&mut output,{e},{t},{actual_status},Ghost(raw));']
    lines += ['}']
    manifest.append({'function':ident,'records':[name+'/strict',name+'/recommended-beta'],'segment_lengths':[len(p) for p in ps]})
lines += ['} // verus!', '']
(ROOT/'src/witness_policy_cases.rs').write_text('\n'.join(lines))
(ROOT/'tools/witness_policy.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Authored four tail-policy families / eight records, both original segment layouts and four window/tail combinations at R and R+1')
