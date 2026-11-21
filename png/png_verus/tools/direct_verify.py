#!/usr/bin/env python3
"""Replay Cargo's complete primary PNG command without invoking Cargo.

Usage: python3 -I tools/direct_verify.py --target host --stack-mib 256 --expect-count 1452
The recorded host/i686 commands come from accepted runs 332/333. No solver
option is added or removed. RUST_MIN_STACK reaches only the direct process.
"""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import runpy
import shlex
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pins():
    rows = json.loads((ROOT / 'evidence/vstd-cache-copy.json').read_text())
    cross = json.loads((ROOT / 'evidence/i686-cache-copy-0317.json').read_text())
    rows += [x for x in cross['files']
             if '/i686-unknown-linux-gnu/debug/deps/' in x['source']
             and 'vstd-' in Path(x['source']).name]
    result = []
    for row in rows:
        source, copy = Path(row['source']), Path(row['copy'])
        assert digest(source) == row['sha256'] == digest(copy), source
        assert source.stat().st_mtime_ns == row['source_mtime_ns'], source
        assert not source.samefile(copy), copy
        for path in (source, copy):
            result.append({'path': str(path), 'sha256': digest(path),
                           'mtime_ns': path.stat().st_mtime_ns})
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', choices=['host', 'i686'], required=True)
    parser.add_argument('--stack-mib', type=int, required=True)
    parser.add_argument('--expect-count', type=int)
    args = parser.parse_args()
    assert 1 <= args.stack_mib <= 1024
    helper = runpy.run_path(str(ROOT / 'tools/verify_m3.py'))
    assert not helper['corpus_running'](), 'Recorded two-thread replay requires no corpus process'
    lock = Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a')
    fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    baseline = 332 if args.target == 'host' else 333
    text = (ROOT / 'runs' / f'{baseline:04d}.log').read_text()
    command_lines = [line.strip() for line in text.splitlines()
                     if 'Running `' in line and '--crate-name png_verus ' in line]
    assert len(command_lines) == 1
    command = shlex.split(command_lines[0].removeprefix('Running `').removesuffix('`'))
    assert command[:2] == ['/opt/verus/verus', '/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc']
    assert command[command.index('--crate-name') + 1] == 'png_verus'
    assert not any('verify-only' in value or 'rlimit' in value for value in command)
    assert ('--target' in command) == (args.target == 'i686')
    env_line = next(line for line in text.splitlines() if line.startswith('cd ') and ' && ' in line)
    assignments = shlex.split(env_line.split(' && ', 1)[1])
    recorded_env = {}
    for value in assignments:
        if '=' not in value or value.startswith('/'):
            break
        key, val = value.split('=', 1)
        recorded_env[key] = val
    primary = '__VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'
    assert primary in recorded_env
    assert '--no-cheating' in recorded_env[primary]
    assert '--num-threads__VERUS_DRIVER_ARGS_SEP__2' in recorded_env[primary]
    assert 'rlimit' not in recorded_env[primary]
    env = {key: value for key, value in os.environ.items()
           if not key.startswith('__VERUS_DRIVER_') and key != 'RUST_MIN_STACK'}
    env.update(recorded_env)
    env.update({'PATH': '/opt/cargo/bin:/opt/verus:' + env.get('PATH', ''),
                'CARGO_MANIFEST_DIR': str(ROOT.resolve()),
                'CARGO_MANIFEST_PATH': str(ROOT.resolve() / 'Cargo.toml'),
                'CARGO_PKG_NAME': 'png_verus', 'CARGO_CRATE_NAME': 'png_verus',
                'CARGO_PKG_VERSION': '0.1.0', 'CARGO_PKG_VERSION_MAJOR': '0',
                'CARGO_PKG_VERSION_MINOR': '1', 'CARGO_PKG_VERSION_PATCH': '0',
                'CARGO_PKG_VERSION_PRE': '', 'CARGO_PRIMARY_PACKAGE': '1',
                'CARGO_BUILD_JOBS': '1', 'RUST_MIN_STACK': str(args.stack_mib * 1024 * 1024)})
    if args.target == 'i686':
        env['CARGO_BUILD_TARGET'] = 'i686-unknown-linux-gnu'
    else:
        env.pop('CARGO_BUILD_TARGET', None)
    before = helper['inventory']()
    dependencies = pins()
    label = (f'Direct full PNG {args.target}: RUST_MIN_STACK={env["RUST_MIN_STACK"]} bytes; '
             f'RLIMIT_STACK={resource.getrlimit(resource.RLIMIT_STACK)}; '
             f'exact run-{baseline} primary command; no Cargo; default solver limits')
    result = subprocess.run(['python3', '-I', str(ROOT / 'tools/run_logged.py'),
                             '--timeout', '1500', '--label', label, '--', *command],
                            cwd=ROOT, env=env, stdout=subprocess.PIPE, text=True, timeout=1520)
    print(result.stdout, end='')
    run = int(re.findall(r'^\[run\] (\d+), exit', result.stdout, re.M)[-1])
    log = (ROOT / 'runs' / f'{run:04d}.log').read_text()
    counts = re.findall(r'^verification results:: (\d+) verified, (\d+) errors$', log, re.M)
    same_source = before == helper['inventory']()
    same_pins = dependencies == pins()
    verified, errors = (map(int, counts[-1]) if counts else (0, None))
    accepted = (result.returncode == 0 and len(counts) == 1 and verified > 0 and errors == 0
                and same_source and same_pins
                and (args.expect_count is None or verified == args.expect_count))
    record = {'kind': 'direct-full-crate-checkpoint', 'run': run, 'target': args.target,
              'recorded_primary_command_run': baseline, 'command': command,
              'recorded_driver_environment': recorded_env, 'RUST_MIN_STACK': env['RUST_MIN_STACK'],
              'RLIMIT_STACK': resource.getrlimit(resource.RLIMIT_STACK),
              'source_sha256': before, 'source_unchanged': same_source,
              'vstd_pins_before_after': dependencies, 'vstd_pins_unchanged': same_pins,
              'cargo_invoked': False, 'solver_limits_changed': False,
              'verified': verified, 'errors': errors, 'expected_count': args.expect_count,
              'accepted': accepted, 'log_sha256': hashlib.sha256(log.encode()).hexdigest()}
    (ROOT / 'runs' / f'{run:04d}.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'[direct checkpoint] run {run}: {verified} verified, {errors} errors; accepted={accepted}')
    return 0 if accepted else (result.returncode or 1)


if __name__ == '__main__':
    sys.exit(main())
