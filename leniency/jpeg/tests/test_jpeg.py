"""Determinism, provenance, comparator and decoder smoke tests for the JPEG adapter.
Run after ./build.sh:  PYTHONDONTWRITEBYTECODE=1 python3 -I tests/test_jpeg.py"""
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
import harness as h  # noqa: E402
import jpegenc  # noqa: E402
import jt_harness as jt  # noqa: E402


def seed(name):
    return next(b for b, m in h.seeds(Path('/tmp/claude-0/leniency-jpeg/test-gen')) if m['name'] == name)


class JpegTests(unittest.TestCase):
    def test_seeded_generation_is_deterministic(self):
        a, nb, ns = h.generate(1500, 731)
        b, _, _ = h.generate(1500, 731)
        self.assertEqual([(hashlib.sha256(x).digest(), m) for x, m in a], [(hashlib.sha256(x).digest(), m) for x, m in b])
        self.assertGreaterEqual(nb, 80)
        for _, m in a[nb:]:
            self.assertEqual(m['source'], 'mutation')
            self.assertTrue({'rule', 'field', 'value', 'seed'} <= set(m), m)

    def test_huffman_tables_satisfy_kraft(self):
        bits, vals = jpegenc.huff_lengths({i: (i * 37) % 11 + 1 for i in range(200)})
        self.assertEqual(sum(bits), 200)
        self.assertLess(sum(n / 2 ** (k + 1) for k, n in enumerate(bits)), 1.0)

    def test_segment_parser_finds_frame_and_scan(self):
        markers = [m for m, _, _ in h.segments(seed('ycc-444-17x9'))]
        self.assertIn(0xC0, markers)
        self.assertIn(0xDA, markers)
        self.assertEqual(markers[-1], 0xD9)

    def test_tolerance_and_orientation_comparator(self):
        a = bytes([10, 20, 30, 255]) * 6
        b = bytes([11, 19, 32, 255]) * 6  # max 2, mean 1.0
        self.assertTrue(jt.tol_equal(jt.diff(a, b)))
        img = bytes(range(24))  # 3x2 RGBA
        rot, w, hh = jt.transform(img, 3, 2, 6)
        back, _, _ = jt.transform(rot, w, hh, 8)
        self.assertEqual((w, hh, back), (2, 3, img))
        ok = lambda w_, h_, s: {'status': 'ok', 'width': w_, 'height': h_, 'rgba_sha256': s}
        res = {'x': ok(3, 2, 'a'), 'y': ok(2, 3, 'b')}
        groups, n, pairs = jt.compare(res, {'x': img, 'y': rot})
        self.assertEqual(pairs[0][4], 'orientation-6')  # applying EXIF 6 to x gives y
        self.assertEqual(jt.classify(res, groups, pairs), 'all-accept-orientation-differs')

    def test_baseline_seed_agrees_within_tolerance(self):
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            p = Path(d) / 'x.jpg'
            p.write_bytes(seed('ycc-444-33x31'))
            res, groups, _, pairs, cls = jt.evaluate(h.AD, p, 5, Path(d))
            self.assertIn(cls, ('all-accept-identical-pixels', 'all-accept-within-tolerance'), (res, pairs))

    def test_python_encoder_control_decodes(self):
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            p = Path(d) / 'x.jpg'
            p.write_bytes(seed('sof1-8bit-python-3c-17x9'))
            res, _, _, _, cls = jt.evaluate(h.AD, p, 5, Path(d))
            self.assertEqual(res['libjpeg']['status'], 'ok', res)
            self.assertEqual(res['glycin']['status'], 'ok', res)


if __name__ == '__main__':
    unittest.main()
