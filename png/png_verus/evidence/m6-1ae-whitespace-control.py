#!/usr/bin/env python3
"""Check checkpoint diff and new evidence, including untracked EOF blank lines."""
import argparse
import gzip
import json
from pathlib import Path
import subprocess


def check_text(data, label):
    assert data.endswith(b'\n'), f'{label}: missing final LF'
    assert data.splitlines()[-1].strip(b' \t\r'), f'{label}: trailing blank line'
    for number, line in enumerate(data.splitlines(), 1):
        assert not line.endswith((b' ', b'\t')), f'{label}:{number}: trailing whitespace'


def self_test():
    check_text(b'x\n', 'valid')
    for data in [b'x\n\n', b'x \n', b'x\n \n', b'x\n\t\n', b'x', b'\n']:
        try:
            check_text(data, 'invalid')
        except AssertionError:
            continue
        raise AssertionError(f'checker accepted {data!r}')


def check_checkpoint(root, base):
    self_test()
    root = Path(root)
    repo = root.parents[1]
    subprocess.run(['git', '-C', str(repo), 'diff', '--check', base, '--',
                    'png/png_verus'], check=True, timeout=30)
    changed = subprocess.check_output(
        ['git', '-C', str(repo), 'diff', '--name-only', base, '--',
         'png/png_verus'], text=True, timeout=30).splitlines()
    files = {repo / name for name in changed}
    files.add(root / 'src/still_equivalence.rs')
    files.update((root / 'evidence').glob('m6-1ae-*'))
    checked = []
    for path in sorted(files):
        if not path.is_file():
            continue
        data = path.read_bytes()
        if path.name.endswith('.log.gz'):
            data = gzip.decompress(data)
        check_text(data, str(path.relative_to(root)))
        checked.append(str(path.relative_to(root)))
    return {'whitespace_clean': True,
            'whitespace_scope': 'tracked diff from base and all new 1a-E source/evidence, including decompressed logs',
            'whitespace_base': base,
            'whitespace_files_checked': checked,
            'whitespace_self_test_rejects_trailing_blank_lines': True,
            'historical_exception_unchanged': 'evidence/m6-1am-public-errors-0675.log'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base', required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    print(json.dumps(check_checkpoint(root, args.base), indent=2))
