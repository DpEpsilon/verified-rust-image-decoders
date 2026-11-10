#!/usr/bin/env python3
"""JPEG 2000 adapter. Seeds: libopenjp2 encoder (runners/mkseeds.c) plus Python-built JP2 wrappers (palette, cdef,
colr variants). Mutations: JP2 box fields, codestream marker segments (SIZ/COD/QCD/SOT...), wrapper add/strip."""
import os, random, struct, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'modern'))
import common as c

BIN = Path(os.environ.get('LENIENCY_JP2_BIN', '/tmp/claude-0/leniency-jp2/bin'))
RUST = Path(os.environ.get('LENIENCY_TARGET', '/tmp/claude-0/target-modern')) / 'release/jp2-runner'
SIG = b'\x00\x00\x00\x0cjP  \r\n\x87\n'


def box(tag, payload):
    return struct.pack('>I', 8 + len(payload)) + tag + payload


def boxes(b, start=0, end=None, depth=0):
    """Yield (tag, header_off, header_len, payload_off, payload_end, depth); descends into jp2h."""
    end = len(b) if end is None else end
    p = start
    while p + 8 <= end:
        L = c.u(b, p, 4, 'big')
        tag = b[p + 4:p + 8]
        hl = 8
        if L == 1 and p + 16 <= end:
            L = c.u(b, p + 8, 8, 'big')
            hl = 16
        elif L == 0:
            L = end - p
        if L < hl or p + L > end:
            L = max(hl, min(L, end - p))
        yield tag, p, hl, p + hl, p + L, depth
        if tag == b'jp2h' and depth == 0:
            yield from boxes(b, p + hl, p + L, depth + 1)
        p += L


def codestream_off(b):
    if b[:2] == b'\xff\x4f':
        return 0
    for tag, h, hl, ps, pe, d in boxes(b):
        if tag == b'jp2c':
            return ps
    return None


def markers(b, off):
    """Yield (marker, offset, seg_len) walking main header + SOT/SOD loosely (stops at first SOD)."""
    p = off + 2
    n = len(b)
    while p + 2 <= n:
        m = c.u(b, p, 2, 'big')
        if m == 0xff93 or m == 0xffd9:
            yield m, p, 0
            return
        if m < 0xff30 or m > 0xffff:
            return
        L = c.u(b, p + 2, 2, 'big')
        yield m, p, L
        p += 2 + L


def fields(b):
    f = []
    for tag, h, hl, ps, pe, d in boxes(b):
        t = tag.decode('latin1').strip() or 'box'
        f.append((f'{t}_LBox', h, 4, 'big'))
        f.append((f'{t}_TBox', h + 4, 4, 'big'))
        if tag == b'ihdr':
            f += [('ihdr_height', ps, 4, 'big'), ('ihdr_width', ps + 4, 4, 'big'), ('ihdr_nc', ps + 8, 2, 'big'), ('ihdr_bpc', ps + 10, 1, 'big'),
                  ('ihdr_c', ps + 11, 1, 'big'), ('ihdr_unkc', ps + 12, 1, 'big'), ('ihdr_ipr', ps + 13, 1, 'big')]
        elif tag == b'colr':
            f += [('colr_meth', ps, 1, 'big'), ('colr_prec', ps + 1, 1, 'big'), ('colr_approx', ps + 2, 1, 'big')]
            if b[ps:ps + 1] == b'\x01':
                f.append(('colr_enumcs', ps + 3, 4, 'big'))
        elif tag == b'pclr':
            f += [('pclr_ne', ps, 2, 'big'), ('pclr_npc', ps + 2, 1, 'big'), ('pclr_b0', ps + 3, 1, 'big')]
        elif tag == b'cmap':
            f += [('cmap_cmp0', ps, 2, 'big'), ('cmap_mtyp0', ps + 2, 1, 'big'), ('cmap_pcol0', ps + 3, 1, 'big')]
        elif tag == b'cdef':
            f += [('cdef_n', ps, 2, 'big'), ('cdef_cn0', ps + 2, 2, 'big'), ('cdef_typ0', ps + 4, 2, 'big'), ('cdef_asoc0', ps + 6, 2, 'big')]
            if pe - ps >= 14:
                f += [('cdef_cnlast', pe - 6, 2, 'big'), ('cdef_typlast', pe - 4, 2, 'big'), ('cdef_asoclast', pe - 2, 2, 'big')]
        elif tag == b'bpcc':
            f.append(('bpcc_b0', ps, 1, 'big'))
    off = codestream_off(b)
    if off is not None:
        for m, p, L in markers(b, off):
            nm = {0xff51: 'SIZ', 0xff52: 'COD', 0xff53: 'COC', 0xff5c: 'QCD', 0xff5d: 'QCC', 0xff64: 'COM', 0xff90: 'SOT', 0xff5e: 'RGN', 0xff5f: 'POC'}.get(m, f'M{m:04x}')
            f.append((f'{nm}_marker', p, 2, 'big'))
            if L:
                f.append((f'{nm}_Lseg', p + 2, 2, 'big'))
            q = p + 4
            if m == 0xff51:
                f += [('SIZ_Rsiz', q, 2, 'big'), ('SIZ_Xsiz', q + 2, 4, 'big'), ('SIZ_Ysiz', q + 6, 4, 'big'), ('SIZ_XOsiz', q + 10, 4, 'big'),
                      ('SIZ_YOsiz', q + 14, 4, 'big'), ('SIZ_XTsiz', q + 18, 4, 'big'), ('SIZ_YTsiz', q + 22, 4, 'big'), ('SIZ_XTOsiz', q + 26, 4, 'big'),
                      ('SIZ_YTOsiz', q + 30, 4, 'big'), ('SIZ_Csiz', q + 34, 2, 'big')]
                nc = c.u(b, q + 34, 2, 'big')
                for i in range(min(nc, 5)):
                    f += [(f'SIZ_Ssiz', q + 36 + 3 * i, 1, 'big'), (f'SIZ_XRsiz', q + 37 + 3 * i, 1, 'big'), (f'SIZ_YRsiz', q + 38 + 3 * i, 1, 'big')]
            elif m == 0xff52:
                f += [('COD_Scod', q, 1, 'big'), ('COD_prog', q + 1, 1, 'big'), ('COD_layers', q + 2, 2, 'big'), ('COD_mct', q + 4, 1, 'big'),
                      ('COD_levels', q + 5, 1, 'big'), ('COD_cbw', q + 6, 1, 'big'), ('COD_cbh', q + 7, 1, 'big'), ('COD_cbstyle', q + 8, 1, 'big'),
                      ('COD_transform', q + 9, 1, 'big'), ('COD_precinct0', q + 10, 1, 'big')]
            elif m == 0xff5c:
                f += [('QCD_Sqcd', q, 1, 'big'), ('QCD_SPqcd0', q + 1, 1, 'big'), ('QCD_SPqcd1', q + 2, 1, 'big')]
            elif m == 0xff90:
                f += [('SOT_Isot', q, 2, 'big'), ('SOT_Psot', q + 2, 4, 'big'), ('SOT_TPsot', q + 6, 1, 'big'), ('SOT_TNsot', q + 7, 1, 'big')]
    return [x for x in f if x[1] + x[2] <= len(b)]


def cut_points(b):
    pts = set()
    for tag, h, hl, ps, pe, d in boxes(b):
        pts |= {h, ps, pe}
    off = codestream_off(b)
    if off is not None:
        for m, p, L in markers(b, off):
            pts |= {p, p + 2, p + 2 + L}
    return sorted(x for x in pts if 0 < x < len(b))


def siz(b):
    off = codestream_off(b)
    if off is None or c.u(b, off + 2, 2, 'big') != 0xff51:
        return None
    q = off + 6
    return dict(w=c.u(b, q + 2, 4, 'big') - c.u(b, q + 10, 4, 'big'), h=c.u(b, q + 6, 4, 'big') - c.u(b, q + 14, 4, 'big'),
                nc=c.u(b, q + 34, 2, 'big'), ssiz=b[q + 36] if q + 36 < len(b) else 7)


def codestream_of(b):
    off = codestream_off(b)
    if off is None:
        return None
    if off == 0:
        return b
    for tag, h, hl, ps, pe, d in boxes(b):
        if tag == b'jp2c':
            return b[ps:pe]


def ihdr(w, h, nc, bpc, unkc=0, ipr=0, comp=7):
    return box(b'ihdr', struct.pack('>IIHBBBB', h, w, nc, bpc, comp, unkc, ipr))


def colr_enum(v):
    return box(b'colr', bytes([1, 0, 0]) + struct.pack('>I', v))


def wrap(cs, w, h, nc, bpc, jp2h_extra=b'', colr=None, ftyp=True, order_swap=False):
    jp2h = box(b'jp2h', ihdr(w, h, nc, bpc) + (colr if colr is not None else colr_enum(17 if nc < 3 else 16)) + jp2h_extra)
    parts = [box(b'jP  ', b'\r\n\x87\n')]
    if ftyp:
        parts.append(box(b'ftyp', b'jp2 \0\0\0\0jp2 '))
    parts += [box(b'jp2c', cs), jp2h] if order_swap else [jp2h, box(b'jp2c', cs)]
    return b''.join(parts)


def python_seeds(base):
    """JP2 wrappers the OpenJPEG encoder API cannot emit (palette, explicit cdef/colr variants, bpcc, res, bare to boxed)."""
    out = []
    g = base.get('j2k-gray8-ll.j2k')
    r = base.get('j2k-rgb8-ll-mct.j2k')
    if g:
        cs = g
        pal = bytes([7, 7, 7]) and b''.join(bytes([i, 255 - i, (i * 3) & 255]) for i in range(256))
        pclr = box(b'pclr', struct.pack('>HB', 256, 3) + bytes([7, 7, 7]) + pal)
        cmap = box(b'cmap', b''.join(struct.pack('>HBB', 0, 1, i) for i in range(3)))
        out.append(('py-jp2-palette-gray8', wrap(cs, 16, 16, 1, 7, pclr + cmap, colr_enum(16))))
        out.append(('py-jp2-gray-colr-enum17', wrap(cs, 16, 16, 1, 7)))
        out.append(('py-jp2-gray-as-srgb', wrap(cs, 16, 16, 1, 7, b'', colr_enum(16))))
        out.append(('py-jp2-no-ftyp', wrap(cs, 16, 16, 1, 7, ftyp=False)))
        out.append(('py-jp2-jp2c-before-jp2h', wrap(cs, 16, 16, 1, 7, order_swap=True)))
        out.append(('py-jp2-bpcc-res', wrap(cs, 16, 16, 1, 255, box(b'bpcc', bytes([7])) + box(b'res ', box(b'resc', struct.pack('>HHHHBB', 72, 1, 72, 1, 0, 0))))))
        out.append(('py-jp2-colr-icc-junk', wrap(cs, 16, 16, 1, 7, b'', box(b'colr', bytes([2, 0, 0]) + b'\0' * 12))))
        out.append(('py-jp2-colr-meth3', wrap(cs, 16, 16, 1, 7, b'', box(b'colr', bytes([3, 0, 0]) + b'\0' * 4))))
    if r:
        cdef_swap = box(b'cdef', struct.pack('>H', 3) + b''.join(struct.pack('>HHH', i, 0, 3 - i) for i in range(3)))
        out.append(('py-jp2-rgb-cdef-reversed', wrap(r, 16, 16, 3, 7, cdef_swap, colr_enum(16))))
        out.append(('py-jp2-rgb-ihdr-wrong-nc', wrap(r, 16, 16, 1, 7, b'', colr_enum(16))))
    return out


def structural(b, rng):
    def R(rule, field, data, old=None, new=None):
        return data, {'rule': rule, 'field': field, 'old': old, 'new': new}
    bl = [(t, h, hl, ps, pe, d) for t, h, hl, ps, pe, d in boxes(b)]
    top = [x for x in bl if x[5] == 0]
    boxed = bool(top) and top[0][0] != b'' and b[:2] != b'\xff\x4f'
    if boxed:
        for t, h, hl, ps, pe, d in bl:
            nm = t.decode('latin1').strip()
            yield R('box-delete', nm, b[:h] + b[pe:]) if d == 0 else R('box-delete-inner', nm, b[:h] + b[pe:]) if False else R('box-delete', nm, b[:h] + b[pe:])
            yield R('box-duplicate', nm, b[:pe] + b[h:pe] + b[pe:])
        for t, h, hl, ps, pe, d in top:
            nm = t.decode('latin1').strip()
            if t == b'jp2c':
                yield R('box-length-zero', 'jp2c-to-eof', b[:h] + b'\0\0\0\0' + b[h + 4:])
                yield R('box-xlbox', 'jp2c-extended', b[:h] + struct.pack('>I', 1) + b[h + 4:h + 8] + struct.pack('>Q', pe - h + 8) + b[ps:])
            else:
                yield R('box-length-zero', nm, b[:h] + b'\0\0\0\0' + b[h + 4:])
        for tag in (b'free', b'uuid', b'xml ', b'abcd', b'jp2i'):
            yield R('box-insert', tag.decode(), b[:top[-1][1]] + box(tag, b'\0' * 17) + b[top[-1][1]:])
        yield R('box-insert', 'trailing-after-jp2c', b + box(b'free', b'zz'))
        yield R('box-insert', 'trailing-jp2c-copy', b + b[top[-1][1]:top[-1][4]])
        yield R('unwrap', 'bare-codestream', codestream_of(b) or b)
        cs = codestream_of(b)
        s = siz(b)
        if s and cs:
            for nm, nc, bpc, w, h in (('ihdr-width-plus1', s['nc'], s['ssiz'], s['w'] + 1, s['h']), ('ihdr-height-plus1', s['nc'], s['ssiz'], s['w'], s['h'] + 1),
                                      ('ihdr-nc-plus1', s['nc'] + 1, s['ssiz'], s['w'], s['h']), ('ihdr-bpc-8->12', s['nc'], 11, s['w'], s['h'])):
                yield R('wrap-inconsistent', nm, wrap(cs, w, h, nc, bpc))
            for v in (12, 14, 15, 16, 17, 18, 20, 21, 22, 24, 0, 255):
                yield R('wrap-colr-enumcs', f'enumCS={v}', wrap(cs, s['w'], s['h'], s['nc'], s['ssiz'], b'', colr_enum(v)))
    else:
        s = siz(b)
        if s:
            for nm, v in (('gray', 17), ('srgb', 16), ('sycc', 18), ('cmyk', 12)):
                yield R('wrap-bare', f'colr-{nm}', wrap(b, s['w'], s['h'], s['nc'], s['ssiz'], b'', colr_enum(v)))
            yield R('wrap-bare', 'no-ftyp', wrap(b, s['w'], s['h'], s['nc'], s['ssiz'], ftyp=False))
            yield R('wrap-bare', 'ihdr-width-plus1', wrap(b, s['w'] + 1, s['h'], s['nc'], s['ssiz']))
    # codestream marker structure
    off = codestream_off(b)
    if off is not None:
        ms = list(markers(b, off))
        for m, p, L in ms:
            if L and m not in (0xff51,):
                seg = b[p:p + 2 + L]
                yield R('marker-delete', f'{m:04x}', b[:p] + b[p + 2 + L:])
                yield R('marker-duplicate', f'{m:04x}', b[:p + 2 + L] + seg + b[p + 2 + L:])
            elif m == 0xff93:
                pass
        com = b'\xff\x64' + struct.pack('>H', 6) + b'\0\1xx'
        sot = [p for m, p, L in ms if m == 0xff90]
        ins = sot[0] if sot else (len(b) - 2)
        yield R('marker-insert', 'COM-before-SOT', b[:ins] + com + b[ins:])
        yield R('marker-insert', 'unknown-FF30-seg', b[:ins] + b'\xff\x31\x00\x04ab' + b[ins:])
        yield R('marker-insert', 'reserved-FF4E-seg', b[:ins] + b'\xff\x4e\x00\x04ab' + b[ins:])
        if b[-2:] == b'\xff\xd9':
            yield R('eoc', 'drop-EOC', b[:-2])
            yield R('eoc', 'double-EOC', b + b'\xff\xd9')
        if sot:
            p = sot[0]
            yield R('sot-psot', 'zero-to-eoc', b[:p + 6] + b'\0\0\0\0' + b[p + 10:])
            yield R('sot-psot', 'one-more', b[:p + 6] + struct.pack('>I', c.u(b, p + 6, 4, 'big') + 1) + b[p + 10:])
            yield R('sot-psot', 'one-less', b[:p + 6] + struct.pack('>I', max(0, c.u(b, p + 6, 4, 'big') - 1)) + b[p + 10:])
            yield R('sot-psot', 'huge', b[:p + 6] + b'\xff\xff\xff\xff' + b[p + 10:])
            sod = [q for m, q, L in ms if m == 0xff93]
            if sod:
                body = b[sod[0] + 2:]
                for k in (1, 2, 5, 17):
                    if len(body) > k + 2:
                        yield R('tile-data-truncate', f'-{k}', b[:len(b) - 2 - k] + b[-2:])
        # CBLK / packet-data corruption
        if sot:
            for _ in range(8):
                i = rng.randrange(sot[0] + 14, max(sot[0] + 15, len(b) - 2))
                yield R('packet-data-byte', f'off{i - sot[0]}', b[:i] + bytes([rng.randrange(256)]) + b[i + 1:], b[i], None)


def mutations(data, meta, rng):
    yield from c.field_mutations(data, fields(data), rng=rng, per_field=6)
    yield from c.generic_mutations(data, rng, cut_points=cut_points(data), nflip=22)
    yield from structural(data, rng)


def seeds():
    base = {p.name: p.read_bytes() for p in sorted((HERE / 'seeds').glob('*')) if not p.name.startswith('py-')}
    out = [(d, {'name': n.rsplit('.', 1)[0], 'kind': 'jp2' if n.endswith('jp2') else 'j2k'}) for n, d in base.items()]
    out += [(d, {'name': n, 'kind': 'jp2'}) for n, d in python_seeds(base)]
    return out


class JP2(c.Adapter):
    fmt, ext, title = 'jp2', 'jp2', 'JPEG 2000 (image/jp2, image/x-jp2-codestream)'
    tol = 3
    dir = HERE
    runners = {'glycin': (RUST, ['jp2:glycin']), 'hayro': (RUST, ['jp2:hayro']), 'hayro-strict': (RUST, ['jp2:hayro-strict']),
               'openjpeg': (BIN / 'openjpeg', ['default']), 'openjpeg-nonstrict': (BIN / 'openjpeg', ['nonstrict']), 'jasper': (BIN / 'jasper', ['x'])}
    groups = {'glycin': 'hayro_jpeg2000', 'hayro': 'hayro_jpeg2000', 'hayro-strict': 'hayro_jpeg2000',
              'openjpeg': 'openjpeg', 'openjpeg-nonstrict': 'openjpeg', 'jasper': 'jasper'}
    decoders_doc = {
        'glycin': 'glycin-image-rs `ImgLoader` for `image/jp2` / `image/x-jp2-codestream` (hayro_jpeg2000 0.3.5 `integration::Jp2Decoder`, default settings)',
        'hayro': 'hayro-jpeg2000 0.3.5 called directly, `strict=false` (glycin default)',
        'hayro-strict': 'hayro-jpeg2000 with `strict=true`',
        'openjpeg': 'OpenJPEG 2.5.x (github master, built with gcc/cmake) `opj_decode`, default strict mode; runner-side RGBA8 normalisation',
        'openjpeg-nonstrict': 'OpenJPEG with `opj_decoder_set_strict_mode(false)` (accepts truncated codestreams)',
        'jasper': 'JasPer 4.2.9 `jas_image_decode`, gray/RGB(+opacity) only; runner-side RGBA8 normalisation'}
    unavailable = [('gdk-pixbuf JP2/J2K module', 'not installed'),
                   ('subsampled/YCC/CMYK colour conversion in the native runners', 'OpenJPEG and JasPer rows report `skip` (n/a) for sYCC/eYCC/CMYK/ICC-only/>4-component outputs: no colour conversion is implemented in the runners, so those inputs do not vote for those columns'),
                   ('`jpeg2k` / `openjp2` Rust crates', 'wrap or port OpenJPEG: not independent of the OpenJPEG column, not built')]
    notes = (open(HERE / 'NOTES.md').read() if (HERE / 'NOTES.md').exists() else '')
    seeds = staticmethod(seeds)
    mutations = staticmethod(mutations)


if __name__ == '__main__':
    c.run(JP2())
