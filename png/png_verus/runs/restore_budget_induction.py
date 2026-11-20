#!/usr/bin/env python3
"""Preserve the failed attempt, then restore exact accepted proof inputs."""
from pathlib import Path
import hashlib,json,shutil
ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
base=json.loads((ROOT/'evidence/checkpoint-0318.json').read_text())['source_sha256']
for name,h in base.items():
    if name!='src/lib.rs':assert sha(ROOT/name)==h,name
for source,dest in [('tools/generate_m3_budget_inductive.py','runs/budget-induction-generator-0322.py'),('src/witness_m3_budget_inductive.rs','runs/budget-induction-candidate-0322.rs')]:
    p=ROOT/source;q=ROOT/dest
    assert p.is_file() and not q.exists(),str(q)
    p.rename(q)
p=ROOT/'src/lib.rs';s=p.read_text();registration='\n#[cfg(verus_only)]\nmod witness_m3_budget_inductive;\n'
assert s.count(registration)==1
p.write_text(s.replace(registration,''))
for name,h in base.items():assert sha(ROOT/name)==h,name
paths=sorted((ROOT/'src').rglob('*.rs'))+[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'verify.sh']+sorted((ROOT/'tools').glob('*.py'))
assert {str(p.relative_to(ROOT)):sha(p) for p in paths}==base
# Force one fresh PNG-only replay without touching any dependency artifact.
cache=ROOT/'target/debug/.fingerprint'
assert cache.resolve().is_relative_to((ROOT/'target').resolve())
removed=[]
for p in cache.glob('png_verus-*'):
    assert p.is_dir() and not p.is_symlink()
    shutil.rmtree(p);removed.append(str(p))
(ROOT/'runs/budget-induction-restoration.json').write_text(json.dumps({'source_inventory_exactly_restored_to':'checkpoint-0318.json','source_sha256':base,'preserved_generator':'runs/budget-induction-generator-0322.py','preserved_generated_candidate':'runs/budget-induction-candidate-0322.rs','png_only_fingerprint_directories_removed':removed,'dependencies_or_inflate_target_cleaned':False},indent=2)+'\n')
print('Restored all',len(base),'accepted proof-input hashes exactly; preserved attempt outside source; removed',len(removed),'PNG-only fingerprints for a fresh native check.')
