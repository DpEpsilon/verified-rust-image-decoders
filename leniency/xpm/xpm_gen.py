"""Deterministic synthetic XPM generator: valid seeds per syntax variant, then model-level and token-level mutations.

Provenance: every seed is built here from a seeded PRNG (no third-party bytes). Syntax reference: the XPM 3 format
description (C array of strings: `"w h ncolors cpp [xhot yhot] [XPMEXT]"`, ncolors lines `code key value [key value]...`
with keys c/s/m/g/g4, h*w pixel strings of cpp characters, optional XPMEXT blocks) and the line-oriented XPM 2 layout
(`! XPM2` first line, same content without C quoting).
"""
import copy, random, sys
from pathlib import Path
sys.path.append(str(Path(__file__).resolve().parent.parent / 'xbm'))
import textmut

CHARS = '.#abcXO+@ox%&*=-;:>,<1234567890qwertyuiopasdfghjklzxcvbnm!~^()_[]{}|?/'
NAMED = ['red', 'blue', 'green', 'white', 'black', 'gray50', 'DarkSlateGray', 'dark slate gray', 'LightGoldenrod', 'NavyBlue', 'navy', 'gray', 'grey',
         'RED', 'Red', 'SeaGreen3', 'salmon', 'CornflowerBlue', 'tomato', 'snow', 'orange', 'rebeccapurple', 'crimson', 'indigo', 'WebGreen', 'light slate blue', 'gray100', 'grey0', 'gray101']
HEXLEN = {1: 'hex1', 2: 'hex2', 3: 'hex3', 4: 'hex4'}
COLVALS = ['#', '#12', '#1234', '#12345', '#1234567', '#GGGGGG', '#ff00', 'none', 'None', 'NONE', 'transparent', 'red ', ' red', 'nosuchcolor', 'rgb:ff/00/00',
           'rgb:f/0/0', 'rgbi:1.0/0/0', '%ff0000', '#+f00', '#ffffffffffff1', '', 'None#', 'gray', 'gray0', 'gray100', 'gray101', '#FFF FFF', '#ABCDEF', '#abcdef', '#ABC', '#abcd1234abcd']
KEYS = ['x', 'C', 'm', 'g', 'g4', 's', 'G4', 'cc', 'c ', '', 'M', 'g5']
HEADN = [0, 1, 2, 3, 4, 5, 8, 9, 16, 255, 256, 257, 65535, 65536, 2147483647, 2147483648, 4294967295, 4294967296, -1, 'x', '', '1.5', '0x10', '+3', '007', '99999999999999999999']
DECLS = ['static char * {n}[] = {{', 'static const char *{n}[] = {{', 'static char **{n} = {{', 'char *{n}[]={{', 'static unsigned char * {n}[] = {{',
         'const char * const {n}[] = {{', 'static char * {n} [ ] = {{', 'static char *{n}[] =\n{{', 'extern char *{n}[] = {{', 'static char *{n}[5000] = {{', 'static char *{n}[] = {{ /* x */']
MAGICS = ['/* XPM */', '/*XPM*/', '/* xpm */', '/* XPM2 */', '/*  XPM  */', '', '/* XPM */ ', '// XPM', '/* XPM 3 */']


def hexcol(rng, digits):
    return '#' + ''.join('%0*X' % (digits, rng.randrange(16 ** digits)) for _ in range(3))


def make(p, rng):
    cpp = p['cpp']; w, h, n = p['w'], p['h'], p['n']; pool = p['chars']
    codes = []
    while len(codes) < n:
        c = ''.join(rng.choice(pool) for _ in range(cpp))
        if c not in codes and not any(c.startswith(x) or x.startswith(c) for x in codes[:0]): codes.append(c)
    cols = []
    for i, code in enumerate(codes):
        kind = p['kinds'][i % len(p['kinds'])]
        val = None if kind == 'None' else kind
        if kind.startswith('hex'): val = hexcol(rng, int(kind[3]))
        elif kind == 'named': val = rng.choice(NAMED[:20])
        elif kind == 'None': val = 'None'
        elif kind == 'Nonelower': val = 'none'
        elif kind == 'gray': val = rng.choice(['gray50', 'grey25', 'gray', 'Gray75'])
        elif kind == 'cssnamed': val = rng.choice(['rebeccapurple', 'crimson', 'indigo', 'WebGreen'])
        spec = []
        for key in p['keys']:
            v = val
            if key == 'm': v = rng.choice(['black', 'white'])
            elif key == 's': v = 'sym%d' % i
            elif key in ('g', 'g4'): v = rng.choice(['gray10', 'gray90', '#808080'])
            spec.append((key, v))
        cols.append([code, spec])
    rows = [''.join(codes[rng.randrange(n)] for _ in range(w)) for _ in range(h)]
    header = [str(w), str(h), str(n), str(cpp)]
    if p['hot'] is not None: header += [str(p['hot'][0]), str(p['hot'][1])]
    ext = None
    if p['ext']:
        header.append('XPMEXT')
        ext = ['XPMEXT comment', 'a comment line', 'XPMEXT ext1', 'ext1 line one', 'ext1 line two', 'XPMENDEXT']
    return dict(version=p['version'], name=p['name'], magic=p['magic'], decl=p['decl'], header=header, cols=cols, rows=rows, ext=ext, nl=p['nl'],
                trailing=p['trailing'], sep=p['sep'], comment=p['comment'], lastcomma=False, rowsep=',', colsep=' ')


def render(m):
    nl = m['nl']
    cl = [m['colsep'].join([c[0]] + [x for kv in c[1] for x in kv if x is not None]) if True else '' for c in m['cols']]
    strings = [m['sep'].join(m['header'])] + cl + list(m['rows']) + (list(m['ext']) if m['ext'] else [])
    if m['version'] == 'xpm2':
        return (('! XPM2' if m['magic'] is not None else '') + nl + nl.join(strings) + nl).encode('utf-8', 'surrogatepass')
    out = (m['magic'] + nl if m['magic'] else '') + m['decl'].format(n=m['name']).replace('\n', nl) + nl
    body = []
    for i, s in enumerate(strings):
        last = i == len(strings) - 1
        body.append('"' + s + '"' + ('' if last and not m['lastcomma'] else m['rowsep']) + (' ' + m['comment'] if m['comment'] else ''))
    out += nl.join(body) + nl + m['trailing'].replace('\n', nl)
    return out.encode('utf-8', 'surrogatepass')


BASE = dict(version='xpm3', name='img', magic='/* XPM */', decl=DECLS[0], w=5, h=4, n=3, cpp=1, chars=CHARS[:6], kinds=['hex2', 'named', 'None'], keys=['c'],
            hot=None, ext=False, nl='\n', trailing='};\n', sep=' ', comment='')
AXES = {
    'version': ['xpm2'], 'cpp': [2, 3, 4], 'keys': [['s', 'c'], ['c', 's'], ['m', 'c'], ['c', 'm', 'g'], ['g4', 'c'], ['g'], ['m'], ['g4'], ['s'], ['c', 'g4', 'g', 'm']],
    'kinds': [['hex1'], ['hex2'], ['hex3'], ['hex4'], ['named'], ['None'], ['Nonelower', 'hex2'], ['gray'], ['cssnamed'], ['hex1', 'hex2', 'hex3', 'hex4', 'named', 'None']],
    'hot': [(0, 0), (3, 2), (99, 99), (-1, -1)], 'ext': [True], 'nl': ['\r\n', '\r'], 'comment': ['/* c */', '// c'],
    'decl': DECLS[1:], 'magic': MAGICS[1:], 'trailing': ['}', '};\n/* end */\n', ';', '};', '};\ngarbage\n', '}\n;\n'], 'sep': ['  ', '\t'],
    'w': [1, 2, 8, 16, 40], 'h': [1, 2, 8, 16], 'n': [1, 2, 16, 64, 300], 'name': ['', 'a-b', '1x', 'x' * 300],
    'chars': [' ' + CHARS[1:6], '#,/*;:', '\\.' + CHARS[:4]],
}


def seeds(rng):
    out = []

    def add(label, **kw):
        p = dict(BASE, **kw)
        if p['n'] > 90 and p['cpp'] == 1: p['cpp'] = 2
        if p['n'] > len(p['chars']) ** p['cpp']: p['cpp'] += 1
        if p['n'] > len(CHARS) and len(p['chars']) < len(CHARS): p['chars'] = CHARS
        if p['n'] > len(p['chars']) and p['cpp'] == 1: p['cpp'] = 2
        m = make(p, rng)
        m['version'] = p['version']
        out.append((render(m), {'source': 'seed', 'name': label, 'rule': 'seed:' + label, 'params': {k: v for k, v in kw.items()}}, m))
    add('base')
    for axis, vals in AXES.items():
        for v in vals:
            kw = {axis: v}
            if axis in ('w', 'h') or axis == 'n': kw.setdefault('chars', CHARS[:max(6, 1)])
            if axis == 'n' and v > 6: kw['chars'] = CHARS; kw['cpp'] = 1 if v <= 60 else 2
            add(f'{axis}={v!r}'[:60], **kw)
    for i in range(24):
        kw = {a: rng.choice(AXES[a]) for a in rng.sample([k for k in AXES if k not in ('n', 'chars', 'version')], rng.randrange(2, 5))}
        if rng.random() < .15: kw['version'] = 'xpm2'
        add(f'combo{i}', **kw)
    return out


def mutations(model, b, rng):
    m0 = model; cats = []
    def R(mm): return render(mm)
    def edit(f):
        mm = copy.deepcopy(m0); f(mm); return mm
    hd = []
    for idx, nm in enumerate(['width', 'height', 'ncolors', 'cpp']):
        for v in rng.sample(HEADN, 14):
            hd.append((R(edit(lambda mm, idx=idx, v=v: mm['header'].__setitem__(idx, str(v)))), {'rule': f'hdr-boundary:{nm}', 'field': nm, 'value': v}))
        hd.append((R(edit(lambda mm, idx=idx: mm['header'].pop(idx))), {'rule': f'hdr-missing:{nm}', 'field': nm, 'value': 'deleted'}))
    for nm, vals in (('hot_x', HEADN), ('hot_y', HEADN)):
        for v in rng.sample(vals, 6):
            def f(mm, v=v, nm=nm):
                if len(mm['header']) < 6: mm['header'][4:6] = ['1', '1']
                mm['header'][4 if nm == 'hot_x' else 5] = str(v)
            hd.append((R(edit(f)), {'rule': f'hdr-boundary:{nm}', 'field': nm, 'value': v}))
    for k, extra in enumerate(['7', 'XPMEXT', '1', '1 2 3', 'c', '0 0 XPMEXT x']):
        hd.append((R(edit(lambda mm, extra=extra: mm['header'].append(extra))), {'rule': 'hdr-extra-token', 'field': 'header', 'value': extra}))
    for d in (-1, 1, -2, 2):
        hd.append((R(edit(lambda mm, d=d: mm['header'].__setitem__(2, str(int(mm['header'][2]) + d)))), {'rule': 'hdr-ncolors-off-by', 'field': 'ncolors', 'value': d}))
    rng.shuffle(hd); cats.append(hd[:34])
    cm = []
    ci = len(m0['cols']) // 2
    for key in rng.sample(KEYS, 9):
        cm.append((R(edit(lambda mm, key=key: mm['cols'][ci][1].__setitem__(0, (key, mm['cols'][ci][1][0][1])))), {'rule': 'color-key', 'field': 'key', 'value': key}))
    for val in rng.sample(COLVALS, 14):
        cm.append((R(edit(lambda mm, val=val: mm['cols'][ci][1].__setitem__(0, (mm['cols'][ci][1][0][0], val)))), {'rule': 'color-value', 'field': 'value', 'value': val}))
    for val in rng.sample(NAMED, 8):
        cm.append((R(edit(lambda mm, val=val: mm['cols'][ci][1].__setitem__(0, (mm['cols'][ci][1][0][0], val)))), {'rule': 'color-named', 'field': 'value', 'value': val}))
    for nm, f in (('no-value', lambda mm: mm['cols'][ci][1].__setitem__(0, (mm['cols'][ci][1][0][0], None))),
                  ('only-code', lambda mm: mm['cols'].__setitem__(ci, [mm['cols'][ci][0], []])),
                  ('drop-color-line', lambda mm: mm['cols'].pop(ci)),
                  ('dup-color-line', lambda mm: mm['cols'].insert(ci, copy.deepcopy(mm['cols'][ci]))),
                  ('dup-code-new-color', lambda mm: mm['cols'].insert(0, [mm['cols'][ci][0], [('c', '#00FF00')]])),
                  ('swap-keys-order', lambda mm: mm['cols'][ci][1].reverse()),
                  ('extra-key-pair', lambda mm: mm['cols'][ci][1].append(('m', 'white'))),
                  ('key-after-none', lambda mm: mm['cols'][ci][1].extend([('c', 'None'), ('c', 'red')])),
                  ('tab-sep', lambda mm: mm.__setitem__('colsep', '\t')), ('multi-space', lambda mm: mm.__setitem__('colsep', '   ')),
                  ('code-shorter', lambda mm: mm['cols'][ci].__setitem__(0, mm['cols'][ci][0][:-1])),
                  ('code-longer', lambda mm: mm['cols'][ci].__setitem__(0, mm['cols'][ci][0] + 'z')),
                  ('code-space', lambda mm: mm['cols'][ci].__setitem__(0, ' ' * len(mm['cols'][ci][0]))),
                  ('code-hash', lambda mm: mm['cols'][ci].__setitem__(0, '#' * len(mm['cols'][ci][0]))),
                  ('all-none', lambda mm: [c.__setitem__(1, [('c', 'None')]) for c in mm['cols']]),
                  ('mono-only', lambda mm: [c.__setitem__(1, [('m', 'black')]) for c in mm['cols']]),
                  ('sym-only', lambda mm: [c.__setitem__(1, [('s', 'x')]) for c in mm['cols']])):
        try: cm.append((R(edit(f)), {'rule': 'color-line:' + nm, 'field': 'color', 'value': nm}))
        except (IndexError, TypeError): pass
    cats.append(cm)
    px = []
    ri = len(m0['rows']) // 2; cpp = len(m0['cols'][0][0]) if m0['cols'] else 1
    for nm, f in (('row-short-1', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][:-cpp] if cpp else '')), ('row-short-char', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][:-1])),
                  ('row-long-1', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri] + mm['rows'][ri][:cpp])), ('row-long-char', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri] + 'z')),
                  ('row-empty', lambda mm: mm['rows'].__setitem__(ri, '')), ('row-unknown-code', lambda mm: mm['rows'].__setitem__(ri, 'Z' * len(mm['rows'][ri]))),
                  ('row-one-unknown', lambda mm: mm['rows'].__setitem__(ri, 'Z' + mm['rows'][ri][1:])), ('row-trailing-space', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri] + ' ')),
                  ('row-tab', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][:-1] + '\t')), ('row-backslash', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][:-1] + '\\\\')),
                  ('row-drop', lambda mm: mm['rows'].pop(ri)), ('row-drop-last', lambda mm: mm['rows'].pop()), ('row-dup', lambda mm: mm['rows'].insert(ri, mm['rows'][ri])),
                  ('row-extra-at-end', lambda mm: mm['rows'].append(mm['rows'][0])), ('row-swap', lambda mm: mm['rows'].reverse()),
                  ('row-offset-by-1', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][1:] + mm['rows'][ri][:1])),
                  ('rows-all-dropped', lambda mm: mm['rows'].clear()), ('row-nonascii', lambda mm: mm['rows'].__setitem__(ri, 'é' + mm['rows'][ri][1:])),
                  ('row-cut-mid-code', lambda mm: mm['rows'].__setitem__(ri, mm['rows'][ri][:-1]) if cpp > 1 else mm['rows'].__setitem__(ri, mm['rows'][ri][:-2]))):
        try: px.append((R(edit(f)), {'rule': 'pixels:' + nm, 'field': 'rows', 'value': nm}))
        except (IndexError, TypeError): pass
    cats.append(px)
    st = []
    for nm, f in (('comment-between-strings', lambda mm: mm.__setitem__('comment', '/* x */')), ('last-comma', lambda mm: mm.__setitem__('lastcomma', True)),
                  ('semicolon-row-sep', lambda mm: mm.__setitem__('rowsep', ';')), ('no-row-sep', lambda mm: mm.__setitem__('rowsep', '')),
                  ('space-row-sep', lambda mm: mm.__setitem__('rowsep', ' ,')), ('header-sep-tab', lambda mm: mm.__setitem__('sep', '\t')),
                  ('header-sep-multi', lambda mm: mm.__setitem__('sep', '   ')), ('no-trailing', lambda mm: mm.__setitem__('trailing', '')),
                  ('no-magic', lambda mm: mm.__setitem__('magic', None if mm['version'] == 'xpm2' else '')),
                  ('ext-without-flag', lambda mm: mm.__setitem__('ext', ['XPMEXT comment', 'c', 'XPMENDEXT'])),
                  ('ext-flag-without-ext', lambda mm: mm['header'].append('XPMEXT')), ('ext-no-end', lambda mm: (mm['header'].append('XPMEXT'), mm.__setitem__('ext', ['XPMEXT comment', 'c']))),
                  ('ext-bad-name', lambda mm: (mm['header'].append('XPMEXT'), mm.__setitem__('ext', ['XPMEXT', 'XPMENDEXT']))),
                  ('xpm2-as-xpm3-magic', lambda mm: mm.__setitem__('version', 'xpm3' if mm['version'] == 'xpm2' else 'xpm2'))):
        try: st.append((R(edit(f)), {'rule': 'syntax:' + nm, 'field': 'structure', 'value': nm}))
        except (IndexError, TypeError): pass
    for i, d in enumerate(DECLS): st.append((R(edit(lambda mm, d=d: mm.__setitem__('decl', d))), {'rule': 'syntax:decl', 'field': 'decl', 'value': i}))
    for mg in MAGICS: st.append((R(edit(lambda mm, mg=mg: mm.__setitem__('magic', mg))), {'rule': 'syntax:magic', 'field': 'magic', 'value': mg}))
    st.append((b + b, {'rule': 'syntax:doubled', 'field': 'text', 'value': 'twice'}))
    st.append((b + b'/* trailing */\n', {'rule': 'syntax:trailing-comment', 'field': 'text', 'value': 'comment'}))
    st.append((b + b'x', {'rule': 'syntax:trailing-garbage', 'field': 'text', 'value': 'x'}))
    st.append((b'#define X 1\n' + b, {'rule': 'syntax:leading-define', 'field': 'text', 'value': 'define'}))
    st.append((b.replace(b'",\n"', b'" "'), {'rule': 'syntax:string-concat', 'field': 'text', 'value': 'adjacent literals'}))
    st.append((b.replace(b'",\n"', b'",\n\n\n"'), {'rule': 'syntax:blank-lines', 'field': 'text', 'value': 'blank lines'}))
    st.append((b.replace(b'"', b"'"), {'rule': 'syntax:single-quotes', 'field': 'text', 'value': 'single quotes'}))
    st.append((b.replace(b'\n', b'\n  '), {'rule': 'syntax:indent', 'field': 'text', 'value': 'indented'}))
    rng.shuffle(st); cats.append(st[:30])
    cats.append(list(textmut.truncations(b, rng, 14)))
    cats.append(list(textmut.token_ops(b, rng, 16)))
    cats.append(list(textmut.byte_flips(b, rng, 8)))
    cats.append(list(textmut.bad_chars(b, rng, 8)))
    cats.append(list(textmut.long_lines(b, rng, 3)))
    cats.append(list(textmut.newline_styles(b)))
    for c in cats: rng.shuffle(c)
    merged = []
    for i in range(max(len(c) for c in cats)):
        for c in cats:
            if i < len(c): merged.append(c[i])
    return iter(merged)


def generate(limit, seed):
    rng = random.Random(seed)
    base = seeds(rng)
    items = [(b, m) for b, m, _ in base]
    streams = [(m['name'], mutations(model, b, random.Random(f'{seed}:{m["name"]}'))) for b, m, model in base]
    return textmut.interleave(streams, limit, items[:limit]), len(base)
