#!/usr/bin/env python3
"""Verify one complete PNG crate; bind the fresh positive count to source hashes."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def inventory():
    paths = sorted((ROOT / 'src').rglob('*.rs'))
    paths += [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', ROOT / 'verify.sh']
    paths += sorted((ROOT / 'tools').glob('*.py'))
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}


def main():
    before = inventory()
    versions = {
        'verus': subprocess.check_output(['verus', '--version'], text=True, timeout=10).strip(),
        'rustc': subprocess.check_output(['rustc', '--version'], text=True, timeout=10).strip(),
        'cargo_verus_binary_sha256': hashlib.sha256(Path('/opt/verus/cargo-verus').read_bytes()).hexdigest(),
        'cargo': subprocess.check_output(['cargo', '--version'], text=True, timeout=10).strip(),
        'vstd': '0.0.0-2026-11-01-0251; default-features=false; features=[alloc]',
    }
    if 'Version: 0.2026.11.02.8bf68ee\n' not in versions['verus']:
        raise SystemExit('Refusing an unpinned Verus version')
    threads = 1 if subprocess.run(['pgrep', '-f', '[c]orpus_pass.py'],
                                  stdout=subprocess.DEVNULL, timeout=10).returncode == 0 else 2
    # No solver-limit overrides: use the pinned verifier's default limits.
    cmd = ['cargo', 'verus', 'verify', '-v', '--', '--num-threads', str(threads), '--no-cheating']
    result = subprocess.run(['python3', '-I', str(ROOT / 'tools/run_logged.py'),
                             '--timeout', '1500', '--label', 'Verus: ' + sys.argv[1], '--', *cmd],
                            cwd=ROOT, stdout=subprocess.PIPE, text=True, timeout=1520)
    print(result.stdout, end='')
    run = int(re.findall(r'^\[run\] (\d+), exit', result.stdout, re.M)[-1])
    text = (ROOT / 'runs' / f'{run:04d}.log').read_text()
    # A dependency count or a cargo replay is not a PNG proof run. Read only
    # the primary crate's fresh invocation, after its own Checking line.
    root_checks = list(re.finditer(r'^\s+Checking png_verus v0\.1\.0 ', text, re.M))
    root_text = text[root_checks[-1].end():] if root_checks else ''
    counts = re.findall(r'^verification results:: (\d+) verified, (\d+) errors$', root_text, re.M)
    unchanged = before == inventory()
    valid = (result.returncode == 0 and unchanged and counts and int(counts[-1][0]) > 0
             and int(counts[-1][1]) == 0 and re.search(r'Fresh vstd v0\.0\.0-2026-11-01-0251', text)
             and re.search(r'Running .*--crate-name png_verus ', root_text)
             and not re.search(r'(Checking|Compiling) vstd ', text))
    record = {'kind': 'M3-checkpoint', 'run': run, 'command': cmd, 'threads': threads,
              'versions': versions,
              'source_sha256': before, 'source_unchanged': unchanged,
              'verified': int(counts[-1][0]) if counts else 0,
              'errors': int(counts[-1][1]) if counts else None,
              'accepted': bool(valid), 'log_sha256': hashlib.sha256(text.encode()).hexdigest()}
    (ROOT / 'runs' / f'{run:04d}.json').write_text(json.dumps(record, indent=2) + '\n')
    if not valid:
        print('Checkpoint refused: require unchanged sources, Fresh vstd and a fresh positive zero-error count.',
              file=sys.stderr)
        return result.returncode or 1
    print(f'[checkpoint] run {run}: {record["verified"]} verified, 0 errors; {threads} verifier thread(s)')
    return 0


if __name__ == '__main__':
    sys.exit(main())
