#!/usr/bin/env python3
"""Explicit draft-two fixtures. Expected colours are specified here, not computed by the oracle."""
import json
from pathlib import Path

path = Path(__file__).parent / 'spec' / 'VECTORS.json'
vectors = json.loads(path.with_name('VECTORS-draft1.json').read_text())
for vector in vectors:
    if vector['name'] in ('raw-16-alpha-zero', 'rle-16-alpha'):
        vector['expected']['rgba_runs'][0]['hex'] = 'ff0000ff'


def header(typ=2, depth=24, width=1, height=1, flag=0, first=0, length=0,
           map_depth=0, ident=0, desc=0x20):
    return (bytes((ident, flag, typ)) + first.to_bytes(2, 'little') +
            length.to_bytes(2, 'little') + bytes((map_depth,)) +
            bytes(4) + width.to_bytes(2, 'little') + height.to_bytes(2, 'little') +
            bytes((depth, desc)))


def add(name, data, pixels=None, error=None):
    assert (pixels is None) != (error is None), name
    expected = {'error': error} if error else {'width': int.from_bytes(data[12:14], 'little'),
                                               'height': int.from_bytes(data[14:16], 'little'),
                                               'rgba_runs': []}
    if pixels is not None:
        assert len(pixels) == expected['width'] * expected['height'], name
        for pixel in pixels:
            assert len(pixel) == 8, name
            runs = expected['rgba_runs']
            if runs and runs[-1]['hex'] == pixel:
                runs[-1]['count'] += 1
            else:
                runs.append({'count': 1, 'hex': pixel})
    vectors.append({'name': name, 'input_hex': data.hex(), 'expected': expected})


# B1: all four non-indexed types with and without an ID, all four map depths.
for typ, depth, sample, pixel in ((2, 24, bytes.fromhex('112233'), '332211ff'),
                                   (3, 8, bytes.fromhex('42'), '424242ff'),
                                   (10, 24, bytes.fromhex('80112233'), '332211ff'),
                                   (11, 8, bytes.fromhex('8042'), '424242ff')):
    for map_depth, entry in ((15, '630c'), (16, '4404'), (24, '010203'),
                             (32, '01020304')):
        ident = b'xyz' if (typ + map_depth) % 2 else b''
        data = header(typ=typ, depth=depth, flag=1, length=1,
                      map_depth=map_depth, ident=len(ident)) + ident + bytes.fromhex(entry) + sample
        add(f'unused-map-type-{typ}-entry-{map_depth}-id-{len(ident)}', data, [pixel])
add('unused-map-truncated-after-id', header(flag=1, length=1, map_depth=32, ident=2) + b'ab' + b'\x01\x02', error='TruncatedColorMap')
add('unused-map-range-overflow', header(flag=1, first=65535, length=2, map_depth=24), error='ColorMapRange')
add('unused-map-invalid-depth-before-range', header(flag=1, first=65535, length=2, map_depth=8), error='InvalidColorMap')
# N-1: the attached map never controls a true-colour image's descriptor policy.
add('unused-map-32-entry-24-image-attr-eight',
    header(typ=2, depth=24, flag=1, length=1, map_depth=32, desc=0x28) +
    bytes.fromhex('01020304') + bytes.fromhex('112233'), error='InvalidAttributeBits')
add('unused-map-24-entry-32-image-attr-eight',
    header(typ=2, depth=32, flag=1, length=1, map_depth=24, desc=0x28) +
    bytes.fromhex('010203') + bytes.fromhex('11223344'), ['33221144'])

# B2: bit replication, distinct channels, and both palette entry widths.
add('raw-16-mixed-0444', header(depth=16) + bytes.fromhex('4404'), ['081021ff'])
add('raw-15-mixed-0c63', header(depth=15) + bytes.fromhex('630c'), ['181818ff'])
add('raw-16-distinct-453e', header(depth=16) + bytes.fromhex('3e45'), ['8c4af7ff'])
add('palette-15-mixed-0c63', header(typ=1, depth=8, flag=1, length=1, map_depth=15) + bytes.fromhex('630c00'), ['181818ff'])
add('palette-16-mixed-0444', header(typ=1, depth=8, flag=1, length=1, map_depth=16) + bytes.fromhex('440400'), ['081021ff'])
add('palette-16-distinct-453e', header(typ=1, depth=8, flag=1, length=1, map_depth=16) + bytes.fromhex('3e4500'), ['8c4af7ff'])

# S1: map starts after ID, including the maximum ID size; truncation at its boundary.
add('indexed-map-after-id-3', header(typ=1, depth=8, flag=1, length=1, map_depth=24, ident=3) + b'abc' + bytes.fromhex('10203000'), ['302010ff'])
add('indexed-map-after-id-255', header(typ=1, depth=8, flag=1, length=1, map_depth=24, ident=255) + b'Z' * 255 + bytes.fromhex('10203000'), ['302010ff'])
add('indexed-map-short-after-id', header(typ=1, depth=8, flag=1, length=1, map_depth=24, ident=3) + b'abc' + bytes.fromhex('1020'), error='TruncatedColorMap')

# S2: second absolute index, nonzero first, variable two-/four-byte strides.
for entry_depth, entries, expected in ((15, '00003e45', '8c4af7ff'),
                                        (16, '00004404', '081021ff'),
                                        (32, '0000000011223344', '332211ff')):
    for typ in (1, 9):
        index_sample = bytes.fromhex('8006') if typ == 9 else bytes.fromhex('06')
        add(f'palette-{entry_depth}-index-6-type-{typ}',
            header(typ=typ, depth=8, flag=1, first=5, length=2, map_depth=entry_depth) +
            bytes.fromhex(entries) + index_sample, [expected])
add('palette-16-index16-second', header(typ=1, depth=16, flag=1, first=255, length=2, map_depth=16) + bytes.fromhex('000044040001'), ['081021ff'])

# S3: flag-zero coherence precedes range; flag-one non-indexed range checked above.
add('map-range-with-flag-zero', header(typ=1, depth=8, flag=0, first=65535, length=2, map_depth=24), error='InvalidColorMap')

# N2/N6 and unpinned error-order pairs.
for typ in (4, 8, 12, 32):
    add(f'unsupported-type-{typ}', header(typ=typ), error='UnsupportedImageType')
for bits in (0x80, 0xc0):
    add(f'interleave-{bits}', header(desc=bits), error='UnsupportedInterleave')
add('dimensions-before-interleave', header(width=0, desc=0x60), error='InvalidDimensions')
add('depth-before-attribute', header(depth=8, desc=0x21), error='UnsupportedPixelDepth')
add('rle-literal-crosses-row', header(typ=10, width=2, height=2, desc=0x20) + bytes.fromhex('030000ff00ff00ff0000010101'),
    ['ff0000ff', '00ff00ff', '0000ffff', '010101ff'])
add('gray-origin-bottom-right', header(typ=3, depth=8, width=2, height=2, desc=0x10) + bytes.fromhex('01020304'),
    ['040404ff', '030303ff', '020202ff', '010101ff'])
add('indexed-origin-top-right', header(typ=1, depth=8, width=2, flag=1, length=2, map_depth=24, desc=0x30) + bytes.fromhex('0000ff00ff000001'),
    ['00ff00ff', 'ff0000ff'])
add('attribute-rle-indexed-15-one', header(typ=9, depth=8, flag=1, length=1, map_depth=15, desc=0x21), error='InvalidAttributeBits')
add('attribute-rle-indexed-24-one', header(typ=9, depth=8, flag=1, length=1, map_depth=24, desc=0x21), error='InvalidAttributeBits')

assert len({v['name'] for v in vectors}) == len(vectors)
path.write_text(json.dumps(vectors, indent=2) + '\n')
print(f'{len(vectors)} vectors, {len(vectors) - 118} added')
