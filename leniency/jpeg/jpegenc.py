"""Minimal pure-Python JPEG encoder for seeds libjpeg-turbo 2.1 (8-bit build) cannot write:
12-bit extended sequential (SOF1, P=12) and lossless (SOF3, P=8/12/16, predictor 1..7).
Huffman tables are optimal per image (JPEG Annex K.2, code lengths limited to 16).
"""
import math
import struct

ZIGZAG = [0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21,
          28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61,
          54, 47, 55, 62, 63]


def seg(marker, payload):
    return bytes([0xFF, marker]) + struct.pack('>H', len(payload) + 2) + payload


def huff_lengths(freq):
    """Annex K.2: code size per symbol from frequencies, limited to 16 bits."""
    freq = dict(freq)
    freq[256] = 1  # reserved symbol guarantees no all-ones code
    syms = list(freq)
    f = {s: freq[s] for s in syms}
    size = {s: 0 for s in syms}
    other = {s: None for s in syms}
    while True:
        live = [s for s in syms if f[s] > 0]
        if len(live) < 2:
            break
        live.sort(key=lambda s: (f[s], -s))
        v1, v2 = live[0], live[1]
        # merge v1 (smallest) with v2 (next smallest)
        f[v2] += f[v1]
        f[v1] = 0
        a = v2
        while True:
            size[a] += 1
            if other[a] is None:
                break
            a = other[a]
        other[a] = v1
        b = v1
        while True:
            size[b] += 1
            if other[b] is None:
                break
            b = other[b]
    bits = [0] * 33
    for s in syms:
        if size[s]:
            bits[size[s]] += 1
    i = 32
    while i > 16:
        while bits[i] > 0:
            j = i - 2
            while bits[j] == 0:
                j -= 1
            bits[i] -= 2
            bits[i - 1] += 1
            bits[j + 1] += 2
            bits[j] -= 1
        i -= 1
    while bits[i] == 0:
        i -= 1
    bits[i] -= 1  # drop reserved symbol
    order = sorted((s for s in syms if s != 256 and size[s]), key=lambda s: (size[s], s))
    return bits[1:17], order


def huff_table(freq):
    bits, vals = huff_lengths(freq)
    code, k, table = 0, 0, {}
    for length in range(1, 17):
        for _ in range(bits[length - 1]):
            table[vals[k]] = (code, length)
            code += 1
            k += 1
        code <<= 1
    return bits, vals, table


class BitWriter:
    def __init__(self):
        self.out = bytearray()
        self.acc = 0
        self.n = 0

    def put(self, value, length):
        for i in range(length - 1, -1, -1):
            self.acc = (self.acc << 1) | ((value >> i) & 1)
            self.n += 1
            if self.n == 8:
                self.out.append(self.acc)
                if self.acc == 0xFF:
                    self.out.append(0)
                self.acc = self.n = 0

    def flush(self):
        if self.n:
            self.put((1 << (8 - self.n)) - 1, 8 - self.n)
        return bytes(self.out)


def category(v):
    v = abs(v)
    return v.bit_length()


def extra(v, s):
    return v if v >= 0 else v + (1 << s) - 1


def dht(tc, th, bits, vals):
    return seg(0xC4, bytes([(tc << 4) | th]) + bytes(bits) + bytes(vals))


def encode_dct(planes, w, h, precision=12, quality_scale=1):
    """SOF1 extended sequential Huffman; planes: 1 or 3 row-major sample lists in 0..2^P-1; no subsampling or colour transform."""
    nc = len(planes)
    q = [max(1, min(65535, int((8 + (k % 8) + (k // 8)) * (16 if precision == 12 else 1) * quality_scale // 4 + 1))) for k in range(64)]
    cos = [[math.cos((2 * x + 1) * u * math.pi / 16) for x in range(8)] for u in range(8)]
    bw, bh = (w + 7) // 8, (h + 7) // 8
    blocks = []  # per MCU, per component: list of (dc_diff, ac list of (run,val))
    pred = [0] * nc
    for by in range(bh):
        for bx in range(bw):
            mcu = []
            for c in range(nc):
                px = [planes[c][min(h - 1, by * 8 + y) * w + min(w - 1, bx * 8 + x)] - (1 << (precision - 1)) for y in range(8) for x in range(8)]
                coef = []
                for v in range(8):
                    for u in range(8):
                        s = sum(px[y * 8 + x] * cos[u][x] * cos[v][y] for y in range(8) for x in range(8))
                        s *= 0.25 * (1 / math.sqrt(2) if u == 0 else 1) * (1 / math.sqrt(2) if v == 0 else 1)
                        coef.append(s)
                zz = [int(round(coef[ZIGZAG[k]] / q[ZIGZAG[k]])) for k in range(64)]
                dc = zz[0] - pred[c]
                pred[c] = zz[0]
                ac, run = [], 0
                for k in range(1, 64):
                    if zz[k] == 0:
                        run += 1
                        continue
                    while run > 15:
                        ac.append((15, 0))
                        run -= 16
                    ac.append((run, zz[k]))
                    run = 0
                if run:
                    ac.append((0, None))  # EOB
                mcu.append((dc, ac))
            blocks.append(mcu)
    dcf, acf = {}, {}
    for mcu in blocks:
        for dc, ac in mcu:
            dcf[category(dc)] = dcf.get(category(dc), 0) + 1
            for r, v in ac:
                sym = 0 if v is None else (r << 4) | category(v)
                acf[sym] = acf.get(sym, 0) + 1
    dbits, dvals, dtab = huff_table(dcf)
    abits, avals, atab = huff_table(acf)
    bwr = BitWriter()
    for mcu in blocks:
        for dc, ac in mcu:
            s = category(dc)
            bwr.put(*dtab[s])
            if s:
                bwr.put(extra(dc, s), s)
            for r, v in ac:
                if v is None:
                    bwr.put(*atab[0])
                    continue
                s = category(v)
                bwr.put(*atab[(r << 4) | s])
                if s:
                    bwr.put(extra(v, s), s)
    dqt = seg(0xDB, bytes([0x10]) + b''.join(struct.pack('>H', q[ZIGZAG[k]]) for k in range(64)))
    sof = seg(0xC1, bytes([precision]) + struct.pack('>HHB', h, w, nc) + b''.join(bytes([c + 1, 0x11, 0]) for c in range(nc)))
    sos = seg(0xDA, bytes([nc]) + b''.join(bytes([c + 1, 0x00]) for c in range(nc)) + bytes([0, 63, 0]))
    return b'\xff\xd8' + dqt + sof + dht(0, 0, dbits, dvals) + dht(1, 0, abits, avals) + sos + bwr.flush() + b'\xff\xd9'


def encode_lossless(planes, w, h, precision=8, predictor=1):
    """SOF3 lossless, H=V=1 for every component, interleaved, point transform 0."""
    nc = len(planes)
    diffs = []
    for y in range(h):
        for x in range(w):
            for c in range(nc):
                p = planes[c]
                if x == 0 and y == 0:
                    pr = 1 << (precision - 1)
                elif y == 0:
                    pr = p[x - 1]
                elif x == 0:
                    pr = p[(y - 1) * w]
                else:
                    ra, rb, rc = p[y * w + x - 1], p[(y - 1) * w + x], p[(y - 1) * w + x - 1]
                    pr = [0, ra, rb, rc, ra + rb - rc, ra + ((rb - rc) >> 1), rb + ((ra - rc) >> 1), (ra + rb) // 2][predictor]
                d = (p[y * w + x] - pr) & 0xFFFF
                if d >= 0x8000:
                    d -= 0x10000
                diffs.append(d)
    freq = {}
    for d in diffs:
        s = 16 if d == -32768 else category(d)
        freq[s] = freq.get(s, 0) + 1
    bits, vals, tab = huff_table(freq)
    bwr = BitWriter()
    for d in diffs:
        s = 16 if d == -32768 else category(d)
        bwr.put(*tab[s])
        if s and s < 16:
            bwr.put(extra(d, s), s)
    sof = seg(0xC3, bytes([precision]) + struct.pack('>HHB', h, w, nc) + b''.join(bytes([c + 1, 0x11, 0]) for c in range(nc)))
    sos = seg(0xDA, bytes([nc]) + b''.join(bytes([c + 1, 0x00]) for c in range(nc)) + bytes([predictor, 0, 0]))
    return b'\xff\xd8' + sof + dht(0, 0, bits, vals) + sos + bwr.flush() + b'\xff\xd9'
