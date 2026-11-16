#!/usr/bin/env python3
"""Exhaustive check of gif_oracle.plays / total_plays (SPEC.md G20) over all 65,537 inputs:
None and every u16. Expectations restate the browser convention independently: absent
block = one play, stored 0 = forever, stored n >= 1 = n repeats after the first play.
Usage: python3 -I check_plays.py"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import gif_oracle as O  # noqa: E402

U32_MAX = (1 << 32) - 1


def expected(loop):
    if loop is None:
        return ('Once',), 1
    if loop == 0:
        return ('Forever',), None
    return ('Repeats', loop), loop + 1


def main():
    inputs = [None] + list(range(1 << 16))
    bad, outputs = 0, set()
    for loop in inputs:
        p = O.plays(loop)
        t = O.total_plays(p)
        if (p, t) != expected(loop):
            bad += 1
            print('MISMATCH', loop, p, t)
        if t is not None and not 1 <= t <= U32_MAX:     # total_plays is computed in u32 without overflow
            bad += 1
            print('RANGE', loop, t)
        outputs.add(p)
    injective = len(outputs) == len(inputs)
    tmax = max(O.total_plays(O.plays(n)) for n in range(1, 1 << 16))
    print(f'plays: {len(inputs) - bad}/{len(inputs)} inputs as specified; injective: {injective}; '
          f'largest total_plays {tmax} (u32 max {U32_MAX})')
    return 1 if bad or not injective else 0


if __name__ == '__main__':
    sys.exit(main())
