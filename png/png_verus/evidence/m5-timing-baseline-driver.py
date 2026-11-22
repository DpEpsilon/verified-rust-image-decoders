#!/usr/bin/env python3
"""Profile the immutable pre-M5 implementation/model/witness source in scratch.

Only diagnostic timing flags are added to the accepted host invocation. This
run earns no proof credit and builds no Cargo/vstd artifacts.
"""
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess

ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
WORK=Path(__file__).resolve().parent
CASE=WORK/'m4-source'
REV='cafce7b134dfbf73febb74ddccc6e01a0b2f4b7c'
PRIMARY=' __VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,d):p.write_text(json.dumps(d,indent=2)+'\n')

def prepare():
    repo=ROOT.parents[1]
    names=subprocess.check_output(['git','-C',str(repo),'ls-tree','-r','--name-only',REV,'--',
        'png/png_verus/src','png/png_verus/tools','png/png_verus/Cargo.toml',
        'png/png_verus/Cargo.lock','png/png_verus/verify.sh'],text=True).splitlines()
    assert not CASE.exists()
    baseline={}
    for path in names:
        rel=Path(path).relative_to('png/png_verus')
        dst=CASE/rel;dst.parent.mkdir(parents=True,exist_ok=True)
        dst.write_bytes(subprocess.check_output(['git','-C',str(repo),'show',REV+':'+path]))
        baseline[str(rel)]=sha(dst)
    # Cargo.lock is sometimes an ignored replay input rather than a git blob.
    if 'Cargo.lock' not in baseline:
        dst=CASE/'Cargo.lock';dst.write_bytes((ROOT/'Cargo.lock').read_bytes());baseline['Cargo.lock']=sha(dst)
    record={'revision':REV,'source_sha256':baseline,'Cargo_lock_qualification':'The accepted replay lockfile is copied only if absent from git; manifests are checked separately.'}
    write(WORK/'m4-source-inventory.json',record)
    return record

def main():
    baseline=json.loads((WORK/'m4-source-inventory.json').read_text()) if CASE.exists() else prepare()
    lock=Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    spec=importlib.util.spec_from_file_location('diagnostic_direct',ROOT/'tools/direct_verify.py');direct=importlib.util.module_from_spec(spec);spec.loader.exec_module(direct)
    template=json.loads((ROOT/'evidence/m4-interlace-host-0449.json').read_text())
    command=template['command'];bindings=direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json','host',command);pins=direct.pins()
    env={k:v for k,v in os.environ.items() if not k.lstrip().startswith('__VERUS_DRIVER_') and k!='RUST_MIN_STACK'}
    env.update(template['recorded_driver_environment'])
    captured=json.loads((ROOT/'runs/cargo-primary-environment-capture.json').read_text())['environment']
    for k,v in captured.items():
        if k.startswith('CARGO_') and k not in ['CARGO_MAKEFLAGS','CARGO_BUILD_TARGET']:env[k]=v
        if k in ['LD_LIBRARY_PATH','RUSTUP_TOOLCHAIN']:env[k]=v
    env.update(template['effective_cargo_context']);env.pop('CARGO_BUILD_TARGET',None)
    for flag in ['--time','--time-expanded','--output-json']:env[PRIMARY]+='__VERUS_DRIVER_ARGS_SEP__'+flag
    assert '--no-cheating' in env[PRIMARY] and '--verify-' not in env[PRIMARY]
    env.update(PATH='/opt/cargo/bin:/opt/verus:'+env.get('PATH',''),RUST_MIN_STACK='268435456',PYTHONDONTWRITEBYTECODE='1')
    wrapper=['python3','-I','-c','import json,os,sys;os.chdir(sys.argv[1]);a=json.loads(sys.argv[2]);os.execvpe(a[0],a,os.environ)',str(CASE),json.dumps(command)]
    result=subprocess.run(['python3','-I',str(ROOT/'tools/run_logged.py'),'--timeout','3600','--label',
        'M5 timing diagnostic: immutable pre-M5 cafce7b full source in scratch; --no-cheating/default limits; no checkpoint credit','--',*wrapper],cwd=ROOT,env=env,stdout=subprocess.PIPE,text=True,timeout=3620)
    print(result.stdout,flush=True)
    run=int(re.findall(r'^\[run\] (\d+), exit',result.stdout,re.M)[-1]);log=(ROOT/'runs'/f'{run:04d}.log').read_text()
    decoder=json.JSONDecoder();reports=[]
    for m in re.finditer(r'^\{',log,re.M):
        try:d,_=decoder.raw_decode(log[m.start():])
        except ValueError:continue
        if isinstance(d,dict) and 'verification-results' in d:reports.append(d)
    assert len(reports)==1
    same_source=all(sha(CASE/p)==h for p,h in baseline['source_sha256'].items())
    same_bindings=bindings==direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json','host',command)
    same_pins=pins==direct.pins()
    record={'kind':'M5 timing diagnostic of pre-M5 scratch source','run':run,'revision':REV,
        'source_sha256':baseline['source_sha256'],'scratch_source':str(CASE),'source_unchanged':same_source,
        'command':command,'wrapper_command':wrapper,'effective_primary_driver_args':env[PRIMARY],
        'bindings_unchanged':same_bindings,'vstd_pins_unchanged':same_pins,'accepted':False,
        'solver_limits_changed':False,'wall_timeout_seconds':3600,'RUST_MIN_STACK':268435456,
        'timing_report':reports[0],'log_sha256':hashlib.sha256(log.encode()).hexdigest()}
    write(ROOT/'runs'/f'{run:04d}.json',record);write(WORK/'timing-m4-result.json',record)
    assert result.returncode==0 and reports[0]['verification-results']['errors']==0 and same_source and same_bindings and same_pins
    print('Pre-M5 scratch timing completed; no checkpoint credit.',flush=True)

if __name__=='__main__':main()
