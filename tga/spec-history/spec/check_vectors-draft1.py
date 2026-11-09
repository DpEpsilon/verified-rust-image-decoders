#!/usr/bin/env python3
"""Independent TGA reference oracle and strict fixture checker. Run with python3 -I."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
LIMIT = 67_108_864
SIGNATURE = b'TRUEVISION-XFILE.\x00'
ERRORS = set('FileTooLarge TruncatedHeader UnsupportedImageType InvalidColorMapFlag InvalidDimensions UnsupportedInterleave UnsupportedPixelDepth InvalidColorMap ColorMapRange InvalidAttributeBits OutputTooLarge TruncatedImageId TruncatedColorMap TruncatedPixels RleTruncated RleOverrun ColorIndexOutOfRange TrailingBytes'.split())

class Invalid(Exception):
    pass

def reject(name):
    raise Invalid(name)

def le16(buf, pos):
    return buf[pos] | buf[pos + 1] << 8

def rgb5(c):
    return (c << 3) | (c >> 2)

def colour(sample, depth, attrs):
    if depth in (15, 16):
        v = le16(sample, 0)
        return bytes((rgb5((v >> 10) & 31), rgb5((v >> 5) & 31), rgb5(v & 31),
                      255 if depth == 15 or attrs == 0 or v & 0x8000 else 0))
    if depth == 24:
        return bytes((sample[2], sample[1], sample[0], 255))
    return bytes((sample[2], sample[1], sample[0], sample[3] if attrs == 8 else 255))

def decode(data):
    if len(data) > LIMIT:
        reject('FileTooLarge')
    if len(data) < 18:
        reject('TruncatedHeader')
    typ = data[2]
    if typ not in (1, 2, 3, 9, 10, 11):
        reject('UnsupportedImageType')
    flag = data[1]
    if flag > 1:
        reject('InvalidColorMapFlag')
    width, height = le16(data, 12), le16(data, 14)
    if not (1 <= width <= 8192 and 1 <= height <= 8192):
        reject('InvalidDimensions')
    desc = data[17]
    if desc & 0xc0:
        reject('UnsupportedInterleave')
    indexed, gray = typ in (1, 9), typ in (3, 11)
    depth = data[16]
    allowed = (8, 16) if indexed or gray else (15, 16, 24, 32)
    if depth not in allowed:
        reject('UnsupportedPixelDepth')
    first, length, map_depth = le16(data, 3), le16(data, 5), data[7]
    if indexed:
        if flag != 1 or length == 0 or map_depth not in (15, 16, 24, 32):
            reject('InvalidColorMap')
        if first + length > 65536:
            reject('ColorMapRange')
    elif flag or first or length or map_depth:
        reject('InvalidColorMap')
    attr = desc & 15
    alpha_depth = map_depth if indexed else depth
    valid_attr = ((0, 1) if alpha_depth == 16 and not gray else
                  (0, 8) if alpha_depth == 32 or gray and depth == 16 else (0,))
    if attr not in valid_attr:
        reject('InvalidAttributeBits')
    count = width * height
    if 4 * count > LIMIT:
        reject('OutputTooLarge')
    pos = 18
    if len(data) - pos < data[0]:
        reject('TruncatedImageId')
    pos += data[0]
    map_bytes = length * ((map_depth + 7) // 8) if indexed else 0
    if len(data) - pos < map_bytes:
        reject('TruncatedColorMap')
    palette = memoryview(data)[pos:pos + map_bytes]
    pos += map_bytes
    unit = (depth + 7) // 8
    entry_unit = (map_depth + 7) // 8 if indexed else 0
    output = bytearray(4 * count)
    right, upper = bool(desc & 16), bool(desc & 32)

    def pixel(sample):
        if indexed:
            key = sample[0] if unit == 1 else le16(sample, 0)
            if key < first or key >= first + length:
                reject('ColorIndexOutOfRange')
            begin = (key - first) * entry_unit
            return colour(palette[begin:begin + entry_unit], map_depth, attr)
        if gray:
            y = sample[0]
            return bytes((y, y, y, sample[1] if depth == 16 and attr == 8 else 255))
        return colour(sample, depth, attr)

    def put(k, rgba):
        x, y = k % width, k // width
        dest = 4 * ((y if upper else height - y - 1) * width +
                    (width - x - 1 if right else x))
        output[dest:dest + 4] = rgba

    if typ in (1, 2, 3):
        for k in range(count):
            if len(data) - pos < unit:
                reject('TruncatedPixels')
            put(k, pixel(data[pos:pos + unit]))
            pos += unit
    else:
        produced = 0
        while produced < count:
            if pos == len(data):
                reject('RleTruncated')
            control = data[pos]
            pos += 1
            run = (control & 127) + 1
            if run > count - produced:
                reject('RleOverrun')
            needed = unit if control & 128 else run * unit
            if len(data) - pos < needed:
                reject('RleTruncated')
            if control & 128:
                rgba = pixel(data[pos:pos + unit])
                for k in range(produced, produced + run):
                    put(k, rgba)
            else:
                for j in range(run):
                    put(produced + j, pixel(data[pos + j * unit:pos + (j + 1) * unit]))
            pos += needed
            produced += run
    suffix = data[pos:]
    if suffix and not (len(suffix) >= 26 and suffix.endswith(SIGNATURE)):
        reject('TrailingBytes')
    return width, height, bytes(output)

def runs(rgba):
    out = []
    for p in range(0, len(rgba), 4):
        h = rgba[p:p + 4].hex()
        if out and out[-1]['hex'] == h:
            out[-1]['count'] += 1
        else:
            out.append({'count': 1, 'hex': h})
    return out

def check_vector(v):
    keys = {'name', 'input_hex', 'expected', 'input_repeat'}
    assert set(v) <= keys and {'name', 'input_hex', 'expected'} <= set(v), v['name']
    h = v['input_hex']
    assert isinstance(h, str) and h == h.lower() and len(h) % 2 == 0
    count = v.get('input_repeat', 1)
    assert type(count) is int and count >= 1
    expanded_len = len(h) // 2 * count
    if expanded_len > LIMIT:
        actual = {'error': 'FileTooLarge'}  # R1 symbolic length lemma; no huge allocation
    else:
        data = bytes.fromhex(h) * count
        try:
            w, ht, output = decode(data)
            actual = {'width': w, 'height': ht, 'rgba_runs': runs(output)}
        except Invalid as exc:
            actual = {'error': str(exc)}
    expected = v['expected']
    if 'error' in expected:
        assert set(expected) == {'error'} and expected['error'] in ERRORS
    else:
        assert set(expected) == {'width', 'height', 'rgba_runs'}
        rr = expected['rgba_runs']
        assert sum(r['count'] for r in rr) == expected['width'] * expected['height']
        assert all(set(r) == {'count', 'hex'} and type(r['count']) is int and r['count'] > 0 and len(r['hex']) == 8 and r['hex'] == r['hex'].lower() and (i == 0 or rr[i-1]['hex'] != r['hex']) for i, r in enumerate(rr))
    if actual != expected:
        raise AssertionError(f"{v['name']}: expected {expected!r}, got {actual!r}")

def main():
    vectors = json.loads((ROOT / 'VECTORS.json').read_text())
    assert isinstance(vectors, list)
    names = set()
    for v in vectors:
        assert v['name'] not in names, v['name']
        names.add(v['name'])
        check_vector(v)
    print(f'{len(vectors)} independent TGA conformance vectors passed')

if __name__ == '__main__':
    main()
