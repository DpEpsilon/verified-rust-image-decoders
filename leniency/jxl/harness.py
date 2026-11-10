#!/usr/bin/env python3
"""JPEG XL adapter. Seeds: libjxl encoder (runners/mkseeds.c): modular/VarDCT, gray/RGB/alpha, 8/16-bit, animation,
container with Exif/XMP boxes. Mutations: ISOBMFF-style container boxes (jxlc/jxlp/brob/ftyp), container <-> bare codestream
conversion, jxlp splitting, and byte/bit-level sweeps over the bit-packed codestream header (no bit-level parser)."""
import os, random, struct, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'modern'))
import common as c

BIN = Path(os.environ.get('LENIENCY_JXL_BIN', '/tmp/claude-0/leniency-jxl/bin'))
RUST = Path(os.environ.get('LENIENCY_TARGET', '/tmp/claude-0/target-modern')) / 'release/jxl-runner'
SIG = b'\x00\x00\x00\x0cJXL \r\n\x87\n'


def box(tag, payload):
    return struct.pack('>I', 8 + len(payload)) + tag + payload


def is_container(b):
    return b[:12] == SIG or b[4:8] == b'JXL '


def boxes(b):
    """(tag, header_off, header_len, payload_off, end)"""
    out, p, n = [], 0, len(b)
    while p + 8 <= n:
        L = c.u(b, p, 4, 'big')
        hl = 8
        if L == 1 and p + 16 <= n:
            L, hl = c.u(b, p + 8, 8, 'big'), 16
        elif L == 0:
            L = n - p
        if L < hl:
            break
        end = min(p + L, n)
        out.append((b[p + 4:p + 8], p, hl, p + hl, end))
        p += L
    return out


def codestream(b):
    """Concatenated codestream bytes of a container (jxlc, or jxlp parts in file order), or b itself if bare."""
    if not is_container(b):
        return b
    parts = []
    for tag, h, hl, ps, pe in boxes(b):
        if tag == b'jxlc':
            parts.append(b[ps:pe])
        elif tag == b'jxlp':
            parts.append(b[ps + 4:pe])
    return b''.join(parts)


def fields(b):
    f = []
    if is_container(b):
        for tag, h, hl, ps, pe in boxes(b):
            t = tag.decode('latin1').strip() or 'box'
            f += [(f'{t}_LBox', h, 4, 'big'), (f'{t}_TBox', h + 4, 4, 'big')]
            if tag == b'jxlp':
                f.append(('jxlp_index', ps, 4, 'big'))
            if tag == b'ftyp':
                f += [('ftyp_brand', ps, 4, 'big'), ('ftyp_minor', ps + 4, 4, 'big')]
            if tag == b'jxlc':
                f += [(f'jxlc_cs_byte{i}', ps + i, 1, 'big') for i in range(4)]
    else:
        f += [(f'cs_byte{i}', i, 1, 'big') for i in range(min(len(b), 16))]
    return [x for x in f if x[1] + x[2] <= len(b)]


def cut_points(b):
    pts = set()
    if is_container(b):
        for tag, h, hl, ps, pe in boxes(b):
            pts |= {h, ps, pe}
    return sorted(x for x in pts if 0 < x < len(b))


def structural(b, rng):
    def R(rule, field, data, old=None, new=None):
        return data, {'rule': rule, 'field': field, 'old': old, 'new': new}
    cs = codestream(b)
    cont = is_container(b)
    # container conversions
    if cont:
        yield R('unwrap', 'bare-codestream', cs)
        bl = boxes(b)
        for tag, h, hl, ps, pe in bl:
            nm = tag.decode('latin1').strip()
            yield R('box-delete', nm, b[:h] + b[pe:])
            yield R('box-duplicate', nm, b[:pe] + b[h:pe] + b[pe:])
            yield R('box-length-zero', nm, b[:h] + b'\0\0\0\0' + b[h + 4:])
            if hl == 8:
                yield R('box-xlbox', nm, b[:h] + struct.pack('>I', 1) + b[h + 4:h + 8] + struct.pack('>Q', pe - h + 8) + b[ps:])
        for i in range(len(bl) - 1):
            a, bb = bl[i], bl[i + 1]
            yield R('box-swap', f'{a[0].decode("latin1").strip()}<->{bb[0].decode("latin1").strip()}', b[:a[1]] + b[bb[1]:bb[4]] + b[a[1]:a[4]] + b[bb[4]:])
        last = bl[-1]
        for tag in (b'JUNK', b'free', b'jxli', b'jbrd', b'Exif', b'xml ', b'abcd', b'brob'):
            yield R('box-insert', f'{tag.decode()}-before-codestream', b[:bl[-1][1]] + box(tag, b'\0' * 12) + b[bl[-1][1]:])
        yield R('box-insert', 'trailing-box', b + box(b'free', b'zz'))
        yield R('trailing-codestream-copy', 'duplicate-jxlc', b + b[last[1]:last[4]])
        for nm, v in (('JXL\\x20 -> JXL!', b'\x00\x00\x00\x0cJXL!\r\n\x87\n'), ('LF-only', b'\x00\x00\x00\x0cJXL \n\n\x87\n'), ('short-sig', b'\x00\x00\x00\x08JXL ')):
            yield R('signature-box', nm, v + b[len(SIG):] if len(v) == len(SIG) else v + b[len(SIG):])
        for brand in (b'jxl\x20', b'jxl2', b'jxlx', b'XXXX'):
            for tag, h, hl, ps, pe in bl:
                if tag == b'ftyp':
                    yield R('ftyp-brand', brand.decode('latin1'), b[:ps] + brand + b[ps + 4:])
        # jxlc -> jxlp splits
        for tag, h, hl, ps, pe in bl:
            if tag == b'jxlc':
                pay = b[ps:pe]
                pre, post = b[:h], b[pe:]
                for nparts in (2, 3):
                    k = max(1, len(pay) // nparts)
                    chunks = [pay[i:i + k] for i in range(0, len(pay), k)]
                    def jp(idx_list):
                        return b''.join(box(b'jxlp', struct.pack('>I', ix) + ch) for ix, ch in zip(idx_list, chunks))
                    n = len(chunks)
                    good = [i | (0x80000000 if i == n - 1 else 0) for i in range(n)]
                    yield R('jxlp-split', f'{n}-parts-correct', pre + jp(good) + post)
                    yield R('jxlp-split', f'{n}-parts-no-last-flag', pre + jp(list(range(n))) + post)
                    yield R('jxlp-split', f'{n}-parts-last-flag-early', pre + jp([0x80000000] + list(range(1, n))) + post)
                    yield R('jxlp-split', f'{n}-parts-index-gap', pre + jp([i * 2 | (0x80000000 if i == n - 1 else 0) for i in range(n)]) + post)
                    yield R('jxlp-split', f'{n}-parts-reversed-order', pre + b''.join(box(b'jxlp', struct.pack('>I', i | (0x80000000 if i == n - 1 else 0)) + ch) for i, ch in reversed(list(enumerate(chunks)))) + post)
                yield R('jxlc-plus-jxlp', 'both', pre + box(b'jxlp', struct.pack('>I', 0x80000000) + pay[:8]) + b[h:pe] + post)
    else:
        sig = SIG + box(b'ftyp', b'jxl \0\0\0\0jxl ')
        yield R('wrap', 'jxlc', sig + box(b'jxlc', b))
        yield R('wrap', 'jxlc-no-ftyp', SIG + box(b'jxlc', b))
        yield R('wrap', 'jxlc-no-signature', box(b'ftyp', b'jxl \0\0\0\0jxl ') + box(b'jxlc', b))
        yield R('wrap', 'jxlc-length-zero', sig + b'\0\0\0\0jxlc' + b)
        k = max(1, len(b) // 2)
        yield R('wrap', 'jxlp-2-correct', sig + box(b'jxlp', struct.pack('>I', 0) + b[:k]) + box(b'jxlp', struct.pack('>I', 0x80000001) + b[k:]))
        yield R('wrap', 'jxlp-1-last', sig + box(b'jxlp', struct.pack('>I', 0x80000000) + b))
        yield R('prefix', 'leading-zero-byte', b'\0' + b)
        yield R('prefix', 'leading-ff0a-twice', b[:2] + b)
        yield R('signature', 'ff0b', b'\xff\x0b' + b[2:])
        yield R('signature', 'ff0a-dropped', b[2:])
    # codestream header byte sweeps (bit-packed header: no parser, probe each of the first bytes with a few values)
    ps0 = 0
    if cont:
        for tag, h, hl, ps, pe in boxes(b):
            if tag in (b'jxlc',):
                ps0 = ps
            elif tag == b'jxlp' and not ps0:
                ps0 = ps + 4
    for i in range(min(10, len(cs))):
        for v in rng.sample(range(256), 10):
            if cs[i] != v:
                yield R('cs-header-byte-sweep', f'byte{i}', b[:ps0 + i] + bytes([v]) + b[ps0 + i + 1:], cs[i], v)


def mutations(data, meta, rng):
    yield from c.field_mutations(data, fields(data), rng=rng, per_field=6)
    yield from c.generic_mutations(data, rng, cut_points=cut_points(data), nflip=26)
    yield from structural(data, rng)


def seeds():
    out = []
    for p in sorted((HERE / 'seeds').glob('*.jxl')):
        d = p.read_bytes()
        n = p.stem
        out.append((d, {'name': n, 'kind': ('animation' if n.startswith('anim') else 'modular' if n.startswith('ml') else 'vardct') + ('-container' if is_container(d) else '-bare')}))
    return out


class JXL(c.Adapter):
    fmt, ext, title = 'jxl', 'jxl', 'JPEG XL (image/jxl)'
    tol = 2
    dir = HERE
    runners = {'glycin': (RUST, ['jxl:glycin']), 'jxl-oxide': (RUST, ['jxl:oxide']), 'libjxl': (BIN / 'libjxl', ['x'])}
    groups = {'glycin': 'jxl-rs', 'jxl-oxide': 'jxl-oxide', 'libjxl': 'libjxl'}
    decoders_doc = {
        'glycin': 'glycin-image-rs `ImgLoader` for `image/jxl` (special loader::jxl -> `jxl` crate 0.8.0 = jxl-rs), frames via `specific_frame` until `NoMoreFrames`',
        'jxl-oxide': 'jxl-oxide 0.12.6 `JxlImage::render_frame` for every keyframe; f32 -> RGBA8 by round(clamp*255); independent of jxl-rs',
        'libjxl': 'libjxl (github master 0.13.0-dev, built with cmake, single-threaded) `JxlDecoder`, float RGBA output, unpremultiplied alpha, orientation applied, round(clamp*255)'}
    unavailable = [('gdk-pixbuf JXL module', 'not installed'),
                   ('`jpegxl-rs` / `jpegxl-sys` crates', 'bind libjxl: not independent of the libjxl column, not built'),
                   ('CMYK and spot-colour output in the native/oxide runners', 'reported as `skip` (n/a); no colour conversion is implemented')]
    notes = (open(HERE / 'NOTES.md').read() if (HERE / 'NOTES.md').exists() else '')
    seeds = staticmethod(seeds)
    mutations = staticmethod(mutations)


if __name__ == '__main__':
    c.run(JXL())
