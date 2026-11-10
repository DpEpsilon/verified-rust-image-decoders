"""Determinism, writer/parser and decoder smoke tests for the TIFF adapter.
Run after ../jpeg/build.sh:  PYTHONDONTWRITEBYTECODE=1 python3 -I tests/test_tiff.py"""
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
sys.path.append(str(ROOT.parent / 'jpeg'))
import harness as h  # noqa: E402
import jt_harness as jt  # noqa: E402
import tiffgen as tg  # noqa: E402


def seed(name):
    return next(b for b, m in h.seeds() if m['name'] == name)


def run(data):
    with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
        p = Path(d) / 'x.tif'
        p.write_bytes(data)
        return jt.evaluate(h.AD, p, 5, Path(d))


class TiffTests(unittest.TestCase):
    def test_seeded_generation_is_deterministic(self):
        a, nb, _ = h.generate(1500, 731)
        b, _, _ = h.generate(1500, 731)
        self.assertEqual([(hashlib.sha256(x).digest(), m) for x, m in a], [(hashlib.sha256(x).digest(), m) for x, m in b])
        for _, m in a[nb:]:
            self.assertTrue({'rule', 'field', 'value', 'seed', 'ifd'} <= set(m), m)

    def test_parser_reads_classic_and_bigtiff(self):
        for name in ('rgb8-none-MM', 'bigtiff-multipage'):
            info = tg.parse(seed(name))
            tags = {e['tag'] for e in info['ifds'][0]['entries']}
            self.assertTrue({256, 257, 273, 279} <= tags, name)
        self.assertEqual(len(tg.parse(seed('bigtiff-multipage'))['ifds']), 2)

    def test_packbits_and_lzw_match_uncompressed(self):
        ref = run(seed('rgb8-none-II'))[0]['libtiff']['rgba_sha256']
        for name in ('rgb8-packbits-II', 'rgb8-lzw-II', 'rgb8-deflate-II'):
            res = run(seed(name))[0]
            self.assertEqual(res['libtiff']['rgba_sha256'], ref, name)
            self.assertEqual(res['tiff-crate']['rgba_sha256'], ref, name)

    def test_lzw_width_growth_round_trips(self):
        # 48x40 RGB exceeds 254 codes, so 10-bit codes and the early-change rule are exercised.
        res, _, _, _, cls = run(seed('rgb8-lzw-big-48x40'))
        self.assertEqual(cls, 'all-accept-identical-pixels', res)

    def test_orientation_reported(self):
        res = run(seed('orientation-6'))[0]
        self.assertEqual(res['libtiff']['orientation'], 6)
        self.assertEqual(res['tiff-crate']['orientation'], 6)


if __name__ == '__main__':
    unittest.main()
