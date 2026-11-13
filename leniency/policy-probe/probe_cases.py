#!/usr/bin/env python3
"""Hand-made ICNS/ICO probe inputs for the policy-layer specification phase.

Usage: python3 -I probe_cases.py icns|ico OUTDIR RESULTS_JSON
Writes every case to OUTDIR/<name>.<ext>, runs every incumbent runner on it in a
separate process (10 s timeout each) and writes {case: {runner: result}} to
RESULTS_JSON. No corpus bytes are involved: every input is built here.
"""
import json, os, struct, subprocess, sys, zlib

BIN = '/tmp/claude-0/policy-probe-target'
PROBE = f'{BIN}/release/policy-probe'
GDK = f'{BIN}/gdkprobe'

# ---------------------------------------------------------------- PNG / BMP
def png(w, h, rgba=True, px=None):
    def chunk(t, d):
        return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d) & 0xffffffff)
    ch = 4 if rgba else 3
    px = px or bytes([0x11, 0x22, 0x33, 0x80][:ch])
    raw = b''.join(b'\x00' + px * w for _ in range(h))
    ihdr = struct.pack('>IIBBBBB', w, h, 8, 6 if rgba else 2, 0, 0, 0)
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'IDAT', zlib.compress(raw)) + chunk(b'IEND', b'')

def bmp24(w, h, bgr=b'\x30\x20\x10'):
    stride = (w * 3 + 3) // 4 * 4
    rows = (bgr * w).ljust(stride, b'\x00') * h
    hdr = struct.pack('<IiiHHIIiiII', 40, w, h, 1, 24, 0, 0, 0, 0, 0, 0)
    return b'BM' + struct.pack('<IHHI', 14 + 40 + len(rows), 0, 0, 54) + hdr + rows

# ---------------------------------------------------------------- ICNS
def elem(t, payload, length=None):
    return t + struct.pack('>I', len(payload) + 8 if length is None else length) + payload

def icns(elems, declared=None, suffix=b''):
    body = b''.join(elems)
    return b'icns' + struct.pack('>I', 8 + len(body) if declared is None else declared) + body + suffix

def rle_const(n, values=(0x10, 0x20, 0x30)):
    """Three channels, each a constant, as repeat runs of at most 130."""
    out = b''
    for v in values:
        left = n
        while left:
            c = min(130, left)
            if c >= 3:
                out += bytes([c + 125, v]); left -= c
            else:
                out += bytes([c - 1]) + bytes([v]) * c; left -= c
    return out

def mask(n, v=0xff):
    return bytes([v]) * n

def icns_cases():
    c = {}
    N16, N32 = 256, 1024
    is32 = elem(b'is32', rle_const(N16)); s8mk = elem(b's8mk', mask(N16, 0xc0))
    il32 = elem(b'il32', rle_const(N32, (0x40, 0x50, 0x60))); l8mk = elem(b'l8mk', mask(N32, 0x80))
    base = icns([is32, s8mk])
    c['base-is32'] = base
    c['base-is32-il32'] = icns([is32, s8mk, il32, l8mk])
    # container length (R4)
    c['suffix-1-byte'] = base + b'\x00'
    c['suffix-8-bytes'] = base + b'junkjunk'
    c['suffix-entry'] = base + elem(b'xxxx', b'abc')
    c['declared-short-cuts-mask'] = icns([is32, s8mk], declared=8 + len(is32) + 100)
    c['declared-short-at-entry-boundary'] = icns([is32, s8mk, il32, l8mk], declared=8 + len(is32) + len(s8mk))
    c['declared-long'] = icns([is32, s8mk], declared=8 + len(is32) + len(s8mk) + 16)
    c['declared-below-8'] = icns([is32, s8mk], declared=4)
    # entry scan (R5/R6)
    c['tail-3-bytes-inside-declared'] = icns([is32, s8mk], declared=8 + len(is32) + len(s8mk) + 3, suffix=b'\x00\x00\x00')
    c['entry-length-0-last'] = icns([is32, s8mk, b'xxxx\x00\x00\x00\x00'])
    c['entry-length-4-last'] = icns([is32, s8mk, b'xxxx\x00\x00\x00\x04'])
    c['entry-oob-last'] = icns([is32, s8mk, b'xxxx\x00\x01\x00\x00abcd'])
    c['entry-oob-first'] = icns([b'xxxx\x00\x01\x00\x00', is32, s8mk])
    c['entries-4097-junk'] = icns([elem(b'junk', b'')] * 4097 + [is32, s8mk])
    c['empty-container'] = icns([])
    c['bad-magic'] = b'icnx' + base[4:]
    # RLE colour payload (R10-R12)
    argb = bytes([0xff, 0x10, 0x20, 0x30]) * N16
    c['argb-raw-4N'] = icns([elem(b'is32', argb), s8mk])
    c['rle-trailing-1'] = icns([elem(b'is32', rle_const(N16) + b'\x00'), s8mk])
    # overrun: B channel = 128 (253,v) then a 130-run (255,v) when 128 remain -> exceeds by 2
    r = rle_const(N16, (0x10, 0x20))[:]  # R,G only
    c['rle-overrun-last-run'] = icns([elem(b'is32', r + bytes([253, 0x30, 255, 0x30])), s8mk])
    c['rle-truncated-then-mask'] = icns([elem(b'is32', rle_const(N16)[:-1]), s8mk])
    c['rle-empty-payload'] = icns([elem(b'is32', b''), s8mk])
    zero4 = b'\x00\x00\x00\x00'
    c['is32-zero-prefix'] = icns([elem(b'is32', zero4 + rle_const(N16)), s8mk])
    N128 = 128 * 128
    it32 = elem(b'it32', zero4 + rle_const(N128, (1, 2, 3))); t8mk = elem(b't8mk', mask(N128, 0x7f))
    c['it32-prefix'] = icns([it32, t8mk])
    c['it32-no-prefix'] = icns([elem(b'it32', rle_const(N128, (1, 2, 3))), t8mk])
    c['it32-nonzero-prefix'] = icns([elem(b'it32', b'\x00\x00\x01\x00' + rle_const(N128, (1, 2, 3))), t8mk])
    # masks and pairing (R13/R14)
    c['mask-long'] = icns([is32, elem(b's8mk', mask(N16 + 1))])
    c['mask-short'] = icns([is32, elem(b's8mk', mask(N16 - 1))])
    c['colour-without-mask'] = icns([is32])
    c['colour-without-mask-plus-il32-pair'] = icns([is32, il32, l8mk])
    c['mask-before-colour'] = icns([s8mk, is32])
    c['duplicate-is32'] = icns([is32, s8mk, elem(b'is32', rle_const(N16, (0x99, 0x88, 0x77)))])
    c['broken-il32-good-is32'] = icns([is32, s8mk, elem(b'il32', rle_const(N32)[:-3]), l8mk])
    r32 = rle_const(N32, (0x40, 0x50))
    c['overrun-il32-good-is32'] = icns([is32, s8mk, elem(b'il32', r32 + bytes([253, 0x60] * 7 + [255, 0x60, 255, 0x60])), l8mk])
    c['mismatch-is32-l8mk'] = icns([is32, l8mk])
    # opaque candidates (R9; outside the legacy core)
    c['png-ic07-plus-is32'] = icns([elem(b'ic07', png(128, 128)), is32, s8mk])
    c['png-ic08-256'] = icns([elem(b'ic08', png(256, 256))])
    c['png-ic08-16x16'] = icns([elem(b'ic08', png(16, 16)), is32, s8mk])
    c['bmp-in-ic08'] = icns([elem(b'ic08', bmp24(4, 4)), is32, s8mk])
    c['png-ic09-512'] = icns([elem(b'ic09', png(512, 512))])
    c['png-ic10-1024-plus-is32'] = icns([elem(b'ic10', png(1024, 1024)), is32, s8mk])
    c['png-icp4-16'] = icns([elem(b'icp4', png(16, 16))])
    c['png-rgb-ic07'] = icns([elem(b'ic07', png(128, 128, rgba=False))])
    c['jp2sig-ic08-plus-is32'] = icns([elem(b'ic08', bytes.fromhex('0000000c6a5020200d0a870a') + b'\x00' * 20), is32, s8mk])
    c['png-ic08-plus-broken-it32'] = icns([elem(b'ic08', png(256, 256)), elem(b'it32', zero4 + b'\x00'), t8mk])
    return c

ICNS_RUNNERS = [
    ('strict', [PROBE, 'icns-strict-largest']),
    ('crate', [PROBE, 'icns-crate-largest']),
    ('gdk', [GDK, 'icns', 'forced-whole']),
    ('gdk-byte', [GDK, 'icns', 'forced-byte']),
]

# ---------------------------------------------------------------- ICO
def bih(w, sh, bpp, comp=0, hsize=40, planes=1, sizeimage=0, clrused=0):
    h = struct.pack('<IiiHHIIiiII', 40, w, sh, planes, bpp, comp, sizeimage, 0, 0, clrused, 0)
    if hsize == 40:
        return h
    if hsize == 12:
        return struct.pack('<IHHHH', 12, w, sh, planes, bpp)
    return struct.pack('<I', hsize) + h[4:] + b'\x00' * (hsize - 40)

def rowpad(b):
    return b + b'\x00' * ((-len(b)) % 4)

def and_rows(w, h, bits):
    """bits: list of h lists of w 0/1, bottom-up rows as stored."""
    out = b''
    for r in bits:
        v = 0
        bs = bytearray((w + 7) // 8)
        for x, bit in enumerate(r):
            if bit:
                bs[x // 8] |= 0x80 >> (x % 8)
        out += rowpad(bytes(bs))
    return out

def dib(w, h, bpp, xor_rows, palette=b'', andm=None, **kw):
    sh = kw.pop('stored_height', 2 * h)
    hdr = bih(w, sh, bpp, **kw)
    am = and_rows(w, h, [[0] * w for _ in range(h)]) if andm is None else andm
    return hdr + palette + b''.join(rowpad(r) for r in xor_rows) + am

def ico(entries, typ=1, reserved=0, count=None, dir_override=None):
    """entries: list of (dib_bytes, dict of dir-field overrides)."""
    n = len(entries) if count is None else count
    dir_end = 6 + 16 * len(entries)
    out_dir = struct.pack('<HHH', reserved, typ, n)
    data = b''
    for payload, ov in entries:
        off = ov.get('offset', dir_end + len(data))
        size = ov.get('size', len(payload))
        out_dir += struct.pack('<BBBBHHII', ov.get('w', 2), ov.get('h', 2), ov.get('cc', 0), ov.get('res', 0),
                               ov.get('planes', 1), ov.get('bpp', 24), size, off)
        data += payload
    return out_dir + data

def ico_cases():
    c = {}
    W = H = 2
    px24 = [bytes([0x30, 0x20, 0x10]) * W, bytes([0x60, 0x50, 0x40]) * W]   # bottom row first
    andbits = and_rows(W, H, [[1, 0], [0, 0]])
    d24 = dib(W, H, 24, px24, andm=andbits)
    c['base-24'] = ico([(d24, {})])
    c['dir-reserved-1'] = ico([(d24, {})], reserved=1)
    c['dir-type-3'] = ico([(d24, {})], typ=3)
    c['dir-type-0'] = ico([(d24, {})], typ=0)
    c['count-0'] = struct.pack('<HHH', 0, 1, 0) + d24
    many = [(d24, {'offset': 6 + 16 * 1025})] * 1025
    c['count-1025'] = ico(many)
    c['truncated-table'] = ico([(d24, {})])[:6 + 8]
    c['entry-reserved-1-selected'] = ico([(d24, {'res': 1})])
    c['entry-reserved-1-unselected'] = ico([(d24, {}), (d24, {'res': 1, 'w': 1, 'h': 1})])
    c['entry-size-0'] = ico([(d24, {'size': 0})])
    c['entry-size-xor-only'] = ico([(d24, {'size': len(d24) - len(andbits)})])
    c['entry-size-short-header'] = ico([(d24, {'size': 8})])
    c['entry-size-beyond-eof'] = ico([(d24, {'size': len(d24) + 100})])
    c['entry-offset-beyond-eof'] = ico([(d24, {'offset': 10000})])
    c['entry-oob-unselected'] = ico([(d24, {}), (d24, {'w': 1, 'h': 1, 'offset': 10000})])
    c['entry-size-0-unselected-small'] = ico([(d24, {}), (d24, {'w': 1, 'h': 1, 'size': 0})])
    c['slack-after-and'] = ico([(d24 + b'\x00' * 8, {})])
    c['dir-dims-16-dib-2'] = ico([(d24, {'w': 16, 'h': 16})])
    c['dir-dims-0-dib-2'] = ico([(d24, {'w': 0, 'h': 0})])
    c['cur-hotspot-300'] = ico([(d24, {'planes': 300, 'bpp': 2})], typ=2)
    c['ico-planes-300'] = ico([(d24, {'planes': 300})])
    # PNG entries
    p = png(2, 2)
    c['png-rgba'] = ico([(p, {'bpp': 32})])
    c['png-rgb'] = ico([(png(2, 2, rgba=False), {'bpp': 24})])
    c['png-plus-dib'] = ico([(p, {'bpp': 32, 'w': 2, 'h': 2}), (dib(1, 1, 24, [b'\x05\x06\x07']), {'w': 1, 'h': 1})])
    c['png-sig-4-only'] = ico([(b'\x89PNG' + b'\x00' * 60, {})])
    # DIB-in-ICO semantics
    px32 = [bytes([0x30, 0x20, 0x10, 0x80]) * W, bytes([0x60, 0x50, 0x40, 0xff]) * W]
    c['dib32-alpha-and-set'] = ico([(dib(W, H, 32, px32, andm=and_rows(W, H, [[1, 1], [1, 0]])), {'bpp': 32})])
    px32z = [bytes([0x30, 0x20, 0x10, 0x00]) * W] * 2
    c['dib32-zero-alpha-and'] = ico([(dib(W, H, 32, px32z, andm=and_rows(W, H, [[1, 0], [0, 0]])), {'bpp': 32})])
    c['dib32-no-and'] = ico([(dib(W, H, 32, px32, andm=b''), {'bpp': 32})])
    c['dib24-no-and'] = ico([(dib(W, H, 24, px24, andm=b''), {})])
    pal1 = bytes([0x00, 0x00, 0xff, 0, 0xff, 0x00, 0x00, 0])   # red, blue
    c['dib1-colour-palette'] = ico([(dib(W, H, 1, [b'\x40', b'\x80'], palette=pal1), {'bpp': 1})])
    c['dib16-555-value3'] = ico([(dib(W, H, 16, [struct.pack('<H', (3 << 10) | (3 << 5) | 3) * W] * 2), {'bpp': 16})])
    c['dib16-bit15'] = ico([(dib(W, H, 16, [struct.pack('<H', 0x8000 | (31 << 10)) * W] * 2), {'bpp': 16})])
    c['odd-stored-height-5'] = ico([(dib(W, H, 24, px24, stored_height=5), {})])
    c['stored-height-equal-h'] = ico([(dib(W, H, 24, px24, stored_height=2, andm=b''), {})])
    c['negative-stored-height'] = ico([(dib(W, H, 24, px24, stored_height=-4), {})])
    c['bih-v4-108'] = ico([(dib(W, H, 24, px24, andm=andbits, hsize=108), {})])
    c['bih-core-12'] = ico([(bih(W, 4, 24, hsize=12) + b''.join(rowpad(r) for r in px24) + andbits, {})])
    rle8 = b'\x02\x00\x00\x00' + b'\x02\x00\x00\x00' + b'\x00\x01'
    pal8 = bytes([0x30, 0x20, 0x10, 0]) + b'\x00' * 4 * 255
    c['rle8-in-ico'] = ico([(bih(W, 4, 8, comp=1) + pal8 + rle8 + andbits, {'bpp': 8})])
    masks = struct.pack('<III', 0xff0000, 0xff00, 0xff)
    c['bitfields32-in-ico'] = ico([(bih(W, 4, 32, comp=3) + masks + b''.join(px32) + andbits, {'bpp': 32})])
    pal2 = bytes([0x30, 0x20, 0x10, 0, 0x60, 0x50, 0x40, 0])
    c['dib8-clrused-2'] = ico([(dib(W, H, 8, [b'\x00\x01', b'\x01\x00'], palette=pal2, clrused=2), {'bpp': 8})])
    c['dib8-index-out-of-range'] = ico([(dib(W, H, 8, [b'\x00\x05', b'\x01\x00'], palette=pal2, clrused=2), {'bpp': 8})])
    c['dib8-clrused-0-full'] = ico([(dib(W, H, 8, [b'\x00\x01', b'\x01\x00'], palette=pal2 + b'\x00' * 4 * 254), {'bpp': 8})])
    c['sizeimage-wrong'] = ico([(dib(W, H, 24, px24, andm=andbits, sizeimage=7), {})])
    c['bih-planes-0'] = ico([(dib(W, H, 24, px24, andm=andbits, planes=0), {})])
    c['xor-truncated'] = ico([(dib(W, H, 24, px24, andm=andbits)[:40 + 8], {})])
    c['and-truncated-half'] = ico([(dib(W, H, 24, px24, andm=andbits)[:-4], {})])
    # selection
    d24b = dib(W, H, 24, [bytes([0x01, 0x02, 0x03]) * W] * 2)
    c['tie-two-equal'] = ico([(d24, {}), (d24b, {})])
    c['largest-broken-smaller-ok'] = ico([(d24[:30], {'w': 2, 'h': 2}), (dib(1, 1, 24, [b'\x05\x06\x07']), {'w': 1, 'h': 1})])
    c['bpp-vs-area'] = ico([(dib(1, 1, 32, [b'\x05\x06\x07\x80']), {'w': 1, 'h': 1, 'bpp': 32}), (d24, {'bpp': 24})])
    return c

ICO_RUNNERS = [
    ('strict-e0', [PROBE, 'ico-strict-entry:0']),
    ('strict-largest', [PROBE, 'ico-strict-largest']),
    ('crate-e0', [PROBE, 'ico-crate-entry:0']),
    ('crate-largest', [PROBE, 'ico-crate-largest']),
    ('image', [PROBE, 'ico-image']),
    ('gdk', [GDK, 'ico', 'forced-whole']),
    ('gdk-byte', [GDK, 'ico', 'forced-byte']),
]

def main():
    fmt, outdir, results = sys.argv[1], sys.argv[2], sys.argv[3]
    os.makedirs(outdir, exist_ok=True)
    cases = icns_cases() if fmt == 'icns' else ico_cases()
    runners = ICNS_RUNNERS if fmt == 'icns' else ICO_RUNNERS
    out = {}
    for name, data in cases.items():
        path = os.path.join(outdir, f'{name}.{fmt}')
        with open(path, 'wb') as f:
            f.write(data)
        row = {'len': len(data), 'head_hex': data[:64].hex()}
        for rn, cmd in runners:
            try:
                p = subprocess.run(cmd + [path], capture_output=True, text=True, timeout=10)
                line = p.stdout.strip().splitlines()
                row[rn] = json.loads(line[-1])['r'] if line else {'status': 'crash', 'error': f'rc={p.returncode} {p.stderr[-200:]}'}
            except subprocess.TimeoutExpired:
                row[rn] = {'status': 'timeout'}
        out[name] = row
    with open(results, 'w') as f:
        json.dump(out, f, indent=1, sort_keys=True)

if __name__ == '__main__':
    main()
