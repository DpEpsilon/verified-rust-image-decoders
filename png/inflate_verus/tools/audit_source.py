#!/usr/bin/env python3
"""M1a source/build-input checks, not the signed production call-graph audit."""
import argparse
import hashlib
from pathlib import Path
import re
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parent.parent


def checked(command):
    subprocess.run(command, cwd=ROOT, check=True, timeout=90)


def erased_work_ok(text):
    return not re.search(r'\b(?:core|advances|core_steps|seg_steps)\s*:\s*u128\b', text)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--self-test', action='store_true')
    args = p.parse_args()
    manifest = tomllib.loads((ROOT / 'Cargo.toml').read_text())
    # Coordinator-approved cache reuse: vstd/alloc is available to ghost/spec
    # code; source and symbol gates must still show no shipped allocator use.
    assert manifest.get('dependencies') == {'vstd': {
        'version': '=0.0.0-2026-11-01-0251', 'default-features': False, 'features': ['alloc'],
    }}, 'review new library dependencies or vstd feature changes'
    lib = (ROOT / 'src/lib.rs').read_text()
    assert '#![no_std]' in lib and '#![forbid(unsafe_code)]' in lib
    src = sorted((ROOT / 'src').rglob('*.rs'))
    assert src and not (ROOT / 'src/alloc_guard').exists()
    forbidden = re.compile(r'\b(?:extern\s+crate\s+(?:alloc|std)|unsafe\s*\{|'
                           r'assume_specification|external_body|admit\s*\(|assume\s*\()')
    for path in src:
        text = path.read_text()
        assert not forbidden.search(text), path
        assert erased_work_ok(text), f'runtime work counter: {path}'
    for field in ('core_steps', 'seg_steps'):
        assert re.search(rf'pub {field}: Ghost<nat>', lib), field
    if args.self_test:
        for field in ('core', 'advances', 'core_steps', 'seg_steps'):
            assert not erased_work_ok(f'pub {field}: u128,'), field
            assert erased_work_ok(f'pub {field}: Ghost<nat>,'), field
        print('erased-counter lint positive/negative controls passed')
    # Reuse the GIF scanner instead of copying its Rust/Verus lexical machinery.
    scanner = REPO / 'gif/tools/alloc_scan.py'
    checked([sys.executable, '-I', str(scanner), '--production-files', str(ROOT / 'src')])
    for mode in ([], ['--allow-reserved']):
        checked([sys.executable, '-I', str(scanner), *mode, *map(str, src)])
    if args.self_test:
        checked([sys.executable, '-I', str(scanner), '--self-test'])
    checked([sys.executable, '-I', str(ROOT / 'tools/audit_witnesses.py')])
    checked([sys.executable, '-I', str(ROOT / 'tools/lint_caps.py'),
             *(['--self-test'] if args.self_test else [])])
    frozen = ROOT.parent / 'spec-frozen'
    count = 0
    for line in (frozen / 'FREEZE.txt').read_text().splitlines():
        if not line.strip() or line.startswith('#'):
            continue
        digest, file = line.split(maxsplit=1)
        assert hashlib.sha256((frozen / file).read_bytes()).hexdigest() == digest, file
        count += 1
    print(f'{count} frozen hashes match; allocation scanner clean; pinned ghost vstd/alloc cache configuration')
    print('No signed production call-graph audit or Verus theorem is claimed.')

if __name__ == '__main__':
    main()
