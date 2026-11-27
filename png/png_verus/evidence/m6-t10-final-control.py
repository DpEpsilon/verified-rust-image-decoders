#!/usr/bin/env python3
"""Bind T10 full no-cheating runs and native controls to the same final sources."""
import importlib.util,json,re,subprocess
from pathlib import Path
W=Path(__file__).resolve().parent;R=Path('/home/ws/projects/verified-decoders/png/png_verus')
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def check():
    source_path=W/'source_control.py'
    if not source_path.exists():source_path=R/'evidence/m6-t10-source-control.py'
    control=load('t10_source_control',source_path);report=control.sources();actual=report['source_sha256']
    direct=load('t10_direct',R/'tools/direct_verify.py')
    archive=(W/'review-full-runs.json').exists()
    packet=None if archive else json.loads((R/'evidence/m6-t10-packet.json').read_text())
    full=json.loads((W/'review-full-runs.json').read_text()) if archive else packet['full_runs']
    assert [row['target'] for row in full]==['host','i686']
    counts=[];walls=[]
    runlog={int(line.split('\t')[0]):line.split('\t') for line in (R/'RUNLOG.tsv').read_text().splitlines() if line.split('\t')[0].isdigit()}
    for row in full:
        record=json.loads((R/'runs'/f'{row["run"]:04d}.json').read_text())
        assert record['accepted'] and record['errors']==0 and record['source_sha256']==actual
        assert not record['verify_modules'] and not record['timing_json'] and not record['solver_limits_changed']
        assert direct.check_bindings(R/'evidence/direct-verify-bindings.json',row['target'],record['command'])==record['bindings_before_after']
        assert direct.pins()==record['vstd_pins_before_after'];counts.append(record['verified'])
        wall=float(runlog[row['run']][3].removesuffix('s'));assert wall<3480;walls.append(wall)
    assert counts[0]==counts[1]>7329
    if archive:
        runtime=json.loads((W/'review-final-runtime.json').read_text());public=json.loads((W/'domain-public-errors-result.json').read_text())
    else:
        records={row['kind']:json.loads((R/row['record']['file']).read_text()) for row in packet['gates']}
        runtime=records['runtime'];public=records['public-errors']
    for row in [runtime,public]:assert row['source_sha256']==actual and row['source_unchanged'] and row['vstd_pins_unchanged']
    assert (runtime['pass'],runtime['total'],runtime['fail'],runtime['skip'])==(846,848,0,2)
    assert (public['pass'],public['fail'],public['skip'])==(72,0,0)
    active=[line for line in subprocess.check_output(['ps','-eo','comm,args'],text=True).splitlines() if line.split(None,1)[0] in ['rust_verify','verus','z3']];assert not active
    return dict(kind='M6 T10 final control',passed=True,**report,full_runs=full,verified=counts[0],goal_delta=counts[0]-7329,
        wall_seconds=dict(zip(['host','i686'],walls)),process_timeout_seconds=3600,host_warning_seconds=2880,mandatory_stop_seconds=3480,
        default_solver_budget=30000000,module_scoped_acceptance=False,runtime_run=runtime['run'],public_errors_run=public['run'],whitespace=control.whitespace())
if __name__=='__main__':print(json.dumps(check(),indent=2))
