"""Targeted probe inputs for the PNG option catalogue (OPTIONS.md cites them by name).

Each probe is (name, bytes, question). Builders come from ../../spec-draft/gen_vectors.py.
Unless stated, the base image is 4x3 RGB8 (`T`) or 4x3 palette-8 with 4 entries (`P`).
"""
import struct
import zlib

import gen_vectors as G
from gen_vectors import SIG, IEND, chunk, ihdr, scan, z, zwrap, stored, fixed, BW, actl, fctl, fdat

PROBES = []


def p(name, b, why):
    PROBES.append((name, b, why))


TW, TH = 4, 3
TPX = [((x * 60) % 256, (y * 90) % 256, (x * y * 30) % 256) for y in range(TH) for x in range(TW)]
TRAW = scan(TPX, TW, TH, 2, 8, 0)
TZ = z(TRAW)
TH_ = SIG + ihdr(TW, TH, 8, 2)
TIDAT = chunk(b'IDAT', TZ)
T = TH_ + TIDAT + IEND
PAL = bytes([255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255])
PPX = [((x + y) % 4,) for y in range(TH) for x in range(TW)]
PH = SIG + ihdr(TW, TH, 8, 3)
PIDAT = chunk(b'IDAT', z(scan(PPX, TW, TH, 3, 8, 0)))
P = PH + chunk(b'PLTE', PAL) + PIDAT + IEND


def t(*mid, post=b''):
    return TH_ + b''.join(mid) + TIDAT + post + IEND


def bad_crc(c):
    return c[:-4] + struct.pack('>I', struct.unpack('>I', c[-4:])[0] ^ 1)


# --- CRC
p('base-rgb8', T, 'reference image')
p('base-palette', P, 'reference palette image')
p('crc-idat', TH_ + bad_crc(TIDAT) + IEND, 'bad CRC on IDAT (critical)')
p('crc-plte', PH + bad_crc(chunk(b'PLTE', PAL)) + PIDAT + IEND, 'bad CRC on PLTE (critical)')
p('crc-ihdr', SIG + bad_crc(ihdr(TW, TH, 8, 2)) + TIDAT + IEND, 'bad CRC on IHDR')
p('crc-iend', TH_ + TIDAT + bad_crc(IEND), 'bad CRC on IEND')
p('crc-text', t(bad_crc(chunk(b'tEXt', b'Title\0x'))), 'bad CRC on tEXt (ancillary)')
p('crc-gama', t(bad_crc(chunk(b'gAMA', struct.pack('>I', 100000)))), 'bad CRC on gAMA (ancillary)')
p('crc-trns', PH + chunk(b'PLTE', PAL) + bad_crc(chunk(b'tRNS', b'\0')) + PIDAT + IEND, 'bad CRC on tRNS (ancillary that changes pixels)')
p('crc-unknown-ancillary', t(bad_crc(chunk(b'prVt', b'abc'))), 'bad CRC on an unknown ancillary chunk')
p('crc-text-after-idat', TH_ + TIDAT + bad_crc(chunk(b'tEXt', b'a\0b')) + IEND, 'bad CRC on tEXt after IDAT')
# --- zlib
p('adler-bad', TH_ + chunk(b'IDAT', TZ[:-1] + bytes([TZ[-1] ^ 1])) + IEND, 'wrong Adler-32')
p('adler-missing', TH_ + chunk(b'IDAT', TZ[:-4]) + IEND, 'zlib stream without its Adler-32')
p('adler-partial', TH_ + chunk(b'IDAT', TZ[:-2]) + IEND, 'zlib stream with half an Adler-32')
p('zlib-trailing-in-idat', TH_ + chunk(b'IDAT', TZ + b'\0\0\0') + IEND, 'bytes after the zlib stream inside IDAT')
p('zlib-extra-idat-zeros', TH_ + TIDAT + chunk(b'IDAT', bytes(16)) + IEND, 'a further IDAT after the zlib stream ends')
p('zlib-second-stream', TH_ + TIDAT + chunk(b'IDAT', TZ) + IEND, 'a second complete zlib stream in a later IDAT')
p('zlib-fcheck', TH_ + chunk(b'IDAT', bytes([0x78, 0x9d]) + TZ[2:]) + IEND, 'bad FCHECK')
p('zlib-cm7', TH_ + chunk(b'IDAT', zwrap(stored(TRAW), TRAW, cmf=0x77)) + IEND, 'CM = 7')
p('zlib-cinfo8', TH_ + chunk(b'IDAT', zwrap(stored(TRAW), TRAW, cmf=0x88)) + IEND, 'CINFO = 8')
p('zlib-fdict', TH_ + chunk(b'IDAT', bytes([0x78, 0xBB]) + b'\0\0\0\1' + stored(TRAW) + struct.pack('>I', zlib.adler32(TRAW))) + IEND,
  'FDICT set')
p('zlib-raw-deflate', TH_ + chunk(b'IDAT', TZ[2:-4]) + IEND, 'raw DEFLATE without zlib wrapper')
p('zlib-window-too-small', SIG + ihdr(402, 1, 8, 0) + chunk(b'IDAT', G.zwrap(G.bwf.flush(), G.payload, cmf=0x08)) + IEND,
  'a distance of 300 under CINFO=0 (256-byte window)')
p('deflate-btype3', TH_ + chunk(b'IDAT', zwrap(bytes([0b111]), b'')) + IEND, 'block type 3')
p('deflate-incomplete-dist-code', G.tiny(zwrap(G.dyn_lengths_only(G.lt, [2, 2, 2]) + b'\0' * 8, b'')), 'incomplete distance code')
p('deflate-single-literal-code', G.tiny(zwrap(G.bw1.flush() + struct.pack('<HH', 4, 4 ^ 0xFFFF) + G.TINY_RAW, G.TINY_RAW)),
  'literal/length code with one code of length 1 (zlib accepts)')
p('deflate-empty-fixed-block', G.tiny(zwrap(bytes([0x03, 0x00]), b'')), 'a zlib stream holding one empty fixed block')
# --- truncation and amount of data
p('truncated-mid-idat', (TH_ + TIDAT)[:-8], 'file cut inside IDAT')
p('truncated-after-idat', TH_ + TIDAT, 'file ends after the IDAT run (missing IEND)')
p('missing-iend-after-text', TH_ + TIDAT + chunk(b'tEXt', b'a\0b'), 'missing IEND after a trailing text chunk')
p('too-little-data', TH_ + chunk(b'IDAT', z(TRAW[:-4])) + IEND, 'complete zlib stream, last row short')
p('too-little-data-one-row', TH_ + chunk(b'IDAT', z(TRAW[:13])) + IEND, 'complete zlib stream with one row of three')
p('excess-data-byte', TH_ + chunk(b'IDAT', z(TRAW + b'\0')) + IEND, 'one byte more than the image needs')
p('excess-data-row', TH_ + chunk(b'IDAT', z(TRAW + TRAW[:13])) + IEND, 'one row more than the image needs')
p('trailing-garbage', T + b'garbage', 'bytes after IEND')
p('trailing-second-png', T + T, 'a second PNG after IEND')
p('trailing-iend-length-1', TH_ + TIDAT + chunk(b'IEND', b'\0'), 'IEND with length 1')
p('zero-length-idat-between', TH_ + chunk(b'IDAT', TZ[:9]) + chunk(b'IDAT', b'') + chunk(b'IDAT', TZ[9:]) + IEND, 'empty IDAT inside the run')
p('zero-length-idat-first', TH_ + chunk(b'IDAT', b'') + TIDAT + IEND, 'empty first IDAT')
p('zero-length-idat-only', TH_ + chunk(b'IDAT', b'') + IEND, 'only an empty IDAT')
p('no-idat', TH_ + IEND, 'no IDAT at all')
p('idat-1-byte-chunks', TH_ + G.split_idat(TZ, [1]) + IEND, 'IDAT split into 1-byte chunks')
p('non-consecutive-idat', TH_ + chunk(b'IDAT', TZ[:9]) + chunk(b'tEXt', b'a\0b') + chunk(b'IDAT', TZ[9:]) + IEND, 'text chunk between IDATs')
# --- order and duplicates
GAMA = chunk(b'gAMA', struct.pack('>I', 45455))
p('order-gama-after-plte', TH_ + chunk(b'PLTE', PAL) + GAMA + TIDAT + IEND, 'gAMA after PLTE')
p('order-gama-after-idat', TH_ + TIDAT + GAMA + IEND, 'gAMA after IDAT')
p('order-srgb-after-idat', TH_ + TIDAT + chunk(b'sRGB', b'\0') + IEND, 'sRGB after IDAT')
p('order-iccp-after-idat', TH_ + TIDAT + chunk(b'iCCP', b'n\0\0' + z(G.ICC)) + IEND, 'iCCP after IDAT')
p('order-trns-before-plte', PH + chunk(b'tRNS', b'\0\x80') + chunk(b'PLTE', PAL) + PIDAT + IEND, 'tRNS before PLTE (palette image)')
p('order-trns-after-idat-palette', PH + chunk(b'PLTE', PAL) + PIDAT + chunk(b'tRNS', b'\0\x80') + IEND, 'tRNS after IDAT (palette image)')
p('order-trns-after-idat-rgb', TH_ + TIDAT + chunk(b'tRNS', struct.pack('>HHH', *TPX[0])) + IEND, 'tRNS after IDAT (RGB)')
p('order-bkgd-before-plte', PH + chunk(b'bKGD', b'\1') + chunk(b'PLTE', PAL) + PIDAT + IEND, 'bKGD before PLTE (palette)')
p('order-plte-after-idat-palette', PH + PIDAT + chunk(b'PLTE', PAL) + IEND, 'PLTE only after IDAT (palette)')
p('order-plte-after-idat-rgb', TH_ + TIDAT + chunk(b'PLTE', PAL) + IEND, 'PLTE after IDAT (RGB)')
p('order-phys-after-idat', TH_ + TIDAT + chunk(b'pHYs', struct.pack('>IIB', 2835, 2835, 1)) + IEND, 'pHYs after IDAT')
p('order-chunk-before-ihdr', SIG + GAMA + ihdr(TW, TH, 8, 2) + TIDAT + IEND, 'gAMA before IHDR')
p('dup-ihdr', TH_ + ihdr(TW, TH, 8, 2) + TIDAT + IEND, 'second IHDR (identical)')
PAL2 = bytes([0, 0, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90])
p('dup-plte', PH + chunk(b'PLTE', PAL) + chunk(b'PLTE', PAL2) + PIDAT + IEND, 'two PLTE: which wins?')
p('dup-gama', t(GAMA, chunk(b'gAMA', struct.pack('>I', 100000))), 'two gAMA')
p('dup-trns-palette', PH + chunk(b'PLTE', PAL) + chunk(b'tRNS', b'\0') + chunk(b'tRNS', b'\xff\0') + PIDAT + IEND, 'two tRNS: which wins?')
p('dup-srgb', t(chunk(b'sRGB', b'\0'), chunk(b'sRGB', b'\1')), 'two sRGB')
p('dup-time', TH_ + TIDAT + chunk(b'tIME', struct.pack('>HBBBBB', 2000, 1, 1, 0, 0, 0)) * 2 + IEND, 'two tIME')
p('dup-phys', t(chunk(b'pHYs', struct.pack('>IIB', 1, 1, 0)) * 2), 'two pHYs')
# --- chunk-level
p('unknown-critical', t(chunk(b'ABCD', b'')), 'unknown critical chunk')
p('reserved-bit-ancillary', t(chunk(b'abcd', b'x')), 'reserved bit set, ancillary')
p('reserved-bit-critical-idat-like', t(chunk(b'IDaT', b'')), 'IDaT (reserved bit, critical)')
p('type-nonletter', t(chunk(b'te1t', b'')), 'chunk type with a digit')
p('length-2^31', TH_ + struct.pack('>I', 1 << 31) + b'teXt' + bytes(16) + TIDAT + IEND, 'chunk length 2^31')
p('ihdr-length-14', SIG + chunk(b'IHDR', struct.pack('>IIBBBBBB', TW, TH, 8, 2, 0, 0, 0, 0)) + TIDAT + IEND, 'IHDR of 14 bytes')
p('ihdr-compression-1', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', TW, TH, 8, 2, 1, 0, 0)) + TIDAT + IEND, 'compression method 1')
p('ihdr-filter-1', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', TW, TH, 8, 2, 0, 1, 0)) + TIDAT + IEND, 'filter method 1')
p('ihdr-interlace-2', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', TW, TH, 8, 2, 0, 0, 2)) + TIDAT + IEND, 'interlace method 2')
# --- PLTE and palette indices
p('plte-len-13', PH + chunk(b'PLTE', PAL + b'\x7f') + PIDAT + IEND, 'PLTE of 13 bytes (not a multiple of 3)')
p('plte-len-14', PH + chunk(b'PLTE', PAL + b'\x7f\x7f') + PIDAT + IEND, 'PLTE of 14 bytes')
p('plte-len-0', PH + chunk(b'PLTE', b'') + PIDAT + IEND, 'empty PLTE')
p('plte-in-grey', SIG + ihdr(2, 1, 8, 0) + chunk(b'PLTE', PAL) + chunk(b'IDAT', z(b'\0\x10\x20')) + IEND, 'PLTE in greyscale')
p('plte-more-than-depth', SIG + ihdr(2, 1, 1, 3) + chunk(b'PLTE', PAL) + chunk(b'IDAT', z(b'\0\x40')) + IEND, '4 entries at depth 1')
p('plte-257', SIG + ihdr(2, 1, 8, 3) + chunk(b'PLTE', bytes(771)) + chunk(b'IDAT', z(b'\0\0\1')) + IEND, '257 entries')
p('index-oob-8bit', PH + chunk(b'PLTE', PAL) + chunk(b'IDAT', z(scan([(i % 6,) for i in range(12)], TW, TH, 3, 8, 0))) + IEND,
  'indices 4 and 5 with 4 entries')
p('index-oob-1bit', SIG + ihdr(4, 1, 1, 3) + chunk(b'PLTE', PAL[:3]) + chunk(b'IDAT', z(b'\0\x50')) + IEND, 'index 1 with 1 entry')
p('index-oob-with-trns', PH + chunk(b'PLTE', PAL) + chunk(b'tRNS', b'\x10\x20\x30\x40') + chunk(b'IDAT', z(scan([(i % 6,) for i in range(12)], TW, TH, 3, 8, 0))) + IEND,
  'out-of-range indices with a full tRNS')
p('missing-plte', PH + PIDAT + IEND, 'palette image without PLTE')
p('suggested-plte-rgb', t(chunk(b'PLTE', PAL)), 'PLTE in an RGB image')
# --- tRNS
p('trns-palette-long', PH + chunk(b'PLTE', PAL) + chunk(b'tRNS', b'\x10\x20\x30\x40\x50') + PIDAT + IEND, 'tRNS longer than PLTE')
p('trns-palette-short', PH + chunk(b'PLTE', PAL) + chunk(b'tRNS', b'\x10') + PIDAT + IEND, 'tRNS for entry 0 only')
p('trns-palette-empty', PH + chunk(b'PLTE', PAL) + chunk(b'tRNS', b'') + PIDAT + IEND, 'zero-length tRNS')
p('trns-rgb-len-4', t(chunk(b'tRNS', bytes(4))), 'RGB tRNS of 4 bytes')
p('trns-rgb-len-8', t(chunk(b'tRNS', struct.pack('>HHHH', *TPX[0], 0))), 'RGB tRNS of 8 bytes, first 6 match pixel 0')
p('trns-rgb-match', t(chunk(b'tRNS', struct.pack('>HHH', *TPX[0]))), 'RGB key matching pixel 0')
p('trns-grey-len-1', SIG + ihdr(2, 1, 8, 0) + chunk(b'tRNS', b'\x10') + chunk(b'IDAT', z(b'\0\x10\x20')) + IEND, 'grey tRNS of 1 byte')
p('trns-grey-high-bits', SIG + ihdr(4, 1, 2, 0) + chunk(b'tRNS', b'\xff\x01') + chunk(b'IDAT', z(b'\0\x1b')) + IEND,
  '2-bit grey key 0xff01: masked to 1?')
p('trns-in-rgba', SIG + ihdr(1, 1, 8, 6) + chunk(b'tRNS', bytes(6)) + chunk(b'IDAT', z(b'\0\1\2\3\4')) + IEND, 'tRNS in RGBA')
p('trns-in-grey-alpha', SIG + ihdr(1, 1, 8, 4) + chunk(b'tRNS', bytes(2)) + chunk(b'IDAT', z(b'\0\1\2')) + IEND, 'tRNS in grey+alpha')
G16 = [(v,) for v in [0, 127, 128, 383, 384, 32767, 32768, 32895, 32896, 65407, 65408, 65535]]
p('trns-grey16-key', SIG + ihdr(12, 1, 16, 0) + chunk(b'tRNS', struct.pack('>H', 32896)) + chunk(b'IDAT', z(scan(G16, 12, 1, 0, 16, 0))) + IEND,
  '16-bit grey key 32896 (high byte 128 also in 32768, 32895)')
p('trns-grey4-key', SIG + ihdr(8, 1, 4, 0) + chunk(b'tRNS', b'\0\5') + chunk(b'IDAT', z(scan([(i,) for i in range(8)], 8, 1, 0, 4, 0))) + IEND,
  '4-bit grey key 5')
# --- 16-bit and depth scaling
p('depth16-grey-rounding', SIG + ihdr(12, 1, 16, 0) + chunk(b'IDAT', z(scan(G16, 12, 1, 0, 16, 0))) + IEND, '16-bit to 8-bit: round or strip')
R16 = [(v, 65535 - v, v ^ 0x5555, (v * 7) & 0xFFFF) for v in [0, 128, 383, 32895, 32896, 65408]]
p('depth16-rgba', SIG + ihdr(6, 1, 16, 6) + chunk(b'IDAT', z(scan(R16, 6, 1, 6, 16, 0))) + IEND, 'RGBA16 reduction')
p('depth-grey-1-2-4', SIG + ihdr(4, 1, 2, 0) + chunk(b'IDAT', z(b'\0\x1b')) + IEND, '2-bit grey 0..3 scaling')
SB = chunk(b'sBIT', b'\x05')
p('sbit-grey8', SIG + ihdr(2, 1, 8, 0) + SB + chunk(b'IDAT', z(b'\0\x80\xf8')) + IEND, 'sBIT 5 on grey 8: applied?')
# --- gamma and colour chunks (does anyone transform pixels?)
RAMP = [(i * 17,) * 3 for i in range(16)]
RAMPB = SIG + ihdr(16, 1, 8, 2)
RAMPD = chunk(b'IDAT', z(scan(RAMP, 16, 1, 2, 8, 0)))
p('gamma-none', RAMPB + RAMPD + IEND, 'ramp without colour chunks')
p('gamma-045455', RAMPB + GAMA + RAMPD + IEND, 'ramp with gAMA 0.45455')
p('gamma-1', RAMPB + chunk(b'gAMA', struct.pack('>I', 100000)) + RAMPD + IEND, 'ramp with gAMA 1.0')
p('gamma-0', RAMPB + chunk(b'gAMA', bytes(4)) + RAMPD + IEND, 'gAMA 0')
p('gamma-srgb', RAMPB + chunk(b'sRGB', b'\0') + GAMA + RAMPD + IEND, 'sRGB + gAMA')
p('gamma-chrm', RAMPB + GAMA + chunk(b'cHRM', struct.pack('>8I', 31270, 32900, 64000, 33000, 30000, 60000, 15000, 6000)) + RAMPD + IEND,
  'gAMA + cHRM')
p('gamma-cicp-pq', RAMPB + chunk(b'cICP', b'\x09\x10\0\1') + RAMPD + IEND, 'cICP BT.2100 PQ')
p('iccp-valid', RAMPB + chunk(b'iCCP', b'n\0\0' + z(G.ICC)) + RAMPD + IEND, 'iCCP with a junk profile (not a real ICC)')
p('iccp-bad-zlib', RAMPB + chunk(b'iCCP', b'n\0\0\x78\x9c\xff\xff') + RAMPD + IEND, 'iCCP whose zlib stream is corrupt')
p('iccp-method-1', RAMPB + chunk(b'iCCP', b'n\0\1' + z(G.ICC)) + RAMPD + IEND, 'iCCP compression method 1')
p('iccp-bomb-100m', RAMPB + chunk(b'iCCP', b'n\0\0' + z(bytes(100 << 20), level=9)) + RAMPD + IEND, 'iCCP inflating to 100 MiB')
p('cicp-matrix-1', RAMPB + chunk(b'cICP', b'\1\15\1\1') + RAMPD + IEND, 'cICP with matrix coefficients 1')
p('cicp-len-5', RAMPB + chunk(b'cICP', b'\1\15\0\1\0') + RAMPD + IEND, 'cICP of 5 bytes')
p('bkgd-rgb', RAMPB + chunk(b'bKGD', b'\0\xff\0\0\0\0') + RAMPD + IEND, 'bKGD red: composited?')
p('bkgd-index-oob', PH + chunk(b'PLTE', PAL) + chunk(b'bKGD', b'\x09') + PIDAT + IEND, 'bKGD index 9 with 4 entries')
p('srgb-intent-4', RAMPB + chunk(b'sRGB', b'\4') + RAMPD + IEND, 'sRGB rendering intent 4')
p('phys-unit-2', t(chunk(b'pHYs', struct.pack('>IIB', 1, 1, 2))), 'pHYs unit 2')
p('exif-bad-magic', t(chunk(b'eXIf', b'XX\0\0')), 'eXIf without II/MM')
p('time-invalid', TH_ + TIDAT + chunk(b'tIME', struct.pack('>HBBBBB', 2026, 13, 40, 25, 61, 61)) + IEND, 'tIME out of range')
p('sbit-zero', t(chunk(b'sBIT', b'\0\0\0')), 'sBIT zero')
p('hist-wrong-length', PH + chunk(b'PLTE', PAL) + chunk(b'hIST', bytes(2)) + PIDAT + IEND, 'hIST of 1 entry for 4')
# --- text
p('text-keyword-empty', t(chunk(b'tEXt', b'\0value')), 'tEXt with empty keyword')
p('text-keyword-80', t(chunk(b'tEXt', b'k' * 80 + b'\0v')), 'tEXt keyword of 80 bytes')
p('text-no-separator', t(chunk(b'tEXt', b'Title')), 'tEXt without null separator')
p('ztxt-bad-zlib', t(chunk(b'zTXt', b'k\0\0garbage')), 'zTXt with corrupt zlib')
p('ztxt-method-1', t(chunk(b'zTXt', b'k\0\1' + z(b'x'))), 'zTXt compression method 1')
p('itxt-flag-2', t(chunk(b'iTXt', b'k\0\2\0\0\0x')), 'iTXt compression flag 2')
p('ztxt-bomb-64m', t(chunk(b'zTXt', b'Comment\0\0' + z(bytes(64 << 20), level=9))), 'zTXt inflating to 64 MiB')
p('itxt-bomb-64m', t(chunk(b'iTXt', b'XML:com.adobe.xmp\0\1\0\0\0' + z(bytes(64 << 20), level=9))), 'compressed XMP iTXt inflating to 64 MiB')
p('text-many-20000', t(*[chunk(b'tEXt', b'k\0v')] * 20000), '20,000 tEXt chunks')
p('text-large-8m', t(chunk(b'tEXt', b'k\0' + b'v' * (8 << 20))), 'one 8 MiB tEXt')
# --- dimensions
p('dims-65535-square-tiny-data', SIG + ihdr(65535, 65535, 8, 6) + chunk(b'IDAT', z(bytes(100))) + IEND, '65535x65535 RGBA, 100 data bytes')
p('dims-100000x1', SIG + ihdr(100000, 1, 8, 0) + chunk(b'IDAT', z(bytes(100001))) + IEND, '100000x1 grey, complete data')
p('dims-16385x16', SIG + ihdr(16385, 16, 8, 0) + chunk(b'IDAT', z(bytes(16386 * 16))) + IEND, 'one pixel above the proposed side cap')
p('dims-2^31', SIG + ihdr(1 << 31, 1, 8, 0) + chunk(b'IDAT', z(bytes(10))) + IEND, 'width 2^31')
p('dims-zero', SIG + ihdr(0, 1, 8, 0) + chunk(b'IDAT', z(bytes(10))) + IEND, 'width 0')
# --- filters and interlace
rr = bytearray(TRAW)
rr[13] = 5
p('filter-type-5', TH_ + chunk(b'IDAT', z(bytes(rr))) + IEND, 'filter type 5 on row 1')
rr[13] = 0
rr[26] = 255
p('filter-type-255-last-row', TH_ + chunk(b'IDAT', z(bytes(rr))) + IEND, 'filter type 255 on the last row')
for w_, h_ in [(1, 1), (2, 1), (1, 2), (3, 3), (5, 5)]:
    px = [((x * 50) % 256, (y * 70) % 256, 9, 200) for y in range(h_) for x in range(w_)]
    p(f'interlace-{w_}x{h_}', SIG + ihdr(w_, h_, 8, 6, il=1) + chunk(b'IDAT', z(scan(px, w_, h_, 6, 8, 1))) + IEND, f'interlaced {w_}x{h_}')
p('interlace-1x1-empty-pass-bytes', SIG + ihdr(1, 1, 8, 0, il=1) + chunk(b'IDAT', z(bytes([0, 5]) + bytes(6))) + IEND,
  'interlaced 1x1 with filter bytes for the six empty passes')
p('interlace-1x1-grey1', SIG + ihdr(1, 1, 1, 0, il=1) + chunk(b'IDAT', z(b'\0\x80')) + IEND, 'interlaced 1x1 1-bit')
# --- APNG
AW, AH = G.AW, G.AH
AH_ = SIG + ihdr(AW, AH, 8, 6)
Z0, Z1, Z2 = G.Z0, G.Z1, G.Z2
OK3 = AH_ + actl(3, 2) + fctl(0, AW, AH, 0, 0, 5, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 1, 1, 1, 10, 1, 0) + fdat(2, Z1) \
    + fctl(3, 2, 1, 0, 0, 3, 100, 0, 1) + fdat(4, Z2) + IEND
p('apng-ok-3-frames', OK3, 'default image is frame 0; dispose background; blend over')
p('apng-default-not-frame', AH_ + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, 1, 1) + fdat(1, Z1) + fctl(2, 2, 1, 2, 2, 1, 1)
  + fdat(3, Z2) + IEND, 'default image outside the animation')
p('apng-dispose-previous-first', AH_ + actl(2) + fctl(0, AW, AH, 0, 0, 1, 1, 2, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 1, 1, 0, 1)
  + fdat(2, Z1) + IEND, 'first frame dispose PREVIOUS')
p('apng-dispose-previous-after-bg', AH_ + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 1, 1, 1, 0) + fdat(2, Z1)
  + fctl(3, 2, 2, 1, 0, 1, 1, 2, 0) + fdat(4, Z1) + IEND, 'BACKGROUND then overlapping PREVIOUS (image-rs order of save and clear)')
p('apng-blend-over', AH_ + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 3, 2, 0, 0) + fdat(1, G.ZB_) + fctl(2, 3, 2, 1, 1, bop=1) + fdat(3, G.ZA)
  + IEND, 'OVER arithmetic on semi-transparent pixels')
p('apng-actl-no-fctl', AH_ + actl(1) + chunk(b'IDAT', Z0) + IEND, 'acTL without any fcTL')
p('apng-actl-zero', AH_ + actl(0) + chunk(b'IDAT', Z0) + IEND, 'acTL num_frames 0')
p('apng-actl-after-idat', AH_ + chunk(b'IDAT', Z0) + actl(1) + fctl(0, 2, 2, 0, 0) + fdat(1, Z1) + IEND, 'acTL after IDAT')
p('apng-fewer-frames', AH_ + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1) + IEND,
  'num_frames 3, two frames')
p('apng-more-frames', AH_ + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1) + IEND,
  'num_frames 1, two frames')
p('apng-seq-gap', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'sequence gap')
p('apng-seq-fdat-dup', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:4]) + fdat(2, Z1[4:])
  + IEND, 'repeated fdAT sequence number')
p('apng-fdat-before-fctl', AH_ + actl(1) + chunk(b'IDAT', Z0) + fdat(0, Z1) + fctl(1, 2, 2, 0, 0) + IEND, 'fdAT before its fcTL')
p('apng-fctl-no-fdat', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + IEND, 'last fcTL without fdAT')
p('apng-frame-outside', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 3, 0) + fdat(2, Z1) + IEND,
  'frame crosses the right edge')
p('apng-default-fctl-mismatch', AH_ + actl(1) + fctl(0, 2, 2, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'fcTL before IDAT smaller than IHDR')
p('apng-dispose-3', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, dop=3) + fdat(2, Z1) + IEND, 'dispose_op 3')
p('apng-blend-2', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, bop=2) + fdat(2, Z1) + IEND, 'blend_op 2')
p('apng-frame2-bad-zlib', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:-1] + b'\0') + IEND,
  'frame 1 with a wrong Adler-32')
p('apng-frame2-truncated-zlib', AH_ + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:6]) + IEND,
  'frame 1 zlib stream cut short')
p('apng-delay-den-0', AH_ + actl(2) + fctl(0, AW, AH, 0, 0, 7, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 0, 0) + fdat(2, Z1) + IEND,
  'delay 7/0 and 0/0')
p('apng-16bit', SIG + ihdr(2, 1, 16, 6) + actl(1) + fctl(0, 2, 1, 0, 0) + chunk(b'IDAT', z(scan([(1, 2, 3, 65535), (65535, 0, 0, 32768)], 2, 1, 6, 16, 0)))
  + IEND, '16-bit APNG')
p('apng-fctl-without-actl', AH_ + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'fcTL without acTL')
p('apng-interlaced', SIG + ihdr(AW, AH, 8, 6, il=1) + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', G.zframe(G.F0, AW, AH, 1))
  + fctl(1, 3, 3, 1, 0) + fdat(2, G.zframe(G.FI, 3, 3, 1)) + IEND, 'interlaced APNG with a narrower subframe')
p('apng-num-plays-3', AH_ + actl(1, 3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'single-frame APNG, num_plays 3')
# --- second round (added after the first run; see OPTIONS.md hold-out note)
p('apng-bg-then-previous-4', AH_ + actl(4) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 1, 1, 1, 0) + fdat(2, Z1)
  + fctl(3, 2, 2, 1, 0, 1, 1, 2, 0) + fdat(4, G.zframe(G.rgba_frame(2, 2, [G.BLU]), 2, 2)) + fctl(5, 1, 1, 3, 2) + fdat(6, G.zframe([G.RED], 1, 1))
  + IEND, 'BACKGROUND, then an overlapping PREVIOUS, then a fourth frame (what PREVIOUS restores)')
p('apng-previous-chain', AH_ + actl(4) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0, 1, 1, 2, 0) + fdat(2, Z1)
  + fctl(3, 2, 2, 1, 1, 1, 1, 2, 0) + fdat(4, G.zframe(G.rgba_frame(2, 2, [G.BLU]), 2, 2)) + fctl(5, 1, 1, 0, 0) + fdat(6, G.zframe([G.HALF], 1, 1))
  + IEND, 'two consecutive PREVIOUS frames')
p('idat-then-4-bytes', TH_ + TIDAT + b'\0\0\0\0', 'four bytes after the IDAT run, then EOF')
p('idat-then-iend-no-crc', TH_ + TIDAT + b'\0\0\0\0IEND', 'IEND header without its CRC')
p('crc-extra-idat-after-stream', TH_ + TIDAT + bad_crc(chunk(b'IDAT', b'\0')) + IEND, 'bad CRC on an IDAT after the zlib stream ended')
p('unknown-critical-after-idat', TH_ + TIDAT + chunk(b'ABCD', b'') + IEND, 'unknown critical chunk after IDAT')
p('crc-text-after-idat-image', TH_ + TIDAT + bad_crc(chunk(b'zzzz', b'abc')) + IEND, 'bad CRC on unknown ancillary after IDAT')
p('trns-rgb16-lowbyte', SIG + ihdr(2, 1, 16, 2) + chunk(b'tRNS', struct.pack('>HHH', 0x1234, 0x5678, 0x9ABD))
  + chunk(b'IDAT', z(scan([(0x1234, 0x5678, 0x9ABC), (0x1234, 0x5678, 0x9ABD)], 2, 1, 2, 16, 0))) + IEND, '16-bit RGB key: exact compare')
p('index-oob-2bit-trns', SIG + ihdr(4, 1, 2, 3) + chunk(b'PLTE', PAL[:6]) + chunk(b'tRNS', b'\x10\x20') + chunk(b'IDAT', z(b'\0\x1b')) + IEND,
  '2-bit indices 2 and 3 beyond a 2-entry palette with tRNS')
