#!/usr/bin/env python3
"""PNM adapter (P1-P6 plain/raw PBM, PGM, PPM and P7 PAM): synthetic seeds, token-level header mutations, decoder set."""
import random, struct, sys
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'ext'))
import common  # noqa: E402

NAME, EXT = 'pnm', 'pnm'
R = common.RUNNER
G = common.GDK
DECODERS = {
    'image': (R, ['pnm:image']),
    'glycin': (R, ['pnm:glycin']),
    'zune-ppm': (R, ['pnm:zune']),
    'oxideav-pbm': (R, ['pnm:oxideav']),
    'gdk-file': (G, ['pnm', 'file']),
    'gdk-auto': (G, ['pnm', 'auto-whole']),
    'gdk-forced': (G, ['pnm', 'forced-whole']),
    'gdk-forced-byte': (G, ['pnm', 'forced-byte']),
}
UNAVAILABLE = {
    'netpbm CLI (pnmtopng/pamfile)': 'not installed; upstream is on SourceForge, outside the allowed network (crates.io and github.com only)',
    'pnm crate 0.1.1': 'does not build on the stable toolchain (its dependencies use #![feature(core_intrinsics)] and iter_array_chunks)',
    'ImageMagick / Pillow': 'not installed',
}
# image and glycin share image::codecs::pnm (one vote); the four GDK entry points share one module (one vote).
GROUPS = {'image': 'image-rs', 'glycin': 'image-rs', 'gdk-file': 'gdk-pixbuf', 'gdk-auto': 'gdk-pixbuf',
          'gdk-forced': 'gdk-pixbuf', 'gdk-forced-byte': 'gdk-pixbuf'}

CH = {'P1': 1, 'P4': 1, 'P2': 1, 'P5': 1, 'P3': 3, 'P6': 3}


def samples(rng, n, maxval):
    pool = [0, maxval, maxval // 2, 1 if maxval > 1 else 0, maxval - 1 if maxval > 1 else 0]
    return [rng.choice(pool) if rng.random() < 0.4 else rng.randrange(maxval + 1) for _ in range(n)]


def enc(s):
    return b''.join(c.encode('latin1') if ord(c) < 256 else c.encode('utf-8') for c in s)


def ser(spec, **o):
    """Serialise a classic-PNM spec; any token or separator may be overridden (text values) to build mutations."""
    magic, w, h, mv = o.get('magic', spec['magic']), o.get('w', str(spec['w'])), o.get('h', str(spec['h'])), o.get('mv', str(spec['maxval']) if spec['maxval'] else None)
    sep = list(o.get('seps', spec['seps']))
    head = o.get('pre', '') + magic + sep[0] + w + sep[1] + h
    if spec['magic'] not in ('P1', 'P4') and mv is not None:
        head += sep[2] + mv
    head += sep[3]
    return enc(head) + o.get('body', spec['body'])


def body_bytes(magic, w, h, ch, maxval, vals):
    if magic in ('P2', 'P3'):
        out, per = [], w * ch
        for i in range(0, len(vals), per):
            out.append(' '.join(map(str, vals[i:i + per])))
        return ('\n'.join(out) + '\n').encode()
    if magic == 'P1':
        return '\n'.join(''.join(str(v) for v in vals[i:i + w]) for i in range(0, len(vals), w)).encode() + b'\n' if False else \
            ('\n'.join(' '.join(str(v) for v in vals[i:i + w]) for i in range(0, len(vals), w)) + '\n').encode()
    if magic == 'P4':
        row = (w + 7) // 8
        out = bytearray()
        for y in range(h):
            bits = vals[y * w:(y + 1) * w] + [0] * (row * 8 - w)
            out += bytes(int(''.join(map(str, bits[i:i + 8])), 2) for i in range(0, row * 8, 8))
        return bytes(out)
    if maxval < 256:
        return bytes(vals)
    return b''.join(struct.pack('>H', v) for v in vals)


def classic(magic, w, h, maxval, name):
    rng = random.Random(name)
    ch = CH[magic]
    mv = 1 if magic in ('P1', 'P4') else maxval
    vals = samples(rng, w * h * ch, mv)
    spec = {'kind': 'classic', 'magic': magic, 'w': w, 'h': h, 'maxval': None if magic in ('P1', 'P4') else maxval, 'ch': ch,
            'seps': [' ', ' ', ' ' if magic not in ('P1', 'P4') else '', '\n'], 'body': body_bytes(magic, w, h, ch, mv, vals)}
    spec['seps'][1] = ' ' if magic in ('P1', 'P4') else ' '
    data = ser(spec)
    return data, {'name': name, 'spec': spec, 'header_len': len(data) - len(spec['body'])}


PAM_TUPL = {'BLACKANDWHITE': 1, 'GRAYSCALE': 1, 'GRAYSCALE_ALPHA': 2, 'RGB': 3, 'RGB_ALPHA': 4}


def pam_ser(fields, body, eol='\n', end='ENDHDR'):
    return ('P7' + eol + ''.join(f'{k} {v}{eol}' for k, v in fields) + end + eol).encode('latin1') + body


def pam(tupl, w, h, maxval, name, depth=None):
    rng = random.Random(name)
    d = PAM_TUPL[tupl] if depth is None else depth
    vals = samples(rng, w * h * d, maxval)
    body = bytes(vals) if maxval < 256 else b''.join(struct.pack('>H', v) for v in vals)
    fields = [('WIDTH', w), ('HEIGHT', h), ('DEPTH', d), ('MAXVAL', maxval), ('TUPLTYPE', tupl)]
    data = pam_ser(fields, body)
    return data, {'name': name, 'spec': {'kind': 'pam', 'w': w, 'h': h, 'fields': fields, 'body': body, 'maxval': maxval, 'depth': d}, 'header_len': len(data) - len(body)}


def seeds():
    out = []
    for w, h in ((1, 1), (5, 3), (8, 2), (9, 2), (17, 2)):
        for m in ('P1', 'P4'):
            out.append(classic(m, w, h, 1, f'{m}-{w}x{h}'))
    for m in ('P2', 'P5'):
        for mv in (255, 15, 1, 254, 256, 1000, 65535):
            out.append(classic(m, 5, 3, mv, f'{m}-5x3-max{mv}'))
        out.append(classic(m, 1, 1, 255, f'{m}-1x1-max255'))
    for m in ('P3', 'P6'):
        for mv in (255, 100, 1, 256, 65535):
            out.append(classic(m, 4, 3, mv, f'{m}-4x3-max{mv}'))
        out.append(classic(m, 1, 1, 255, f'{m}-1x1-max255'))
    for t, mv in (('BLACKANDWHITE', 1), ('GRAYSCALE', 255), ('GRAYSCALE', 65535), ('GRAYSCALE', 15), ('GRAYSCALE_ALPHA', 255), ('GRAYSCALE_ALPHA', 65535),
                  ('RGB', 255), ('RGB', 65535), ('RGB_ALPHA', 255), ('RGB_ALPHA', 65535), ('RGB_ALPHA', 15)):
        out.append(pam(t, 5, 3, mv, f'P7-{t}-max{mv}'))
    out.append(pam('RGB', 1, 1, 255, 'P7-RGB-1x1'))
    return out


SEPS = ['\t', '\r', '\n', '\r\n', ' \n', '  ', '\x0b', '\x0c', '\0', '', '\n\n', '#c\n', ' #c\n', '\xa0']
NUMS = ['0', '1', '2', '255', '256', '65535', '65536', '2147483647', '2147483648', '4294967295', '4294967296', '18446744073709551615',
        '18446744073709551616', '99999999999999999999999', '-1', '+3', '03', '000000000000003', '3.0', '0x3', '3e0', '', '3a', '٣']
MAXV = ['0', '1', '2', '15', '16', '127', '254', '255', '256', '257', '1000', '32767', '32768', '65534', '65535', '65536', '65537', '4294967295', '4294967296', '-1', '+255', '0255', '255.0', '', 'x']


def classic_mutations(data, meta, rng):
    sp = meta['spec']
    magic, w, h, mv = sp['magic'], sp['w'], sp['h'], sp['maxval']
    M = lambda rule, field, old, new, b: (b, {'rule': rule, 'field': field, 'old': old, 'new': new})
    for t in ('P0', 'P7', 'P8', 'P9', 'P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'p6', 'P', 'PP', 'P66', 'P 6', 'Pf', 'PF', 'P6\0', 'Q6', ''):
        if t != magic:
            yield M('magic', 'magic', magic, t, ser(sp, magic=t))
    for pre in ('\n', ' ', '#c\n', '﻿', '\0', '\r\n'):
        yield M('magic-prefix', 'prefix', '', repr(pre), ser(sp, pre=pre))
    for t in NUMS:
        yield M('header-width', 'width', w, t, ser(sp, w=t))
        yield M('header-height', 'height', h, t, ser(sp, h=t))
    for v in {w - 1, w + 1, h, w * 2, h * 2} - {w, 0}:
        yield M('header-width', 'width', w, v, ser(sp, w=str(v)))
        yield M('header-height', 'height', h, v, ser(sp, h=str(v)))
    for ww, hh in ((65536, 65536), (2**31, 2), (2**32 - 1, 2**32 - 1), (2**63 - 1, 2), (0, 0), (2**16, 2**16 + 1), (46341, 46341), (3, 2**31)):
        yield M('header-both', 'width+height', f'{w}x{h}', f'{ww}x{hh}', ser(sp, w=str(ww), h=str(hh)))
    if mv:
        for t in MAXV:
            if t != str(mv):
                yield M('header-maxval', 'maxval', mv, t, ser(sp, mv=t))
        yield M('header-maxval', 'maxval', mv, 'removed', ser(sp, mv=None) if False else (ser(sp, mv='') if False else data.replace(f'{mv}'.encode(), b'', 1)))
    for idx, name in enumerate(('after-magic', 'before-height', 'before-maxval', 'after-header')):
        if idx == 2 and not mv:
            continue
        for s in SEPS:
            seps = list(sp['seps'])
            if seps[idx] == s:
                continue
            yield M('separator-' + name, 'sep', repr(seps[idx]), repr(s), ser(sp, seps=[*seps[:idx], s, *seps[idx + 1:]]))
    # whitespace between maxval and raw body: extra byte becomes sample data
    if magic in ('P4', 'P5', 'P6'):
        for s in ('  ', ' \n', '\r\n', '\n\n', '\n#c\n', '\t'):
            yield M('separator-after-header', 'sep', "'\\n'", repr(s), ser(sp, seps=[*sp['seps'][:3], s]))
    for pos, text in (('before-magic', '#c\n'), ('inside-magic', '#c\n'), ('after-magic', '#c\n'), ('between-width-height', '#c\n'), ('after-height', '#c\n'),
                      ('after-maxval-before-ws', '#c'), ('in-body-start', '#c\n'), ('unterminated', '#c'), ('cr-only', '#c\r'), ('long', '#' + 'x' * 4096 + '\n'),
                      ('binary-bytes', '#\xff\xfe\n'), ('p-hash-newline', '#\n')):
        s = list(sp['seps'])
        if pos == 'before-magic':
            b = ser(sp, pre=text)
        elif pos == 'inside-magic':
            b = ser(sp, magic=magic[0] + text + magic[1:])
        elif pos == 'after-magic':
            b = ser(sp, seps=[text, *s[1:]])
        elif pos == 'between-width-height':
            b = ser(sp, seps=[s[0], ' ' + text, *s[2:]])
        elif pos == 'after-height':
            b = ser(sp, seps=[*s[:2], ' ' + text if mv else s[2], s[3]] if mv else [*s[:3], text + ' '])
        elif pos == 'after-maxval-before-ws':
            b = ser(sp, seps=[*s[:3], text + '\n']) if mv else ser(sp, seps=[*s[:3], text + '\n'])
        elif pos == 'in-body-start':
            b = data[:meta['header_len']] + text.encode('latin1') + data[meta['header_len']:]
        elif pos == 'unterminated':
            b = ser(sp, seps=[*s[:3], text])
        else:
            b = ser(sp, seps=[*s[:3], text + '\n'])
        yield M('comment', 'position', 'none', pos, b)
    hl = meta['header_len']
    body = sp['body']
    if magic in ('P1', 'P2', 'P3'):
        toks = body.decode().split()
        join = lambda t, sep=' ': ser(sp, body=(sep.join(t) + '\n').encode())
        yield M('plain-body', 'samples', len(toks), 'dropped last', join(toks[:-1]))
        yield M('plain-body', 'samples', len(toks), 'extra sample', join(toks + ['0']))
        yield M('plain-body', 'samples', len(toks), 'extra 100 samples', join(toks + ['1'] * 100))
        yield M('plain-body', 'separators', 'space/newline', 'tabs', join(toks, '\t'))
        yield M('plain-body', 'separators', 'space/newline', 'CRLF', join(toks, '\r\n'))
        yield M('plain-body', 'separators', 'space/newline', 'comment per sample', join([t + ' #c\n' for t in toks], ''))
        yield M('plain-body', 'separators', 'space/newline', 'comment inside body', join([toks[0] + ' #c\n'] + toks[1:]))
        yield M('plain-body', 'separators', 'space/newline', 'unterminated comment at end', join(toks) + '#c' if False else ser(sp, body=body + b'#c'))
        yield M('plain-body', 'separators', 'space/newline', 'NUL separators', join(toks, '\0'))
        if magic == 'P1':
            yield M('p1-packed-digits', 'samples', 'separated', 'no whitespace', ser(sp, body=''.join(toks).encode()))
            yield M('p1-packed-digits', 'samples', 'separated', 'no whitespace plus newline', ser(sp, body=(''.join(toks) + '\n').encode()))
            for bad in ('2', '9', 'x', '-1', '01', '00'):
                yield M('plain-body-value', 'sample0', toks[0], bad, join([bad] + toks[1:]))
        else:
            for bad in (str(mv + 1), str(mv * 2 + 1), '65535', '65536', '4294967296', '-1', '+1', 'abc', '1e2', '007', '0x10', str(mv) + 'x', '1.5'):
                yield M('plain-body-value', 'sample0', toks[0], bad, join([bad] + toks[1:]))
            yield M('plain-body-value', 'sample0', toks[0], 'all above maxval', join([str(mv + 1)] * len(toks)))
    elif magic == 'P4':
        row = (w + 7) // 8
        if w % 8:
            pad = bytearray(body)
            for y in range(h):
                pad[y * row + row - 1] |= (1 << (8 - w % 8)) - 1
            yield M('p4-padding-bits', 'padding', 'zero', 'set', ser(sp, body=bytes(pad)))
        yield M('p4-padding-bits', 'rows', 'padded', 'unpadded (packed across rows)', ser(sp, body=bytes(int(''.join(format(b, '08b') for b in body)[i:i + 8].ljust(8, '0'), 2) for i in range(0, w * h, 8))))
    else:
        bps = 2 if mv > 255 else 1
        for v in (mv + 1, 255, 0xFF if mv < 255 else mv):
            if mv < 255 and v > mv:
                b = bytearray(body)
                b[0] = v
                yield M('binary-sample-above-maxval', 'sample0', body[0], v, ser(sp, body=bytes(b)))
        if mv > 255 and mv < 65535:
            b = bytearray(body)
            b[0:2] = struct.pack('>H', mv + 1)
            yield M('binary-sample-above-maxval', 'sample0', 'seed', mv + 1, ser(sp, body=bytes(b)))
            b[0:2] = struct.pack('>H', 65535)
            yield M('binary-sample-above-maxval', 'sample0', 'seed', 65535, ser(sp, body=bytes(b)))
        if bps == 2:
            yield M('binary-endianness', 'samples', 'big-endian', 'little-endian', ser(sp, body=b''.join(body[i + 1:i + 2] + body[i:i + 1] for i in range(0, len(body), 2))))
        yield M('binary-body', 'body', len(body), 'first byte 0x23 then newline', ser(sp, body=b'#\n' + body[2:]))
        yield M('binary-body', 'body', len(body), 'one byte short', ser(sp, body=body[:-1]))
        yield M('binary-body', 'body', len(body), 'one sample short', ser(sp, body=body[:-bps]))
        yield M('binary-body', 'body', len(body), 'one sample extra', ser(sp, body=body + b'\0' * bps))
        yield M('binary-body', 'body', len(body), 'one row extra', ser(sp, body=body + b'\0' * (w * sp['ch'] * bps)))


def pam_mutations(data, meta, rng):
    sp = meta['spec']
    fields, body, w, h, mv = sp['fields'], sp['body'], sp['w'], sp['h'], sp['maxval']
    M = lambda rule, field, old, new, b: (b, {'rule': rule, 'field': field, 'old': old, 'new': new})
    P = lambda f, **kw: pam_ser(f, kw.pop('body', body), **kw)
    for i, (k, v) in enumerate(fields):
        yield M('pam-missing-field', k, v, 'removed', P(fields[:i] + fields[i + 1:]))
        yield M('pam-duplicate-field', k, v, 'duplicated', P(fields[:i + 1] + [(k, v)] + fields[i + 1:]))
        yield M('pam-duplicate-field', k, v, 'duplicated with other value', P(fields[:i + 1] + [(k, '1' if k != 'TUPLTYPE' else 'RGB')] + fields[i + 1:]))
        yield M('pam-key-case', k, k, k.lower(), P(fields[:i] + [(k.lower(), v)] + fields[i + 1:]))
        yield M('pam-key-spacing', k, ' ', 'tab/multiple spaces', pam_ser(fields, body).replace(f'{k} '.encode(), f'{k}\t  '.encode(), 1))
        yield M('pam-key-spacing', k, ' ', 'no space', pam_ser(fields, body).replace(f'{k} '.encode(), k.encode(), 1))
        yield M('pam-key-spacing', k, v, 'trailing spaces', pam_ser(fields, body).replace(f'{k} {v}\n'.encode(), f'{k} {v}  \n'.encode(), 1))
    yield M('pam-order', 'fields', 'canonical', 'reversed', P(fields[::-1]))
    yield M('pam-order', 'fields', 'canonical', 'TUPLTYPE first', P(fields[-1:] + fields[:-1]))
    yield M('pam-order', 'fields', 'canonical', 'MAXVAL before DEPTH', P([fields[0], fields[1], fields[3], fields[2], fields[4]]))
    for s in (w - 1, w + 1, 0, 65536, 2**32, 2**32 - 1, 2**63, -1):
        yield M('pam-header-value', 'WIDTH', w, s, P([('WIDTH', s)] + fields[1:]))
        yield M('pam-header-value', 'HEIGHT', h, s, P([fields[0], ('HEIGHT', s)] + fields[2:]))
    for s in (0, 1, 2, 3, 4, 5, 255, 256, -1):
        if s != sp['depth']:
            yield M('pam-depth-vs-tupltype', 'DEPTH', sp['depth'], s, P([*fields[:2], ('DEPTH', s), *fields[3:]]))
    for s in (0, 1, 2, 15, 255, 256, 65535, 65536, 4294967296, -1):
        if s != mv:
            yield M('pam-header-value', 'MAXVAL', mv, s, P([*fields[:3], ('MAXVAL', s), fields[4]]))
    tl = fields[4][1]
    for t in ('RGB', 'GRAYSCALE', 'RGB_ALPHA', 'GRAYSCALE_ALPHA', 'BLACKANDWHITE', 'BLACKANDWHITE_ALPHA', 'rgb', 'CMYK', 'CUSTOM', '', 'RGB RGB', 'RGB_ALPHA GRAYSCALE'):
        if t != tl:
            yield M('pam-tupltype', 'TUPLTYPE', tl, t, P([*fields[:4], ('TUPLTYPE', t)]))
    yield M('pam-tupltype', 'TUPLTYPE', tl, 'multiple lines', P([*fields[:4], ('TUPLTYPE', 'RGB'), ('TUPLTYPE', 'ALPHA')]))
    yield M('pam-header-comment', 'comment', 'none', 'after P7', pam_ser(fields, body).replace(b'P7\n', b'P7\n# c\n', 1))
    yield M('pam-header-comment', 'comment', 'none', 'before ENDHDR', pam_ser(fields, body).replace(b'ENDHDR', b'# c\nENDHDR', 1))
    yield M('pam-header-comment', 'comment', 'none', 'after value on same line', pam_ser(fields, body).replace(b'MAXVAL ', b'MAXVAL # c\n', 1))
    yield M('pam-header-comment', 'comment', 'none', 'unterminated in header', pam_ser(fields, body, end='ENDHDR# c').replace(b'ENDHDR# c\n', b'#c'))
    yield M('pam-endhdr', 'ENDHDR', 'ENDHDR\\n', 'CRLF', P(fields, eol='\r\n'))
    yield M('pam-endhdr', 'ENDHDR', 'ENDHDR\\n', 'no newline', pam_ser(fields, body).replace(b'ENDHDR\n', b'ENDHDR', 1))
    yield M('pam-endhdr', 'ENDHDR', 'ENDHDR\\n', 'ENDHDR then space', pam_ser(fields, body).replace(b'ENDHDR\n', b'ENDHDR \n', 1))
    yield M('pam-endhdr', 'ENDHDR', 'ENDHDR\\n', 'missing', pam_ser(fields, body, end='').replace(b'\n\n', b'\n', 1) if False else pam_ser(fields, body).replace(b'ENDHDR\n', b'', 1))
    yield M('pam-endhdr', 'ENDHDR', 'ENDHDR\\n', 'lowercase', pam_ser(fields, body).replace(b'ENDHDR', b'endhdr', 1))
    yield M('pam-magic', 'magic', 'P7\\n', 'P7 space', pam_ser(fields, body).replace(b'P7\n', b'P7 ', 1))
    yield M('pam-magic', 'magic', 'P7\\n', 'P7 no newline', pam_ser(fields, body).replace(b'P7\n', b'P7', 1))
    for t in ('P6', 'P5', 'P8'):
        yield M('magic', 'magic', 'P7', t, t.encode() + data[2:])
    yield M('pam-body', 'body', len(body), 'one sample short', P(fields, body=body[:-1]))
    yield M('pam-body', 'body', len(body), 'one sample extra', P(fields, body=body + b'\0'))
    if mv < 255:
        b = bytearray(body)
        b[0] = 255
        yield M('pam-body', 'sample0', body[0], 255, P(fields, body=bytes(b)))
    if mv > 255 and mv < 65535:
        b = bytearray(body)
        b[0:2] = b'\xff\xff'
        yield M('pam-body', 'sample0', 'seed', 65535, P(fields, body=bytes(b)))


def mutations(data, meta, rng):
    return classic_mutations(data, meta, rng) if meta['spec']['kind'] == 'classic' else pam_mutations(data, meta, rng)


if __name__ == '__main__':
    common.main(sys.modules[__name__])
