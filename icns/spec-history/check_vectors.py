#!/usr/bin/env python3
"""Independent, bounded ICNS core oracle and conformance-vector checker.

Run: python3 -I /home/ws/projects/spec/check_vectors.py
Only standard-library imports; fixtures are never executed as code.
"""
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent
PAIRS = {b'is32': (b's8mk', 16), b'il32': (b'l8mk', 32),
         b'ih32': (b'h8mk', 48), b'it32': (b't8mk', 128)}
MASKS = {m: (c, side) for c, (m, side) in PAIRS.items()}
OPAQUE = {b'icp4': 16, b'icp5': 32, b'icp6': 64,
          b'ic07': 128, b'ic08': 256, b'ic09': 512, b'ic10': 1024,
          b'ic11': 32, b'ic12': 64, b'ic13': 256, b'ic14': 512,
          b'icsB': 36, b'sb24': 24, b'SB24': 48}
PNG = bytes.fromhex('89504e470d0a1a0a')
JP2 = bytes.fromhex('0000000c6a5020200d0a870a')

class DecodeError(Exception):
    pass

def fail(name):
    raise DecodeError(name)

def parse(data):
    if len(data) > 67108864:
        fail('FileTooLarge')
    if len(data) < 8:
        fail('TruncatedHeader')
    if data[:4] != b'icns':
        fail('BadMagic')
    length = int.from_bytes(data[4:8], 'big')
    if length < 8:
        fail('InvalidFileLength')
    if length > len(data):
        fail('TruncatedFile')
    if length < len(data):
        fail('TrailingFileBytes')
    offset, entries = 8, []
    while offset < length:
        if length - offset < 8:
            fail('TruncatedEntryHeader')
        typ = data[offset:offset+4]
        size = int.from_bytes(data[offset+4:offset+8], 'big')
        if size < 8:
            fail('InvalidEntryLength')
        if size > length - offset:
            fail('EntryOutOfBounds')
        if len(entries) == 4096:
            fail('TooManyEntries')
        start, end = offset + 8, offset + size
        payload = data[start:end]
        nominal = OPAQUE.get(typ)
        opaque = (nominal is not None or
                  (typ not in PAIRS and typ not in MASKS and
                   (payload.startswith(PNG) or payload.startswith(JP2))))
        entries.append({'type_hex': typ.hex(), 'payload': [start, end],
                        'opaque_candidate': opaque, 'nominal_side': nominal})
        offset = end
    return entries

def selected(data, entries, index, mapping):
    if not isinstance(index, int) or isinstance(index, bool) or index < 0 or index >= len(entries):
        fail('IndexOutOfRange')
    e = entries[index]
    typ = bytes.fromhex(e['type_hex'])
    if typ not in mapping:
        fail('UnsupportedType')
    return typ, mapping[typ][1], data[e['payload'][0]:e['payload'][1]]

def rgb(data, entries, index):
    typ, side, payload = selected(data, entries, index, PAIRS)
    if typ == b'it32' and payload.startswith(b'\x00\x00\x00\x00'):
        payload = payload[4:]
    n = side * side
    planes, pos = [], 0
    for _ in range(3):
        channel = bytearray()
        while len(channel) < n:
            if pos == len(payload):
                fail('RleTruncated')
            c = payload[pos]
            pos += 1
            count = c + 1 if c < 128 else c - 125
            if count > n - len(channel):
                fail('RleOverrun')
            needed = count if c < 128 else 1
            if len(payload) - pos < needed:
                fail('RleTruncated')
            if c < 128:
                channel.extend(payload[pos:pos+count])
            else:
                channel.extend(payload[pos:pos+1] * count)
            pos += needed
        planes.append(channel)
    if pos != len(payload):
        fail('RleTrailingBytes')
    return side, bytes(value for triplet in zip(*planes) for value in triplet)

def mask(data, entries, index):
    _typ, side, payload = selected(data, entries, index, MASKS)
    if len(payload) != side * side:
        fail('InvalidMaskLength')
    return side, payload

def combine(data, entries, color_index, mask_index):
    if color_index is None:
        fail('MissingColor')
    if mask_index is None:
        fail('MissingMask')
    side, colors = rgb(data, entries, color_index)
    mside, alphas = mask(data, entries, mask_index)
    if side != mside:
        fail('TypeMismatch')
    return side, bytes(v for i in range(side * side)
                       for v in (*colors[3*i:3*i+3], alphas[i]))

def encoded_runs(data, stride):
    assert len(data) % stride == 0
    groups = [data[i:i+stride] for i in range(0, len(data), stride)]
    result = []
    for group in groups:
        if result and result[-1]['hex'] == group.hex():
            result[-1]['count'] += 1
        else:
            result.append({'count': 1, 'hex': group.hex()})
    return result

def evaluate(vector):
    data = bytes.fromhex(vector['input_hex']) * vector.get('input_repeat', 1)
    entries = parse(data)
    op = vector['operation']
    if op == 'parse':
        return {'entries': entries}
    if op == 'rgb':
        side, pixels = rgb(data, entries, vector['color_index'])
        return {'side': side, 'rgb_runs': encoded_runs(pixels, 3)}
    if op == 'mask':
        side, pixels = mask(data, entries, vector['mask_index'])
        return {'side': side, 'alpha_runs': encoded_runs(pixels, 1)}
    if op == 'combine':
        side, pixels = combine(data, entries, vector.get('color_index'), vector.get('mask_index'))
        return {'side': side, 'rgba_runs': encoded_runs(pixels, 4)}
    raise AssertionError('Unknown vector operation: ' + op)

def main():
    vectors = json.loads((BASE / 'VECTORS.json').read_text())
    names = set()
    for v in vectors:
        assert v['name'] not in names, v['name']
        names.add(v['name'])
        try:
            actual = evaluate(v)
        except DecodeError as error:
            actual = {'error': str(error)}
        if actual != v['expected']:
            raise AssertionError(f"{v['name']}: expected {v['expected']!r}; got {actual!r}")
    print(f'{len(vectors)} independent conformance vectors passed')

if __name__ == '__main__':
    main()
