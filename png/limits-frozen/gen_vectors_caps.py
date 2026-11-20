#!/usr/bin/env python3
"""Generate VECTORS-CAPS.json: the Strict records whose outcome depends on a cap value (SPEC P1, LIMITS.md).

Encoder helpers come from ../spec-draft/gen_vectors.py (imported; its records are not written here) and the values from
limits.py next to this file. Record names keep the shapes of the current values; an assertion below fails when a value
changes so that a name or an expectation no longer holds, and the records must then be re-authored (REVIEW-BRIEF.md,
Stage 2). Expectations are written from intent, never computed by the oracle (P19).
Usage: python3 -I gen_vectors_caps.py   (writes VECTORS-CAPS.json next to this file; deterministic)
"""
import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
import gen_vectors as GV  # noqa: E402
from gen_vectors import (SIG, IEND, TOK, IDAT_OK, Z0, AHDR, chunk, ihdr, doc, z, actl, fctl, fdat, add, bad, err,  # noqa: E402
                         hashlib, json)

_spec = importlib.util.spec_from_file_location('png_limits', HERE / 'limits.py')
L = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(L)

# the shapes below are named for these values
assert L.MAX_DIMENSION == (1 << 31) - 1 and L.MAX_OUTPUT == 1 << 28 and L.MAX_FRAMES == 65536
assert L.MAX_TOTAL_PIXELS == 1 << 30

START = len(GV.V)
# P4: the area cap and the per-side cap
for nm, (w_, h_, dd, cc_, cm, fm, im), e in [
        ('width-2^31-1-output', ((1 << 31) - 1, 1, 8, 2, 0, 0, 0), 'OutputTooLarge'),
        ('67108865x1-output', (67108865, 1, 8, 2, 0, 0, 0), 'OutputTooLarge'),
        ('8192x8193-output', (8192, 8193, 8, 2, 0, 0, 0), 'OutputTooLarge')]:
    bad(f'ihdr/{nm}', SIG + chunk(b'IHDR', GV.struct.pack('>IIBBBBB', w_, h_, dd, cc_, cm, fm, im)) + IDAT_OK + IEND, e)
# the largest admissible outputs (4*W*H = MAX_OUTPUT): header accepted, data then missing (shows the caps pass)
bad('ihdr/8192x8192-accepted-then-truncated-zlib', SIG + ihdr(8192, 8192, 8, 6) + chunk(b'IDAT', b'\x78\x01') + IEND, 'TruncatedZlib')
bad('ihdr/67108864x1-accepted-then-truncated-zlib', SIG + ihdr(67108864, 1, 8, 6) + chunk(b'IDAT', b'\x78\x01') + IEND, 'TruncatedZlib')
bad('ihdr/1x67108864-accepted-then-truncated-zlib', SIG + ihdr(1, 67108864, 8, 6) + chunk(b'IDAT', b'\x78\x01') + IEND, 'TruncatedZlib')
# P2: the input cap is tested with >
bad('limits/file-too-large', TOK, 'FileTooLarge')
GV.V[-1]['pad_to'] = L.MAX_INPUT + 1
# D18: exactly MAX_INPUT bytes is not FileTooLarge: the walk ends at IEND, then TrailingBytes
bad('limits/exact-max-input-trailing', TOK, 'TrailingBytes')
GV.V[-1]['pad_to'] = L.MAX_INPUT
# P17: the ICC limit
big = z(bytes(L.MAX_ICC + 1), level=9)
add('icc/too-large', 'icc', doc(chunk(b'iCCP', b'n\0\0' + big), IDAT_OK, IEND), err('IccProfileTooLarge'))
exact = z(bytes(L.MAX_ICC), level=9)
add('icc/exactly-max', 'icc', doc(chunk(b'iCCP', b'n\0\0' + exact), IDAT_OK, IEND),
    {'icc_len': L.MAX_ICC, 'icc_sha256': hashlib.sha256(bytes(L.MAX_ICC)).hexdigest()})
# P14.1: the frame cap
add('apng-bad/actl-too-many-frames', 'frames', AHDR + actl(L.MAX_FRAMES + 1) + chunk(b'IDAT', Z0) + IEND, err('TooManyFrames'))
# pixel budget: 65 full 8192x2048 frames (2^24 px each) exceed MAX_TOTAL_PIXELS; the walk stops before any inflate
BW_, BH_ = 8192, 2048
assert 64 * BW_ * BH_ == L.MAX_TOTAL_PIXELS
bb = SIG + ihdr(BW_, BH_, 8, 6) + actl(65) + chunk(b'IDAT', b'\x78\x01')
seq = 0
for k in range(65):
    bb += fctl(seq, BW_, BH_, 0, 0)
    bb += fdat(seq + 1, b'\x78\x01')
    seq += 2
add('apng-bad/pixel-budget', 'frames', bb + IEND, err('PixelBudgetExceeded'))
bb = SIG + ihdr(BW_, BH_, 8, 6) + actl(64) + chunk(b'IDAT', b'\x78\x01')
seq = 0
for k in range(64):
    bb += fctl(seq, BW_, BH_, 0, 0) + fdat(seq + 1, b'\x78\x01')
    seq += 2
add('apng-bad/pixel-budget-exactly-2^30-then-zlib', 'frames', bb + IEND, err('TruncatedZlib'))
CAPS = GV.V[START:]

if __name__ == '__main__':
    names = [v['name'] for v in CAPS]
    assert len(names) == len(set(names))
    (HERE / 'VECTORS-CAPS.json').write_text(json.dumps(CAPS, indent=0) + '\n')
    print(len(CAPS), 'vectors')
