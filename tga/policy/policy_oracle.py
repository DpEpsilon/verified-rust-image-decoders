#!/usr/bin/env python3
"""Independent bounded TGA policy oracle (stdlib only).

Implements TGA-POLICY-SPEC (POLICY-SPEC.md in this directory) as an extension
of frozen TGA-CORE-SPEC 1.0.  The Strict profile reproduces all 169 frozen
vectors.  A compatibility profile is a byte-semantics proposal, not a wrapper
around a native decoder; library I/O, sniffing and allocator effects are
excluded.  Written from SPEC.md and POLICY-SPEC.md, without importing the
Verus crate or the frozen check_vectors.py.
"""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
FROZEN = HERE.parent / 'spec-frozen' / 'VECTORS.json'
MAX_INPUT = MAX_OUTPUT = 67_108_864
MAX_DIMENSION = 8192
ERRORS = ('FileTooLarge TruncatedHeader UnsupportedImageType InvalidColorMapFlag '
          'InvalidDimensions UnsupportedInterleave UnsupportedPixelDepth InvalidColorMap '
          'ColorMapRange InvalidAttributeBits OutputTooLarge TruncatedImageId '
          'TruncatedColorMap TruncatedPixels RleTruncated RleOverrun '
          'ColorIndexOutOfRange TrailingBytes').split()
FOOTER_SIGNATURE = b'TRUEVISION-XFILE.\x00'

OPTIONS = {
    'trailing': ('FooterOnly', 'Ignore'),
    'rle_overrun': ('Reject', 'ClipPacket', 'ClipSamples'),
    'attribute_bits': ('Exact', 'Lenient', 'ImageTable'),
    'map_header': ('Core', 'IgnoreAbsent', 'Gdk', 'Image'),
    'depths': ('Core', 'Gdk', 'Image'),
    'alpha': ('AttributeBits', 'Stored'),
    'map16_alpha': ('Bit15', 'Opaque'),
    'rgb5': ('Replicate', 'Round'),
    'map5': ('Replicate', 'Round', 'GdkSwapShift'),
    'map_index': ('Absolute', 'ZeroBased'),
    'index_range': ('Reject', 'Black'),
    'interleave': ('Reject', 'Ignore'),
    'image_type': ('Core', 'GreyFallback'),
}
STRICT = {k: v[0] for k, v in OPTIONS.items()}
RECOMMENDED = dict(STRICT, trailing='Ignore', rle_overrun='ClipPacket',
                   attribute_bits='Lenient', map_header='IgnoreAbsent')
GDK = dict(STRICT, trailing='Ignore', rle_overrun='ClipPacket', attribute_bits='Lenient',
           map_header='Gdk', depths='Gdk', alpha='Stored', map16_alpha='Opaque',
           map5='GdkSwapShift', map_index='ZeroBased', index_range='Black')
IMAGE = dict(STRICT, trailing='Ignore', rle_overrun='ClipSamples', attribute_bits='ImageTable',
             map_header='Image', depths='Image', alpha='Stored', map16_alpha='Opaque',
             rgb5='Round', map5='Round', interleave='Ignore', image_type='GreyFallback')
PROFILES = {'Strict': STRICT, 'Recommended': RECOMMENDED,
            'GdkCompat-2.42.12': GDK, 'ImageCrateCompat-0.25.10': IMAGE}
# Custom single-field records are internal reference-model probes only.
EXPERIMENTAL_OPTIONS = True

# R5 allowed attribute counts, keyed by (element class, depth).  'map' entries
# are keyed by map-entry depth; true-colour/grey by image depth.
CORE_ATTR = {('true', 15): (0,), ('true', 16): (0, 1), ('true', 24): (0,), ('true', 32): (0, 8),
             ('grey', 8): (0,), ('grey', 16): (0, 8),
             ('map', 15): (0,), ('map', 16): (0, 1), ('map', 24): (0,), ('map', 32): (0, 8)}
IMAGE_ATTR = dict(CORE_ATTR)
IMAGE_ATTR[('grey', 8)] = (0, 8)
IMAGE_ATTR[('grey', 16)] = (8,)

ROUND5 = [(255 * c + 15) // 31 for c in range(32)]
REPLICATE5 = [(c << 3) | (c >> 2) for c in range(32)]


class Invalid(Exception):
    pass


def fail(name):
    assert name in ERRORS, name
    raise Invalid(name)


def le16(b, p):
    return b[p] | (b[p + 1] << 8)


def _decode(b, policy):
    """Pure-byte model; returns (width, height, rgba, recovered) or raises Invalid."""
    if set(policy) != set(OPTIONS) or any(policy[k] not in OPTIONS[k] for k in OPTIONS):
        raise ValueError('unknown internal policy record')
    if not EXPERIMENTAL_OPTIONS and policy not in PROFILES.values():
        raise ValueError('experimental option records disabled')
    P = policy
    recovered = False
    # R1
    if len(b) > MAX_INPUT:
        fail('FileTooLarge')
    if len(b) < 18:
        fail('TruncatedHeader')
    id_len, flag, itype = b[0], b[1], b[2]
    first, length, edepth = le16(b, 3), le16(b, 5), b[7]
    width, height, depth, desc = le16(b, 12), le16(b, 14), b[16], b[17]
    if itype in (1, 2, 3, 9, 10, 11):
        kind = {1: 'map', 2: 'true', 3: 'grey'}[itype & 3]
        rle = itype >= 9
    elif P['image_type'] == 'GreyFallback':
        kind, rle = 'grey', False  # image-rs decodes every other type as raw grey
    else:
        fail('UnsupportedImageType')
    # R2
    if P['map_header'] in ('Core', 'IgnoreAbsent'):
        if flag not in (0, 1):
            fail('InvalidColorMapFlag')
        present = flag == 1
    elif P['map_header'] == 'Gdk':
        present = flag != 0
    else:
        present = flag == 1
    if not (1 <= width <= MAX_DIMENSION and 1 <= height <= MAX_DIMENSION):
        fail('InvalidDimensions')
    if desc & 0xc0 and P['interleave'] == 'Reject':
        fail('UnsupportedInterleave')
    # R3
    supported = {'Core': {'map': (8, 16), 'true': (15, 16, 24, 32), 'grey': (8, 16)},
                 'Gdk': {'map': (8,), 'true': (16, 24, 32), 'grey': (8, 16)},
                 'Image': {'map': (8, 16), 'true': (15, 16, 24, 32), 'grey': (8, 16)}}[P['depths']]
    if depth not in supported[kind]:
        fail('UnsupportedPixelDepth')
    # R4
    mh = P['map_header']
    if mh == 'Core':
        if not present:
            if first or length or edepth or kind == 'map':
                fail('InvalidColorMap')
        elif length == 0 or edepth not in (15, 16, 24, 32):
            fail('InvalidColorMap')
        if present and first + length > 65536:
            fail('ColorMapRange')
    elif mh == 'IgnoreAbsent':
        if not present:
            if kind == 'map':
                fail('InvalidColorMap')
        elif length == 0 or edepth not in (15, 16, 24, 32):
            fail('InvalidColorMap')
        if present and first + length > 65536:
            fail('ColorMapRange')
    elif mh == 'Gdk':
        if not present and kind == 'map':
            fail('InvalidColorMap')
        if present and length and edepth not in (15, 16, 24, 32):
            fail('InvalidColorMap')
    else:  # Image
        if not present and kind == 'map':
            fail('InvalidColorMap')
        if present and edepth not in (15, 16, 24, 32):
            fail('InvalidColorMap')
    if P['depths'] == 'Image' and kind == 'map' and depth > edepth:
        fail('UnsupportedPixelDepth')
    # R5
    count = desc & 15
    element = ('map', edepth) if kind == 'map' else (kind, depth)
    table = IMAGE_ATTR if P['attribute_bits'] == 'ImageTable' else CORE_ATTR
    allowed = table.get(element, (0,))
    if count in allowed:
        eff = count
    elif P['attribute_bits'] == 'Lenient':
        # an 8-bit attribute slot (32-bit colour, 16-bit grey) keeps its stored
        # alpha; every other element is treated as having no attribute bits
        eff = 8 if 8 in allowed else 0
    else:
        fail('InvalidAttributeBits')
    # R6
    n = width * height
    if 4 * n > MAX_OUTPUT:
        fail('OutputTooLarge')
    # R7
    pos = 18
    if len(b) - pos < id_len:
        fail('TruncatedImageId')
    pos += id_len
    ebytes = (edepth + 7) // 8
    map_bytes = length * ebytes if present else 0
    if len(b) - pos < map_bytes:
        fail('TruncatedColorMap')
    map_start = pos
    pos += map_bytes
    sbytes = (depth + 7) // 8
    stored_alpha = P['alpha'] == 'Stored' or eff == 8

    def five(v, rule):
        lut = ROUND5 if rule == 'Round' else REPLICATE5
        return lut[(v >> 10) & 31], lut[(v >> 5) & 31], lut[v & 31]

    # R8 conversion of one sample (bytes) to RGBA
    cache = {}

    def convert(s):
        nonlocal recovered
        got = cache.get(s)
        if got is not None:
            return got
        if kind == 'true':
            if depth in (15, 16):
                r, g, bl = five(s[0] | (s[1] << 8), P['rgb5'])
                px = bytes((r, g, bl, 255))
            elif depth == 24:
                px = bytes((s[2], s[1], s[0], 255))
            else:
                px = bytes((s[2], s[1], s[0], s[3] if stored_alpha else 255))
        elif kind == 'grey':
            px = bytes((s[0], s[0], s[0], s[1] if depth == 16 and stored_alpha else 255))
        else:
            index = s[0] if depth == 8 else s[0] | (s[1] << 8)
            j = index - first if P['map_index'] == 'Absolute' else index
            if not (0 <= j < length):
                if P['index_range'] == 'Reject':
                    fail('ColorIndexOutOfRange')
                recovered = True
                return bytes((0, 0, 0, 0 if edepth == 32 else 255))  # not cached: sets flag
            e = map_start + j * ebytes
            if edepth in (15, 16):
                v = b[e] | (b[e + 1] << 8)
                if P['map5'] == 'GdkSwapShift':
                    r, g, bl = (v & 31) << 3, ((v >> 5) & 31) << 3, ((v >> 10) & 31) << 3
                else:
                    r, g, bl = five(v, P['map5'])
                a = 255
                if edepth == 16 and eff == 1 and P['map16_alpha'] == 'Bit15':
                    a = 255 if v & 0x8000 else 0
                px = bytes((r, g, bl, a))
            elif edepth == 24:
                px = bytes((b[e + 2], b[e + 1], b[e], 255))
            else:
                px = bytes((b[e + 2], b[e + 1], b[e], b[e + 3] if stored_alpha else 255))
        cache[s] = px
        return px

    out = bytearray(4 * n)
    xflip = bool(desc & 0x10)
    top = bool(desc & 0x20)

    def row_base(y):
        return 4 * width * (y if top else height - 1 - y)

    def put_run(k, px, cnt):
        """R9: write cnt copies of px for input samples k..k+cnt-1."""
        while cnt:
            y, x = divmod(k, width)
            seg = min(cnt, width - x)
            x0 = (width - x - seg) if xflip else x
            at = row_base(y) + 4 * x0
            out[at:at + 4 * seg] = px * seg
            k += seg
            cnt -= seg

    def put(k, px):
        y, x = divmod(k, width)
        at = row_base(y) + 4 * ((width - 1 - x) if xflip else x)
        out[at:at + 4] = px

    produced = 0
    if not rle:
        # R10: rows are aligned with sample numbers for raw images
        for y in range(height):
            if len(b) - pos < width * sbytes:
                # convert available samples in order (index errors first), then fail
                avail = (len(b) - pos) // sbytes
                for i in range(avail):
                    convert(bytes(b[pos + i * sbytes:pos + (i + 1) * sbytes]))
                fail('TruncatedPixels')
            row = [convert(bytes(b[pos + i * sbytes:pos + (i + 1) * sbytes])) for i in range(width)]
            if xflip:
                row.reverse()
            at = row_base(y)
            out[at:at + 4 * width] = b''.join(row)
            pos += width * sbytes
        produced = n
    else:
        # R11 with the selected overrun rule
        while produced < n:
            if len(b) - pos < 1:
                fail('RleTruncated')
            control = b[pos]
            cnt = (control & 0x7f) + 1
            remaining = n - produced
            over = cnt > remaining
            if over and P['rle_overrun'] == 'Reject':
                fail('RleOverrun')
            paint = min(cnt, remaining)
            if control & 0x80:
                need = sbytes
            elif P['rle_overrun'] == 'ClipSamples':
                need = paint * sbytes
            else:
                need = cnt * sbytes
            if len(b) - pos - 1 < need:
                fail('RleTruncated')
            pos += 1
            if control & 0x80:
                put_run(produced, convert(bytes(b[pos:pos + sbytes])), paint)
            else:
                for i in range(paint):
                    put(produced + i, convert(bytes(b[pos + i * sbytes:pos + (i + 1) * sbytes])))
            pos += need
            produced += paint
            recovered |= over
    # R12
    if pos != len(b) and P['trailing'] == 'FooterOnly':
        suffix = len(b) - pos
        if suffix < 26 or b[-18:] != FOOTER_SIGNATURE:
            fail('TrailingBytes')
    return width, height, bytes(out), recovered


def decode(b):
    """Stable TGA-CORE-SPEC 1.0 API: (width, height, rgba)."""
    w, h, rgba, _ = _decode(b, STRICT)
    return w, h, rgba


def decode_with_policy(b, profile):
    """Named-profile API: (width, height, rgba, recovered)."""
    if profile not in PROFILES:
        raise ValueError('only named versioned profiles are public')
    return _decode(b, PROFILES[profile])


def runs(pixels):
    result = []
    for p in range(0, len(pixels), 4):
        h = pixels[p:p + 4].hex()
        if result and result[-1]['hex'] == h:
            result[-1]['count'] += 1
        else:
            result.append({'count': 1, 'hex': h})
    return result


def actual(b, profile='Strict', with_recovery=False):
    try:
        if isinstance(profile, str):
            w, h, rgba, rec = decode_with_policy(b, profile)
        else:
            w, h, rgba, rec = _decode(b, profile)
        result = {'width': w, 'height': h, 'rgba_runs': runs(rgba)}
        if with_recovery:
            result['recovered'] = rec
        return result
    except Invalid as e:
        return {'error': str(e)}


def expand(v):
    raw = bytes.fromhex(v['input_hex'])
    repeat = v.get('input_repeat', 1)
    length = len(raw) * repeat
    return length, None if length > MAX_INPUT else raw * repeat


def check_vector(v, profile='Strict'):
    length, b = expand(v)
    if length > MAX_INPUT:
        got = {'error': 'FileTooLarge'}  # R1 length lemma: no expansion needed
    else:
        got = actual(b, profile, 'recovered' in v['expected'])
    if got != v['expected']:
        raise AssertionError(f"{v['name']}: {v['expected']} != {got}")


def main():
    vectors = json.loads(FROZEN.read_text())
    assert len(vectors) == 169
    for v in vectors:
        check_vector(v)
    print('169 frozen TGA-CORE-SPEC 1.0 vectors passed under Strict')
    path = HERE / 'POLICY-VECTORS.json'
    if path.exists():
        pv = json.loads(path.read_text())
        for profile, entries in pv['profiles'].items():
            for v in entries:
                check_vector(v, profile)
        print('policy vectors passed:', {k: len(v) for k, v in pv['profiles'].items()})
        for row in pv['custom']:
            check_vector(row, row['policy'])
        print(len(pv['custom']), 'custom single-option probes passed')


if __name__ == '__main__':
    main()
