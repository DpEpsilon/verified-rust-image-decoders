#!/usr/bin/env python3
"""Generate VECTORS-POLICY.json: the profile records (SPEC P19 `profile` field) of the PNG policy draft.

Encoder helpers and the Strict fixtures come from ../spec-draft/gen_vectors.py (imported; its records are not written
here). The records continue that module's random stream, so they are byte-identical to the profile records of the
pre-split VECTORS.json. Six records are also cap-dependent (`policy/apng-static/{actl-too-many-frames,pixel-budget,
output-too-large}/*`). Expectations are written from intent, never computed by the oracle (P19).
Usage: python3 -I gen_vectors_policy.py   (writes VECTORS-POLICY.json next to this file; deterministic)
"""
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
import gen_vectors as _GEN  # noqa: E402
from gen_vectors import *  # noqa: E402,F401,F403

START = len(_GEN.V)

# ---------------------------------------------------------------- policy profiles: RecommendedBeta, ImageCrateCompat
# A record with `profile` (a name from PROFILES in ../policy-draft/policy_oracle.py) is decoded by that profile's entry
# point instead of the Strict one; `profile: "Strict"` and no `profile` mean the same. For every other profile an accepted
# result carries `recovered` (still) or, per frame, `recovered`, and `events`: the sorted list from the closed R1-R5 of
# ../policy-draft/OPTIONS.md that explains it. Errors keep the `{error}` form. Each expectation below is written from the
# construction (which chunk is damaged and what the profile's field value says happens to it), not taken from the oracle.
RB, IMG = 'RecommendedBeta', 'ImageCrateCompat-0.25.10'


def pol(exp, ev=()):
    """Add `recovered` and `events` to a still expectation, or to every frame of a frames expectation."""
    exp = json.loads(json.dumps(exp))
    if 'frames' in exp:
        for f in exp['frames']:
            f['recovered'], f['events'] = bool(ev), list(ev)
    else:
        exp['recovered'], exp['events'] = bool(ev), list(ev)
    return exp


def sexp(w_, h_, pix):
    return {'width': w_, 'height': h_, 'rgba_runs': runs(pix)}


def pair(name, entry, b, strict, rb, ev=()):
    """strict, rb: an error name or an expectation (rb is given without recovered/events; ev lists the events)."""
    add(f'policy/{name}/strict', entry, b, err(strict) if isinstance(strict, str) else strict, profile='Strict')
    add(f'policy/{name}/recommended-beta', entry, b, err(rb) if isinstance(rb, str) else pol(rb, ev), profile=RB)


def T22():
    return sexp(2, 2, TEXP)


# --- crc_ancillary=SkipExceptFdat (PNG-Q2): a bad CRC drops an ancillary chunk, but not fdAT or a critical chunk
pair('crc-ancillary/text', 'still', doc(chunk(b'tEXt', b'Title\0x', crc=1), IDAT_OK, IEND), 'CrcMismatch', T22())
pair('crc-ancillary/text-after-idat', 'still', doc(IDAT_OK, chunk(b'tEXt', b'Title\0x', crc=1), IEND), 'CrcMismatch', T22())
pair('crc-ancillary/gama', 'still', doc(chunk(b'gAMA', struct.pack('>I', 45455), crc=1), IDAT_OK, IEND), 'CrcMismatch', T22())
pair('crc-ancillary/unknown-ancillary', 'still', doc(chunk(b'prVt', b'abc', crc=1), IDAT_OK, IEND), 'CrcMismatch', T22())
# tRNS: the key colour is pixel 0, so a valid chunk makes pixel 0 transparent; the dropped chunk leaves every pixel opaque (R1)
KEY = struct.pack('>HHH', *TPX[0])
TRNS_OK_EXP = sexp(2, 2, [rgba_of(s, 2, 8, trns=tuple(TPX[0])) for s in TPX])
assert TRNS_OK_EXP['rgba_runs'] != runs(TEXP)
add('policy/crc-ancillary/trns-valid-control/recommended-beta', 'still', doc(chunk(b'tRNS', KEY), IDAT_OK, IEND), pol(TRNS_OK_EXP), profile=RB)
pair('crc-ancillary/trns-rgb', 'still', doc(chunk(b'tRNS', KEY, crc=1), IDAT_OK, IEND), 'CrcMismatch', T22(), ev=('R1',))
PTRNS = bytes([7, 200])
add('policy/crc-ancillary/trns-palette-valid-control/recommended-beta', 'still', P3 + chunk(b'PLTE', PL3) + chunk(b'tRNS', PTRNS) + P3IDAT + IEND,
    pol(sexp(2, 2, [rgba_of(s, 3, 8, PL3, PTRNS) for s in P3PX])), profile=RB)
pair('crc-ancillary/trns-palette', 'still', P3 + chunk(b'PLTE', PL3) + chunk(b'tRNS', PTRNS, crc=1) + P3IDAT + IEND, 'CrcMismatch',
     sexp(2, 2, P3EXP), ev=('R1',))
# fdAT: a bad CRC is an error in RecommendedBeta, in frames and in still, not a dropped chunk
FD_OK = AHDR + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0)
FD_BAD = FD_OK + chunk(b'fdAT', struct.pack('>I', 2) + Z1, crc=1) + IEND
pair('crc-ancillary/fdat-frames', 'frames', FD_BAD, 'CrcMismatch', 'CrcMismatch')
pair('crc-ancillary/fdat-still', 'still', FD_BAD, 'CrcMismatch', 'CrcMismatch')
pair('crc-ancillary/fdat-second-chunk-of-frame', 'frames',
     FD_OK + fdat(2, Z1[:5]) + chunk(b'fdAT', struct.pack('>I', 3) + Z1[5:], crc=1) + IEND, 'CrcMismatch', 'CrcMismatch')
# critical chunks: an error in both profiles
pair('crc-ancillary/idat', 'still', doc(chunk(b'IDAT', z(scan(TPX, 2, 2, 2, 8, 0)), crc=1), IEND), 'CrcMismatch', 'CrcMismatch')
pair('crc-ancillary/plte', 'still', P3 + chunk(b'PLTE', PL3, crc=1) + P3IDAT + IEND, 'CrcMismatch', 'CrcMismatch')
pair('crc-ancillary/ihdr', 'still', SIG + chunk(b'IHDR', struct.pack('>IIBBBBB', 2, 2, 8, 2, 0, 0, 0), crc=0) + IDAT_OK + IEND,
     'CrcMismatch', 'CrcMismatch')
# ImageCrateCompat keeps crc_ancillary=Skip, so a bad-CRC fdAT is dropped as png does. Frame 1's only fdAT is gone, the fcTL at
# IEND has no data, and image (apng_errors=Prefix) returns the complete frame 0 and ends the sequence with that error.
add('policy/crc-ancillary/fdat-frames/image-crate-compat', 'frames', FD_BAD,
    pol(frames_exp([((0, 0, AW, AH), F0, 0, 0)], [((1, 10), 0, 0)], end={'error': 'MissingFrameData'})), profile=IMG)
# the still path of image never reads past the IDAT run (after_idat=ImageRs), so the bad fdAT is not seen
add('policy/crc-ancillary/fdat-still/image-crate-compat', 'still', FD_BAD, pol(sexp(AW, AH, F0)), profile=IMG)

# --- apng_errors=Static (PNG-Q11): R4, the default image replaces an animation whose structure is invalid
R4 = {'width': AW, 'height': AH, 'num_plays': None, 'end': 'iend',
      'frames': [{'rect': [0, 0, AW, AH], 'delay': None, 'dispose': 0, 'blend': 0, 'rgba_runs': runs(F0)}]}
FCTL0_BADCRC = chunk(b'fcTL', struct.pack('>IIIIIHHBB', 0, AW, AH, 0, 0, 1, 10, 0, 0), crc=1)
FCTL1_BADCRC = chunk(b'fcTL', struct.pack('>IIIIIHHBB', 1, 2, 2, 0, 0, 1, 10, 0, 0), crc=1)
STRUCT = [
    ('seq-gap', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'SequenceError'),
    ('seq-starts-at-1', G + actl(2) + fctl(1, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'SequenceError'),
    ('seq-fdat-repeats', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:4]) + fdat(2, Z1[4:]) + IEND,
     'SequenceError'),
    ('fctl-outside-x', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 3, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('fctl-outside-y', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 2) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('fctl-offset-overflow-u32', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0xFFFFFFFF, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('fctl-width-0', G + actl(2) + chunk(b'IDAT', Z0) + fctl(0, 0, 2, 0, 0) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('fctl-dispose-3', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, dop=3) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('fctl-blend-2', G + actl(1) + chunk(b'IDAT', Z0) + fctl(0, 2, 2, 0, 0, bop=2) + fdat(1, Z1) + IEND, 'InvalidFrameControl'),
    ('default-fctl-smaller', G + actl(1) + fctl(0, 2, 2, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'InvalidFrameControl'),
    ('fewer-frames-than-declared', G + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail, 'FrameCountMismatch'),
    ('more-frames-than-declared', G + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail, 'FrameCountMismatch'),
    ('no-fctl-at-all', G + actl(1) + chunk(b'IDAT', Z0) + IEND, 'FrameCountMismatch'),
    ('fctl-no-fdat-then-iend', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + IEND, 'MissingFrameData'),
    ('fctl-no-fdat-then-fctl', G + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fctl(2, 2, 2, 0, 0)
     + fdat(3, Z1) + IEND, 'MissingFrameData'),
    ('fdat-before-fctl', G + actl(1) + chunk(b'IDAT', Z0) + fdat(0, Z1) + fctl(1, 2, 2, 0, 0) + IEND, 'UnexpectedFrameData'),
    ('fdat-for-idat-frame', G + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fdat(1, Z1) + IEND, 'UnexpectedFrameData'),
    ('fctl-without-actl', G + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + IEND, 'MissingAnimationControl'),
    ('fdat-without-actl', G + chunk(b'IDAT', Z0) + fdat(0, Z1) + IEND, 'MissingAnimationControl'),
    ('actl-zero-frames', G + actl(0) + chunk(b'IDAT', Z0) + IEND, 'InvalidAnimationControl'),
    # a bad-CRC acTL is dropped first (crc_ancillary), which leaves fcTL and fdAT chunks without an acTL
    ('actl-bad-crc', G + chunk(b'acTL', struct.pack('>II', 2, 0), crc=1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail, 'CrcMismatch'),
    # the dropped fcTL(0) leaves fcTL(1) first: wrong sequence number, and one frame against acTL 2
    ('fctl0-bad-crc', G + actl(2) + FCTL0_BADCRC + chunk(b'IDAT', Z0) + ok_tail, 'CrcMismatch'),
    # the dropped fcTL(1) leaves its fdAT without a current frame
    ('fctl1-bad-crc', G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + FCTL1_BADCRC + fdat(2, Z1) + IEND, 'CrcMismatch'),
]
for nm, bb, e in STRUCT:
    pair(f'apng-static/{nm}', 'frames', bb, e, R4, ev=('R4',))
    pair(f'apng-static/{nm}-still', 'still', bb, e, sexp(AW, AH, F0))
# duplicate acTL: ancillary_duplicate=FirstWins drops the second silently, no R4, and the animation (first acTL's num_plays) goes on
DUP = G + actl(2, 3) + actl(1, 7) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + ok_tail
pair('apng-static/actl-duplicate-first-wins', 'frames', DUP, 'DuplicateChunk',
     frames_exp([((0, 0, AW, AH), F0, 0, 0), ((0, 0, 2, 2), F1, 0, 0)], [((1, 10), 0, 0), ((1, 10), 0, 0)], plays=3))
# not caught by Static: resource caps, a bad fdAT CRC (above), inflate errors, an undecodable default image
pair('apng-static/actl-too-many-frames', 'frames', G + actl(65537) + chunk(b'IDAT', Z0) + IEND, 'TooManyFrames', 'TooManyFrames')
BW_, BH_ = 8192, 2048                  # 65 frames of 2^24 pixels exceed MAX_TOTAL_PIXELS (cap-dependent pair)
bb = SIG + ihdr(BW_, BH_, 8, 6) + actl(65) + chunk(b'IDAT', b'\x78\x01')
seq = 0
for k in range(65):
    bb += fctl(seq, BW_, BH_, 0, 0) + fdat(seq + 1, b'\x78\x01')
    seq += 2
pair('apng-static/pixel-budget', 'frames', bb + IEND, 'PixelBudgetExceeded', 'PixelBudgetExceeded')
pair('apng-static/output-too-large', 'frames', SIG + ihdr(8192, 8193, 8, 2) + actl(1) + chunk(b'IDAT', Z0) + IEND, 'OutputTooLarge', 'OutputTooLarge')
# a later frame's data is corrupt: the error comes from the iterator at that frame, after the frames before it
B1 = AHDR + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1[:-1] + b'\0') + IEND
e1 = frames_exp([((0, 0, AW, AH), F0, 0, 0)], [((1, 10), 0, 0)], end={'error': 'AdlerMismatch'})
pair('apng-static/frame1-adler-error', 'frames', B1, e1, e1)
B2 = AHDR + actl(3) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', Z0) + fctl(1, 2, 2, 0, 0) + fdat(2, Z1) + fctl(3, 2, 1, 0, 0) \
    + fdat(4, z(scan(F2, 2, 1, 6, 8, 0)[:-1])) + IEND
e2 = frames_exp([((0, 0, AW, AH), F0, 0, 0), ((0, 0, 2, 2), F1, 0, 0)], [((1, 10), 0, 0)] * 2, end={'error': 'TruncatedImageData'})
pair('apng-static/frame2-truncated-data', 'frames', B2, e2, e2)
# the default image does not decode: an error, no fallback, whether or not the structure is also invalid
BADZ = Z0[:-1] + b'\0'
pair('apng-static/default-image-adler-error', 'frames', G + actl(1) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', BADZ) + IEND,
     'AdlerMismatch', 'AdlerMismatch')
pair('apng-static/structural-error-and-default-image-adler-error', 'frames',
     G + actl(2) + fctl(0, AW, AH, 0, 0) + chunk(b'IDAT', BADZ) + fctl(2, 2, 2, 0, 0) + fdat(3, Z1) + IEND, 'SequenceError', 'AdlerMismatch')

# --- the other RecommendedBeta fields (a Strict error and an accepting RecommendedBeta result each)
# iend=Lenient: IEND's CRC and length are ignored
pair('iend/bad-crc', 'still', doc(IDAT_OK, chunk(b'IEND', b'', crc=0)), 'CrcMismatch', T22())
pair('iend/length-1', 'still', doc(IDAT_OK, chunk(b'IEND', b'\0')), 'InvalidChunkLength', T22())
# missing_iend=AtEof: EOF at a chunk boundary after the first IDAT acts as IEND
pair('missing-iend/after-idat', 'still', doc(IDAT_OK), 'MissingIend', T22())
pair('missing-iend/after-text', 'still', doc(IDAT_OK, chunk(b'tEXt', b'a\0b')), 'MissingIend', T22())
# trailing=Ignore
pair('trailing/byte', 'still', TOK + b'\0', 'TrailingBytes', T22())
pair('trailing/second-png', 'still', TOK + TOK, 'TrailingBytes', T22())
# ancillary_order=Drop (R1 only for tRNS)
pair('ancillary-order/gama-after-idat', 'still', doc(IDAT_OK, chunk(b'gAMA', struct.pack('>I', 45455)), IEND), 'ChunkOutOfOrder', T22())
pair('ancillary-order/gama-after-plte', 'still', doc(chunk(b'PLTE', PL3), chunk(b'gAMA', struct.pack('>I', 45455)), IDAT_OK, IEND),
     'ChunkOutOfOrder', T22())
pair('ancillary-order/trns-after-idat', 'still', doc(IDAT_OK, chunk(b'tRNS', KEY), IEND), 'ChunkOutOfOrder', T22(), ev=('R1',))
# ancillary_duplicate=FirstWins (R1 when the dropped later chunk is a tRNS)
pair('ancillary-duplicate/gama', 'still', doc(chunk(b'gAMA', struct.pack('>I', 45455)), chunk(b'gAMA', struct.pack('>I', 100000)), IDAT_OK, IEND),
     'DuplicateChunk', T22())
pair('ancillary-duplicate/trns-first-wins', 'still', doc(chunk(b'tRNS', KEY), chunk(b'tRNS', struct.pack('>HHH', 1, 2, 3)), IDAT_OK, IEND),
     'DuplicateChunk', TRNS_OK_EXP, ev=('R1',))
# ancillary_values=Drop: a metadata chunk with an invalid value or length is discarded, no event
pair('ancillary-values/gamma-0', 'still', doc(chunk(b'gAMA', bytes(4)), IDAT_OK, IEND), 'InvalidGamma', T22())
pair('ancillary-values/gama-len-3', 'still', doc(chunk(b'gAMA', bytes(3)), IDAT_OK, IEND), 'InvalidChunkLength', T22())
pair('ancillary-values/srgb-intent-4', 'still', doc(chunk(b'sRGB', b'\4'), IDAT_OK, IEND), 'InvalidSrgb', T22())
pair('ancillary-values/phys-unit-2', 'still', doc(chunk(b'pHYs', struct.pack('>IIB', 1, 1, 2)), IDAT_OK, IEND), 'InvalidPhys', T22())
# trns_invalid=Drop (R1 for colour types 0, 2, 3)
pair('trns-invalid/palette-too-long', 'still', P3 + chunk(b'PLTE', PL3) + chunk(b'tRNS', b'\0' * 4) + P3IDAT + IEND, 'InvalidTransparency',
     sexp(2, 2, P3EXP), ev=('R1',))
pair('trns-invalid/rgb-length-2', 'still', doc(chunk(b'tRNS', bytes(2)), IDAT_OK, IEND), 'InvalidTransparency', T22(), ev=('R1',))
G1 = SIG + ihdr(2, 2, 8, 4) + chunk(b'tRNS', bytes(2)) + chunk(b'IDAT', z(scan([(1, 2)] * 4, 2, 2, 4, 8, 0))) + IEND
pair('trns-invalid/in-grey-alpha', 'still', G1, 'InvalidTransparency', sexp(2, 2, [rgba_of((1, 2), 4, 8)] * 4))
# plte_unexpected=Ignore
GV = [(1,), (2,), (3,), (4,)]
GI = SIG + ihdr(2, 2, 8, 0) + chunk(b'PLTE', PL3) + chunk(b'IDAT', z(scan(GV, 2, 2, 0, 8, 0))) + IEND
pair('plte-unexpected/in-greyscale', 'still', GI, 'UnexpectedPalette', sexp(2, 2, [rgba_of(s, 0, 8) for s in GV]))
pair('plte-unexpected/after-idat-rgb', 'still', doc(IDAT_OK, chunk(b'PLTE', PL3), IEND), 'ChunkOutOfOrder', T22())
# palette_size=Allow: indices cannot exceed 2^d - 1, so the extra entries are never drawn
P5 = [(0,), (1,), (3,), (2,)]
PL5 = pal(5)
pair('palette-size/5-entries-depth-2', 'still', SIG + ihdr(2, 2, 2, 3) + chunk(b'PLTE', PL5) + chunk(b'IDAT', z(scan(P5, 2, 2, 3, 2, 0))) + IEND,
     'InvalidPalette', sexp(2, 2, [rgba_of(s, 3, 2, PL5) for s in P5]))
# palette_index=Black (R3): index 4 with 4 entries is drawn (0,0,0,255)
PI = [(0,), (1,), (2,), (3,), (4,), (0,)]
PL4 = pal(4)
pair('palette-index/out-of-range-black', 'still', png(3, 2, 3, 8, PI, pre=chunk(b'PLTE', PL4)), 'PaletteIndexOutOfRange',
     sexp(3, 2, [(0, 0, 0, 255) if s[0] >= 4 else rgba_of(s, 3, 8, PL4) for s in PI]), ev=('R3',))
def bsz(data):
    return png(23, 11, 6, 8, data=data)


# zlib_tail=IgnoreTail: bytes after the Adler-32 are ignored; surplus decompressed output stays ExcessImageData
TOKZ = zwrap(stored(TINY_RAW), TINY_RAW)
tiny_exp = sexp(TINY_W, TINY_H, TINY_EXP)
pair('zlib-tail/trailing-byte', 'still', tiny(TOKZ + b'\0'), 'TrailingZlibData', tiny_exp)
pair('zlib-tail/second-stream', 'still', tiny(TOKZ * 2), 'TrailingZlibData', tiny_exp)
pair('zlib-tail/trailing-in-later-idat', 'still', SIG + ihdr(1, 2, 8, 0) + chunk(b'IDAT', TOKZ) + chunk(b'IDAT', b'\0') + IEND,
     'TrailingZlibData', tiny_exp)
pair('zlib-tail/excess-output-stays-an-error', 'still', bsz(z(BASE_RAW + b'\0')), 'ExcessImageData', 'ExcessImageData')
pair('zlib-tail/excess-row-stays-an-error', 'still', bsz(z(BASE_RAW + BASE_RAW[:1 + 23 * 4])), 'ExcessImageData', 'ExcessImageData')
pair('zlib-tail/excess-output-and-trailing-byte', 'still', bsz(z(BASE_RAW + b'\0') + b'\0'), 'ExcessImageData', 'ExcessImageData')
pair('zlib-tail/bad-adler-with-trailing-byte-stays-an-error', 'still', tiny(zwrap(stored(TINY_RAW), TINY_RAW, adler=1) + b'\0'),
     'AdlerMismatch', 'AdlerMismatch')
# window=Ignore: a distance beyond the declared window but within the output so far is accepted
pair('window/distance-beyond-declared-window', 'still', SIG + ihdr(402, 1, 8, 0) + chunk(b'IDAT', zwrap(bwf.flush(), payload, cmf=0x08)) + IEND,
     'DistanceTooFar', sexp(402, 1, [(v, v, v, 255) for v in payload[1:]]))
bw = BW()
bw.bits(1, 1)
bw.bits(1, 2)
bw.fixed_lit(257)
bw.code(0, 5)
pair('window/distance-before-output-stays-an-error', 'still', tiny(zwrap(bw.flush(), b'')), 'DistanceTooFar', 'DistanceTooFar')

POLICY = _GEN.V[START:]

if __name__ == '__main__':
    names = [v['name'] for v in POLICY]
    assert len(names) == len(set(names))
    (HERE / 'VECTORS-POLICY.json').write_text(json.dumps(POLICY, indent=0) + '\n')
    print(len(POLICY), 'vectors')
