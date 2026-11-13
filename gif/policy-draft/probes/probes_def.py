"""Targeted probe inputs for the GIF option catalogue (OPTIONS.md cites them by name).

Each probe is (name, bytes, question). Builders come from spec-draft/gen_vectors.py.
Screen 4x2 with PAL4 (0 black, 1 red, 2 green, 3 blue) unless stated otherwise.
"""
import gen_vectors as G
from gen_vectors import header, image, raw_image, gce, le, TRAILER, PAL4, PAL2, LOCAL4

H = header(4, 2, PAL4)
RED = image(0, 0, 4, 2, [1] * 8)              # full-screen red frame
GRN = lambda l, t, w, h: image(l, t, w, h, [2] * (w * h))  # noqa: E731
PROBES = []


def p(name, b, why):
    PROBES.append((name, b, why))


# --- frame geometry
p('clip-right', H + image(2, 0, 4, 2, [2, 3, 2, 3, 2, 3, 2, 3]) + TRAILER, 'frame crosses the right edge: clip or reject')
p('clip-bottom', H + image(0, 1, 4, 2, [2] * 4 + [3] * 4) + TRAILER, 'frame crosses the bottom edge')
p('clip-entirely-outside', H + RED + image(10, 10, 2, 2, [2] * 4) + TRAILER, 'second frame wholly outside the screen')
p('first-frame-entirely-outside', H + image(10, 10, 2, 2, [2] * 4) + TRAILER, 'only frame wholly outside the screen')
p('frame-larger-than-screen', H + image(0, 0, 6, 3, [2] * 18) + TRAILER, 'frame larger than the screen at origin')
p('clip-dispose2', H + RED + gce(disposal=2) + image(2, 0, 4, 2, [2] * 8) + GRN(0, 0, 1, 1) + TRAILER,
  'disposal 2 on a frame crossing the right edge, then another frame')
p('clip-dispose2-x-beyond', H + RED + gce(disposal=2) + image(6, 0, 2, 2, [2] * 4) + GRN(0, 0, 1, 1) + TRAILER,
  'disposal 2 on a frame whose left edge is beyond the screen width (GDK memset length)')
p('clip-dispose3', H + RED + gce(disposal=3) + image(2, 0, 4, 2, [2] * 8) + GRN(0, 0, 1, 1) + TRAILER,
  'disposal 3 on a frame crossing the right edge (GDK copy_area bounds)')
p('screen-zero', header(0, 0, PAL4) + image(0, 0, 2, 2, [1, 2, 3, 1]) + TRAILER, 'zero logical screen')
p('screen-zero-width', header(0, 2, PAL4) + image(0, 0, 2, 2, [1, 2, 3, 1]) + TRAILER, 'zero logical screen width')
p('frame-zero-width-first', H + raw_image(0, 0, 0, 2, 2, [4, 5]) + RED + TRAILER, 'zero-width first frame then a valid one')
p('frame-zero-width-second', H + RED + raw_image(0, 0, 0, 2, 2, [4, 5]) + GRN(0, 0, 1, 1) + TRAILER, 'zero-width second frame')

# --- disposal values
for d in range(8):
    p(f'disposal-{d}', H + RED + gce(disposal=d) + GRN(1, 0, 2, 1) + image(0, 1, 2, 1, [3, 3]) + TRAILER,
      f'disposal {d} on the middle frame')

# --- colour tables and indices
p('index-oob-global', header(2, 1, PAL2) + image(0, 0, 2, 1, [1, 2]) + TRAILER, 'index 2 with a 2-entry global table')
p('index-oob-global-anim', header(2, 1, PAL4) + image(0, 0, 2, 1, [1, 1]) + image(0, 0, 2, 1, [0, 2], lct=PAL2) + TRAILER,
  'index 2 in a 2-entry local table over a red frame')
big_local = bytes((i * 11) % 256 for i in range(768))
p('index-oob-stale-local', header(2, 1, PAL4) + image(0, 0, 2, 1, [5, 6], m=8, lct=big_local)
  + image(0, 0, 2, 1, [0, 5], m=3, lct=PAL2) + TRAILER, 'GDK keeps stale entries of an earlier, larger local table')
p('index-oob-first-local', header(2, 1) + image(0, 0, 2, 1, [0, 3], lct=PAL2) + TRAILER, 'index 3 in a 2-entry local table, no global')
p('no-table-at-all', header(2, 1) + image(0, 0, 2, 1, [0, 1]) + TRAILER, 'neither table')
p('no-table-second-frame', header(2, 1) + image(0, 0, 2, 1, [0, 1], lct=PAL2) + image(0, 0, 2, 1, [1, 0]) + TRAILER,
  'second frame without any table')
p('gct-flag-short-file', header(2, 1, PAL4)[:16], 'global table truncated')
p('background-index-5', header(4, 2, PAL4, bg=3) + GRN(1, 0, 2, 1) + TRAILER, 'background index 3 (blue) with an inset frame')

# --- LZW
for m in (1, 9, 10, 11, 12):
    p(f'lzw-min-{m}', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, m, [1 << m, 1, 2, (1 << m) + 1]) + TRAILER,
      f'LZW minimum code size {m}: clear, 1, 2, EOI')
p('lzw-min-0', header(2, 1, PAL4) + b'\x2c' + le(0) + le(0) + le(2) + le(1) + b'\x00\x00\x02\x01\x0e\x00' + TRAILER,
  'LZW minimum code size 0')
p('lzw-min-8-with-4-colours', header(4, 2, PAL4) + image(0, 0, 4, 2, [1, 2, 3, 0] * 2, m=8) + TRAILER, 'm=8 with a 4-entry table')
p('lzw-short-first', H + raw_image(0, 0, 4, 2, 2, [4, 1, 2, 5]) + TRAILER, 'first frame: 2 of 8 pixels then EOI')
p('lzw-short-second', H + RED + raw_image(0, 0, 4, 2, 2, [4, 2, 3, 5]) + TRAILER, 'second frame: 2 of 8 pixels then EOI')
p('lzw-short-no-eoi', H + raw_image(0, 0, 4, 2, 2, [4, 2, 3], widths=[3, 3, 3]) + TRAILER, 'first frame: 2 pixels, data ends')
p('lzw-empty-terminator-only', H + b'\x2c' + le(0) + le(0) + le(4) + le(2) + b'\x00\x02\x00' + TRAILER, 'no data sub-block')
p('lzw-empty-terminator-only-second', H + RED + b'\x2c' + le(0) + le(0) + le(4) + le(2) + b'\x00\x02\x00' + TRAILER,
  'second frame without data sub-block')
p('lzw-missing-eoi-exact', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2], widths=[3, 3, 3]) + TRAILER,
  'raster complete, zero padding, no EOI')
p('lzw-excess-pixels', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 3, 3, 5]) + TRAILER, 'two extra pixels then EOI')
p('lzw-excess-in-string', header(3, 1, PAL4) + raw_image(0, 0, 3, 1, 2, [4, 1, 2, 6, 5]) + TRAILER, 'last string crosses the raster end')
p('lzw-data-after-eoi', header(2, 1, PAL4) + raw_image(0, 0, 2, 1, 2, [4, 1, 2, 5], extra=b'\xff\xff') + TRAILER, 'bytes after EOI')
p('lzw-invalid-code-first', H + raw_image(0, 0, 4, 2, 2, [4, 1, 7, 5], widths=[3, 3, 3, 3]) + TRAILER,
  'invalid code after one pixel (GDK 2.42.12 returns output_length)')
p('lzw-invalid-code-second', H + RED + raw_image(0, 0, 4, 2, 2, [4, 2, 7, 5], widths=[3, 3, 3, 3]) + TRAILER,
  'invalid code in the second frame after one pixel')
p('lzw-invalid-first-is-next', H + raw_image(0, 0, 4, 2, 2, [4, 6, 5], widths=[3, 3, 3]) + TRAILER, 'first code equals next free')
# TIFF-style early change: widen one code early (width 4 already for the code that makes the table 7 entries)
codes = [4, 1, 2, 3, 1, 2, 3, 1, 2, 5]
p('lzw-early-change', H + raw_image(0, 0, 4, 2, 2, codes, widths=[3, 3, 3, 4, 4, 4, 4, 4, 4, 4]) + TRAILER,
  'encoder widened one code early (TIFF style)')
p('lzw-late-change', H + raw_image(0, 0, 4, 2, 2, codes, widths=[3, 3, 3, 3, 3, 4, 4, 4, 4, 4]) + TRAILER,
  'encoder widened one code late')

# --- stream structure
p('trailing-garbage', H + RED + TRAILER + b'garbage', 'bytes after the trailer')
p('missing-trailer-one', H + RED, 'single frame, no trailer')
p('missing-trailer-two', H + RED + GRN(0, 0, 2, 1), 'two frames, no trailer')
p('truncated-first-data', (H + RED)[:-3], 'truncated inside the first frame data')
p('truncated-second-data', (H + RED + image(0, 0, 4, 2, list(range(4)) * 2))[:-3], 'truncated inside the second frame data')
p('truncated-second-descriptor', H + RED + b'\x2c\x00\x00', 'truncated inside the second descriptor')
p('stray-byte-between', H + RED + b'\x00' + GRN(0, 0, 2, 1) + TRAILER, 'stray 0x00 between blocks')
p('stray-bytes-before-first', H + b'\x01\x02' + RED + TRAILER, 'stray bytes before the first block')
p('unknown-ext-label', H + b'\x21\x99\x03abc\x00' + RED + TRAILER, 'unknown extension label 0x99')
p('unknown-ext-graphic-label', H + gce(transparent=1) + b'\x21\x02\x01z\x00' + RED + RED.replace(b'\x2c', b'\x2c', 1) + TRAILER,
  'GCE, unknown label 0x02, then two red frames')
p('gce-size-5', H + b'\x21\xf9\x05\x05\x00\x00\x01\x00\x00' + image(0, 0, 4, 2, [1, 2] * 4) + TRAILER, 'GCE block size 5 (transparent 1)')
p('gce-size-3', H + b'\x21\xf9\x03\x01\x00\x00\x00' + image(0, 0, 4, 2, [1, 2] * 4) + TRAILER, 'GCE block size 3')
p('gce-duplicate', H + gce(transparent=1) + gce(transparent=2) + image(0, 0, 4, 2, [1, 2] * 4) + TRAILER, 'two GCEs: which wins')
p('gce-sticky', H + RED + gce(transparent=2, delay=7) + image(0, 0, 4, 2, [2, 3] * 4) + image(0, 0, 4, 2, [2] * 8) + TRAILER,
  'GCE on frame 2 only; frame 3 index 2 should be opaque green')
p('gce-before-plaintext', H + RED + gce(transparent=2) + G.plain + image(0, 0, 4, 2, [2] * 8) + TRAILER,
  'GCE belongs to the plain-text block, not to the next image')
p('gce-without-image', H + RED + gce(transparent=1) + TRAILER, 'GCE then trailer')
p('gce-87a', header(4, 2, PAL4, version=b'87a') + gce(transparent=1) + image(0, 0, 4, 2, [1, 2] * 4) + TRAILER, '89a GCE in an 87a file')
p('version-88a', b'GIF88a' + H[6:] + RED + TRAILER, 'version 88a')
p('reserved-bits', H + gce(reserved=7) + image(0, 0, 4, 2, [1] * 8, extra_flags=0x18) + TRAILER, 'reserved bits set')
for d in (0, 1, 2, 5, 10):
    p(f'delay-{d}', H + gce(delay=d) + RED + gce(delay=d) + GRN(0, 0, 1, 1) + TRAILER, f'delay {d} cs on both frames')
p('no-gce-delays', H + RED + GRN(0, 0, 1, 1) + TRAILER, 'two frames without GCE')
ns = lambda n: b'\x21\xff\x0bNETSCAPE2.0\x03\x01' + le(n) + b'\x00'  # noqa: E731
for n in (0, 1, 2):
    p(f'loop-{n}', H + ns(n) + gce(delay=10) + RED + gce(delay=10) + GRN(0, 0, 1, 1) + TRAILER, f'NETSCAPE loop count {n}')
p('loop-animexts', H + ns(2).replace(b'NETSCAPE2.0', b'ANIMEXTS1.0') + gce(delay=10) + RED + gce(delay=10) + GRN(0, 0, 1, 1) + TRAILER,
  'ANIMEXTS1.0 loop 2')
p('duplicate-identical-frames', H + gce(delay=10) + RED + gce(delay=10) + RED + TRAILER, 'two identical frames')
p('interlace-h5', header(1, 5, PAL4) + image(0, 0, 1, 5, [0, 1, 2, 3, 1], interlaced=True) + TRAILER, 'interlaced height 5')
p('interlace-short', header(1, 9, PAL4) + image(0, 0, 1, 9, interlaced=True, data=G.pack([4, 1, 2, 3, 5], G.widths_for([4, 1, 2, 3, 5], 2)))
  + TRAILER, 'interlaced 1x9 with 3 of 9 rows')
