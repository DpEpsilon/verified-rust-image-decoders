"""Seeded synthetic PNG / APNG generator with chunk-level provenance (no third-party bytes).

A file is a list of chunks [(type, data)]; `serialize` adds lengths/CRCs with optional
per-chunk overrides. Seeds cover every colour type / bit depth, interlace, tRNS, PLTE
edge cases, colour-metadata chunks, ancillary order, IDAT splitting and APNG frames.
Mutations edit fields (CRC recomputed unless the rule is about CRCs), chunk lengths,
chunk-type bits, chunk order, zlib streams, filter bytes, APNG control chunks,
truncation, trailing data and random bits.
"""
import random
import struct
import zlib

SIG = b'\x89PNG\r\n\x1a\n'
DEPTHS = {0: (1, 2, 4, 8, 16), 2: (8, 16), 3: (1, 2, 4, 8), 4: (8, 16), 6: (8, 16)}
CHANNELS = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}
ADAM7 = ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2))


def crc(t, d):
    return zlib.crc32(t + d) & 0xffffffff


def serialize(chunks, crc_override=None, length_override=None, signature=SIG):
    out = bytearray(signature)
    for i, (t, d) in enumerate(chunks):
        n = (length_override or {}).get(i, len(d))
        c = (crc_override or {}).get(i, crc(t, d))
        out += struct.pack('>I', n & 0xffffffff) + t + d + struct.pack('>I', c & 0xffffffff)
    return bytes(out)


def pack_row(row, depth):
    if depth == 16:
        return b''.join(struct.pack('>H', s) for px in row for s in px)
    if depth == 8:
        return bytes(s for px in row for s in px)
    out, acc, n = bytearray(), 0, 0
    for px in row:
        for s in px:
            acc = (acc << depth) | s
            n += depth
            if n == 8:
                out.append(acc)
                acc = n = 0
    if n:
        out.append(acc << (8 - n))
    return bytes(out)


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    return a if pa <= pb and pa <= pc else (b if pb <= pc else c)


def filter_rows(rows, bpp, ftypes):
    out, prev = bytearray(), bytes(len(rows[0])) if rows else b''
    for y, row in enumerate(rows):
        f = ftypes[y % len(ftypes)]
        enc = bytearray()
        for i, x in enumerate(row):
            a = row[i - bpp] if i >= bpp else 0
            b = prev[i]
            c = prev[i - bpp] if i >= bpp else 0
            pred = [0, a, b, (a + b) // 2, paeth(a, b, c)][f] if f < 5 else 0
            enc.append((x - pred) & 255)
        out += bytes([f]) + enc
        prev = row
    return bytes(out)


def raw_image(pixels, w, h, depth, ctype, interlace, ftypes=(0, 1, 2, 3, 4)):
    bpp = max(1, CHANNELS[ctype] * depth // 8)
    if not interlace:
        return filter_rows([pack_row(pixels[y * w:(y + 1) * w], depth) for y in range(h)], bpp, ftypes)
    out = b''
    for x0, y0, dx, dy in ADAM7:
        rows = [pack_row([pixels[y * w + x] for x in range(x0, w, dx)], depth) for y in range(y0, h, dy)]
        rows = [r for r in rows if r]
        if rows:
            out += filter_rows(rows, bpp, ftypes)
    return out


def make_pixels(rng, w, h, depth, ctype, palette_len=None):
    top = (1 << depth) - 1
    ch = CHANNELS[ctype]
    if ctype == 3:
        top = min(top, (palette_len or 1) - 1)
    pix = []
    for y in range(h):
        for x in range(w):
            if rng.random() < 0.5:
                pix.append(tuple(((x * 37 + y * 11 + c * 101) * (top // 7 + 1)) % (top + 1) for c in range(ch)))
            else:
                pix.append(tuple(rng.randrange(top + 1) for _ in range(ch)))
    return pix


def ihdr(w, h, depth, ctype, interlace=0):
    return struct.pack('>IIBBBBB', w, h, depth, ctype, 0, 0, interlace)


def fctl(seq, w, h, x, y, dn, dd, dispose, blend):
    return struct.pack('>IIIIIHHBB', seq, w, h, x, y, dn, dd, dispose, blend)


FAKE_ICC = struct.pack('>I', 132) + b'lnny' + b'\x02\x10\x00\x00' + b'mntrRGB XYZ ' + bytes(12) + b'acsp' + bytes(92) + struct.pack('>I', 0)


def anc(name, rng):
    return {'gAMA': struct.pack('>I', 45455), 'gAMA-1.0': struct.pack('>I', 100000),
            'sRGB': b'\x00', 'cHRM': struct.pack('>8I', 31270, 32900, 64000, 33000, 30000, 60000, 15000, 6000),
            'iCCP': b'synthetic\x00\x00' + zlib.compress(FAKE_ICC), 'tEXt': b'Comment\x00synthetic fixture',
            'zTXt': b'Comment\x00\x00' + zlib.compress(b'synthetic'), 'iTXt': b'Comment\x00\x00\x00\x00\x00synthetic',
            'pHYs': struct.pack('>IIB', 2835, 2835, 1), 'tIME': struct.pack('>HBBBBB', 2026, 1, 2, 3, 4, 5),
            'leNy': b'private-ancillary', 'eXIf': b'MM\x00*\x00\x00\x00\x08\x00\x00'}[name]


def build_static(rng, w, h, depth, ctype, interlace=0, plte=None, trns=None, extra=(), split=1, ftypes=(0, 1, 2, 3, 4),
                 level=6, pixels=None):
    if ctype == 3 and plte is None:
        plte = [tuple(rng.randrange(256) for _ in range(3)) for _ in range(min(1 << depth, 1 + rng.randrange(1 << depth)))]
    pix = pixels if pixels is not None else make_pixels(rng, w, h, depth, ctype, len(plte) if plte else None)
    data = zlib.compress(raw_image(pix, w, h, depth, ctype, interlace, ftypes), level)
    chunks = [(b'IHDR', ihdr(w, h, depth, ctype, interlace))]
    pre = [(t.encode()[:4], anc(t, rng)) for pos, t in extra if pos == 'pre']
    chunks += pre
    if plte is not None:
        chunks.append((b'PLTE', b''.join(bytes(c) for c in plte)))
    chunks += [(t.encode()[:4], anc(t, rng)) for pos, t in extra if pos == 'post-plte']
    if trns is not None:
        chunks.append((b'tRNS', trns))
    step = max(1, -(-len(data) // split))
    chunks += [(b'IDAT', data[i:i + step]) for i in range(0, len(data), step)]
    chunks += [(t.encode()[:4], anc(t, rng)) for pos, t in extra if pos == 'post-idat']
    chunks.append((b'IEND', b''))
    return chunks, {'w': w, 'h': h, 'depth': depth, 'ctype': ctype, 'interlace': interlace, 'pixels': pix, 'plte': plte}


def build_apng(rng, w, h, depth=8, ctype=6, frames=3, default_in_anim=True, plays=0, ops=None, delays=None):
    plte = [tuple(rng.randrange(256) for _ in range(3)) for _ in range(16)] if ctype == 3 else None
    base, info = build_static(rng, w, h, depth, ctype, plte=plte, trns=bytes(range(0, 256, 17))[:16] if ctype == 3 else None)
    idat = [i for i, (t, _) in enumerate(base) if t == b'IDAT'][0]
    chunks = base[:idat]
    seq = 0
    n_anim = frames if default_in_anim else frames
    chunks.append((b'acTL', struct.pack('>II', n_anim, plays)))
    ops = ops or [(k % 3, k % 2) for k in range(frames)]
    delays = delays or [(1 + k, 10) for k in range(frames)]
    if default_in_anim:
        chunks.append((b'fcTL', fctl(seq, w, h, 0, 0, delays[0][0], delays[0][1], ops[0][0], ops[0][1])))
        seq += 1
    chunks += [c for c in base if c[0] == b'IDAT']
    for k in range(1 if default_in_anim else 0, frames):
        fw, fh = max(1, w // 2 + k), max(1, h // 2)
        fw = min(fw, w)
        x, y = (w - fw) * k // max(1, frames), (h - fh) // 2
        chunks.append((b'fcTL', fctl(seq, fw, fh, x, y, delays[k][0], delays[k][1], ops[k][0], ops[k][1])))
        seq += 1
        pix = make_pixels(rng, fw, fh, depth, ctype, 16 if ctype == 3 else None)
        data = zlib.compress(raw_image(pix, fw, fh, depth, ctype, 0))
        chunks.append((b'fdAT', struct.pack('>I', seq) + data))
        seq += 1
    chunks.append((b'IEND', b''))
    return chunks, dict(info, apng=True, frames=frames)


def seeds(rng):
    out = []
    for ctype, depths in DEPTHS.items():
        for depth in depths:
            for interlace in (0, 1):
                w, h = rng.choice([(1, 1), (3, 2), (9, 7), (17, 5), (5, 17), (33, 9), (16, 16)])
                out.append((f'ct{ctype}-d{depth}-i{interlace}', build_static(rng, w, h, depth, ctype, interlace)))
    out.append(('trns-gray8', build_static(rng, 8, 8, 8, 0, trns=struct.pack('>H', 0))))
    out.append(('trns-gray16', build_static(rng, 8, 8, 16, 0, trns=struct.pack('>H', 0))))
    out.append(('trns-gray2', build_static(rng, 8, 8, 2, 0, trns=struct.pack('>H', 1))))
    out.append(('trns-rgb8', build_static(rng, 8, 8, 8, 2, trns=struct.pack('>HHH', 0, 0, 0))))
    out.append(('trns-rgb16', build_static(rng, 8, 8, 16, 2, trns=struct.pack('>HHH', 0, 0, 0))))
    out.append(('trns-palette-partial', build_static(rng, 8, 8, 4, 3, plte=[(k * 16, 255 - k * 16, k) for k in range(16)], trns=bytes([0, 128, 255, 64]))))
    out.append(('trns-palette-full', build_static(rng, 8, 8, 8, 3, plte=[(k, k, k) for k in range(256)], trns=bytes(range(256)))))
    out.append(('plte-256', build_static(rng, 16, 16, 8, 3, plte=[(k, 255 - k, k // 2) for k in range(256)])))
    out.append(('plte-1', build_static(rng, 5, 5, 1, 3, plte=[(200, 10, 10)])))
    out.append(('plte-suggested-rgb', build_static(rng, 6, 6, 8, 2, plte=[(1, 2, 3), (4, 5, 6)])))
    out.append(('plte-suggested-rgba', build_static(rng, 6, 6, 8, 6, plte=[(1, 2, 3)])))
    for t in ('gAMA', 'gAMA-1.0', 'sRGB', 'iCCP', 'cHRM'):
        out.append((f'color-{t}', build_static(rng, 7, 7, 8, 2, extra=[('pre', t)])))
    out.append(('color-all', build_static(rng, 7, 7, 16, 6, extra=[('pre', 'gAMA'), ('pre', 'cHRM'), ('pre', 'iCCP'), ('pre', 'sRGB')])))
    out.append(('anc-text', build_static(rng, 7, 7, 8, 0, extra=[('pre', 'tEXt'), ('post-idat', 'zTXt'), ('post-idat', 'iTXt')])))
    out.append(('anc-misc', build_static(rng, 7, 7, 8, 4, extra=[('pre', 'pHYs'), ('post-idat', 'tIME'), ('pre', 'leNy'), ('pre', 'eXIf')])))
    out.append(('anc-order-gama-after-plte', build_static(rng, 7, 7, 4, 3, extra=[('post-plte', 'gAMA')])))
    out.append(('anc-order-gama-after-idat', build_static(rng, 7, 7, 8, 2, extra=[('post-idat', 'gAMA')])))
    out.append(('idat-split-1byte', build_static(rng, 9, 9, 8, 6, split=10_000)))
    out.append(('idat-split-7', build_static(rng, 20, 20, 8, 2, split=7)))
    out.append(('zlib-stored', build_static(rng, 13, 11, 8, 6, level=0)))
    out.append(('big-rgb', build_static(rng, 300, 200, 8, 2, ftypes=(4,))))
    out.append(('big-gray16-interlaced', build_static(rng, 150, 130, 16, 0, interlace=1)))
    out.append(('filter-none', build_static(rng, 12, 12, 8, 6, ftypes=(0,))))
    out.append(('filter-paeth', build_static(rng, 12, 12, 16, 4, ftypes=(4,))))
    for d in range(3):
        for b in range(2):
            out.append((f'apng-dispose{d}-blend{b}', build_apng(rng, 12, 10, ops=[(d, b)] * 3)))
    out.append(('apng-default-hidden', build_apng(rng, 12, 10, default_in_anim=False)))
    out.append(('apng-one-frame', build_apng(rng, 8, 8, frames=1)))
    out.append(('apng-delay-den0', build_apng(rng, 8, 8, delays=[(1, 0), (0, 0), (65535, 1)])))
    out.append(('apng-palette', build_apng(rng, 10, 10, ctype=3)))
    out.append(('apng-rgba16', build_apng(rng, 10, 10, depth=16)))
    out.append(('apng-plays3', build_apng(rng, 10, 10, plays=3, frames=4)))
    return out


def set_field(chunks, idx, off, size, value):
    t, d = chunks[idx]
    d = d[:off] + value.to_bytes(size, 'big') + d[off + size:]
    return chunks[:idx] + [(t, d)] + chunks[idx + 1:]


IHDR_FIELDS = [('width', 0, 4), ('height', 4, 4), ('bit_depth', 8, 1), ('color_type', 9, 1), ('compression', 10, 1),
               ('filter_method', 11, 1), ('interlace', 12, 1)]
FCTL_FIELDS = [('seq', 0, 4), ('width', 4, 4), ('height', 8, 4), ('x_offset', 12, 4), ('y_offset', 16, 4), ('delay_num', 20, 2),
               ('delay_den', 22, 2), ('dispose_op', 24, 1), ('blend_op', 25, 1)]


def find(chunks, t, n=0):
    hits = [i for i, c in enumerate(chunks) if c[0] == t]
    return hits[n] if len(hits) > n else None


def recompress(info, rng, mutate):
    """Rebuild the IDAT payload from the seed's pixels with a raw-stream mutation."""
    raw = raw_image(info['pixels'], info['w'], info['h'], info['depth'], info['ctype'], info['interlace'])
    return mutate(raw)


def mutations(chunks, info, rng):
    data = serialize(chunks)
    n = len(data)
    w, h = info['w'], info['h']

    def S(c, **kw):
        return serialize(c, **kw)

    ih = find(chunks, b'IHDR')
    for name, off, size in IHDR_FIELDS:
        old = int.from_bytes(chunks[ih][1][off:off + size], 'big')
        vals = {0, 1, 2, 3, 4, 5, 6, 7, 8, 16, 32, 255} if size == 1 else {0, 1, old - 1, old + 1, 2 * old, 1 << 16, (1 << 24) + 1, 0x7fffffff, 0x80000000, 0xffffffff}
        for v in sorted(x for x in vals if 0 <= x < 1 << (8 * size) and x != old):
            yield S(set_field(chunks, ih, off, size, v)), {'rule': 'ihdr:' + name, 'field': 'IHDR.' + name, 'value': v, 'old': old}
    for i, (t, d) in enumerate(chunks):
        tn = t.decode('latin1')
        yield S(chunks, crc_override={i: crc(t, d) ^ 1}), {'rule': f'crc:{"critical" if t[0] < 97 else "ancillary"}', 'field': tn + '.crc', 'value': i}
        for v in sorted({0, len(d) - 1, len(d) + 1, len(d) + 4, 0x7fffffff, 0x80000000, 0xffffffff} - {len(d)}):
            if v >= 0:
                yield S(chunks, length_override={i: v}), {'rule': 'length:' + tn, 'field': tn + '.length', 'value': v, 'old': len(d)}
        for bit, label in [(0, 'ancillary'), (2, 'reserved'), (3, 'safe-to-copy'), (1, 'private')]:
            t2 = bytearray(t)
            t2[bit] ^= 0x20
            c2 = chunks[:i] + [(bytes(t2), d)] + chunks[i + 1:]
            yield S(c2), {'rule': f'type-bit:{label}', 'field': tn, 'value': bytes(t2).decode('latin1')}
    # chunk order / presence
    order = []
    idats = [i for i, c in enumerate(chunks) if c[0] == b'IDAT']
    pl = find(chunks, b'PLTE')
    order.append(('ihdr-not-first', [chunks[1], chunks[0]] + chunks[2:]))
    order.append(('duplicate-ihdr', chunks[:1] + chunks[:1] + chunks[1:]))
    order.append(('missing-iend', chunks[:-1]))
    order.append(('iend-with-data', chunks[:-1] + [(b'IEND', b'x')]))
    order.append(('chunk-after-iend', chunks + [(b'tEXt', b'k\x00v')]))
    order.append(('idat-after-iend', chunks + [chunks[idats[0]]]))
    order.append(('missing-idat', [c for c in chunks if c[0] != b'IDAT']))
    order.append(('empty-idat-inserted', chunks[:idats[0]] + [(b'IDAT', b'')] + chunks[idats[0]:]))
    order.append(('empty-idat-only', [c for c in chunks if c[0] != b'IDAT'][:-1] + [(b'IDAT', b''), chunks[-1]]))
    order.append(('ancillary-between-idat', chunks[:idats[0] + 1] + [(b'tEXt', b'k\x00v')] + chunks[idats[0] + 1:]))
    order.append(('unknown-critical', chunks[:1] + [(b'CRIT', b'abc')] + chunks[1:]))
    order.append(('unknown-ancillary', chunks[:1] + [(b'abCd', b'abc')] + chunks[1:]))
    order.append(('ancillary-before-ihdr', [(b'tEXt', b'k\x00v')] + chunks))
    order.append(('gama-after-idat', chunks[:-1] + [(b'gAMA', struct.pack('>I', 45455)), chunks[-1]]))
    order.append(('double-gama', chunks[:1] + [(b'gAMA', struct.pack('>I', 45455)), (b'gAMA', struct.pack('>I', 100000))] + chunks[1:]))
    order.append(('srgb-and-iccp', chunks[:1] + [(b'sRGB', b'\x00'), (b'iCCP', anc('iCCP', rng))] + chunks[1:]))
    order.append(('iccp-bad-method', chunks[:1] + [(b'iCCP', b'p\x00\x01' + zlib.compress(FAKE_ICC))] + chunks[1:]))
    order.append(('iccp-bad-zlib', chunks[:1] + [(b'iCCP', b'p\x00\x00garbage')] + chunks[1:]))
    order.append(('iccp-no-name', chunks[:1] + [(b'iCCP', b'\x00\x00' + zlib.compress(FAKE_ICC))] + chunks[1:]))
    order.append(('gama-zero', chunks[:1] + [(b'gAMA', bytes(4))] + chunks[1:]))
    order.append(('srgb-intent-9', chunks[:1] + [(b'sRGB', b'\x09')] + chunks[1:]))
    order.append(('text-no-null', chunks[:1] + [(b'tEXt', b'novalue')] + chunks[1:]))
    order.append(('ztxt-bad-zlib', chunks[:1] + [(b'zTXt', b'k\x00\x00junk')] + chunks[1:]))
    order.append(('phys-short', chunks[:1] + [(b'pHYs', b'\x00\x01')] + chunks[1:]))
    if info['ctype'] == 3:
        pd = chunks[pl][1]
        order.append(('missing-plte', [c for c in chunks if c[0] != b'PLTE']))
        order.append(('plte-after-idat', [c for c in chunks if c[0] != b'PLTE'][:-1] + [chunks[pl], chunks[-1]]))
        order.append(('plte-short', chunks[:pl] + [(b'PLTE', pd[:3])] + chunks[pl + 1:]))
        order.append(('plte-not-multiple-of-3', chunks[:pl] + [(b'PLTE', pd + b'\x01')] + chunks[pl + 1:]))
        order.append(('plte-257', chunks[:pl] + [(b'PLTE', bytes(771))] + chunks[pl + 1:]))
        order.append(('plte-empty', chunks[:pl] + [(b'PLTE', b'')] + chunks[pl + 1:]))
        order.append(('trns-before-plte', chunks[:pl] + [(b'tRNS', b'\x00\x80')] + chunks[pl:]))
        order.append(('trns-longer-than-plte', chunks[:pl + 1] + [(b'tRNS', bytes(len(pd) // 3 + 5))] + chunks[pl + 1:]))
        order.append(('duplicate-plte', chunks[:pl + 1] + [chunks[pl]] + chunks[pl + 1:]))
    else:
        order.append(('plte-in-' + ('gray' if info['ctype'] in (0, 4) else 'truecolor'), chunks[:1] + [(b'PLTE', bytes(range(12)))] + chunks[1:]))
        order.append(('trns-wrong-length', chunks[:1] + [(b'tRNS', b'\x00')] + chunks[1:]))
        if info['ctype'] in (4, 6):
            order.append(('trns-with-alpha', chunks[:1] + [(b'tRNS', bytes(6))] + chunks[1:]))
    for name, c in order:
        yield S(c), {'rule': 'order:' + name.split('-in-')[0], 'field': 'chunks', 'value': name}
    # zlib / filter / data volume, re-encoded from the seed pixels
    raw = raw_image(info['pixels'], w, h, info['depth'], info['ctype'], info['interlace'])
    comp = zlib.compress(raw)
    first = idats[0]
    rest = [c for c in chunks if c[0] != b'IDAT']

    def with_idat(payload):
        return rest[:first] + [(b'IDAT', payload)] + rest[first:]

    rowlen = len(raw) // max(1, h) if not info['interlace'] else None
    zl = [('bad-zlib-header', b'\x78\x9d' + comp[2:]), ('zlib-fdict', bytes([0x78, 0x20 | (31 - ((0x78 * 256 + 0x20) % 31))]) + comp[2:]),
          ('bad-cm', b'\x79' + comp[1:]), ('bad-adler32', comp[:-4] + bytes(b ^ 0xff for b in comp[-4:])),
          ('missing-adler32', comp[:-4]), ('truncated-deflate', comp[:max(2, len(comp) // 2)]),
          ('extra-after-zlib', comp + b'\x00\x00junk'), ('raw-deflate', zlib.compress(raw)[2:-4]),
          ('too-little-data', zlib.compress(raw[:max(0, len(raw) - max(1, len(raw) // 4))])),
          ('one-byte-short', zlib.compress(raw[:-1])), ('one-byte-long', zlib.compress(raw + b'\x00')),
          ('extra-row', zlib.compress(raw + raw[:rowlen] if rowlen else raw + bytes(8))),
          ('two-zlib-streams', comp + comp), ('level0', zlib.compress(raw, 0)), ('level9', zlib.compress(raw, 9))]
    for name, payload in zl:
        yield S(with_idat(payload)), {'rule': 'zlib:' + name, 'field': 'IDAT', 'value': name}
    for f in (5, 6, 128, 255):
        for where in ('first', 'last'):
            r = bytearray(raw)
            pos = 0 if where == 'first' else (len(raw) - rowlen if rowlen else len(raw) - 2)
            if 0 <= pos < len(r):
                r[pos] = f
                yield S(with_idat(zlib.compress(bytes(r)))), {'rule': 'filter-type', 'field': where + '-row', 'value': f}
    if info['depth'] < 8 and not info['interlace']:  # nonzero padding bits at the end of each row
        r = bytearray(raw)
        r[rowlen - 1] |= 1
        yield S(with_idat(zlib.compress(bytes(r)))), {'rule': 'row-padding-bits', 'field': 'raw', 'value': 1}
    # APNG control chunks
    if info.get('apng'):
        ac = find(chunks, b'acTL')
        for v in (0, info['frames'] - 1, info['frames'] + 1, 0x7fffffff, 0xffffffff):
            if v != info['frames'] and v >= 0:
                yield S(set_field(chunks, ac, 0, 4, v)), {'rule': 'actl:num_frames', 'field': 'acTL.num_frames', 'value': v}
        yield S(set_field(chunks, ac, 4, 4, 0xffffffff)), {'rule': 'actl:num_plays', 'field': 'acTL.num_plays', 'value': 0xffffffff}
        yield S([c for c in chunks if c[0] != b'acTL']), {'rule': 'apng:missing-actl', 'field': 'acTL', 'value': 'removed'}
        yield S(chunks[:-1] + [chunks[ac], chunks[-1]]), {'rule': 'apng:actl-after-idat', 'field': 'acTL', 'value': 'moved'}
        for k in range(2):
            fi = find(chunks, b'fcTL', k)
            if fi is None:
                continue
            for name, off, size in FCTL_FIELDS:
                old = int.from_bytes(chunks[fi][1][off:off + size], 'big')
                vals = {0, 1, 2, 3, 255} if size == 1 else ({0, 1, old + 1, 65535} if size == 2 else {0, 1, old + 1, w, h, w + 1, 0x7fffffff, 0xffffffff})
                for v in sorted(x for x in vals if x != old and x < 1 << (8 * size)):
                    yield S(set_field(chunks, fi, off, size, v)), {'rule': f'fctl:{name}', 'field': f'fcTL[{k}].{name}', 'value': v, 'old': old}
        fd = find(chunks, b'fdAT')
        if fd is not None:
            yield S(set_field(chunks, fd, 0, 4, 0)), {'rule': 'fdat:seq', 'field': 'fdAT.seq', 'value': 0}
            yield S(chunks[:fd - 1] + [chunks[fd], chunks[fd - 1]] + chunks[fd + 1:]), {'rule': 'apng:fdat-before-fctl', 'field': 'fdAT', 'value': 'swapped'}
            yield S(chunks[:fd - 1] + chunks[fd + 1:]), {'rule': 'apng:drop-frame', 'field': 'fcTL+fdAT', 'value': 'removed'}
            t, d = chunks[fd]
            yield S(chunks[:fd] + [(t, d[:4] + d[4:4 + max(1, (len(d) - 4) // 2)])] + chunks[fd + 1:]), {'rule': 'apng:fdat-truncated-zlib', 'field': 'fdAT', 'value': 'half'}
            yield S(chunks[:fd] + [(t, d[:4])] + chunks[fd + 1:]), {'rule': 'apng:fdat-empty', 'field': 'fdAT', 'value': 'empty'}
    else:
        frames = chunks[:first] + [(b'acTL', struct.pack('>II', 1, 0)), (b'fcTL', fctl(0, w, h, 0, 0, 1, 10, 0, 0))] + chunks[first:]
        yield S(frames), {'rule': 'apng:add-single-frame', 'field': 'acTL+fcTL', 'value': 'added'}
        yield S(chunks[:first] + [(b'acTL', struct.pack('>II', 2, 0))] + chunks[first:]), {'rule': 'apng:actl-without-fctl', 'field': 'acTL', 'value': 'added'}
    # signature, truncation, trailing data, random bits
    for i in range(8):
        sig = bytearray(SIG)
        sig[i] ^= 0x01
        yield S(chunks, signature=bytes(sig)), {'rule': 'signature', 'field': f'sig[{i}]', 'value': sig[i]}
    yield data.replace(b'\r\n', b'\n', 1), {'rule': 'signature', 'field': 'sig', 'value': 'crlf-to-lf'}
    bounds, off = [8], 8
    for t, d in chunks:
        off += 12 + len(d)
        bounds.append(off)
    for p in sorted({x for b in bounds for x in (b - 4, b - 1, b, b + 1, b + 8)} | {n // 2}):
        if 0 < p < n:
            yield data[:p], {'rule': 'truncation', 'field': 'length', 'value': p}
    for tail, label in [(b'\x00', 'zero-1'), (bytes(64), 'zero-64'), (b'garbage', 'ascii'), (SIG, 'second-signature'), (serialize([(b'IEND', b'')], signature=b''), 'second-iend')]:
        yield data + tail, {'rule': 'trailing-data', 'field': 'tail', 'value': label}
    for _ in range(24):
        p = rng.randrange(8, n)
        a = bytearray(data)
        a[p] ^= 1 << rng.randrange(8)
        yield bytes(a), {'rule': 'byte-flip', 'field': 'offset', 'value': p}


def interleave(candidates):
    by_rule = {}
    for b, m in candidates:
        by_rule.setdefault(m['rule'], []).append((b, m))
    queues = [iter(v) for v in by_rule.values()]
    while queues:
        for q in list(queues):
            try:
                yield next(q)
            except StopIteration:
                queues.remove(q)


def generate(limit, seed):
    rng = random.Random(seed)
    base, series = [], []
    for name, (chunks, info) in seeds(rng):
        kind = name.split('-')[0]
        base.append((serialize(chunks), {'source': 'synthetic', 'name': name, 'kind': kind, 'rule': 'seed:' + kind}))
        series.append((name, chunks, info))
    order = list(range(len(series)))
    rng.shuffle(order)
    out = base[:limit]
    iters = {i: interleave(mutations(series[i][1], series[i][2], random.Random(f'{seed}:{series[i][0]}'))) for i in order}
    while len(out) < limit and iters:
        for i in list(iters):
            try:
                b, m = next(iters[i])
            except StopIteration:
                del iters[i]
                continue
            m.update(source='mutation', seed=series[i][0], seed_source='synthetic', kind=series[i][0].split('-')[0])
            out.append((b, m))
            if len(out) >= limit:
                break
    return out, len(base), len(series)
