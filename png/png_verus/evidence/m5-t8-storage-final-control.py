#!/usr/bin/env python3
"""Check identical accepted source, existing cache pins and restored T8 scratches."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
WORK=Path('/home/ws/m5-proof-work/0826ec8-20261122')
BASE='0826ec8f4606bbff184f058eae4c0abba8c12105'
p=argparse.ArgumentParser();p.add_argument('--host',type=int,required=True);p.add_argument('--i686',type=int,required=True);a=p.parse_args()
spec=importlib.util.spec_from_file_location('final_t8_direct',ROOT/'tools/direct_verify.py');direct=importlib.util.module_from_spec(spec);spec.loader.exec_module(direct)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
paths=sorted((ROOT/'src').rglob('*.rs'))+sorted((ROOT/'tools').glob('*.py'))+[ROOT/x for x in ['Cargo.toml','Cargo.lock','verify.sh']]
actual={str(f.relative_to(ROOT)):sha(f) for f in paths}
for n,target in [(a.host,'host'),(a.i686,'i686')]:
    d=json.loads((ROOT/'runs'/f'{n:04d}.json').read_text())
    assert d['accepted'] and d['errors']==0 and d['source_sha256']==actual and not d['verify_modules'] and not d['timing_json']
    assert direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json',target,d['command'])==d['bindings_before_after']
    assert direct.pins()==d['vstd_pins_before_after']
repo=ROOT.parents[1]
subprocess.run(['git','-C',str(repo),'diff','--exit-code',BASE,'--','png/THEOREMS.md','png/ERRATA.md',':(glob)png/REVIEW-*.md','png/spec-frozen','png/limits-frozen','png/policy-frozen','png/png_verus/QUESTIONS.md'],check=True,timeout=30)
subprocess.run(['git','-C',str(repo),'diff','--check'],check=True,timeout=30)
assert subprocess.check_output(['git','-C',str(repo),'rev-parse','HEAD'],text=True).strip()==BASE
ps=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True,timeout=10)
active=[line for line in ps.splitlines()[1:] if len(line.split(None,2))>1 and line.split(None,2)[1] in ['rust_verify','verus','z3']]
assert not active,active
restored=[]
for f in sorted(WORK.glob('t8-base-*-result.json')):
    d=json.loads(f.read_text());case=Path(d['source_root'])
    assert {p:sha(case/p) for p in d['canonical_source_sha256']}==d['canonical_source_sha256']
    restored.append(d['run'])
# Run 483 stopped in the front end before the old development reporter could
# write JSON. Its finally block restored lib.rs; verify every file directly
# against the selected-source archive, except for that deliberate restoration.
import tarfile
case=WORK/'t8-base-03'
with tarfile.open(WORK/'t8-base-03-source.tar.gz') as archive:
    for m in archive.getmembers():
        if m.isfile() and m.name!='src/lib.rs':
            assert sha(case/m.name)==hashlib.sha256(archive.extractfile(m).read()).hexdigest()
assert sha(case/'src/lib.rs')==sha(ROOT/'src/lib.rs')
print(json.dumps({'kind':'T8 storage/protocol post-checkpoint control; no new verifier credit','base_commit':BASE,'full_runs':[a.host,a.i686],'source_inventory_count':len(actual),'source_inventory_matches_both_full_runs':True,'host_and_i686_bindings_unchanged':True,'vstd_hashes_and_mtimes_unchanged':True,'restored_development_runs':restored,'frontend_only_run_483_scratch_restored':True,'protected_files_unchanged':True,'whitespace_clean':True,'active_verifier_or_solver_processes':active,'accepted_checkpoint':False},indent=2))
