#!/usr/bin/env python3
"""Exhaustive check of P14.4's integer OVER over its whole operand domain (PNG-Q8).

For every source alpha sa in 1..254, destination alpha da in 0..255 and colour samples sc, dc in 0..255
(4,261,412,864 combinations; sa = 0 and sa = 255 are the two special cases of P14.4 and never divide) it
checks, with A = 255*sa + da*(255-sa) > 0 and N = 255*sa*sc + da*(255-sa)*dc:
  1. the colour result c = floor((2N + A) / (2A)) is at most 255 (so no narrowing is needed);
  2. c is the round-half-up of N/A: 0 <= 2N + A - 2A*c < 2A;
  3. the alpha result a = floor((2A + 255) / 510) is at most 255, and is exactly sa when da = 0;
  4. when da = 0, c = sc (a transparent destination contributes nothing to the colour).
It also checks that the oracle's `over` agrees with the closed form on a sample (every sa, da with fixed colours).
Compiles a C program into a temporary directory (removed afterwards); about half a minute on one core.
Usage: python3 -I check_over.py
"""
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import png_oracle as O  # noqa: E402

C = r'''
#include <stdio.h>
#include <stdint.h>
int main(void) {
    uint64_t n = 0, bad = 0;
    for (int64_t sa = 1; sa <= 254; sa++)
    for (int64_t da = 0; da <= 255; da++) {
        int64_t A = 255 * sa + da * (255 - sa);
        int64_t a = (2 * A + 255) / 510;
        if (a > 255 || (da == 0 && a != sa)) bad++;
        for (int64_t sc = 0; sc <= 255; sc++)
        for (int64_t dc = 0; dc <= 255; dc++) {
            int64_t N = 255 * sa * sc + da * (255 - sa) * dc;
            int64_t c = (2 * N + A) / (2 * A);
            int64_t r = 2 * N + A - 2 * A * c;
            n++;
            if (c > 255 || r < 0 || r >= 2 * A || (da == 0 && c != sc)) bad++;
        }
    }
    printf("%llu %llu\n", (unsigned long long)n, (unsigned long long)bad);
    return 0;
}
'''


def main():
    with tempfile.TemporaryDirectory() as d:
        src, exe = Path(d, 'over.c'), Path(d, 'over')
        src.write_text(C)
        subprocess.run(['gcc', '-O2', '-o', str(exe), str(src)], check=True)
        n, bad = map(int, subprocess.run([str(exe)], check=True, capture_output=True, text=True, timeout=1200).stdout.split())
    sample_bad = 0
    for sa in range(256):
        for da in range(256):
            s, dd = (200, 100, 50, sa), (10, 20, 30, da)
            if sa in (0, 255):
                want = dd if sa == 0 else s
            else:
                A = 255 * sa + da * (255 - sa)
                want = tuple((2 * (255 * sa * s[i] + da * (255 - sa) * dd[i]) + A) // (2 * A) for i in range(3)) + ((2 * A + 255) // 510,)
            sample_bad += O.over(s, dd) != want
    print(f'exhaustive OVER: {n} operand tuples, {bad} violations; oracle sample {sample_bad} mismatches')
    return 1 if bad or sample_bad or n != 254 * 256 * 256 * 256 else 0


if __name__ == '__main__':
    sys.exit(main())
