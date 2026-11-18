#!/usr/bin/env python3
"""Check authored witness input provenance without consulting an inflate oracle."""
import ast
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def idat(data):
    pos = 8
    parts = []
    while pos + 12 <= len(data):
        size = int.from_bytes(data[pos:pos + 4], 'big')
        assert pos + 12 + size <= len(data)
        if data[pos + 4:pos + 8] == b'IDAT':
            parts.append(data[pos + 8:pos + 8 + size])
        pos += 12 + size
    assert parts, 'This checker covers the authored IDAT witness subset only'
    return b''.join(parts)


def main():
    vectors = {v['name']: v for v in json.loads((ROOT.parent / 'spec-frozen/VECTORS.json').read_text())}
    for name in ('author_more_witnesses.py', 'author_dynamic_witnesses.py', 'author_d4_witnesses.py', 'author_window_witnesses.py', 'author_invalid_dynamic_witnesses.py'):
        tree = ast.parse((ROOT / 'tools' / name).read_text())
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                assert all(alias.name in {'json', 'pathlib'} for alias in node.names), name
            if isinstance(node, ast.ImportFrom):
                assert node.module == 'pathlib', name
            if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant):
                assert node.slice.value not in {'expected', 'expected_error', 'rgba', 'rgba_runs'}, name
    text = '\n'.join((ROOT / 'src' / name).read_text() for name in ('witnesses.rs', 'witness_cases.rs', 'witness_dynamic_cases.rs', 'witness_d4.rs', 'witness_overlap.rs', 'witness_invalid_dynamic.rs'))
    pattern = re.compile(r'/// ([\w/-]+):[^\n]*\n(?:#\[[^\n]*\]\s*)?fn \w+\(\) \{\s*(?:hide\([^;]*\);\s*)*let wire(?:\s*:\s*\[u8;\s*0\])?\s*=\s*\[([^\]]*)\]', re.M)
    seen = set()
    for name, elements in pattern.findall(text):
        wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in elements.split(',') if x.strip())
        assert name not in seen, name
        assert wire == idat(bytes.fromhex(vectors[name]['input_hex'])), name
        seen.add(name)
    # The large FCHECK fixture uses a separately verified literal constructor;
    # its caller needs only the authored header because rejection precedes BTYPE.
    ctor = re.search(r'fn fcheck_bytes\(\) -> \(wire: \[u8;\d+\]\)\s*ensures[^\{]*\{\s*\[([^\]]*)\]\s*\}', text)
    assert ctor and 'let wire = fcheck_bytes();' in text
    header_wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[1].split(',') if x.strip())
    assert header_wire == idat(bytes.fromhex(vectors['zlib/fcheck']['input_hex']))
    seen.add('zlib/fcheck')
    # This is the authored subset, not a claim of coverage of all 631 streams.
    assert len(seen) == 43, (len(seen), sorted(seen))
    assert 'zlib/empty-idat-only' in seen
    print(f'{len(seen)} authored witness streams match frozen IDAT extraction, including Z=[]; no oracle used')


if __name__ == '__main__':
    main()
