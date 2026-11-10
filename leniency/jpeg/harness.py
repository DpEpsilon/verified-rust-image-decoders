#!/usr/bin/env python3
"""JPEG (image/jpeg) leniency adapter.

Seeds are synthetic: deterministic pixel patterns encoded by the system libjpeg-turbo
through runners/jpeggen.c, plus jpegenc.py for 12-bit (SOF1) and lossless (SOF3) streams.
Mutations are marker-aware: every changed field is recorded with its segment, offset,
old and new value.
"""
import random
import struct
import subprocess
import sys
from pathlib import Path

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
import jt_harness as jt  # noqa: E402
import jpegenc  # noqa: E402

GEN = jt.BIN / 'jpeggen'
RUST = jt.BIN / 'leniency-jt-runner'


def pattern(w, h, ch, variant):
    """Deterministic content: smooth gradients, one hard edge, low-amplitude seeded noise."""
    rng = random.Random(variant * 7919 + w * 31 + h)
    out = bytearray()
    for y in range(h):
        for x in range(w):
            edge = 60 if (x * 3 > w * 2) ^ (y * 2 > h) else 0
            base = [(x * 255 // max(1, w - 1)), (y * 255 // max(1, h - 1)), ((x + y) * 127 // max(1, w + h - 2)) + 64, 128 + (x - y) % 64]
            for c in range(ch):
                v = base[c] if variant % 2 == 0 else (base[c] // 2 + edge + 40)
                out.append(max(0, min(255, v + rng.randrange(-3, 4))))
    return bytes(out)


SIZES = [(1, 1), (8, 8), (16, 16), (17, 9), (33, 31), (64, 48), (7, 66)]
SUBSAMPLING = {'444': '11,11,11', '422': '21,11,11', '420': '22,11,11', '440': '12,11,11', '411': '41,11,11', '410': '42,11,11'}


def seed_specs():
    """(name, input kind, jpeggen parameters, orientation). Covers process, colour, sampling, markers."""
    specs = []
    for s, f in SUBSAMPLING.items():
        for wh in [(33, 31), (17, 9)]:
            specs.append((f'ycc-{s}-{wh[0]}x{wh[1]}', 'rgb', wh, dict(cs='ycc', samp=f)))
    for wh in SIZES:
        specs.append((f'size-ycc-420-{wh[0]}x{wh[1]}', 'rgb', wh, dict(cs='ycc', samp='22,11,11')))
        specs.append((f'gray-{wh[0]}x{wh[1]}', 'gray', wh, dict(cs='gray')))
    for cs in ('rgb', 'cmyk', 'ycck'):
        kind = 'rgb' if cs == 'rgb' else 'cmyk'
        for wh in [(16, 16), (33, 31)]:
            specs.append((f'{cs}-{wh[0]}x{wh[1]}', kind, wh, dict(cs=cs, samp='11,11,11' if cs == 'rgb' else '')))
    specs.append(('ycck-420-33x31', 'cmyk', (33, 31), dict(cs='ycck', samp='22,11,11,22')))
    for mode, extra in [('progressive', dict(prog=1)), ('arithmetic', dict(arith=1)), ('arith-progressive', dict(arith=1, prog=1)),
                        ('optimized', dict(opt=1)), ('extended-q5', dict(q=5, baseline=0)), ('q100', dict(q=100)),
                        ('rst1', dict(rst=1)), ('rst3', dict(rst=3)), ('rstrow', dict(rstrows=1)), ('rst1-prog', dict(rst=1, prog=1)),
                        ('separate-scans', dict(scans='separate')), ('separate-scans-rst', dict(scans='separate', rst=2)),
                        ('no-jfif', dict(jfif=0)), ('adobe-ycc', dict(adobe=1)), ('icc300', dict(icc=300)), ('icc70000', dict(icc=70000)),
                        ('comment', dict(com=40)), ('ifast', dict(dct='ifast')), ('smooth', dict(smooth=50))]:
        for wh in [(33, 31)]:
            specs.append((f'{mode}-420-{wh[0]}x{wh[1]}', 'rgb', wh, dict(cs='ycc', samp='22,11,11', **extra)))
            if mode in ('progressive', 'arithmetic', 'rst1', 'extended-q5'):
                specs.append((f'{mode}-gray-{wh[0]}x{wh[1]}', 'gray', wh, dict(cs='gray', **extra)))
                specs.append((f'{mode}-cmyk-{wh[0]}x{wh[1]}', 'cmyk', wh, dict(cs='cmyk', **extra)))
    for o in range(1, 9):
        specs.append((f'exif-orientation-{o}', 'rgb', (17, 9), dict(cs='ycc', samp='22,11,11', exif=o)))
    specs.append(('exif-orientation-6-progressive', 'rgb', (17, 9), dict(cs='ycc', samp='22,11,11', exif=6, prog=1)))
    return specs


def seeds(workdir):
    workdir.mkdir(parents=True, exist_ok=True)
    out = []
    for name, kind, (w, h), params in seed_specs():
        ch = {'gray': 1, 'rgb': 3, 'cmyk': 4}[kind]
        raw = workdir / 'seed.raw'
        raw.write_bytes(pattern(w, h, ch, len(out)))
        dst = workdir / 'seed.jpg'
        args = [str(GEN), str(dst), str(raw), f'w={w}', f'h={h}', f'in={kind}'] + [f'{k}={v}' for k, v in params.items() if v != '']
        subprocess.run(args, check=True, capture_output=True)
        meta = {'source': 'synthetic', 'name': name, 'rule': 'seed', 'generator': 'jpeggen/libjpeg-turbo ' + ' '.join(args[4:])}
        if 'exif' in params:
            meta['orientation'] = params['exif']
        out.append((dst.read_bytes(), meta))
    for (w, h) in [(16, 16), (17, 9)]:
        for nc in (1, 3):
            planes = [[(x * 4095 // max(1, w - 1) + c * 600 + y * 37) % 4096 for y in range(h) for x in range(w)] for c in range(nc)]
            out.append((jpegenc.encode_dct(planes, w, h, 12), {'source': 'synthetic', 'name': f'sof1-12bit-{nc}c-{w}x{h}', 'rule': 'seed',
                                                               'generator': 'jpegenc.encode_dct precision 12'}))
            # Control: the same Python DCT encoder at 8-bit precision, to show the encoder itself is decodable.
            planes8 = [[v >> 4 for v in p] for p in planes]
            out.append((jpegenc.encode_dct(planes8, w, h, 8), {'source': 'synthetic', 'name': f'sof1-8bit-python-{nc}c-{w}x{h}', 'rule': 'seed',
                                                               'generator': 'jpegenc.encode_dct precision 8 (control)'}))
    for precision, pred, nc in [(8, 1, 1), (8, 4, 3), (12, 1, 1), (16, 7, 1)]:
        w, h = 17, 9
        top = (1 << precision) - 1
        planes = [[(x * top // (w - 1) + c * 50 + y * 3) % (top + 1) for y in range(h) for x in range(w)] for c in range(nc)]
        out.append((jpegenc.encode_lossless(planes, w, h, precision, pred),
                    {'source': 'synthetic', 'name': f'sof3-lossless-p{precision}-pred{pred}-{nc}c', 'rule': 'seed', 'generator': 'jpegenc.encode_lossless'}))
    return out


def segments(b):
    """List of (marker, offset, length-field-value) up to and including the first SOS; plus scan end."""
    segs, p = [], 2
    while p + 2 <= len(b):
        if b[p] != 0xFF:
            break
        m = b[p + 1]
        if m == 0xFF:
            p += 1
            continue
        if m in (0xD8, 0x01) or 0xD0 <= m <= 0xD7:
            segs.append((m, p, 0))
            p += 2
            continue
        if m == 0xD9:
            segs.append((m, p, 0))
            break
        if p + 4 > len(b):
            break
        n = struct.unpack('>H', b[p + 2:p + 4])[0]
        segs.append((m, p, n))
        if m == 0xDA:
            q = p + 2 + n
            while q + 1 < len(b) and not (b[q] == 0xFF and b[q + 1] not in (0,) and not 0xD0 <= b[q + 1] <= 0xD7):
                q += 1
            p = q
            continue
        p += 2 + n
    return segs


NAMES = {0xC0: 'SOF0', 0xC1: 'SOF1', 0xC2: 'SOF2', 0xC3: 'SOF3', 0xC4: 'DHT', 0xC9: 'SOF9', 0xCA: 'SOF10', 0xCC: 'DAC', 0xDB: 'DQT',
         0xDA: 'SOS', 0xDD: 'DRI', 0xE0: 'APP0', 0xE1: 'APP1', 0xE2: 'APP2', 0xEE: 'APP14', 0xFE: 'COM', 0xD9: 'EOI'}


def setb(b, pos, val, n=1):
    a = bytearray(b)
    a[pos:pos + n] = val.to_bytes(n, 'big')
    return bytes(a)


def variants(b, rng):
    segs = segments(b)
    L = len(b)
    def mk(data, rule, field, value, old=None):
        m = {'rule': rule, 'field': field, 'value': value}
        if old is not None:
            m['old'] = old
        return data, m
    sos = [s for s in segs if s[0] == 0xDA]
    scan0 = sos[0][1] + 2 + sos[0][2] if sos else L
    # Truncation at every marker boundary, inside the first scan and at the end.
    for m, p, n in segs:
        yield mk(b[:p], 'truncate-at-marker', NAMES.get(m, hex(m)), p)
    for frac in (1, 4, 2):
        if scan0 < L - 2:
            q = scan0 + (L - 2 - scan0) // frac if frac > 1 else scan0 + 1
            yield mk(b[:q], 'truncate-in-scan', 'offset', q)
    yield mk(b[:L - 2], 'missing-EOI', 'length', L - 2)
    yield mk(b[:L - 1], 'missing-EOI', 'length', L - 1)
    for n in (1, 2, 16, 256):
        yield mk(b + bytes(n), 'trailing-bytes', 'zeros', n)
    yield mk(b + b'\xff' * 8, 'trailing-bytes', 'ff', 8)
    yield mk(b + b, 'trailing-bytes', 'second-image', L)
    yield mk(b'\x00' + b, 'leading-garbage', 'bytes', 1)
    yield mk(b'\xff\xd8' + b, 'leading-garbage', 'duplicate-SOI', 2)
    yield mk(b[:2] + b'\xff\xff\xff' + b[2:], 'fill-bytes', 'after-SOI', 3)
    for m, p, n in segs:
        name = NAMES.get(m, hex(m))
        if n:
            for v in sorted({0, 1, 2, n - 1, min(n + 1, 0xFFFF), 0xFFFF} - {n}):
                yield mk(setb(b, p + 2, v, 2), 'segment-length', name, v, n)
            if m not in (0xDA, 0xC0, 0xC1, 0xC2, 0xC3):
                yield mk(b[:p] + b[p + 2 + n:], 'remove-segment', name, p)
                yield mk(b[:p] + b[p:p + 2 + n] + b[p:], 'duplicate-segment', name, p)
        if m in (0xC0, 0xC1, 0xC2, 0xC3, 0xC9, 0xCA):
            for new in [0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCF]:
                if new != m:
                    yield mk(setb(b, p + 1, new), 'SOF-marker', name, NAMES.get(new, hex(new)), name)
            prec, hh, ww, nc = b[p + 4], struct.unpack('>H', b[p + 5:p + 7])[0], struct.unpack('>H', b[p + 7:p + 9])[0], b[p + 9]
            for v in [0, 1, 2, 7, 9, 12, 16, 255]:
                if v != prec:
                    yield mk(setb(b, p + 4, v), 'SOF-precision', 'P', v, prec)
            for field, pos, old in (('Y', p + 5, hh), ('X', p + 7, ww)):
                for v in sorted({0, 1, old - 1, old + 1, old * 2, 65535} - {old}):
                    if 0 <= v <= 65535:
                        yield mk(setb(b, pos, v, 2), 'SOF-dimension', field, v, old)
            for v in [0, 1, 2, 3, 4, 5, 255]:
                if v != nc:
                    yield mk(setb(b, p + 9, v), 'SOF-components', 'Nf', v, nc)
            for c in range(min(nc, 4)):
                base = p + 10 + 3 * c
                if base + 3 > L:
                    break
                for v in [0x00, 0x11, 0x12, 0x21, 0x22, 0x13, 0x31, 0x41, 0x14, 0x44, 0x55, 0xFF]:
                    if v != b[base + 1]:
                        yield mk(setb(b, base + 1, v), 'SOF-sampling', f'H/V[{c}]', hex(v), hex(b[base + 1]))
                for v in [0, 1, 2, 3, 4, 255]:
                    if v != b[base + 2]:
                        yield mk(setb(b, base + 2, v), 'SOF-quant-table', f'Tq[{c}]', v, b[base + 2])
                for v in [b[p + 10] if c else (b[p + 13] if nc > 1 else 9), 0, 255]:
                    if v != b[base]:
                        yield mk(setb(b, base, v), 'SOF-component-id', f'C[{c}]', v, b[base])
        if m == 0xDA:
            ns = b[p + 4]
            for v in [0, 1, 2, 3, 4, 5]:
                if v != ns:
                    yield mk(setb(b, p + 4, v), 'SOS-components', 'Ns', v, ns)
            for c in range(ns):
                base = p + 5 + 2 * c
                for v in [0x00, 0x01, 0x10, 0x11, 0x22, 0x33, 0x44, 0xFF]:
                    if v != b[base + 1]:
                        yield mk(setb(b, base + 1, v), 'SOS-table-selectors', f'Td/Ta[{c}]', hex(v), hex(b[base + 1]))
                yield mk(setb(b, base, 0x77), 'SOS-component-selector', f'Cs[{c}]', 0x77, b[base])
            tail = p + 5 + 2 * ns
            for field, off, vals in (('Ss', 0, [0, 1, 63, 64, 255]), ('Se', 1, [0, 1, 62, 63, 64, 255]), ('Ah/Al', 2, [0x00, 0x01, 0x10, 0x0D, 0xFF])):
                for v in vals:
                    if tail + off < L and v != b[tail + off]:
                        yield mk(setb(b, tail + off, v), 'SOS-spectral', field, v, b[tail + off])
        if m == 0xDB:
            pq = b[p + 4]
            for v in [0x00, 0x01, 0x03, 0x04, 0x0F, 0x10, 0x13, 0x20, 0xFF]:
                if v != pq:
                    yield mk(setb(b, p + 4, v), 'DQT-Pq/Tq', 'Pq/Tq', hex(v), hex(pq))
            yield mk(setb(b, p + 5, 0, 1 if pq < 16 else 2), 'DQT-zero-entry', 'Q[0]', 0)
            a = bytearray(b)
            for k in range(p + 5, min(p + 5 + 64, L)):
                a[k] = 0
            yield mk(bytes(a), 'DQT-zero-entry', 'Q[all]', 0)
            yield mk(setb(b, p + 5, 255), 'DQT-max-entry', 'Q[0]', 255)
        if m == 0xC4:
            tc = b[p + 4]
            for v in [0x00, 0x01, 0x02, 0x03, 0x04, 0x10, 0x11, 0x14, 0x20, 0xFF]:
                if v != tc:
                    yield mk(setb(b, p + 4, v), 'DHT-Tc/Th', 'Tc/Th', hex(v), hex(tc))
            counts = list(b[p + 5:p + 21])
            for k, v in [(0, 2), (0, 3), (15, 255), (1, 0), (8, 200)]:
                if counts and counts[k] != v:
                    yield mk(setb(b, p + 5 + k, v), 'DHT-counts', f'L{k + 1}', v, counts[k])
            a = bytearray(b)
            a[p + 5:p + 21] = bytes(16)
            yield mk(bytes(a), 'DHT-counts', 'all-zero', 0)
            if p + 21 < L:
                yield mk(setb(b, p + 21, 0xFF), 'DHT-symbol', 'V[0]', 255, b[p + 21])
        if m == 0xDD:
            ri = struct.unpack('>H', b[p + 4:p + 6])[0]
            for v in [0, 1, 2, 65535]:
                if v != ri:
                    yield mk(setb(b, p + 4, v, 2), 'DRI-interval', 'Ri', v, ri)
        if m == 0xE1 and b[p + 4:p + 10] == b'Exif\x00\x00':
            t = p + 10
            for v in [0, 9, 255, 65535]:
                yield mk(setb(b, t + 8 + 2 + 8, v, 2), 'EXIF-orientation', 'Orientation', v)
            for v in [0, 1, 0xFFFF]:
                yield mk(setb(b, t + 4, v, 4), 'EXIF-ifd-offset', 'IFD0', v)
            yield mk(setb(b, t + 8 + 2 + 2, 4), 'EXIF-orientation-type', 'type', 4)
            yield mk(setb(b, t + 8 + 2 + 4, 0xFFFFFFFF, 4), 'EXIF-orientation-count', 'count', 0xFFFFFFFF)
            yield mk(setb(b, t, 0x4949, 2), 'EXIF-byte-order', 'II-on-MM-data', 'II')
        if m == 0xE2 and b[p + 4:p + 16] == b'ICC_PROFILE\x00':
            for field, off, vals in (('seq', 16, [0, 2, 255]), ('count', 17, [0, 2, 255])):
                for v in vals:
                    yield mk(setb(b, p + off, v), 'ICC-chunk', field, v, b[p + off])
            yield mk(setb(b, p + 18, 0xFFFFFFFF, 4), 'ICC-declared-size', 'size', 0xFFFFFFFF)
        if m == 0xEE and b[p + 4:p + 9] == b'Adobe':
            for v in [0, 1, 2, 3, 255]:
                yield mk(setb(b, p + 15, v), 'Adobe-transform', 'transform', v, b[p + 15])
        if m == 0xE0 and b[p + 4:p + 9] == b'JFIF\x00':
            for v in [0x0000, 0x0100, 0x0300, 0xFFFF]:
                yield mk(setb(b, p + 9, v, 2), 'JFIF-version', 'version', hex(v))
    # Restart markers inside entropy-coded data: renumber, delete.
    rst = [i for i in range(scan0, L - 1) if b[i] == 0xFF and 0xD0 <= b[i + 1] <= 0xD7]
    for i in rst[:3]:
        yield mk(setb(b, i + 1, 0xD0 + ((b[i + 1] - 0xD0 + 3) % 8)), 'RST-sequence', 'RSTn', i)
        yield mk(b[:i] + b[i + 2:], 'RST-removed', 'RSTn', i)
    if not rst and scan0 + 4 < L - 2:
        mid = scan0 + (L - 2 - scan0) // 2
        while mid < L - 2 and (b[mid] == 0xFF or b[mid - 1] == 0xFF):
            mid += 1
        yield mk(b[:mid] + b'\xff\xd0' + b[mid:], 'RST-unexpected', 'RST0', mid)
    # Byte stuffing: drop a stuffed zero.
    stuffed = [i for i in range(scan0, L - 2) if b[i] == 0xFF and b[i + 1] == 0]
    for i in stuffed[:2]:
        yield mk(b[:i + 1] + b[i + 2:], 'unstuffed-FF', 'offset', i)
    # Random single-bit flips: half in headers, half in entropy-coded data.
    for k in range(24):
        lo, hi = (2, scan0) if k % 2 == 0 else (scan0, L - 2)
        if hi <= lo:
            lo, hi = 2, L
        pos = rng.randrange(lo, hi)
        bit = rng.randrange(8)
        a = bytearray(b)
        a[pos] ^= 1 << bit
        yield mk(bytes(a), 'byte-flip-header' if k % 2 == 0 else 'byte-flip-scan', 'offset', pos)
    for k in range(6):
        pos = rng.randrange(scan0, max(scan0 + 1, L - 2))
        a = bytearray(b)
        a[pos] = rng.randrange(256)
        yield mk(bytes(a), 'byte-replace-scan', 'offset', pos)


def interleave(gen):
    """Round-robin across rules so a bounded run reaches every mutation class of a seed."""
    buckets = {}
    for data, meta in gen:
        buckets.setdefault(meta['rule'], []).append((data, meta))
    queues = list(buckets.values())
    k = 0
    while any(queues):
        for q in queues:
            if k < len(q):
                yield q[k]
        k += 1
        queues = [q for q in queues if k < len(q)]


def generate(limit, seed):
    rng = random.Random(seed)
    base = seeds(Path(f'/tmp/claude-0/leniency-jpeg/gen'))
    series = list(base)
    rng.shuffle(series)
    items = base[:limit]
    its = [iter(interleave(variants(b, rng))) for b, _ in series]
    active = list(range(len(series)))
    while len(items) < limit and active:
        for i in active[:]:
            try:
                data, meta = next(its[i])
            except StopIteration:
                active.remove(i)
                continue
            meta.update(source='mutation', seed=series[i][1]['name'])
            items.append((data, meta))
            if len(items) >= limit:
                break
    return items, len(base), len(series)


class Jpeg(jt.Adapter):
    fmt, ext = 'jpeg', 'jpg'
    runners = {
        'glycin': (RUST, ['jpeg:glycin']),
        'image': (RUST, ['jpeg:image']),
        'jpeg-decoder': (RUST, ['jpeg:jpeg-decoder']),
        'libjpeg': (jt.BIN / 'libjpeg-runner', []),
        'gdk-file': (jt.BIN / 'gdk-jt', ['jpeg', 'file']),
        'gdk-chunk': (jt.BIN / 'gdk-jt', ['jpeg', 'forced-chunk']),
    }
    families = {'zune-jpeg 0.5.15': ['glycin', 'image'], 'jpeg-decoder 0.3.2': ['jpeg-decoder'],
                'libjpeg-turbo 2.1.5 (8-bit build)': ['libjpeg', 'gdk-file', 'gdk-chunk']}
    unavailable = {
        'libjpeg-turbo 3.x / 12-bit libjpeg': 'only the 8-bit 2.1.5 build is installed; building 3.x needs cmake, which is absent',
        'Pillow': 'not installed and PyPI is outside the allowed network',
        'stb_image / other C decoders': 'no source in the environment; not fetched',
    }

    def generate(self, limit, seed):
        return generate(limit, seed)


AD = Jpeg()
AD.notes = [
    'glycin column: `glycin_image_rs::ImgLoader` for `image/jpeg` (gufo metadata strip → zune-jpeg with `DecoderOptions::new_fast()`), then '
    'glycin\'s own `change_memory_format` to R8g8b8a8, which also performs glycin\'s CMYK→RGB conversion. The orientation glycin-core would '
    'apply (gufo-exif) is reported, not applied. image column: `image::codecs::jpeg::JpegDecoder` (zune-jpeg) → `to_rgba8`. '
    'jpeg-decoder: CMYK32 output normalised as R=(255−C)(255−K)/255. libjpeg: default `jpeg_read_header`/`jpeg_start_decompress` '
    '(islow IDCT, fancy upsampling), CMYK with an Adobe marker treated as inverted. GDK: built-in JPEG loader by path and by a forced '
    '`jpeg` loader fed 4096-byte writes; GDK performs its own CMYK conversion. libjpeg warnings (e.g. premature end of data) are recorded '
    'in the per-input `warnings` field but do not turn an accepted image into a rejection.',
]

AD.findings = [
    'Non-interleaved sequential scans (one scan per component, libjpeg `scan_info`) with 4:2:0 sampling: zune-jpeg (glycin, image) output differs from jpeg-decoder and libjpeg by up to 227-255 per channel, while those two agree within tolerance; the restart-interval variant behaves the same. These are valid encoder outputs.',
    'Arithmetic coding (sequential and progressive; gray, YCbCr, CMYK): only the libjpeg-turbo family (libjpeg, both GDK paths) decodes; glycin rejects with "DAC not supported" and jpeg-decoder reports an unsupported feature.',
    '12-bit extended sequential (SOF1, P=12) is rejected by all six columns; the same Python encoder at P=8 is decoded by all six, so no 12-bit-capable decoder is present here. Lossless SOF3 (P=8/12/16) is decoded only by jpeg-decoder.',
    'Truncated entropy data and a missing EOI: zune-jpeg and libjpeg both accept (libjpeg with a warning) but fill the missing area differently; jpeg-decoder rejects; GDK\'s incremental 4096-byte loader produces a third result that differs from GDK\'s path loader (211 truncate-in-scan, 146 missing-EOI changed inputs).',
    'Missing or mislabelled Huffman tables (DHT removed, or Tc/Th remapped, or scan Td/Ta pointing at an undefined table): libjpeg-turbo substitutes its default tables and decodes; zune-jpeg and jpeg-decoder reject (126 / 227 / 172 changed inputs).',
    'Out-of-range spectral fields in a sequential scan (e.g. Ss=64): libjpeg ignores them and decodes; zune-jpeg and jpeg-decoder reject (266). A zero quantisation entry is accepted by zune-jpeg and libjpeg but rejected by jpeg-decoder (234).',
    'zune-jpeg is the most lenient on stream structure: it accepts a sequential stream relabelled SOF2, a removed 0xFF00 stuffing byte, a duplicate SOI and a scan component selector that names no frame component, all of which libjpeg and jpeg-decoder reject (66 / 79 / 1 / 4).',
    'zune-jpeg is stricter on metadata: an Adobe APP14 transform value of 3 or an APP0 length field of 0 makes glycin and image reject while libjpeg decodes (36 / 152).',
    'Same bytes, different pixels without any rejection: corrupted Huffman symbols, a 255 quantisation entry, altered sampling factors and unexpected RST markers give three tolerance groups (zune, jpeg-decoder, libjpeg-turbo) in hundreds of inputs; see the pixel table.',
    'EXIF orientation: no column rotates its output. glycin (the transform glycin-core would apply), image and GDK (pixbuf option) all report tags 2-8 consistently; jpeg-decoder and libjpeg do not parse EXIF. GDK\'s forced incremental loader timed out on 4 inputs (counted, not investigated).',
]
AD.public_findings = AD.findings

if __name__ == '__main__':
    jt.main(AD)
