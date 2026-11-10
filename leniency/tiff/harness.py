#!/usr/bin/env python3
"""TIFF (image/tiff) leniency adapter. Seeds come from tiffgen.py (synthetic only);
mutations are IFD-aware and record IFD index, tag, field, old and new values."""
import random
import struct
import sys
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.append(str(HERE.parent / 'jpeg'))
import jt_harness as jt  # noqa: E402
import tiffgen as tg  # noqa: E402
from tiffgen import Page  # noqa: E402

RUST = jt.BIN / 'leniency-jt-runner'
W, H = 13, 7


def seed_specs():
    """(name, pages, endian, bigtiff, orientation)."""
    s = []
    def add(name, pages, endian='<', big=False, orientation=None):
        s.append((name, pages if isinstance(pages, list) else [pages], endian, big, orientation))
    for e, en in (('<', 'II'), ('>', 'MM')):
        for comp, cn in ((1, 'none'), (32773, 'packbits'), (5, 'lzw'), (8, 'deflate'), (32946, 'deflate-old')):
            add(f'rgb8-{cn}-{en}', Page(W, H, 2, 8, 3, compression=comp, rows=3, seed=1), e)
        add(f'gray8-none-{en}', Page(W, H, 1, 8, 1, seed=2), e)
        add(f'gray16-none-{en}', Page(W, H, 1, 16, 1, seed=3), e)
        add(f'rgb16-lzw-pred2-{en}', Page(W, H, 2, 16, 3, compression=5, predictor=2, seed=4), e)
        add(f'rgb-float32-pred3-{en}', Page(W, H, 2, 32, 3, fmt='float', compression=8, predictor=3, seed=5), e)
        add(f'bigtiff-rgb8-none-{en}', Page(W, H, 2, 8, 3, seed=6), e, True)
        add(f'bigtiff-gray8-lzw-{en}', Page(W, H, 1, 8, 1, compression=5, seed=7), e, True)
    add('rgb8-lzw-pred2', Page(W, H, 2, 8, 3, compression=5, predictor=2, seed=8))
    add('rgb8-deflate-pred2', Page(W, H, 2, 8, 3, compression=8, predictor=2, seed=9))
    add('rgb8-lzw-big-48x40', Page(48, 40, 2, 8, 3, compression=5, seed=10))
    add('rgb8-rows1', Page(W, H, 2, 8, 3, rows=1, seed=11))
    for bps in (1, 2, 4):
        add(f'gray{bps}-minisblack', Page(W, H, 1, bps, 1, seed=12 + bps))
        add(f'palette{bps}', Page(W, H, 3, bps, 1, seed=20 + bps))
    add('bilevel-miniswhite-packbits', Page(W, H, 0, 1, 1, compression=32773, seed=17))
    add('bilevel-miniswhite-fillorder2', Page(W, H, 0, 1, 1, fill_order=2, seed=18))
    add('gray8-miniswhite', Page(W, H, 0, 8, 1, seed=19))
    add('palette8', Page(W, H, 3, 8, 1, seed=25))
    add('palette8-lzw', Page(W, H, 3, 8, 1, compression=5, seed=26))
    add('gray32-uint', Page(W, H, 1, 32, 1, seed=27))
    add('gray-float16', Page(W, H, 1, 16, 1, fmt='float', seed=28))
    add('gray-float32', Page(W, H, 1, 32, 1, fmt='float', seed=29))
    add('gray-float64', Page(W, H, 1, 64, 1, fmt='float', seed=30))
    add('gray-int8', Page(W, H, 1, 8, 1, fmt='int', seed=31))
    add('gray-int16', Page(W, H, 1, 16, 1, fmt='int', seed=32))
    add('rgb32-uint', Page(W, H, 2, 32, 3, seed=33))
    add('rgb-float32', Page(W, H, 2, 32, 3, fmt='float', seed=34))
    add('rgb-float16', Page(W, H, 2, 16, 3, fmt='float', seed=35))
    add('rgb-float64-pred3', Page(W, H, 2, 64, 3, fmt='float', compression=8, predictor=3, seed=36))
    for extra, en in ((0, 'unspecified'), (1, 'associated'), (2, 'unassociated')):
        add(f'rgba8-{en}', Page(W, H, 2, 8, 4, extra=extra, seed=37 + extra))
        add(f'graya8-{en}', Page(W, H, 1, 8, 2, extra=extra, seed=40 + extra))
    add('rgba16-unassociated', Page(W, H, 2, 16, 4, extra=2, seed=43))
    add('rgba-float32-associated', Page(W, H, 2, 32, 4, fmt='float', extra=1, seed=44))
    add('cmyk8', Page(W, H, 5, 8, 4, ink=1, seed=45))
    add('cmyk16', Page(W, H, 5, 16, 4, ink=1, seed=46))
    add('cmyka8', Page(W, H, 5, 8, 5, extra=2, ink=1, seed=47))
    add('cmyk8-lzw', Page(W, H, 5, 8, 4, compression=5, seed=48))
    add('ycbcr8-1x1', Page(W, H, 6, 8, 3, ycbcr_sub=(1, 1), seed=49))
    add('ycbcr8-2x2', Page(14, 8, 6, 8, 3, ycbcr_sub=(2, 2), seed=50))
    add('cielab8', Page(W, H, 8, 8, 3, seed=51))
    add('jpeg-ycbcr-2x2', Page(32, 16, 6, 8, 3, compression=7, ycbcr_sub=(2, 2), seed=52))
    add('jpeg-ycbcr-2x2-strips16', Page(32, 32, 6, 8, 3, compression=7, rows=16, ycbcr_sub=(2, 2), seed=53))
    add('jpeg-gray', Page(24, 16, 1, 8, 1, compression=7, seed=54))
    add('jpeg-rgb', Page(24, 16, 2, 8, 3, compression=7, seed=55))
    add('jpeg-ycbcr-tiled', Page(32, 32, 6, 8, 3, compression=7, tile=(16, 16), ycbcr_sub=(2, 2), seed=56))
    add('planar-rgb8', Page(W, H, 2, 8, 3, planar=2, seed=57))
    add('planar-rgb16-lzw', Page(W, H, 2, 16, 3, planar=2, compression=5, seed=58))
    add('planar-rgba8', Page(W, H, 2, 8, 4, planar=2, extra=2, seed=59))
    add('planar-rgb8-rows3', Page(W, H, 2, 8, 3, planar=2, rows=3, seed=60))
    add('tiled-rgb8', Page(40, 24, 2, 8, 3, tile=(16, 16), seed=61))
    add('tiled-gray8-lzw', Page(40, 24, 1, 8, 1, tile=(16, 16), compression=5, seed=62))
    add('tiled-planar-rgb8', Page(40, 24, 2, 8, 3, tile=(16, 16), planar=2, seed=63))
    add('tiled-rgb16-deflate-MM', Page(40, 24, 2, 16, 3, tile=(16, 16), compression=8, seed=64), '>')
    for o in range(1, 9):
        add(f'orientation-{o}', Page(W, H, 2, 8, 3, orientation=o, seed=65), orientation=o)
    add('orientation-6-MM-lzw', Page(W, H, 2, 8, 3, orientation=6, compression=5, seed=66), '>', orientation=6)
    add('multipage-2', [Page(W, H, 2, 8, 3, seed=67), Page(9, 5, 1, 8, 1, seed=68)])
    add('multipage-thumbnail-first', [Page(5, 3, 2, 8, 3, subfile=1, seed=69), Page(W, H, 2, 8, 3, subfile=0, seed=70)])
    add('multipage-3-mixed-MM', [Page(W, H, 1, 16, 1, seed=71), Page(W, H, 3, 4, 1, seed=72), Page(W, H, 5, 8, 4, seed=73)], '>')
    add('bigtiff-multipage', [Page(W, H, 2, 8, 3, seed=74), Page(7, 7, 1, 8, 1, seed=75)], '<', True)
    add('one-pixel', Page(1, 1, 2, 8, 3, seed=76))
    return s


def seeds():
    out = []
    for name, pages, endian, big, orientation in seed_specs():
        data = tg.write(pages, endian, big)
        meta = {'source': 'synthetic', 'name': name, 'rule': 'seed', 'generator': 'tiffgen',
                'pages': [{k: v for k, v in p.__dict__.items() if v is not None} for p in pages], 'byte_order': 'II' if endian == '<' else 'MM', 'bigtiff': big}
        if orientation:
            meta['orientation'] = orientation
        out.append((data, meta))
    return out


TAG = {254: 'NewSubfileType', 256: 'ImageWidth', 257: 'ImageLength', 258: 'BitsPerSample', 259: 'Compression', 262: 'Photometric',
       266: 'FillOrder', 273: 'StripOffsets', 274: 'Orientation', 277: 'SamplesPerPixel', 278: 'RowsPerStrip', 279: 'StripByteCounts',
       282: 'XResolution', 283: 'YResolution', 284: 'PlanarConfig', 296: 'ResolutionUnit', 317: 'Predictor', 320: 'ColorMap',
       322: 'TileWidth', 323: 'TileLength', 324: 'TileOffsets', 325: 'TileByteCounts', 332: 'InkSet', 338: 'ExtraSamples',
       339: 'SampleFormat', 530: 'YCbCrSubSampling', 532: 'ReferenceBlackWhite'}
SEMANTIC = {256: [0, 1, 2, 'old-1', 'old+1', 65535, 65536, 0xFFFFFFFF], 257: [0, 1, 'old-1', 'old+1', 65535, 0xFFFFFFFF],
            258: [0, 1, 2, 3, 4, 7, 8, 12, 16, 24, 32, 64, 255], 259: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 32773, 32946, 34712, 34887, 50000, 65535],
            262: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 32844, 65535], 277: [0, 1, 2, 3, 4, 5, 6, 255, 65535],
            278: [0, 1, 'old-1', 'old+1', 0xFFFF, 0xFFFFFFFF], 284: [0, 1, 2, 3, 65535], 317: [0, 1, 2, 3, 4, 65535],
            274: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 65535], 338: [0, 1, 2, 3, 65535], 339: [0, 1, 2, 3, 4, 5, 6, 65535],
            266: [0, 1, 2, 3], 322: [0, 1, 8, 15, 16, 17, 32, 65535], 323: [0, 1, 8, 15, 16, 17, 32, 65535], 530: [0, 1, 2, 3, 4, 8],
            254: [0, 1, 2, 4, 0xFFFFFFFF], 332: [0, 1, 2, 3]}


def variants(b, rng):
    info = tg.parse(b)
    L = len(b)
    e = info['endian'] if info else '<'
    big = info['big'] if info else False
    def mk(data, rule, field, value, old=None, ifd=0):
        m = {'rule': rule, 'field': field, 'value': value, 'ifd': ifd}
        if old is not None:
            m['old'] = old
        return data, m
    def put(pos, val, size, data=b):
        if val < 0 or val >= 1 << (8 * size):
            return None
        a = bytearray(data)
        a[pos:pos + size] = struct.pack(e + {1: 'B', 2: 'H', 4: 'I', 8: 'Q'}[size], val)
        return bytes(a)
    def emit(data, *args, **kw):
        return None if data is None else mk(data, *args, **kw)
    out = []
    # Header.
    out.append(mk(b'MM' + b[2:] if b[:2] == b'II' else b'II' + b[2:], 'header-byte-order', 'order', 'swapped'))
    out.append(mk(b'IM' + b[2:], 'header-byte-order', 'order', 'IM'))
    for v in (0, 41, 43, 44, 0x2A00) if not big else (42, 0, 44):
        out.append(emit(put(2, v, 2), 'header-magic', 'magic', v, 43 if big else 42))
    first = 8 if big else 4
    osz = 8 if big else 4
    ifd0 = info['ifds'][0]['offset'] if info and info['ifds'] else 0
    for v in (0, 1, 2, 7, ifd0 - 1, ifd0 + 1, ifd0 + 2, L, L + 1, (1 << (8 * osz)) - 1):
        if v != ifd0:
            out.append(emit(put(first, v, osz), 'header-ifd-offset', 'IFD0', v, ifd0))
    if big:
        for v in (4, 0, 16):
            out.append(emit(put(4, v, 2), 'bigtiff-offset-size', 'bytesize', v, 8))
    # Truncation at structural boundaries and inside data, trailing bytes.
    cuts = {4, 8, 16, L // 2, L - 1, ifd0, ifd0 + 1, ifd0 + 2 + 3}
    for d in info['ifds'] if info else []:
        cuts |= {d['offset'], d['next_pos'], d['entries'][len(d['entries']) // 2]['pos'] if d['entries'] else d['offset']}
    if info:
        for ent in info['ifds'][0]['entries']:
            if ent['tag'] in (273, 324):
                offs, _ = tg.values(b, info, ent)
                for o in offs[:2]:
                    cuts |= {o, o + 1, o + 4}
    for c in sorted(x for x in cuts if 0 < x < L):
        out.append(mk(b[:c], 'truncation', 'length', c))
    for n in (1, 4, 64, 4096):
        out.append(mk(b + bytes(n), 'trailing-bytes', 'zeros', n))
    # Per-IFD structure.
    for k, d in enumerate(info['ifds'] if info else []):
        cnt_size = 8 if big else 2
        n = len(d['entries'])
        for v in (0, 1, n - 1, n + 1, n + 10, 0xFFFF):
            if v != n:
                out.append(emit(put(d['offset'], v, cnt_size), 'ifd-entry-count', 'count', v, n, ifd=k))
        for v in (d['offset'], ifd0, 1, L, L - 2, (1 << (8 * osz)) - 1):
            out.append(emit(put(d['next_pos'], v, osz), 'ifd-next-offset', 'next', 'self' if v == d['offset'] else ('IFD0' if v == ifd0 else v), ifd=k))
        if n >= 2:
            a = bytearray(b)
            es = 20 if big else 12
            p0, p1 = d['entries'][0]['pos'], d['entries'][-1]['pos']
            a[p0:p0 + es], a[p1:p1 + es] = b[p1:p1 + es], b[p0:p0 + es]
            out.append(mk(bytes(a), 'ifd-unsorted', 'swap-first-last', 'swap', ifd=k))
            q = d['entries'][1]['pos']
            a = bytearray(b)
            a[q:q + es] = b[p0:p0 + es]
            out.append(mk(bytes(a), 'ifd-duplicate-tag', TAG.get(d['entries'][0]['tag'], d['entries'][0]['tag']), 'dup', ifd=k))
        for ent in d['entries']:
            tag, name = ent['tag'], TAG.get(ent['tag'], str(ent['tag']))
            q = ent['pos']
            out.append(emit(put(q, 65000, 2), 'tag-removed', name, 65000, tag, ifd=k))
            for t in (0, 1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 18, 255, 65535):
                if t != ent['type']:
                    out.append(emit(put(q + 2, t, 2), 'tag-type', name, t, ent['type'], ifd=k))
            csz = 8 if big else 4
            for v in (0, 1, ent['count'] - 1, ent['count'] + 1, ent['count'] * 2, 0xFFFF, 0x7FFFFFFF, (1 << (8 * csz)) - 1):
                if v != ent['count']:
                    out.append(emit(put(q + 4, v, csz), 'tag-count', name, v, ent['count'], ifd=k))
            vals, vpos = tg.values(b, info, ent)
            if tag in SEMANTIC and vals:
                size = tg.SIZE[ent['type']]
                for v in SEMANTIC[tag]:
                    old = vals[0]
                    v = old - 1 if v == 'old-1' else old + 1 if v == 'old+1' else v
                    if v != old:
                        data = put(vpos, v, size)
                        if data is not None and len(vals) > 1 and tag in (258, 339):
                            for j in range(1, len(vals)):
                                data = put(vpos + j * size, v, size, data)
                        out.append(emit(data, 'field-' + name, name, v, old, ifd=k))
            if tag in (273, 324, 279, 325) and vals:
                size = tg.SIZE[ent['type']]
                for j in range(min(2, len(vals))):
                    old = vals[j]
                    choices = (0, 1, old - 1, old + 1, L, L + 1, (1 << (8 * size)) - 1) if tag in (273, 324) else (0, 1, old - 1, old + 1, old * 2, L, (1 << (8 * size)) - 1)
                    for v in choices:
                        if v != old:
                            out.append(emit(put(vpos + j * size, v, size), 'strip-' + ('offset' if tag in (273, 324) else 'bytecount'), f'{name}[{j}]', v, old, ifd=k))
            if tag not in (273, 279, 324, 325) and ent['count'] * tg.SIZE.get(ent['type'], 1) > (8 if big else 4):
                for v in (0, 1, L - 1, L, (1 << (8 * osz)) - 1):
                    out.append(emit(put(ent['value_pos'], v, osz), 'value-offset', name, v, ifd=k))
    # Random single-bit flips over the whole file and inside the first IFD.
    for j in range(24):
        if info and info['ifds'] and j % 2:
            d = info['ifds'][0]
            lo, hi = d['offset'], d['next_pos']
        else:
            lo, hi = 0, L
        pos = rng.randrange(lo, max(lo + 1, hi))
        a = bytearray(b)
        a[pos] ^= 1 << rng.randrange(8)
        out.append(mk(bytes(a), 'byte-flip-ifd' if j % 2 else 'byte-flip', 'offset', pos))
    return [x for x in out if x is not None]


def interleave(lst):
    buckets = {}
    for data, meta in lst:
        buckets.setdefault(meta['rule'], []).append((data, meta))
    queues = list(buckets.values())
    k = 0
    while queues:
        for q in queues:
            yield q[k]
        k += 1
        queues = [q for q in queues if k < len(q)]


def generate(limit, seed):
    rng = random.Random(seed)
    base = seeds()
    series = list(base)
    rng.shuffle(series)
    items = base[:limit]
    its = [interleave(variants(b, rng)) for b, _ in series]
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


class Tiff(jt.Adapter):
    fmt, ext = 'tiff', 'tif'
    runners = {
        'glycin': (RUST, ['tiff:glycin']),
        'image': (RUST, ['tiff:image']),
        'tiff-crate': (RUST, ['tiff:crate']),
        'libtiff': (jt.BIN / 'libtiff-runner', []),
        'gdk-file': (jt.BIN / 'gdk-jt', ['tiff', 'file']),
        'gdk-chunk': (jt.BIN / 'gdk-jt', ['tiff', 'forced-chunk']),
    }
    families = {'tiff crate 0.11.3': ['glycin', 'image', 'tiff-crate'], 'libtiff 4.7.0': ['libtiff', 'gdk-file', 'gdk-chunk']}
    unavailable = {
        'second independent TIFF reader (Pillow, ImageMagick, tifffile)': 'none installed; PyPI is outside the allowed network',
        'libtiff tools (tiffinfo/tiffcp)': 'not installed; the libtiff C API runner covers libtiff',
    }

    def generate(self, limit, seed):
        return generate(limit, seed)


AD = Tiff()
AD.notes = [
    'glycin column: `glycin_image_rs::ImgLoader` for `image/tiff` (tiff crate `read_image_to_buffer`, first plane buffer only, then '
    'glycin\'s memory-format table) followed by glycin\'s `change_memory_format` to R8g8b8a8. glycin-core would apply the TIFF Orientation '
    'tag (it parses the whole TIFF as Exif); the column reports it without applying it. image column: `image::codecs::tiff::TiffDecoder` → '
    '`to_rgba8`. tiff-crate column: the tiff crate used directly with the harness\'s own normalisation (all planes, sub-byte unpacking, '
    'palette via ColorMap>>8, naive CMYK, JFIF YCbCr). libtiff: `TIFFReadRGBAImageOriented(ORIENTATION_TOPLEFT)`, which **applies** '
    'Orientation and premultiplies alpha; the runner un-premultiplies. GDK: `libpixbufloader-tiff.so` by path and a forced `tiff` loader '
    'fed 4096-byte writes (GDK buffers the whole file and also uses libtiff\'s RGBA interface). Only the first IFD is compared; the '
    '`pages` field records how many IFDs libtiff and the tiff crate walk.',
]

AD.findings = [
    'PlanarConfig=2 (separate planes, strips or tiles): glycin returns pixels built from the first plane only and differs from image, the tiff crate used directly and libtiff by 241-251 per channel; those three agree. image uses the same tiff crate, so the difference is in glycin\'s glue (`as_buffer(0)`).',
    '1-, 2- and 4-bit grayscale and bilevel images are rejected by glycin ("ColorType Gray(n) not supported"); image accepts only 1-bit; the tiff crate and libtiff decode all of them.',
    'Palette images (Photometric 3, 1/2/4/8-bit), CIELab, uncompressed 2x2-subsampled YCbCr and CCITT compression are rejected by the whole tiff-rs family (glycin, image, tiff crate) and decoded by libtiff and GDK.',
    'YCbCr (uncompressed 1x1 and JPEG-compressed, strips and tiles): the tiff crate decodes and agrees with libtiff within IDCT tolerance; glycin and image reject the YCbCr colour type.',
    'Float and 32-bit samples: libtiff\'s RGBA interface (and so GDK) rejects every float and 32-bit uint image; the tiff crate decodes all of them; glycin accepts RGB float16/32 but rejects gray float, gray32 and float64; image and the tiff crate disagree on 32-bit uint RGB by up to 252.',
    'Orientation: libtiff\'s RGBA path applies tags 2-4 and only the flip part of 5-8 (no transpose, output keeps stored dimensions); GDK then exposes a residual orientation option (6→7, 7→5, 8→7, none for 2-4); glycin reports the tag (glycin-core would apply it), image and the tiff crate report without applying.',
    'Extra samples: with associated alpha glycin un-premultiplies (matching libtiff after the runner un-premultiplies) while image and the tiff crate return stored values; with unassociated alpha GDK differs from all others by up to 133; with ExtraSamples=0 the three families give three different results.',
    'Structural leniency of libtiff: an IFD next-offset pointing at itself, a removed or out-of-range Photometric tag, and IFD byte flips still decode the first page in libtiff and GDK, while the tiff-rs family rejects the whole file (138 / 93 / 151 changed inputs).',
    'Leniency of tiff-rs: StripByteCounts smaller than the compressed strip (e.g. 1) and ExtraSamples=3 are accepted by glycin/image/tiff crate but rejected by libtiff and GDK (53 / 36).',
    'GDK path differences: big-endian BigTIFF is rejected by the path loader (format detection) but decoded by the forced loader; the forced 4096-byte loader accepts some invalid ImageLength/RowsPerStrip/PlanarConfig values that libtiff and the path loader reject, and rejects a tiled RGB seed that the path loader decodes. The tiff-rs columns crashed on 33 inputs (including one unmutated tiled planar seed); counted only.',
]
AD.public_findings = AD.findings

if __name__ == '__main__':
    jt.main(AD)
