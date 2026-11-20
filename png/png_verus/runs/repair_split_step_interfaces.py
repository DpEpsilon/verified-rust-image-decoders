#!/usr/bin/env python3
"""Narrow checked split interfaces and expose sequence extensionality."""
from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/png/png_verus/tools/generate_m3_walk_witnesses.py')
s=p.read_text()
old='    transitions = []\n    for line in apng_facts(data, ident):'
new='''    # Raw payload words discharge local transition certificates in the literal
    # caller; composition needs only the resulting checked transitions.
    requirements = list(dict.fromkeys(x for x in requirements
        if 'crate::critical_model::read(' in x
        or 'crate::walk_model::classify(' in x
        or 'crate::apng_model::' in x
        or 'u8 & 32u8' in x))
    transitions = []
    for line in apng_facts(data, ident):'''
assert old in s
s=s.replace(old,new,1)
old="                   '{',*hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);','}']\n        if terminal_error:break"
new="""                   '{',*hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);']
        if not error_match:
            step=f'm::step(b,m::strict(),{str(frames_entry).lower()},false,{prefix}_state_{stage}(h))'
            output += [f'    let actual=match {step} {{m::Step::Next(next)=>next,_=>{prefix}_state_{stage}(h)}};',
                       f'    assert({step}==m::Step::Next(actual));',
                       f'    let expected={prefix}_state_{stage+1}(h);',
                       '    assert(actual.pos==expected.pos);',
                       '    assert(actual.phase==expected.phase);',
                       '    assert(actual.control==expected.control);',
                       '    assert(actual.doc.info==expected.doc.info);',
                       '    assert(actual.doc.frames =~= expected.doc.frames);',
                       '    assert(actual.doc.idat =~= expected.doc.idat);']
        output += ['}']
        if terminal_error:break"""
assert old in s
s=s.replace(old,new,1)
old="if name == 'container/non-consecutive-idat' or name.startswith('apng-bad/') else []"
new="if name in ['container/non-consecutive-idat','anc-bad/bkgd-index-3-of-3'] or name.startswith('apng-bad/') else []"
assert old in s
s=s.replace(old,new,1)
p.write_text(s)
