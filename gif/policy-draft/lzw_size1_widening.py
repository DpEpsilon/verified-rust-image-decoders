#!/usr/bin/env python3
"""LZW minimum code size 1: which code-width rule do gdk-pixbuf, image/gif (weezl) and the oracle implement?

  python3 -I lzw_size1_widening.py [--out DIR] [--bin DIR]

Builds synthetic GIFs (one image, global colour table, one LZW stream) with a parameterised encoder,
runs them through gdk-pixbuf 2.42.12 (animation path and forced loader, whole buffer), the `image`
0.25.10 runner (gif 0.14.2, weezl 0.1.12) and policy_oracle.py (RecommendedBeta; Strict for m >= 2),
and compares every output with the intended pixels and with three reference decoders that differ only
in when they widen the code:

  gdk    G10.2 / gdk-pixbuf lzw.c:204: after a table entry is added, widen when table size == 2^width.
  std    giflib: after every code read (also the first after a Clear) the counter next = first_free + j
         is incremented and the code widens when next > 2^width.
  weezl  decode.rs:1097: when a table entry is derived (every code but the first after a Clear),
         widen when the index of the new entry >= 2^width - 1.

Encoder width rules (E = index of the table entry just assigned, w = current width):
  std    widen when E >= 2^w                 (the entry E needs w+1 bits; every GIF encoder, Appendix F)
  early  widen when E + 1 >= 2^w             (one code earlier, TIFF style)
  late   widen when E >= 2^w + 1             (one code later)
  weezl  std, except that at m = 1 the first widening (E = 4) is deferred to E = 5
  lit-X  only literal codes (no table entry is named), widths as decoder X reads them (X = std, weezl, gdk)
Writes only under --out (default: scratchpad), never into the repository.
"""
import argparse
import hashlib
import json
import random
import subprocess
import sys
import tempfile
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
import policy_oracle as PO  # noqa: E402

PALETTES = {1: [(10, 20, 30), (200, 210, 220)]}


def palette(m):
    n = 1 << max(m, 1)
    if m in PALETTES:
        return PALETTES[m]
    return [((37 * i) & 255, (91 * i + 7) & 255, (151 * i + 3) & 255) for i in range(n)]


# ---------------------------------------------------------------- container

def pack_codes(codes):
    """codes: list of (value, width). LSB-first bit packing."""
    acc = nbits = 0
    out = bytearray()
    for v, w in codes:
        acc |= v << nbits
        nbits += w
        while nbits >= 8:
            out.append(acc & 255)
            acc >>= 8
            nbits -= 8
    if nbits:
        out.append(acc & 255)
    return bytes(out)


def make_gif(m, w, h, data):
    pal = palette(m)
    n = len(pal)
    gct = b''.join(bytes(c) for c in pal)
    field = n.bit_length() - 2          # 2^(field+1) colours
    b = bytearray(b'GIF89a' + w.to_bytes(2, 'little') + h.to_bytes(2, 'little') + bytes([0x80 | field << 4 | field, 0, 0]))
    b += gct
    b += b'\x2c\0\0\0\0' + w.to_bytes(2, 'little') + h.to_bytes(2, 'little') + b'\0'
    b.append(m)
    for i in range(0, len(data), 255):
        chunk = data[i:i + 255]
        b.append(len(chunk))
        b += chunk
    b += b'\0;'
    return bytes(b)


def expected_rgba(m, pixels):
    pal = palette(m)
    return b''.join(bytes(pal[p]) + b'\xff' for p in pixels)


# ---------------------------------------------------------------- encoders

def widen(rule, m, E, w):
    if w >= 12:
        return False
    if rule == 'std':
        return E >= (1 << w)
    if rule == 'early':
        return E + 1 >= (1 << w)
    if rule == 'late':
        return E >= (1 << w) + 1
    if rule == 'weezl':
        return E >= 5 if (m == 1 and w == 2) else E >= (1 << w)
    raise ValueError(rule)


def lit_widths(rule, m, J):
    """Widths with which decoder `rule` reads codes 1..J after a Clear when no code names a table entry."""
    clear = 1 << m
    w, size, running, out = m + 1, clear + 2, clear + 2, []
    for j in range(1, J + 1):
        out.append(w)
        if rule == 'std':
            running += 1
            if running > (1 << w) and w < 12:
                w += 1
        elif rule == 'gdk':
            if j >= 2:
                size += 1
                if size == (1 << w) and w < 12:
                    w += 1
        elif rule == 'weezl':
            if j >= 2:
                E, size = size, size + 1
                if E >= (1 << w) - 1 and w < 12:
                    w += 1
    return out


def encode(m, pixels, rule):
    """GIF LZW encoder. Returns (packed image data, ok); ok is False when the schedule forced a code
    that does not fit its width (the stream is then corrupt by construction)."""
    clear, eoi = 1 << m, (1 << m) + 1
    if rule.startswith('lit-'):              # literal codes only, widths from a decoder schedule
        ws = lit_widths(rule[4:], m, len(pixels) + 1)
        return pack_codes([(clear, m + 1)] + [(p, ws[i]) for i, p in enumerate(pixels)] + [(eoi, ws[len(pixels)])]), True
    codes, ok = [], True
    w, nxt = m + 1, clear + 2
    table = {}
    codes.append((clear, w))
    prefix = None

    def emit(c):
        nonlocal ok
        if c >= (1 << w):
            ok = False
        codes.append((c, w))
    for p in pixels:
        if prefix is None:
            prefix = p
            continue
        key = (prefix, p)
        if key in table:
            prefix = table[key]
            continue
        emit(prefix)
        E = nxt
        table[key] = E
        nxt += 1
        if widen(rule, m, E, w):
            w += 1
        if E == 4095:                        # table full: Clear (the decoders are one entry behind)
            codes.append((clear, w))
            table, nxt, w = {}, clear + 2, m + 1
        prefix = p
    emit(prefix)
    if widen(rule, m, nxt, w):               # giflib also applies the width test after the last code, as if E = nxt were assigned
        w += 1
    emit(eoi)
    return pack_codes(codes), ok


# ---------------------------------------------------------------- reference decoders

def bits_of(data):
    n = len(data) * 8
    return n


def model_decode(m, data, area, rule):
    """Returns (indices, why): why is 'complete' or the reason the stream ended before `area` indices."""
    clear, eoi = 1 << m, (1 << m) + 1
    nbits, pos = len(data) * 8, 0
    width, table, prev, j = m + 1, [(i,) for i in range(clear)] + [(), ()], None, 0
    running = clear + 2
    out = []
    while len(out) < area:
        if pos + width > nbits:
            return out, 'data-ended'
        code = 0
        for k in range(width):
            i = pos + k
            code |= ((data[i >> 3] >> (i & 7)) & 1) << k
        pos += width
        if code == clear:
            width, table, prev, j, running = m + 1, table[:clear + 2], None, 0, clear + 2
            continue
        if code == eoi:
            return out, 'eoi-early'
        if code < len(table):
            s = table[code]
        elif code == len(table) and prev is not None and len(table) < 4096:
            s = prev + prev[:1]
        else:
            return out, f'invalid-code-{code}'
        j += 1
        if rule == 'std':
            if prev is not None and len(table) < 4096:
                table.append(prev + s[:1])
            if running < 4097:
                running += 1
                if running > (1 << width) and width < 12:
                    width += 1
        else:
            if prev is not None and len(table) < 4096:
                table.append(prev + s[:1])
                E = len(table) - 1
                if rule == 'gdk' and len(table) == (1 << width) and width < 12:
                    width += 1
                if rule == 'weezl' and E >= (1 << width) - 1 and width < 12:
                    width += 1
        prev = s
        out.extend(s)
    return out[:area], 'complete'


# ---------------------------------------------------------------- runners

def limits():
    import resource
    resource.setrlimit(resource.RLIMIT_AS, (3 << 30, 3 << 30))


def run_runner(cmd, dump):
    try:
        p = subprocess.run(cmd, capture_output=True, timeout=60, preexec_fn=limits)
    except subprocess.TimeoutExpired:
        return {'status': 'timeout'}
    for line in reversed(p.stdout.decode('utf-8', 'replace').strip().splitlines()):
        if line.startswith('{'):
            try:
                j = json.loads(line)
            except ValueError:
                break
            if j.get('status') == 'ok':
                return {'status': 'ok', 'rgba': Path(dump).read_bytes() if Path(dump).exists() else None,
                        'tail_error': j.get('tail_error')}
            return {'status': 'error', 'error': (j.get('message') or j.get('error_class') or '')[:80]}
    return {'status': 'crash'}


def incumbents(path, bindir, scratch):
    d = scratch / 'dump.rgba'
    out = {}
    for name, cmd in (('gdk_anim', [str(bindir / 'gdk-anim'), 'gif', 'anim']),
                      ('gdk_forced', [str(bindir / 'gdk-anim'), 'gif', 'forced-whole']),
                      ('image', [str(bindir / 'rust-image'), 'gif:image'])):
        if d.exists():
            d.unlink()
        out[name] = run_runner(cmd + [str(path), str(d)], d)
    return out


def oracle(b, m):
    P = PO.RECOMMENDED_BETA if m == 1 else PO.STRICT
    r = PO.decode_still(b, P)
    return {'status': 'error', 'error': r['error']} if 'error' in r else {'status': 'ok', 'rgba': r['rgba']}


# ---------------------------------------------------------------- experiment

def classify(res, want, want_partial_rgba=None):
    """'ok' (== intended), 'wrong:<n pixels differ>', or 'error:<msg>'."""
    if res['status'] != 'ok' or res.get('rgba') is None:
        return 'error:' + (res.get('error') or res['status'])[:40]
    got = res['rgba']
    if got == want:
        return 'ok'
    if len(got) != len(want):
        return f'wrong:size{len(got)}'
    nd = sum(1 for i in range(0, len(want), 4) if got[i:i + 4] != want[i:i + 4])
    return f'wrong:{nd}px'


def patterns(m, n, rng):
    k = 1 << m
    yield 'random', [rng.randrange(k) for _ in range(n)]
    yield 'zeros', [0] * n
    yield 'ones', [k - 1] * n
    yield 'stripes3', [(i // 3) % k for i in range(n)]
    yield 'sparse', [rng.randrange(k) if rng.random() < 0.1 else 0 for _ in range(n)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=None)
    ap.add_argument('--bin', default='/tmp/claude-0/leniency-gifpng-bin')
    ap.add_argument('--json', default=None)
    a = ap.parse_args()
    scratch = Path(a.out) if a.out else Path(tempfile.mkdtemp(prefix='lzw1-', dir='/tmp/claude-0'))
    scratch.mkdir(parents=True, exist_ok=True)
    bindir = Path(a.bin)
    rng = random.Random(20261114)
    rows = []

    def one(m, w, h, label, pixels, rule):
        data, enc_ok = encode(m, pixels, rule)
        b = make_gif(m, w, h, data)
        path = scratch / 'case.gif'
        path.write_bytes(b)
        want = expected_rgba(m, pixels)
        res = incumbents(path, bindir, scratch)
        res['oracle'] = oracle(b, m)
        cls = {k: classify(v, want) for k, v in res.items()}
        # what each reference decoder yields
        for mod in ('gdk', 'std', 'weezl'):
            idx, why = model_decode(m, data, len(pixels), mod)
            cls['model_' + mod] = 'ok' if idx == pixels and why == 'complete' else f'wrong:{why}'
            if mod == 'gdk':
                res['_model_gdk'] = (idx, why)
            if mod == 'weezl':
                res['_model_weezl'] = (idx, why)
        # model-vs-incumbent: does the incumbent's pixel output equal the model's (also when wrong)?
        pal = palette(m)

        def rgba_of(idx, area):
            return b''.join((bytes(pal[i]) + b'\xff') if i < len(pal) else b'\0\0\0\0' for i in idx) + b'\0\0\0\0' * (area - len(idx))
        same = {}
        for inc, mod in (('gdk_anim', 'gdk'), ('gdk_forced', 'gdk'), ('oracle', 'gdk'), ('image', 'weezl')):
            r = res[inc]
            idx, why = res['_model_' + mod]
            if why != 'complete':
                same[inc] = 'model-incomplete'
            elif r['status'] != 'ok' or r.get('rgba') is None:
                same[inc] = 'inc-error'
            else:
                same[inc] = 'equal' if r['rgba'] == rgba_of(idx, len(pixels)) else 'differ'
        def pair(x, y):
            ox, oy = res[x]['status'] == 'ok', res[y]['status'] == 'ok'
            if ox and oy:
                return 'same-pixels' if res[x]['rgba'] == res[y]['rgba'] else 'different-pixels'
            return 'both-reject' if not ox and not oy else 'one-rejects'
        agree = {'gdk_vs_image': pair('gdk_anim', 'image'), 'gdk_vs_gdk_forced': pair('gdk_anim', 'gdk_forced'),
                 'gdk_vs_oracle': pair('gdk_anim', 'oracle'), 'image_vs_oracle': pair('image', 'oracle')}
        rows.append({'agree': agree, 'm': m, 'size': f'{w}x{h}', 'pattern': label, 'rule': rule, 'enc_ok': enc_ok, 'bytes': len(data), 'cls': cls,
                     'model_match': same, 'model_why': {k: res['_model_' + k][1] for k in ('gdk', 'weezl')}})

    # A: long streams (the width must change many times)
    for m in (1, 2, 3, 8):
        for (w, h) in ((64, 64), (256, 256), (60, 60)):
            for label, pixels in patterns(m, w * h, rng):
                if (w, h) == (256, 256) and label not in ('random', 'sparse'):
                    continue
                rules = ['lit-std', 'lit-weezl', 'lit-gdk'] if (w, h) == (60, 60) else ['std', 'early', 'late', 'weezl']
                if (w, h) == (60, 60) and label not in ('random', 'sparse'):
                    continue
                for rule in rules:
                    one(m, w, h, label, pixels, rule)
    # B: short streams (where do they first diverge?)
    for m in (1, 2):
        for n in (1, 2, 3, 4, 5, 6, 8, 12, 16):
            pats = [('zeros', [0] * n), ('ones', [(1 << m) - 1] * n)] + [(f'rand{i}', [rng.randrange(1 << m) for _ in range(n)]) for i in range(6)]
            for label, pixels in pats:
                for rule in (['std', 'lit-std', 'lit-weezl', 'lit-gdk'] if m == 1 else ['std']):
                    one(m, n, 1, label, pixels, rule)
    if a.json:
        Path(a.json).write_text(json.dumps(rows, indent=1) + '\n')
    summarise(rows)


def summarise(rows):
    from collections import Counter, defaultdict
    decs = ['gdk_anim', 'gdk_forced', 'image', 'oracle', 'model_gdk', 'model_std', 'model_weezl']

    def kind(c):
        return 'ok' if c == 'ok' else c.split(':')[0]
    for part, sel in (('long (>= 3600 px)', lambda r: r['size'] in ('64x64', '256x256', '60x60')), ('short (<= 16 px)', lambda r: r['size'] not in ('64x64', '256x256', '60x60'))):
        print('==', part)
        agg = defaultdict(lambda: defaultdict(Counter))
        for r in rows:
            if sel(r) and r['enc_ok']:
                for d in decs:
                    agg[(r['m'], r['rule'])][d][kind(r['cls'][d])] += 1
        for (m, rule), dd in sorted(agg.items()):
            print(f'm={m} enc={rule:6s} n={sum(dd["gdk_anim"].values()):3d} ' + '  '.join(f'{d}={dict(dd[d])}' for d in decs))
    print('== pairwise agreement on encoder-correct (std) streams: (m, long/short, pair) -> counts')
    for m in (1, 2):
        for part, sel in (('long', lambda r: r['size'] in ('64x64', '256x256', '60x60')), ('short', lambda r: r['size'] not in ('64x64', '256x256', '60x60'))):
            for pr in ('gdk_vs_image', 'gdk_vs_gdk_forced', 'gdk_vs_oracle', 'image_vs_oracle'):
                c = Counter(r['agree'][pr] for r in rows if r['m'] == m and r['rule'] == 'std' and sel(r))
                print(m, part, pr, dict(c))
    print('== m=1 std, short: fraction of streams on which gdk, image and oracle all give the intended pixels, by pixel count')
    by = defaultdict(Counter)
    for r in rows:
        if r['m'] == 1 and r['rule'] == 'std' and r['size'] not in ('64x64', '256x256', '60x60'):
            n = int(r['size'].split('x')[0])
            c = r['cls']
            by[n][(c['gdk_anim'] == 'ok', c['image'] == 'ok', c['oracle'] == 'ok')] += 1
    for n in sorted(by):
        print(n, {('gdk' if k[0] else '-') + ('+image' if k[1] else '') + ('+oracle' if k[2] else ''): v for k, v in by[n].items()})
    # incumbent output == reference-decoder output (also when both are "wrong")
    print('== incumbent pixels equal to reference decoder pixels (where the incumbent produced an image)')
    tot = defaultdict(Counter)
    for r in rows:
        if not r['enc_ok']:
            continue
        for inc, v in r['model_match'].items():
            tot[(r['m'], inc)][v] += 1
    for k, v in sorted(tot.items()):
        print(k, dict(v))


if __name__ == '__main__':
    main()
