#!/usr/bin/env python3
"""WebP adapter: seeds come from runners/mkseeds.c (libwebp encoder/mux/anim encoder), mutations are
RIFF/VP8X/ANIM/ANMF/ALPH/VP8/VP8L field and structure probes."""
import os, random, struct, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'modern'))
import common as c

BIN = Path(os.environ.get('LENIENCY_WEBP_BIN', '/tmp/claude-0/leniency-webp/bin'))
RUST = Path(os.environ.get('LENIENCY_TARGET', '/tmp/claude-0/target-modern')) / 'release/webp-runner'
CONTAINERS = {b'VP8X', b'ANIM', b'ANMF', b'ALPH', b'VP8 ', b'VP8L', b'ICCP', b'EXIF', b'XMP '}


def chunks(b, start=12, end=None, depth=0):
    """Yield (fourcc, header_offset, size, payload_offset, depth) for RIFF chunks; descends into ANMF."""
    end = len(b) if end is None else min(end, len(b))
    p = start
    while p + 8 <= end:
        tag, size = b[p:p + 4], c.u(b, p + 4, 4)
        yield tag, p, size, p + 8, depth
        if tag == b'ANMF' and depth == 0:
            yield from chunks(b, p + 24, min(p + 8 + size, end), depth + 1)
        p += 8 + size + (size & 1)


def fields(b):
    f = [('riff_size', 4, 4, 'little')]
    for tag, h, size, pl, depth in chunks(b):
        t = tag.decode('latin1').strip()
        f.append((f'{t}_fourcc', h, 4, 'big'))
        f.append((f'{t}_size', h + 4, 4, 'little'))
        if tag == b'VP8X':
            f += [('vp8x_flags', pl, 1, 'little'), ('vp8x_reserved', pl + 1, 3, 'little'),
                  ('vp8x_canvas_w_minus1', pl + 4, 3, 'little'), ('vp8x_canvas_h_minus1', pl + 7, 3, 'little')]
        elif tag == b'ANIM':
            f += [('anim_bgcolor', pl, 4, 'little'), ('anim_loop_count', pl + 4, 2, 'little')]
        elif tag == b'ANMF':
            f += [('anmf_x', pl, 3, 'little'), ('anmf_y', pl + 3, 3, 'little'), ('anmf_w_minus1', pl + 6, 3, 'little'),
                  ('anmf_h_minus1', pl + 9, 3, 'little'), ('anmf_duration', pl + 12, 3, 'little'), ('anmf_flags', pl + 15, 1, 'little')]
        elif tag == b'ALPH':
            f.append(('alph_header', pl, 1, 'little'))
        elif tag == b'VP8 ':
            f += [('vp8_frame_tag', pl, 3, 'little'), ('vp8_start_code', pl + 3, 3, 'big'), ('vp8_width_scale', pl + 6, 2, 'little'),
                  ('vp8_height_scale', pl + 8, 2, 'little')]
        elif tag == b'VP8L':
            f += [('vp8l_signature', pl, 1, 'little'), ('vp8l_dims_flags', pl + 1, 4, 'little')]
    return f


def cut_points(b):
    pts = {12}
    for tag, h, size, pl, d in chunks(b):
        pts |= {h, h + 4, h + 8, pl + size, pl + size + (size & 1)}
    return sorted(p for p in pts if 0 < p < len(b))


def chunk_list(b):
    """top-level chunks as raw bytes (header+payload+pad)."""
    out = []
    for tag, h, size, pl, d in chunks(b):
        if d == 0:
            out.append((tag, b[h:pl + size + (size & 1)]))
    return out


def rebuild(parts, tail=b''):
    body = b'WEBP' + b''.join(parts)
    return b'RIFF' + struct.pack('<I', len(body)) + body + tail


def mk_chunk(tag, payload, pad=True):
    return tag + struct.pack('<I', len(payload)) + payload + (b'\0' if len(payload) & 1 and pad else b'')


def vp8x(w, h, flags):
    return mk_chunk(b'VP8X', bytes([flags, 0, 0, 0]) + (w - 1).to_bytes(3, 'little') + (h - 1).to_bytes(3, 'little'))


def structural(b, rng):
    ch = chunk_list(b)
    n = len(b)
    tags = [t for t, _ in ch]
    parts = [x for _, x in ch]
    def R(rule, field, data, old=None, new=None):
        return data, {'rule': rule, 'field': field, 'old': old, 'new': new}
    # RIFF magic / form type
    for i, (name, off, val) in enumerate([('riff_magic', 0, b'RIFX'), ('riff_magic', 0, b'RF64'), ('riff_magic', 0, b'riff'),
                                          ('webp_form', 8, b'WEBQ'), ('webp_form', 8, b'webp'), ('webp_form', 8, b'WEB\0')]):
        yield R('riff-magic', name, b[:off] + val + b[off + len(val):], new=val.decode('latin1'))
    # riff size relative to actual
    for d in (-9, -8, -2, -1, 1, 2, 7, 8, 9, 1000):
        yield R('riff-size-delta', f'{d:+d}', b[:4] + struct.pack('<I', max(0, n - 8 + d)) + b[8:], n - 8, n - 8 + d)
    # drop / duplicate / swap / insert chunks
    for i, t in enumerate(tags):
        yield R('chunk-delete', t.decode('latin1').strip(), rebuild(parts[:i] + parts[i + 1:]))
        yield R('chunk-duplicate', t.decode('latin1').strip(), rebuild(parts[:i + 1] + parts[i:]))
        if i + 1 < len(parts):
            yield R('chunk-swap', f'{t.decode("latin1").strip()}<->{tags[i+1].decode("latin1").strip()}',
                    rebuild(parts[:i] + [parts[i + 1], parts[i]] + parts[i + 2:]))
    for i in range(len(parts) + 1):
        for tag, pl in [(b'JUNK', b'xyz'), (b'abcd', b'\1\2\3\4'), (b'EXIF', b'II*\0'), (b'XMP ', b'<x/>'), (b'ICCP', b'\0' * 8), (b'ANIM', b'\0' * 6), (b'VP8X', b'\0' * 10)]:
            yield R('chunk-insert', f'{tag.decode()}@{i}', rebuild(parts[:i] + [mk_chunk(tag, pl)] + parts[i:]))
    # odd-size chunk padding: remove or add pad bytes
    for i, (t, x) in enumerate(ch):
        size = c.u(x, 4, 4)
        if size & 1 and len(x) == 8 + size + 1:
            yield R('padding-missing', t.decode('latin1').strip(), rebuild(parts[:i] + [x[:-1]] + parts[i + 1:]))
            yield R('padding-nonzero', t.decode('latin1').strip(), rebuild(parts[:i] + [x[:-1] + b'\x55'] + parts[i + 1:]))
        else:
            yield R('padding-extra', t.decode('latin1').strip(), rebuild(parts[:i] + [x + b'\0'] + parts[i + 1:]))
    # chunk size one larger / smaller than payload, riff size kept consistent via rebuild
    for i, (t, x) in enumerate(ch):
        for d in (-1, 1, 2):
            sz = c.u(x, 4, 4)
            if sz + d >= 0:
                y = x[:4] + struct.pack('<I', sz + d) + x[8:]
                yield R('chunk-size-delta', f'{t.decode("latin1").strip()}{d:+d}', rebuild(parts[:i] + [y] + parts[i + 1:]), sz, sz + d)
    # simple <-> extended container conversion
    if tags and tags[0] in (b'VP8 ', b'VP8L'):
        w = h = 0
        pl = parts[0][8:]
        if tags[0] == b'VP8L':
            v = c.u(pl, 1, 4)
            w, h = (v & 0x3fff) + 1, ((v >> 14) & 0x3fff) + 1
        else:
            w, h = c.u(pl, 6, 2) & 0x3fff, c.u(pl, 8, 2) & 0x3fff
        for flags, nm in ((0, 'vp8x-flags0'), (0x10, 'vp8x-alpha-flag'), (0x02, 'vp8x-anim-flag'), (0x20, 'vp8x-icc-flag')):
            yield R('wrap-vp8x', nm, rebuild([vp8x(max(w, 1), max(h, 1), flags)] + parts), None, flags)
        yield R('wrap-vp8x', 'canvas-larger', rebuild([vp8x(w + 1, h, 0)] + parts))
        yield R('wrap-vp8x', 'canvas-smaller', rebuild([vp8x(max(w - 1, 1), h, 0)] + parts))
    if tags and tags[0] == b'VP8X':
        yield R('unwrap-vp8x', 'drop-vp8x', rebuild(parts[1:]))
        flags = b[20]
        for bit in range(8):
            nb = bytearray(b); nb[20] = flags ^ (1 << bit)
            yield bytes(nb), {'rule': 'vp8x-flag-bit', 'field': f'bit{bit}', 'old': flags, 'new': flags ^ (1 << bit)}
    # frame-structure probes for animations
    if b'ANMF' in tags:
        anmf = [i for i, t in enumerate(tags) if t == b'ANMF']
        for k in (1, 2, 4):
            yield R('anmf-repeat', f'x{k}', rebuild(parts[:anmf[0]] + [parts[anmf[0]]] * k + parts[anmf[0]:]))
        yield R('anmf-reorder', 'reverse', rebuild([p for i, p in enumerate(parts) if i not in anmf] + [parts[i] for i in reversed(anmf)]))
        yield R('anmf-only-first', 'keep1', rebuild([p for i, p in enumerate(parts) if i not in anmf[1:]]))
        # unknown chunk / extra image inside ANMF
        i = anmf[0]
        x = parts[i]
        sz = c.u(x, 4, 4)
        inner = x[8:8 + sz]
        for nm, extra in (('unknown', mk_chunk(b'abcd', b'zz')), ('junk-odd', mk_chunk(b'JUNK', b'abc')), ('second-frame-data', inner[16:])):
            nx = b'ANMF' + struct.pack('<I', sz + len(extra)) + inner + extra
            yield R('anmf-inner-extra', nm, rebuild(parts[:i] + [nx + (b'\0' if len(nx) & 1 else b'')] + parts[i + 1:]))
        nx = b'ANMF' + struct.pack('<I', 16) + inner[:16]
        yield R('anmf-inner-empty', 'no-image-data', rebuild(parts[:i] + [nx] + parts[i + 1:]))
        # frame rectangle vs canvas: x,y beyond canvas; width beyond canvas
        for nm, off, val in (('x-offset-big', 8, 0xfffffe), ('y-offset-big', 11, 0xfffffe), ('x-odd', 8, 1), ('w-plus', 14, 0xff)):
            nb = bytearray(b)
            p = [h for t, h, s, pl, d in chunks(b) if t == b'ANMF'][0] + off
            nb[p:p + 3] = (val & 0xffffff).to_bytes(3, 'little')
            yield bytes(nb), {'rule': 'anmf-geometry', 'field': nm, 'old': None, 'new': val}
    # specialised bit fields
    for tag, h, size, pl, d in chunks(b):
        if tag == b'VP8L' and size >= 5:
            v = c.u(b, pl + 1, 4)
            w1, h1, al, ver = v & 0x3fff, (v >> 14) & 0x3fff, (v >> 28) & 1, v >> 29
            for nm, nv in (('width-minus1=0', v & ~0x3fff), ('width-minus1=max', v | 0x3fff), ('width-minus1+1', (v & ~0x3fff) | ((w1 + 1) & 0x3fff)),
                           ('width-minus1-1', (v & ~0x3fff) | ((w1 - 1) & 0x3fff)), ('height-minus1=max', v | (0x3fff << 14)),
                           ('height-minus1+1', (v & ~(0x3fff << 14)) | (((h1 + 1) & 0x3fff) << 14)), ('height-minus1-1', (v & ~(0x3fff << 14)) | (((h1 - 1) & 0x3fff) << 14)),
                           ('alpha-hint-toggle', v ^ (1 << 28)), ('version=1', (v & 0x1fffffff) | (1 << 29)), ('version=7', v | (7 << 29))):
                yield b[:pl + 1] + struct.pack('<I', nv & 0xffffffff) + b[pl + 5:], {'rule': 'vp8l-header-bits', 'field': nm, 'old': v, 'new': nv}
        if tag == b'VP8 ' and size >= 10:
            tg = c.u(b, pl, 3)
            for nm, nv, off, nbytes in (('keyframe-bit', tg ^ 1, 0, 3), ('version=1', (tg & ~0xe) | 2, 0, 3), ('version=4', (tg & ~0xe) | 8, 0, 3),
                                        ('show-frame=0', tg & ~0x10, 0, 3), ('first-part-size=0', tg & 0x1f, 0, 3), ('first-part-size=max', tg | 0xffffe0, 0, 3),
                                        ('first-part-size+1', ((tg + (1 << 5)) & 0xffffff), 0, 3), ('first-part-size-1', ((tg - (1 << 5)) & 0xffffff), 0, 3)):
                yield b[:pl] + nv.to_bytes(3, 'little') + b[pl + 3:], {'rule': 'vp8-frame-tag', 'field': nm, 'old': tg, 'new': nv}
            ws = c.u(b, pl + 6, 2)
            hs = c.u(b, pl + 8, 2)
            for sc in (1, 2, 3):
                yield b[:pl + 6] + struct.pack('<H', (ws & 0x3fff) | (sc << 14)) + b[pl + 8:], {'rule': 'vp8-scale-bits', 'field': f'w-scale={sc}', 'old': ws, 'new': sc}
                yield b[:pl + 8] + struct.pack('<H', (hs & 0x3fff) | (sc << 14)) + b[pl + 10:], {'rule': 'vp8-scale-bits', 'field': f'h-scale={sc}', 'old': hs, 'new': sc}
            for nm, nv in (('w=0', 0), ('h=0', 0), ('w+1', (ws & 0x3fff) + 1), ('h+1', (hs & 0x3fff) + 1), ('w-1', (ws & 0x3fff) - 1), ('h-1', (hs & 0x3fff) - 1), ('w=16383', 0x3fff)):
                off = pl + 6 if nm[0] == 'w' else pl + 8
                yield b[:off] + struct.pack('<H', nv & 0x3fff) + b[off + 2:], {'rule': 'vp8-dimension', 'field': nm, 'old': ws, 'new': nv}
        if tag == b'ALPH' and size >= 1:
            for v in rng.sample(range(256), 48):
                yield b[:pl] + bytes([v]) + b[pl + 1:], {'rule': 'alph-header-byte', 'field': f'0x{v:02x}', 'old': b[pl], 'new': v}


def mutations(data, meta, rng):
    yield from c.field_mutations(data, fields(data), rng=rng, per_field=7)
    yield from c.generic_mutations(data, rng, cut_points=cut_points(data), nflip=24)
    yield from structural(data, rng)


def seeds():
    out = []
    for p in sorted((HERE / 'seeds').glob('*.webp')):
        d = p.read_bytes()
        out.append((d, {'name': p.stem, 'kind': 'animated' if b'ANMF' in d[:200] else ('lossy' if b'VP8 ' in d[:40] else 'lossless' if b'VP8L' in d[:40] else 'extended')}))
    return out


class WebP(c.Adapter):
    fmt, ext, title = 'webp', 'webp', 'WebP (image/webp)'
    tol = 2
    dir = HERE
    runners = {'glycin': (RUST, ['webp:glycin']), 'image-webp': (RUST, ['webp:image']),
               'libwebp-simple': (BIN / 'libwebp', ['simple']), 'libwebp-anim': (BIN / 'libwebp', ['anim'])}
    groups = {'glycin': 'image-webp', 'image-webp': 'image-webp', 'libwebp-simple': 'libwebp', 'libwebp-anim': 'libwebp'}
    decoders_doc = {
        'glycin': 'glycin-image-rs `ImgLoader` for `image/webp` (image::WebPDecoder, image-webp 0.2.4), frames pulled with `specific_frame` until `NoMoreFrames`',
        'image-webp': '`image::codecs::webp::WebPDecoder` called directly (same code as glycin; `into_frames()` for animation)',
        'libwebp-simple': 'system libwebp 1.5.0 `WebPDecodeRGBA` (still images; animated files unsupported by this API)',
        'libwebp-anim': 'system libwebp 1.5.0 `WebPAnimDecoder` (demux validation + canvas composition, every frame; also used for stills)'}
    unavailable = [('gdk-pixbuf WebP module', 'not installed (module list: ani bmp gif icns ico pnm qtif tga tiff xbm xpm)'),
                   ('`webp` crate', 'wraps libwebp-sys, i.e. the same libwebp code: not an independent vote, not built'),
                   ('independent Rust WebP decoder other than image-webp', 'none found among the crates reachable in this survey; see SURVEY.md')]
    notes = (open(HERE / 'NOTES.md').read() if (HERE / 'NOTES.md').exists() else '')
    seeds = staticmethod(seeds)
    mutations = staticmethod(mutations)


if __name__ == '__main__':
    c.run(WebP())
