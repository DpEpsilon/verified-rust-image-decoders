#!/usr/bin/env python3
"""Audit M4 retained mutation archives, diagnostics, restoration and pinned inputs.

This checks evidence; it is neither an independent ghost-code review nor a
verifier invocation. It never extracts archives or invokes Cargo.
"""
import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = {19: ('paeth-pb-strict', 'paeth', 'postcondition not satisfied'),
            20: ('average-round-up', 'average', 'postcondition not satisfied'),
            21: ('rgb8-sub-bpp-one', 'unfilter', 'assertion failed'),
            22: ('adam7-empty-pass-filter-byte', 'raw_size', 'invariant not satisfied at end of loop body'),
            23: ('adam7-swap-passes-4-5-one-based', 'pass', 'assertion failed')}
LEAVES = ['witness_m3_frozen', 'witness_m3_walk_frozen', 'witness_m3_projection',
          'witness_m4_complete', 'witness_m4_filter_projection', 'witness_m4_interlace_projection']
PRIMARY = ' __VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def digest(d):
    return hashlib.sha256(json.dumps(d, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def load_module(name, p):
    spec = importlib.util.spec_from_file_location(name, p)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def audit_record(row, baseline, template, runlog):
    assert sha(ROOT/row['metadata_file']) == row['metadata_sha256'], 'metadata hash'
    d = json.loads((ROOT/row['metadata_file']).read_text())
    n = d['number']
    assert row['selector'] == d['selector']
    for k in ['run', 'mutation', 'verified', 'errors', 'exit_code', 'command', 'wrapper_command',
              'recorded_driver_environment', 'base_pass', 'base_pass_omitted_generated_leaves',
              'full_no_cheating', 'solver_limits_changed', 'accepted_checkpoint',
              'scratch_restored_to_canonical', 'canonical_source_unchanged_after_restore']:
        assert row[k] == d[k], k
    assert d['base_commit'] == baseline['base_commit'] == 'b8c6629bd2e60134537e4f3339140b8ff207b8bb'
    rev = row['source_revision']
    for k in ['base_commit', 'source_archive', 'source_archive_sha256', 'canonical_source_tree_sha256',
              'canonical_source_sha256', 'compiled_source_tree_sha256', 'compiled_source_sha256']:
        assert rev[k] == d[k], k
    assert d['canonical_source_sha256'] == baseline['source_sha256']
    assert digest(d['canonical_source_sha256']) == d['canonical_source_tree_sha256']
    assert digest(d['compiled_source_sha256']) == d['compiled_source_tree_sha256']
    assert d['base_pass_omitted_generated_leaves'] == LEAVES
    assert d['full_no_cheating'] and not d['solver_limits_changed'] and not d['accepted_checkpoint']
    assert d['command'] == template['command']
    assert d['recorded_driver_environment'] == template['recorded_driver_environment']
    flags = d['recorded_driver_environment'][PRIMARY].split('__VERUS_DRIVER_ARGS_SEP__')
    assert '--no-cheating' in flags
    assert not any(x.startswith(('--verify-', '--rlimit', '--smt-option')) for x in flags)
    assert d['wall_timeout_seconds'] == 3600 and d['stack_bytes'] == 268435456
    for k in ['selected_source_unchanged', 'canonical_source_unchanged', 'bindings_unchanged',
              'vstd_pins_unchanged', 'scratch_restored_to_canonical', 'canonical_source_unchanged_after_restore']:
        assert row[k] and d[k], k
    assert sha(ROOT/rev['source_archive']) == rev['source_archive_sha256'], 'archive hash'
    with tarfile.open(ROOT/rev['source_archive']) as archive:
        members = archive.getmembers()
        expected_names = set(d['compiled_source_sha256']) | {'mutation-selector.json'}
        assert len(members) == len(expected_names)
        assert {m.name for m in members} == expected_names and all(m.isfile() for m in members)
        data = {m.name: archive.extractfile(m).read() for m in members}
    selected = json.loads(data.pop('mutation-selector.json'))
    assert all(d[k] == v for k, v in selected.items()), 'archived selector'
    assert {p: hashlib.sha256(b).hexdigest() for p, b in data.items()} == d['compiled_source_sha256']
    for p in data:
        expected = (ROOT/p).read_bytes()
        if p == 'src/lib.rs':
            for leaf in LEAVES:
                before = f'#[cfg(verus_only)]\nmod {leaf};'.encode()
                assert expected.count(before) == 1
                expected = expected.replace(before, f'// M2-style mutation base omits independent generated leaf {leaf}.'.encode())
        if n and p == d['mutation']['file']:
            before, after = (d['mutation'][k].encode() for k in ['before', 'after'])
            assert expected.count(before) == 1
            expected = expected.replace(before, after)
        assert data[p] == expected, p
    scratch = Path(d['scratch_root'])
    assert {p: sha(scratch/p) for p in baseline['source_sha256']} == baseline['source_sha256']
    log = (ROOT/row['log']).read_text()
    assert sha(ROOT/row['log']) == row['log_sha256'] == d['log_sha256']
    assert row['verification_results'] in log
    assert re.findall(r'^verification results:: (\d+) verified, (\d+) errors$', log, re.M) == [(str(d['verified']), str(d['errors']))]
    line = [line for line in runlog.splitlines() if line.startswith(str(d['run'])+'\t')]
    assert len(line) == 1 and f'\texit={d["exit_code"]}\t' in line[0]
    assert row['verification_results'] in line[0]
    assert not any(s in log for s in ['resource limit exceeded', 'timed out', 'panicked at', 'internal compiler error'])
    diagnostics = []
    for line in log.splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if item.get('level') == 'error' and not item.get('message', '').startswith('aborting due to'):
            diagnostics.append(item)
    if n:
        selector, function, message = EXPECTED[n]
        assert d['selector'] == selector and row['failed_function'] == 'image::'+function
        assert row['diagnostic_message'] == message and d['expected_failed_function'] == function
        assert d['exit_code'] == 1 and d['verified'] == 603 and d['errors'] == 1
        assert d['verifier_killed'] and row['verifier_killed'] and not d['accepted_mutation_control']
        assert len(diagnostics) == 1 and diagnostics[0]['message'] == message
        item = diagnostics[0]
        rendered = re.sub(r'\x1b\[[0-9;]*m', '', item['rendered'])
        assert row['failed_proof'] == rendered == d['semantic_diagnostics'][0]['diagnostic']
        assert row['diagnostic_spans'] == item['spans']
        text = data['src/image.rs'].decode()
        functions = [(text[:m.start()].count('\n')+1, m.group(1)) for m in re.finditer(
            r'^\s*(?:pub(?:\(crate\))?\s+)?(?:proof\s+)?fn\s+(\w+)(?:<[^\n]*>)?\s*\(', text, re.M)]
        lo, = [n for n, name in functions if name == function]
        hi = min([n for n, _ in functions if n > lo], default=len(text.splitlines())+1)
        assert any(s['file_name'] == 'src/image.rs' and lo <= s['line_start'] < hi for s in item['spans'])
    else:
        assert not diagnostics and d['exit_code'] == 0 and d['verified'] == 604 and d['errors'] == 0
        assert d['accepted_mutation_control'] and not d['verifier_killed']
    return {'run': d['run'], 'selector': d['selector'], 'archive_files_checked': len(data),
            'compiled_source_tree_sha256': d['compiled_source_tree_sha256'],
            'restored_scratch_matches_canonical': True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    runner = load_module('mutation_runner', ROOT/'tools/run_m4_mutations.py')
    index = json.loads((ROOT/'evidence/mutation-index.json').read_text())
    baseline = json.loads(Path('/home/ws/m4-proof-mutations/b8c6629-20261122/canonical-source-before.json').read_text())
    template = json.loads((ROOT/'evidence/m4-interlace-host-0449.json').read_text())
    runlog = (ROOT/'RUNLOG.tsv').read_text()
    assert runner.inputs() == baseline['source_sha256'] and len(runner.inputs()) == 70
    assert all(sha(ROOT/p) == h for p, h in template['source_sha256'].items())
    assert len(template['source_sha256']) == 69
    assert sha(ROOT/index['selector_definitions']) == index['selector_definitions_sha256']
    assert sha(ROOT.parent/'THEOREMS.md') == index['requirement_source_sha256']
    assert [r['number'] for r in index['requirements']] == list(range(1, 41))
    frozen_requirements = {int(n): text for n, text in re.findall(r'^(\d+)\. (.+)$', (ROOT.parent/'THEOREMS.md').read_text(), re.M)}
    mutations = []
    for r in index['requirements']:
        assert r['requirement'] == frozen_requirements[r['number']]
        assert r['status'] == ('verifier-killed' if r['number'] in EXPECTED else 'not-credited-here')
        assert len(r['mutants']) == (1 if r['number'] in EXPECTED else 0)
        mutations.extend(r['mutants'])
    assert len(mutations) == index['verifier_kill_total_here'] == 5 and not index['additional_mutations']
    rows = [index['passing_base_control']] + mutations
    checked = [audit_record(r, baseline, template, runlog) for r in rows]
    negative_controls = {
        'wrong metadata hash': ('metadata_sha256', '0'*64),
        'wrong archive hash': ('source_revision.source_archive_sha256', '0'*64),
        'wrong log hash': ('log_sha256', '0'*64),
        'wrong function': ('failed_function', 'image::average'),
        'wrong diagnostic': ('failed_proof', 'error: runtime failure'),
        'missing no-cheating': ('full_no_cheating', False),
        'false restoration': ('scratch_restored_to_canonical', False),
        'passing mutant summary': ('verification_results', 'verification results:: 604 verified, 0 errors'),
        'altered generated-leaf selector': ('base_pass_omitted_generated_leaves', LEAVES+['filter_model'])}
    for label, (key, value) in negative_controls.items():
        candidate = copy.deepcopy(mutations[0])
        parent = candidate
        path = key.split('.')
        for k in path[:-1]:
            parent = parent[k]
        parent[path[-1]] = value
        try:
            audit_record(candidate, baseline, template, runlog)
        except AssertionError:
            continue
        raise AssertionError('negative control accepted: '+label)
    direct = runner.load_direct()
    assert direct.pins() == json.loads((ROOT/rows[0]['metadata_file']).read_text())['vstd_pins_before_after']
    bindings = {}
    for target, filename in [('host', 'm4-interlace-host-0449.json'), ('i686', 'm4-interlace-i686-0450.json')]:
        d = json.loads((ROOT/'evidence'/filename).read_text())
        bindings[target] = direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json', target, d['command'])
    assert bindings['host'] == json.loads((ROOT/rows[0]['metadata_file']).read_text())['bindings_before_after']
    repo = ROOT.parents[1]
    protected = ['png/THEOREMS.md', 'png/ERRATA.md', ':(glob)png/REVIEW-*.md',
                 'png/spec-frozen', 'png/limits-frozen', 'png/policy-frozen', 'png/png_verus/QUESTIONS.md']
    subprocess.run(['git', '-C', str(repo), 'diff', '--exit-code', '605bebd', '--', *protected], check=True, timeout=30)
    subprocess.run(['git', '-C', str(repo), 'diff', '--check'], check=True, timeout=30)
    subprocess.run(['git', '-C', str(repo), 'diff', '--exit-code', baseline['base_commit'], '--', 'png/png_verus/src'], check=True, timeout=30)
    processes = subprocess.check_output(['ps', '-eo', 'pid,comm,args'], text=True, timeout=10)
    active = []
    for line in processes.splitlines()[1:]:
        parts = line.split(None, 2)
        if len(parts) > 1 and parts[1] in ['verus', 'rust_verify', 'z3']:
            active.append(line)
    assert not active, active
    result = {'kind': 'M4 mutation evidence audit; not a verifier run or ghost-code review',
              'mutation_index_sha256': sha(ROOT/'evidence/mutation-index.json'),
              'audit_script_sha256': sha(Path(__file__)), 'base_commit': baseline['base_commit'],
              'canonical_source_tree_sha256': digest(baseline['source_sha256']),
              'canonical_source_inventory_count': 70, 'full_checkpoint_inputs_unchanged': 69,
              'canonical_source_matches_before': True, 'canonical_rust_source_matches_base_commit': True,
              'archives_and_restored_scratch_copies': checked,
              'negative_controls_rejected': list(negative_controls),
              'host_and_i686_bindings_checked': True, 'vstd_hashes_and_mtimes_unchanged': True,
              'protected_owner_baseline': '605bebd', 'protected_files_unchanged': True,
              'whitespace_clean': True, 'active_verifier_or_solver_processes': active,
              'accepted_checkpoint': False, 'sign_off_claimed': False}
    if args.output:
        args.output.write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
