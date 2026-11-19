#!/usr/bin/env python3
"""Run a split Verus checkpoint: the passes of tools/verify_passes.json, serially.

Only verify.sh calls this, after it has taken the shared verifier lock (held on
an inherited file descriptor for the whole sequence) and refused a running
legacy invocation. Each pass is its own run_logged.py run with the unchanged
25-minute bound. The sequence stops at the first failing pass. A complete
checkpoint writes runs/checkpoint-NNNN.json (NNNN = first pass run) recording
the source tree hash, each pass's run, duration and count, the plain sum and
the unique total (see tools/split_checkpoint.py).
"""
import argparse
import datetime
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import split_checkpoint as sc

LEGACY = '(cargo-verus verify|rust_verify .*--crate-name)'


def runlog_row(run):
    for line in (ROOT / 'RUNLOG.tsv').read_text().splitlines():
        fields = line.split('\t')
        if fields[0] == str(run):
            return fields
    raise AssertionError(('run missing from RUNLOG.tsv', run))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--label', required=True)
    ap.add_argument('--only', help='run one named pass; never a checkpoint')
    a = ap.parse_args()
    manifest = sc.load_manifest(ROOT)
    sc.check_coverage(manifest, ROOT)
    passes = manifest['passes']
    if a.only:
        passes = [p for p in passes if p['name'] == a.only]
        assert passes, ('unknown pass', a.only)
    inventory = sc.source_inventory(ROOT)
    digest = sc.tree_hash(inventory)
    print(f'[split] source tree {digest}; {len(passes)} pass(es)', flush=True)
    done = []
    for i, p in enumerate(passes, 1):
        assert sc.tree_hash(sc.source_inventory(ROOT)) == digest, 'source tree changed during the checkpoint'
        if subprocess.run(['pgrep', '-f', LEGACY], stdout=subprocess.DEVNULL).returncode == 0:
            print('Another Verus invocation is already running.', file=sys.stderr)
            return 2
        tag = f'single pass {p["name"]}, not a checkpoint' if a.only else f'split pass {i}/{len(passes)} {p["name"]}'
        out = subprocess.run(['python3', '-I', str(ROOT / 'tools/run_logged.py'), '--timeout', '1500',
                              '--label', f'Verus: {a.label} [{tag}]', '--', *sc.pass_command(p, digest)],
                             cwd=ROOT, stdout=subprocess.PIPE, text=True)
        sys.stdout.write(out.stdout)
        run = int(re.findall(r'^\[run\] (\d+), exit (-?\d+)', out.stdout, re.M)[-1][0])
        row = runlog_row(run)
        log = ROOT / 'runs' / f'{run:04d}.log'
        try:
            verified = sc.check_pass_log(manifest, p, log.read_text(), digest)['verified']
        except AssertionError as e:
            print(f'[split] pass {p["name"]} (run {run}) failed its check: {e}', file=sys.stderr)
            return out.returncode or 1
        assert out.returncode == 0 and row[2] == 'exit=0', row
        done.append({'pass': p['name'], 'feature': p['feature'], 'modules': p['modules'], 'run': run,
                     'seconds': float(row[3].rstrip('s')), 'verified': verified,
                     'verification_results': f'verification results:: {verified} verified, 0 errors'})
        print(f'[split] pass {p["name"]}: run {run}, {row[3]}, {verified} verified, 0 errors', flush=True)
    assert sc.tree_hash(sc.source_inventory(ROOT)) == digest, 'source tree changed during the checkpoint'
    summed = sum(d['verified'] for d in done)
    if a.only:
        print(f'[split] single pass {a.only}: {summed} verified, 0 errors (not a checkpoint)')
        return 0
    logs = {d['pass']: (ROOT / 'runs' / f'{d["run"]:04d}.log').read_text() for d in done}
    counts, total = sc.check_passes(manifest, logs, digest)
    assert counts == {d['pass']: d['verified'] for d in done}
    record = {'kind': 'split-checkpoint', 'label': a.label,
              'finished': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'source_tree_sha256': digest, 'source_sha256': inventory,
              'passes': done, 'verified_sum': summed, 'verified_unique_total': total,
              'seconds_total': round(sum(d['seconds'] for d in done), 2)}
    path = ROOT / 'runs' / f'checkpoint-{done[0]["run"]:04d}.json'
    with path.open('x') as f:
        f.write(json.dumps(record, indent=1) + '\n')
    print(f'[split] checkpoint {path}: {len(done)} passes, 0 errors; unique total {total} verified '
          f'(plain sum {summed} counts base {len(done)} times); {record["seconds_total"]}s')
    return 0


if __name__ == '__main__':
    sys.exit(main())
