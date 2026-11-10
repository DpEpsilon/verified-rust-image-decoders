"""Deterministic synthetic XBM generator: valid seeds per syntax variant, then token-level mutations.

Provenance: every seed is built here from a seeded PRNG (no third-party bytes). Syntax references: the X11
XBM description in the libX11 manual (X11 `unsigned char` bit arrays, X10 `short` word arrays, optional
`_x_hot`/`_y_hot` defines, LSB-first bits in each byte, rows padded to whole bytes (X10: whole 16-bit words)).
"""
import random, re, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import textmut

BOUNDS = [0, 1, 2, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 65535, 65536,
          2147483647, 2147483648, 4294967295, 4294967296, -1, -8, 99999999999999999999]
NUMFORMS = ['0x8', '010', '+8', '8u', '8L', '8.0', '1e1', '08', ' 8 ', '8,', '8;', '0b1000', "'8'", '８', '8_0', '-0', '0X8', '8 8']
BASE = dict(name='img', w=13, h=7, hot=None, style='x11', ctype='unsigned char', static='static ', nl='\n', comment='',
            hexfmt='0x%02x', sep=',', wrap=12, order='wh', trailing='};\n', pad='random', indent=' ', brace_nl=False)
AXES = {
    'style': ['x10'], 'ctype': ['char', 'const unsigned char', 'const char', 'unsigned short', 'signed char'],
    'static': ['', 'const static ', 'extern ', 'static  '], 'nl': ['\r\n', '\r'], 'hot': [(0, 0), (3, 2), (12, 6), (-1, -1), (99, 99)],
    'comment': ['/* Created by test */\n', '// line comment\n', '/* a */ /* b */\n', '/* multi\nline */\n'],
    'hexfmt': ['0x%02X', '0X%02x', '0x%x', '%d', '0%o', '0x%04x'], 'sep': [', ', ',\t', ' , ', ',\n'], 'wrap': [1, 4, 100],
    'order': ['hw'], 'trailing': ['}', '};', '} ;\n', '}\n', ';\n'], 'pad': ['zero', 'ones'], 'name': ['', 'a-b', 'my_icon', '1x', 'café', 'x' * 300, 'width'],
    'w': [1, 7, 8, 9, 16, 17, 32, 64, 100], 'h': [1, 2, 16, 33], 'brace_nl': [True], 'indent': ['', '\t', '        '],
}


def build(p, rng):
    w, h = p['w'], p['h']; style = p['style']
    rowb = (w + 15) // 16 * 2 if style == 'x10' else (w + 7) // 8
    data = []
    for _ in range(h):
        row = [rng.randrange(256) for _ in range(rowb)]
        if p['pad'] != 'random' and w % 8:
            mask = (0xff << (w % 8)) & 0xff if rowb == (w + 7) // 8 else 0
            if mask: row[(w - 1) // 8] = (row[(w - 1) // 8] & ~mask & 0xff) | (mask if p['pad'] == 'ones' else 0)
        data += row
    if style == 'x10': vals = [data[i] | data[i + 1] << 8 for i in range(0, len(data), 2)]
    else: vals = data
    hexs = [p['hexfmt'] % v for v in vals]
    nm = p['name'] + '_' if p['name'] else ''
    defs = [f'#define {nm}width {w}', f'#define {nm}height {h}']
    if p['order'] == 'hw': defs.reverse()
    if p['hot'] is not None: defs += [f'#define {nm}x_hot {p["hot"][0]}', f'#define {nm}y_hot {p["hot"][1]}']
    ctype = 'short' if style == 'x10' else p['ctype']
    lines = [d for d in defs]
    head = f'{p["static"]}{ctype} {nm}bits[] = {{'
    body = []
    sep = p['sep']
    for i in range(0, len(hexs), p['wrap']):
        chunk = hexs[i:i + p['wrap']]
        body.append(p['indent'] + sep.join(chunk) + (sep.rstrip(' ') if i + p['wrap'] < len(hexs) else ''))
    text = p['comment'] + p['nl'].join(lines) + p['nl']
    text += head + (p['nl'] if p['brace_nl'] else '') + p['nl'] + p['nl'].join(body) + p['nl'] + p['trailing'].replace('\n', p['nl'])
    return text.encode('utf-8')


def seeds(rng):
    out = []
    def add(label, **kw):
        p = dict(BASE, **kw); out.append((build(p, rng), {'source': 'seed', 'name': label, 'rule': 'seed:' + label, 'params': {k: v for k, v in kw.items()}}))
    add('base')
    for axis, vals in AXES.items():
        for v in vals: add(f'{axis}={v!r}'[:60], **{axis: v})
    combos = list(AXES)
    for i in range(22):
        kw = {a: rng.choice(AXES[a]) for a in rng.sample(combos, rng.randrange(2, 6))}
        add(f'combo{i}', **kw)
    return out


ROLE = re.compile(rb'(#define\s+\S*?_?(width|height|x_hot|y_hot)\s+)(\S+)')


def roles(b):
    spans = {}
    for m in ROLE.finditer(b): spans.setdefault(m.group(2).decode(), (m.start(3), m.end(3)))
    return spans


def data_span(b):
    s = b.find(b'{'); e = b.rfind(b'}')
    return (s + 1, e) if 0 <= s < e else None


def mutations(b, rng):
    cats = []
    sp = roles(b)
    num = []
    for role, (s, e) in sp.items():
        vals = BOUNDS if role in ('width', 'height') else rng.sample(BOUNDS, 10)
        for v in vals: num.append((b[:s] + str(v).encode() + b[e:], {'rule': f'num-boundary:{role}', 'field': role, 'value': v}))
        for f in rng.sample(NUMFORMS, 6): num.append((b[:s] + f.encode() + b[e:], {'rule': f'num-format:{role}', 'field': role, 'value': f}))
        num.append((b[:s] + b[e:], {'rule': f'num-missing:{role}', 'field': role, 'value': 'deleted'}))
    rng.shuffle(num); cats.append(num[:56])
    ds = data_span(b); dm = []
    if ds:
        s, e = ds; body = b[s:e]; toks = [m for m in re.finditer(rb'[0-9A-Za-z_+\-]+', body)]
        n = len(toks)
        for k in (1, 2, 3, 7, 13):
            if n > k:
                dm.append((b[:s] + body[:toks[n - k].start()] + b[e:], {'rule': 'data-short', 'field': 'tokens', 'value': -k}))
        for k in (1, 2, 3, 12, 50):
            dm.append((b[:e] + b', ' + b', '.join([b'0xff'] * k) + b[e:], {'rule': 'data-extra', 'field': 'tokens', 'value': k}))
        dm.append((b[:s] + b[e:], {'rule': 'data-empty', 'field': 'tokens', 'value': 0}))
        for t in (b'0x100', b'0x1ff', b'256', b'-1', b'0xZZ', b'0x', b'0xfffff', b'1.5', b'"a"', b"'a'", b'0xA', b'0xabc', b'0', b'07', b'08', b'0x0g', b'a', b'0x-1'):
            if n:
                k = rng.randrange(n); dm.append((b[:s] + body[:toks[k].start()] + t + body[toks[k].end():] + b[e:], {'rule': 'data-token', 'field': 'value', 'value': t.decode()}))
        for name, f in (('no-commas', lambda x: x.replace(b',', b' ')), ('double-commas', lambda x: x.replace(b',', b',,')),
                        ('lead-comma', lambda x: b',' + x), ('trail-comma', lambda x: x.rstrip() + b','),
                        ('no-last-comma', lambda x: x.rstrip().rstrip(b',')), ('semicolons', lambda x: x.replace(b',', b';')),
                        ('uppercase-hex', lambda x: x.upper()), ('comment-inside', lambda x: x[:len(x) // 2] + b'/* c */' + x[len(x) // 2:])):
            dm.append((b[:s] + f(body) + b[e:], {'rule': 'data-' + name, 'field': 'array', 'value': name}))
    cats.append(dm)
    st = []
    subs = [(b'static ', b'', 'no-static'), (b'[]', b'[1]', 'array-size-1'), (b'[]', b'[9999]', 'array-size-big'), (b'[]', b'', 'no-brackets'), (b'{', b'', 'no-open-brace'),
            (b'}', b'', 'no-close-brace'), (b'#define', b'# define', 'hash-space'), (b'#define', b'#DEFINE', 'define-upper'), (b'#define', b'#defin', 'define-trunc'),
            (b'_width', b'_Width', 'suffix-case'), (b'_width', b'_w', 'suffix-short'), (b'_bits', b'_data', 'bits-rename'), (b'_bits', b'', 'bits-noname'),
            (b'_height', b'_len', 'height-rename'), (b'_x_hot', b'_xhot', 'hot-rename'), (b'=', b'', 'no-equals'), (b' char', b' int', 'type-int'),
            (b'unsigned', b'', 'no-unsigned'), (b';', b'', 'no-semicolon'), (b'#define', b'#undef', 'undef'), (b'#define', b'#define x 1\n#define', 'extra-define'),
            (b' ', b'  ', 'double-space'), (b' ', b'\t', 'tab-space')]
    for old, new, nm in subs:
        i = b.find(old)
        if i >= 0: st.append((b[:i] + new + b[i + len(old):], {'rule': 'syntax:' + nm, 'field': 'text', 'value': nm}))
    st += [(b'#ifndef X\n' + b + b'\n#endif\n', {'rule': 'syntax:ifndef', 'field': 'text', 'value': 'ifndef'}),
           (b + b, {'rule': 'syntax:doubled', 'field': 'text', 'value': 'file twice'}),
           (b + b'\n#define extra_width 3\n', {'rule': 'syntax:trailing-define', 'field': 'text', 'value': 'define after data'}),
           (b + b'garbage', {'rule': 'syntax:trailing-garbage', 'field': 'text', 'value': 'garbage'}),
           (b + b'\x00', {'rule': 'syntax:trailing-nul', 'field': 'text', 'value': 'nul'}),
           (b'\n\n' + b, {'rule': 'syntax:leading-blank', 'field': 'text', 'value': 'blank lines'})]
    cats.append(st)
    cats.append(list(textmut.truncations(b, rng, 12)))
    cats.append(list(textmut.token_ops(b, rng, 16)))
    cats.append(list(textmut.byte_flips(b, rng, 8)))
    cats.append(list(textmut.bad_chars(b, rng, 8)))
    cats.append(list(textmut.long_lines(b, rng, 3)))
    cats.append(list(textmut.newline_styles(b)))
    for c in cats[1:2] + cats[2:3]: rng.shuffle(c)
    # round-robin across categories so a bounded run still covers all of them
    merged = []
    for i in range(max(len(c) for c in cats)):
        for c in cats:
            if i < len(c): merged.append(c[i])
    return iter(merged)


def generate(limit, seed):
    rng = random.Random(seed)
    base = seeds(rng)
    streams = [(m['name'], mutations(bb, random.Random(f'{seed}:{m["name"]}'))) for bb, m in base]
    out = textmut.interleave(streams, limit, base[:limit])
    return out, len(base)
