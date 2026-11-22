#!/usr/bin/env python3
"""Cheap T8 development using M4's accepted scratch-only base selector.

All implementation, independent models, shared proof helpers and manual
witnesses stay enabled. No result earns checkpoint credit. Full canonical
host/i686 --no-cheating replays remain mandatory before a component report.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile

ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
WORK=Path(__file__).resolve().parent
PRIMARY=' __VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,d):p.write_text(json.dumps(d,indent=2)+'\n')
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('label');a=parser.parse_args()
    assert re.fullmatch('[a-z0-9-]+',a.label)
    runner=module('existing_m4_base_selector',ROOT/'tools/run_m4_mutations.py');direct=runner.load_direct()
    import fcntl
    lock=Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    before=runner.inputs();case=WORK/a.label;assert not case.exists();case.mkdir()
    for rel in before:
        dst=case/rel;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/rel,dst)
    original=(case/'src/lib.rs').read_text();(case/'src/lib.rs').write_text(runner.select_base(original))
    compiled=runner.copied_inputs(case,before);assert [p for p in before if before[p]!=compiled[p]]==['src/lib.rs']
    archive=WORK/f'{a.label}-source.tar.gz'
    with tarfile.open(archive,'w:gz') as out:
        for rel in compiled:out.add(case/rel,arcname=rel,recursive=False)
    template=json.loads((ROOT/'evidence/m5-arithmetic-host-0465.json').read_text());command=template['command']
    pins=direct.pins();bindings=direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json','host',command)
    env={k:v for k,v in os.environ.items() if not k.lstrip().startswith('__VERUS_DRIVER_') and k!='RUST_MIN_STACK'}
    env.update(template['recorded_driver_environment'])
    capture=json.loads((ROOT/'runs/cargo-primary-environment-capture.json').read_text())['environment']
    for k,v in capture.items():
        if k.startswith('CARGO_') and k not in ['CARGO_MAKEFLAGS','CARGO_BUILD_TARGET']:env[k]=v
        if k in ['LD_LIBRARY_PATH','RUSTUP_TOOLCHAIN']:env[k]=v
    env.update(template['effective_cargo_context']);env.pop('CARGO_BUILD_TARGET',None)
    for flag in ['--time','--time-expanded','--output-json']:env[PRIMARY]+='__VERUS_DRIVER_ARGS_SEP__'+flag
    assert '--no-cheating' in env[PRIMARY] and '--verify-' not in env[PRIMARY]
    env.update(PATH='/opt/cargo/bin:/opt/verus:'+env.get('PATH',''),RUST_MIN_STACK='268435456',PYTHONDONTWRITEBYTECODE='1')
    wrapper=['python3','-I','-c','import json,os,sys;os.chdir(sys.argv[1]);a=json.loads(sys.argv[2]);os.execvpe(a[0],a,os.environ)',str(case),json.dumps(command)]
    try:
        result=subprocess.run(['python3','-I',str(ROOT/'tools/run_logged.py'),'--timeout','3600','--label',
            'M5 T8 development '+a.label+': M4 scratch base, six generated leaves omitted; --no-cheating/default solver limits; NO CREDIT','--',*wrapper],cwd=ROOT,env=env,stdout=subprocess.PIPE,text=True,timeout=3620)
        print(result.stdout,flush=True)
        run=int(re.findall(r'^\[run\] (\d+), exit',result.stdout,re.M)[-1]);log=(ROOT/'runs'/f'{run:04d}.log').read_text()
        reports=[];decoder=json.JSONDecoder()
        for m in re.finditer(r'^\{',log,re.M):
            try:d,_=decoder.raw_decode(log[m.start():])
            except ValueError:continue
            if isinstance(d,dict) and 'verification-results' in d:reports.append(d)
        assert len(reports)==1
        stable=runner.inputs()==before and runner.copied_inputs(case,before)==compiled
        pin_same=pins==direct.pins();binding_same=bindings==direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json','host',command)
        counts=reports[0]['verification-results']
        counts.setdefault('verified',0);counts.setdefault('errors',0)
        counts.setdefault('encountered-error',True);counts.setdefault('encountered-vir-error',True)
        record={'kind':'T8 scratch-base development, no checkpoint credit','run':run,'label':a.label,
            'accepted':False,'full_no_cheating':True,'solver_limits_changed':False,'wall_timeout_seconds':3600,
            'omitted_generated_leaves':runner.LEAVES,'source_root':str(case),'canonical_source_sha256':before,
            'compiled_source_sha256':compiled,'source_archive':str(archive),'source_archive_sha256':sha(archive),
            'command':command,'wrapper_command':wrapper,'effective_primary_driver_args':env[PRIMARY],
            'canonical_and_compiled_source_unchanged':stable,'vstd_pins_unchanged':pin_same,'bindings_unchanged':binding_same,
            'verified':counts['verified'],'errors':counts['errors'],'timing_report':reports[0],
            'log_sha256':hashlib.sha256(log.encode()).hexdigest()}
    finally:
        (case/'src/lib.rs').write_text(original)
    record['scratch_source_restored_to_canonical']=runner.copied_inputs(case,before)==before
    write(ROOT/'runs'/f'{run:04d}.json',record);write(WORK/f'{a.label}-result.json',record)
    print('T8 development base:',run,counts,'restored:',record['scratch_source_restored_to_canonical'],flush=True)
    ok=(result.returncode==0 and counts['verified']>0 and counts['errors']==0 and not counts['encountered-error'] and not counts['encountered-vir-error'] and stable and pin_same and binding_same and record['scratch_source_restored_to_canonical'])
    return 0 if ok else 1

if __name__=='__main__':sys.exit(main())
