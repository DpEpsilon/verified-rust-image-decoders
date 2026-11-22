#!/usr/bin/env python3
"""Final unchanged-input, protected-text and pinned-cache M5 control."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess

ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
BASE='cafce7b134dfbf73febb74ddccc6e01a0b2f4b7c'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
spec=importlib.util.spec_from_file_location('m5_direct',ROOT/'tools/direct_verify.py');direct=importlib.util.module_from_spec(spec);spec.loader.exec_module(direct)
paths=sorted((ROOT/'src').rglob('*.rs'))+sorted((ROOT/'tools').glob('*.py'))+[ROOT/x for x in ['Cargo.toml','Cargo.lock','verify.sh']]
actual={str(p.relative_to(ROOT)):sha(p) for p in paths}
for n,target in [(465,'host'),(466,'i686')]:
    d=json.loads((ROOT/'runs'/f'{n:04d}.json').read_text())
    assert d['accepted'] and d['errors']==0 and d['source_sha256']==actual
    assert direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json',target,d['command'])==d['bindings_before_after']
    assert direct.pins()==d['vstd_pins_before_after']
repo=ROOT.parents[1]
subprocess.run(['git','-C',str(repo),'diff','--exit-code',BASE,'--','png/THEOREMS.md','png/ERRATA.md',':(glob)png/REVIEW-*.md','png/spec-frozen','png/limits-frozen','png/policy-frozen','png/png_verus/QUESTIONS.md'],check=True,timeout=30)
subprocess.run(['git','-C',str(repo),'diff','--check'],check=True,timeout=30)
assert subprocess.check_output(['git','-C',str(repo),'rev-parse','HEAD'],text=True).strip()==BASE
processes=subprocess.check_output(['ps','-eo','pid,comm,args'],text=True,timeout=10)
active=[x for x in processes.splitlines()[1:] if len(x.split(None,2))>1 and x.split(None,2)[1] in ['rust_verify','verus','z3']]
assert not active,active
print(json.dumps({'kind':'M5 post-checkpoint source/import/cache/protected control','base_commit':BASE,'full_runs':[465,466],'source_inventory_count':len(actual),'source_inventory_matches_both_full_runs':True,'host_and_i686_bindings_unchanged':True,'source_and_copied_vstd_hashes_and_mtimes_unchanged':True,'protected_files_unchanged':True,'whitespace_clean':True,'active_verifier_or_solver_processes':active,'new_verifier_credit':False},indent=2))
