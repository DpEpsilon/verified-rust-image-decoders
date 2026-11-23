#!/usr/bin/env python3
"""Reuse 1a-E whitespace checks for the combined corrected-Strict/1b evidence."""
import argparse
import gzip
import importlib.util
import json
from pathlib import Path


def previous_control(root):
    path = Path(root) / 'evidence/m6-1ae-whitespace-control.py'
    spec = importlib.util.spec_from_file_location('previous_whitespace_control', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def check_text(data, label):
    root = Path(__file__).resolve().parents[1]
    previous_control(root).check_text(data, label)


def check_checkpoint(root, base):
    root = Path(root)
    previous = previous_control(root)
    report = previous.check_checkpoint(root, base)
    files = {root / name for name in report['whitespace_files_checked']}
    files.update(root / 'src' / name for name in
                 ['still_recovery.rs', 'policy_still_model.rs', 'policy_still_equivalence.rs'])
    files.update((root / 'evidence').glob('m6-1b-*'))
    for path in sorted(files):
        if not path.is_file():
            continue
        data = path.read_bytes()
        if path.name.endswith('.log.gz'):
            data = gzip.decompress(data)
        previous.check_text(data, str(path.relative_to(root)))
    report['whitespace_files_checked'] = [str(path.relative_to(root)) for path in sorted(files) if path.is_file()]
    report['whitespace_scope'] = 'tracked diff from base plus new 1b source/evidence and retained 1a-E evidence, including decompressed logs'
    report['reuses_1ae_checker_without_modification'] = True
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base', required=True)
    args = parser.parse_args()
    print(json.dumps(check_checkpoint(Path(__file__).resolve().parents[1], args.base), indent=2))
