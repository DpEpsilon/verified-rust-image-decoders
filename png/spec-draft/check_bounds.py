#!/usr/bin/env python3
"""Check P11's raw-size bound (PNG-Q5, Adam7 correction) and the worked examples quoted in SPEC.md.

R(w,h) = sum over non-empty passes of ph*(1 + ceil(pw*B/8)), B = bits per pixel <= 64. For interlaced images
the rows term is S(h) = sum of ph over non-empty passes, which exceeds h, so R <= h*(1 + 8w) is false.
Bound used by the proofs: R <= 8*w*h + S(h) and S(h) <= 2h + 2. Checked here:
  - S(h) <= ceil(15h/8) + 2 <= 2h + 2 for every h in 1..70000 (w large enough that all seven passes with y0 < h are non-empty);
  - R <= 8*w*h + 2h + 2 for every w, h in 1..96 and every (colour type, depth) pair, interlaced and not;
  - R <= h*(1 + 8w) fails for interlaced images (a counterexample is printed), holds when not interlaced;
  - the worked examples of SPEC.md P11.
Usage: python3 -I check_bounds.py
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import png_oracle as O  # noqa: E402

PAIRS = [(0, d) for d in (1, 2, 4, 8, 16)] + [(2, 8), (2, 16)] + [(3, d) for d in (1, 2, 4, 8)] + [(4, 8), (4, 16), (6, 8), (6, 16)]


def doc(ct, d, il):
    return {'color_type': ct, 'depth': d, 'interlace': il}


def S(h):
    return sum(ph for *_, pw, ph in O.passes(doc(0, 8, 1), 10 ** 6, h))


def main():
    bad = 0
    worst = max(S(h) - 2 * h for h in range(1, 70001))
    worst2 = max(S(h) - (15 * h + 7) // 8 for h in range(1, 70001))
    if worst > 2 or worst2 > 2:
        bad += 1
    print(f'max over h<=70000 of S(h) - 2h = {worst}, of S(h) - ceil(15h/8) = {worst2} (bounds: 2, 2)')
    viol = None
    for ct, d in PAIRS:
        for il in (0, 1):
            dd = doc(ct, d, il)
            for w in range(1, 97):
                for h in range(1, 97):
                    r = O.raw_size(dd, w, h)
                    if r > 8 * w * h + 2 * h + 2:
                        bad += 1
                    if r > h * (1 + 8 * w):
                        if not il:
                            bad += 1
                        elif viol is None or r - h * (1 + 8 * w) > viol[0]:
                            viol = (r - h * (1 + 8 * w), ct, d, w, h, r)
    print(f'R <= 8wh + 2h + 2: {"holds" if not bad else "VIOLATED"} for 13 (colour type, depth) pairs x 2 interlace modes x 96 x 96 sizes')
    print(f'old bound h(1+8w) fails when interlaced; largest excess {viol[0]} at colour type {viol[1]}, depth {viol[2]}, {viol[3]}x{viol[4]}: R = {viol[5]}')
    # worked examples (RGBA16, interlaced): 5x5, 1024x1024, 16384x1024
    d = doc(6, 16, 1)
    for w, h in ((5, 5), (1024, 1024), (16384, 1024)):
        print(f'RGBA16 interlaced {w}x{h}: R = {O.raw_size(d, w, h)}; h(1+8w) = {h * (1 + 8 * w)}; 8wh + S(h) = {8 * w * h + S(h)}')
    return 1 if bad else 0


if __name__ == '__main__':
    sys.exit(main())
