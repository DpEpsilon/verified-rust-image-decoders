"""Deterministic synthetic TIFF writer for leniency seeds (classic and BigTIFF, II/MM).

Covers strips/tiles, compression None/PackBits/LZW/Deflate(8, 32946)/JPEG(7),
predictors 2 and 3, chunky/planar, photometric 0/1/2/3/5/6/8, 1..64-bit samples incl.
signed and IEEE float, ExtraSamples, Orientation and multiple IFDs. JPEG strips are
written by the libjpeg-turbo-based jpeggen helper.
"""
import math
import random
import struct
import subprocess
import zlib
from pathlib import Path

TYPES = {1: 'B', 2: 's', 3: 'H', 4: 'I', 5: 'II', 6: 'b', 7: 'B', 8: 'h', 9: 'i', 11: 'f', 12: 'd', 16: 'Q', 17: 'q'}
SIZE = {1: 1, 2: 1, 3: 2, 4: 4, 5: 8, 6: 1, 7: 1, 8: 2, 9: 4, 10: 8, 11: 4, 12: 8, 13: 4, 16: 8, 17: 8, 18: 8}
JPEGGEN = Path('/tmp/claude-0/leniency-jt-bin/jpeggen')


def packbits(data):
    out, i, n = bytearray(), 0, len(data)
    while i < n:
        j = i
        while j + 1 < n and data[j + 1] == data[i] and j - i < 127:
            j += 1
        if j > i:
            out += bytes([257 - (j - i + 1), data[i]])
            i = j + 1
            continue
        j = i
        while j < n and j - i < 128 and not (j + 1 < n and data[j + 1] == data[j]):
            j += 1
        j = max(j, i + 1)
        out += bytes([j - i - 1]) + data[i:j]
        i = j
    return bytes(out)


def lzw(data):
    """TIFF LZW (MSB-first, early change), mirroring libtiff's LZWEncode/LZWPostEncode."""
    out, acc, nacc = bytearray(), 0, 0
    width, nxt = 9, 258
    table = {bytes([i]): i for i in range(256)}
    def put(code):
        nonlocal acc, nacc
        acc = (acc << width) | code
        nacc += width
        while nacc >= 8:
            out.append((acc >> (nacc - 8)) & 0xFF)
            nacc -= 8
    put(256)
    w = b''
    for c in data:
        wc = w + bytes([c])
        if wc in table:
            w = wc
            continue
        put(table[w])
        if nxt == 4094:
            put(256)
            table = {bytes([i]): i for i in range(256)}
            width, nxt = 9, 258
        else:
            table[wc] = nxt
            nxt += 1
            if nxt > (1 << width) - 1:
                width += 1
        w = bytes([c])
    if w:
        put(table[w])
        nxt += 1
        if nxt == 4094:
            put(256)
            width = 9
        elif nxt > (1 << width) - 1:
            width += 1
    put(257)
    if nacc:
        out.append((acc << (8 - nacc)) & 0xFF)
    return bytes(out)


def sample_values(w, h, spp, seed):
    """Floats in [0,1] per pixel and channel: gradients, an edge, seeded noise."""
    rng = random.Random(seed)
    vals = []
    for y in range(h):
        for x in range(w):
            edge = 0.25 if (x * 3 > w * 2) ^ (y * 2 > h) else 0.0
            base = [x / max(1, w - 1), y / max(1, h - 1), (x + y) / max(1, w + h - 2), 0.3 + 0.7 * ((x * y) % 7) / 6]
            px = []
            for c in range(spp):
                v = base[c % 4] * 0.7 + edge + rng.random() * 0.04
                px.append(min(1.0, max(0.0, v)))
            vals.append(px)
    return vals


def pack_row(samples, bps, fmt, endian):
    """samples: list of floats in [0,1] (or palette indices when fmt=='index')."""
    if fmt == 'float':
        code = {16: 'e', 32: 'f', 64: 'd'}[bps]
        return struct.pack(endian + code * len(samples), *samples)
    if fmt == 'int':
        code = {8: 'b', 16: 'h', 32: 'i'}[bps]
        top = (1 << (bps - 1)) - 1
        return struct.pack(endian + code * len(samples), *[int(round(s * 2 * top - top)) for s in samples])
    ints = [int(s) if fmt == 'index' else int(round(s * ((1 << bps) - 1))) for s in samples]
    if bps < 8:
        acc, n, out = 0, 0, bytearray()
        for v in ints:
            acc = (acc << bps) | v
            n += bps
            if n >= 8:
                out.append((acc >> (n - 8)) & 0xFF)
                n -= 8
        if n:
            out.append((acc << (8 - n)) & 0xFF)
        return bytes(out)
    code = {8: 'B', 16: 'H', 32: 'I', 64: 'Q'}[bps]
    return struct.pack(endian + code * len(ints), *ints)


def predict2(row, bps, spp, endian):
    if bps == 8:
        a = bytearray(row)
        for i in range(len(a) - 1, spp - 1, -1):
            a[i] = (a[i] - a[i - spp]) & 0xFF
        return bytes(a)
    code = {16: 'H', 32: 'I'}[bps]
    n = len(row) // (bps // 8)
    v = list(struct.unpack(endian + code * n, row))
    for i in range(n - 1, spp - 1, -1):
        v[i] = (v[i] - v[i - spp]) & ((1 << bps) - 1)
    return struct.pack(endian + code * n, *v)


def predict3(row, bps, spp, endian):
    """Adobe floating-point predictor: most-significant-first byte planes (independent of
    the file byte order, as in libtiff's fpAcc), then byte differencing."""
    nb = bps // 8
    n = len(row) // nb
    out = bytearray(len(row))
    for i in range(n):
        sample = row[i * nb:(i + 1) * nb]
        if endian == '<':
            sample = sample[::-1]
        for b in range(nb):
            out[b * n + i] = sample[b]
    for i in range(len(out) - 1, spp - 1, -1):
        out[i] = (out[i] - out[i - spp]) & 0xFF
    return bytes(out)


def jpeg_strip(raw, w, h, kind, params):
    tmp = Path('/tmp/claude-0/leniency-tiff/gen')
    tmp.mkdir(parents=True, exist_ok=True)
    (tmp / 'strip.raw').write_bytes(raw)
    args = [str(JPEGGEN), str(tmp / 'strip.jpg'), str(tmp / 'strip.raw'), f'w={w}', f'h={h}', f'in={kind}', 'jfif=0'] + params
    subprocess.run(args, check=True, capture_output=True)
    return (tmp / 'strip.jpg').read_bytes()


class Page:
    def __init__(self, w, h, photometric, bps, spp, fmt='uint', compression=1, predictor=1, planar=1, rows=None,
                 tile=None, extra=None, orientation=None, ycbcr_sub=None, seed=0, subfile=None, fill_order=None, ink=None):
        self.__dict__.update(locals())
        del self.__dict__['self']


def encode_chunks(pg, endian):
    """Returns (list of chunk bytes, extra tags dict)."""
    w, h, spp, bps = pg.w, pg.h, pg.spp, pg.bps
    vals = sample_values(w, h, spp, pg.seed)
    if pg.extra == 1:  # associated alpha: store premultiplied colour so the data is self-consistent
        vals = [[c * px[-1] for c in px[:-1]] + [px[-1]] for px in vals]
    tags = {}
    if pg.photometric == 3:
        n = 1 << bps
        vals = [[(x + y) % n] for y in range(h) for x in range(w)]
        cmap = [((i * 65535) // max(1, n - 1)) for i in range(n)] + [((n - 1 - i) * 65535) // max(1, n - 1) for i in range(n)] + [(i * 40503) % 65536 for i in range(n)]
        tags[320] = (3, cmap)
    if pg.photometric == 0:
        vals = [[1 - v[0]] + v[1:] for v in vals]  # stored as ink so the image looks the same
    fmt = 'index' if pg.photometric == 3 else pg.fmt
    def rows_for(y0, y1, x0, x1, plane=None):
        data = b''
        for y in range(y0, y1):
            if pg.photometric == 6 and pg.ycbcr_sub == (2, 2):
                continue
            row = []
            for x in range(x0, x1):
                px = vals[min(y, h - 1) * w + min(x, w - 1)]
                row += [px[plane]] if plane is not None else px
            r = pack_row(row, bps, fmt, endian)
            k = 1 if plane is not None else spp
            if pg.predictor == 2:
                r = predict2(r, bps, k, endian)
            elif pg.predictor == 3:
                r = predict3(r, bps, k, endian)
            data += r
        return data
    def ycc_sub(y0, y1):
        out = bytearray()
        for by in range(y0, y1, 2):
            for bx in range(0, w + (w % 2), 2):
                ys, cbs, crs = [], [], []
                for dy in (0, 1):
                    for dx in (0, 1):
                        r, g, b = vals[min(by + dy, h - 1) * w + min(bx + dx, w - 1)][:3]
                        ys.append(round(255 * (0.299 * r + 0.587 * g + 0.114 * b)))
                        cbs.append(128 + 255 * (-0.168736 * r - 0.331264 * g + 0.5 * b))
                        crs.append(128 + 255 * (0.5 * r - 0.418688 * g - 0.081312 * b))
                out += bytes(ys) + bytes([max(0, min(255, round(sum(cbs) / 4))), max(0, min(255, round(sum(crs) / 4)))])
        return bytes(out)
    def compress(data, y0, y1, x0, x1, plane=None):
        c = pg.compression
        if c == 1:
            return data
        if c == 32773:
            return packbits(data)
        if c == 5:
            return lzw(data)
        if c in (8, 32946):
            return zlib.compress(data, 6)
        if c == 7:
            cw, ch = x1 - x0, y1 - y0
            if pg.photometric == 1:
                raw = bytes(int(round(vals[min(y, h - 1) * w + min(x, w - 1)][0] * 255)) for y in range(y0, y1) for x in range(x0, x1))
                return jpeg_strip(raw, cw, ch, 'gray', ['cs=gray'])
            raw = bytes(int(round(v * 255)) for y in range(y0, y1) for x in range(x0, x1) for v in vals[min(y, h - 1) * w + min(x, w - 1)][:3])
            if pg.photometric == 6:
                return jpeg_strip(raw, cw, ch, 'rgb', ['cs=ycc', 'samp=22,11,11'])
            return jpeg_strip(raw, cw, ch, 'rgb', ['cs=rgb', 'samp=11,11,11', 'adobe=1'])
        raise ValueError(c)
    chunks = []
    planes = range(spp) if pg.planar == 2 else [None]
    if pg.tile:
        tw, th = pg.tile
        for plane in planes:
            for ty in range(0, h, th):
                for tx in range(0, w, tw):
                    chunks.append(compress(rows_for(ty, ty + th, tx, tx + tw, plane), ty, ty + th, tx, tx + tw, plane))
    else:
        rps = pg.rows or h
        for plane in planes:
            for y in range(0, h, rps):
                y1 = min(h, y + rps)
                raw = ycc_sub(y, y1) if (pg.photometric == 6 and pg.ycbcr_sub == (2, 2) and pg.compression != 7) else rows_for(y, y1, 0, w, plane)
                chunks.append(compress(raw, y, y1, 0, w, plane))
    return chunks, tags


def ifd_tags(pg, extra):
    sf = {'uint': 1, 'int': 2, 'float': 3}[pg.fmt]
    t = {256: (4, [pg.w]), 257: (4, [pg.h]), 258: (3, [pg.bps] * pg.spp), 259: (3, [pg.compression]), 262: (3, [pg.photometric]),
         277: (3, [pg.spp]), 284: (3, [pg.planar]), 282: (5, [72, 1]), 283: (5, [72, 1]), 296: (3, [2])}
    if pg.tile:
        t[322], t[323] = (4, [pg.tile[0]]), (4, [pg.tile[1]])
    else:
        t[278] = (4, [pg.rows or pg.h])
    if sf != 1:
        t[339] = (3, [sf] * pg.spp)
    if pg.predictor != 1:
        t[317] = (3, [pg.predictor])
    if pg.extra is not None:
        t[338] = (3, [pg.extra])
    if pg.orientation:
        t[274] = (3, [pg.orientation])
    if pg.photometric == 6:
        t[530] = (3, list(pg.ycbcr_sub or (2, 2)))
        t[532] = (5, [0, 1, 255, 1, 128, 1, 255, 1, 128, 1, 255, 1])
    if pg.subfile is not None:
        t[254] = (4, [pg.subfile])
    if pg.fill_order:
        t[266] = (3, [pg.fill_order])
    if pg.ink:
        t[332] = (3, [pg.ink])
    t.update(extra)
    return t


def write(pages, endian='<', big=False):
    """Serialise pages; returns bytes. Layout: header, then per page chunk data, values, IFD."""
    out = bytearray(b'II' if endian == '<' else b'MM')
    if big:
        out += struct.pack(endian + 'HHHQ', 43, 8, 0, 0)
    else:
        out += struct.pack(endian + 'HI', 42, 0)
    link = 8 if big else 4  # where the first IFD offset is stored
    off_t, off_fmt, cnt_fmt, ent = (16, 'Q', 'Q', 20) if big else (4, 'I', 'H', 12)
    inline = 8 if big else 4
    for pg in pages:
        chunks, extra = encode_chunks(pg, endian)
        offsets = []
        for c in chunks:
            offsets.append(len(out))
            out += c
            if len(out) % 2:
                out += b'\0'
        tags = ifd_tags(pg, extra)
        otag, btag = (324, 325) if pg.tile else (273, 279)
        tags[otag] = (off_t, offsets)
        tags[btag] = (off_t, [len(c) for c in chunks])
        if not big:
            tags[otag] = (4, offsets)
            tags[btag] = (4, [len(c) for c in chunks])
        entries = []
        for tag in sorted(tags):
            typ, values = tags[tag]
            if typ == 2:
                payload = values
                count = len(values)
            elif typ == 5:
                payload = struct.pack(endian + 'I' * len(values), *values)
                count = len(values) // 2
            else:
                payload = struct.pack(endian + TYPES[typ] * len(values), *values)
                count = len(values)
            if len(payload) > inline:
                pos = len(out)
                out += payload
                if len(out) % 2:
                    out += b'\0'
                field = struct.pack(endian + off_fmt, pos)
            else:
                field = payload + bytes(inline - len(payload))
            entries.append(struct.pack(endian + 'HH' + ('Q' if big else 'I'), tag, typ, count) + field)
        ifd = len(out)
        struct.pack_into(endian + off_fmt, out, link, ifd)
        out += struct.pack(endian + cnt_fmt, len(entries)) + b''.join(entries)
        link = len(out)
        out += bytes(8 if big else 4)
    return bytes(out)


def parse(b):
    """Generic IFD walker: list of IFDs, each {'offset', 'count_pos', 'entries': [...], 'next_pos'}."""
    if len(b) < 8 or b[:2] not in (b'II', b'MM'):
        return None
    e = '<' if b[:2] == b'II' else '>'
    magic = struct.unpack(e + 'H', b[2:4])[0]
    big = magic == 43
    if big:
        if len(b) < 16:
            return None
        off = struct.unpack(e + 'Q', b[8:16])[0]
    else:
        off = struct.unpack(e + 'I', b[4:8])[0]
    ifds, seen = [], set()
    while off and off not in seen and off + 2 <= len(b) and len(ifds) < 8:
        seen.add(off)
        n = struct.unpack(e + ('Q' if big else 'H'), b[off:off + (8 if big else 2)])[0]
        p = off + (8 if big else 2)
        es = 20 if big else 12
        if p + n * es > len(b):
            break
        entries = []
        for k in range(n):
            q = p + k * es
            tag, typ = struct.unpack(e + 'HH', b[q:q + 4])
            cnt = struct.unpack(e + ('Q' if big else 'I'), b[q + 4:q + (12 if big else 8)])[0]
            vpos = q + (12 if big else 8)
            entries.append({'pos': q, 'tag': tag, 'type': typ, 'count': cnt, 'value_pos': vpos})
        nxt = p + n * es
        ifds.append({'offset': off, 'entries': entries, 'next_pos': nxt})
        if nxt + (8 if big else 4) > len(b):
            break
        off = struct.unpack(e + ('Q' if big else 'I'), b[nxt:nxt + (8 if big else 4)])[0]
    return {'endian': e, 'big': big, 'ifds': ifds}


def values(b, info, ent):
    """Decode an entry's integer values (SHORT/LONG/LONG8) for strip arrays; [] when not applicable."""
    e, big = info['endian'], info['big']
    code = {3: 'H', 4: 'I', 16: 'Q'}.get(ent['type'])
    if not code or ent['count'] > 4096:
        return [], None
    size = SIZE[ent['type']] * ent['count']
    pos = ent['value_pos'] if size <= (8 if big else 4) else struct.unpack(e + ('Q' if big else 'I'), b[ent['value_pos']:ent['value_pos'] + (8 if big else 4)])[0]
    if pos + size > len(b):
        return [], None
    return list(struct.unpack(e + code * ent['count'], b[pos:pos + size])), pos
