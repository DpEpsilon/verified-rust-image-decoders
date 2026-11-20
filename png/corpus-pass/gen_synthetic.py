#!/usr/bin/env python3
"""Project-authored synthetic PNG files for the M0-corpus cap tests (large multi-monitor screenshots, long thin images,
stored (uncompressed) images near the output cap, the input-cap boundary).

  python3 -I gen_synthetic.py [--out DIR] [--only NAME,...] [--small] [--merge]

The pinned real corpus (~/corpora/png, 21,966 files) has no multi-monitor screenshot, no file above 64 MiB (the fetcher
dropped them) and no stored image near the output cap, so these files are generated here. They are written outside the
repository (default ~/corpora/png-synthetic/files/<sha256>.png, with MANIFEST.tsv beside it) because the largest is
1 GiB; the generator is deterministic, so the SHA-256 column of MANIFEST.tsv pins them. Licence: project-authored, the
repository's MIT licence (no third-party content). `expect` is the Strict outcome by intent (written before any oracle
run from the cap values of ../limits-draft/limits.py), `old_caps` the outcome the 2026-11-16 caps would give.

The encoder uses the standard library only (zlib for deflate and CRC-32), never the oracles. Pixels: each monitor of a
screenshot has a title bar, a window with text-like glyph rows, a gradient desktop and a panel; rows repeat with a
period of PERIOD rows so the five filter types can be computed once per distinct (row, previous row) pair. --small
writes scaled-down variants (each side divided until the area is under 64 Kpx) to --out for a quick oracle check.
"""
import argparse
import hashlib
import struct
import sys
import zlib
from pathlib import Path

sys.dont_write_bytecode = True
SIG = b'\x89PNG\r\n\x1a\n'
CH = {0: 1, 2: 3, 4: 2, 6: 4}
PERIOD = 256
OLD = {'MAX_INPUT': 256 << 20, 'MAX_DIMENSION': 16384, 'MAX_OUTPUT': 64 << 20}
NEW = {'MAX_INPUT': 1 << 30, 'MAX_DIMENSION': (1 << 31) - 1, 'MAX_OUTPUT': 256 << 20}
M4K = 3840


def caps_outcome(s, caps, size=None):
    """Strict's cap outcome by intent: the input length is tested first (P2), then IHDR's sides and area (P4)."""
    w, h = s['w'], s['h']
    size = s.get('pad_to') if size is None else size
    if size is not None and size > caps['MAX_INPUT']:
        return 'FileTooLarge'
    if not (1 <= w <= caps['MAX_DIMENSION'] and 1 <= h <= caps['MAX_DIMENSION']):
        return 'InvalidDimensions'
    if 4 * w * h > caps['MAX_OUTPUT']:
        return 'OutputTooLarge'
    return 'accept'


# name, w, h, colour type, depth, interlace, encoding, kind, monitors (screenshot layout), note
SPECS = [
    dict(name='screen-2x4k-rgb8', w=2 * M4K, h=2160, ct=2, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 2, note='two 4K monitors side by side; 16.6 Mpx, just under the old 16.7 Mpx area cap'),
    dict(name='screen-3x4k-rgba8', w=3 * M4K, h=2160, ct=6, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 3, note='three 4K monitors; 24.9 Mpx'),
    dict(name='screen-2x5k-rgb8', w=2 * 5120, h=2880, ct=2, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(5120, 2880)] * 2, note='two 5K monitors; 29.5 Mpx'),
    dict(name='screen-4x4k-row-rgba8', w=4 * M4K, h=2160, ct=6, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 4, note='four 4K monitors in a row; 33.2 Mpx, long side 15,360'),
    dict(name='screen-2x2-4k-rgba8', w=2 * M4K, h=2 * 2160, ct=6, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 2, rows_of_monitors=2, note='2 x 2 wall of 4K monitors; 33.2 Mpx'),
    dict(name='screen-4x2-4k-rgba8', w=4 * M4K, h=2 * 2160, ct=6, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 4, rows_of_monitors=2, note='4 x 2 wall of 4K monitors; 66.4 Mpx, 4WH = 98.9% of MAX_OUTPUT'),
    dict(name='screen-4k-plus-1440p-rgba8', w=M4K + 2560, h=2160, ct=6, depth=8, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160), (2560, 1440)], note='mixed 4K + 1440p; bounding box with a transparent region below the smaller monitor'),
    dict(name='screen-2x4k-rgba16', w=2 * M4K, h=2160, ct=6, depth=16, il=0, enc='z6', kind='screenshot-multimonitor',
         layout=[(M4K, 2160)] * 2, note='two 4K monitors, 16-bit RGBA (HDR capture); RGBA16 output 8N = 127 MiB'),
    dict(name='screen-3x8k-rgba8-overcap', w=3 * 7680, h=4320, ct=6, depth=8, il=0, enc='z1', kind='screenshot-multimonitor',
         layout=[(7680, 4320)] * 3, note='three 8K monitors; 99.5 Mpx, over MAX_OUTPUT (rejected at IHDR)'),
    dict(name='cap-8192x8192-rgba8', w=8192, h=8192, ct=6, depth=8, il=0, enc='z6', kind='cap-boundary',
         layout=[(8192, 8192)], note='4WH = MAX_OUTPUT exactly'),
    dict(name='cap-8192x8193-rgba8', w=8192, h=8193, ct=6, depth=8, il=0, enc='z1', kind='cap-boundary',
         layout=[(8192, 8193)], note='one row over MAX_OUTPUT'),
    dict(name='cap-8192x8192-rgba16', w=8192, h=8192, ct=6, depth=16, il=0, enc='z6', kind='cap-boundary',
         layout=[(8192, 8192)], note='RGBA16 output 512 MiB = 2*MAX_OUTPUT; Raw 536,879,104 bytes'),
    dict(name='cap-8192x8192-rgba16-adam7', w=8192, h=8192, ct=6, depth=16, il=1, enc='z6', kind='cap-boundary',
         layout=[(8192, 8192)], note='interlaced: Raw 536,886,272 bytes (check_bounds.py)'),
    dict(name='thin-1x67108864-rgba8', w=1, h=1 << 26, ct=6, depth=8, il=0, enc='z6', kind='long-thin',
         note='one pixel wide, H = MAX_OUTPUT/4 (the longest admissible side)'),
    dict(name='thin-67108864x1-rgba8', w=1 << 26, h=1, ct=6, depth=8, il=0, enc='z6', kind='long-thin',
         note='one pixel high, W = MAX_OUTPUT/4'),
    dict(name='thin-1x67108865-rgba8', w=1, h=(1 << 26) + 1, ct=6, depth=8, il=0, enc='z1', kind='long-thin',
         note='one row over MAX_OUTPUT'),
    dict(name='thin-65536x1024-grey8', w=65536, h=1024, ct=0, depth=8, il=0, enc='z6', kind='long-thin',
         note='64:1 banner, side 4x the old 16,384 cap; 4WH = MAX_OUTPUT'),
    dict(name='thin-1048576x64-grey8', w=1 << 20, h=64, ct=0, depth=8, il=0, enc='z6', kind='long-thin',
         note='side above libpng\'s default 1,000,000 user limit (GdkCompat per-side question)'),
    dict(name='thin-2147483647x1-grey1', w=(1 << 31) - 1, h=1, ct=0, depth=1, il=0, enc='z6', kind='long-thin',
         note='side = MAX_DIMENSION (wire limit) passes; area over MAX_OUTPUT'),
    dict(name='thin-2147483648x1-grey1', w=1 << 31, h=1, ct=0, depth=1, il=0, enc='z6', kind='long-thin',
         note='side = 2^31, beyond the wire limit'),
    dict(name='stored-8192x8192-rgba8', w=8192, h=8192, ct=6, depth=8, il=0, enc='stored', kind='stored-near-cap',
         layout=[(8192, 8192)], note='stored deflate at MAX_OUTPUT; about 268.5 MB, over the old 256 MiB MAX_INPUT'),
    dict(name='stored-8192x8192-rgba16', w=8192, h=8192, ct=6, depth=16, il=0, enc='stored', kind='stored-near-cap',
         layout=[(8192, 8192)], note='stored 16-bit at 2^26 pixels; about 537 MB'),
    dict(name='stored-1x67108864-rgba16-adam7', w=1, h=1 << 26, ct=6, depth=16, il=1, enc='stored', kind='stored-near-cap',
         note='the largest Raw of any admissible image (603,979,776 bytes, check_bounds.py), stored: about 604 MB'),
    dict(name='input-exactly-2^30', w=64, h=64, ct=6, depth=8, il=0, enc='z6', kind='input-cap', layout=[(64, 64)],
         pad_to=1 << 30, note='small image plus one private ancillary chunk (prVt) so the file is exactly MAX_INPUT bytes'),
    dict(name='input-2^30-plus-1', w=64, h=64, ct=6, depth=8, il=0, enc='z6', kind='input-cap', layout=[(64, 64)],
         pad_to=(1 << 30) + 1, note='one byte over MAX_INPUT'),
]
for s in SPECS:
    s['expect'] = caps_outcome(s, NEW)
    s['old_caps'] = caps_outcome(s, OLD)


# ------------------------------------------------------------------ pixels
def monitor_rgb(m, mw, x, t):
    """8-bit RGB of pixel x (within monitor m of width mw) in template row t (0 <= t < PERIOD)."""
    if t < 24:                                            # title bar
        return (53, 132, 228) if x < mw - 90 else ((224, 27, 36) if x >= mw - 30 else (60, 60, 60))
    if mw * 3 // 32 <= x < mw * 19 // 32 and 24 <= t < 232:   # window with text lines
        if (t - 24) % 16 < 10 and (x % 7) < 5 and (((x // 7) * 31 + (t // 16) * 17 + m * 5) % 9) < 6 \
                and ((x * 7 + t * 13) % 11) < 7:
            return (36, 31, 49)
        return (250, 250, 250)
    if t >= PERIOD - 16:                                  # panel
        return (32, 32, 32) if (x // 48) % 7 else (120, 160, 220)
    return ((x * 255 // max(mw - 1, 1) + m * 40) & 255, (t * 3 + m * 70) & 255, (200 - x * 64 // max(mw, 1)) & 255)


def pixel_bytes(ct, depth, rgb, alpha, x, t):
    if depth == 16:
        lo = (x * 13 + t * 7) & 0xFF
        vals = {0: (rgb[1],), 2: rgb, 4: (rgb[1], alpha), 6: rgb + (alpha,)}[ct]
        return b''.join(bytes((v, lo if v not in (0, 255) else v)) for v in vals)
    return bytes({0: (rgb[1],), 2: rgb, 4: (rgb[1], alpha), 6: rgb + (alpha,)}[ct])


def screen_row(s, key):
    """Template row bytes (no filter byte) for key = (t, band): monitors laid out left to right; a monitor whose height
    is below the bounding box is transparent (RGBA) or black (RGB) in the band below it."""
    t, low = key
    out = []
    for m, (mw, mh) in enumerate(s['layout']):
        below = low and mh < s['h']
        for x in range(mw):
            if below:
                out.append(pixel_bytes(s['ct'], s['depth'], (0, 0, 0), 0, x, t))
            else:
                out.append(pixel_bytes(s['ct'], s['depth'], monitor_rgb(m, mw, x, t), 255, x, t))
    row = b''.join(out)
    assert len(row) == s['w'] * CH[s['ct']] * s['depth'] // 8, (len(row), s['name'])
    return row


def key_of(s, y):
    low = any(mh < s['h'] and y >= mh for mw, mh in s['layout'])
    return (y % PERIOD, low)


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    return a if pa <= pb and pa <= pc else (b if pb <= pc else c)


def filt(ft, row, prev, bpp):
    if ft == 0:
        return b'\x00' + row
    out = bytearray(len(row) + 1)
    out[0] = ft
    for i, x in enumerate(row):
        a = row[i - bpp] if i >= bpp else 0
        b = prev[i]
        c = prev[i - bpp] if i >= bpp else 0
        if ft == 1:
            v = x - a
        elif ft == 2:
            v = x - b
        elif ft == 3:
            v = x - ((a + b) >> 1)
        else:
            v = x - paeth(a, b, c)
        out[i + 1] = v & 0xFF
    return bytes(out)


def bpp_of(s):
    return max(1, CH[s['ct']] * s['depth'] // 8)


def raw_screen(s):
    """Non-interlaced scanlines of a screenshot-layout image; the filter type cycles 0..4 with the template row."""
    bpp, rows, filtered = bpp_of(s), {}, {}
    zero = bytes(s['w'] * CH[s['ct']] * s['depth'] // 8)
    parts = []
    prev_key = None
    for y in range(s['h']):
        k = key_of(s, y)
        if k not in rows:
            rows[k] = screen_row(s, k)
        fk = (k, prev_key)
        if fk not in filtered:
            filtered[fk] = filt(k[0] % 5, rows[k], rows[prev_key] if prev_key is not None else zero, bpp)
        parts.append(filtered[fk])
        prev_key = k
    return b''.join(parts)


def raw_screen_adam7(s):
    """Interlaced scanlines (filter type None) of a screenshot-layout image: each pass row samples a template row."""
    ch = CH[s['ct']] * s['depth'] // 8
    rows, sub = {}, {}
    parts = []
    for x0, y0, dx, dy in ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)):
        if s['w'] <= x0 or s['h'] <= y0:
            continue
        for y in range(y0, s['h'], dy):
            k = key_of(s, y)
            if (k, x0, dx) not in sub:
                if k not in rows:
                    rows[k] = screen_row(s, k)
                r = rows[k]
                sub[(k, x0, dx)] = b'\x00' + b''.join(r[i * ch:(i + 1) * ch] for i in range(x0, s['w'], dx))
            parts.append(sub[(k, x0, dx)])
    return b''.join(parts)


def column_pixel(s, y):
    v = y % 4096
    rgb = ((v * 255) // 4095, (v * 7) & 255, 255 - (v >> 4))
    return pixel_bytes(s['ct'], s['depth'], rgb, 255, 0, y & 0xFF)


def raw_thin_column(s):
    """w = 1: the column repeats every 4096 rows (a multiple of 8, so every Adam7 pass repeats too). Non-interlaced
    rows use filter types cycling 0..4 (prev = the pixel above); interlaced rows use None."""
    P, H = 4096, s['h']
    pix = [column_pixel(s, y) for y in range(P)]
    bpp = bpp_of(s)
    if not s['il']:
        gen = [filt(y % 5, pix[y], pix[(y - 1) % P], bpp) for y in range(P)]
        first = filt(0, pix[0], bytes(len(pix[0])), bpp)
        block = b''.join(gen)
        n, rem = divmod(H, P)
        return first + b''.join(gen[1:]) + block * (n - 1) + b''.join(gen[:rem]) if n else b''.join([first] + gen[1:rem])
    parts = []
    for y0, dy in ((0, 8), (4, 8), (2, 4), (1, 2)):      # passes with x0 = 0 (w = 1 leaves the others empty)
        per = P // dy
        rows = [b'\x00' + pix[(y0 + k * dy) % P] for k in range(per)]
        ph = (H - y0 + dy - 1) // dy
        n, rem = divmod(ph, per)
        parts.append(b''.join(rows) * n + b''.join(rows[:rem]))
    return b''.join(parts)


def raw_thin_row(s):
    """h = 1: one scanline; the pixels repeat every 4096; filter Sub (periodic after the first period)."""
    P, W = 4096, s['w']
    ch = CH[s['ct']] * s['depth'] // 8
    if s['depth'] < 8:
        assert s['ct'] == 0
        nbytes = (W * s['depth'] + 7) // 8
        return b'\x00' + bytes(nbytes)                    # all-black 1-bit row (rejected at IHDR anyway)
    if s['ct'] == 0:
        unit = bytes((x * 255) // (P - 1) for x in range(P))
    else:
        unit = b''.join(column_pixel(s, x) for x in range(P))
    two = filt(1, unit * 2, bytes(len(unit) * 2), ch)[1:]
    first, rest = two[:len(unit)], two[len(unit):]
    n, rem = divmod(W, P)
    body = first + rest * (n - 1) + rest[:rem * ch] if n else first[:rem * ch]
    return b'\x01' + body


def raw_grey_banner(s):
    """ct 0 depth 8, wide: row bytes repeat every 4096 pixels; filter Up after the first row."""
    P = 4096
    rows = []
    for t in range(16):
        unit = bytes(((x * 255) // (P - 1) + t * 16) & 255 for x in range(P))
        rows.append(unit * (s['w'] // P) + unit[:s['w'] % P])
    parts = [b'\x00' + rows[0]]
    cache = {}
    for y in range(1, s['h']):
        k = (y % 16, (y - 1) % 16)
        if k not in cache:
            cache[k] = filt(2, rows[k[0]], rows[k[1]], 1)
        parts.append(cache[k])
    return b''.join(parts)


def raw_of(s):
    if s['w'] == 1:
        return raw_thin_column(s)
    if s['h'] == 1:
        return raw_thin_row(s)
    if 'layout' not in s:
        return raw_grey_banner(s)
    return raw_screen_adam7(s) if s['il'] else raw_screen(s)


def expected_raw_size(s):
    bits = CH[s['ct']] * s['depth']
    rb = lambda w: (w * bits + 7) // 8
    if not s['il']:
        return s['h'] * (1 + rb(s['w']))
    tot = 0
    for x0, y0, dx, dy in ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)):
        pw = (s['w'] - x0 + dx - 1) // dx if s['w'] > x0 else 0
        ph = (s['h'] - y0 + dy - 1) // dy if s['h'] > y0 else 0
        if pw and ph:
            tot += ph * (1 + rb(pw))
    return tot


# ------------------------------------------------------------------ container
def chunk(t, d):
    return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(d, zlib.crc32(t)))


def write_png(s, path):
    raw = raw_of(s)
    assert len(raw) == expected_raw_size(s), (s['name'], len(raw), expected_raw_size(s))
    level = {'z6': 6, 'z1': 1, 'stored': 0}[s['enc']]
    z = zlib.compress(raw, level)
    del raw
    ihdr = struct.pack('>IIBBBBB', s['w'], s['h'], s['depth'], s['ct'], 0, 0, s['il'])
    cs = 65536 if s['enc'] == 'stored' else 8192
    h = hashlib.sha256()
    n = 0
    with open(path, 'wb') as f:
        def put(b):
            nonlocal n
            f.write(b)
            h.update(b)
            n += len(b)
        put(SIG)
        put(chunk(b'IHDR', ihdr))
        if s.get('pad_to'):
            fixed = len(SIG) + 25 + sum(12 + len(z[i:i + cs]) for i in range(0, len(z), cs)) + 12
            plen = s['pad_to'] - fixed - 12
            assert plen > 0
            t = b'prVt'
            put(struct.pack('>I', plen) + t)
            crc = zlib.crc32(t)
            block = bytes(1 << 24)
            left = plen
            while left:
                piece = block[:min(left, len(block))]
                put(piece)
                crc = zlib.crc32(piece, crc)
                left -= len(piece)
            put(struct.pack('>I', crc))
        for i in range(0, len(z), cs):
            put(chunk(b'IDAT', z[i:i + cs]))
        put(chunk(b'IEND', b''))
    if s.get('pad_to'):
        assert n == s['pad_to'], (n, s['pad_to'])
    return h.hexdigest(), n


def small_variant(s):
    t = dict(s, name=s['name'] + '-small')
    f = 1
    while (t['w'] // f) * (t['h'] // f) > 1 << 16 and min(t['w'] // f, t['h'] // f) > 1:
        f *= 2
    t['w'], t['h'] = max(1, s['w'] // f), max(1, s['h'] // f)
    if s['w'] == 1 or s['h'] == 1:
        t['w'], t['h'] = (1, 70000) if s['w'] == 1 else (70000, 1)
    if 'layout' in s:
        t['layout'] = [(max(1, mw // f), max(1, mh // f)) for mw, mh in s['layout']]
        t['w'] = sum(mw for mw, _ in t['layout'])
    t.pop('pad_to', None)
    return t


COLS = ['path', 'sha256', 'name', 'kind', 'origin', 'licence', 'w', 'h', 'ct', 'depth', 'interlace', 'encoding', 'size',
        'expect', 'old_caps', 'note']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=str(Path.home() / 'corpora' / 'png-synthetic'))
    ap.add_argument('--only', default='')
    ap.add_argument('--small', action='store_true')
    ap.add_argument('--merge', action='store_true', help='only rebuild MANIFEST.tsv from the per-name row files')
    a = ap.parse_args()
    out = Path(a.out)
    (out / 'files').mkdir(parents=True, exist_ok=True)
    only = set(filter(None, a.only.split(',')))
    specs = [s for s in SPECS if not only or s['name'] in only]
    for s in specs:   # rows_of_monitors: the layout is a row of monitors, repeated vertically (template rows repeat anyway)
        if 'layout' in s:
            assert sum(mw for mw, _ in s['layout']) == s['w'], s['name']
            if s.get('rows_of_monitors'):
                s['layout'] = [(mw, s['h']) for mw, _ in s['layout']]
    if a.small:
        specs = [small_variant(s) for s in specs if not s['name'].startswith('input-') and s['w'] < 1 << 30]
    man = out / ('MANIFEST-small.tsv' if a.small else 'MANIFEST.tsv')
    rowdir = out / ('rows-small' if a.small else 'rows')
    rowdir.mkdir(exist_ok=True)
    for s in ([] if a.merge else specs):
        tmp = out / 'files' / (s['name'] + '.partial')
        sha, n = write_png(s, tmp)
        dst = out / 'files' / (sha + '.png')
        tmp.replace(dst)
        r = {'path': f'files/{sha}.png', 'sha256': sha, 'name': s['name'], 'kind': s['kind'],
             'origin': 'synthetic (gen_synthetic.py)', 'licence': 'MIT (project-authored)',
             'w': str(s['w']), 'h': str(s['h']), 'ct': str(s['ct']), 'depth': str(s['depth']),
             'interlace': str(s['il']), 'encoding': s['enc'], 'size': str(n),
             'expect': caps_outcome(s, NEW, n), 'old_caps': caps_outcome(s, OLD, n), 'note': s['note']}
        (rowdir / (s['name'] + '.tsv')).write_text('\t'.join(r[c] for c in COLS) + '\n')
        print(f"{s['name']}: {n} bytes, {sha[:16]}", flush=True)
    # one row file per name (parallel runs never write the same file); the manifest is rebuilt from all of them
    lines = []
    for p in sorted(rowdir.glob('*.tsv')):
        r = dict(zip(COLS, p.read_text().rstrip('\n').split('\t')))
        q = {'w': int(r['w']), 'h': int(r['h'])}           # recomputed from the written size (FileTooLarge comes first)
        r['expect'], r['old_caps'] = caps_outcome(q, NEW, int(r['size'])), caps_outcome(q, OLD, int(r['size']))
        lines.append('\t'.join(r[c] for c in COLS) + '\n')
    man.write_text('\t'.join(COLS) + '\n' + ''.join(lines))

if __name__ == '__main__':
    main()
