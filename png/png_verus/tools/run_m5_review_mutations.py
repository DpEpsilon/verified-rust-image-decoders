#!/usr/bin/env python3
"""M5 review-fix verifier mutations against canonical production contracts.

Follow M2's accepted base-pass mutation precedent: retain all implementation,
independent-model and common proof modules; omit eight independent generated
witness leaves only in the scratch lib.rs. Keep --no-cheating/default limits.
Neither the base control nor a mutant is a whole-crate milestone checkpoint.
"""
import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]
LEAVES = ['witness_m3_frozen', 'witness_m3_walk_frozen', 'witness_m3_projection',
          'witness_m4_complete', 'witness_m4_filter_projection', 'witness_m4_interlace_projection', 'witness_m5_still', 'witness_m5_frames']
CASES = {24: ('canonical-trns16-high-byte', 'src/image.rs', '    key[0]==pixel[0] && key[1]==pixel[1] && key[2]==pixel[2]', '    if _depth==16 {key[0]/256==pixel[0]/256 && key[1]/256==pixel[1]/256 && key[2]/256==pixel[2]/256}else{key[0]==pixel[0] && key[1]==pixel[1] && key[2]==pixel[2]}', 'trns_samples_equal'), 25: ('sample16-strip', 'src/image.rs', '        ((v * 510 + 65535) / 131070) as u16', '        (v >> 8) as u16', 'scale'), 26: ('previous-frame-zero-none', 'src/frame_state.rs', '        let effective_dispose=if index==0 && control.dispose==2 {1} else {control.dispose};', '        let effective_dispose=if index==0 && control.dispose==2 {0} else {control.dispose};', 'select'), 27: ('save-rectangle-restore-canvas', 'src/png.rs', '                if let Some(s)=&self.saved {copy_rect(&mut self.canvas,s,w,px,py,pw,ph);}', '                if let Some(s)=&self.saved {copy_rect(&mut self.canvas,s,w,0,0,w,h);}', 'dispose_and_save'), 28: ('over-truncate-colour', 'src/image.rs', '        o[i] = ((2 * num + a) / (2 * a)) as u8;', '        o[i] = (num / a) as u8;', 'over'), 29: ('saved-reserve-every-previous', 'src/png.rs', '        if !needed || self.saved.is_some() {return Ok(());}', '        if !needed {return Ok(());}', 'ensure_saved'), 30: ('saved-on-still-path', 'src/png.rs', '        Err(e)=>Err(e),Ok((raw,out))=>Ok((doc,raw,out)),', '        Err(e)=>Err(e),Ok((raw,out))=>{\n            if !frames {\n                let extra=try_zeroed(AllocSite::Saved,out_bytes,Tracked(trace.log.borrow_mut()));\n                trace.record(AllocSite::Saved,out_bytes);\n                match extra {Err(o)=>return Err(oom(o)),Ok(_)=>{}}\n            }\n            Ok((doc,raw,out))\n        },', 'prepare_image'), 32: ('continue-after-saved-failure', 'src/png.rs', '                return Err(e);\n                }}\n                proof {self.allocation_context();assert(crate::alloc_guard::all_ok(self.log_view()));}', '                let _=e;\n                }}\n                proof {self.allocation_context();assert(crate::alloc_guard::all_ok(self.log_view()));}', 'selected_logged')}

PRIMARY = ' __VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def digest(rows):
    return hashlib.sha256(json.dumps(rows, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def load_direct():
    spec = importlib.util.spec_from_file_location('pinned_direct_verifier', ROOT/'tools/direct_verify.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def inputs():
    paths = sorted((ROOT/'src').rglob('*.rs')) + sorted((ROOT/'tools').glob('*.py'))
    paths += [ROOT/x for x in ('Cargo.toml', 'Cargo.lock', 'verify.sh')]
    return {str(p.relative_to(ROOT)): sha(p) for p in sorted(paths)}


def copied_inputs(case, baseline):
    return {p: sha(case/p) for p in baseline}


def json_write(path, value):
    path.write_text(json.dumps(value, indent=2)+'\n')


def select_base(text):
    for name in LEAVES:
        old = '#[cfg(verus_only)]\nmod '+name+';'
        assert text.count(old) == 1, name
        text = text.replace(old, '// M2-style mutation base omits independent generated leaf '+name+'.')
    return text


def semantic_errors(log, case, expected_function):
    errors = []
    if expected_function is None:
        return errors
    text = (case/CASES[next(n for n,v in CASES.items() if v[4]==expected_function)][1]).read_text()
    filename = CASES[next(n for n,v in CASES.items() if v[4]==expected_function)][1]
    functions = [(text[:m.start()].count('\n')+1, m.group(1)) for m in
                 re.finditer(r'^\s*(?:pub(?:\(crate\))?\s+)?(?:proof\s+)?fn\s+(\w+)(?:<[^\n]*>)?\s*\(', text, re.M)]
    starts = [n for n, name in functions if name == expected_function]
    assert len(starts) == 1, expected_function
    lo = starts[0]
    hi = min([n for n, _ in functions if n > lo], default=len(text.splitlines())+1)
    accepted = ['postcondition not satisfied', 'assertion failed', 'invariant not satisfied',
                'precondition not satisfied', 'possible arithmetic underflow/overflow']
    for line in log.splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        message = item.get('message', '')
        if item.get('level') != 'error' or not any(x in message for x in accepted):
            continue
        spans = item.get('spans', [])
        if not any(s.get('file_name') == filename and lo <= s['line_start'] < hi for s in spans):
            continue
        rendered = re.sub(r'\x1b\[[0-9;]*m', '', item.get('rendered') or message)
        errors.append({'failed_function': expected_function, 'message': message,
                       'diagnostic': rendered, 'spans': spans})
    return errors


def run_one(number, parent, baseline, revision, template, direct):
    control = number == 0
    selector, filename, before, after, failed_function = (
        ('unmutated-review-base-control', None, None, None, None) if control else CASES[number])
    case = parent/selector
    assert not case.exists(), case
    case.mkdir(parents=True)
    for rel in baseline:
        dst = case/rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT/rel, dst)
    assert copied_inputs(case, baseline) == baseline
    original_lib = (case/'src/lib.rs').read_text()
    selected_lib = select_base(original_lib)
    (case/'src/lib.rs').write_text(selected_lib)
    prepared=copied_inputs(case,baseline)
    if not control:
        original = (case/filename).read_text()
        assert original.count(before) == 1, (selector, original.count(before))
        (case/filename).write_text(original.replace(before, after))
    selected = copied_inputs(case, baseline)
    differences = sorted(p for p in baseline if baseline[p] != selected[p])
    assert differences == (['src/lib.rs'] if control else sorted(['src/lib.rs', filename]))
    record = {'kind': 'M2-style-mutation-base-control' if control else 'implementation-proof-mutation',
              'number': number, 'selector': selector, 'base_commit': revision,'checkpoint_path':template['source_file'],'checkpoint_record_sha256':template['source_record_sha256'],
              'scratch_root': str(case), 'base_pass': 'mutation-base-no-generated-leaves',
              'base_pass_omitted_generated_leaves': LEAVES,
              'base_pass_qualification': 'All implementation, independent-model and common proof modules retained; no --verify-* filters; not a milestone checkpoint.',
              'scratch_preparation': None,'prepared_unmutated_source_sha256':prepared,
              'canonical_source_sha256': baseline, 'canonical_source_tree_sha256': digest(baseline),
              'compiled_source_sha256': selected, 'compiled_source_tree_sha256': digest(selected),
              'changed_files_relative_to_canonical': differences,
              'mutation': None if control else {'file': filename, 'before': before, 'after': after},
              'expected_failed_function': failed_function,
              'accepted_checkpoint': False, 'full_no_cheating': True,
              'solver_limits_changed': False, 'wall_timeout_seconds': 3600, 'stack_bytes': 268435456}
    json_write(case/'mutation-selector.json', record)
    archive = parent/f'm5-mutation-{selector}-source.tar.gz'
    assert not archive.exists(), archive
    with tarfile.open(archive, 'w:gz') as out:
        for rel in selected:
            out.add(case/rel, arcname=rel, recursive=False)
        out.add(case/'mutation-selector.json', arcname='mutation-selector.json', recursive=False)
    record['source_archive'] = str(archive)
    record['source_archive_sha256'] = sha(archive)
    record['archive_required_for_replay'] = False
    record['reconstruction'] = 'Use the coordinator commit matching canonical_source_sha256; base_commit identifies its parent while these fixes are uncommitted. Alternatively use the retained source archive. Apply only the recorded implementation mutation and generated-leaf omissions; check compiled_source_sha256 and replay the pinned command/environment.'
    record['selector_definitions'] = str(Path(__file__).resolve().relative_to(ROOT))
    record['selector_definitions_sha256'] = sha(Path(__file__))
    command = template['command']
    bindings = direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json', 'host', command)
    pins = direct.pins()
    assert '--no-cheating' in template['recorded_driver_environment'][PRIMARY]
    assert '--verify-' not in template['recorded_driver_environment'][PRIMARY]
    env = {k: v for k, v in os.environ.items() if not k.lstrip().startswith('__VERUS_DRIVER_') and k != 'RUST_MIN_STACK'}
    env.update(template['recorded_driver_environment'])
    captured = json.loads((ROOT/'runs/cargo-primary-environment-capture.json').read_text())['environment']
    for key, value in captured.items():
        if key.startswith('CARGO_') and key not in ('CARGO_MAKEFLAGS', 'CARGO_BUILD_TARGET'):
            env[key] = value
        if key in ('LD_LIBRARY_PATH', 'RUSTUP_TOOLCHAIN'):
            env[key] = value
    env.update(template['effective_cargo_context'])
    env.pop('CARGO_BUILD_TARGET', None)
    # Keep the recorded Cargo identity for driver-argument dispatch. Rustc's
    # relative source argument resolves in the scratch cwd; Cargo is not run.
    env.update(PATH='/opt/cargo/bin:/opt/verus:'+env.get('PATH', ''),
               RUST_MIN_STACK='268435456', CARGO_BUILD_JOBS='1', PYTHONDONTWRITEBYTECODE='1')
    wrapper = ['python3', '-I', '-c',
               'import json,os,sys;os.chdir(sys.argv[1]);a=json.loads(sys.argv[2]);os.execvpe(a[0],a,os.environ)',
               str(case), json.dumps(command)]
    label = f'M5 mutation {number} {selector}: M2-style base; full --no-cheating, default solver limits, wall_timeout=3600s; scratch source'
    try:
        result = subprocess.run(['python3', '-I', str(ROOT/'tools/run_logged.py'), '--timeout', '3600',
                                 '--label', label, '--', *wrapper], cwd=ROOT, env=env,
                                stdout=subprocess.PIPE, text=True, timeout=3620)
        print(result.stdout, end='', flush=True)
        run = int(re.findall(r'^\[run\] (\d+), exit', result.stdout, re.M)[-1])
        log = (ROOT/'runs'/f'{run:04d}.log').read_text()
        counts = re.findall(r'^verification results:: (\d+) verified, (\d+) errors$', log, re.M)
        verified, errors = map(int, counts[-1]) if counts else (0, None)
        semantic = semantic_errors(log, case, failed_function)
        record.update(run=run, command=command, wrapper_command=wrapper,
                      recorded_driver_environment=template['recorded_driver_environment'],
                      cargo_context={k: v for k, v in env.items() if k.startswith('CARGO_')},
                      binding_pin_sha256=sha(ROOT/'evidence/direct-verify-bindings.json'),
                      bindings_before_after=bindings, vstd_pins_before_after=pins,
                      exit_code=result.returncode, verified=verified, errors=errors,
                      semantic_diagnostics=semantic,
                      selected_source_unchanged=selected == copied_inputs(case, baseline),
                      canonical_source_unchanged=baseline == inputs(),
                      bindings_unchanged=bindings == direct.check_bindings(ROOT/'evidence/direct-verify-bindings.json', 'host', command),
                      vstd_pins_unchanged=pins == direct.pins(),
                      log_sha256=hashlib.sha256(log.encode()).hexdigest())
        good_pins = all(record[k] for k in ('selected_source_unchanged', 'canonical_source_unchanged', 'bindings_unchanged', 'vstd_pins_unchanged'))
        record['accepted_mutation_control'] = control and result.returncode == 0 and len(counts) == 1 and verified > 0 and errors == 0 and good_pins
        record['nonsemantic_failure_present'] = any(x in log for x in ['resource limit (rlimit) exceeded','panicked at','internal error:']) or result.returncode in [124,137]
        record['verifier_killed'] = not record['nonsemantic_failure_present'] and not control and result.returncode != 0 and len(counts) == 1 and errors is not None and errors > 0 and bool(semantic) and good_pins
    finally:
        (case/'src/lib.rs').write_text(original_lib)
        if not control:
            restore_file=filename
            shutil.copy2(ROOT/restore_file, case/restore_file)
        record['scratch_restored_to_canonical'] = copied_inputs(case, baseline) == baseline
        record['canonical_source_unchanged_after_restore'] = inputs() == baseline
        json_write(parent/f'{selector}-result.json', record)
    assert record['scratch_restored_to_canonical'] and record['canonical_source_unchanged_after_restore']
    if 'run' in record:
        json_write(ROOT/'runs'/f'{record["run"]:04d}.json', record)
        evidence_base = ROOT/'evidence'/f'm5-mutation-{number:02d}-{record["run"]:04d}'
        json_write(evidence_base.with_suffix('.json'), record)
        shutil.copyfile(ROOT/'runs'/f'{record["run"]:04d}.log', evidence_base.with_suffix('.log'))
    assert record.get('accepted_mutation_control') if control else record.get('verifier_killed'), record
    print(f'M5 {selector}: '+('unmutated control verified' if control else f'verifier killed in {failed_function}')+f'; run {record["run"]}; scratch restored; canonical hashes unchanged.', flush=True)
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--scratch-root', type=Path, required=True)
    parser.add_argument('--case', choices=['control']+[str(n) for n in CASES], default='control')
    parser.add_argument('--checkpoint',type=Path,required=True)
    args = parser.parse_args()
    parent = args.scratch_root.resolve()
    assert parent.is_relative_to(Path('/home/ws').resolve()) and not parent.is_relative_to(ROOT.parents[1]), parent
    parent.mkdir(parents=True, exist_ok=True)
    lock = Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a')
    fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    direct = load_direct()
    baseline = inputs()
    revision = subprocess.check_output(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], text=True).strip()
    template = json.loads(args.checkpoint.read_text())
    template['source_file']=str(args.checkpoint.resolve())
    template['source_record_sha256']=sha(args.checkpoint)
    assert template['accepted'] and template['errors'] == 0 and template['verified'] > 0
    assert template['target']=='host'
    for path, value in template['source_sha256'].items():
        assert sha(ROOT/path) == value, path
    canonical_record = parent/'canonical-source-before.json'
    if canonical_record.exists():
        assert json.loads(canonical_record.read_text())['source_sha256'] == baseline
    else:
        json_write(canonical_record, {'base_commit': revision,'checkpoint_path':template['source_file'],'checkpoint_record_sha256':template['source_record_sha256'], 'source_sha256': baseline,
                                     'source_tree_sha256': digest(baseline), 'selector_outside_proof_inventory': True})
    run_one(0 if args.case == 'control' else int(args.case), parent, baseline, revision, template, direct)


if __name__ == '__main__':
    main()
