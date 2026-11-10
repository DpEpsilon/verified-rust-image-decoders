"""Determinism, provenance, normalisation and decoder smoke tests for the qoi adapter. Run after ext/build.sh:
PYTHONDONTWRITEBYTECODE=1 python3 -I tests/test_qoi.py"""
import hashlib, sys, tempfile, unittest
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
import harness as h

SMOKE = {'pnm': ('P6-4x3-max255', ['image', 'glycin', 'zune-ppm', 'oxideav-pbm', 'gdk-file', 'gdk-forced']),
         'qoi': ('noise-8x8-c4', None), 'farbfeld': ('random-2x2', None)}['qoi']


class Tests(unittest.TestCase):
    def test_seeded_generation_is_deterministic(self):
        a, n = h.common.generate(h, 400, 731)
        b, _ = h.common.generate(h, 400, 731)
        c, _ = h.common.generate(h, 400, 732)
        digest = lambda xs: [(hashlib.sha256(x).hexdigest(), sorted(m.items(), key=str)) for x, m in xs]
        self.assertEqual(digest(a), digest(b))
        self.assertNotEqual([x for x, _ in a], [x for x, _ in c])
        self.assertGreater(n, 20)
        self.assertEqual(len({hashlib.sha256(x).digest() for x, _ in a}), len(a), 'inputs are unique')

    def test_provenance_fields(self):
        items, _ = h.common.generate(h, 300, 731)
        for _, m in items:
            self.assertIn(m['source'], ('seed', 'mutation'))
            self.assertIn('rule', m)
            self.assertIn('seed', m)
        self.assertTrue(any(m['source'] == 'mutation' and m.get('field') for _, m in items))
        rules = {m['rule'] for _, m in items}
        self.assertGreater(len(rules), 8)

    def test_classification_precision_class(self):
        ok = lambda h16: {'status': 'ok', 'width': 1, 'height': 1, 'rgba_sha256': 'a', 'rgba16_sha256': h16}
        self.assertEqual(h.common.classify16({'x': ok('1'), 'y': ok('1')}), 'all-accept-identical-pixels')
        self.assertEqual(h.common.classify16({'x': ok('1'), 'y': ok('2')}), 'accept-identical-rgba8-different-rgba16')
        self.assertEqual(h.common.classify16({'x': ok('1'), 'y': {'status': 'error'}}), 'accept-vs-reject')
        self.assertEqual(h.common.classify16({'x': ok('1'), 'y': {'status': 'crash'}}), 'crash-or-timeout')

    def test_smoke_seed_decodes(self):
        data = next(b for b, m in h.seeds() if m['name'] == SMOKE[0])
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            p = Path(d) / ('s.' + h.EXT)
            p.write_bytes(data)
            res = h.common.invoke_all(h, p, 5)
        names = SMOKE[1] or list(h.DECODERS)
        self.assertTrue(all(res[k]['status'] == 'ok' for k in names), res)
        self.assertEqual(len({res[k]['rgba_sha256'] for k in names}), 1, res)


if __name__ == '__main__':
    unittest.main()
