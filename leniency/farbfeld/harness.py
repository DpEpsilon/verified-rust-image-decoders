#!/usr/bin/env python3
"""farbfeld adapter: synthetic seeds, header/body mutations, decoder set."""
import random, struct, sys
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'ext'))
import common  # noqa: E402

NAME, EXT = 'farbfeld', 'ff'
R = common.RUNNER
DECODERS = {
    'image': (R, ['farbfeld:image']),
    'glycin': (R, ['farbfeld:glycin']),
    'farbfeld-crate': (R, ['farbfeld:crate']),
    'oracle-spec': (sys.executable, ['-I', str(HERE.parent / 'ext' / 'ff_oracle.py')]),
}
UNAVAILABLE = {'zune-farbfeld 0.5.2': 'unusable as published: decode() always fails with "Too small output buffer size" (it allocates width*height*4 u16 but requires twice that), and decode_into with a doubled buffer consumes twice the body; runner selector farbfeld:zune is kept in ext/src/main.rs but not counted',
               'gdk-pixbuf': 'no farbfeld module installed (loaders: ani bmp gif icns ico pnm qtif tga tiff xbm xpm)',
               'suckless farbfeld tools': 'not installed; source is git.suckless.org, outside the allowed network'}
GROUPS = {'image': 'image-rs', 'glycin': 'image-rs'}

MAGIC = b'farbfeld'


def build(w, h, body, magic=MAGIC):
    return magic + struct.pack('>II', w, h) + body


def body_for(kind, w, h, rng):
    n = w * h
    if kind == 'ramp':
        v = [(i * 257 + c * 4097) & 0xFFFF for i in range(n) for c in range(4)]
    elif kind == 'random':
        v = [rng.randrange(65536) for _ in range(n * 4)]
    elif kind == 'extremes':   # byte-order and 16->8 rounding probes: 0x00ff, 0xff00, 0x0080, 0x7fff, 0x8000, 0xff7f
        pool = [0, 65535, 0x00FF, 0xFF00, 0x0080, 0x007F, 0x7FFF, 0x8000, 0xFF7F, 0x0101, 0x8081]
        v = [pool[(i * 3 + 1) % len(pool)] for i in range(n * 4)]
    elif kind == 'alpha0':     # transparent pixels with non-zero colour: premultiplication probe
        v = [x for i in range(n) for x in (60000, 30000, 1000, 0)]
    elif kind == 'alpha-half':
        v = [x for i in range(n) for x in (60000, 30000, 1000, 0x8000)]
    else:
        v = [x for i in range(n) for x in (i * 4000 % 65536,) * 3 + (65535,)]
    return struct.pack('>%dH' % len(v), *v)


def seeds():
    rng = random.Random(9)
    out = []
    for (w, h) in ((1, 1), (2, 2), (3, 5), (8, 1), (1, 8), (7, 7)):
        for kind in ('ramp', 'random', 'extremes', 'alpha0', 'alpha-half', 'gray'):
            if (w * h < 4 and kind in ('ramp', 'gray')) or (w, h) not in ((1, 1), (3, 5), (2, 2)) and kind in ('alpha0', 'alpha-half'):
                continue
            out.append((build(w, h, body_for(kind, w, h, rng)), {'name': f'{kind}-{w}x{h}', 'header_len': 16, 'w': w, 'h': h}))
    return out


def mutations(data, meta, rng):
    w, h = meta['w'], meta['h']
    body = data[16:]
    for mg in (b'Farbfeld', b'FARBFELD', b'farbfelD', b'farbfel\0', b'farbfel ', b'\0' * 8, b'farbfeed', b'qoif\0\0\0\0', b'farbfeld'[::-1]):
        yield mg + data[8:], {'rule': 'magic', 'field': 'magic', 'old': 'farbfeld', 'new': mg.decode('latin1')[:8]}
    yield data[1:], {'rule': 'magic', 'field': 'magic', 'old': 'farbfeld', 'new': 'dropped first byte'}
    yield b' ' + data, {'rule': 'magic', 'field': 'magic', 'old': 'farbfeld', 'new': 'leading space'}
    yield data[:8] + data[12:16] + data[8:12] + data[16:], {'rule': 'swap-width-height', 'field': 'width/height', 'old': f'{w}x{h}', 'new': f'{h}x{w}'}
    vals = [0, 1, 2, w - 1, w + 1, h, w * h, 255, 256, 65535, 65536, 2**24, 2**31 - 1, 2**31, 2**32 - 1]
    for name, pos, other in (('width', 8, h), ('height', 12, w)):
        old = w if name == 'width' else h
        for v in vals:
            if 0 <= v < 2**32 and v != old:
                yield data[:pos] + struct.pack('>I', v) + data[pos + 4:], {'rule': 'header-' + name, 'field': name, 'old': old, 'new': v}
        # little-endian encoded value in a big-endian field
        yield data[:pos] + struct.pack('<I', old if old > 255 else old + 256) + data[pos + 4:], {'rule': 'header-' + name, 'field': name, 'old': old, 'new': 'little-endian'}
    for ww, hh in ((65536, 65536), (2**32 - 1, 2**32 - 1), (2**31, 2), (2**16 + 1, 2**16 + 1), (2**32 - 1, 1), (1, 2**32 - 1), (0, 0), (0, h), (w, 0)):
        yield data[:8] + struct.pack('>II', ww, hh) + body, {'rule': 'header-both', 'field': 'width+height', 'old': f'{w}x{h}', 'new': f'{ww}x{hh}'}
    # body shorter / longer than the header declares, in whole pixels and whole rows
    if w * h > 1:
        yield data[:16] + body[:-8], {'rule': 'body-short', 'field': 'body', 'old': len(body), 'new': len(body) - 8}
        yield data[:16] + body[:-7], {'rule': 'body-short', 'field': 'body', 'old': len(body), 'new': len(body) - 7}
        yield data[:16] + body[:len(body) // 2], {'rule': 'body-short', 'field': 'body', 'old': len(body), 'new': len(body) // 2}
    yield data + b'\0' * 8, {'rule': 'body-long', 'field': 'body', 'old': len(body), 'new': len(body) + 8}
    yield data + b'\0' * 7, {'rule': 'body-long', 'field': 'body', 'old': len(body), 'new': len(body) + 7}
    if h > 1:
        yield build(w, h - 1, body), {'rule': 'body-long', 'field': 'height-1 (one spare row)', 'old': h, 'new': h - 1}
    if w > 1:
        yield build(w - 1, h, body), {'rule': 'body-long', 'field': 'width-1 (spare columns)', 'old': w, 'new': w - 1}
    for a in (0, 1, 0x80, 0xFF):   # alpha byte pairs forced
        b = bytearray(body)
        for i in range(0, len(b), 8):
            b[i + 6:i + 8] = bytes([a, a])
        yield data[:16] + bytes(b), {'rule': 'alpha-forced', 'field': 'alpha', 'old': 'seed', 'new': a}


if __name__ == '__main__':
    import types
    common.main(sys.modules[__name__])
