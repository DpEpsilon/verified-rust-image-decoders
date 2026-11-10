#!/usr/bin/env python3
"""Spec oracle for farbfeld, written for this harness from the format description (not a third-party decoder):
8-byte magic 'farbfeld', u32be width, u32be height, then width*height*4 big-endian u16 (RGBA, non-premultiplied).
The description is silent on trailing data and zero dimensions: this oracle ignores trailing bytes and treats a
zero dimension as the harness's common 'empty-image' error. Usage: ff_oracle.py FILE [RGBA8_DUMP]"""
import hashlib, json, struct, sys
d = open(sys.argv[1], 'rb').read()
def out(o): print(json.dumps(o))
if len(d) < 16 or d[:8] != b'farbfeld':
    out({'status': 'error', 'error_class': 'BadHeader', 'message': 'magic or header missing'}); sys.exit(0)
w, h = struct.unpack('>II', d[8:16])
if w == 0 or h == 0:
    out({'status': 'error', 'error_class': 'empty-image', 'message': 'zero width or height'}); sys.exit(0)
if w * h > 16_000_000:
    out({'status': 'error', 'error_class': 'harness-limit', 'message': 'more than 16,000,000 pixels; not decoded'}); sys.exit(0)
if len(d) - 16 < 8 * w * h:
    out({'status': 'error', 'error_class': 'Truncated', 'message': 'body shorter than 8*w*h'}); sys.exit(0)
v = struct.unpack('>%dH' % (4 * w * h), d[16:16 + 8 * w * h])
r8 = bytes((x + 128) // 257 for x in v)
r16 = struct.pack('<%dH' % len(v), *v)
if len(sys.argv) > 2: open(sys.argv[2], 'wb').write(r8)
out({'status': 'ok', 'width': w, 'height': h, 'orig_color_type': 'Rgba16', 'alpha_present': any(r8[i] != 255 for i in range(3, len(r8), 4)),
     'rgba_sha256': hashlib.sha256(r8).hexdigest(), 'rgba16_sha256': hashlib.sha256(r16).hexdigest(), 'selection': 'single'})
