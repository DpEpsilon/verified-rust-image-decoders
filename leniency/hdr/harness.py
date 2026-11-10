#!/usr/bin/env python3
"""Radiance HDR (image/vnd.radiance) leniency adapter.

Decoders (identical bytes, one subprocess each). Output compared = decoded linear float RGB samples exactly as the
library returns them: no tone mapping, no reorientation, no exposure/COLORCORR scaling by the harness.
  image            image 0.25.10 HdrDecoder::new (strict header parsing)        }
  image-nonstrict  image 0.25.10 HdrDecoder::new_nonstrict (what glycin uses)   } same implementation: ONE vote
  glycin           glycin-image-rs ImgLoader, mime image/vnd.radiance           }
  radiant          radiant 0.3.0 (independent)
  hdrldr           hdrldr 0.1.2 (independent, flipcode-derived)
  stb              stb_image v2.30 stbi_loadf_from_memory, 3 components (C, public domain, fetched from github.com)
Unavailable: GDK (no HDR module installed), DirectXTex (not an HDR reader), Radiance reference tools (not fetched).

Comparison rule (documented, applied by refine()): when accepting decoders have the same dimensions but different
exact f32 bit patterns, the raw samples are re-dumped and two decoders are "equal" iff every sample satisfies
|a-b| <= 1e-6 * max(|a|,|b|) (about 8 ulp of f32), NaN matching NaN. Equal clusters share one hash; the exact bit-pattern
hash is kept otherwise. So `accept-different-pixels` means a difference larger than 1e-6 relative on some sample.
All inputs are synthetic (seeded PRNG); no third-party files.
"""
import argparse, math, random, struct, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'dds'))
import leniency_core as core  # noqa: E402
engine = core.engine

RUST = Path('/tmp/claude-0/target-hdr/release/leniency-hdr-runner')
STB = Path('/tmp/claude-0/target-hdr/stb-hdr')
TOL = 1e-6
ORIENT = ['-Y {a} +X {b}', '+Y {a} +X {b}', '-Y {a} -X {b}', '+Y {a} -X {b}', '+X {a} -Y {b}', '-X {a} -Y {b}', '+X {a} +Y {b}', '-X {a} +Y {b}']


# ---------------- builder ----------------
def pixels(rng, minor, major, style):
    rows = []
    for y in range(major):
        row = []
        for x in range(minor):
            if style == 'smooth': m = ((x * 9 + y * 5) % 120) + 100; row.append((m, m // 2 + 60, 255 - m // 2, 128 + (x // 4) % 3))
            elif style == 'noisy': row.append((rng.randrange(256), rng.randrange(256), rng.randrange(256), rng.randrange(100, 160)))
            elif style == 'flatcolour': row.append((200, 100, 50, 129))
            elif style == 'extremes': row.append(rng.choice([(255, 255, 255, 1), (255, 255, 255, 255), (128, 0, 0, 0), (0, 0, 0, 0), (1, 200, 3, 254), (255, 255, 255, 128), (128, 128, 128, 127), (200, 10, 10, 136)]))
            else: row.append((rng.randrange(128, 256), rng.randrange(256), rng.randrange(256), rng.randrange(120, 140)))
        rows.append(row)
    return rows


def rle_channel(rng, vals):
    out = bytearray(); i = 0; n = len(vals)
    while i < n:
        j = i
        while j + 1 < n and vals[j + 1] == vals[i] and j + 1 - i < 127: j += 1
        run = j - i + 1
        if run >= 3:
            out += bytes([128 + run, vals[i]]); i += run
        else:
            k = i; lit = []
            while k < n and len(lit) < 128 and not (k + 2 < n and vals[k] == vals[k + 1] == vals[k + 2]): lit.append(vals[k]); k += 1
            if not lit: lit = [vals[i]]; k = i + 1
            out += bytes([len(lit)]) + bytes(lit); i = k
    return bytes(out)


def encode_rows(rng, rows, mode):
    """mode: flat | new | old. Returns (data, [row start offsets])."""
    out = bytearray(); starts = []
    for row in rows:
        starts.append(len(out)); w = len(row)
        if mode == 'flat':
            for p in row: out += bytes(p)
        elif mode == 'new':
            out += bytes([2, 2, w >> 8, w & 255])
            for c in range(4): out += rle_channel(rng, [p[c] for p in row])
        else:  # old-style RLE: (1,1,1,e) pixel repeats previous pixel (count in 8-bit shifts)
            i = 0
            while i < w:
                j = i
                while j + 1 < w and row[j + 1] == row[i]: j += 1
                out += bytes(row[i]); rep = j - i; i = j + 1
                while rep > 0:
                    k = min(rep, 255); out += bytes([1, 1, 1, k]); rep -= k
    return bytes(out), starts


def build(rng, minor, major, *, mode='new', style='mixed', orient=0, header=None, magic=b'#?RADIANCE', eol=b'\n', fmt='FORMAT=32-bit_rle_rgbe'):
    rows = pixels(rng, minor, major, style)
    data, starts = encode_rows(rng, rows, mode)
    lines = [magic] if magic is not None else []
    for h in (header if header is not None else ['# synthetic', fmt]): lines.append(h.encode())
    head = eol.join(lines) + eol + eol if lines else b''
    dims = ORIENT[orient].format(a=major, b=minor).encode()
    pre = head + dims + eol
    return pre + data, {'major': major, 'minor': minor, 'mode': mode, 'data_off': len(pre), 'rows': [len(pre) + s for s in starts],
                        'dims_off': len(head), 'dims_len': len(dims), 'head_len': len(head), 'dims': dims.decode()}


def seeds(rng):
    out = []
    def add(name, family, minor, major, **kw):
        b, info = build(rng, minor, major, **kw); info.update(name=name, family=family); out.append((b, info))
    for (w, h) in [(1, 1), (3, 2), (5, 4), (7, 7), (16, 4), (33, 2)]: add(f'flat-{w}x{h}', 'flat-scanlines', w, h, mode='flat')
    add('flat-33000x1', 'flat-scanlines', 33000, 1, mode='flat', style='smooth')
    for (w, h, st) in [(8, 4, 'smooth'), (16, 16, 'mixed'), (64, 2, 'flatcolour'), (300, 3, 'smooth'), (1000, 2, 'noisy'), (130, 2, 'noisy')]:
        add(f'rle-new-{w}x{h}-{st}', 'rle-new', w, h, mode='new', style=st)
    for (w, h) in [(16, 2), (20, 3)]: add(f'rle-old-{w}x{h}', 'rle-old', w, h, mode='old', style='flatcolour')
    for o in range(1, 8):
        add(f'orient-{o}-rle-8x4', 'orientation', 8, 4, mode='new', style='smooth', orient=o)
        if o in (1, 3, 4): add(f'orient-{o}-flat-5x3', 'orientation', 5, 3, mode='flat', style='smooth', orient=o)
    add('extremes-flat-8x4', 'exponent-extremes', 8, 4, mode='flat', style='extremes')
    add('extremes-rle-16x4', 'exponent-extremes', 16, 4, mode='new', style='extremes')
    H = {'no-comment': ['FORMAT=32-bit_rle_rgbe'], 'comments': ['# a', '#b', 'FORMAT=32-bit_rle_rgbe', '# c'],
         'exposure': ['FORMAT=32-bit_rle_rgbe', 'EXPOSURE=2.0'], 'exposure-twice': ['FORMAT=32-bit_rle_rgbe', 'EXPOSURE=2.0', 'EXPOSURE=4.0'],
         'colorcorr': ['FORMAT=32-bit_rle_rgbe', 'COLORCORR=1.0 0.5 0.25'], 'primaries': ['FORMAT=32-bit_rle_rgbe', 'PRIMARIES=0.640 0.330 0.290 0.600 0.150 0.060 0.333 0.333'],
         'software-view': ['SOFTWARE=synthetic', 'FORMAT=32-bit_rle_rgbe', 'VIEW=-vtv', 'PIXASPECT=1.0'], 'xyze': ['FORMAT=32-bit_rle_xyze'],
         'no-format': ['# nothing'], 'empty-header': [], 'unknown-attr': ['FORMAT=32-bit_rle_rgbe', 'FOO=bar', 'BAZ']}
    for k, v in H.items():
        add(f'header-{k}-rle', 'header-lines', 8, 4, mode='new', style='smooth', header=v)
        add(f'header-{k}-flat', 'header-lines', 5, 3, mode='flat', style='smooth', header=v)
    add('magic-rgbe', 'header-lines', 8, 4, magic=b'#?RGBE', style='smooth')
    add('magic-none-pic', 'header-lines', 8, 4, magic=None, header=['FORMAT=32-bit_rle_rgbe'], style='smooth')
    add('crlf-rle', 'header-lines', 8, 4, eol=b'\r\n', style='smooth')
    return out


DIMS = ['-Y 0 +X 0', '-Y {h} +X 0', '-Y 0 +X {w}', '-Y {h1} +X {w}', '-Y {hm} +X {w}', '-Y {h} +X {w1}', '-Y {h} +X {wm}', '-Y 4294967295 +X 1', '-Y 1 +X 4294967295',
        '-Y 2147483648 +X 1', '-Y 65536 +X 65536', '-Y  {h}   +X   {w}', '-Y\t{h}\t+X\t{w}', '-Y {h} +X {w} extra', '+Y {h} +X {w}', '-Y {h} -X {w}', '+Y {h} -X {w}',
        '+X {w} -Y {h}', '-X {w} -Y {h}', '+X {w} +Y {h}', '-X {w} +Y {h}', '-Y +{h} +X {w}', '-Y {h}.0 +X {w}', '-Y 0x{h:x} +X {w}', '-y {h} +x {w}', '-Y -{h} +X {w}',
        'Y {h} X {w}', '-Y {h} +X', '-Y {h}', '-Y', '', '   ', '-Y {h} +X {w}\r', '-Y {h} +X {w} ', ' -Y {h} +X {w}', '-Y 16777216 +X 1', '-Y 1 +X 16777216', '-Y 16777217 +X 1']
MAGICS = [b'#?RGBE', b'#?radiance', b'#?', b'#?RADIANCE\r', b'', b'#!RADIANCE', b' #?RADIANCE', b'#?RADIANCEX', b'#?XYZ', b'#?RADIANCE ', b'?RADIANCE', b'#RADIANCE']
FORMATS = ['FORMAT=32-bit_rle_rgbe', 'FORMAT=32-bit_rle_xyze', 'FORMAT=32-bit_rle_rgb', 'FORMAT=32-bit_rle_rgbe ', 'FORMAT=', 'format=32-bit_rle_rgbe', 'FORMAT=32-bit_rle_rgbe\r', 'FORMAT=rgbe', 'FORMAT=32-bit_rle_rgbe;', 'FORMAT = 32-bit_rle_rgbe']
EXPOSURES = ['EXPOSURE=2.0', 'EXPOSURE=0.5', 'EXPOSURE=0', 'EXPOSURE=-1', 'EXPOSURE=nan', 'EXPOSURE=inf', 'EXPOSURE=1e40', 'EXPOSURE=abc', 'EXPOSURE=1.0 2.0', 'EXPOSURE= 2', 'EXPOSURE=', 'EXPOSURE=1e-40', 'EXPOSURE=2.0\r']
COLORCORRS = ['COLORCORR=1 1 1', 'COLORCORR=2 2 2', 'COLORCORR=1 1', 'COLORCORR=1 1 1 1', 'COLORCORR=a b c', 'COLORCORR=0 0 0', 'COLORCORR=', 'COLORCORR=1,1,1']
EXTRA_LINES = ['FOO', 'FOO=', '=', '=bar', ' ', 'X' * 5000, 'A=\x00B', 'PIXASPECT=2.0', 'PIXASPECT=0', 'PRIMARIES=1 2 3', 'SOFTWARE=' + 'z' * 300, 'GAMMA=2.2', 'VIEW=-vtv -vp 0 0 0', '#', '# ' + 'c' * 5000, '\x7f\xff']


def variants(b, info, rng):
    n = len(b); fixed = []; pool = []
    head = b[:info['head_len']]; dims_off = info['dims_off']; dl = info['dims_len']; data_off = info['data_off']
    w, h = info['minor'], info['major']
    def hv(newhead, rule, field, value): pool.append((newhead + b[info['head_len']:], {'rule': rule, 'field': field, 'value': value}))
    def dimv(text, i): pool.append((b[:dims_off] + text.encode('latin1') + b[dims_off + dl:], {'rule': 'dims-line', 'field': 'dims', 'value': text[:40]}))
    cuts = {0, 1, 2, 9, 10, 11, 12, 20, info['head_len'] - 1, info['head_len'], dims_off + dl, data_off - 1, data_off, data_off + 1, data_off + 3, data_off + 4, data_off + 5, n // 2, n - 1, n - 4}
    cuts |= {r + k for r in info['rows'][:3] for k in (0, 2, 3, 4, 5)}
    for c in sorted(x for x in cuts if 0 <= x < n): fixed.append((b[:c], {'rule': 'truncation', 'field': 'length', 'value': c}))
    for k in (1, 4, 16, 1024): fixed.append((b + bytes(k), {'rule': 'trailing-bytes', 'field': 'tail', 'value': k}))
    fixed.append((b + b'\n', {'rule': 'trailing-bytes', 'field': 'tail-newline', 'value': 1}))
    for _ in range(10):
        p = rng.randrange(n); a = bytearray(b); a[p] ^= 1 << rng.randrange(8)
        fixed.append((bytes(a), {'rule': 'byte-flip-header' if p < data_off else 'byte-flip-data', 'field': 'offset', 'value': p}))
    for _ in range(6):
        p = rng.randrange(data_off); a = bytearray(b); a[p] ^= 1 << rng.randrange(8)
        fixed.append((bytes(a), {'rule': 'byte-flip-header', 'field': 'offset', 'value': p}))
    for t in DIMS: dimv(t.format(h=h, w=w, h1=h + 1, hm=max(h - 1, 0), w1=w + 1, wm=max(w - 1, 0)), 0)
    for m in MAGICS:
        if head.startswith(b'#?'): hv(m + head[head.index(b'\n'):] if b'\n' in head else m, 'magic', 'magic', m.decode())
    if head.startswith(b'#?'): hv(head[head.index(b'\n') + 1:], 'magic', 'magic', 'removed')
    body = head.split(b'\n', 1)[1] if b'\n' in head else b''
    for f in FORMATS: hv(head.split(b'\n', 1)[0] + b'\n' + f.encode() + b'\n' + body, 'format-line', 'FORMAT', f)
    for e in EXPOSURES: hv(head.split(b'\n', 1)[0] + b'\n' + e.encode() + b'\n' + body, 'exposure-line', 'EXPOSURE', e)
    for c in COLORCORRS: hv(head.split(b'\n', 1)[0] + b'\n' + c.encode('latin1') + b'\n' + body, 'colorcorr-line', 'COLORCORR', c)
    for x in EXTRA_LINES: hv(head.split(b'\n', 1)[0] + b'\n' + x.encode('latin1') + b'\n' + body, 'extra-header-line', 'line', x[:30])
    for k in (1, 100, 1000): hv(head.split(b'\n', 1)[0] + b'\n' + b'# x\n' * k + body, 'many-header-lines', 'count', k)
    # header termination / line endings
    hv(head[:-1], 'header-end', 'blank-line', 'missing-blank')  # dims line follows header line directly
    hv(head + b'\n', 'header-end', 'blank-line', 'double-blank')
    hv(head.replace(b'\n', b'\r\n'), 'header-end', 'eol', 'crlf')
    hv(head.replace(b'\n', b'\r'), 'header-end', 'eol', 'cr-only')
    hv(head[:-1] + b' \n', 'header-end', 'blank-line', 'space-blank')
    hv(head + b' \n', 'header-end', 'blank-line', 'blank-then-space-line')
    hv(head.replace(b'\n', b'\n\n', 1), 'header-end', 'blank-line', 'blank-after-magic')
    hv(head + b'\x00', 'header-end', 'nul', 'nul-before-dims')
    # RLE / flat scanline structure
    for r_i, r in enumerate(info['rows'][:2]):
        if info['mode'] == 'new':
            for v in (0, w - 1, w + 1, 1, 7, 255, 256, 0x7fff, 0x8000, 0xffff):
                a = bytearray(b); a[r + 2:r + 4] = struct.pack('>H', v & 0xffff); pool.append((bytes(a), {'rule': 'rle-width-field', 'field': 'scanline-width', 'value': v}))
            for k, v in [(0, 1), (0, 3), (1, 1), (1, 3), (0, 0), (1, 0)]:
                a = bytearray(b); a[r + k] = v; pool.append((bytes(a), {'rule': 'rle-marker-bytes', 'field': f'byte{k}', 'value': v}))
            a = bytearray(b); a[r + 2] |= 0x80; pool.append((bytes(a), {'rule': 'rle-marker-bytes', 'field': 'width-hi-bit', 'value': 0x80}))
            for off in (4, 5, 6):
                for v in (0, 1, 127, 128, 129, 200, 255):
                    if r + off < n:
                        a = bytearray(b); a[r + off] = v; pool.append((bytes(a), {'rule': 'rle-control-byte', 'field': f'row{r_i}+{off}', 'value': v}))
            a = bytearray(b); del a[r + 4:r + 5]; pool.append((bytes(a), {'rule': 'rle-delete-byte', 'field': 'row', 'value': r + 4}))
            a = bytearray(b); a[r + 4:r + 4] = b'\x00'; pool.append((bytes(a), {'rule': 'rle-insert-byte', 'field': 'row', 'value': r + 4}))
        elif info['mode'] == 'old':
            for v in (0, 1, 2, 255):
                for off in range(0, 12):
                    if r + off + 3 < n:
                        a = bytearray(b); a[r + off + 3] = v; pool.append((bytes(a), {'rule': 'old-rle-count', 'field': f'row{r_i}+{off}', 'value': v}))
            a = bytearray(b); a[r:r + 4] = bytes([1, 1, 1, 5]); pool.append((bytes(a), {'rule': 'old-rle-first-pixel-marker', 'field': 'row', 'value': 5}))
        else:
            for v, tag in [((2, 2, 0, 128), 'rle-lookalike'), ((2, 2, 127, 128), 'rle-lookalike'), ((2, 2, 128, 128), 'rle-lookalike-hi'), ((1, 1, 1, 3), 'old-rle-marker')]:
                a = bytearray(b); a[r:r + 4] = bytes(v); pool.append((bytes(a), {'rule': 'flat-first-pixel-' + tag, 'field': 'row0', 'value': list(v)}))
    for r in info['rows'][:1]:
        for v in (0, 1, 2, 127, 128, 129, 254, 255):
            for k in range(1, 4):
                if r + 4 * k - 1 < n:
                    a = bytearray(b); a[r + 4 * k - 1] = v; pool.append((bytes(a), {'rule': 'exponent-byte-flat' if info['mode'] == 'flat' else 'byte-value', 'field': f'+{4*k-1}', 'value': v}))
    rng.shuffle(pool)
    return fixed, pool


def generate(limit, seed):
    rng = random.Random(seed)
    ss = seeds(rng)
    items = [(b, {'source': 'seed', 'name': i['name'], 'family': i['family'], 'rule': 'seed:' + i['family'], 'expected_ok': True}) for b, i in ss][:limit]
    gens = []
    for b, info in ss:
        fixed, pool = variants(b, info, rng)
        gens.append((info["name"], info["family"], iter(core.interleave_by_rule(fixed + pool, rng))))
    active = list(range(len(gens)))
    while len(items) < limit and active:
        for i in active[:]:
            try: b, m = next(gens[i][2])
            except StopIteration: active.remove(i); continue
            m.update(source='mutation', seed=gens[i][0], family=gens[i][1]); items.append((b, m))
            if len(items) >= limit: break
    return items, len(ss)


# ---------------- float comparison ----------------
def load_f32(path):
    import array
    a = array.array('f'); a.frombytes(Path(path).read_bytes()); return a


def close(x, y):
    if x == y: return True
    if math.isnan(x) and math.isnan(y): return True
    if math.isnan(x) or math.isnan(y) or math.isinf(x) or math.isinf(y): return False
    return abs(x - y) <= TOL * max(abs(x), abs(y))


def max_rel(a, b):
    m = 0.0
    for x, y in zip(a, b):
        if x == y or (math.isnan(x) and math.isnan(y)): continue
        if math.isnan(x) or math.isnan(y) or math.isinf(x) or math.isinf(y): return float('inf')
        m = max(m, abs(x - y) / max(abs(x), abs(y)))
    return m


class HdrConfig(core.Config):
    fmt = 'hdr'; ext = 'hdr'; adapter = HERE
    runners = {'image': (RUST, ['image']), 'image-nonstrict': (RUST, ['image-nonstrict']), 'glycin': (RUST, ['glycin']),
               'radiant': (RUST, ['radiant']), 'hdrldr': (RUST, ['hdrldr']), 'stb': (STB, [])}
    unavailable = {'gdk-pixbuf Radiance module': 'not installed (gdk-pixbuf-query-loaders lists none)', 'Radiance reference tools (ra_*)': 'not fetched',
                   'DirectXTex': 'does not read Radiance HDR'}
    def generate(self, limit, seed): return generate(limit, seed)

    def _dumps(self, path, res, timeout, names):
        tmp = Path(tempfile.mkdtemp(prefix='hdr-', dir='/tmp/claude-0'))
        out = {}
        try:
            for n in names:
                t = tmp / f'{n}.f32'; v = engine.invoke(path, self.runners[n], timeout, t)
                if v['status'] != 'ok' or not t.exists(): return None
                out[n] = load_f32(t)
        finally:
            for f in tmp.glob('*'): f.unlink()
            tmp.rmdir()
        return out

    alt_hash = True
    alt_description = 'decoders whose samples differ from another decoder by exactly one uniform factor of 256/255 are treated as equal; shows what remains once that known normalisation difference is set aside'

    def refine(self, path, results, timeout):
        ok = [n for n, v in results.items() if v['status'] == 'ok']
        results = {k: dict(v) for k, v in results.items()}
        for n in ok: results[n]['alt_sha256'] = results[n]['rgba_sha256']
        if len({results[n]['rgba_sha256'] for n in ok}) <= 1 or len({(results[n]['width'], results[n]['height']) for n in ok}) > 1: return results
        d = self._dumps(path, results, timeout, ok)
        if d is None: return results
        clusters = []  # list of (rep name, members)
        for n in ok:
            for rep, mem in clusters:
                if len(d[rep]) == len(d[n]) and all(close(x, y) for x, y in zip(d[rep], d[n])): mem.append(n); break
            else: clusters.append((n, [n]))
        out = {k: dict(v) for k, v in results.items()}
        ref = d[ok[0]]
        for rep, mem in clusters:
            for n in mem: out[n]['rgba_sha256'] = results[rep]['rgba_sha256']
        for n in ok: out[n]['max_rel_err_vs_ref'] = max_rel(ref, d[n]) if len(ref) == len(d[n]) else None
        # convention-adjusted view: merge clusters that differ by one uniform factor 256/255 (the flipcode m/255 vs Radiance m*2^-8 convention)
        label = {rep: rep for rep, _ in clusters}
        def root(x):
            while label[x] != x: x = label[x]
            return x
        for i, (ra, _) in enumerate(clusters):
            for rb, _ in clusters[i + 1:]:
                a, b = d[ra], d[rb]
                if len(a) != len(b) or any((x == 0.0) != (y == 0.0) for x, y in zip(a, b)): continue
                r = [y / x for x, y in zip(a, b) if x != 0.0 and math.isfinite(x) and math.isfinite(y)]
                if r and max(r) / min(r) < 1 + 1e-5 and (abs(r[0] - 256 / 255) < 1e-5 or abs(r[0] - 255 / 256) < 1e-5): label[root(rb)] = root(ra)
        for rep, mem in clusters:
            for n in mem: out[n]['alt_sha256'] = results[root(rep)]['rgba_sha256']
        return out

    def pixel_difference(self, path, results, timeout, folder):
        ok = [n for n, v in results.items() if v['status'] == 'ok']
        if len(ok) < 2: return ''
        if len({(results[n]['width'], results[n]['height']) for n in ok}) > 1: return 'dimensions'
        d = self._dumps(path, results, timeout, ok)
        if d is None: return 'dump-unavailable'
        reps = []
        for n in ok:
            if not any(len(d[r]) == len(d[n]) and all(close(x, y) for x, y in zip(d[r], d[n])) for r in reps): reps.append(n)
        notes = []
        for r in reps[1:]:
            a, b = d[reps[0]], d[r]; ratios = [y / x for x, y in zip(a, b) if x != 0.0 and y != 0.0 and math.isfinite(x) and math.isfinite(y)]
            w = results[reps[0]]['width']; hgt = results[reps[0]]['height']
            if any(math.isnan(x) or math.isinf(x) for x in list(a) + list(b)): notes.append(f'{reps[0]} vs {r}: non-finite samples')
            elif ratios and min(ratios) > 0 and max(ratios) / min(ratios) < 1 + 1e-5: notes.append(f'{reps[0]} vs {r}: uniform scale x{sum(ratios)/len(ratios):.6f}')
            elif len(a) == len(b) and any(all(close(a[3 * (y * w + x) + c], b[3 * ((hgt - 1 - y) * w + x) + c]) for y in range(hgt) for x in range(w) for c in range(3)) for _ in [0]): notes.append(f'{reps[0]} vs {r}: vertical flip')
            elif len(a) == len(b) and all(close(a[3 * (y * w + x) + c], b[3 * (y * w + (w - 1 - x)) + c]) for y in range(hgt) for x in range(w) for c in range(3)): notes.append(f'{reps[0]} vs {r}: horizontal flip')
            else: notes.append(f'{reps[0]} vs {r}: sample values (max rel err {max_rel(a, b):.3g})' if len(a) == len(b) else f'{reps[0]} vs {r}: length')
        return '; '.join(notes)

    def intro(self):
        return ('Decoded quantity compared: **linear float RGB samples exactly as returned** (no tone mapping, no reorientation, no EXPOSURE/COLORCORR scaling by the harness; per-input hash = SHA-256 of little-endian f32 bit patterns, R,G,B per pixel). '
                'Tolerance rule: when accepting decoders have equal dimensions but different bits, samples are re-dumped and counted equal iff |a-b| <= 1e-6 * max(|a|,|b|) (about 8 f32 ulp; NaN matches NaN), so `accept-different-pixels` means a larger difference. '
                '`image`, `image-nonstrict` and `glycin` are the same implementation (image::HdrDecoder; glycin uses the non-strict constructor) and count as one vote; radiant, hdrldr and stb_image v2.30 are independent. '
                'hdrldr and radiant compute `m/255 * 2^(e-128)`; image and stb compute `m * 2^(e-136)` (the Radiance convention without the +0.5 rounding term); the first form is 256/255 larger.')
    def notes(self):
        return ('Seeds: flat (uncompressed) scanlines, new-style adaptive RLE (runs and literals), old-style RLE, all eight orientation lines, exponent extremes (e=0/1/255), header variants (comments, FORMAT, EXPOSURE, COLORCORR, PRIMARIES, CRLF, missing magic). '
                'Mutations: dimensions-line text, magic, FORMAT/EXPOSURE/COLORCORR/extra attribute lines, header termination, RLE scanline header and control bytes, old-RLE counts, flat first-pixel RLE look-alikes, truncation, trailing data, byte flips.')


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('--inputs', type=int, default=10000); ap.add_argument('--seed', type=int, default=731)
    ap.add_argument('--jobs', type=int, default=4); ap.add_argument('--timeout', type=float, default=4.0); ap.add_argument('--min-checks', type=int, default=24)
    core.run(HdrConfig(), ap.parse_args())
