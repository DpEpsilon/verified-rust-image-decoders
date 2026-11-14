#!/usr/bin/env python3
"""GIF policy draft 0.2: the strict oracle (../spec-draft/gif_oracle.py) indexed by an
option record. Every option site names its OPTIONS.md field; with STRICT the result
equals gif_oracle.py on every input (check: validate.py --strict-bridge).

decode_frames(b, P) / decode_still(b, P) return gif_oracle's records plus, per frame,
'recovered': bool (OPTIONS.md, closed event list E1-E4). `loop_scan` is a metadata
field: it changes only `loop_count`, never a pixel or an error.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'spec-draft'))
from gif_oracle import (GifError, le16, need, read_sub_blocks, interlace_row,  # noqa: E402
                        MAX_INPUT, MAX_DIMENSION, MAX_OUTPUT, MAX_FRAMES, MAX_TOTAL_PIXELS)

STRICT = {
    'version': 'Known', 'frame_bounds': 'Reject', 'disposal_undefined': 'Reject',
    'gce_size': 'Exact', 'gce_duplicate': 'Reject', 'gce_scope': 'NextGraphic',
    'gce_consumers': 'Reject', 'gce_orphan': 'Reject', 'ext_sizes': 'Exact',
    'unknown_extension': 'Skip', 'unknown_block': 'Reject', 'lzw_min': 'Spec',
    'lzw_invalid': 'Reject', 'lzw_tail': 'Strict', 'short_frame': 'Reject',
    'truncation': 'Reject', 'index_range': 'Reject', 'missing_trailer': 'Reject',
    'trailing': 'Reject', 'restore_outside': 'Restore', 'image_data': 'Whole',
    'empty_first_subblock': 'Terminator',
    'loop_scan': 'EverySubBlock',                      # metadata field (OPTIONS.md, Q15)
}
RECOMMENDED_BETA = dict(STRICT, frame_bounds='Clip', disposal_undefined='Keep', gce_duplicate='LastWins',
                        gce_consumers='ImageOnly', gce_orphan='Ignore', ext_sizes='Ignore',
                        lzw_min='Range1to11Checked', lzw_tail='StopAtRaster', missing_trailer='AtEof',
                        trailing='Ignore')
GDK = dict(RECOMMENDED_BETA, gce_size='GdkFirstBlock', gce_scope='Sticky', gce_consumers='ImageOnly',
           unknown_block='SkipByte', lzw_min='Range0to11', lzw_invalid='StopFrame',
           short_frame='KeepCanvas', index_range='GdkTable', missing_trailer='Reject',
           restore_outside='Keep')
IMAGE = dict(RECOMMENDED_BETA, gce_consumers='ImageOnly', unknown_extension='Reject', lzw_min='Range1to11',
             index_range='Transparent', image_data='Streaming', empty_first_subblock='Skip',
             loop_scan='FirstSubBlockOnly')
PROFILES = {'Strict': STRICT, 'RecommendedBeta': RECOMMENDED_BETA, 'GdkCompat-2.42.12': GDK,
            'ImageCrateCompat-0.25.10': IMAGE}
VALUES = {
    'version': ('Known',), 'frame_bounds': ('Reject', 'Clip'),
    'disposal_undefined': ('Reject', 'Keep'), 'gce_size': ('Exact', 'GdkFirstBlock'),
    'gce_duplicate': ('Reject', 'LastWins'), 'gce_scope': ('NextGraphic', 'Sticky'),
    'gce_consumers': ('Reject', 'ImageOnly'), 'gce_orphan': ('Reject', 'Ignore'),
    'ext_sizes': ('Exact', 'Ignore'), 'unknown_extension': ('Skip', 'Reject'),
    'unknown_block': ('Reject', 'SkipByte'),
    'lzw_min': ('Spec', 'Range1to11', 'Range0to11', 'Range1to11Checked'),
    'lzw_invalid': ('Reject', 'StopFrame'), 'lzw_tail': ('Strict', 'StopAtRaster'),
    'short_frame': ('Reject', 'KeepCanvas'), 'truncation': ('Reject', 'KeepPartial'),
    'index_range': ('Reject', 'Transparent', 'Black', 'GdkTable'),
    'missing_trailer': ('Reject', 'AtEof'), 'trailing': ('Reject', 'Ignore'),
    'restore_outside': ('Restore', 'Keep'), 'image_data': ('Whole', 'Streaming'),
    'empty_first_subblock': ('Terminator', 'Skip'),
    'loop_scan': ('EverySubBlock', 'FirstSubBlockOnly'),
}
PIXEL_FIELDS = tuple(k for k in VALUES if k != 'loop_scan')   # the 22-field record
for _p in PROFILES.values():
    assert set(_p) == set(VALUES) and all(_p[k] in VALUES[k] for k in _p)


class Truncated(Exception):
    """Internal: image data ended at EOF under truncation=KeepPartial."""


def parse_header(b, P):
    """G2-G4 (`version` has the single value Known since Q12)."""
    if len(b) > MAX_INPUT:
        raise GifError('FileTooLarge')
    need(b, 0, 13, 'TruncatedHeader')
    if b[0:3] != b'GIF':
        raise GifError('BadSignature')
    if b[3:6] not in (b'87a', b'89a'):
        raise GifError('UnsupportedVersion')
    w, h, packed = le16(b, 6), le16(b, 8), b[10]
    if not (1 <= w <= MAX_DIMENSION and 1 <= h <= MAX_DIMENSION):
        raise GifError('InvalidDimensions')
    if 4 * w * h > MAX_OUTPUT:
        raise GifError('OutputTooLarge')
    p, gct = 13, None
    if packed & 0x80:
        n = 3 << ((packed & 7) + 1)
        need(b, p, n, 'TruncatedColorTable')
        gct = bytes(b[p:p + n])
        p += n
    return {'width': w, 'height': h, 'gct': gct}, p


def parse_extension(b, p, st, P):
    """G6/G7 with gce_size, gce_duplicate, gce_scope, gce_consumers, ext_sizes, unknown_extension."""
    need(b, p, 2, 'TruncatedExtension')
    label = b[p + 1]
    p += 2
    if label <= 0x7F and st['gce'] is not None and st['gce_fresh'] and P['gce_consumers'] == 'Reject':
        raise GifError('UnsupportedGraphicControlTarget')   # G7; ImageOnly: the GCE waits for the image
    if label == 0xF9:
        if st['gce'] is not None and st['gce_fresh'] and P['gce_duplicate'] == 'Reject':
            raise GifError('DuplicateGraphicControl')
        if P['gce_size'] == 'Exact':
            need(b, p, 1, 'TruncatedExtension')
            if b[p] != 4:
                raise GifError('InvalidGraphicControl')
            need(b, p, 6, 'TruncatedExtension')
            if b[p + 5] != 0:
                raise GifError('InvalidGraphicControl')
            body, p = b[p + 1:p + 5], p + 6
        else:                                   # GdkFirstBlock: first sub-block, any length
            subs, p = read_sub_blocks(b, p, 'TruncatedExtension')
            body = (subs[0] if subs else b'') + bytes(4)
        packed = body[0]
        disposal = (packed >> 2) & 7
        if disposal > 3 and P['disposal_undefined'] == 'Reject':
            raise GifError('UndefinedDisposal')
        st['gce'] = {'disposal': disposal, 'user_input': bool(packed & 2),
                     'delay_cs': le16(body, 1), 'transparent': body[3] if packed & 1 else None}
        st['gce_fresh'] = True
        return p
    if label == 0x01:
        need(b, p, 1, 'TruncatedExtension')
        if b[p] != 12 and P['ext_sizes'] == 'Exact':
            raise GifError('InvalidPlainText')
        _, p = read_sub_blocks(b, p, 'TruncatedExtension')
        return p
    if label == 0xFF:
        need(b, p, 1, 'TruncatedExtension')
        if b[p] != 11 and P['ext_sizes'] == 'Exact':
            raise GifError('InvalidApplicationExtension')
        subs, p = read_sub_blocks(b, p, 'TruncatedExtension')
        if subs and subs[0] == b'NETSCAPE2.0' and st['frames_seen'] == 0:
            data = subs[1:] if P['loop_scan'] == 'EverySubBlock' else subs[1:2]
            for s in data:                           # EverySubBlock: last 3-byte id-1 sub-block wins
                if len(s) == 3 and s[0] == 1:
                    st['loop_count'] = s[1] | (s[2] << 8)
        return p
    if label != 0xFE and P['unknown_extension'] == 'Reject':
        raise GifError('UnknownBlock')
    _, p = read_sub_blocks(b, p, 'TruncatedExtension')
    return p


def parse_image_descriptor(b, p, scr, st, P):
    """G8 with frame_bounds and index_range=GdkTable bookkeeping."""
    if st['frames_seen'] >= MAX_FRAMES:
        raise GifError('TooManyFrames')
    need(b, p, 10, 'TruncatedImageDescriptor')
    left, top, w, h, packed = le16(b, p + 1), le16(b, p + 3), le16(b, p + 5), le16(b, p + 7), b[p + 9]
    p += 10
    if w == 0 or h == 0:
        raise GifError('InvalidFrameDimensions')
    inside = left + w <= scr['width'] and top + h <= scr['height']
    if not inside and P['frame_bounds'] == 'Reject':
        raise GifError('FrameOutsideScreen')
    if st['total_pixels'] + w * h > MAX_TOTAL_PIXELS:
        raise GifError('PixelBudgetExceeded')
    table = scr['gct']
    ncolors = len(table) // 3 if table is not None else 0
    if P['index_range'] == 'GdkTable' and table is not None:
        table = table + bytes(768 - len(table))      # GDK: 256-entry zero-filled global map
    if packed & 0x80:
        n = 3 << ((packed & 7) + 1)
        need(b, p, n, 'TruncatedColorTable')
        table, ncolors = bytes(b[p:p + n]), n // 3
        p += n
        if P['index_range'] == 'GdkTable':           # GDK: local entries persist across frames
            st['gdk_local'][:n] = table
            table = bytes(st['gdk_local'])
    if table is None:
        raise GifError('NoColorTable')
    return {'rect': (left, top, w, h), 'interlaced': bool(packed & 0x40), 'table': table,
            'ncolors': ncolors, 'inside': inside}, p


def read_image_data(b, p, P):
    """G9 with lzw_min and truncation."""
    need(b, p, 1, 'TruncatedImageData')
    m = b[p]
    lo = {'Spec': 2, 'Range1to11': 1, 'Range0to11': 0, 'Range1to11Checked': 1}[P['lzw_min']]
    hi = 8 if P['lzw_min'] == 'Spec' else 11
    if not lo <= m <= hi:
        raise GifError('InvalidLzwMinCodeSize')
    p += 1
    out = []
    if P['empty_first_subblock'] == 'Skip' and p < len(b) and b[p] == 0:
        p += 1                                      # gif crate: a leading empty sub-block is not the terminator
    while True:
        if p >= len(b) or p + 1 + b[p] > len(b):
            if P['image_data'] == 'Streaming':      # decode what arrived; the error follows the frame
                out.append(bytes(b[p + 1:]) if p < len(b) else b'')
                return m, b''.join(out), len(b), 'TruncatedImageData'
            if P['truncation'] == 'Reject':
                raise GifError('TruncatedImageData')
            out.append(bytes(b[p + 1:]) if p < len(b) else b'')
            return m, b''.join(out), len(b), True
        n = b[p]
        if n == 0:
            return m, b''.join(out), p + 1, False
        out.append(bytes(b[p + 1:p + 1 + n]))
        p += 1 + n


def lzw_decode(m, data, area, emit, P, ev, pending=None):
    """G10 with lzw_invalid, lzw_tail and short_frame. Returns normally when the frame
    is complete or accepted short; `ev` collects recovery events."""
    m_eff = max(m, 1)                     # Range0to11: GDK runs a 1-bit decoder for m=0 (residual, see OPTIONS)
    clear, eoi = 1 << m_eff, (1 << m_eff) + 1
    nbits, pos, width = len(data) * 8, 0, m_eff + 1
    table = [bytes([i & 255]) for i in range(clear)] + [b'', b'']
    prev, q = None, 0

    def short(err):
        if q < area:
            if P['short_frame'] == 'Reject':
                raise GifError(err)
            ev.add('E3')
            return
        if P['lzw_tail'] == 'Strict':
            raise GifError('MissingEndOfInformation')
    while True:
        if q == area and P['lzw_tail'] == 'StopAtRaster' and not pending:
            return q
        if pos + width > nbits:
            if pending:                              # data exhausted before EOI: frame not emitted
                return -1
            short('TooFewPixels')
            return q
        code = 0
        for k in range(width):
            i = pos + k
            code |= ((data[i >> 3] >> (i & 7)) & 1) << k
        pos += width
        if code == clear:
            table, width, prev = table[:clear + 2], m_eff + 1, None
            continue
        if q == area and code != eoi:
            if pending:
                return -1
            raise GifError('MissingEndOfInformation')
        if code == eoi:
            if q < area:
                short('TooFewPixels')
                return q
            if pending:
                return q if (pos + 7) // 8 < len(data) else -1
            if (pos + 7) // 8 < len(data):
                raise GifError('DataAfterEndOfInformation')
            return q
        if code < clear and code >= 256 and P['lzw_min'] == 'Range1to11Checked':
            raise GifError('ColorIndexOutOfRange')   # Q6(b): a literal above 255 is no 8-bit index
        if code < len(table):
            s = table[code]
        elif code == len(table) and prev is not None and len(table) < 4096:
            s = prev + prev[:1]
        else:
            if P['lzw_invalid'] == 'Reject':
                raise GifError('InvalidLzwCode')
            short('InvalidLzwCode')
            return q
        if prev is not None and len(table) < 4096:
            table.append(prev + s[:1])
            if len(table) == (1 << width) and width < 12:
                width += 1
        prev = s
        for idx in s:
            if q >= area:
                if P['lzw_tail'] == 'Strict':
                    raise GifError('TooManyPixels')
                ev.add('E4')
                return q
            emit(q, idx)
            q += 1


def render_frame(canvas, scr, fh, gce, m, data, P, ev, pending=None):
    """G12 drawing with frame_bounds=Clip (E1) and index_range (E2)."""
    left, top, w, h = fh['rect']
    table, t = fh['table'], (gce or {}).get('transparent')
    W, H = scr['width'], scr['height']
    if not fh['inside']:
        ev.add('E1')                     # some frame position lies outside the screen

    def emit(q, idx):
        if idx == t:
            return
        if idx >= fh['ncolors']:
            if P['index_range'] == 'Reject':
                raise GifError('ColorIndexOutOfRange')
            ev.add('E2')
            rgba = {'Transparent': None, 'Black': b'\0\0\0\xff',
                    'GdkTable': table[3 * idx:3 * idx + 3] + b'\xff'}[P['index_range']]
        else:
            rgba = table[3 * idx:3 * idx + 3] + b'\xff'
        r = q // w
        x, y = left + q % w, top + (interlace_row(r, h) if fh['interlaced'] else r)
        if x >= W or y >= H:
            return
        if rgba is not None:
            o = 4 * (y * W + x)
            canvas[o:o + 4] = rgba
    return lzw_decode(m, data, w * h, emit, P, ev, pending)


def dispose(canvas, scr, prev):
    """G12.3, clipped to the screen."""
    if prev is None or prev['disposal'] not in (2, 3):
        return
    left, top, w, h = prev['rect']
    W, H = scr['width'], scr['height']
    for y in range(top, min(top + h, H)):
        a, z = 4 * (y * W + left), 4 * (y * W + min(left + w, W))
        if a >= z:
            continue
        canvas[a:z] = bytes(z - a) if prev['disposal'] == 2 else prev['saved'][y - top]


def run(b, P, still):
    scr, p = parse_header(b, P)
    W, H = scr['width'], scr['height']
    canvas = bytearray(4 * W * H)
    st = {'gce': None, 'gce_fresh': False, 'frames_seen': 0, 'total_pixels': 0, 'loop_count': None,
          'gdk_local': bytearray(768)}
    frames, prev, at_image = [], None, False
    res = {'width': W, 'height': H, 'frames': frames}
    try:
        while True:
            if p >= len(b) and P['missing_trailer'] == 'Reject':
                raise GifError('MissingTrailer')
            c = b[p] if p < len(b) else 0x3B           # missing_trailer=AtEof: EOF acts as the trailer
            if c == 0x3B:
                if st['gce'] is not None and st['gce_fresh'] and P['gce_orphan'] == 'Reject':
                    raise GifError('GraphicControlWithoutGraphic')
                if p + 1 < len(b) and P['trailing'] == 'Reject':
                    raise GifError('TrailingBytes')
                res['end'] = 'trailer'
                break
            if c == 0x21:
                p = parse_extension(b, p, st, P)
                continue
            if c != 0x2C:
                if P['unknown_block'] == 'Reject':
                    raise GifError('UnknownBlock')
                p += 1
                continue
            at_image = True                                # G15: the constructor's walk ends here
            fh, p = parse_image_descriptor(b, p, scr, st, P)
            m, data, p, cut = read_image_data(b, p, P)
            gce = st['gce']
            st['gce_fresh'] = False
            if P['gce_scope'] == 'NextGraphic':
                st['gce'] = None
            st['frames_seen'] += 1
            st['total_pixels'] += fh['rect'][2] * fh['rect'][3]
            work = bytearray(canvas)
            dispose(work, scr, prev)
            disp = gce['disposal'] if gce else 0
            left, top, w, h = fh['rect']
            saved = None
            if disp == 3:
                saved = [bytes(work[4 * (y * W + left):4 * (y * W + min(left + w, W))]) if left < W else b''
                         for y in range(top, min(top + h, H))]
            ev = set()
            if isinstance(cut, str):                   # image_data=Streaming hit a structural error
                q = render_frame(work, scr, fh, gce, m, data, P, ev, pending=cut)
                if q < fh['rect'][2] * fh['rect'][3]:
                    raise GifError(cut)
            elif cut:
                ev.add('E3')
                try:
                    render_frame(work, scr, fh, gce, m, data, dict(P, short_frame='KeepCanvas'), ev)
                except GifError:
                    pass                           # KeepPartial: an error inside the cut data ends the frame
            else:
                render_frame(work, scr, fh, gce, m, data, P, ev)
            canvas[:] = work
            pdisp = disp if disp <= 3 else 1
            if pdisp == 3 and not fh['inside'] and P['restore_outside'] == 'Keep':
                pdisp = 1                                  # GDK: copy_area outside the canvas is a no-op
            prev = {'disposal': pdisp, 'rect': fh['rect'], 'saved': saved}
            frames.append({'rgba': bytes(canvas), 'rect': fh['rect'], 'delay_cs': gce['delay_cs'] if gce else 0,
                           'disposal': disp, 'user_input': gce['user_input'] if gce else False,
                           'transparent': gce['transparent'] if gce else None,
                           'interlaced': fh['interlaced'], 'recovered': bool(ev), 'events': sorted(ev)})
            if cut:
                res['end'] = {'error': cut if isinstance(cut, str) else 'TruncatedImageData'}
                break
            if still:
                break
    except GifError as e:
        if not at_image and not still:
            raise                                          # G15: the constructor fails
        res['end'] = {'error': e.name}
    res['loop_count'] = st['loop_count']
    return res


def decode_still(b, P=STRICT):
    try:
        res = run(bytes(b), P, True)
    except GifError as e:
        return {'error': e.name}
    if not res['frames']:
        end = res.get('end')
        return {'error': end['error'] if isinstance(end, dict) else 'NoImage'}
    f = res['frames'][0]
    return {'width': res['width'], 'height': res['height'], 'rgba': f['rgba'], 'recovered': f['recovered']}


def decode_frames(b, P=STRICT):
    try:
        return run(bytes(b), P, False)
    except GifError as e:
        return {'error': e.name}
