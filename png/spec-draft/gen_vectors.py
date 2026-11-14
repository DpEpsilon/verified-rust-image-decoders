#!/usr/bin/env python3
"""Generate VECTORS.json for PNG-CORE-SPEC draft 0.1 (SPEC.md P19).

Independence: inputs are built by an encoder in this file (sample packing, forward filters,
Adam7 pass extraction, a chunk writer, Python's zlib for ordinary streams and a hand-written
DEFLATE bit writer for malformed ones). Every expectation is written from the generator's
intent: the samples it encoded mapped to RGBA by the rules restated here, a canvas composed
here, or a hand-named error. The oracle (png_oracle.py) is never imported.
Usage: python3 -I gen_vectors.py   (writes VECTORS.json next to this file; deterministic)
"""
import hashlib
import json
import random
import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SIG = b'\x89PNG\r\n\x1a\n'
MAX_INPUT = 256 * 1024 * 1024
MAX_ICC = 16 * 1024 * 1024
V = []
RNG = random.Random(20261114)

# ---------------------------------------------------------------- chunk writer


def chunk(t, d, crc=None, length=None):
    c = zlib.crc32(t + d) if crc is None else crc
    return struct.pack('>I', len(d) if length is None else length) + t + d + struct.pack('>I', c)


def ihdr(w, h, depth, ct, il=0, cm=0, fm=0):
    return chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, depth, ct, cm, fm, il))


IEND = chunk(b'IEND', b'')
CH = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}

# ---------------------------------------------------------------- encoder side


def pack_row(vals, depth):
    if depth == 16:
        return b''.join(struct.pack('>H', v) for v in vals)
    if depth == 8:
        return bytes(vals)
    out, acc, nb = bytearray(), 0, 0
    for v in vals:
        acc = (acc << depth) | v
        nb += depth
        if nb == 8:
            out.append(acc)
            acc = nb = 0
    if nb:
        out.append(acc << (8 - nb))
    return bytes(out)


def filt(ft, row, prev, bpp):
    """Forward filter (encoder direction) using the original bytes, as an encoder does."""
    out = bytearray()
    for i, x in enumerate(row):
        a = row[i - bpp] if i >= bpp else 0
        b = prev[i]
        c = prev[i - bpp] if i >= bpp else 0
        if ft == 0:
            p = 0
        elif ft == 1:
            p = a
        elif ft == 2:
            p = b
        elif ft == 3:
            p = (a + b) // 2
        else:
            pa, pb, pc = abs(b - c), abs(a - c), abs(a + b - 2 * c)
            p = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
        out.append((x - p) & 255)
    return bytes(out)


A7 = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)]


def scan(px, w, h, ct, depth, il, filters=None):
    """px: list of sample tuples, row-major. Returns the filtered, uncompressed image data."""
    bpp = max(1, CH[ct] * depth // 8)
    grids = [(0, 0, 1, 1)] if not il else A7
    out = bytearray()
    k = 0
    for x0, y0, dx, dy in grids:
        cols, rows = list(range(x0, w, dx)), list(range(y0, h, dy))
        if not cols or not rows:
            continue
        prev = bytes(len(pack_row([0] * (len(cols) * CH[ct]), depth)))
        for y in rows:
            row = pack_row([s for x in cols for s in px[y * w + x]], depth)
            ft = filters(k) if callable(filters) else (k % 5 if filters is None else filters)
            out += bytes([ft]) + filt(ft, row, prev, bpp)
            prev = row
            k += 1
    return bytes(out)


def z(raw, level=6, strategy=zlib.Z_DEFAULT_STRATEGY, wbits=15):
    c = zlib.compressobj(level, zlib.DEFLATED, wbits, 9, strategy)
    return c.compress(raw) + c.flush()


def split_idat(data, sizes=None):
    if sizes is None:
        return chunk(b'IDAT', data)
    out, p, i = b'', 0, 0
    while p < len(data) or i == 0:
        n = sizes[i % len(sizes)]
        out += chunk(b'IDAT', data[p:p + n])
        p += n
        i += 1
    return out


def png(w, h, ct, depth, px=None, il=0, pre=b'', post=b'', data=None, filters=None, zopts=None,
        sizes=None, raw=None):
    if data is None:
        raw = scan(px, w, h, ct, depth, il, filters) if raw is None else raw
        data = z(raw, **(zopts or {}))
    return SIG + ihdr(w, h, depth, ct, il) + pre + split_idat(data, sizes) + post + IEND


# ---------------------------------------------------------------- expectation side (intent)


def to8(v, d):
    return (v * 510 + 65535) // 131070 if d == 16 else v * 255 // ((1 << d) - 1)


def to16(v, d):
    return v * 65535 // ((1 << d) - 1)


def rgba_of(s, ct, d, plte=None, trns=None, wide=False):
    cv = (lambda v: to16(v, d)) if wide else (lambda v: to8(v, d))
    top = 65535 if wide else 255
    if ct == 3:
        r, g, b = plte[3 * s[0]:3 * s[0] + 3]
        a = trns[s[0]] if trns and s[0] < len(trns) else 255
        return (r * 257, g * 257, b * 257, a * 257) if wide else (r, g, b, a)
    if ct == 0:
        a = 0 if trns is not None and trns == (s[0],) else top
        return (cv(s[0]),) * 3 + (a,)
    if ct == 2:
        a = 0 if trns is not None and trns == tuple(s) else top
        return tuple(cv(v) for v in s) + (a,)
    if ct == 4:
        return (cv(s[0]),) * 3 + (cv(s[1]),)
    return tuple(cv(v) for v in s)


def runs(pix, digits=8):
    out = []
    for p in pix:
        h = ''.join(f'{v:0{digits // 4}x}' for v in p)
        if out and out[-1]['hex'] == h:
            out[-1]['count'] += 1
        else:
            out.append({'count': 1, 'hex': h})
    return out


def add(name, entry, b, expected, **extra):
    v = {'name': name, 'entry': entry, 'input_hex': b.hex(), 'expected': expected}
    v.update(extra)
    V.append(v)


def err(name):
    return {'error': name}


def still(name, b, w, h, pix):
    add(name, 'still', b, {'width': w, 'height': h, 'rgba_runs': runs(pix)})


def bad(name, b, e, entry='still'):
    add(name, entry, b, err(e))


def rand_px(w, h, ct, d, n_pal=None):
    m = (1 << d) - 1
    if ct == 3:
        return [(RNG.randrange(n_pal),) for _ in range(w * h)]
    return [tuple(RNG.randrange(m + 1) for _ in range(CH[ct])) for _ in range(w * h)]


def pal(n):
    return bytes(RNG.randrange(256) for _ in range(3 * n))


COMBOS = [(0, 1), (0, 2), (0, 4), (0, 8), (0, 16), (2, 8), (2, 16), (3, 1), (3, 2), (3, 4), (3, 8),
          (4, 8), (4, 16), (6, 8), (6, 16)]
SIZES = [(1, 1), (2, 3), (7, 5), (9, 10), (17, 2)]

# ---------------------------------------------------------------- P11-P13 valid images
for ct, d in COMBOS:
    for il in (0, 1):
        for w, h in SIZES:
            n = min(1 << d, 1 + RNG.randrange(256)) if ct == 3 else None
            plte = pal(n) if ct == 3 else None
            px = rand_px(w, h, ct, d, n)
            pre = chunk(b'PLTE', plte) if ct == 3 else b''
            b = png(w, h, ct, d, px, il, pre=pre)
            still(f'ok/ct{ct}-d{d}-il{il}-{w}x{h}', b, w, h, [rgba_of(s, ct, d, plte) for s in px])
            if (w, h) == (7, 5):
                add(f'ok16/ct{ct}-d{d}-il{il}-{w}x{h}', 'still16', b,
                    {'width': w, 'height': h, 'rgba16_runs': runs([rgba_of(s, ct, d, plte, wide=True) for s in px], 16)})

# filters one at a time, every bpp
for ft in range(5):
    for ct, d in [(0, 1), (0, 8), (0, 16), (2, 8), (2, 16), (4, 8), (6, 16)]:
        w, h = 6, 4
        px = rand_px(w, h, ct, d)
        still(f'filter/{ft}-ct{ct}-d{d}', png(w, h, ct, d, px, filters=ft), w, h, [rgba_of(s, ct, d) for s in px])

# tRNS
for d in (1, 2, 4, 8, 16):
    w, h = 5, 3
    px = rand_px(w, h, 0, d)
    key = px[2][0]
    still(f'trns/grey-d{d}', png(w, h, 0, d, px, pre=chunk(b'tRNS', struct.pack('>H', key))), w, h,
          [rgba_of(s, 0, d, trns=(key,)) for s in px])
    if d < 16:  # high bits set in the key: masked (P7)
        still(f'trns/grey-d{d}-masked', png(w, h, 0, d, px, pre=chunk(b'tRNS', struct.pack('>H', key | 0xFF00 & ~((1 << d) - 1) & 0xFFFF))),
              w, h, [rgba_of(s, 0, d, trns=(key,)) for s in px])
for d in (8, 16):
    w, h = 4, 4
    px = rand_px(w, h, 2, d)
    px[5] = px[9] = px[0]
    still(f'trns/rgb-d{d}', png(w, h, 2, d, px, pre=chunk(b'tRNS', struct.pack('>HHH', *px[0]))), w, h,
          [rgba_of(s, 2, d, trns=tuple(px[0])) for s in px])
    if d == 16:  # 16-bit key compared exactly, before any reduction (P7, P13.2)
        k = (px[0][0] ^ 1, px[0][1], px[0][2])
        still('trns/rgb-d16-low-byte-differs', png(w, h, 2, d, px, pre=chunk(b'tRNS', struct.pack('>HHH', *k))), w, h,
              [rgba_of(s, 2, d, trns=k) for s in px])
for d, n, tn in [(8, 16, 3), (8, 16, 16), (4, 16, 1), (1, 2, 2)]:
    w, h = 4, 3
    plte = pal(n)
    t = bytes(RNG.randrange(256) for _ in range(tn))
    px = rand_px(w, h, 3, d, n)
    still(f'trns/palette-d{d}-{n}-entries-{tn}-alpha', png(w, h, 3, d, px, pre=chunk(b'PLTE', plte) + chunk(b'tRNS', t)), w, h,
          [rgba_of(s, 3, d, plte, t) for s in px])
    add(f'trns16/palette-d{d}-{n}-{tn}', 'still16', png(w, h, 3, d, px, pre=chunk(b'PLTE', plte) + chunk(b'tRNS', t)),
        {'width': w, 'height': h, 'rgba16_runs': runs([rgba_of(s, 3, d, plte, t, wide=True) for s in px], 16)})

# 16-bit rounding boundaries (P13.2)
vals = [0, 127, 128, 383, 384, 32767, 32768, 32895, 32896, 65407, 65408, 65535]
px = [(v,) for v in vals]
still('depth16/rounding-boundaries', png(len(vals), 1, 0, 16, px, filters=0), len(vals), 1, [rgba_of(s, 0, 16) for s in px])

# ---------------------------------------------------------------- P9/P10 zlib and DEFLATE variants (valid)
w, h, ct, d = 23, 11, 6, 8
BASE_PX = rand_px(w, h, ct, d)
BASE_PX[40:80] = [BASE_PX[40]] * 40
BASE_RAW = scan(BASE_PX, w, h, ct, d, 0)
BASE_EXP = [rgba_of(s, ct, d) for s in BASE_PX]


def base(data=None, pre=b'', post=b'', sizes=None, zopts=None):
    return png(w, h, ct, d, data=data if data is not None else z(BASE_RAW, **(zopts or {})), pre=pre, post=post, sizes=sizes)


for lv in (0, 1, 6, 9):
    for st, sn in [(zlib.Z_DEFAULT_STRATEGY, 'default'), (zlib.Z_FIXED, 'fixed'), (zlib.Z_HUFFMAN_ONLY, 'huffman'),
                   (zlib.Z_RLE, 'rle'), (zlib.Z_FILTERED, 'filtered')]:
        still(f'zlib/level{lv}-{sn}', base(zopts={'level': lv, 'strategy': st}), w, h, BASE_EXP)
for wb in (9, 10, 12, 15):
    still(f'zlib/window-{wb}', base(zopts={'wbits': wb}), w, h, BASE_EXP)
c = zlib.compressobj(6)
fl = c.compress(BASE_RAW[:300]) + c.flush(zlib.Z_SYNC_FLUSH) + c.compress(BASE_RAW[300:]) + c.flush(zlib.Z_FULL_FLUSH) + c.flush()
still('zlib/sync-and-full-flush-empty-stored-blocks', base(fl), w, h, BASE_EXP)
ZB = z(BASE_RAW)
still('idat/split-1-byte', base(sizes=[1]), w, h, BASE_EXP)
still('idat/split-irregular', base(sizes=[3, 1, 50, 7]), w, h, BASE_EXP)
still('idat/zero-length-idats', SIG + ihdr(w, h, d, ct) + chunk(b'IDAT', b'') + chunk(b'IDAT', ZB[:10]) + chunk(b'IDAT', b'')
      + chunk(b'IDAT', ZB[10:]) + chunk(b'IDAT', b'') + IEND, w, h, BASE_EXP)


class BW:
    """Hand DEFLATE bit writer (LSB-first fields, Huffman codes MSB-first)."""

    def __init__(self):
        self.out, self.acc, self.n = bytearray(), 0, 0

    def bits(self, v, n):
        for i in range(n):
            self.acc |= ((v >> i) & 1) << self.n
            self.n += 1
            if self.n == 8:
                self.out.append(self.acc)
                self.acc = self.n = 0

    def code(self, c, n):
        for i in range(n - 1, -1, -1):
            self.bits((c >> i) & 1, 1)

    def flush(self):
        if self.n:
            self.out.append(self.acc)
            self.acc = self.n = 0
        return bytes(self.out)

    def fixed_lit(self, v):
        if v < 144:
            self.code(0x30 + v, 8)
        elif v < 256:
            self.code(0x190 + v - 144, 9)
        elif v < 280:
            self.code(v - 256, 7)
        else:
            self.code(0xC0 + v - 280, 8)


def zwrap(body, payload, cmf=0x78, flg=None, adler=None):
    if flg is None:
        flg = (31 - (cmf * 256) % 31) % 31
    a = zlib.adler32(payload) if adler is None else adler
    return bytes([cmf, flg]) + body + struct.pack('>I', a)


def canon(lengths):
    """Canonical codes (RFC 1951 3.2.2) for the generator's own dynamic blocks."""
    bl = [0] * 16
    for l in lengths:
        bl[l] += 1
    bl[0] = 0
    code, nxt = 0, [0] * 16
    for b in range(1, 16):
        code = (code + bl[b - 1]) << 1
        nxt[b] = code
    out = {}
    for s, l in enumerate(lengths):
        if l:
            out[s] = (nxt[l], l)
            nxt[l] += 1
    return out


def dyn_header(bw, litl, distl, cl_override=None, hlit=None, hdist=None):
    """Emit HLIT/HDIST/HCLEN, a code-length code (all 19 lengths 5 unless overridden: complete? no -
    lengths 4 for 16 symbols + 5 for the rest), then the lengths literally (no repeat codes)."""
    cl = cl_override or [4] * 13 + [5] * 6        # complete: 13*2^-4 + 6*2^-5 = 1
    order = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]
    clv = [0] * 19
    for i, s in enumerate(order):
        clv[s] = cl[i]
    bw.bits((len(litl) - 257) if hlit is None else hlit, 5)
    bw.bits((len(distl) - 1) if hdist is None else hdist, 5)
    bw.bits(15, 4)
    for i in range(19):
        bw.bits(cl[i], 3)
    cc = canon(clv)
    return cc


def emit_lengths(bw, cc, seq):
    """seq: list of (symbol, extra_value) code-length-code symbols."""
    for s, e in seq:
        bw.code(*cc[s])
        if s == 16:
            bw.bits(e, 2)
        elif s == 17:
            bw.bits(e, 3)
        elif s == 18:
            bw.bits(e, 7)


def dyn_block(payload, litl, distl, final=1, use_match=None):
    """A dynamic block coding `payload` as literals (plus optional (len,dist) match at the end)."""
    bw = BW()
    bw.bits(final, 1)
    bw.bits(2, 2)
    cc = dyn_header(bw, litl, distl)
    emit_lengths(bw, cc, [(l, 0) for l in litl + distl])
    lc, dc = canon(litl), canon(distl)
    for x in payload:
        bw.code(*lc[x])
    if use_match:
        (lsym, lextra, lbits), (dsym, dextra, dbits) = use_match
        bw.code(*lc[lsym])
        bw.bits(lextra, lbits)
        bw.code(*dc[dsym])
        bw.bits(dextra, dbits)
    bw.code(*lc[256])
    return bw


def dyn_lengths_only(litl, distl):
    """A dynamic block header followed by its code lengths and nothing else."""
    bw = BW()
    bw.bits(1, 1)
    bw.bits(2, 2)
    cc = dyn_header(bw, litl, distl)
    emit_lengths(bw, cc, [(l, 0) for l in litl + distl])
    return bw.flush()


def lit_lengths(used):
    """A complete literal/length code covering `used` (plus 256): first symbols get short codes."""
    syms = sorted(set(used) | {256})
    n = len(syms)
    L = max(1, (n - 1).bit_length())
    # complete code: (2^L - n) symbols at length L-1 ... simple: give all length L, then fill with
    # unused symbols at length L until complete
    lengths = [0] * 286
    for s in syms:
        lengths[s] = L
    extra = (1 << L) - n
    for s in range(286):
        if extra == 0:
            break
        if lengths[s] == 0:
            lengths[s] = L
            extra -= 1
    return lengths


# Tiny 1x2 grey-8 image (raw 4 bytes: 00 v 00 v) for hand-built streams.
TINY_W, TINY_H = 1, 2
TINY_RAW = bytes([0, 0x41, 0, 0x42])
TINY_EXP = [(0x41, 0x41, 0x41, 255), (0x42, 0x42, 0x42, 255)]


def tiny(data, pre=b''):
    return SIG + ihdr(TINY_W, TINY_H, 8, 0) + pre + chunk(b'IDAT', data) + IEND


def stored(payload, final=1):
    return bytes([final]) + struct.pack('<HH', len(payload), len(payload) ^ 0xFFFF) + payload


def fixed(payload, final=1, tail=None):
    bw = BW()
    bw.bits(final, 1)
    bw.bits(1, 2)
    for x in payload:
        bw.fixed_lit(x)
    if tail:
        tail(bw)
    bw.fixed_lit(256)
    return bw


still('deflate/stored-only', tiny(zwrap(stored(TINY_RAW), TINY_RAW)), 1, 2, TINY_EXP)
still('deflate/empty-stored-then-stored', tiny(zwrap(stored(b'', 0) + stored(TINY_RAW), TINY_RAW)), 1, 2, TINY_EXP)
still('deflate/fixed-literals', tiny(zwrap(fixed(TINY_RAW).flush(), TINY_RAW)), 1, 2, TINY_EXP)
lt = lit_lengths(TINY_RAW)
still('deflate/dynamic-literals-two-dist-codes', tiny(zwrap(dyn_block(TINY_RAW, lt, [1, 1]).flush(), TINY_RAW)), 1, 2, TINY_EXP)
still('deflate/dynamic-single-dist-code-len1', tiny(zwrap(dyn_block(TINY_RAW, lt, [1]).flush(), TINY_RAW)), 1, 2, TINY_EXP)
still('deflate/dynamic-no-dist-codes', tiny(zwrap(dyn_block(TINY_RAW, lt, [0]).flush(), TINY_RAW)), 1, 2, TINY_EXP)
# overlapping copy: 1x 300-pixel grey row "00" + 300 x 0x41 via literal 00 41 then match len 258 dist 1, len 41 dist 1
W300 = 300
raw300 = bytes([0]) + bytes([0x41]) * W300


def tail300(bw):
    bw.fixed_lit(285)                    # length 258
    bw.code(0, 5)                        # distance code 0 = 1
    bw.fixed_lit(265)                    # length 11 + 1 extra bit -> 11 or 12
    bw.bits(0, 1)
    bw.code(0, 5)
    # 2 + 258 + 11 = 271; need 301: add length 30 (code 271 base 27 + 2 extra bits value 3)
    bw.fixed_lit(271)
    bw.bits(3, 2)
    bw.code(0, 5)


s300 = fixed(bytes([0, 0x41]), tail=tail300).flush()
still('deflate/fixed-overlapping-matches', SIG + ihdr(W300, 1, 8, 0) + chunk(b'IDAT', zwrap(s300, raw300)) + IEND, W300, 1,
      [(0x41, 0x41, 0x41, 255)] * W300)
# distance exactly equal to the output so far (allowed): literals 00 07, then length 3 at distance 2
raw_eq = bytes([0, 7, 0, 7, 0])
s_eq = fixed(bytes([0, 7]), tail=lambda bw: (bw.fixed_lit(257), bw.code(1, 5))).flush()
still('deflate/distance-equals-output', SIG + ihdr(4, 1, 8, 0) + chunk(b'IDAT', zwrap(s_eq, raw_eq)) + IEND, 4, 1,
      [(7, 7, 7, 255), (0, 0, 0, 255), (7, 7, 7, 255), (0, 0, 0, 255)])
s_d2 = s_eq
add('deflate/match-overruns-image-is-excess', 'still', tiny(zwrap(s_d2, bytes([0, 9, 0, 9, 0]))), err('ExcessImageData'))

# ---------------------------------------------------------------- P9/P10 errors
bad('zlib/fcheck', tiny(bytes([0x78, 0x9d]) + ZB[2:]), 'InvalidZlibHeader')
bad('zlib/cm-7', tiny(zwrap(stored(TINY_RAW), TINY_RAW, cmf=0x77)), 'UnsupportedZlibMethod')
bad('zlib/cinfo-8', tiny(zwrap(stored(TINY_RAW), TINY_RAW, cmf=0x88)), 'UnsupportedZlibMethod')
bad('zlib/fdict', tiny(bytes([0x78, 0xBB]) + b'\0\0\0\1' + stored(TINY_RAW) + struct.pack('>I', zlib.adler32(TINY_RAW))), 'ZlibPresetDictionary')
bad('zlib/one-byte', tiny(b'\x78'), 'TruncatedZlib')
bad('zlib/empty-idat-only', tiny(b''), 'TruncatedZlib')
bad('zlib/bad-adler', tiny(zwrap(stored(TINY_RAW), TINY_RAW, adler=1)), 'AdlerMismatch')
bad('zlib/missing-adler', tiny(zwrap(stored(TINY_RAW), TINY_RAW)[:-4]), 'TruncatedZlib')
bad('zlib/partial-adler', tiny(zwrap(stored(TINY_RAW), TINY_RAW)[:-2]), 'TruncatedZlib')
bad('zlib/trailing-byte', tiny(zwrap(stored(TINY_RAW), TINY_RAW) + b'\0'), 'TrailingZlibData')
bad('zlib/second-stream', tiny(zwrap(stored(TINY_RAW), TINY_RAW) * 2), 'TrailingZlibData')
bad('zlib/trailing-in-later-idat', SIG + ihdr(1, 2, 8, 0) + chunk(b'IDAT', zwrap(stored(TINY_RAW), TINY_RAW)) + chunk(b'IDAT', b'\0') + IEND,
    'TrailingZlibData')
bad('zlib/cut-mid-block', base(ZB[:len(ZB) // 2]), 'TruncatedZlib')
bad('zlib/cut-mid-stored', tiny(zwrap(stored(TINY_RAW), TINY_RAW)[:6]), 'TruncatedZlib')
bad('deflate/btype-3', tiny(zwrap(bytes([0b111]), b'')), 'InvalidBlockType')
bad('deflate/stored-nlen', tiny(bytes([0x78, 0x01, 1, 4, 0, 0xFA, 0xFF]) + TINY_RAW + struct.pack('>I', zlib.adler32(TINY_RAW))), 'StoredLengthMismatch')
bw = BW()
bw.bits(1, 1)
bw.bits(1, 2)
bw.fixed_lit(286)
bad('deflate/fixed-lit-286', tiny(zwrap(bw.flush(), b'')), 'InvalidSymbol')
bw = BW()
bw.bits(1, 1)
bw.bits(1, 2)
bw.fixed_lit(0)
bw.fixed_lit(257)
bw.code(30, 5)
bad('deflate/fixed-dist-30', tiny(zwrap(bw.flush(), b'')), 'InvalidSymbol')
bw = BW()
bw.bits(1, 1)
bw.bits(1, 2)
bw.fixed_lit(257)
bw.code(0, 5)
bad('deflate/distance-before-output', tiny(zwrap(bw.flush(), b'')), 'DistanceTooFar')
bw = BW()
bw.bits(1, 1)
bw.bits(1, 2)
bw.fixed_lit(0)
bw.fixed_lit(257)
bw.code(1, 5)
bad('deflate/distance-beyond-output', tiny(zwrap(bw.flush(), b'')), 'DistanceTooFar')
# distance beyond the declared window: CINFO=0 (256 bytes) and a distance of 257..384 into a 600-byte row
WW = 600
rawW = bytes([0]) + bytes(RNG.randrange(256) for _ in range(WW))


def far(bw):
    bw.fixed_lit(257)        # length 3
    bw.code(16, 5)           # distance code 16: base 257, 7 extra bits
    bw.bits(43, 7)           # 300


bwf = fixed(rawW[:400], tail=far)
payload = rawW[:400] + bytes(rawW[100 + i] for i in range(3))
add('deflate/distance-beyond-declared-window', 'still',
    SIG + ihdr(402, 1, 8, 0) + chunk(b'IDAT', zwrap(bwf.flush(), payload, cmf=0x08)) + IEND, err('DistanceTooFar'))
add('deflate/distance-within-declared-window', 'still',
    SIG + ihdr(402, 1, 8, 0) + chunk(b'IDAT', zwrap(bwf.flush(), payload, cmf=0x18)) + IEND,
    {'width': 402, 'height': 1, 'rgba_runs': runs([(v, v, v, 255) for v in payload[1:]])})
# dynamic-header errors
for nm, kw in [('hlit-287', {'hlit': 30}), ('hdist-31', {'hdist': 30})]:
    bw = BW()
    bw.bits(1, 1)
    bw.bits(2, 2)
    dyn_header(bw, lt, [1, 1], **kw)
    bad(f'deflate/{nm}', tiny(zwrap(bw.flush() + b'\0' * 40, b'')), 'InvalidCodeLengths')
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
dyn_header(bw, lt, [1, 1], cl_override=[3] * 19)    # 19 codes of length 3: oversubscribed
bad('deflate/code-length-code-oversubscribed', tiny(zwrap(bw.flush() + b'\0' * 40, b'')), 'InvalidCodeLengths')
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
dyn_header(bw, lt, [1, 1], cl_override=[5] * 19)    # incomplete code-length code
bad('deflate/code-length-code-incomplete', tiny(zwrap(bw.flush() + b'\0' * 40, b'')), 'InvalidCodeLengths')
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
cc = dyn_header(bw, lt, [1, 1])
emit_lengths(bw, cc, [(16, 0)])
bad('deflate/repeat-with-no-previous', tiny(zwrap(bw.flush() + b'\0' * 40, b'')), 'InvalidCodeLengths')
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
cc = dyn_header(bw, lt, [1, 1])
emit_lengths(bw, cc, [(18, 127)] * 3)              # 3 x 138 zeros > 257 + 2
bad('deflate/repeat-overflows-count', tiny(zwrap(bw.flush() + b'\0' * 40, b'')), 'InvalidCodeLengths')
lt_no256 = list(lt)
lt_no256[256] = 0
bad('deflate/no-end-of-block-code', tiny(zwrap(dyn_lengths_only(lt_no256, [1, 1]) + b'\0' * 8, b'')),
    'InvalidCodeLengths')
lt_inc = [0] * 286
lt_inc[0] = lt_inc[0x41] = lt_inc[256] = 2                    # three codes of length 2: incomplete
bad('deflate/incomplete-literal-code', tiny(zwrap(dyn_lengths_only(lt_inc, [1, 1]) + b'\0' * 8, b'')), 'InvalidCodeLengths')
bad('deflate/incomplete-distance-code', tiny(zwrap(dyn_lengths_only(lt, [2, 2, 2]) + b'\0' * 8, b'')), 'InvalidCodeLengths')
bad('deflate/oversubscribed-literal-code', tiny(zwrap(dyn_lengths_only([1, 1, 1] + [0] * 253 + [1] + [0] * 29, [1, 1]) + b'\0' * 8, b'')),
    'InvalidCodeLengths')
# single literal code of length 1 (only 256): an empty block; then the data in a stored block. Unused bit '1' hits nothing.
lt_one = [0] * 257
lt_one[256] = 1
bw1 = dyn_block(b'', lt_one, [0], final=0)
bw1.bits(1, 1)
bw1.bits(0, 2)
still('deflate/single-literal-code-len1', tiny(zwrap(bw1.flush() + struct.pack('<HH', 4, 4 ^ 0xFFFF) + TINY_RAW, TINY_RAW)), 1, 2, TINY_EXP)
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
cc = dyn_header(bw, lt_one, [0])
emit_lengths(bw, cc, [(l, 0) for l in lt_one + [0]])
bw.code(1, 1)                                                  # the unassigned code '1'
bad('deflate/unassigned-code-in-incomplete-literal-code', tiny(zwrap(bw.flush(), b'')), 'InvalidSymbol')
# no distance codes, but a length symbol is used
bw = BW()
bw.bits(1, 1)
bw.bits(2, 2)
lt_m = lit_lengths(list(TINY_RAW) + [257])
cc = dyn_header(bw, lt_m, [0])
emit_lengths(bw, cc, [(l, 0) for l in lt_m + [0]])
lc = canon(lt_m)
bw.code(*lc[0])
bw.code(*lc[0x41])
bw.code(*lc[257])
bad('deflate/length-with-no-distance-codes', tiny(zwrap(bw.flush() + b'\0' * 4, b'')), 'InvalidSymbol')
bad('image/truncated-data', base(z(BASE_RAW[:-5])), 'TruncatedImageData')
bad('image/excess-data', base(z(BASE_RAW + b'\0')), 'ExcessImageData')
bad('image/excess-row', base(z(BASE_RAW + BASE_RAW[:1 + w * 4])), 'ExcessImageData')
rr = bytearray(BASE_RAW)
rr[(1 + w * 4) * 3] = 5
bad('image/filter-type-5', base(z(bytes(rr))), 'InvalidFilterType')
rr[(1 + w * 4) * 3] = 255
bad('image/filter-type-255', base(z(bytes(rr))), 'InvalidFilterType')
plte4 = pal(4)
px = [(0,), (1,), (2,), (3,), (4,), (0,)]
bad('image/palette-index-out-of-range', png(3, 2, 3, 8, px, pre=chunk(b'PLTE', plte4)), 'PaletteIndexOutOfRange')
bad('image/palette-index-out-of-range-d2', png(3, 2, 3, 2, [(0,), (1,), (3,), (0,), (0,), (0,)], pre=chunk(b'PLTE', pal(3))),
    'PaletteIndexOutOfRange')
# precedence: inflate error before filter error; filter error before index error in a later row
rr2 = bytearray(BASE_RAW)
rr2[0] = 9
bad('precedence/adler-before-filter', base(z(bytes(rr2))[:-4] + b'\0\0\0\0'), 'AdlerMismatch')
pxp = [(0,), (9,), (0,), (0,)]
rawp = bytes([0, 0, 9, 7, 0, 0])
bad('precedence/index-row0-before-filter-row1', png(2, 2, 3, 8, data=z(rawp), pre=chunk(b'PLTE', plte4)), 'PaletteIndexOutOfRange')

# interlace edge cases: every size up to 9x9 for grey-1 (empty passes) and RGBA16
for (W_, H_) in [(1, 1), (1, 2), (2, 1), (1, 5), (5, 1), (2, 2), (3, 3), (4, 4), (5, 5), (8, 8), (9, 1), (1, 9)]:
    for ct, d in [(0, 1), (6, 16), (3, 4)]:
        n = 16 if ct == 3 else None
        plte = pal(16) if ct == 3 else None
        px = rand_px(W_, H_, ct, d, n)
        still(f'interlace/ct{ct}-d{d}-{W_}x{H_}', png(W_, H_, ct, d, px, 1, pre=chunk(b'PLTE', plte) if plte else b''), W_, H_,
              [rgba_of(s, ct, d, plte) for s in px])
# an interlaced 1x1 image whose data carries one row per Adam7 pass (empty passes written anyway): excess
bad('interlace/1x1-filter-bytes-for-empty-passes', png(1, 1, 0, 8, data=z(bytes([0, 5]) + bytes(6))), 'ExcessImageData', )
V[-1]['input_hex'] = (SIG + ihdr(1, 1, 8, 0, il=1) + chunk(b'IDAT', z(bytes([0, 5]) + bytes(6))) + IEND).hex()

# ---------------------------------------------------------------- P2-P5 container errors
TPX = rand_px(2, 2, 2, 8)
TOK = png(2, 2, 2, 8, TPX)
TEXP = [rgba_of(s, 2, 8) for s in TPX]
still('container/minimal-ok', TOK, 2, 2, TEXP)
IH = SIG + ihdr(2, 2, 8, 2)
IDAT_OK = chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0)))


def doc(*chunks_):
    return IH + b''.join(chunks_)


bad('container/empty', b'', 'BadSignature')
bad('container/short-signature', SIG[:7], 'BadSignature')
bad('container/bad-signature', b'\x89PNG\r\n\x1a\x0a'.replace(b'\r\n', b'\n\r'), 'BadSignature')
bad('container/signature-only', SIG, 'TruncatedChunk')
bad('container/truncated-ihdr-header', SIG + ihdr(2, 2, 8, 2)[:6], 'TruncatedChunk')
bad('container/truncated-ihdr-crc', SIG + ihdr(2, 2, 8, 2)[:-1], 'TruncatedChunk')
bad('container/first-chunk-not-ihdr', SIG + chunk(b'gAMA', struct.pack('>I', 45455)) + ihdr(2, 2, 8, 2) + IDAT_OK + IEND, 'MissingIhdr')
bad('container/ihdr-length-12', SIG + chunk(b'IHDR', struct.pack('>IIBBBB', 2, 2, 8, 2, 0, 0)) + IDAT_OK + IEND, 'InvalidChunkLength')
bad('container/ihdr-crc', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', 2, 2, 8, 2, 0, 0, 0), crc=0) + IDAT_OK + IEND, 'CrcMismatch')
bad('container/idat-crc', doc(chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0)), crc=1), IEND), 'CrcMismatch')
bad('container/ancillary-crc', doc(chunk(b'tEXt', b'Title\0x', crc=1), IDAT_OK, IEND), 'CrcMismatch')
bad('container/unknown-ancillary-crc', doc(chunk(b'prVt', b'abc', crc=1), IDAT_OK, IEND), 'CrcMismatch')
bad('container/iend-crc', doc(IDAT_OK, chunk(b'IEND', b'', crc=0)), 'CrcMismatch')
bad('container/length-2^31', doc(struct.pack('>I', 1 << 31) + b'tEXt' + b'\0' * 8), 'ChunkTooLong')
bad('container/length-ffffffff', doc(struct.pack('>I', 0xFFFFFFFF) + b'IDAT'), 'ChunkTooLong')
bad('container/type-digit', doc(chunk(b'te1t', b''), IDAT_OK, IEND), 'InvalidChunkType')
bad('container/type-space', doc(chunk(b'IDA ', b''), IDAT_OK, IEND), 'InvalidChunkType')
bad('container/truncated-mid-idat', doc(IDAT_OK[:-6]), 'TruncatedChunk')
bad('container/length-beyond-eof', doc(chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0)), length=1000)), 'TruncatedChunk')
bad('container/missing-iend', doc(IDAT_OK), 'MissingIend')
bad('container/missing-iend-after-text', doc(IDAT_OK, chunk(b'tEXt', b'a\0b')), 'MissingIend')
bad('container/trailing-byte', TOK + b'\0', 'TrailingBytes')
bad('container/trailing-second-iend', TOK + IEND, 'TrailingBytes')
bad('container/iend-length-1', doc(IDAT_OK, chunk(b'IEND', b'\0')), 'InvalidChunkLength')
bad('container/no-idat', doc(IEND), 'MissingImageData')
bad('container/unknown-critical', doc(chunk(b'ABCD', b''), IDAT_OK, IEND), 'UnknownCriticalChunk')
bad('container/reserved-bit-on-critical', doc(chunk(b'IDaT', b''), IDAT_OK, IEND), 'UnknownCriticalChunk')
still('container/unknown-ancillary-everywhere', doc(chunk(b'abCd', b'xyz'), IDAT_OK, chunk(b'abcd', b''), IEND), 2, 2, TEXP)
still('container/reserved-bit-ancillary', doc(chunk(b'zzzz', b'1'), IDAT_OK, IEND), 2, 2, TEXP)
bad('container/duplicate-ihdr', doc(ihdr(2, 2, 8, 2), IDAT_OK, IEND), 'DuplicateChunk')
bad('container/non-consecutive-idat', doc(IDAT_OK[:0] + chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0))[:5]), chunk(b'tEXt', b'a\0b'),
                                          chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0))[5:]), IEND), 'NonConsecutiveIdat')
bad('container/idat-after-iend', TOK + IDAT_OK, 'TrailingBytes')

# ---------------------------------------------------------------- P4 IHDR
for nm, (w_, h_, dd, cc_, cm, fm, im), e in [
        ('width-0', (0, 2, 8, 2, 0, 0, 0), 'InvalidDimensions'), ('height-0', (2, 0, 8, 2, 0, 0, 0), 'InvalidDimensions'),
        ('width-16385', (16385, 1, 8, 2, 0, 0, 0), 'InvalidDimensions'), ('width-2^31', (1 << 31, 1, 8, 2, 0, 0, 0), 'InvalidDimensions'),
        ('16384x1025-output', (16384, 1025, 8, 2, 0, 0, 0), 'OutputTooLarge'),
        ('depth-3', (2, 2, 3, 0, 0, 0, 0), 'InvalidColorTypeDepth'), ('rgb-depth-4', (2, 2, 4, 2, 0, 0, 0), 'InvalidColorTypeDepth'),
        ('palette-depth-16', (2, 2, 16, 3, 0, 0, 0), 'InvalidColorTypeDepth'), ('ct-1', (2, 2, 8, 1, 0, 0, 0), 'InvalidColorTypeDepth'),
        ('ct-5', (2, 2, 8, 5, 0, 0, 0), 'InvalidColorTypeDepth'), ('ct-7', (2, 2, 8, 7, 0, 0, 0), 'InvalidColorTypeDepth'),
        ('ga-depth-1', (2, 2, 1, 4, 0, 0, 0), 'InvalidColorTypeDepth'),
        ('compression-1', (2, 2, 8, 2, 1, 0, 0), 'UnsupportedCompressionMethod'),
        ('filter-method-1', (2, 2, 8, 2, 0, 1, 0), 'UnsupportedFilterMethod'),
        ('interlace-2', (2, 2, 8, 2, 0, 0, 2), 'UnsupportedInterlaceMethod')]:
    bad(f'ihdr/{nm}', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', w_, h_, dd, cc_, cm, fm, im)) + IDAT_OK + IEND, e)
# the largest admissible output: header accepted, data then missing (shows the caps pass)
bad('ihdr/16384x1024-accepted-then-truncated-zlib', SIG + ihdr(16384, 1024, 8, 6) + chunk(b'IDAT', b'\x78\x01') + IEND, 'TruncatedZlib')
bad('limits/file-too-large', TOK, 'FileTooLarge')
V[-1]['pad_to'] = MAX_INPUT + 1

# ---------------------------------------------------------------- P6 PLTE, P7 tRNS
P3 = SIG + ihdr(2, 2, 8, 3)
P3PX = [(0,), (1,), (2,), (1,)]
P3IDAT = chunk(b'IDAT', z(scan(P3PX, 2, 2, 3, 8, 0)))
PL3 = pal(3)
P3EXP = [rgba_of(s, 3, 8, PL3) for s in P3PX]
still('plte/ok', P3 + chunk(b'PLTE', PL3) + P3IDAT + IEND, 2, 2, P3EXP)
bad('plte/missing', P3 + P3IDAT + IEND, 'MissingPalette')
bad('plte/after-idat', P3 + P3IDAT + chunk(b'PLTE', PL3) + IEND, 'MissingPalette')
bad('plte/after-idat-rgb', doc(IDAT_OK, chunk(b'PLTE', PL3), IEND), 'ChunkOutOfOrder')
bad('plte/duplicate', P3 + chunk(b'PLTE', PL3) + chunk(b'PLTE', PL3) + P3IDAT + IEND, 'DuplicateChunk')
bad('plte/length-8', P3 + chunk(b'PLTE', PL3[:8]) + P3IDAT + IEND, 'InvalidPalette')
bad('plte/length-0', P3 + chunk(b'PLTE', b'') + P3IDAT + IEND, 'InvalidPalette')
bad('plte/257-entries', SIG + ihdr(2, 2, 8, 3) + chunk(b'PLTE', pal(257)) + P3IDAT + IEND, 'InvalidPalette')
bad('plte/5-entries-depth-2', SIG + ihdr(2, 2, 2, 3) + chunk(b'PLTE', pal(5)) + chunk(b'IDAT', z(scan(P3PX, 2, 2, 3, 2, 0))) + IEND, 'InvalidPalette')
bad('plte/in-greyscale', SIG + ihdr(2, 2, 8, 0) + chunk(b'PLTE', PL3) + chunk(b'IDAT', z(scan([(1,)] * 4, 2, 2, 0, 8, 0))) + IEND, 'UnexpectedPalette')
bad('plte/in-grey-alpha', SIG + ihdr(2, 2, 8, 4) + chunk(b'PLTE', PL3) + chunk(b'IDAT', z(scan([(1, 2)] * 4, 2, 2, 4, 8, 0))) + IEND,
    'UnexpectedPalette')
still('plte/suggested-palette-in-rgb', doc(chunk(b'PLTE', pal(200)), chunk(b'bKGD', b'\0\1\0\2\0\3'), chunk(b'hIST', bytes(400)), IDAT_OK, IEND),
      2, 2, TEXP)
bad('trns/before-plte', P3 + chunk(b'tRNS', b'\0') + chunk(b'PLTE', PL3) + P3IDAT + IEND, 'ChunkOutOfOrder')
bad('trns/before-plte-rgb', doc(chunk(b'tRNS', bytes(6)), chunk(b'PLTE', PL3), IDAT_OK, IEND), 'ChunkOutOfOrder')
bad('trns/after-idat', doc(IDAT_OK, chunk(b'tRNS', bytes(6)), IEND), 'ChunkOutOfOrder')
bad('trns/duplicate', doc(chunk(b'tRNS', bytes(6)), chunk(b'tRNS', bytes(6)), IDAT_OK, IEND), 'DuplicateChunk')
bad('trns/palette-too-long', P3 + chunk(b'PLTE', PL3) + chunk(b'tRNS', b'\0' * 4) + P3IDAT + IEND, 'InvalidTransparency')
bad('trns/palette-empty', P3 + chunk(b'PLTE', PL3) + chunk(b'tRNS', b'') + P3IDAT + IEND, 'InvalidTransparency')
bad('trns/rgb-length-2', doc(chunk(b'tRNS', bytes(2)), IDAT_OK, IEND), 'InvalidTransparency')
bad('trns/grey-length-6', SIG + ihdr(2, 2, 8, 0) + chunk(b'tRNS', bytes(6)) + chunk(b'IDAT', z(scan([(1,)] * 4, 2, 2, 0, 8, 0))) + IEND,
    'InvalidTransparency')
bad('trns/in-rgba', SIG + ihdr(1, 1, 8, 6) + chunk(b'tRNS', bytes(6)) + chunk(b'IDAT', z(bytes(5))) + IEND, 'InvalidTransparency')
bad('trns/in-grey-alpha', SIG + ihdr(1, 1, 8, 4) + chunk(b'tRNS', bytes(2)) + chunk(b'IDAT', z(bytes(3))) + IEND, 'InvalidTransparency')

# ---------------------------------------------------------------- P8 ancillary chunks (valid and invalid)
ICC = bytes(range(256)) * 3
ANC_OK = [
    ('gAMA', struct.pack('>I', 45455)), ('cHRM', struct.pack('>8I', 31270, 32900, 64000, 33000, 30000, 60000, 15000, 6000)),
    ('sRGB', b'\0'), ('iCCP', b'sRGB IEC61966-2.1\0\0' + z(ICC)), ('sBIT', b'\5\6\5'), ('cICP', b'\1\15\0\1'),
    ('mDCV', bytes(24)), ('cLLI', struct.pack('>II', 1000, 400)), ('bKGD', b'\0\xff\0\0\0\x10'),
    ('pHYs', struct.pack('>IIB', 2835, 2835, 1)), ('sPLT', b'pal\0\x08' + bytes(12)), ('sPLT', b'p16\0\x10' + bytes(10)),
    ('eXIf', b'MM\0*' + bytes(4)), ('eXIf', b'II*\0'), ('tIME', struct.pack('>HBBBBB', 2026, 11, 14, 23, 59, 60)),
    ('tEXt', b'Title\0hello'), ('tEXt', b'Comment\0'), ('zTXt', b'Software\0\0' + z(b'x' * 100)), ('iTXt', b'Author\0\0\0en\0\0txt'),
    ('iTXt', b'XML:com.adobe.xmp\0\1\0\0\0' + z(b'<x/>')), ('acTL', None)]
for t, dat in ANC_OK:
    if t == 'acTL':
        continue
    pos_after = t in ('tIME', 'tEXt', 'zTXt', 'iTXt')
    b = doc(chunk(t.encode(), dat), IDAT_OK, IEND)
    still(f'anc-ok/{t}-{len(dat)}', b, 2, 2, TEXP)
    if pos_after:
        still(f'anc-ok/{t}-{len(dat)}-after-idat', doc(IDAT_OK, chunk(t.encode(), dat), IEND), 2, 2, TEXP)
still('anc-ok/srgb-and-iccp-both', doc(chunk(b'sRGB', b'\1'), chunk(b'iCCP', b'x\0\0' + z(ICC)), IDAT_OK, IEND), 2, 2, TEXP)
still('anc-ok/many-text', doc(*[chunk(b'tEXt', b'k%d\0v' % i) for i in range(5)], IDAT_OK, IEND), 2, 2, TEXP)
still('anc-ok/ztxt-with-corrupt-zlib-not-inflated', doc(chunk(b'zTXt', b'k\0\0garbage'), IDAT_OK, IEND), 2, 2, TEXP)
ANC_BAD = [
    ('gamma-0', chunk(b'gAMA', bytes(4)), 'InvalidGamma'), ('gama-len-3', chunk(b'gAMA', bytes(3)), 'InvalidChunkLength'),
    ('chrm-len-31', chunk(b'cHRM', bytes(31)), 'InvalidChunkLength'), ('srgb-4', chunk(b'sRGB', b'\4'), 'InvalidSrgb'),
    ('srgb-len-2', chunk(b'sRGB', b'\0\0'), 'InvalidChunkLength'),
    ('iccp-no-null', chunk(b'iCCP', b'name'), 'InvalidIccp'), ('iccp-empty-name', chunk(b'iCCP', b'\0\0' + z(ICC)), 'InvalidIccp'),
    ('iccp-method-1', chunk(b'iCCP', b'n\0\1' + z(ICC)), 'InvalidIccp'), ('iccp-name-80', chunk(b'iCCP', b'n' * 80 + b'\0\0' + z(ICC)), 'InvalidIccp'),
    ('iccp-leading-space', chunk(b'iCCP', b' n\0\0' + z(ICC)), 'InvalidIccp'), ('iccp-no-method', chunk(b'iCCP', b'n\0'), 'InvalidIccp'),
    ('sbit-0', chunk(b'sBIT', b'\0\5\5'), 'InvalidSbit'), ('sbit-9', chunk(b'sBIT', b'\x09\5\5'), 'InvalidSbit'),
    ('sbit-len-4', chunk(b'sBIT', b'\5\5\5\5'), 'InvalidChunkLength'),
    ('cicp-matrix-1', chunk(b'cICP', b'\1\15\1\1'), 'InvalidCicp'), ('cicp-range-2', chunk(b'cICP', b'\1\15\0\2'), 'InvalidCicp'),
    ('cicp-len-5', chunk(b'cICP', b'\1\15\0\1\0'), 'InvalidChunkLength'), ('mdcv-len-23', chunk(b'mDCV', bytes(23)), 'InvalidChunkLength'),
    ('clli-len-9', chunk(b'cLLI', bytes(9)), 'InvalidChunkLength'),
    ('bkgd-len-2-rgb', chunk(b'bKGD', bytes(2)), 'InvalidChunkLength'), ('bkgd-high-bits', chunk(b'bKGD', b'\1\0\0\0\0\0'), 'InvalidBackground'),
    ('phys-unit-2', chunk(b'pHYs', struct.pack('>IIB', 1, 1, 2)), 'InvalidPhys'), ('phys-len-8', chunk(b'pHYs', bytes(8)), 'InvalidChunkLength'),
    ('splt-depth-4', chunk(b'sPLT', b'p\0\x04' + bytes(6)), 'InvalidSplt'), ('splt-len', chunk(b'sPLT', b'p\0\x08' + bytes(7)), 'InvalidSplt'),
    ('splt-no-null', chunk(b'sPLT', b'p'), 'InvalidSplt'),
    ('exif-bad-magic', chunk(b'eXIf', b'MM\0\0'), 'InvalidExif'), ('exif-len-3', chunk(b'eXIf', b'MM\0'), 'InvalidExif'),
    ('time-month-13', chunk(b'tIME', struct.pack('>HBBBBB', 2026, 13, 1, 0, 0, 0)), 'InvalidTime'),
    ('time-second-61', chunk(b'tIME', struct.pack('>HBBBBB', 2026, 1, 1, 0, 0, 61)), 'InvalidTime'),
    ('time-len-8', chunk(b'tIME', bytes(8)), 'InvalidChunkLength'),
    ('text-no-null', chunk(b'tEXt', b'Title'), 'InvalidText'), ('text-empty-keyword', chunk(b'tEXt', b'\0x'), 'InvalidText'),
    ('text-keyword-nbsp', chunk(b'tEXt', b'a\xa0b\0x'), 'InvalidText'), ('text-keyword-double-space', chunk(b'tEXt', b'a  b\0x'), 'InvalidText'),
    ('text-keyword-control', chunk(b'tEXt', b'a\x01\0x'), 'InvalidText'),
    ('ztxt-method-1', chunk(b'zTXt', b'k\0\1' + z(b'x')), 'InvalidText'), ('ztxt-no-method', chunk(b'zTXt', b'k\0'), 'InvalidText'),
    ('itxt-flag-2', chunk(b'iTXt', b'k\0\2\0\0\0'), 'InvalidText'), ('itxt-compressed-method-1', chunk(b'iTXt', b'k\0\1\1\0\0'), 'InvalidText'),
    ('itxt-missing-separators', chunk(b'iTXt', b'k\0\0\0en'), 'InvalidText')]
for nm, ch_, e in ANC_BAD:
    bad(f'anc-bad/{nm}', doc(ch_, IDAT_OK, IEND), e)
still('anc-ok/itxt-uncompressed-method-ignored', doc(chunk(b'iTXt', b'k\0\0\7\0\0x'), IDAT_OK, IEND), 2, 2, TEXP)
bad('anc-bad/hist-without-plte', doc(chunk(b'hIST', bytes(6)), IDAT_OK, IEND), 'ChunkOutOfOrder')
bad('anc-bad/hist-wrong-length', P3 + chunk(b'PLTE', PL3) + chunk(b'hIST', bytes(4)) + P3IDAT + IEND, 'InvalidHistogram')
still('anc-ok/hist', P3 + chunk(b'PLTE', PL3) + chunk(b'hIST', bytes(6)) + P3IDAT + IEND, 2, 2, P3EXP)
bad('anc-bad/bkgd-index-3-of-3', P3 + chunk(b'PLTE', PL3) + chunk(b'bKGD', b'\3') + P3IDAT + IEND, 'InvalidBackground')
still('anc-ok/bkgd-index-2', P3 + chunk(b'PLTE', PL3) + chunk(b'bKGD', b'\2') + P3IDAT + IEND, 2, 2, P3EXP)
bad('anc-bad/bkgd-before-plte-palette', P3 + chunk(b'bKGD', b'\2') + chunk(b'PLTE', PL3) + P3IDAT + IEND, 'ChunkOutOfOrder')
bad('anc-bad/sbit-palette-9', P3 + chunk(b'sBIT', b'\x09\1\1') + chunk(b'PLTE', PL3) + P3IDAT + IEND, 'InvalidSbit')
still('anc-ok/sbit-palette-8', P3 + chunk(b'sBIT', b'\x08\1\1') + chunk(b'PLTE', PL3) + P3IDAT + IEND, 2, 2, P3EXP)
for t, dat in [('gAMA', struct.pack('>I', 45455)), ('cHRM', bytes(32)), ('sRGB', b'\0'), ('iCCP', b'n\0\0' + z(ICC)), ('sBIT', b'\5\5\5'),
               ('cICP', b'\1\15\0\1'), ('mDCV', bytes(24)), ('cLLI', bytes(8))]:
    bad(f'order/{t}-after-plte', doc(chunk(b'PLTE', PL3), chunk(t.encode(), dat), IDAT_OK, IEND), 'ChunkOutOfOrder')
    bad(f'order/{t}-after-idat', doc(IDAT_OK, chunk(t.encode(), dat), IEND), 'ChunkOutOfOrder')
    bad(f'order/{t}-duplicate', doc(chunk(t.encode(), dat), chunk(t.encode(), dat), IDAT_OK, IEND), 'DuplicateChunk')
for t, dat in [('pHYs', struct.pack('>IIB', 1, 1, 0)), ('eXIf', b'II*\0'), ('sPLT', b'p\0\x08')]:
    bad(f'order/{t}-after-idat', doc(IDAT_OK, chunk(t.encode(), dat), IEND), 'ChunkOutOfOrder')
for t, dat in [('pHYs', struct.pack('>IIB', 1, 1, 0)), ('eXIf', b'II*\0'), ('tIME', struct.pack('>HBBBBB', 2000, 1, 1, 0, 0, 0)),
               ('bKGD', bytes(6))]:
    bad(f'order/{t}-duplicate', doc(chunk(t.encode(), dat), chunk(t.encode(), dat), IDAT_OK, IEND), 'DuplicateChunk')
still('order/splt-twice', doc(chunk(b'sPLT', b'a\0\x08'), chunk(b'sPLT', b'b\0\x08'), IDAT_OK, IEND), 2, 2, TEXP)
still('order/bkgd-hist-after-plte-before-idat-rgb', doc(chunk(b'PLTE', PL3), chunk(b'hIST', bytes(6)), chunk(b'bKGD', bytes(6)), IDAT_OK, IEND),
      2, 2, TEXP)

# ---------------------------------------------------------------- P17 icc_profile
add('icc/present', 'icc', doc(chunk(b'iCCP', b'n\0\0' + z(ICC)), IDAT_OK, IEND),
    {'icc_len': len(ICC), 'icc_sha256': hashlib.sha256(ICC).hexdigest()})
add('icc/absent', 'icc', TOK, {'icc': None})
add('icc/bad-adler', 'icc', doc(chunk(b'iCCP', b'n\0\0' + z(ICC)[:-1] + b'\0'), IDAT_OK, IEND), err('AdlerMismatch'))
add('icc/structural-error-first', 'icc', doc(chunk(b'iCCP', b'n\0\0' + z(ICC)), IDAT_OK), err('MissingIend'))
big = z(bytes(MAX_ICC + 1), level=9)
add('icc/too-large', 'icc', doc(chunk(b'iCCP', b'n\0\0' + big), IDAT_OK, IEND), err('IccProfileTooLarge'))
exact = z(bytes(MAX_ICC), level=9)
add('icc/exactly-max', 'icc', doc(chunk(b'iCCP', b'n\0\0' + exact), IDAT_OK, IEND),
    {'icc_len': MAX_ICC, 'icc_sha256': hashlib.sha256(bytes(MAX_ICC)).hexdigest()})
still('icc/still-ignores-icc-payload-errors', doc(chunk(b'iCCP', b'n\0\0' + b'\x78\x9c\xff'), IDAT_OK, IEND), 2, 2, TEXP)

# ---------------------------------------------------------------- P14, P16 APNG
AW, AH = 4, 3


def actl(n, plays=0):
    return chunk(b'acTL', struct.pack('>II', n, plays))


def fctl(seq, w_, h_, x, y, dn=1, dd=10, dop=0, bop=0):
    return chunk(b'fcTL', struct.pack('>IIIIIHHBB', seq, w_, h_, x, y, dn, dd, dop, bop))


def fdat(seq, data):
    return chunk(b'fdAT', struct.pack('>I', seq) + data)


def rgba_frame(w_, h_, colours):
    return [colours[i % len(colours)] for i in range(w_ * h_)]


def zframe(px, w_, h_, il=0):
    return z(scan(px, w_, h_, 6, 8, il))


AHDR = SIG + ihdr(AW, AH, 8, 6)
RED, GRN, BLU, CLR = (255, 0, 0, 255), (0, 255, 0, 255), (0, 0, 255, 255), (0, 0, 0, 0)
HALF = (0, 0, 255, 128)
F0 = rgba_frame(AW, AH, [RED])
F1 = rgba_frame(2, 2, [GRN])
F2 = rgba_frame(2, 1, [HALF])


def compose(frames):
    """Generator's own canvas model (P14 restated): frames = [(rect, px, dop, bop)]."""
    cv = [CLR] * (AW * AH)
    out, prev, saved = [], None, None
    for k, (rect, px, dop, bop) in enumerate(frames):
        if prev:
            (x0, y0, w_, h_), op = prev
            if op == 1:
                for y in range(y0, y0 + h_):
                    for x in range(x0, x0 + w_):
                        cv[y * AW + x] = CLR
            elif op == 2:
                cv = saved
        op = 1 if k == 0 and dop == 2 else dop
        if op == 2:
            saved = list(cv)
        x0, y0, w_, h_ = rect
        for y in range(h_):
            for x in range(w_):
                s, dd = px[y * w_ + x], cv[(y0 + y) * AW + x0 + x]
                if bop == 1 and s[3] != 255:
                    if s[3] == 0:
                        s = dd
                    else:
                        A = 255 * s[3] + dd[3] * (255 - s[3])
                        s = tuple((2 * (255 * s[3] * s[i] + dd[3] * (255 - s[3]) * dd[i]) + A) // (2 * A) for i in range(3)) + ((2 * A + 255) // 510,)
                cv[(y0 + y) * AW + x0 + x] = s
        prev = (rect, op)
        out.append(list(cv))
    return out


def frames_exp(fr, meta, end='iend', plays=0):
    cvs = compose(fr)
    return {'width': AW, 'height': AH, 'num_plays': plays, 'end': end,
            'frames': [{'rect': list(f[0]), 'delay': list(m[0]), 'dispose': m[1], 'blend': m[2], 'rgba_runs': runs(c)}
                       for f, m, c in zip(fr, meta, cvs)]}


Z0, Z1, Z2 = zframe(F0, AW, AH), zframe(F1, 2, 2), zframe(F2, 2, 1)
# default image is frame 0
b = AHDR + actl(3, 2) + fctl(0, AW, AH, 0, 0, 5, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 1, 1, 1, 10, 1, 0) + fdat(2, Z1) \
    + fctl(3, 2, 1, 0, 0, 3, 100, 0, 1) + fdat(4, Z2[:3]) + fdat(5, Z2[3:]) + IEND
fr = [((0, 0, AW, AH), F0, 0, 0), ((1, 1, 2, 2), F1, 1, 0), ((0, 0, 2, 1), F2, 0, 1)]
add('apng/default-is-frame0-dispose-bg-blend-over', 'frames', b, frames_exp(fr, [((5, 0), 0, 0), ((1, 10), 1, 0), ((3, 100), 0, 1)], plays=2))
still('apng/still-is-default-image', b, AW, AH, F0)
# default image not part of the animation
b = AHDR + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, 1, 1, 0, 0) + fdat(1, Z1) + fctl(2, 2, 1, 2, 2, 1, 1, 0, 0) + fdat(3, Z2) + IEND
fr = [((0, 0, 2, 2), F1, 0, 0), ((2, 2, 2, 1), F2, 0, 0)]
add('apng/default-not-in-animation', 'frames', b, frames_exp(fr, [((1, 1), 0, 0), ((1, 1), 0, 0)]))
still('apng/default-not-in-animation-still', b, AW, AH, F0)
# dispose previous, including on the first frame (treated as background)
b = AHDR + actl(3) + fctl(0, AW, AH, 0, 0, 1, 1, 2, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 1, 1, 2, 0) + fdat(2, Z1) \
    + fctl(3, 2, 1, 1, 1, 1, 1, 0, 1) + fdat(4, Z2) + IEND
fr = [((0, 0, AW, AH), F0, 2, 0), ((0, 0, 2, 2), F1, 2, 0), ((1, 1, 2, 1), F2, 0, 1)]
add('apng/dispose-previous-first-frame-and-later', 'frames', b, frames_exp(fr, [((1, 1), 2, 0), ((1, 1), 2, 0), ((1, 1), 0, 1)]))
# OVER on transparent and on semi-transparent destinations, various alphas
FA = [(10, 200, 30, 0), (10, 200, 30, 1), (10, 200, 30, 127), (10, 200, 30, 254), (200, 10, 99, 64), (1, 2, 3, 200)]
FB = [(250, 5, 77, a) for a in (0, 255, 128, 3, 200, 90)]
ZA, ZB_ = zframe(FA, 3, 2), zframe(FB, 3, 2)
b = AHDR + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 3, 2, 0, 0) + fdat(1, ZB_) + fctl(2, 3, 2, 1, 1, bop=1) + fdat(3, ZA) + IEND
fr = [((0, 0, 3, 2), FB, 0, 0), ((1, 1, 3, 2), FA, 0, 1)]
add('apng/blend-over-arithmetic', 'frames', b, frames_exp(fr, [((1, 10), 0, 0), ((1, 10), 0, 1)]))
# interlaced animation
FI = rgba_frame(3, 3, [RED, GRN, BLU, HALF])
b = SIG + ihdr(AW, AH, 8, 6, il=1) + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', zframe(F0, AW, AH, 1)) \
    + fctl(1, 3, 3, 1, 0) + fdat(2, zframe(FI, 3, 3, 1)) + IEND
fr = [((0, 0, AW, AH), F0, 0, 0), ((1, 0, 3, 3), FI, 0, 0)]
add('apng/interlaced-subframe', 'frames', b, frames_exp(fr, [((1, 10), 0, 0), ((1, 10), 0, 0)]))
# no acTL: one frame, the default image
add('apng/static-file-as-frames', 'frames', AHDR + chunk(b'IDAT', Z0) + IEND,
    {'width': AW, 'height': AH, 'num_plays': None, 'end': 'iend',
     'frames': [{'rect': [0, 0, AW, AH], 'delay': None, 'dispose': 0, 'blend': 0, 'rgba_runs': runs(F0)}]})
# a frame-data error in frame 1 ends the sequence after frame 0
b = AHDR + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:-1] + b'\0') + IEND
add('apng/frame1-adler-error-ends', 'frames', b, frames_exp([((0, 0, AW, AH), F0, 0, 0)], [((1, 10), 0, 0)], end={'error': 'AdlerMismatch'}))
still('apng/frame1-error-still-unaffected', b, AW, AH, F0)
b = AHDR + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, zframe(F1 + F1[:1], 2, 2)[:0] + z(scan(F1, 2, 2, 6, 8, 0)[:-1])) + IEND
add('apng/frame1-truncated-data-ends', 'frames', b, frames_exp([((0, 0, AW, AH), F0, 0, 0)], [((1, 10), 0, 0)], end={'error': 'TruncatedImageData'}))
# structural APNG errors (whole call fails)
G = AHDR
ok_tail = fctl(1, 2, 2, 0, 0) + fdat(2, Z1) + IEND
for nm, bb, e in [
        ('actl-zero-frames', G + actl(0) + chunk(b'IDAT', Z0) + IEND, 'InvalidAnimationControl'),
        ('actl-too-many-frames', G + actl(65537) + chunk(b'IDAT', Z0) + IEND, 'TooManyFrames'),
        ('actl-after-idat', G + chunk(b'IDAT', Z0) + actl(1) + IEND, 'ChunkOutOfOrder'),
        ('actl-duplicate', G + actl(1) + actl(1) + chunk(b'IDAT', Z0) + IEND, 'DuplicateChunk'),
        ('actl-len-4', G + chunk(b'acTL', bytes(4)) + chunk(b'IDAT', Z0) + IEND, 'InvalidChunkLength'),
        ('fctl-without-actl', G + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'MissingAnimationControl'),
        ('fdat-without-actl', G + chunk(b'IDAT', Z0) + fdat(0, Z1) + IEND, 'MissingAnimationControl'),
        ('no-fctl-at-all', G + actl(1) + chunk(b'IDAT', Z0) + IEND, 'FrameCountMismatch'),
        ('fewer-frames-than-declared', G + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail, 'FrameCountMismatch'),
        ('more-frames-than-declared', G + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail, 'FrameCountMismatch'),
        ('seq-starts-at-1', G + actl(2) + fctl(1, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'SequenceError'),
        ('seq-gap', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'SequenceError'),
        ('seq-fdat-repeats', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:4]) + fdat(2, Z1[4:]) + IEND,
         'SequenceError'),
        ('fdat-before-fctl', G + actl(1) + chunk(b'IDAT', Z0) + fdat(0, Z1) + fctl(1, 2, 2, 0, 0) + IEND, 'UnexpectedFrameData'),
        ('fdat-for-idat-frame', G + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fdat(1, Z1) + IEND, 'UnexpectedFrameData'),
        ('fdat-before-idat', G + actl(1) + fctl(0, AW, AH, 0, 0) + fdat(1, Z1) + chunk(b'IDAT', Z0) + IEND, 'ChunkOutOfOrder'),
        ('fdat-len-3', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + chunk(b'fdAT', b'\0\0\2') + IEND,
         'InvalidChunkLength'),
        ('fctl-no-fdat-then-iend', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + IEND, 'MissingFrameData'),
        ('fctl-no-fdat-then-fctl', G + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fctl(2, 2, 2, 0, 0)
         + fdat(3, Z1) + IEND, 'MissingFrameData'),
        ('two-fctl-before-idat', G + actl(2) + fctl(0, AW, AH, 0, 0) + fctl(1, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'ChunkOutOfOrder'),
        ('fctl-len-25', G + actl(1) + chunk(b'fcTL', bytes(25)) + chunk(b'IDAT', Z0) + IEND, 'InvalidChunkLength'),
        ('fctl-width-0', G + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 0, 2, 0, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('fctl-outside-x', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 3, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('fctl-outside-y', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 2) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('fctl-offset-overflow-u32', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0xFFFFFFFF, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('fctl-dispose-3', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, dop=3) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('fctl-blend-2', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, bop=2) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
        ('default-fctl-smaller', G + actl(1) + fctl(0, 2, 2, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'InvalidFrameControl'),
        ('default-fctl-offset', G + actl(1) + fctl(0, AW - 1, AH, 1, 0) + chunk(b'IDAT', Z0) + IEND, 'InvalidFrameControl'),
        ('fctl-crc', G + actl(1) + chunk(b'IDAT', Z0) + chunk(b'fcTL', struct.pack('>IIIIIHHBB', 0, 2, 2, 0, 0, 1, 1, 0, 0), crc=5) + fdat(1, Z1) + IEND,
         'CrcMismatch')]:
    add(f'apng-bad/{nm}', 'frames', bb, err(e))
    if nm in ('actl-zero-frames', 'fctl-without-actl', 'seq-gap', 'fdat-for-idat-frame'):
        bad(f'apng-bad/{nm}-still', bb, e)
# pixel budget: 65 full 8192x2048 frames (2^24 px each) exceed 2^30; structural walk stops before any inflate
BW_, BH_ = 8192, 2048
bb = SIG + ihdr(BW_, BH_, 8, 6) + actl(65) + chunk(b'IDAT', b'\x78\x01')
seq = 0
for k in range(65):
    bb += fctl(seq, BW_, BH_, 0, 0)
    bb += fdat(seq + 1, b'\x78\x01')
    seq += 2
add('apng-bad/pixel-budget', 'frames', bb + IEND, err('PixelBudgetExceeded'))
bb = SIG + ihdr(BW_, BH_, 8, 6) + actl(64) + chunk(b'IDAT', b'\x78\x01')
seq = 0
for k in range(64):
    bb += fctl(seq, BW_, BH_, 0, 0) + fdat(seq + 1, b'\x78\x01')
    seq += 2
add('apng-bad/pixel-budget-exactly-2^30-then-zlib', 'frames', bb + IEND, err('TruncatedZlib'))

# ---------------------------------------------------------------- write
if __name__ == '__main__':
    names = [v['name'] for v in V]
    assert len(names) == len(set(names)), [n for n in names if names.count(n) > 1]
    (ROOT / 'VECTORS.json').write_text(json.dumps(V, indent=0) + '\n')
    print(len(V), 'vectors')
