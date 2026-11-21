"""Apply bounded proof decomposition after the first budget run has ended."""
from pathlib import Path
R=Path('/home/ws/projects/verified-decoders/png/png_verus')
p=R/'tools/generate_m3_walk_witnesses.py';s=p.read_text()
old="""                   f'    ensures m::step(b,m::strict(),{str(frames_entry).lower()},false,{prefix}_state_{stage}(h))=={result},',
                   '{',*step_hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);']"""
new="""                   f'    ensures m::step(b,m::strict(),{str(frames_entry).lower()},false,{prefix}_state_{stage}(h))=={result},',
                   *([f'        {prefix}_state_{stage}(h).pos<=b.len(),'] if ident.startswith('apng_bad_pixel_budget') else []),
                   *([f'        {prefix}_state_{stage}(h).doc.ihdr==h,',
                      f'        {prefix}_state_{stage}(h).doc.idat==seq![crate::walk_model::Range {{offset:61nat,len:2nat}}],']
                     if done and ident.startswith('apng_bad_pixel_budget') else []),
                   '{',*step_hides,'    reveal(m::step);reveal(m::ordered);reveal(m::own);']"""
assert s.count(old)==1;s=s.replace(old,new)
old="""    output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_compose(b: Seq<u8>,h: crate::model::Ihdr)',
               '    requires '+',\\n        '.join(req)+',',
               f'    ensures {conclusion},',
               '{',*hides,'    hide(m::drive);']
    output += [f'    {prefix}_step_{n}(b,h);' for n in range(stage+1)]
    output += [f'    m::drive_step(b,m::strict(),{f},false,{prefix}_state_{n}(h));' for n in reversed(range(stage+1))]
    output += ['}','']
"""
new="""    if ident.startswith('apng_bad_pixel_budget'):
        # Keep every full walk checked, but join only eight transition equations
        # per query. Opaque state descriptors avoid expanding irrelevant frame
        # vectors; checked step contracts export the required position bounds.
        thin_hides=[line.replace('hide(m::initial_info);','') for line in hides]
        for start in reversed(range(0,stage+1,8)):
            stop=min(start+8,stage+1)
            drive=f'm::drive(b,m::strict(),{f},false,{prefix}_state_{start}(h))'
            suffix_goal=conclusion.replace(f'm::{model}(b,m::strict())',drive)
            output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_suffix_{start}(b: Seq<u8>,h: crate::model::Ihdr)',
                       '    requires '+',\\n        '.join(req)+',',f'    ensures {suffix_goal},',
                       '{',*thin_hides,'    hide(m::drive);hide(m::step);',
                       '    '+''.join(f'hide({prefix}_state_{n});' for n in range(start,min(stop,stage)+1))]
            output += [f'    {prefix}_step_{n}(b,h);' for n in range(start,stop)]
            if stop<=stage:output += [f'    {prefix}_suffix_{stop}(b,h);']
            output += [f'    m::drive_step(b,m::strict(),{f},false,{prefix}_state_{n}(h));'
                       for n in reversed(range(start,stop))]
            output += ['}','']
        output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_compose(b: Seq<u8>,h: crate::model::Ihdr)',
                   '    requires '+',\\n        '.join(req)+',',f'    ensures {conclusion},',
                   '{',*thin_hides,'    hide(m::drive);',f'    {prefix}_suffix_0(b,h);','}','']
    else:
        output += ['#[verifier::spinoff_prover]',f'proof fn {prefix}_compose(b: Seq<u8>,h: crate::model::Ihdr)',
                   '    requires '+',\\n        '.join(req)+',',f'    ensures {conclusion},',
                   '{',*hides,'    hide(m::drive);']
        output += [f'    {prefix}_step_{n}(b,h);' for n in range(stage+1)]
        output += [f'    m::drive_step(b,m::strict(),{f},false,{prefix}_state_{n}(h));' for n in reversed(range(stage+1))]
        output += ['}','']
"""
assert s.count(old)==1;s=s.replace(old,new)
old="['apng_bad_seq_gap','apng_bad_seq_gap_still','apng_bad_fctl_no_fdat_then_fctl']"
new="['apng_bad_seq_gap','apng_bad_seq_gap_still','apng_bad_fctl_no_fdat_then_fctl','apng_bad_pixel_budget','apng_bad_pixel_budget_exactly_2_30_then_zlib']"
assert s.count(old)==2;s=s.replace(old,new)
old="""            *facts,
            *([] if symbolic or split"""
new="""            *facts,
            *([f'        assert(bytes_{ident}().len()=={len(data)}) by (compute);',f'        hide(bytes_{ident});']
              if ident=='apng_bad_pixel_budget' else []),
            *([] if symbolic or split"""
assert s.count(old)==1;s=s.replace(old,new)
p.write_text(s)
p=R/'tools/generate_m3_projection_witnesses.py';s=p.read_text()
old="""        if frames and args.split_budget:
            split_helpers, split_composition = certificates.split_apng_steps(data,ident,facts,True,None)
"""
new="""        if frames and args.split_budget:
            split_helpers, split_composition = certificates.split_apng_steps(data,ident,facts,True,None)
            # The composition interface needs checked transitions, not every
            # raw payload word. Keep the short default-stream fact separately.
            requirements=[x for x in requirements if
                'crate::critical_model::read(' in x or 'crate::walk_model::classify(' in x
                or 'crate::apng_model::' in x or 'u8 & 32u8' in x
                or 'crate::model::be32(b,16)' in x or 'crate::model::be32(b,20)' in x
                or f'b.subrange({data_offset},{data_offset+2})' in x]
"""
assert s.count(old)==1;s=s.replace(old,new)
old="""                  *facts,f'    symbolic_{ident}(bytes_{ident}());','}',"""
new="""                  *([f'    literal_facts_{ident}();',
                     f'    assert(bytes_{ident}().subrange({data_offset},{data_offset+2})==seq![120u8,1u8]) by (compute);',
                     f'    assert(bytes_{ident}().len()=={len(data)}) by (compute);',f'    hide(bytes_{ident});']
                    if frames and args.split_budget else facts),
                  f'    symbolic_{ident}(bytes_{ident}());','}',"""
assert s.count(old)==1;s=s.replace(old,new)
p.write_text(s)
print('Applied eight-step opaque-state suffix composition and isolated literal facts for both budgets.')
