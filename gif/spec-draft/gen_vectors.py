#!/usr/bin/env python3
"""Generate VECTORS.json for GIF-CORE-SPEC draft 0.2.

Independence: inputs are built by an LZW *encoder* and a block writer in this file,
and every expectation is written from the generator's intent (the index raster it
encoded, a hand-drawn canvas, or a hand-named error), never by calling the oracle.
check_vectors.py then runs gif_oracle.py against every record, and
../policy-draft/policy_oracle.py against records that name a `profile`.
"""
import json
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parent

# ---------------------------------------------------------------- encoder side

def lzw_codes(indices, m, clear_first=True, eoi=True, clear_on_full=True, clear_every=None):
    """Standard GIF LZW encoder: returns the emitted code list (no widths)."""
    clear, end = 1 << m, (1 << m) + 1
    out = [clear] if clear_first else []
    dic = {bytes([i]): i for i in range(min(clear, 256))}
    nxt = end + 1
    w = b''
    since = 0
    for v in indices:
        c = bytes([v])
        if w + c in dic:
            w += c
            continue
        out.append(dic[w])
        since += 1
        if nxt < 4096:
            dic[w + c] = nxt
            nxt += 1
        elif clear_on_full:
            out.append(clear)
            dic = {bytes([i]): i for i in range(min(clear, 256))}
            nxt = end + 1
        if clear_every and since >= clear_every and nxt < 4096:
            out.append(clear)
            dic = {bytes([i]): i for i in range(min(clear, 256))}
            nxt = end + 1
            since = 0
        w = c
    if w:
        out.append(dic[w])
    if eoi:
        out.append(end)
    return out


def widths_for(codes, m):
    """Width of each code as the reader expects it: one table entry per code that is
    not the first after a clear, widening when the table reaches 2**width."""
    clear, end = 1 << m, (1 << m) + 1
    size, width, first, ws = clear + 2, m + 1, True, []
    for c in codes:
        ws.append(width)
        if c == clear:
            size, width, first = clear + 2, m + 1, True
            continue
        if c == end:
            continue
        if not first and size < 4096:
            size += 1
            if size == (1 << width) and width < 12:
                width += 1
        first = False
    return ws


def pack(codes, widths):
    """LSB-first bit packing of (code, width) pairs into bytes."""
    acc = nb = 0
    out = bytearray()
    for c, w in zip(codes, widths):
        acc |= c << nb
        nb += w
        while nb >= 8:
            out.append(acc & 255)
            acc >>= 8
            nb -= 8
    if nb:
        out.append(acc & 255)
    return bytes(out)


def sub_blocks(data, size=255):
    out = bytearray()
    for i in range(0, len(data), size):
        chunk = data[i:i + size]
        out += bytes([len(chunk)]) + chunk
    return bytes(out) + b'\0'


def le(v):
    return bytes([v & 255, v >> 8])


def header(w, h, gct=None, bg=0, aspect=0, version=b'89a', extra_flags=0):
    flags = extra_flags
    if gct is not None:
        size = {2: 0, 4: 1, 8: 2, 16: 3, 32: 4, 64: 5, 128: 6, 256: 7}[len(gct) // 3]
        flags |= 0x80 | size
    return b'GIF' + version + le(w) + le(h) + bytes([flags, bg, aspect]) + (gct or b'')


def gce(disposal=0, delay=0, transparent=None, user_input=False, reserved=0):
    packed = (reserved << 5) | (disposal << 2) | (2 if user_input else 0) | (1 if transparent is not None else 0)
    return b'\x21\xf9\x04' + bytes([packed]) + le(delay) + bytes([transparent or 0]) + b'\0'


def image(left, top, w, h, indices=None, m=2, lct=None, interlaced=False, data=None, extra_flags=0, **kw):
    """Image descriptor + optional local table + image data. `indices` are in display
    order; they are re-ordered into stream order when interlaced."""
    flags = extra_flags | (0x40 if interlaced else 0)
    if lct is not None:
        flags |= 0x80 | {2: 0, 4: 1, 8: 2, 16: 3, 32: 4, 64: 5, 128: 6, 256: 7}[len(lct) // 3]
    if data is None:
        rows = [indices[r * w:(r + 1) * w] for r in range(h)]
        if interlaced:
            order = list(range(0, h, 8)) + list(range(4, h, 8)) + list(range(2, h, 4)) + list(range(1, h, 2))
            rows = [rows[r] for r in order]
        stream = [v for row in rows for v in row]
        codes = lzw_codes(stream, m, **kw)
        data = pack(codes, widths_for(codes, m))
    return (b'\x2c' + le(left) + le(top) + le(w) + le(h) + bytes([flags]) + (lct or b'')
            + bytes([m]) + sub_blocks(data))


def raw_image(left, top, w, h, m, codes, lct=None, widths=None, extra=b''):
    """Image whose code list is given explicitly (for malformed LZW)."""
    data = pack(codes, widths or widths_for(codes, m)) + extra
    return image(left, top, w, h, m=m, lct=lct, data=data)


TRAILER = b'\x3b'
PAL4 = bytes([0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255])         # black red green blue
PAL2 = bytes([255, 255, 255, 0, 0, 0])                           # white black
LOCAL4 = bytes([255, 255, 0, 0, 255, 255, 255, 0, 255, 128, 128, 128])  # yellow cyan magenta grey
C = {'k': (0, 0, 0, 255), 'r': (255, 0, 0, 255), 'g': (0, 255, 0, 255), 'b': (0, 0, 255, 255),
     'w': (255, 255, 255, 255), '.': (0, 0, 0, 0), 'y': (255, 255, 0, 255), 'c': (0, 255, 255, 255),
     'm': (255, 0, 255, 255), 'e': (128, 128, 128, 255)}

# ---------------------------------------------------------------- expectations

def runs(pixels):
    out = []
    for p in pixels:
        h = '%02x%02x%02x%02x' % p
        if out and out[-1]['hex'] == h:
            out[-1]['count'] += 1
        else:
            out.append({'count': 1, 'hex': h})
    return out


def grid(s):
    """Canvas from a string of colour letters (rows separated by '/')."""
    return [C[ch] for ch in s.replace('/', '')]


def from_indices(indices, pal):
    return [tuple(pal[3 * i:3 * i + 3]) + (255,) for i in indices]


V = []


def add(name, entry, b, expected, profile=None):
    v = {'name': name, 'entry': entry, 'input_hex': b.hex(), 'expected': expected}
    if profile:
        v['profile'] = profile
    V.append(v)


RB, ICC = 'RecommendedBeta', 'ImageCrateCompat-0.25.10'


def rec(expected, *flags):
    """Profile expectation: add `recovered` to a still result or to each frame (flags in order)."""
    e = json.loads(json.dumps(expected))
    if 'frames' in e:
        for f, r in zip(e['frames'], flags or [False] * len(e['frames'])):
            f['recovered'] = r
    elif 'error' not in e:
        e['recovered'] = flags[0] if flags else False
    return e


def both_profile(name, profile, b, w, h, pixels, fmeta=None, loop=None, recovered=False):
    add(f'{name}@{profile}', 'still', b, rec(still_ok(w, h, pixels), recovered), profile)
    f = frame(**(fmeta or {'rect': (0, 0, w, h)}), pixels=pixels)
    add(f'{name}/frames@{profile}', 'frames', b, rec(frames_ok(w, h, [f], loop=loop), recovered), profile)


def still_ok(w, h, pixels):
    assert len(pixels) == w * h
    return {'width': w, 'height': h, 'rgba_runs': runs(pixels)}


def frame(rect, pixels, delay=0, disposal=0, transparent=None, user_input=False, interlaced=False):
    return {'rect': list(rect), 'delay_cs': delay, 'disposal': disposal, 'user_input': user_input,
            'transparent': transparent, 'interlaced': interlaced, 'rgba_runs': runs(pixels)}


def frames_ok(w, h, frames, end='trailer', loop=None):
    return {'width': w, 'height': h, 'loop_count': loop, 'frames': frames, 'end': end}


def err(name):
    return {'error': name}


def both(name, b, w, h, pixels, fmeta=None, loop=None):
    """Single-frame file: still and frames entries share the pixels."""
    add(name, 'still', b, still_ok(w, h, pixels))
    f = frame(**(fmeta or {'rect': (0, 0, w, h)}), pixels=pixels)
    add(name + '/frames', 'frames', b, frames_ok(w, h, [f], loop=loop))

# ---------------------------------------------------------------- basic and colour tables

ix = [0, 1, 2, 3, 3, 2, 1, 0, 1, 1, 2, 2]
base = header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + TRAILER
both('basic-4x3-gct', base, 4, 3, from_indices(ix, PAL4))
both('basic-87a', header(4, 3, PAL4, version=b'87a') + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
both('local-table-overrides-global', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, lct=LOCAL4) + TRAILER, 4, 3, from_indices(ix, LOCAL4))
both('local-table-without-global', header(4, 3) + image(0, 0, 4, 3, ix, lct=LOCAL4) + TRAILER, 4, 3, from_indices(ix, LOCAL4))
both('flags-ignored-colour-resolution-sort-aspect-bg', header(4, 3, PAL4, bg=200, aspect=49, extra_flags=0x78)
     + image(0, 0, 4, 3, ix, extra_flags=0x20 | 0x18) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('no-color-table', 'still', header(4, 3) + image(0, 0, 4, 3, ix) + TRAILER, err('NoColorTable'))
pal256 = bytes((i * 7) % 256 for i in range(768))
ix8 = [(i * 37) % 256 for i in range(16 * 16)]
both('gct-256-m8', header(16, 16, pal256) + image(0, 0, 16, 16, ix8, m=8) + TRAILER, 16, 16, from_indices(ix8, pal256))
both('min-code-8-small-table', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, m=8) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('index-out-of-range', 'still', header(2, 1, PAL2) + image(0, 0, 2, 1, [1, 2]) + TRAILER, err('ColorIndexOutOfRange'))
both('index-out-of-range-but-transparent', header(2, 1, PAL2) + gce(transparent=3) + image(0, 0, 2, 1, [1, 3]) + TRAILER,
     2, 1, [C['k'], C['.']], {'rect': (0, 0, 2, 1), 'transparent': 3})
# 1-bit table with m=2 (spec: black and white images use code size 2)
both('two-colour-m2', header(3, 1, PAL2) + image(0, 0, 3, 1, [0, 1, 0]) + TRAILER, 3, 1, [C['w'], C['k'], C['w']])

# ---------------------------------------------------------------- header and screen

add('empty', 'still', b'', err('TruncatedHeader'))
add('header-12', 'still', base[:12], err('TruncatedHeader'))
add('bad-signature', 'still', b'GIF' .replace(b'G', b'J') + base[3:], err('BadSignature'))
add('version-88a', 'still', b'GIF88a' + base[6:], err('UnsupportedVersion'))
add('version-lowercase-89A', 'still', b'GIF89A' + base[6:], err('UnsupportedVersion'))
add('screen-zero-width', 'still', header(0, 3, PAL4) + image(0, 0, 4, 3, ix) + TRAILER, err('InvalidDimensions'))
add('screen-zero-height', 'still', header(4, 0, PAL4) + TRAILER, err('InvalidDimensions'))
add('screen-8193-accepted', 'still', header(8193, 1, PAL4) + TRAILER, err('NoImage'))   # Q8: no per-side cap below 65,535
add('screen-output-too-large', 'still', header(8192, 2049, PAL4) + TRAILER, err('OutputTooLarge'))
add('screen-output-exact-cap-no-image', 'still', header(8192, 2048, PAL4) + TRAILER, err('NoImage'))
add('screen-output-exact-cap-no-image/frames', 'frames', header(8192, 2048, PAL4) + TRAILER, frames_ok(8192, 2048, []))
add('gct-truncated', 'still', header(4, 3, PAL4)[:20], err('TruncatedColorTable'))
add('header-only-gct-trailer', 'frames', header(4, 3, PAL4) + TRAILER, frames_ok(4, 3, []))
add('header-only-gct-trailer/still', 'still', header(4, 3, PAL4) + TRAILER, err('NoImage'))
V.append({'name': 'over-64m', 'entry': 'still', 'input_hex': base.hex(), 'pad_to': 67_108_865, 'expected': err('FileTooLarge')})
V.append({'name': 'exact-64m-trailing', 'entry': 'still', 'input_hex': base.hex(), 'pad_to': 67_108_864,
          'expected': still_ok(4, 3, from_indices(ix, PAL4))})
V.append({'name': 'exact-64m-trailing/frames', 'entry': 'frames', 'input_hex': base.hex(), 'pad_to': 67_108_864,
          'expected': frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('TrailingBytes'))})

# ---------------------------------------------------------------- stream structure

add('missing-trailer', 'frames', header(4, 3, PAL4) + image(0, 0, 4, 3, ix),
    frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('MissingTrailer')))
add('missing-trailer/still', 'still', header(4, 3, PAL4) + image(0, 0, 4, 3, ix), still_ok(4, 3, from_indices(ix, PAL4)))
add('trailing-bytes', 'frames', base + b'\0',
    frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('TrailingBytes')))
add('trailing-bytes/still', 'still', base + b'junk', still_ok(4, 3, from_indices(ix, PAL4)))
add('unknown-block-byte', 'frames', header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + b'\x00' + TRAILER,
    frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('UnknownBlock')))
add('unknown-block-before-image', 'still', header(4, 3, PAL4) + b'\x2d' + image(0, 0, 4, 3, ix) + TRAILER, err('UnknownBlock'))
comment = b'\x21\xfe\x05hello\x03abc\0'
both('comment-before-image', header(4, 3, PAL4) + comment + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
both('comment-empty', header(4, 3, PAL4) + b'\x21\xfe\0' + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('comment-truncated', 'still', header(4, 3, PAL4) + b'\x21\xfe\x05hel', err('TruncatedExtension'))
add('extension-label-missing', 'still', header(4, 3, PAL4) + b'\x21', err('TruncatedExtension'))
netscape = b'\x21\xff\x0bNETSCAPE2.0\x03\x01' + le(5) + b'\0'
both('netscape-loop-5', header(4, 3, PAL4) + netscape + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4), loop=5)
both('netscape-loop-0', header(4, 3, PAL4) + netscape.replace(le(5) + b'\0', le(0) + b'\0') + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4), loop=0)
both('netscape-buffering-subblock-ignored', header(4, 3, PAL4) + b'\x21\xff\x0bNETSCAPE2.0\x05\x02\x00\x10\x00\x00\x03\x01\x07\x00\0'
     + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4), loop=7)
both('netscape-after-first-image-ignored', header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + netscape + TRAILER, 4, 3, from_indices(ix, PAL4))
both('animexts-not-a-loop-source', header(4, 3, PAL4) + netscape.replace(b'NETSCAPE2.0', b'ANIMEXTS1.0') + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4))
both('xmp-app-extension-skipped', header(4, 3, PAL4) + b'\x21\xff\x0bXMP DataXMP\x04<x/>\x02\x01\x00\0'
     + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('app-extension-size-10', 'still', header(4, 3, PAL4) + b'\x21\xff\x0aNETSCAPE2.\x03\x01\x00\x00\0' + image(0, 0, 4, 3, ix) + TRAILER,
    err('InvalidApplicationExtension'))
both('unknown-extension-special-label-skipped', header(4, 3, PAL4) + b'\x21\xfa\x02ab\0' + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4))
plain = b'\x21\x01\x0c' + le(0) + le(0) + le(8) + le(8) + bytes([8, 8, 1, 0]) + b'\x02hi\0'
both('plain-text-not-rendered', header(4, 3, PAL4) + plain + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('plain-text-size-11', 'still', header(4, 3, PAL4) + b'\x21\x01\x0b' + bytes(11) + b'\0' + image(0, 0, 4, 3, ix) + TRAILER,
    err('InvalidPlainText'))

# ---------------------------------------------------------------- graphic control

both('gce-delay-userinput', header(4, 3, PAL4) + gce(delay=7, user_input=True) + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4), {'rect': (0, 0, 4, 3), 'delay': 7, 'user_input': True})
both('gce-reserved-bits-ignored', header(4, 3, PAL4) + gce(reserved=7) + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, from_indices(ix, PAL4))
tp = [C['.'] if i == 2 else c for i, c in zip(ix, from_indices(ix, PAL4))]
both('gce-transparent-2', header(4, 3, PAL4) + gce(transparent=2) + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, tp,
     {'rect': (0, 0, 4, 3), 'transparent': 2})
both('gce-transparent-flag-clear-index-ignored', header(4, 3, PAL4) + b'\x21\xf9\x04\x00\x00\x00\x02\0' + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4))
add('gce-size-5', 'still', header(4, 3, PAL4) + b'\x21\xf9\x05\x00\x00\x00\x00\x00\0' + image(0, 0, 4, 3, ix) + TRAILER,
    err('InvalidGraphicControl'))
add('gce-size-3', 'still', header(4, 3, PAL4) + b'\x21\xf9\x03\x00\x00\x00\0' + image(0, 0, 4, 3, ix) + TRAILER,
    err('InvalidGraphicControl'))
add('gce-two-subblocks', 'still', header(4, 3, PAL4) + b'\x21\xf9\x04\x00\x00\x00\x00\x01\x00\0' + image(0, 0, 4, 3, ix) + TRAILER,
    err('InvalidGraphicControl'))
add('gce-truncated', 'still', header(4, 3, PAL4) + b'\x21\xf9\x04\x00\x00', err('TruncatedExtension'))
add('gce-duplicate', 'still', header(4, 3, PAL4) + gce() + gce() + image(0, 0, 4, 3, ix) + TRAILER, err('DuplicateGraphicControl'))
both('gce-comment-gce-target', header(4, 3, PAL4) + gce(transparent=2) + comment + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, tp,
     {'rect': (0, 0, 4, 3), 'transparent': 2})
# Q7: a pending Graphic Control meeting Plain Text or an unknown label 0x00-0x7F is unsupported in Strict;
# RecommendedBeta (gce_consumers=ImageOnly) applies it to the next image, without `recovered`.
for nm, blk in (('gce-consumed-by-plain-text', plain), ('gce-consumed-by-unknown-graphic-label', b'\x21\x02\x01z\0')):
    bb = header(4, 3, PAL4) + gce(transparent=2) + blk + image(0, 0, 4, 3, ix) + TRAILER
    add(nm, 'still', bb, err('UnsupportedGraphicControlTarget'))
    add(nm + '/frames', 'frames', bb, err('UnsupportedGraphicControlTarget'))      # constructor error (G15)
    both_profile(nm, RB, bb, 4, 3, tp, {'rect': (0, 0, 4, 3), 'transparent': 2})
both('unknown-graphic-label-without-gce-skipped', header(4, 3, PAL4) + b'\x21\x02\x01z\0' + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4))
both('gce-after-plain-text-applies', header(4, 3, PAL4) + plain + gce(transparent=2) + image(0, 0, 4, 3, ix) + TRAILER, 4, 3, tp,
     {'rect': (0, 0, 4, 3), 'transparent': 2})
add('gce-plain-text-target-before-truncation', 'still', header(4, 3, PAL4) + gce() + b'\x21\x01\x0c\x00', err('UnsupportedGraphicControlTarget'))
add('gce-plain-text-after-first-image', 'frames', header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + gce() + plain + image(0, 0, 4, 3, ix) + TRAILER,
    frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('UnsupportedGraphicControlTarget')))
add('gce-then-trailer', 'frames', header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + gce() + TRAILER,
    frames_ok(4, 3, [frame((0, 0, 4, 3), from_indices(ix, PAL4))], end=err('GraphicControlWithoutGraphic')))
for d in (4, 5, 6, 7):
    add(f'gce-disposal-{d}', 'still', header(4, 3, PAL4) + gce(disposal=d) + image(0, 0, 4, 3, ix) + TRAILER, err('UndefinedDisposal'))

# ---------------------------------------------------------------- image descriptor

add('frame-zero-width', 'still', header(4, 3, PAL4) + raw_image(0, 0, 0, 3, 2, [4, 5]) + TRAILER, err('InvalidFrameDimensions'))
add('frame-zero-height', 'still', header(4, 3, PAL4) + raw_image(0, 0, 4, 0, 2, [4, 5]) + TRAILER, err('InvalidFrameDimensions'))
add('frame-right-edge-outside', 'still', header(4, 3, PAL4) + image(1, 0, 4, 3, ix) + TRAILER, err('FrameOutsideScreen'))
add('frame-bottom-edge-outside', 'still', header(4, 3, PAL4) + image(0, 1, 4, 3, ix) + TRAILER, err('FrameOutsideScreen'))
add('frame-left-65535', 'still', header(4, 3, PAL4) + image(65535, 0, 1, 1, [1]) + TRAILER, err('FrameOutsideScreen'))
both('frame-inset', header(4, 3, PAL4) + image(1, 1, 2, 2, [1, 2, 3, 1]) + TRAILER, 4, 3, grid('..../.rg./.br.'),
     {'rect': (1, 1, 2, 2)})
both('frame-touches-bottom-right', header(4, 3, PAL4) + image(3, 2, 1, 1, [3]) + TRAILER, 4, 3, grid('..../..../...b'),
     {'rect': (3, 2, 1, 1)})
add('descriptor-truncated', 'still', header(4, 3, PAL4) + image(0, 0, 4, 3, ix)[:7], err('TruncatedImageDescriptor'))
add('local-table-truncated', 'still', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, lct=LOCAL4)[:15], err('TruncatedColorTable'))

# ---------------------------------------------------------------- interlace (heights 1..17 cover every empty-pass case)

for h in list(range(1, 10)) + [16, 17]:
    w = 2
    ixs = [(r % 4) for r in range(h) for _ in range(w)]
    both(f'interlace-h{h}', header(w, h, PAL4) + image(0, 0, w, h, ixs, interlaced=True) + TRAILER, w, h,
         from_indices(ixs, PAL4), {'rect': (0, 0, w, h), 'interlaced': True})
# interlaced inset frame
ixs = [(r % 4) for r in range(5) for _ in range(3)]
both('interlace-inset', header(5, 7, PAL4) + image(1, 1, 3, 5, ixs, interlaced=True) + TRAILER, 5, 7,
     [C['.']] * 5 + sum(([C['.']] + from_indices(ixs[3 * r:3 * r + 3], PAL4) + [C['.']] for r in range(5)), []) + [C['.']] * 5,
     {'rect': (1, 1, 3, 5), 'interlaced': True})

# ---------------------------------------------------------------- LZW

both('lzw-no-leading-clear', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, clear_first=False) + TRAILER, 4, 3, from_indices(ix, PAL4))
both('lzw-clear-every-code', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, clear_every=1) + TRAILER, 4, 3, from_indices(ix, PAL4))
both('lzw-clear-every-3', header(4, 3, PAL4) + image(0, 0, 4, 3, ix, clear_every=3) + TRAILER, 4, 3, from_indices(ix, PAL4))
# KwKwK: a run of one colour makes the encoder emit codes equal to the next free entry
run_ix = [1] * 12
both('lzw-kwkwk-run', header(4, 3, PAL4) + image(0, 0, 4, 3, run_ix) + TRAILER, 4, 3, from_indices(run_ix, PAL4))
# sub-block boundaries: one byte per sub-block
data = pack(lzw_codes(ix, 2), widths_for(lzw_codes(ix, 2), 2))
both('lzw-one-byte-subblocks', header(4, 3, PAL4) + b'\x2c' + le(0) + le(0) + le(4) + le(3) + b'\x00\x02' + sub_blocks(data, 1) + TRAILER,
     4, 3, from_indices(ix, PAL4))
# large random image: fills the 4096-entry table; deferred clear vs clear-on-full
rnd = random.Random(731)
big = [rnd.randrange(256) for _ in range(128 * 64)]
both('lzw-table-full-clear', header(128, 64, pal256) + image(0, 0, 128, 64, big, m=8) + TRAILER, 128, 64, from_indices(big, pal256))
both('lzw-table-full-deferred-clear', header(128, 64, pal256) + image(0, 0, 128, 64, big, m=8, clear_on_full=False) + TRAILER,
     128, 64, from_indices(big, pal256))
rnd4 = [rnd.randrange(4) for _ in range(64 * 64)]
both('lzw-m2-reaches-12-bits', header(64, 64, PAL4) + image(0, 0, 64, 64, rnd4, m=2, clear_on_full=False) + TRAILER,
     64, 64, from_indices(rnd4, PAL4))
add('lzw-min-code-1', 'still', header(2, 1, PAL2) + raw_image(0, 0, 2, 1, 1, [2, 0, 1, 3], widths=[2, 2, 2, 2]) + TRAILER,
    err('InvalidLzwMinCodeSize'))
add('lzw-min-code-9', 'still', header(2, 1, PAL2) + raw_image(0, 0, 2, 1, 9, [512, 0, 1, 513]) + TRAILER, err('InvalidLzwMinCodeSize'))
add('lzw-min-code-0', 'still', header(2, 1, PAL2) + b'\x2c' + le(0) + le(0) + le(2) + le(1) + b'\x00\x00\x01\x00\x00' + TRAILER,
    err('InvalidLzwMinCodeSize'))
# m=2: clear=4, eoi=5, first free 6
add('lzw-code-beyond-next', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 7, 5], widths=[3, 3, 3, 3]) + TRAILER,
    err('InvalidLzwCode'))
add('lzw-first-code-is-next-free', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 6, 5], widths=[3, 3, 3]) + TRAILER,
    err('InvalidLzwCode'))
add('lzw-first-code-eoi-after-clear-then-data', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 5]) + TRAILER, err('TooFewPixels'))
add('lzw-empty-data', 'still', header(2, 1, PAL4) + b'\x2c' + le(0) + le(0) + le(2) + le(1) + b'\x00\x02\x00' + TRAILER, err('TooFewPixels'))
add('lzw-too-few-pixels-eoi', 'still', header(2, 2, PAL4) + raw_image(0, 0, 2, 2, 2, [4, 1, 2, 5]) + TRAILER, err('TooFewPixels'))
add('lzw-too-few-pixels-no-eoi', 'still', header(2, 3, PAL4) + raw_image(0, 0, 2, 3, 2, [4, 1, 2]) + TRAILER, err('TooFewPixels'))
add('lzw-literal-after-raster', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 3, 5]) + TRAILER,
    err('MissingEndOfInformation'))
add('lzw-missing-eoi-short-padding', 'still', header(2, 2, PAL4) + raw_image(0, 0, 2, 2, 2, [4, 1, 2, 6]) + TRAILER,
    err('MissingEndOfInformation'))
add('lzw-too-many-pixels-in-one-string', 'still', header(3, 1, PAL4) + raw_image(0, 0, 3, 1, 2, [4, 1, 2, 6, 5]) + TRAILER,
    err('TooManyPixels'))
add('lzw-missing-eoi', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2]) + TRAILER, err('MissingEndOfInformation'))
both('lzw-clear-after-raster-then-eoi', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 4, 5]) + TRAILER, 2, 1, [C['r'], C['g']])
add('lzw-data-after-eoi', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 5], extra=b'\0') + TRAILER,
    err('DataAfterEndOfInformation'))
both('lzw-eoi-padding-bits-ignored', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 5, 7], widths=[3, 3, 3, 3, 4]) + TRAILER,
     2, 1, [C['r'], C['g']])
add('lzw-index-error-before-code-error', 'still', header(2, 1, PAL2) + raw_image(0, 0, 2, 1, 2, [4, 3, 7, 5], widths=[3, 3, 3, 3]) + TRAILER,
    err('ColorIndexOutOfRange'))
add('image-data-truncated-in-subblock', 'still', (header(4, 3, PAL4) + image(0, 0, 4, 3, ix))[:-3], err('TruncatedImageData'))
add('image-data-no-terminator', 'still', (header(4, 3, PAL4) + image(0, 0, 4, 3, ix))[:-1], err('TruncatedImageData'))
add('image-data-no-min-code-byte', 'still', header(4, 3, PAL4) + image(0, 0, 4, 3, ix)[:10], err('TruncatedImageData'))
add('min-code-error-before-truncation', 'still', header(4, 3, PAL4) + b'\x2c' + le(0) + le(0) + le(4) + le(3) + b'\x00\x0c\x05ab',
    err('InvalidLzwMinCodeSize'))
add('truncation-before-lzw-error', 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 7, 5], widths=[3, 3, 3])[:-1],
    err('TruncatedImageData'))

# ---------------------------------------------------------------- animation and compositing
# 4x2 screen, PAL4. Frame A covers all with red; frame B is 2x1 at (1,0) in green.
A = image(0, 0, 4, 2, [1] * 8)
B = image(1, 0, 2, 1, [2, 2])
fa = grid('rrrr/rrrr')
for d, after in ((0, 'rggr/rrrr'), (1, 'rggr/rrrr'), (2, 'rggr/rrrr'), (3, 'rggr/rrrr')):
    pass
# disposal of A affects how B lands
add('anim-dispose-none-then-inset', 'frames', header(4, 2, PAL4) + gce(disposal=1, delay=10) + A + gce(delay=20) + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa, delay=10, disposal=1), frame((1, 0, 2, 1), grid('rggr/rrrr'), delay=20)]))
add('anim-dispose-background-then-inset', 'frames', header(4, 2, PAL4) + gce(disposal=2) + A + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa, disposal=2), frame((1, 0, 2, 1), grid('.gg./....'))]))
add('anim-dispose-previous-first-frame', 'frames', header(4, 2, PAL4) + gce(disposal=3) + A + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa, disposal=3), frame((1, 0, 2, 1), grid('.gg./....'))]))
C3 = image(0, 1, 2, 1, [3, 3])
add('anim-dispose-previous-middle', 'frames', header(4, 2, PAL4) + A + gce(disposal=3) + B + C3 + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa), frame((1, 0, 2, 1), grid('rggr/rrrr'), disposal=3),
                     frame((0, 1, 2, 1), grid('rrrr/bbrr'))]))
add('anim-dispose-previous-twice', 'frames', header(4, 2, PAL4) + A + gce(disposal=3) + B + gce(disposal=3) + C3 + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa), frame((1, 0, 2, 1), grid('rggr/rrrr'), disposal=3),
                     frame((0, 1, 2, 1), grid('rrrr/bbrr'), disposal=3), frame((1, 0, 2, 1), grid('rggr/rrrr'))]))
add('anim-dispose-background-transparent-pixels', 'frames',
    header(4, 2, PAL4) + A + gce(disposal=2, transparent=2) + image(0, 0, 4, 2, [2, 3, 3, 2, 2, 2, 2, 2]) + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa), frame((0, 0, 4, 2), grid('rbbr/rrrr'), disposal=2, transparent=2),
                     frame((1, 0, 2, 1), grid('.gg./....'))]))
add('anim-last-frame-disposal-not-applied', 'frames', header(4, 2, PAL4) + gce(disposal=2) + A + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa, disposal=2)]))
add('anim-gce-scope-one-frame', 'frames', header(4, 2, PAL4) + A + gce(transparent=1, delay=5) + image(0, 0, 4, 2, [1, 2] * 4)
    + image(0, 0, 4, 2, [1, 2] * 4) + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa), frame((0, 0, 4, 2), grid('rgrg/rgrg'), delay=5, transparent=1),
                     frame((0, 0, 4, 2), grid('rgrg/rgrg'))]))
add('anim-local-table-scope-one-frame', 'frames', header(4, 2, PAL4) + image(0, 0, 4, 2, [0] * 8, lct=LOCAL4) + A + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), grid('yyyy/yyyy')), frame((0, 0, 4, 2), fa)]))
add('anim-error-in-second-frame', 'frames', header(4, 2, PAL4) + A + image(1, 1, 4, 1, [2] * 4) + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa)], end=err('FrameOutsideScreen')))
add('anim-lzw-error-in-second-frame-keeps-first', 'frames', header(4, 2, PAL4) + A + raw_image(0, 0, 4, 2, 2, [4, 1, 7, 5], widths=[3, 3, 3, 3]) + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa)], end=err('InvalidLzwCode')))
add('anim-error-in-second-frame/still', 'still', header(4, 2, PAL4) + A + image(1, 1, 4, 1, [2] * 4) + TRAILER, still_ok(4, 2, fa))
add('anim-truncated-second-frame', 'frames', (header(4, 2, PAL4) + A + B)[:-2],
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa)], end=err('TruncatedImageData')))
add('anim-netscape-loop', 'frames', header(4, 2, PAL4) + netscape + A + B + TRAILER,
    frames_ok(4, 2, [frame((0, 0, 4, 2), fa), frame((1, 0, 2, 1), grid('rggr/rrrr'))], loop=5))

# ---------------------------------------------------------------- truncation sweep of one two-frame file
# Expected error from the generator's own layout map of the file.
pieces = [('hdr', header(4, 2, PAL4)[:13]), ('gct', PAL4), ('gce', gce(disposal=1, delay=3)), ('img', A), ('img', B), ('trl', TRAILER)]
full = b''.join(p for _, p in pieces)
layout, off = [], 0
for kind, p in pieces:
    layout.append((kind, off, off + len(p), p))
    off += len(p)


def cut_error(n):
    for kind, a, z, p in layout:
        if n >= z:
            continue
        if n <= a and kind in ('gce', 'img', 'trl'):
            return 'MissingTrailer'
        k = n - a
        if kind == 'hdr':
            return 'TruncatedHeader'
        if kind == 'gct':
            return 'TruncatedColorTable'
        if kind == 'gce':
            return 'TruncatedExtension'
        if kind == 'img':
            return 'TruncatedImageDescriptor' if k < 10 else 'TruncatedImageData'
    return None


for n in range(len(full)):
    e = cut_error(n)
    first_done = n >= layout[3][2]
    if n == layout[3][2] or n == layout[4][2]:
        e = 'MissingTrailer'
    nframes = (1 if n >= layout[3][2] else 0) + (1 if n >= layout[4][2] else 0)
    fr = [frame((0, 0, 4, 2), fa, delay=3, disposal=1), frame((1, 0, 2, 1), grid('rggr/rrrr'))][:nframes]
    if e in ('TruncatedHeader', 'TruncatedColorTable') or n <= layout[3][1]:
        add(f'cut-{n}', 'frames', full[:n], err(e))   # G15: no image descriptor reached, the constructor fails
    else:
        add(f'cut-{n}', 'frames', full[:n], frames_ok(4, 2, fr, end=err(e)))
    add(f'cut-{n}/still', 'still', full[:n], still_ok(4, 2, fa) if first_done else err(e))

# TooManyFrames: 65,536 one-pixel frames are accepted, the 65,537th descriptor is rejected.
one = image(0, 0, 1, 1, [1])
hdr1 = header(1, 1, PAL4)
for n, end in ((65_536, 'trailer'), (65_537, err('TooManyFrames'))):
    V.append({'name': f'frames-{n}', 'entry': 'frames', 'input_hex': hdr1.hex(), 'repeat_hex': one.hex(), 'repeat': n,
              'suffix_hex': TRAILER.hex(),
              'expected': {'summary': True, 'width': 1, 'height': 1, 'frame_count': min(n, 65_536),
                           'last_rgba_runs': runs([C['r']]), 'end': end}})

# ---------------------------------------------------------------- loop count: sub-block scan (Q15)
two_loops = b'\x21\xff\x0bNETSCAPE2.0\x03\x01\x03\x00\x03\x01\x09\x00\0'
buffering = b'\x21\xff\x0bNETSCAPE2.0\x05\x02\x00\x10\x00\x00\x03\x01\x07\x00\0'
both('netscape-two-loop-subblocks-last-wins', header(4, 3, PAL4) + two_loops + image(0, 0, 4, 3, ix) + TRAILER,
     4, 3, from_indices(ix, PAL4), loop=9)
f1 = frame((0, 0, 4, 3), from_indices(ix, PAL4))
add('netscape-buffering-subblock-ignored/frames@' + ICC, 'frames', header(4, 3, PAL4) + buffering + image(0, 0, 4, 3, ix) + TRAILER,
    rec(frames_ok(4, 3, [f1], loop=None)), ICC)            # loop_scan=FirstSubBlockOnly: the buffering block hides the count
add('netscape-two-loop-subblocks-first-only/frames@' + ICC, 'frames', header(4, 3, PAL4) + two_loops + image(0, 0, 4, 3, ix) + TRAILER,
    rec(frames_ok(4, 3, [f1], loop=3)), ICC)
add('netscape-buffering-subblock-ignored/frames@' + RB, 'frames', header(4, 3, PAL4) + buffering + image(0, 0, 4, 3, ix) + TRAILER,
    rec(frames_ok(4, 3, [f1], loop=7)), RB)

# ---------------------------------------------------------------- plays() over the raw loop count (G20)
def plays_exp(loop):
    if loop is None:
        return {'loop_count': None, 'plays': 'Once', 'total_plays': 1}
    if loop == 0:
        return {'loop_count': 0, 'plays': 'Forever', 'total_plays': None}
    return {'loop_count': loop, 'plays': {'Repeats': loop}, 'total_plays': loop + 1}


add('plays-no-netscape', 'plays', base, plays_exp(None))
for n in (0, 1, 5, 65535):
    add(f'plays-loop-{n}', 'plays', header(4, 3, PAL4) + netscape.replace(le(5) + b'\0', le(n) + b'\0') + image(0, 0, 4, 3, ix) + TRAILER,
        plays_exp(n))
add('plays-netscape-after-first-image', 'plays', header(4, 3, PAL4) + image(0, 0, 4, 3, ix) + netscape + TRAILER, plays_exp(None))
add('plays-animexts', 'plays', header(4, 3, PAL4) + netscape.replace(b'NETSCAPE2.0', b'ANIMEXTS1.0') + image(0, 0, 4, 3, ix) + TRAILER,
    plays_exp(None))

# ---------------------------------------------------------------- wire-maximum screens and coordinates (Q8)
row = [(x // 1000) % 4 for x in range(65535)]
both('screen-65535x1', header(65535, 1, PAL4) + image(0, 0, 65535, 1, row) + TRAILER, 65535, 1, from_indices(row, PAL4))
col = [(y // 1000) % 4 for y in range(65535)]
both('screen-1x65535-interlaced', header(1, 65535, PAL4) + image(0, 0, 1, 65535, col, interlaced=True) + TRAILER, 1, 65535,
     from_indices(col, PAL4), {'rect': (0, 0, 1, 65535), 'interlaced': True})
wide = [((x // 4096) + y) % 4 for y in range(100) for x in range(32768)]
both('screen-32768x100', header(32768, 100, PAL4) + image(0, 0, 32768, 100, wide) + TRAILER, 32768, 100, from_indices(wide, PAL4))
add('screen-65535x256-area-cap-exact', 'still', header(65535, 256, PAL4) + TRAILER, err('NoImage'))
add('screen-65535x256-area-cap-exact/frames', 'frames', header(65535, 256, PAL4) + TRAILER, frames_ok(65535, 256, []))
add('screen-65535x257-output-too-large', 'still', header(65535, 257, PAL4) + TRAILER, err('OutputTooLarge'))
add('screen-65535x65535-output-too-large', 'still', header(65535, 65535, PAL4) + TRAILER, err('OutputTooLarge'))
both('frame-touches-right-edge-wire-max', header(65535, 1, PAL4) + image(65534, 0, 1, 1, [3]) + TRAILER, 65535, 1,
     [C['.']] * 65534 + [C['b']], {'rect': (65534, 0, 1, 1)})
far = header(65535, 1, PAL4) + image(65535, 0, 65535, 1, [1] * 65535) + TRAILER     # left + w = 131,070
add('frame-left-65535-width-65535', 'still', far, err('FrameOutsideScreen'))
add('frame-left-65535-width-65535/frames@' + RB, 'frames', far,
    rec(frames_ok(65535, 1, [frame((65535, 0, 65535, 1), [C['.']] * 65535)]), True), RB)   # every position clipped (E1)

# ---------------------------------------------------------------- clipped disposal 2 entirely beyond the screen (Q18 regression)
for left in (5, 65535):
    add(f'clip-dispose2-left-edge-beyond-screen-{left}/frames@' + RB, 'frames',
        header(4, 2, PAL4) + gce(disposal=2) + image(left, 0, 2, 1, [2, 2]) + B + TRAILER,
        rec(frames_ok(4, 2, [frame((left, 0, 2, 1), grid('..../....'), disposal=2), frame((1, 0, 2, 1), grid('.gg./....'))]),
            True, False), RB)

# ---------------------------------------------------------------- RecommendedBeta: LZW minimum code size 1-11, checked literals (Q6)
add('lzw-min-code-1-1x1@' + RB, 'still', header(1, 1, PAL2) + raw_image(0, 0, 1, 1, 1, [2, 1, 3], widths=[2, 2, 2]) + TRAILER,
    rec(still_ok(1, 1, [C['k']])), RB)
add('lzw-min-code-9@' + RB, 'still', header(2, 1, PAL2) + raw_image(0, 0, 2, 1, 9, [512, 0, 1, 513]) + TRAILER,
    rec(still_ok(2, 1, [C['w'], C['k']])), RB)
both_profile('lzw-min-code-10', RB, header(4, 3, PAL4) + image(0, 0, 4, 3, ix, m=10) + TRAILER, 4, 3, from_indices(ix, PAL4))
both_profile('lzw-min-code-11', RB, header(4, 3, PAL4) + image(0, 0, 4, 3, ix, m=11) + TRAILER, 4, 3, from_indices(ix, PAL4))
add('lzw-min-code-9-literal-300@' + RB, 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 9, [512, 300, 7, 513]) + TRAILER,
    err('ColorIndexOutOfRange'), RB)
add('lzw-min-code-12@' + RB, 'still', header(2, 1, PAL2) + raw_image(0, 0, 2, 1, 2, [4, 0, 1, 5])[:10] + b'\x0c\x00' + TRAILER,
    err('InvalidLzwMinCodeSize'), RB)

# ---------------------------------------------------------------- RecommendedBeta: stopping at the raster keeps sub-block structure (Q1)
add('image-data-no-terminator@' + RB, 'still', (header(4, 3, PAL4) + image(0, 0, 4, 3, ix))[:-1], err('TruncatedImageData'), RB)
add('lzw-data-after-eoi@' + RB, 'still', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 5], extra=b'\0') + TRAILER,
    rec(still_ok(2, 1, [C['r'], C['g']])), RB)

V = [v for v in V if v is not None]
if __name__ == '__main__':
    (ROOT / 'VECTORS.json').write_text(json.dumps(V, indent=1) + '\n')
    print(len(V), 'vectors')
