#!/usr/bin/env python3
"""Compare default-limit timing diagnostics without treating them as proof credit."""
from collections import defaultdict
import hashlib
import json
from pathlib import Path

ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
WORK=Path(__file__).resolve().parent
new=json.loads((ROOT/'runs/0470.json').read_text())
old=json.loads((WORK/'timing-m4-result.json').read_text())
assert new['errors']==0 and new['verified']==3362 and new['source_unchanged']
assert old['timing_report']['verification-results']['errors']==0
assert old['timing_report']['verification-results']['verified']==3346
assert new['command']==old['command'] and not new['accepted'] and not old['accepted']
assert new['effective_primary_driver_args']==old['effective_primary_driver_args']
assert not new['solver_limits_changed'] and not old['solver_limits_changed']

def phases(d):
    t=d['timing_report']['times-ms']
    return {'wall_seconds':t['total']/1000,'estimated_cpu_seconds':t['estimated-cpu-time']/1000,
            'rust_seconds':t['rust']['total']/1000,'vir_seconds':t['verification']['vir']['total']/1000,
            'combined_verify_seconds':t['total-verify']/1000,
            'combined_smt_seconds':t['smt']['total']/1000,'combined_air_seconds':t['air']['total']/1000}

def functions(d):
    out=defaultdict(lambda:{'smt_us':0,'rlimit':0,'entries':0})
    for module in d['timing_report']['times-ms']['smt']['smt-run-module-times']:
        for f in module.get('function-breakdown',[]):
            r=out[f['function']];r['smt_us']+=f['time-micros'];r['rlimit']+=f['rlimit'];r['entries']+=1
    return out

def modules(d):
    out=defaultdict(lambda:{'verify_ms':0,'smt_ms':0,'air_ms':0})
    t=d['timing_report']['times-ms']
    for row in t['total-verify-module-times']:out[row['module']]['verify_ms']+=row['time']
    for row in t['smt']['smt-run-module-times']:out[row['module']]['smt_ms']+=row['time']
    for row in t['air']['module-times']:out[row['module']]['air_ms']+=row['time']
    return out

before,after=functions(old),functions(new)
rows=[]
for name in sorted(set(before)|set(after)):
    b=before.get(name,{'smt_us':0,'rlimit':0,'entries':0});a=after.get(name,{'smt_us':0,'rlimit':0,'entries':0})
    module=name.split('::')[1] if name.startswith('png_verus::') else None
    file='src/'+module+'.rs' if module else None
    same=None
    if file and (WORK/'m4-source'/file).exists() and (WORK/'m5-timing-source'/file).exists():
        same=(WORK/'m4-source'/file).read_bytes()==(WORK/'m5-timing-source'/file).read_bytes()
    rows.append({'function':name,'before_smt_seconds':b['smt_us']/1e6,'after_smt_seconds':a['smt_us']/1e6,
                 'smt_delta_seconds':(a['smt_us']-b['smt_us'])/1e6,
                 'before_rlimit':b['rlimit'],'after_rlimit':a['rlimit'],
                 'rlimit_delta':a['rlimit']-b['rlimit'],'source_file':file,'source_file_byte_identical':same})
bmod,amod=modules(old),modules(new);module_rows=[]
for module in sorted(set(bmod)|set(amod)):
    b=bmod.get(module,{'verify_ms':0,'smt_ms':0,'air_ms':0});a=amod.get(module,{'verify_ms':0,'smt_ms':0,'air_ms':0})
    module_rows.append({'module':module,'before_combined_verify_seconds':b['verify_ms']/1000,
        'after_combined_verify_seconds':a['verify_ms']/1000,'verify_delta_seconds':(a['verify_ms']-b['verify_ms'])/1000,
        'before_smt_seconds':b['smt_ms']/1000,'after_smt_seconds':a['smt_ms']/1000,
        'smt_delta_seconds':(a['smt_ms']-b['smt_ms'])/1000})
new_names=[r for r in rows if r['function'] not in before]
result={'kind':'M5 verification-cost diagnostic; no checkpoint credit','baseline_run':old['run'],'current_run':new['run'],
        'baseline_revision':old['revision'],'current_revision':'d8a3d5c98c1ae0d891ef1fec6a49a7465f0fb652 plus timing-only launcher instrumentation',
        'same_primary_command_and_driver_flags':True,'default_solver_limits':True,'full_no_cheating_both':True,
        'same_vstd_and_inflate_pins':old['bindings_unchanged'] and old['vstd_pins_unchanged'] and new['bindings_unchanged'] and new['vstd_pins_unchanged'],
        'before_phases':phases(old),'after_phases':phases(new),
        'new_function_combined_smt_seconds':sum(r['after_smt_seconds'] for r in new_names),
        'module_deltas':sorted(module_rows,key=lambda r:r['verify_delta_seconds'],reverse=True),
        'largest_smt_increases':sorted(rows,key=lambda r:r['smt_delta_seconds'],reverse=True)[:40],
        'most_expensive_current_functions':sorted(rows,key=lambda r:r['after_smt_seconds'],reverse=True)[:40],
        'all_function_comparisons':rows,
        'qualification':'Function times are SMT query totals, not end-to-end function times. Combined times sum workers and can exceed wall time. The immutable scratch baseline and current timed snapshot exclude subsequent T4 edits. No source or solver-limit restructuring is credited here.',
        'accepted_checkpoint':False}
(WORK/'timing-comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k not in ['all_function_comparisons','most_expensive_current_functions','largest_smt_increases','module_deltas']},indent=2))
print('Largest module increases:')
for r in result['module_deltas'][:8]:print(r)
print('Largest function SMT increases:')
for r in result['largest_smt_increases'][:12]:print(r)
