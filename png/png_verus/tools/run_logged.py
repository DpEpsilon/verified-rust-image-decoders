#!/usr/bin/env python3
"""Record bounded foreground commands, including failed ones, in RUNLOG.tsv."""
import argparse
import datetime
import os
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--label', required=True)
    p.add_argument('--timeout', type=int, default=600)
    p.add_argument('command', nargs=argparse.REMAINDER)
    a = p.parse_args()
    command = a.command[1:] if a.command[:1] == ['--'] else a.command
    assert command and 0 < a.timeout <= 1500
    runs = ROOT / 'runs'
    runs.mkdir(exist_ok=True)
    nums = [int(f.stem) for f in runs.glob('[0-9][0-9][0-9][0-9].log')]
    n = max(nums, default=0) + 1
    log = runs / f'{n:04d}.log'
    env = dict(os.environ, CARGO_BUILD_JOBS='1', PYTHONDONTWRITEBYTECODE='1')
    env['PATH'] = '/opt/cargo/bin:/opt/verus:' + env.get('PATH', '')
    start = time.monotonic()
    with log.open('x') as out:
        out.write('command: ' + repr(command) + '\n')
        out.flush()
        result = subprocess.run(['timeout', '--kill-after=10s', str(a.timeout), *command],
                                cwd=ROOT, env=env, stdout=out, stderr=subprocess.STDOUT)
    duration = time.monotonic() - start
    text = log.read_text()
    summaries = re.findall(r'^.*verification results.*$', text, re.M)
    summary = summaries[-1] if summaries else 'no verifier count (runtime/tooling command)'
    label = a.label.replace('\t', ' ').replace('\n', ' ')
    stamp = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (ROOT / 'RUNLOG.tsv').open('a') as out:
        out.write(f'{n}\t{stamp}\texit={result.returncode}\t{duration:.2f}s\t{label}\t{summary}\t{log}\n')
    print('\n'.join(text.splitlines()[-60:]))
    print(f'[run] {n}, exit {result.returncode}, {duration:.2f}s, {log}')
    return result.returncode


if __name__ == '__main__':
    sys.exit(main())
