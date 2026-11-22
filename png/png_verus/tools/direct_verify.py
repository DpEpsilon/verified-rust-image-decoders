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
BINDINGS = ROOT / 'evidence/direct-verify-bindings.json'


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


def check_bindings(pin_file, target, command):
    """Reject replay when Cargo inputs, toolchain or imported inflate files drift."""
    expected = json.loads(pin_file.read_text())
    assert expected['recorded_primary_command_run'][target] == (332 if target == 'host' else 333)
    assert expected['command_sha256'][target] == hashlib.sha256(
        json.dumps(command, separators=(',', ':')).encode()).hexdigest(), 'Recorded command changed'
    manifests = {
        'Cargo.toml': ROOT / 'Cargo.toml',
        'Cargo.lock': ROOT / 'Cargo.lock',
        '../inflate_verus/Cargo.toml': ROOT.parent / 'inflate_verus/Cargo.toml',
    }
    actual = {name: digest(path) if path.exists() else None for name, path in manifests.items()}
    assert actual == expected['manifests'], 'Cargo manifests/lockfile changed'
    version_env = dict(os.environ, PATH='/opt/cargo/bin:/opt/verus:' + os.environ.get('PATH', ''))
    versions = {
        'verus': subprocess.check_output([command[0], '--version'], text=True, timeout=10,
                                         env=version_env).strip(),
        'rustc': subprocess.check_output([command[1], '--version'], text=True, timeout=10,
                                         env=version_env).strip(),
    }
    # Keep verify_m3.py's explicit Verus version gate, in addition to exact pins.
    if 'Version: 0.2026.11.02.8bf68ee\n' not in versions['verus']:
        raise SystemExit('Refusing an unpinned Verus version')
    assert versions == expected['versions'], 'Verifier toolchain changed'
    externs = [command[i + 1].split('=', 1)[1] for i, value in enumerate(command[:-1])
               if value == '--extern' and command[i + 1].startswith('inflate_verus=')]
    assert len(externs) == 1, 'Expected exactly one inflate --extern'
    imported = Path(externs[0])
    assert imported.suffix in ('.rmeta', '.rlib')
    actual_artifacts = {}
    for suffix in ('.rmeta', '.rlib', '.vir'):
        path = imported.with_suffix(suffix)
        if path.exists():
            actual_artifacts[str(path)] = digest(path)
    assert str(imported) in actual_artifacts and str(imported.with_suffix('.vir')) in actual_artifacts
    assert actual_artifacts == expected['inflate_artifacts'][target], 'Imported inflate artifacts changed'
    return {'manifests': actual, 'versions': versions, 'inflate_artifacts': actual_artifacts}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', choices=['host', 'i686'], required=True)
    parser.add_argument('--stack-mib', type=int, required=True)
    parser.add_argument('--expect-count', type=int)
    parser.add_argument('--verify-module', action='append', default=[],
                        help='Development only: verify selected modules; earns no checkpoint credit')
    parser.add_argument('--timing-json', action='store_true',
                        help='Development only: collect expanded timing JSON; earns no checkpoint credit')
    parser.add_argument('--check-bindings', action='store_true',
                        help='Check all replay bindings without invoking the verifier')
    parser.add_argument('--pin-file', type=Path, default=BINDINGS,
                        help='Binding file to check (default: committed evidence pin)')
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
    bindings = check_bindings(args.pin_file, args.target, command)
    dependencies = pins()
    if args.check_bindings:
        print(f'[direct bindings] {args.target}: manifests, toolchain, inflate imports and vstd pins match')
        return 0
    env_line = next(line for line in text.splitlines() if line.startswith('cd ') and ' && ' in line)
    assignments = shlex.split(env_line.split(' && ', 1)[1])
    recorded_env = {}
    for value in assignments:
        if '=' not in value or value.startswith('/'):
            break
        key, val = value.split('=', 1)
        # Command's display is not shell-round-trippable: Cargo preserves a
        # leading space in these six driver variable names. Kernel environment
        # capture 338 confirms the exact spelling; it is part of the key.
        if key.startswith(('__VERUS_DRIVER_ARGS', '__VERUS_DRIVER_IS_BUILTIN')):
            key = ' ' + key
        recorded_env[key] = val
    primary = ' __VERUS_DRIVER_ARGS_FOR_png_verus-0.1.0-d81f5b1329ad'
    assert primary in recorded_env
    assert '--no-cheating' in recorded_env[primary]
    assert '--num-threads__VERUS_DRIVER_ARGS_SEP__2' in recorded_env[primary]
    assert 'rlimit' not in recorded_env[primary]
    env = {key: value for key, value in os.environ.items()
           if not key.lstrip().startswith('__VERUS_DRIVER_') and key != 'RUST_MIN_STACK'}
    env.update(recorded_env)
    if args.verify_module:
        # Verus forbids module selection with --no-cheating. This diagnostic
        # mode drops that flag, never earns credit, and never invokes Cargo.
        env[primary] = env[primary].replace('__VERUS_DRIVER_ARGS_SEP__--no-cheating', '')
    for module in args.verify_module:
        assert re.fullmatch(r'[A-Za-z_][A-Za-z_0-9:]*(?:\*)?', module), module
        env[primary] += '__VERUS_DRIVER_ARGS_SEP__--verify-module__VERUS_DRIVER_ARGS_SEP__' + module
    if args.timing_json:
        for flag in ('--time', '--time-expanded', '--output-json'):
            env[primary] += '__VERUS_DRIVER_ARGS_SEP__' + flag
    # Preserve stable Cargo process context without inheriting dead jobserver
    # descriptors or invoking any Cargo process/build script.
    captured = json.loads((ROOT / 'runs/cargo-primary-environment-capture.json').read_text())
    captured_env = captured['environment']
    assert captured_env[primary] == recorded_env[primary]
    for key, value in captured_env.items():
        if key.startswith('CARGO_') and key not in ('CARGO_MAKEFLAGS', 'CARGO_BUILD_TARGET'):
            env[key] = value
    for key in ('LD_LIBRARY_PATH', 'RUSTUP_TOOLCHAIN'):
        if key in captured_env:
            env[key] = captured_env[key]
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
    scope = ('development timing JSON (NO CREDIT)' if args.timing_json else
             'development modules ' + ','.join(args.verify_module) + ' (NO CREDIT)'
             if args.verify_module else 'full PNG')
    label = (f'Direct {scope} {args.target}: RUST_MIN_STACK={env["RUST_MIN_STACK"]} bytes; '
             f'RLIMIT_STACK={resource.getrlimit(resource.RLIMIT_STACK)}; '
             f'wall_timeout=3600s; exact run-{baseline} primary command; no Cargo; default solver limits')
    result = subprocess.run(['python3', '-I', str(ROOT / 'tools/run_logged.py'),
                             '--timeout', '3600', '--label', label, '--', *command],
                            cwd=ROOT, env=env, stdout=subprocess.PIPE, text=True, timeout=3620)
    print(result.stdout, end='')
    run = int(re.findall(r'^\[run\] (\d+), exit', result.stdout, re.M)[-1])
    log = (ROOT / 'runs' / f'{run:04d}.log').read_text()
    summary_pattern = (r'^verification results:: (\d+) verified, (\d+) errors'
                       r' \(partial verification with `--verify-\*`\)$'
                       if args.verify_module else
                       r'^verification results:: (\d+) verified, (\d+) errors$')
    counts = re.findall(summary_pattern, log, re.M)
    timing_report = None
    if args.timing_json:
        decoder = json.JSONDecoder()
        reports = []
        for start in re.finditer(r'^\{', log, re.M):
            try:
                value, _ = decoder.raw_decode(log[start.start():])
            except ValueError:
                continue
            if isinstance(value, dict) and 'verification-results' in value:
                reports.append(value)
        if len(reports) == 1:
            timing_report = reports[0]
            result_counts = timing_report['verification-results']
            counts = [(str(result_counts['verified']), str(result_counts['errors']))]
    same_source = before == helper['inventory']()
    same_bindings = bindings == check_bindings(args.pin_file, args.target, command)
    same_pins = dependencies == pins()
    verified, errors = (map(int, counts[-1]) if counts else (0, None))
    succeeded = (result.returncode == 0 and len(counts) == 1 and verified > 0 and errors == 0
                 and same_source and same_bindings and same_pins
                 and (args.expect_count is None or verified == args.expect_count))
    accepted = succeeded and not args.verify_module and not args.timing_json
    record = {'kind': 'direct-development-timing' if args.timing_json else
                     'direct-development-modules' if args.verify_module else 'direct-full-crate-checkpoint',
              'timing_json': args.timing_json, 'timing_report': timing_report,
              'verify_modules': args.verify_module, 'effective_primary_driver_args': env[primary],
              'run': run, 'target': args.target,
              'recorded_primary_command_run': baseline, 'command': command,
              'recorded_driver_environment': recorded_env,
              'cargo_environment_capture_sha256': digest(ROOT / 'runs/cargo-primary-environment-capture.json'),
              'effective_cargo_context': {k: v for k, v in env.items() if k.startswith('CARGO_')},
              'RUST_MIN_STACK': env['RUST_MIN_STACK'],
              'RLIMIT_STACK': resource.getrlimit(resource.RLIMIT_STACK),
              'source_sha256': before, 'source_unchanged': same_source,
              'binding_pin_sha256': digest(args.pin_file), 'bindings_before_after': bindings,
              'bindings_unchanged': same_bindings,
              'vstd_pins_before_after': dependencies, 'vstd_pins_unchanged': same_pins,
              'cargo_invoked': False, 'solver_limits_changed': False, 'wall_timeout_seconds': 3600,
              'verified': verified, 'errors': errors, 'expected_count': args.expect_count,
              'accepted': accepted, 'log_sha256': hashlib.sha256(log.encode()).hexdigest()}
    (ROOT / 'runs' / f'{run:04d}.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'[direct {"development" if args.verify_module or args.timing_json else "checkpoint"}] run {run}: '
          f'{verified} verified, {errors} errors; accepted={accepted}')
    return 0 if succeeded else (result.returncode or 1)


if __name__ == '__main__':
    sys.exit(main())
