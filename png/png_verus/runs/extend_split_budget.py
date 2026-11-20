#!/usr/bin/env python3
"""Extend checked per-step scaffolding to the two authored budget fixtures."""
from pathlib import Path
r=Path('/home/ws/projects/verified-decoders/png/png_verus')
p=r/'tools/generate_m3_walk_witnesses.py';s=p.read_text()
old="    terminal_error = None\n    while pos+12<=len(data):"
new="    terminal_error = None\n    terminal_done = False\n    while pos+12<=len(data):"
assert old in s;s=s.replace(old,new,1)
old="        if error_match:\n            terminal_error=error_match.group(1)"
new="""        done = tag==b'IEND' and value=='Ok(())'
        if done:
            assert frozen_error is None
            terminal_done=True
            state=f'{prefix}_state_{stage}(h)'
            result=f'm::Step::Done(m::Doc {{info:m::Info {{frame_count:{state}.control.frame_count,..{state}.doc.info}},..{state}.doc}},m::no_events())'
        elif error_match:
            terminal_error=error_match.group(1)"""
assert old in s;s=s.replace(old,new,1)
old="        output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_step_{stage}(b: Seq<u8>,h: crate::model::Ihdr)',\n                   '    requires '+',\\n        '.join(req)+','"
new="""        # Each step receives only its own wire/payload certificates; the root
        # composition still receives and checks the complete certificate set.
        payload=f'b.subrange({pos+8},{pos+8+size})'
        wire=f'crate::critical_model::read(b,{pos}nat,'
        step_req=[x for x in req if not re.search(r'\\bb\\b',x)
                  or 'b.len()' in x or 'crate::model::mh(b)' in x
                  or payload in x or wire in x]
        step_hides=[hides[0], '    '+''.join(f'hide(crate::apng_model::{method});'
                    for method in ['actl','fctl','fdat','finish','begin_idat']
                    if any(f'crate::apng_model::{method}(' in x for x in step_req))]
        output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_step_{stage}(b: Seq<u8>,h: crate::model::Ihdr)',
                   '    requires '+',\\n        '.join(step_req)+','"""
assert old in s;s=s.replace(old,new,1)
old="                   '{',*hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);']\n        if not error_match:"
new="                   '{',*step_hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);']\n        if not error_match and not done:"
assert old in s;s=s.replace(old,new,1)
s=s.replace('        if terminal_error:break','        if terminal_error or terminal_done:break',1)
s=s.replace('    assert terminal_error is not None','    assert terminal_error is not None or terminal_done',1)
old="    output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_compose(b: Seq<u8>,h: crate::model::Ihdr)',"
new="""    conclusion=(f'm::{model}(b,m::strict())==m::Outcome::Error(Error::{frozen_error},m::no_events())'
                if frozen_error is not None else
                f'match m::{model}(b,m::strict()) {{m::Outcome::Done(doc,ev)=>doc.ihdr==h && ev==m::no_events(),_=>false}}')
    output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_compose(b: Seq<u8>,h: crate::model::Ihdr)',"""
assert old in s;s=s.replace(old,new,1)
old="               f'    ensures m::{model}(b,m::strict())==m::Outcome::Error(Error::{frozen_error},m::no_events()),',"
new="               f'    ensures {conclusion},',"
assert old in s;s=s.replace(old,new,1)
p.write_text(s)
p=r/'tools/generate_m3_projection_witnesses.py';s=p.read_text()
old="    parser.add_argument('--names', nargs='+')";new=old+"\n    parser.add_argument('--split-budget', action='store_true')";assert old in s;s=s.replace(old,new,1)
old="        symbolic_header = header.replace(f'bytes_{ident}()', 'b')"
new="""        split_helpers = []
        if frames and args.split_budget:
            split_helpers, split_composition = certificates.split_apng_steps(data,ident,facts,True,None)
        symbolic_header = header.replace(f'bytes_{ident}()', 'b')"""
assert old in s;s=s.replace(old,new,1)
old="        lines += ['#[verifier::spinoff_prover]',f'proof fn symbolic_{ident}(b: Seq<u8>)',"
new="""        if split_helpers:
            lines += split_helpers
            composition = [x.replace(f'bytes_{ident}()', 'b') for x in split_composition]
        lines += ['#[verifier::spinoff_prover]',f'proof fn symbolic_{ident}(b: Seq<u8>)',"""
assert old in s;s=s.replace(old,new,1)
p.write_text(s)
