#!/usr/bin/env python3
"""GIF-CORE-SPEC draft 0.1: strict reference decoder (Python oracle).

Every function names the SPEC.md section (G1..G18) it implements. The oracle is a
reference model: it allocates freely and is not evidence about any Rust
implementation's memory behaviour (SPEC.md G18).

Public functions:
  decode_still(b)  -> {'width','height','rgba': bytes, 'frame': meta} | {'error': name}
  decode_frames(b) -> {'width','height','loop_count','background_index','aspect',
                       'frames': [frame...], 'end': 'trailer' | {'error': name}}
                      or {'error': name} when the stream fails before the first frame
                      record could be produced and no frame was returned.
A frame is {'rgba': bytes (whole canvas, 4*W*H), 'rect': (left, top, width, height),
'delay_cs', 'disposal', 'user_input', 'transparent': int|None, 'interlaced'}.
"""

# G1. Limits.
MAX_INPUT = 67_108_864
MAX_DIMENSION = 8_192
MAX_OUTPUT = 67_108_864
MAX_FRAMES = 65_536
MAX_TOTAL_PIXELS = 1 << 30

# G16. The closed error list (case-sensitive), in no particular order.
ERRORS = (
    'FileTooLarge', 'TruncatedHeader', 'BadSignature', 'UnsupportedVersion',
    'InvalidDimensions', 'OutputTooLarge', 'TruncatedColorTable', 'MissingTrailer',
    'UnknownBlock', 'TruncatedExtension', 'DuplicateGraphicControl',
    'InvalidGraphicControl', 'UndefinedDisposal', 'InvalidApplicationExtension',
    'InvalidPlainText', 'GraphicControlWithoutGraphic', 'TooManyFrames',
    'TruncatedImageDescriptor', 'InvalidFrameDimensions', 'FrameOutsideScreen',
    'PixelBudgetExceeded', 'NoColorTable', 'TruncatedImageData',
    'InvalidLzwMinCodeSize', 'InvalidLzwCode', 'TooManyPixels', 'TooFewPixels',
    'MissingEndOfInformation', 'DataAfterEndOfInformation',
    'ColorIndexOutOfRange', 'TrailingBytes', 'NoImage',
)
# Strict sets no recovery event; the closed `recovered` list is in ../policy-draft/OPTIONS.md.


class GifError(Exception):
    """A G16 error. `name` is one of ERRORS."""
    def __init__(self, name):
        assert name in ERRORS, name
        super().__init__(name)
        self.name = name


def le16(b, i):
    """D1 (conventions): unsigned little-endian 16-bit word; caller proves i+2 <= len(b)."""
    return b[i] | (b[i + 1] << 8)


def need(b, p, n, err):
    """D2 (conventions): guarded read; raise `err` unless n bytes exist at p."""
    if p + n > len(b):
        raise GifError(err)


def parse_header(b):
    """G2 header, G3 logical screen descriptor, G4 global colour table.
    Returns (screen record, cursor after the global table)."""
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
    return {'width': w, 'height': h, 'gct': gct, 'background_index': b[11],
            'aspect': b[12], 'version': bytes(b[3:6])}, p


def read_sub_blocks(b, p, err):
    """G5 data sub-blocks: from the size byte at p, collect payloads up to and
    including the zero-size block terminator. Returns (payloads, cursor after terminator)."""
    out = []
    while True:
        need(b, p, 1, err)
        n = b[p]
        if n == 0:
            return out, p + 1
        need(b, p + 1, n, err)
        out.append(bytes(b[p + 1:p + 1 + n]))
        p += 1 + n


def parse_extension(b, p, st):
    """G6 extensions (label at p+1) and G7 graphic-control scope. Mutates the
    stream state `st`; returns the cursor after the extension's block terminator."""
    need(b, p, 2, 'TruncatedExtension')
    label = b[p + 1]
    p += 2
    if label == 0xF9:                                   # G6.1 Graphic Control Extension
        if st['gce'] is not None:
            raise GifError('DuplicateGraphicControl')
        need(b, p, 1, 'TruncatedExtension')
        if b[p] != 4:
            raise GifError('InvalidGraphicControl')
        need(b, p, 6, 'TruncatedExtension')
        if b[p + 5] != 0:
            raise GifError('InvalidGraphicControl')
        packed = b[p + 1]
        disposal = (packed >> 2) & 7
        if disposal > 3:
            raise GifError('UndefinedDisposal')
        st['gce'] = {'disposal': disposal, 'user_input': bool(packed & 2),
                     'delay_cs': le16(b, p + 2),
                     'transparent': b[p + 4] if packed & 1 else None}
        return p + 6
    if label == 0x01:                                   # G6.2 Plain Text Extension
        need(b, p, 1, 'TruncatedExtension')
        if b[p] != 12:
            raise GifError('InvalidPlainText')
        need(b, p, 13, 'TruncatedExtension')
        _, p = read_sub_blocks(b, p + 13, 'TruncatedExtension')
        st['gce'] = None                                # G7: a PTE consumes the GCE; not rendered
        return p
    if label == 0xFF:                                   # G6.3 Application Extension
        need(b, p, 1, 'TruncatedExtension')
        if b[p] != 11:
            raise GifError('InvalidApplicationExtension')
        need(b, p, 12, 'TruncatedExtension')
        ident = bytes(b[p + 1:p + 12])
        subs, p = read_sub_blocks(b, p + 12, 'TruncatedExtension')
        if ident == b'NETSCAPE2.0' and st['frames_seen'] == 0:
            for s in subs:                              # G6.3: loop sub-block, last one wins
                if len(s) == 3 and s[0] == 1:
                    st['loop_count'] = s[1] | (s[2] << 8)
        return p
    # G6.4 Comment (0xFE) and unknown labels: skip the sub-blocks.
    _, p = read_sub_blocks(b, p, 'TruncatedExtension')
    if label <= 0x7F:                                   # G7: unknown graphic-rendering label
        st['gce'] = None
    return p


def parse_image_descriptor(b, p, scr, st):
    """G8 image descriptor and local colour table. Returns (frame header, cursor)."""
    if st['frames_seen'] >= MAX_FRAMES:
        raise GifError('TooManyFrames')
    need(b, p, 10, 'TruncatedImageDescriptor')
    left, top, w, h, packed = le16(b, p + 1), le16(b, p + 3), le16(b, p + 5), le16(b, p + 7), b[p + 9]
    p += 10
    if w == 0 or h == 0:
        raise GifError('InvalidFrameDimensions')
    if left + w > scr['width'] or top + h > scr['height']:
        raise GifError('FrameOutsideScreen')
    if st['total_pixels'] + w * h > MAX_TOTAL_PIXELS:
        raise GifError('PixelBudgetExceeded')
    table = scr['gct']
    if packed & 0x80:
        n = 3 << ((packed & 7) + 1)
        need(b, p, n, 'TruncatedColorTable')
        table = bytes(b[p:p + n])
        p += n
    if table is None:
        raise GifError('NoColorTable')
    return {'rect': (left, top, w, h), 'interlaced': bool(packed & 0x40), 'table': table}, p


def read_image_data(b, p):
    """G9 table-based image data: LZW minimum code size and the concatenated
    sub-block payloads. Returns (min_code_size, data, cursor after terminator)."""
    need(b, p, 1, 'TruncatedImageData')
    m = b[p]
    if not 2 <= m <= 8:
        raise GifError('InvalidLzwMinCodeSize')
    subs, p = read_sub_blocks(b, p + 1, 'TruncatedImageData')
    return m, b''.join(subs), p


def lzw_decode(m, data, area, emit):
    """G10 variable-length-code LZW. Calls emit(q, index) for output position q
    in stream order; enforces the G10 pixel-count and end-code rules."""
    clear, eoi = 1 << m, (1 << m) + 1
    nbits = len(data) * 8
    pos = 0                          # bit position of the next code
    width = m + 1
    table = [bytes([i]) for i in range(clear)] + [b'', b'']   # G10.1 initial table
    prev = None                      # previous code's string, None right after a clear
    q = 0                            # pixels produced
    while True:
        if pos + width > nbits:      # G10.5: data exhausted without EOI
            raise GifError('TooFewPixels' if q < area else 'MissingEndOfInformation')
        code = 0
        for k in range(width):       # G10.2: LSB-first packing
            i = pos + k
            code |= ((data[i >> 3] >> (i & 7)) & 1) << k
        pos += width
        if code == clear:            # G10.3
            table, width, prev = table[:clear + 2], m + 1, None
            continue
        if q == area and code != eoi:
            raise GifError('MissingEndOfInformation')   # G10.5: raster complete, no EOI
        if code == eoi:              # G10.4
            if q < area:
                raise GifError('TooFewPixels')
            if (pos + 7) // 8 < len(data):
                raise GifError('DataAfterEndOfInformation')
            return
        if code < len(table) and code not in (clear, eoi):
            s = table[code]
        elif code == len(table) and prev is not None and len(table) < 4096:
            s = prev + prev[:1]      # G10.3: the KwKwK case
        else:
            raise GifError('InvalidLzwCode')
        if prev is not None and len(table) < 4096:
            table.append(prev + s[:1])
            if len(table) == (1 << width) and width < 12:
                width += 1           # G10.3: widen after the table reaches 2**width
        prev = s
        for idx in s:
            if q >= area:
                raise GifError('TooManyPixels')
            emit(q, idx)
            q += 1


def interlace_row(r, h):
    """G11: the image row that holds the r-th row of the stream (0 <= r < h)."""
    for start, step in ((0, 8), (4, 8), (2, 4), (1, 2)):
        n = (h - start + step - 1) // step if h > start else 0
        if r < n:
            return start + r * step
        r -= n
    raise AssertionError('row out of range')


def render_frame(canvas, scr, fh, gce, m, data):
    """G12 drawing: decode the frame's indices onto the canvas (in place)."""
    left, top, w, h = fh['rect']
    table, t = fh['table'], (gce or {}).get('transparent')
    W = scr['width']

    def emit(q, idx):
        if idx == t:
            return                    # G12.2: transparent index leaves the pixel
        if 3 * idx + 3 > len(table):
            raise GifError('ColorIndexOutOfRange')
        r = q // w
        y = top + (interlace_row(r, h) if fh['interlaced'] else r)
        o = 4 * (y * W + left + q % w)
        canvas[o:o + 4] = table[3 * idx:3 * idx + 3] + b'\xff'
    lzw_decode(m, data, w * h, emit)


def dispose(canvas, scr, prev):
    """G12.3: apply the previous frame's disposal before the next frame is drawn."""
    if prev is None or prev['disposal'] not in (2, 3):
        return
    left, top, w, h = prev['rect']
    W = scr['width']
    for y in range(top, top + h):
        a, z = 4 * (y * W + left), 4 * (y * W + left + w)
        canvas[a:z] = bytes(z - a) if prev['disposal'] == 2 else prev['saved'][y - top]


def run(b, still):
    """G13-G15: the shared stream walk. Returns the result record of decode_still
    (still=True) or decode_frames (still=False)."""
    scr, p = parse_header(b)
    W, H = scr['width'], scr['height']
    canvas = bytearray(4 * W * H)                       # G12.1: transparent black
    st = {'gce': None, 'frames_seen': 0, 'total_pixels': 0, 'loop_count': None}
    frames, prev = [], None
    res = {'width': W, 'height': H, 'version': scr['version'].decode(), 'background_index': scr['background_index'],
           'aspect': scr['aspect'], 'frames': frames}
    try:
        while True:                                     # G5 block loop
            if p >= len(b):
                raise GifError('MissingTrailer')
            c = b[p]
            if c == 0x3B:
                if st['gce'] is not None:
                    raise GifError('GraphicControlWithoutGraphic')
                if p + 1 < len(b):
                    raise GifError('TrailingBytes')
                res['end'] = 'trailer'
                break
            if c == 0x21:
                p = parse_extension(b, p, st)
                continue
            if c != 0x2C:
                raise GifError('UnknownBlock')
            fh, p = parse_image_descriptor(b, p, scr, st)
            m, data, p = read_image_data(b, p)
            gce, st['gce'] = st['gce'], None
            st['frames_seen'] += 1
            st['total_pixels'] += fh['rect'][2] * fh['rect'][3]
            work = bytearray(canvas)                    # errors discard this frame's writes
            dispose(work, scr, prev)
            disp = gce['disposal'] if gce else 0
            left, top, w, h = fh['rect']
            saved = [bytes(work[4 * (y * W + left):4 * (y * W + left + w)])
                     for y in range(top, top + h)] if disp == 3 else None
            render_frame(work, scr, fh, gce, m, data)
            canvas[:] = work
            prev = {'disposal': disp, 'rect': fh['rect'], 'saved': saved}
            frames.append({'rgba': bytes(canvas), 'rect': fh['rect'],
                           'delay_cs': gce['delay_cs'] if gce else 0, 'disposal': disp,
                           'user_input': gce['user_input'] if gce else False,
                           'transparent': gce['transparent'] if gce else None,
                           'interlaced': fh['interlaced']})
            if still:
                break
    except GifError as e:
        res['end'] = {'error': e.name}
    res['loop_count'] = st['loop_count']
    return res


def decode_still(b):
    """G14: the first frame composited on the logical screen. Bytes after the first
    image's block terminator are not examined."""
    try:
        res = run(bytes(b), still=True)
    except GifError as e:
        return {'error': e.name}
    if not res['frames']:
        end = res.get('end')
        return {'error': end['error'] if isinstance(end, dict) else 'NoImage'}
    f = res['frames'][0]
    return {'width': res['width'], 'height': res['height'], 'rgba': f['rgba'],
            'frame': {k: f[k] for k in ('rect', 'delay_cs', 'disposal', 'user_input',
                                        'transparent', 'interlaced')}}


def decode_frames(b):
    """G15: every frame in order, then the stream's end ('trailer' or the first error).
    Header errors (G2-G4) return {'error': name} with no frame list."""
    try:
        return run(bytes(b), still=False)
    except GifError as e:
        return {'error': e.name}


if __name__ == '__main__':
    import json
    import sys
    data = open(sys.argv[1], 'rb').read()
    r = decode_frames(data)
    for f in r.get('frames', []):
        f['rgba'] = f['rgba'].hex()[:64] + '...'
    print(json.dumps(r, default=str, indent=1))
