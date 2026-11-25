#!/usr/bin/env python3
"""Apply the accepted whitespace policy to fresh complete E6-M5-S witness evidence."""
import gzip
import importlib.util
from pathlib import Path


def previous(root):
    path = Path(root) / 'evidence/m6-e6-m3-whitespace-control.py'
    spec = importlib.util.spec_from_file_location('icc_previous_whitespace', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def check_text(data, label):
    previous(Path(__file__).resolve().parents[1]).check_text(data, label)


def check_checkpoint(root, base):
    root = Path(root)
    inherited = previous(root)
    report = inherited.check_checkpoint(root, base)
    files = {root / name for name in report['whitespace_files_checked']}
    files.add(root / 'src/witness_m6_m5_still.rs')
    files.add(root / 'src/witness_m6_still_model.rs')
    files.add(root / 'tools/generate_m6_m5_still_witnesses.py')
    files.update((root / 'evidence').glob('m6-e6-m5-s-*'))
    for path in sorted(files):
        if not path.is_file():
            continue
        data = path.read_bytes()
        if path.name.endswith('.log.gz'):
            data = gzip.decompress(data)
        inherited.check_text(data, str(path.relative_to(root)))
    report['whitespace_files_checked'] = [str(path.relative_to(root)) for path in sorted(files) if path.is_file()]
    report['whitespace_scope'] = 'tracked diff plus new complete E6-M5-S witness source/evidence, including decompressed logs; accepted historical evidence unchanged'
    report['reuses_accepted_whitespace_policy_without_modification'] = True
    return report
