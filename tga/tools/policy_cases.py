#!/usr/bin/env python3
"""Stream independent oracle expectations; never import decoder implementation code."""
import importlib.util
import itertools
import json
from pathlib import Path
import random

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('frozen_tga_oracle', ROOT / 'policy/policy_oracle.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
fixtures = json.loads((ROOT / 'policy/POLICY-VECTORS.json').read_text())
rng = random.Random(191_4242)


def emit(group, name, raw, policy, repeat=1, profile=None, expected=None):
    if expected is None:
        expected = ({'error': 'FileTooLarge'} if len(raw) * repeat > oracle.MAX_INPUT
                    else oracle.actual(raw * repeat, policy, with_recovery=True))
    row = dict(group=group, name=name, input_hex=raw.hex(), input_repeat=repeat,
               policy=policy, expected=expected)
    if profile is not None:
        row['profile'] = profile
    print(json.dumps(row, separators=(',', ':')))


def generated(i, kind=None, rle=None):
    kind = kind if kind is not None else rng.choice((1, 2, 3))
    rle = rle if rle is not None else bool(rng.randrange(2))
    width, height = rng.randint(1, 5), rng.randint(1, 3)
    depth = rng.choice({1: (8, 16), 2: (15, 16, 24, 32), 3: (8, 16)}[kind])
    entry_depth = rng.choice((15, 16, 24, 32))
    first, length = rng.choice((0, 1, 39, 65535)), rng.choice((0, 1, 2, 4))
    flag = rng.choice((0, 1, 1, 1, 2)) if kind == 1 else rng.choice((0, 0, 0, 1))
    h = bytearray(18)
    h[0], h[1], h[2] = rng.randrange(3), flag, kind + (8 if rle else 0)
    if flag or kind == 1:
        h[3:5], h[5:7], h[7] = first.to_bytes(2, 'little'), length.to_bytes(2, 'little'), entry_depth
    h[12:14], h[14:16], h[16] = width.to_bytes(2, 'little'), height.to_bytes(2, 'little'), depth
    h[17] = rng.choice((0, 1, 4, 8)) | rng.choice((0, 16, 32, 48, 64))
    data = h + rng.randbytes(h[0])
    if flag:
        data += rng.randbytes(length * ((entry_depth + 7) // 8))
    sb = (depth + 7) // 8
    if rle:
        remaining = width * height
        while remaining > 0:
            count = rng.randint(1, 8)
            rep = bool(rng.randrange(2))
            data.append((128 if rep else 0) | (count - 1))
            data += rng.randbytes(sb * (1 if rep else count))
            remaining -= count
    else:
        data += rng.randbytes(width * height * sb)
    if i % 5 == 0:
        data = data[:rng.randrange(len(data) + 1)]
    elif i % 7 == 0:
        data += b'junk'
    elif i % 11 == 0:
        data += bytes(8) + oracle.FOOTER_SIGNATURE
    return bytes(data)


# Named-profile expectations and every proper prefix of every small fixture.
for name, policy in oracle.PROFILES.items():
    for v in fixtures['profiles'][name]:
        raw = bytes.fromhex(v['input_hex'])
        emit('profile-fixtures/' + name, v['name'], raw, policy, profile=name, expected=v['expected'])
        for stop in range(len(raw)):
            emit('profile-prefixes/' + name, v['name'] + '/' + str(stop), raw[:stop], policy, profile=name)

# The 169 original inputs under every profile; only Strict inherits their frozen
# expected values. Other profiles use their own independent oracle result.
core = json.loads((ROOT / 'spec-frozen/VECTORS.json').read_text())
for name, policy in oracle.PROFILES.items():
    for v in core:
        emit('profile-core/' + name, v['name'], bytes.fromhex(v['input_hex']), policy,
             repeat=v.get('input_repeat', 1), profile=name,
             expected=v['expected'] if name == 'Strict' else None)

for v in fixtures['custom']:
    emit('custom-frozen', v['name'], bytes.fromhex(v['input_hex']), v['policy'], expected=v['expected'])

# Every retained option value gets both frozen probes and independently generated
# inputs, including malformed samples and incomplete prefixes.
samples = [bytes.fromhex(v['input_hex']) for v in fixtures['profiles']['Strict']]
samples += [generated(i) for i in range(96)]
for field, choices in oracle.OPTIONS.items():
    for choice in choices:
        policy = dict(oracle.STRICT, **{field: choice})
        for i, raw in enumerate(samples):
            emit('option/' + field + '/' + choice, str(i), raw, policy)

# Exercise coupled map addressing, conversion and substitution decisions.
palette_samples = [generated(i, kind=1) for i in range(24)]
fields = ('map_header', 'map_index', 'index_range', 'map5', 'map16_alpha', 'alpha')
for values in itertools.product(*(oracle.OPTIONS[f] for f in fields)):
    policy = dict(oracle.RECOMMENDED_BETA, **dict(zip(fields, values)))
    for i, raw in enumerate(palette_samples):
        emit('coupled/palette', str(i) + '/' + '/'.join(values), raw, policy)

fields = ('attribute_bits', 'depths', 'image_type', 'interleave')
for values in itertools.product(*(oracle.OPTIONS[f] for f in fields)):
    policy = dict(oracle.STRICT, **dict(zip(fields, values)))
    for i, raw in enumerate(samples[-32:]):
        raw = bytearray(raw)
        if len(raw) >= 18 and i % 3 == 0:
            raw[2] = rng.choice((0, 4, 255))
        emit('coupled/header', str(i) + '/' + '/'.join(values), bytes(raw), policy)

fields = ('rle_overrun', 'trailing', 'index_range')
rle_samples = [generated(i, rle=True) for i in range(32)]
for values in itertools.product(*(oracle.OPTIONS[f] for f in fields)):
    policy = dict(oracle.RECOMMENDED_BETA, **dict(zip(fields, values)))
    for i, raw in enumerate(rle_samples):
        for stop in sorted({len(raw), max(0, len(raw) - 1), min(18, len(raw))}):
            emit('coupled/rle', str(i) + '/' + '/'.join(values) + '/' + str(stop), raw[:stop], policy)

for i in range(512):
    policy = {f: rng.choice(v) for f, v in oracle.OPTIONS.items()}
    emit('generated/all-fields', str(i), generated(i), policy)
