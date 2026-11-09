#!/usr/bin/env python3
"""Generate tests/corpus_expected.json: the frozen oracle's answer for every corpus file.

Run once with `python3 -I tools/corpus_expected.py`. The generator imports the frozen
oracle `/home/ws/projects/dib/spec/check_vectors.py` (hash recorded in
`spec-frozen/FREEZE.txt`) and the first-DIB entry shim from `../run_corpus.py`, which is a
measurement shim: it picks one DIB out of an ICO/CUR container and passes it to the core
in `IcoEntryDib` mode. The core itself never sees a container.

Each row records what the oracle returned, so the Rust corpus test compares against a file
instead of depending on a Python install at test time.
"""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ORACLE_PATH = Path(__file__).resolve().parent.parent / 'spec-history' / 'spec' / 'check_vectors.py'

spec = importlib.util.spec_from_file_location('dib_oracle', ORACLE_PATH)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


def entry(data, cursor=False):
    """First-DIB selection shim, copied from ../run_corpus.py verbatim."""
    if len(data) < 6 or data[:2] != b'\x00\x00' or data[2:4] not in (b'\x01\x00', b'\x02\x00'):
        return None, 'ContainerInvalid'
    n = int.from_bytes(data[4:6], 'little')
    if n == 0 or len(data) < 6 + 16 * n:
        return None, 'ContainerInvalid'
    png = 0
    for i in range(n):
        p = 6 + 16 * i
        length = int.from_bytes(data[p + 8:p + 12], 'little')
        offset = int.from_bytes(data[p + 12:p + 16], 'little')
        if length == 0 or offset < 6 + 16 * n or offset + length > len(data):
            return None, 'ContainerInvalid'
        if cursor and length == 8 and n == 1 and offset + length < len(data):
            length = len(data) - offset
        content = data[offset:offset + length]
        if content.startswith(b'\x89PNG\r\n\x1a\n'):
            png += 1
            continue
        return content, None
    return None, 'ContainerPngOnly' if png else 'ContainerNoDib'


def main():
    directory = ROOT / 'corpus'
    rows = []
    for path in sorted(directory.iterdir()):
        if path.suffix.lower() not in ('.bmp', '.ico', '.cur') or not path.is_file():
            continue
        data = path.read_bytes()
        if path.suffix.lower() == '.bmp':
            mode, kind = 'BmpFile', None
        else:
            data, kind = entry(data, cursor=path.suffix.lower() == '.cur')
            mode = 'IcoEntryDib'
        if kind is not None:
            rows.append({'file': path.name, 'mode': mode, 'outcome': kind})
            continue
        try:
            width, height, rgba = oracle.decode(data, mode)
        except oracle.Invalid as e:
            rows.append({'file': path.name, 'mode': mode, 'outcome': str(e)})
            continue
        rows.append({
            'file': path.name,
            'mode': mode,
            'outcome': 'Accepted',
            'width': width,
            'height': height,
            'rgba_hex': rgba.hex(),
        })
    out = {'oracle': str(ORACLE_PATH), 'files': rows}
    (ROOT / 'dib_verus' / 'tests' / 'corpus_expected.json').write_text(
        json.dumps(out, indent=1) + '\n')
    accepted = sum(1 for r in rows if r['outcome'] == 'Accepted')
    print(f'wrote {len(rows)} rows ({accepted} accepted) to tests/corpus_expected.json')


if __name__ == '__main__':
    main()
