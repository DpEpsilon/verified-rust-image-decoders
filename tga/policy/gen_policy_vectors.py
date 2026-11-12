#!/usr/bin/env python3
"""Generate seeded, provenance-carrying TGA policy vectors.

Inputs come from three sources, recorded per vector in `provenance`:
  * `frozen:<name>`        an unchanged frozen TGA-CORE-SPEC 1.0 vector input;
  * `hand:<description>`   a synthetic file built here from explicit fields;
  * `seeded:<seed>:<name>` a random option-targeted mutation of a frozen vector.
No corpus bytes are used.  Expected results are computed by policy_oracle.py and
re-checked; the closed `recovered` event list and the Recommended invariant are
asserted independently of the per-profile grid.
Output: POLICY-VECTORS.json {seed, generator, profiles: {name: [vector]}, custom: [vector]}.
"""
import hashlib
import json
from pathlib import Path
import random
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import policy_oracle as O

SEED = 20261111
FOOTER = bytes(8) + O.FOOTER_SIGNATURE


def tga(itype, depth, w, h, data, desc=0, flag=0, first=0, length=0, edepth=0,
        image_id=b'', cmap=b'', tail=b''):
    head = bytes([len(image_id), flag, itype]) + first.to_bytes(2, 'little') + \
        length.to_bytes(2, 'little') + bytes([edepth]) + bytes(4) + \
        w.to_bytes(2, 'little') + h.to_bytes(2, 'little') + bytes([depth, desc])
    return head + image_id + cmap + data + tail


cases = []  # (name, bytes, provenance)


def add(name, b, provenance):
    assert len(b) <= 8192, name
    cases.append((name, bytes(b), provenance))


def hand(name, b):
    add(name, b, 'hand:' + name)


RGB = bytes([0x10, 0x20, 0x30, 0x40, 0x50, 0x60])           # two BGR samples
BGRA = bytes([0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x00])  # two BGRA samples, A=0x40/0x00
W555 = (3 << 10 | 3 << 5 | 3).to_bytes(2, 'little') + (0x8000 | 29 << 10 | 1 << 5 | 6).to_bytes(2, 'little')
MAP24 = bytes([0, 0, 255, 0, 255, 0, 255, 0, 0])              # red, green, blue
MAP32 = bytes([0, 0, 255, 0x80, 0, 255, 0, 0x00])             # red a=0x80, green a=0
MAP16 = (3 << 10 | 7 << 5 | 30).to_bytes(2, 'little') + (0x8000 | 31 << 10).to_bytes(2, 'little')

# trailing
hand('trailing-junk', tga(2, 24, 2, 1, RGB, tail=b'JUNK'))
hand('trailing-footer', tga(2, 24, 2, 1, RGB, tail=b'ext' + FOOTER))
hand('trailing-lone-signature', tga(2, 24, 2, 1, RGB, tail=O.FOOTER_SIGNATURE))
# rle_overrun
hand('rle-repeat-overrun', tga(10, 24, 2, 1, b'\x82' + RGB[:3]))
hand('rle-raw-overrun-whole-packet', tga(10, 24, 2, 1, b'\x02' + RGB + b'\x01\x02\x03'))
hand('rle-raw-overrun-short-packet', tga(10, 24, 2, 1, b'\x02' + RGB))
hand('rle-raw-overrun-footer', tga(10, 24, 2, 1, b'\x02' + RGB + b'\x01\x02\x03', tail=FOOTER))
hand('rle-overrun-after-row', tga(10, 24, 2, 2, b'\x81' + RGB[:3] + b'\x81' + RGB[3:]))
hand('rle-complete-exact', tga(10, 24, 2, 2, b'\x81' + RGB[:3] + b'\x01' + RGB))
hand('rle-indexed-overrun-invalid-tail-index',
     tga(9, 8, 2, 1, b'\x02\x00\x01\x09', flag=1, length=3, edepth=24, cmap=MAP24))
hand('rle-truncated-control', tga(10, 24, 2, 1, b'\x80' + RGB[:3]))
# attribute_bits
hand('attr-32-count4', tga(2, 32, 2, 1, BGRA, desc=0x24))
hand('attr-32-count1', tga(2, 32, 2, 1, BGRA, desc=0x01))
hand('attr-16-count8', tga(2, 16, 2, 1, W555, desc=0x08))
hand('attr-24-count8', tga(2, 24, 2, 1, RGB, desc=0x08))
hand('attr-15-count1', tga(2, 15, 2, 1, W555, desc=0x01))
hand('attr-map24-count8', tga(1, 8, 3, 1, b'\x00\x01\x02', desc=0x08, flag=1, length=3, edepth=24, cmap=MAP24))
hand('attr-map16-count8', tga(1, 8, 2, 1, b'\x00\x01', desc=0x08, flag=1, length=2, edepth=16, cmap=MAP16))
hand('attr-grey8-count8', tga(3, 8, 2, 1, b'\x11\x22', desc=0x08))
hand('attr-grey16-count0', tga(3, 16, 2, 1, b'\x11\x40\x22\x00'))
hand('attr-grey16-count3', tga(3, 16, 2, 1, b'\x11\x40\x22\x00', desc=0x03))
hand('attr-grey16-count8', tga(3, 16, 2, 1, b'\x11\x40\x22\x00', desc=0x08))
# map_header
hand('map-absent-stray-depth', tga(2, 24, 2, 1, RGB, edepth=24))
hand('map-absent-stray-first-length', tga(2, 24, 2, 1, RGB, first=744, length=61954, edepth=255))
hand('map-absent-indexed', tga(1, 8, 2, 1, b'\x00\x01', length=3, edepth=24))
hand('map-flag2-truecolour', tga(2, 24, 2, 1, RGB, flag=2, length=1, edepth=24, cmap=b'\x01\x02\x03'))
hand('map-flag2-indexed', tga(1, 8, 2, 1, b'\x00\x01', flag=2, length=3, edepth=24, cmap=MAP24))
hand('map-present-length0-truecolour', tga(2, 24, 2, 1, RGB, flag=1, edepth=24))
hand('map-present-length0-baddepth', tga(2, 24, 2, 1, RGB, flag=1, edepth=8))
hand('map-present-baddepth-nonempty', tga(2, 24, 2, 1, RGB, flag=1, length=1, edepth=8, cmap=b'\x00'))
hand('map-present-range-truecolour', tga(2, 24, 2, 1, RGB, flag=1, first=65535, length=2, edepth=24, cmap=bytes(6)))
hand('map-present-length0-indexed', tga(1, 8, 2, 1, b'\x00\x01', flag=1, edepth=32))
# depths
hand('depth-true15', tga(2, 15, 2, 1, W555))
hand('depth-index16-map24', tga(1, 16, 2, 1, b'\x00\x00\x02\x00', flag=1, length=3, edepth=24, cmap=MAP24))
hand('depth-index16-map15', tga(1, 16, 2, 1, b'\x00\x00\x01\x00', flag=1, length=2, edepth=15, cmap=MAP16))
# alpha
hand('alpha-32-count0', tga(2, 32, 2, 1, BGRA))
hand('alpha-32-count8', tga(2, 32, 2, 1, BGRA, desc=0x08))
hand('alpha-map32-count0', tga(1, 8, 2, 1, b'\x00\x01', flag=1, length=2, edepth=32, cmap=MAP32))
hand('alpha-rle32-count0', tga(10, 32, 2, 1, b'\x81' + BGRA[:4]))
# map16_alpha and map5
hand('map16-count1', tga(1, 8, 2, 1, b'\x00\x01', desc=0x01, flag=1, length=2, edepth=16, cmap=MAP16))
hand('map16-count0', tga(1, 8, 2, 1, b'\x00\x01', flag=1, length=2, edepth=16, cmap=MAP16))
hand('map15', tga(1, 8, 2, 1, b'\x00\x01', flag=1, length=2, edepth=15, cmap=MAP16))
# rgb5
hand('rgb16-count0', tga(2, 16, 2, 1, W555))
hand('rgb16-count1-bit15', tga(2, 16, 2, 1, W555, desc=0x01))
hand('rgb15-rle', tga(10, 15, 2, 1, b'\x81' + W555[:2]))
# map_index and index_range
hand('index-first1', tga(1, 8, 2, 1, b'\x01\x02', flag=1, first=1, length=2, edepth=24, cmap=MAP24[:6]))
hand('index-below-first', tga(1, 8, 2, 1, b'\x00\x01', flag=1, first=1, length=2, edepth=24, cmap=MAP24[:6]))
hand('index-beyond-map24', tga(1, 8, 2, 1, b'\x00\x05', flag=1, length=3, edepth=24, cmap=MAP24))
hand('index-beyond-map32', tga(1, 8, 2, 1, b'\x00\x05', flag=1, length=2, edepth=32, cmap=MAP32))
hand('index-beyond-rle', tga(9, 8, 3, 1, b'\x82\x07', flag=1, length=3, edepth=24, cmap=MAP24))
hand('index-first-ge-length-zero-based', tga(1, 8, 2, 1, b'\x03\x04', flag=1, first=3, length=2, edepth=24, cmap=MAP24[:6]))
# orientation regression (all profiles honour bits 4 and 5)
hand('orient-right-top', tga(2, 24, 2, 2, RGB + RGB[::-1], desc=0x30))
hand('orient-right-bottom', tga(2, 24, 2, 2, RGB + RGB[::-1], desc=0x10))
# image-rs defects reproduced only by ImageCrateCompat
hand('interleave-bits-set', tga(2, 24, 2, 1, RGB, desc=0x40))
hand('image-type-0-as-grey', tga(0, 8, 2, 1, b'\x11\x22'))
hand('image-type-7-as-grey16', tga(7, 16, 2, 1, b'\x11\x40\x22\x00', desc=0x08))
# truncated raw pixels stay an error everywhere
hand('raw-truncated', tga(2, 24, 2, 1, RGB[:5]))

# unchanged frozen inputs (small ones)
frozen = json.loads(O.FROZEN.read_text())
for v in frozen:
    if 'input_repeat' in v or len(v['input_hex']) > 2 * 8192:
        continue
    add('frozen-' + v['name'], bytes.fromhex(v['input_hex']), 'frozen:' + v['name'])

# seeded option-targeted mutations of frozen vectors that Strict accepts
rng = random.Random(SEED)
accepted = [v for v in frozen if 'error' not in v['expected'] and 'input_repeat' not in v
            and len(v['input_hex']) <= 2 * 4096]


def mutate(b):
    a = bytearray(b)
    choice = rng.choice(['attr', 'flag', 'mapfields', 'tail', 'rlecontrol', 'first', 'depth'])
    if choice == 'attr':
        a[17] = (a[17] & 0xf0) | rng.randrange(16)
        what = f'descriptor={a[17]:#04x}'
    elif choice == 'flag':
        a[1] = rng.choice([0, 1, 2, 255])
        what = f'map_flag={a[1]}'
    elif choice == 'mapfields' and a[1] == 0:
        a[7] = rng.choice([8, 15, 16, 24, 32, 255])
        a[5:7] = rng.choice([0, 1, 300]).to_bytes(2, 'little')
        what = f'absent-map length={a[5] | a[6] << 8} depth={a[7]}'
    elif choice == 'tail':
        extra = bytes(rng.randrange(256) for _ in range(rng.choice([1, 7, 25, 26])))
        a += extra
        what = f'append {len(extra)} random bytes'
    elif choice == 'rlecontrol' and a[2] >= 9:
        p = 18 + a[0] + (a[5] | a[6] << 8) * ((a[7] + 7) // 8) * (a[1] == 1)
        if p < len(a):
            a[p] = rng.choice([0x7f, 0xff, a[p] | 0x40])
        what = f'rle first control={a[p] if p < len(a) else None}'
    elif choice == 'first' and a[1] == 1:
        a[3:5] = rng.choice([0, 1, 2, 255]).to_bytes(2, 'little')
        what = f'map_first={a[3] | a[4] << 8}'
    else:
        a[16] = rng.choice([8, 15, 16, 24, 32])
        what = f'pixel_depth={a[16]}'
    return bytes(a), what


seen = {hashlib.sha256(b).digest() for _, b, _ in cases}
attempts = 0
while sum(1 for _, _, p in cases if p.startswith('seeded:')) < 80 and attempts < 2000:
    attempts += 1
    v = rng.choice(accepted)
    b, what = mutate(bytes.fromhex(v['input_hex']))
    d = hashlib.sha256(b).digest()
    if d in seen:
        continue
    seen.add(d)
    add(f'seeded-{len(seen)}-{v["name"]}', b, f'seeded:{SEED}:{v["name"]}:{what}')

# per-profile grid
profiles = {}
for profile in O.PROFILES:
    rows = []
    for name, b, prov in cases:
        row = {'name': name, 'provenance': prov, 'input_hex': b.hex(),
               'expected': O.actual(b, profile, with_recovery=True)}
        O.check_vector(row, profile)
        rows.append(row)
    profiles[profile] = rows


def expected(profile, name):
    return next(r['expected'] for r in profiles[profile] if r['name'] == name)


# Recommended invariant: never changes a Strict-accepted result except the flag
for name, b, _ in cases:
    s = O.actual(b, 'Strict')
    if 'error' not in s:
        r = O.actual(b, 'Recommended', with_recovery=True)
        assert r.pop('recovered') is False and r == s, name
# closed event list: positive witnesses
for event, (profile, name) in {
        'rle-clip-repeat': ('Recommended', 'rle-repeat-overrun'),
        'rle-clip-raw': ('ImageCrateCompat-0.25.10', 'rle-raw-overrun-short-packet'),
        'index-black-opaque': ('GdkCompat-2.42.12', 'index-beyond-map24'),
        'index-black-transparent': ('GdkCompat-2.42.12', 'index-beyond-map32'),
        'index-black-rle': ('GdkCompat-2.42.12', 'index-beyond-rle')}.items():
    assert expected(profile, name).get('recovered') is True, (event, expected(profile, name))
assert expected('GdkCompat-2.42.12', 'index-beyond-map32')['rgba_runs'][-1]['hex'] == '00000000'
assert expected('GdkCompat-2.42.12', 'index-beyond-map24')['rgba_runs'][-1]['hex'] == '000000ff'
# negatives: tolerances that do not set the flag
for profile in O.PROFILES:
    for name in ('rle-complete-exact', 'trailing-footer'):
        assert expected(profile, name).get('recovered') is False, (profile, name)
for name in ('trailing-junk', 'attr-32-count4', 'map-absent-stray-depth'):
    assert expected('Recommended', name).get('recovered') is False, name

# internal single-option probes: one discriminating witness per non-Strict value
custom = []
undiscriminated = []
for key, values in O.OPTIONS.items():
    for value in values[1:]:
        policy = dict(O.STRICT, **{key: value})
        chosen = next(((n, b, p) for n, b, p in cases
                       if O.actual(b, policy) != O.actual(b, O.STRICT)), None)
        if chosen is None:
            undiscriminated.append((key, value))
            continue
        n, b, p = chosen
        row = {'name': f'{key}-{value}-{n}', 'provenance': p, 'input_hex': b.hex(),
               'policy': policy, 'expected': O.actual(b, policy, with_recovery=True)}
        O.check_vector(row, policy)
        custom.append(row)
# coupled fields: clipped raw packet consumption is visible only with FooterOnly
for name, changes in (('rle-raw-overrun-whole-packet', {'rle_overrun': 'ClipPacket'}),
                      ('rle-raw-overrun-whole-packet', {'rle_overrun': 'ClipSamples'}),
                      ('rle-raw-overrun-footer', {'rle_overrun': 'ClipPacket'}),
                      ('rle-raw-overrun-footer', {'rle_overrun': 'ClipSamples'}),
                      ('rle-indexed-overrun-invalid-tail-index', {'rle_overrun': 'ClipPacket'}),
                      ('index-first-ge-length-zero-based', {'map_index': 'ZeroBased'}),
                      ('map-present-length0-indexed', {'map_header': 'Gdk', 'index_range': 'Black'}),
                      ('attr-map16-count8', {'attribute_bits': 'Lenient'}),
                      ('alpha-32-count8', {'alpha': 'Stored'})):
    b, p = next((b, p) for n, b, p in cases if n == name)
    policy = dict(O.STRICT, **changes)
    row = {'name': 'coupled-' + name + '-' + '-'.join(changes.values()), 'provenance': p,
           'input_hex': b.hex(), 'policy': policy,
           'expected': O.actual(b, policy, with_recovery=True)}
    O.check_vector(row, policy)
    custom.append(row)
assert not undiscriminated, undiscriminated
out = {'seed': SEED, 'generator': 'gen_policy_vectors.py', 'profiles': profiles, 'custom': custom}
(HERE / 'POLICY-VECTORS.json').write_text(json.dumps(out, indent=1) + '\n')
print('generated', len(cases), 'vectors per profile,', len(custom), 'internal probes;',
      sum(1 for _, _, p in cases if p.startswith('seeded:')), 'seeded mutations')
