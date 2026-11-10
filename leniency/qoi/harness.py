#!/usr/bin/env python3
"""QOI adapter: synthetic seeds from a small spec-based encoder, header/stream/end-marker mutations, decoder set."""
import random, struct, sys
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'ext'))
import common  # noqa: E402

NAME, EXT = 'qoi', 'qoi'
R = common.RUNNER
DECODERS = {
    'image': (R, ['qoi:image']),
    'glycin': (R, ['qoi:glycin']),
    'qoi-crate': (R, ['qoi:qoi-crate']),
    'rapid-qoi': (R, ['qoi:rapid']),
    'zune-qoi': (R, ['qoi:zune']),
    'ref-c': (common.BIN / 'qoi-ref', []),
}
UNAVAILABLE = {'gdk-pixbuf': 'no QOI module installed (loaders: ani bmp gif icns ico pnm qtif tga tiff xbm xpm)'}
# image's QoiDecoder wraps the `qoi` crate, so image, glycin and qoi-crate are one implementation = one vote.
GROUPS = {'image': 'image-rs', 'glycin': 'image-rs', 'qoi-crate': 'image-rs'}
END = b'\0\0\0\0\0\0\0\1'


def header(w, h, ch=4, cs=0, magic=b'qoif'):
    return magic + struct.pack('>IIBB', w, h, ch, cs)


def encode(pixels, w, h, ch=4, cs=0):
    """Reference algorithm from the QOI specification. pixels: list of (r,g,b,a)."""
    out = bytearray(header(w, h, ch, cs))
    idx = [(0, 0, 0, 0)] * 64
    prev, run = (0, 0, 0, 255), 0
    for i, px in enumerate(pixels):
        if px == prev:
            run += 1
            if run == 62 or i == len(pixels) - 1:
                out.append(0xC0 | (run - 1)); run = 0
            continue
        if run:
            out.append(0xC0 | (run - 1)); run = 0
        h_ = (px[0] * 3 + px[1] * 5 + px[2] * 7 + px[3] * 11) % 64
        if idx[h_] == px:
            out.append(h_)
        else:
            idx[h_] = px
            if px[3] == prev[3]:
                dr, dg, db = ((px[k] - prev[k] + 128) % 256 - 128 for k in range(3))
                if -2 <= dr <= 1 and -2 <= dg <= 1 and -2 <= db <= 1:
                    out.append(0x40 | (dr + 2) << 4 | (dg + 2) << 2 | (db + 2))
                else:
                    dr_dg, db_dg = dr - dg, db - dg
                    if -32 <= dg <= 31 and -8 <= dr_dg <= 7 and -8 <= db_dg <= 7:
                        out += bytes([0x80 | (dg + 32), (dr_dg + 8) << 4 | (db_dg + 8)])
                    else:
                        out += bytes([0xFE, *px[:3]])
            else:
                out += bytes([0xFF, *px])
        prev = px
    return bytes(out) + END


def make_pixels(kind, w, h, ch, rng):
    n = w * h
    a = lambda v: v if ch == 4 else 255
    if kind == 'flat':
        return [(10, 200, 30, a(255))] * n
    if kind == 'gradient':
        return [((x * 3 + y) % 256, (x + y * 2) % 256, (x * y) % 256, a(255)) for y in range(h) for x in range(w)]
    if kind == 'noise':
        return [(rng.randrange(256), rng.randrange(256), rng.randrange(256), a(rng.randrange(256))) for _ in range(n)]
    if kind == 'palette':
        pal = [(rng.randrange(256), rng.randrange(256), rng.randrange(256), a(255)) for _ in range(5)]
        return [pal[(i * 7 + i // 3) % 5] for i in range(n)]
    if kind == 'alpha':
        return [(200, 100, 50, a((i * 37) % 256)) for i in range(n)]
    if kind == 'runs':
        return [(i // 70 * 40 % 256, 5, 9, a(255)) for i in range(n)]
    if kind == 'transparent':      # stream starts with (0,0,0,0): collides with the zero-initialised index entry
        return [(0, 0, 0, 0)] * (n // 2) + [(0, 0, 0, a(255))] * (n - n // 2)
    if kind == 'wrap':             # diffs that wrap modulo 256
        return [((i * 255) % 256, (i * 3) % 256, (255 - i * 2) % 256, a(255)) for i in range(n)]
    return [(0, 0, 0, a(255))] * n


def raw_stream(ops, w, h, ch=4, cs=0, end=END):
    return header(w, h, ch, cs) + b''.join(ops) + end


def seeds():
    rng = random.Random(5)
    out = []
    for (w, h) in ((1, 1), (3, 2), (8, 8), (70, 2), (5, 13)):
        for ch in (3, 4):
            for kind in ('flat', 'gradient', 'noise', 'palette', 'alpha', 'runs', 'transparent', 'wrap'):
                if w * h < 6 and kind in ('palette', 'runs', 'wrap', 'transparent', 'alpha'):
                    continue
                if (w, h) not in ((8, 8), (3, 2), (70, 2)) and kind in ('palette', 'alpha', 'runs', 'wrap', 'transparent'):
                    continue
                px = make_pixels(kind, w, h, ch, rng)
                out.append((encode(px, w, h, ch, rng.randrange(2)), {'name': f'{kind}-{w}x{h}-c{ch}', 'header_len': 14, 'w': w, 'h': h, 'ch': ch}))
    # hand-built streams probing initial state and op corner cases
    hb = {
        'index0-first': ([bytes([0x00]), bytes([0xFE, 1, 2, 3])], 2, 1, 4),         # INDEX 0 before any pixel is written
        'run62': ([bytes([0xC0 | 61])], 62, 1, 4),                                    # longest legal run, initial pixel (0,0,0,255)
        'run-then-rgb': ([bytes([0xC0 | 1]), bytes([0xFE, 9, 9, 9])], 3, 1, 4),
        'rgb-keeps-alpha': ([bytes([0xFF, 1, 2, 3, 77]), bytes([0xFE, 4, 5, 6])], 2, 1, 4),
        'rgba-in-3ch': ([bytes([0xFF, 1, 2, 3, 77]), bytes([0xFE, 4, 5, 6])], 2, 1, 3),
        'diff-extremes': ([bytes([0x40]), bytes([0x40 | 0x3F]), bytes([0x40 | 0x15])], 3, 1, 4),
        'luma-extremes': ([bytes([0x80, 0x00]), bytes([0x80 | 0x3F, 0xFF])], 2, 1, 4),
        'index-hit': ([bytes([0xFE, 1, 2, 3]), bytes([0xFE, 9, 9, 9]), bytes([(1 * 3 + 2 * 5 + 3 * 7 + 255 * 11) % 64])], 3, 1, 4),
    }
    for name, (ops, w, h, ch) in hb.items():
        out.append((raw_stream(ops, w, h, ch), {'name': 'hand-' + name, 'header_len': 14, 'w': w, 'h': h, 'ch': ch}))
    return out


def parse_ops(data, w, h):
    """Return [(start, end)] chunk boundaries for the pixel stream of a seed; stops at the pixel count."""
    p, n, ops = 14, 0, []
    total = w * h
    while n < total and p < len(data) - 8:
        t = data[p]
        size = 4 if t == 0xFF else 5 if False else 1
        if t == 0xFE:
            size = 4
        elif t == 0xFF:
            size = 5
        elif t >> 6 == 2:
            size = 2
        run = (t & 0x3F) + 1 if t >> 6 == 3 and t < 0xFE else 1
        ops.append((p, p + size))
        p += size
        n += run
    return ops


def mutations(data, meta, rng):
    w, h, ch = meta['w'], meta['h'], meta['ch']
    cs = data[13]
    body = data[14:-8]
    for mg in (b'QOIF', b'qoi\0', b'qoie', b'\0oif', b'qoif'[::-1], b'qoif'[:3] + b' '):
        yield mg + data[4:], {'rule': 'magic', 'field': 'magic', 'old': 'qoif', 'new': mg.decode('latin1')}
    yield data[1:], {'rule': 'magic', 'field': 'magic', 'old': 'qoif', 'new': 'first byte dropped'}
    for name, pos, old in (('width', 4, w), ('height', 8, h)):
        for v in sorted({0, 1, 2, old - 1, old + 1, old * 2, 255, 256, 65535, 65536, 2**24, 2**28 - 1, 2**31 - 1, 2**31, 2**32 - 1} - {old}):
            yield data[:pos] + struct.pack('>I', v) + data[pos + 4:], {'rule': 'header-' + name, 'field': name, 'old': old, 'new': v}
    for ww, hh in ((0, 0), (2**32 - 1, 2**32 - 1), (2**31, 2), (65536, 65536), (400_000_000 // 4 + 1, 4), (2**30, 1), (3, 2**30)):
        yield data[:4] + struct.pack('>II', ww, hh) + data[12:], {'rule': 'header-both', 'field': 'width+height', 'old': f'{w}x{h}', 'new': f'{ww}x{hh}'}
    for v in (0, 1, 2, 3, 4, 5, 8, 16, 255):
        if v != ch:
            yield data[:12] + bytes([v]) + data[13:], {'rule': 'header-channels', 'field': 'channels', 'old': ch, 'new': v}
    for v in (0, 1, 2, 3, 127, 255):
        if v != cs:
            yield data[:13] + bytes([v]) + data[14:], {'rule': 'header-colorspace', 'field': 'colorspace', 'old': cs, 'new': v}
    # end marker
    yield data[:-8], {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'removed'}
    yield data[:-1], {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'last byte removed'}
    yield data[:-7], {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'one byte left'}
    yield data[:-9], {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'marker removed and a pixel byte cut'}
    for i in range(8):
        for v in (0x00, 0x01, 0xFF, 0x80):
            if data[-8 + i] != v:
                yield data[:-8 + i] + bytes([v]) + data[-7 + i:], {'rule': 'end-marker', 'field': f'marker byte {i}', 'old': data[-8 + i], 'new': v}
    yield data[:-8] + b'\0' * 8, {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'eight zero bytes'}
    yield data[:-8] + b'\0' * 7 + b'\1' + b'\0' * 7 + b'\1', {'rule': 'end-marker', 'field': 'marker', 'old': 'single', 'new': 'doubled'}
    yield data[:-8] + b'\1' + b'\0' * 7, {'rule': 'end-marker', 'field': 'marker', 'old': '00*7 01', 'new': 'reversed'}
    # stream edits at chunk granularity
    ops = parse_ops(data, w, h)
    pick = sorted({0, len(ops) // 2, len(ops) - 1} & set(range(len(ops))))
    for i in pick:
        s, e = ops[i]
        yield data[:s] + data[e:], {'rule': 'stream-op-deleted', 'field': f'op {i}', 'old': data[s], 'new': 'deleted'}
        yield data[:e] + data[s:e] + data[e:], {'rule': 'stream-op-duplicated', 'field': f'op {i}', 'old': data[s], 'new': 'duplicated'}
        if e - s > 1:
            yield data[:s + 1] + data[e:], {'rule': 'stream-op-cut', 'field': f'op {i}', 'old': e - s, 'new': 1}
        for tag in (0x00, 0x3F, 0x40, 0x7F, 0x80, 0xBF, 0xC0, 0xC0 | 61, 0xC0 | 62, 0xC0 | 63, 0xFD, 0xFE, 0xFF):
            if data[s] != tag:
                yield data[:s] + bytes([tag]) + data[s + 1:], {'rule': 'stream-op-tag', 'field': f'op {i}', 'old': data[s], 'new': tag}
    yield data[:-8] + bytes([0xC0 | 61]) + END, {'rule': 'stream-excess', 'field': 'extra run', 'old': 0, 'new': '62 extra pixels'}
    yield data[:-8] + bytes([0xFE, 1, 2, 3]) + END, {'rule': 'stream-excess', 'field': 'extra rgb op', 'old': 0, 'new': 1}
    yield data[:-8] + bytes([0xC0 | 61]) * 3 + END, {'rule': 'stream-excess', 'field': 'extra runs', 'old': 0, 'new': 3}
    if ops:
        s, e = ops[-1]
        yield data[:s] + END, {'rule': 'stream-short', 'field': 'last op', 'old': 'present', 'new': 'removed, marker kept'}
        yield data[:s] + bytes([0xC0 | 61]) + END, {'rule': 'stream-run-overrun', 'field': 'last op', 'old': data[s], 'new': 'run of 62'}
    yield header(w, h, ch, cs) + END, {'rule': 'stream-short', 'field': 'all ops', 'old': len(body), 'new': 0}
    yield data[:14] + bytes(len(body)), {'rule': 'stream-short', 'field': 'ops zeroed, no marker', 'old': len(body), 'new': 'INDEX 0 only'}
    for pad in (1, 2, 3):
        yield data[:-8] + b'\0' * pad + END, {'rule': 'stream-padding', 'field': 'zero bytes before marker', 'old': 0, 'new': pad}


if __name__ == '__main__':
    common.main(sys.modules[__name__])
