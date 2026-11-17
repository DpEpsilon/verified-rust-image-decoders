#!/usr/bin/env python3
"""Lexical cap-copy falsification gate, including comments; not a Rust parser.

Pattern/self-test approach follows gif/tools/lint_caps.sh. Values come from the
six PNG cap assignments, not a copied list. limits.rs alone may own constants.
M2 must add caps_ok and CAPS-SYM; a clean lexical scan proves neither.
"""
import argparse
import ast
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
NAMES = {'MAX_INPUT', 'MAX_DIMENSION', 'MAX_OUTPUT', 'MAX_FRAMES', 'MAX_TOTAL_PIXELS', 'MAX_ICC'}


def integer(node):
    if isinstance(node, ast.Constant) and type(node.value) is int:
        return node.value
    if isinstance(node, ast.BinOp):
        a, b = integer(node.left), integer(node.right)
        if isinstance(node.op, ast.Add): return a + b
        if isinstance(node.op, ast.Sub): return a - b
        if isinstance(node.op, ast.Mult): return a * b
        if isinstance(node.op, ast.LShift): return a << b
    raise ValueError('unsupported cap expression')


def patterns():
    tree = ast.parse((ROOT.parent / 'limits-draft/limits.py').read_text())
    values = {n.targets[0].id: integer(n.value) for n in tree.body
              if isinstance(n, ast.Assign) and isinstance(n.targets[0], ast.Name)
              and n.targets[0].id in NAMES}
    assert set(values) == NAMES
    result = []
    for name, value in values.items():
        if value < 32768:
            continue  # Small literal collisions cannot be classified lexically.
        forms = [str(value), hex(value)]
        if value > 0 and value & (value - 1) == 0:
            exponent = value.bit_length() - 1
            forms += [rf'1(?:u(?:32|64|128|size))?\s*<<\s*{exponent}', rf'2\^{exponent}']
        result.append((name, re.compile(r'(?<![a-z0-9_])(?:' + '|'.join(forms)
                                       + r')(?:[ui](?:8|16|32|64|128|size))?(?![a-z0-9_])')))
    return result


def findings(text, checks):
    normalized = text.lower().replace('_', '')
    return [name for name, pattern in checks if pattern.search(normalized)]


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--self-test', action='store_true')
    a = p.parse_args()
    checks = patterns()
    files = [f for f in (ROOT / 'src').rglob('*.rs') if f.name != 'limits.rs']
    assert files
    bad = [(str(f), findings(f.read_text(), checks)) for f in files]
    bad = [(f, hit) for f, hit in bad if hit]
    if bad:
        raise SystemExit(f'copied cap numerals: {bad}')
    if a.self_test:
        assert not findings('assert(k <= MAX_INPUT); assert(u32::MAX > k);', checks)
        assert findings('// copied constant 1_073_741_824u64', checks)
        assert findings('assert(MAX_OUTPUT == 0x1000_0000);', checks)
        assert findings('assert(MAX_INPUT == 1u64 << 30);', checks)
        assert findings('// 2^30', checks)
        print('cap lint positive/negative controls passed')
    print(f'cap lint clean: {len(files)} source files, all six configured caps')

if __name__ == '__main__':
    main()
