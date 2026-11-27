#!/usr/bin/env python3
"""Bind T10 source additions, all 62 frozen names and unchanged accepted code."""
import hashlib,importlib.util,json,re,subprocess
from pathlib import Path
R=Path('/home/ws/projects/verified-decoders/png/png_verus');W=Path(__file__).resolve().parent;E=R/'evidence';REPO=R.parents[1]
BASE='3694dfd42f9eb288809b7b0c4fd7f76cdff35e83'
NEW={'src/precedence_model.rs','src/precedence_walk.rs','src/precedence.rs','src/precedence_public.rs'}
CHANGED={'src/lib.rs','src/document_model.rs','src/png.rs'}
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(REPO),*args],text=True)
def inventory():
    paths=sorted((R/'src').rglob('*.rs'))+sorted((R/'tools').glob('*.py'))+[R/x for x in ['Cargo.toml','Cargo.lock','verify.sh']]
    return {str(p.relative_to(R)):sha(p) for p in paths}
def closed_names(actual,expected):
    assert len(actual)==len(set(actual))==62
    assert set(actual)==set(expected)
def sources():
    assert git('rev-parse','HEAD').strip()==BASE
    spec=(R.parent/'spec-frozen/SPEC.md').read_text()
    line=next(x for x in spec.splitlines() if x.startswith('The closed, case-sensitive error set (62 names)'))
    expected=re.findall(r'`([A-Za-z]+)`',line.split('. `IccProfileTooLarge`')[0])
    source=(R/'src/png.rs').read_text();body=re.search(r'pub enum Error \{(.*?)\n\}',source,re.S).group(1)
    actual=re.findall(r'^\s*([A-Za-z]+),\s*$',body,re.M);closed_names(actual,expected)
    rejected=[]
    for name,candidate in [('duplicate',actual[:-1]+[actual[0]]),('omitted',actual[:-1]),('unexpected',actual[:-1]+['NotFrozen'])]:
        try:closed_names(candidate,expected)
        except AssertionError:rejected.append(name)
        else:raise AssertionError(('invalid error inventory accepted',name))
    tracked=set(git('diff','--name-only',BASE,'--','png/png_verus/src','png/png_verus/tools').splitlines())
    assert tracked=={'png/png_verus/'+p for p in CHANGED},tracked
    fresh=set(git('ls-files','--others','--exclude-standard','--','png/png_verus/src','png/png_verus/tools').splitlines())
    assert fresh=={'png/png_verus/'+p for p in NEW},fresh
    for name in git('ls-files','png/png_verus/src','png/png_verus/tools').splitlines():
        rel=name.removeprefix('png/png_verus/');old=git('show',BASE+':'+name);now=(REPO/name).read_text()
        if rel=='src/lib.rs':
            for module in ['precedence_model','precedence_walk','precedence','precedence_public']:
                marker='#[cfg(verus_only)]\nmod '+module+';\n';assert now.count(marker)==1;now=now.replace(marker,'')
        elif rel=='src/document_model.rs':
            marker='// T10 range observations; the transition definitions above remain unchanged.\n';assert now.count(marker)==1
            now=now.split(marker)[0]+'} // verus!\n'
        elif rel=='src/png.rs':
            start=now.index('    /// T10 observes the established history without executing another walk.\n')
            end=now.index("    /// Work absence follows from the actual site's observation, not from the",start)
            now=now[:start]+now[end:]
        assert now==old,('accepted source changed',rel)
    protected=['png/THEOREMS.md','png/ERRATA.md',':(glob)png/REVIEW-*.md','png/spec-frozen','png/limits-frozen','png/policy-frozen','png/png_verus/QUESTIONS.md','png/inflate_verus','png/png_verus/tests','png/png_verus/src/alloc_guard']
    assert not git('diff','--name-only',BASE,'--',*protected).strip()
    for rel in NEW|CHANGED:
        data=(R/rel).read_text()
        if rel in CHANGED:
            if rel=='src/document_model.rs':data=data.split('// T10 range observations;')[1]
            elif rel=='src/png.rs':data=data.split('    /// T10 observes')[1].split('    /// Work absence follows')[0]
            else:data='\n'.join(x for x in data.splitlines() if 'precedence' in x)
        assert not re.search(r'\b(?:assume|admit|assume_specification)\s*\(|\bexternal_body\b|\brlimit\b',data),rel
    public=(R/'src/precedence_public.rs').read_text()
    calls=['png::try_decode_still_logged(b)','png::try_decode_still16_logged(b)','Frames::try_new_logged(b)','frames.try_next()','png::try_icc_profile_logged(b)']
    assert all(public.count(call)==1 for call in calls)
    assert public.count('Ghost(e): Ghost<Error>')==5
    assert 'loc::data_selected' in (R/'src/precedence.rs').read_text()
    assert 'icc::exact_second_pass' in (R/'src/precedence.rs').read_text()
    old_sums=git('show',BASE+':png/png_verus/evidence/SHA256SUMS');rows={}
    for line in old_sums.splitlines():
        digest,name=line.split(None,1);name=name.strip();assert name not in rows;assert sha(E/name)==digest;rows[name]=digest
    assert len(rows)==1393
    return dict(base_commit=BASE,source_sha256=inventory(),frozen_errors=expected,error_count=len(expected),negative_inventory_controls_rejected=rejected,
        unchanged_accepted_sources=True,native_algorithms_and_contracts_unchanged=True,proof_only_defining_module_observers=True,historical_hashes_unchanged=len(rows),
        protected_files_unchanged=True,new_trust_declarations=False,actual_public_calls=calls)
def whitespace():
    spec=importlib.util.spec_from_file_location('old_whitespace',E/'m6-e6-m4-a7-whitespace-control.py');old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
    report=old.check_checkpoint(R,BASE)
    paths={R/p for p in NEW|CHANGED}|set(E.glob('m6-t10-*'))
    import gzip
    for p in paths:
        data=p.read_bytes()
        if p.name.endswith('.log.gz'):data=gzip.decompress(data)
        old.check_text(data,str(p.relative_to(R)))
    return dict(policy='unchanged accepted whitespace policy',new_files_checked=sorted(str(p.relative_to(R)) for p in paths),inherited=report)
if __name__=='__main__':print(json.dumps(dict(kind='M6 T10 source control',**sources(),whitespace=whitespace()),indent=2))
