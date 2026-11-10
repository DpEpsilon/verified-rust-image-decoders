"""Seeded synthetic GIF generator with field-level provenance (no third-party bytes).

`build(spec)` serialises a structured description and returns (bytes, fields, bounds):
fields maps a name to (offset, size) for boundary-value mutation; bounds lists block
start offsets for truncation. `lzw_encode` is a reference GIF LZW encoder with knobs
for the edge cases (no leading clear, no EOI, deferred clear, extra clears, invalid codes).
"""
import random
import struct

INTERLACE_PASSES = ((0, 8), (4, 8), (2, 4), (1, 2))


class Bits:
    def __init__(self):
        self.out, self.acc, self.n = bytearray(), 0, 0

    def put(self, code, size):
        self.acc |= code << self.n
        self.n += size
        while self.n >= 8:
            self.out.append(self.acc & 255)
            self.acc >>= 8
            self.n -= 8

    def done(self):
        if self.n:
            self.out.append(self.acc & 255)
        return bytes(self.out)


def lzw_encode(pixels, min_size, clear_first=True, eoi=True, reset_full=True, clear_every=None,
               invalid_at=None, garbage_after_eoi=0, early_change=False):
    clear, end = 1 << min_size, (1 << min_size) + 1
    w = Bits()
    size, nxt, table = min_size + 1, end + 1, {}

    def reset():
        nonlocal size, nxt, table
        size, nxt, table = min_size + 1, end + 1, {}

    if clear_first:
        w.put(clear, size)
    emitted, prefix = 0, None
    for p in pixels:
        if prefix is None:
            prefix = p
            continue
        key = (prefix, p)
        if key in table:
            prefix = table[key]
            continue
        w.put(prefix, size)
        emitted += 1
        if invalid_at is not None and emitted == invalid_at:
            w.put(min(nxt + 5, (1 << size) - 1), size)
        if nxt < 4096:
            table[key] = nxt
            nxt += 1
            if nxt - 1 == (1 << size) - (1 if early_change else 0) and size < 12:
                size += 1
        elif reset_full:
            w.put(clear, size)
            reset()
        if clear_every and emitted % clear_every == 0 and nxt < 4096:
            w.put(clear, size)
            reset()
        prefix = p
    if prefix is not None:
        w.put(prefix, size)
        if nxt < 4096 and nxt == (1 << size) and size < 12:
            size += 1
    if eoi:
        w.put(end, size)
    for k in range(garbage_after_eoi):
        w.put((k * 7 + 3) % (1 << size), size)
    return w.done()


def sub_blocks(data, chunk=255, first_zero=False, terminator=True):
    out = bytearray(b'\x00' if first_zero else b'')
    for i in range(0, len(data), chunk):
        part = data[i:i + chunk]
        out.append(len(part))
        out += part
    if terminator:
        out.append(0)
    return bytes(out)


def table_bits(n):
    bits = 0
    while (2 << bits) < n:
        bits += 1
    return bits


def interlace_rows(pixels, w, h):
    rows = [pixels[y * w:(y + 1) * w] for y in range(h)]
    order = [y for start, step in INTERLACE_PASSES for y in range(start, h, step)]
    return [p for y in order for p in rows[y]]


def palette_bytes(colors, bits):
    raw = b''.join(bytes(c) for c in colors)
    return raw + bytes(3 * (2 << bits) - len(raw))


def build(spec):
    out = bytearray(b'GIF' + spec['version'].encode())
    fields = {'version': (3, 3)}
    bounds = [0, 6]
    gct = spec.get('gct')
    gbits = table_bits(len(gct)) if gct else 0
    flags = (0x80 | (spec.get('color_res', 7) << 4) | gbits) if gct else spec.get('lsd_flags', 0)
    fields.update({'screen_width': (6, 2), 'screen_height': (8, 2), 'lsd_flags': (10, 1), 'background': (11, 1), 'aspect': (12, 1)})
    out += struct.pack('<HHBBB', spec['width'], spec['height'], flags, spec.get('background', 0), spec.get('aspect', 0))
    if gct:
        fields['gct'] = (len(out), 3 * (2 << gbits))
        out += palette_bytes(gct, gbits)
    for ext in spec.get('extensions', []):
        bounds.append(len(out))
        fields.setdefault('ext_label', (len(out) + 1, 1))
        out += ext
    for i, fr in enumerate(spec['frames']):
        pre = f'f{i}.' if i else ''
        if fr.get('gce') is not None:
            g = fr['gce']
            bounds.append(len(out))
            base = len(out)
            fields.update({pre + 'gce_size': (base + 2, 1), pre + 'gce_flags': (base + 3, 1), pre + 'delay': (base + 4, 2),
                           pre + 'transparent_index': (base + 6, 1), pre + 'gce_terminator': (base + 7, 1)})
            gflags = (g.get('disposal', 0) << 2) | (2 if g.get('user_input') else 0) | (1 if g.get('transparent') is not None else 0)
            out += struct.pack('<BBBBHBB', 0x21, 0xF9, 4, gflags, g.get('delay', 0), g.get('transparent') or 0, 0)
        for ext in fr.get('extensions', []):
            bounds.append(len(out))
            out += ext
        bounds.append(len(out))
        base = len(out)
        lct = fr.get('lct')
        lbits = table_bits(len(lct)) if lct else 0
        dflags = (0x80 | lbits if lct else 0) | (0x40 if fr.get('interlace') else 0)
        fields.update({pre + 'left': (base + 1, 2), pre + 'top': (base + 3, 2), pre + 'frame_width': (base + 5, 2),
                       pre + 'frame_height': (base + 7, 2), pre + 'image_flags': (base + 9, 1)})
        out += struct.pack('<BHHHHB', 0x2C, fr['x'], fr['y'], fr['w'], fr['h'], dflags)
        if lct:
            fields[pre + 'lct'] = (len(out), 3 * (2 << lbits))
            out += palette_bytes(lct, lbits)
        fields[pre + 'lzw_min'] = (len(out), 1)
        out.append(fr['min_code'])
        pix = interlace_rows(fr['pixels'], fr['w'], fr['h']) if fr.get('interlace') else fr['pixels']
        data = lzw_encode(pix, fr.get('encode_min', fr['min_code']), **fr.get('lzw', {}))
        fields[pre + 'first_sub_block_len'] = (len(out), 1)
        bounds.append(len(out))
        out += sub_blocks(data, fr.get('chunk', 255), fr.get('first_zero', False), fr.get('terminator', True))
    bounds.append(len(out))
    if spec.get('trailer', True):
        fields['trailer'] = (len(out), 1)
        out.append(0x3B)
    return bytes(out), fields, sorted(set(bounds))


def netscape(loops):
    return b'\x21\xFF\x0BNETSCAPE2.0\x03\x01' + struct.pack('<H', loops) + b'\x00'


COMMENT = b'\x21\xFE' + sub_blocks(b'synthetic leniency fixture')
XMP = b'\x21\xFF\x0BXMP DataXMP' + sub_blocks(b'<x:xmpmeta xmlns:x="adobe:ns:meta/"/>')
PLAINTEXT = b'\x21\x01\x0C' + struct.pack('<HHHHBBBB', 0, 0, 8, 8, 8, 8, 1, 0) + sub_blocks(b'Hi')
UNKNOWN_EXT = b'\x21\x99' + sub_blocks(b'unknown label')


def pattern(rng, w, h, ncolors, kind):
    if kind == 'random':
        return [rng.randrange(ncolors) for _ in range(w * h)]
    if kind == 'stripes':
        return [(x // 2 + y) % ncolors for y in range(h) for x in range(w)]
    if kind == 'runs':  # long runs exercise KwKwK / dictionary growth
        out, c = [], 0
        while len(out) < w * h:
            out += [c] * rng.randrange(1, 40)
            c = (c + rng.randrange(1, ncolors)) % ncolors if ncolors > 1 else 0
        return out[:w * h]
    return [(x * 3 + y * 5) % ncolors for y in range(h) for x in range(w)]


def colors(rng, n):
    return [tuple(rng.randrange(256) for _ in range(3)) for _ in range(n)]


def seeds(rng):
    """Valid synthetic seeds covering versions, palettes, interlace, transparency, code sizes, frames, extensions."""
    sizes = [(1, 1), (2, 3), (5, 4), (16, 16), (33, 7), (7, 33), (64, 48), (3, 200)]
    out = []

    def frame(w, h, nc, min_code, x=0, y=0, kind='mixed', **kw):
        return dict(x=x, y=y, w=w, h=h, min_code=min_code, pixels=pattern(rng, w, h, nc, kind), **kw)

    for version in ('89a', '87a'):
        for mc in range(2, 9):
            w, h = sizes[(mc + len(version)) % len(sizes)]
            nc = 1 << mc if mc > 2 else rng.choice([2, 3, 4])
            out.append((f'basic-{version}-mc{mc}', dict(version=version, width=w, height=h, gct=colors(rng, nc),
                        frames=[frame(w, h, nc, mc, kind=rng.choice(['mixed', 'runs', 'random']))])))
    for w, h in sizes:
        out.append((f'size-{w}x{h}', dict(version='89a', width=w, height=h, gct=colors(rng, 16), frames=[frame(w, h, 16, 4)])))
    for w, h in [(8, 8), (5, 13), (16, 3), (21, 17), (1, 9)]:
        out.append((f'interlace-{w}x{h}', dict(version='89a', width=w, height=h, gct=colors(rng, 8), frames=[frame(w, h, 8, 3, interlace=True)])))
    for t in (0, 1, 3, 7):
        out.append((f'transparent-{t}', dict(version='89a', width=12, height=10, gct=colors(rng, 8),
                    frames=[frame(12, 10, 8, 3, gce=dict(transparent=t, delay=5))])))
    out.append(('local-only', dict(version='89a', width=10, height=10, gct=None, frames=[frame(10, 10, 4, 2, lct=colors(rng, 4))])))
    out.append(('local-and-global', dict(version='89a', width=10, height=10, gct=colors(rng, 4), frames=[frame(10, 10, 16, 4, lct=colors(rng, 16))])))
    out.append(('local-87a', dict(version='87a', width=9, height=6, gct=None, frames=[frame(9, 6, 2, 2, lct=colors(rng, 2))])))
    for d in range(4):
        out.append((f'anim-disposal{d}', dict(version='89a', width=16, height=12, gct=colors(rng, 8), extensions=[netscape(0)], frames=[
            frame(16, 12, 8, 3, gce=dict(disposal=d, delay=10)),
            frame(8, 6, 8, 3, x=4, y=3, gce=dict(disposal=d, delay=20, transparent=2)),
            frame(6, 6, 8, 3, x=1, y=5, gce=dict(disposal=d, delay=0, transparent=0))])))
    out.append(('anim-mixed-disposal', dict(version='89a', width=20, height=20, gct=colors(rng, 16), extensions=[netscape(1)], frames=[
        frame(20, 20, 16, 4, gce=dict(disposal=1, delay=7)),
        frame(10, 10, 16, 4, x=5, y=5, gce=dict(disposal=3, delay=3, transparent=1)),
        frame(10, 10, 16, 4, x=10, y=10, gce=dict(disposal=2, delay=1)),
        frame(20, 4, 16, 4, y=16, interlace=True, gce=dict(disposal=0, delay=100), lct=colors(rng, 16))])))
    out.append(('anim-no-gce', dict(version='89a', width=8, height=8, gct=colors(rng, 4), frames=[frame(8, 8, 4, 2), frame(4, 4, 4, 2, x=2, y=2)])))
    out.append(('anim-loop3-delays', dict(version='89a', width=6, height=6, gct=colors(rng, 4), extensions=[netscape(3)],
                frames=[frame(6, 6, 4, 2, gce=dict(delay=dl)) for dl in (0, 1, 2, 10, 65535)])))
    for name, ext in [('ext-comment', COMMENT), ('ext-xmp', XMP), ('ext-plaintext', PLAINTEXT), ('ext-unknown', UNKNOWN_EXT), ('ext-netscape0', netscape(0))]:
        out.append((name, dict(version='89a', width=7, height=5, gct=colors(rng, 4), extensions=[ext], frames=[frame(7, 5, 4, 2, gce=dict(delay=4))])))
    out.append(('big-dict-reset', dict(version='89a', width=96, height=96, gct=colors(rng, 256), frames=[frame(96, 96, 256, 8, kind='random')])))
    out.append(('big-dict-deferred-clear', dict(version='89a', width=96, height=96, gct=colors(rng, 256),
                frames=[frame(96, 96, 256, 8, kind='random', lzw=dict(reset_full=False))])))
    out.append(('big-runs-mc2', dict(version='89a', width=120, height=80, gct=colors(rng, 4), frames=[frame(120, 80, 4, 2, kind='runs')])))
    out.append(('small-subblocks', dict(version='89a', width=12, height=12, gct=colors(rng, 16), frames=[frame(12, 12, 16, 4, chunk=1)])))
    out.append(('frame-inset', dict(version='89a', width=20, height=15, gct=colors(rng, 8), background=3, frames=[frame(8, 5, 8, 3, x=6, y=4)])))
    return out


def lzw_variants(spec):
    """Structural LZW / sub-block edge cases on frame 0, re-encoded (not byte-patched)."""
    f0 = spec['frames'][0]
    mc = f0['min_code']
    opts = [('no-leading-clear', dict(lzw=dict(clear_first=False))), ('no-eoi', dict(lzw=dict(eoi=False))),
            ('garbage-after-eoi', dict(lzw=dict(garbage_after_eoi=6))), ('clear-every-3', dict(lzw=dict(clear_every=3))),
            ('clear-every-1', dict(lzw=dict(clear_every=1))), ('deferred-clear', dict(lzw=dict(reset_full=False))),
            ('invalid-code', dict(lzw=dict(invalid_at=2))), ('early-change', dict(lzw=dict(early_change=True))),
            ('subblock-1', dict(chunk=1)), ('subblock-7', dict(chunk=7)), ('first-subblock-zero', dict(first_zero=True)),
            ('no-block-terminator', dict(terminator=False)),
            ('pixels-short', dict(pixels=f0['pixels'][:max(0, len(f0['pixels']) // 2)])),
            ('pixels-one-short', dict(pixels=f0['pixels'][:-1])),
            ('pixels-long', dict(pixels=f0['pixels'] + f0['pixels'][:f0['w'] + 3])),
            ('pixels-none', dict(pixels=[]))]
    for m in range(0, 13):
        if m != mc:
            opts.append((f'min-code-byte-{m}', dict(min_code=m)))           # header byte disagrees with stream
    for m in range(max(1, mc + 1), 12):
        opts.append((f'encoded-min-code-{m}', dict(min_code=m, encode_min=m)))  # consistent but larger than needed
    if mc == 2 and max(f0['pixels'] or [0]) < 2:
        opts.append(('encoded-min-code-1', dict(min_code=1, encode_min=1)))
    for name, change in opts:
        s = dict(spec)
        s['frames'] = [dict(f0, **change)] + spec['frames'][1:]
        yield name, s


def structure_variants(spec):
    yield 'no-trailer', dict(spec, trailer=False)
    yield 'no-palette', dict(spec, gct=None, frames=[dict(f, lct=None) for f in spec['frames']])
    if spec.get('gct') and len(spec['gct']) > 2:
        yield 'short-global-palette', dict(spec, gct=spec['gct'][:2])
    yield 'frame-beyond-canvas', dict(spec, frames=[dict(spec['frames'][0], x=spec['width'] - 1, y=spec['height'] - 1)] + spec['frames'][1:])
    yield 'frame-larger-than-canvas', dict(spec, width=max(1, spec['width'] // 2), height=max(1, spec['height'] // 2))
    yield 'canvas-zero', dict(spec, width=0, height=0)
    yield 'canvas-larger', dict(spec, width=spec['width'] + 17, height=spec['height'] + 9)
    yield 'duplicate-frame', dict(spec, frames=spec['frames'] + [spec['frames'][0]])
    for d in range(8):
        yield f'disposal-{d}', dict(spec, frames=[dict(f, gce=dict(f.get('gce') or {}, disposal=d)) for f in spec['frames']])
    nc = len(spec['gct'] or spec['frames'][0].get('lct') or [0, 0])
    for t in sorted({0, nc - 1, nc, 255}):
        yield f'transparent-index-{t}', dict(spec, frames=[dict(f, gce=dict(f.get('gce') or {}, transparent=t)) for f in spec['frames']])
    yield 'interlace-toggle', dict(spec, frames=[dict(f, interlace=not f.get('interlace')) for f in spec['frames']])
    yield 'gce-without-image', dict(spec, frames=spec['frames'], extensions=spec.get('extensions', []) + [b'\x21\xF9\x04\x00\x05\x00\x00\x00'])
    for name, ext in [('comment', COMMENT), ('netscape-loop0', netscape(0)), ('netscape-loop1', netscape(1)),
                      ('plaintext', PLAINTEXT), ('unknown-ext', UNKNOWN_EXT), ('xmp', XMP)]:
        yield 'insert-' + name, dict(spec, extensions=spec.get('extensions', []) + [ext])


def mutations(data, fields, bounds, spec, rng):
    n = len(data)
    for name, s in lzw_variants(spec):
        yield build(s)[0], {'rule': 'lzw:' + name.rsplit('-', 1)[0] if name[-1].isdigit() else 'lzw:' + name, 'field': 'f0.lzw', 'value': name}
    for name, s in structure_variants(spec):
        try:
            b = build(s)[0]
        except (struct.error, ValueError):
            continue
        yield b, {'rule': 'struct:' + (name.rsplit('-', 1)[0] if name[-1].isdigit() else name), 'field': 'structure', 'value': name}
    for p in sorted({x for b in bounds for x in (b - 1, b, b + 1)} | {n // 2, n - 2, n - 1}):
        if 0 < p < n:
            yield data[:p], {'rule': 'truncation', 'field': 'length', 'value': p}
    for tail, label in [(b'\x00', 'zero-1'), (bytes(16), 'zero-16'), (b'\x3B', 'second-trailer'), (b'garbage!', 'ascii'),
                        (b'\x2C' + bytes(9), 'image-descriptor-after-trailer'), (b'\x21\xFE\x01x\x00', 'comment-after-trailer')]:
        yield data + tail, {'rule': 'trailing-data', 'field': 'tail', 'value': label}
    for value, label in [(b'87a', '87a'), (b'89a', '89a'), (b'88a', '88a'), (b'89b', '89b'), (b'   ', 'spaces')]:
        if data[3:6] != value:
            yield data[:3] + value + data[6:], {'rule': 'header:version', 'field': 'version', 'value': label}
    yield b'gif' + data[3:], {'rule': 'header:signature', 'field': 'signature', 'value': 'gif'}
    for name, (off, size) in fields.items():
        if name in ('version', 'gct', 'lct') or name.endswith('.lct') or off + size > n or size > 2:
            continue
        old = int.from_bytes(data[off:off + size], 'little')
        top = (1 << (8 * size)) - 1
        vals = {0, 1, 2, 127, 128, 254, 255, old - 1, old + 1, top}
        if size == 2:
            vals |= {256, 257, 4095, 4096, 32767, 32768, 65535, spec['width'], spec['height'], spec['width'] + 1, spec['height'] + 1}
        else:
            vals |= {old ^ (1 << b) for b in range(8)}
        for v in sorted(x for x in vals if 0 <= x <= top and x != old):
            yield data[:off] + v.to_bytes(size, 'little') + data[off + size:], {'rule': 'field:' + name.split('.')[-1], 'field': name, 'value': v, 'old': old}
    for _ in range(20):
        p = rng.randrange(n)
        a = bytearray(data)
        a[p] ^= 1 << rng.randrange(8)
        yield bytes(a), {'rule': 'byte-flip', 'field': 'offset', 'value': p}


def interleave(candidates):
    """Round-robin over rules so a bounded run reaches every rule of every seed early."""
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
    base = []
    series = []
    for name, spec in seeds(rng):
        b, fields, bounds = build(spec)
        meta = {'source': 'synthetic', 'name': name, 'kind': name.split('-')[0], 'rule': 'seed:' + name.split('-')[0]}
        base.append((b, meta))
        series.append((name, b, fields, bounds, spec))
    order = list(range(len(series)))
    rng.shuffle(order)
    out = base[:limit]
    iters = {i: interleave(mutations(series[i][1], series[i][2], series[i][3], series[i][4], random.Random(f'{seed}:{series[i][0]}'))) for i in order}
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
