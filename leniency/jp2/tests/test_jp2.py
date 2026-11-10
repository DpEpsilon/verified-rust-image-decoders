"""Seed determinism, decoder smoke test on a lossless seed, and provenance of mutations (jp2)."""
import hashlib, sys, tempfile, unittest
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
import harness as h, common as c

class T(unittest.TestCase):
    def test_seeded_generation(self):
        ad = h.JP2()
        a, ns, _ = c.generate(ad, 400, 731)
        b, _, _ = c.generate(ad, 400, 731)
        self.assertGreater(ns, 25)
        self.assertEqual([(hashlib.sha256(x).digest(), m['rule']) for x, m in a], [(hashlib.sha256(x).digest(), m['rule']) for x, m in b])
        self.assertTrue(all('rule' in m and 'seed' in m for _, m in a))
        self.assertGreater(len({m['rule'] for _, m in a}), 20)
    def test_lossless_seed_all_decoders_agree(self):
        ad = h.JP2()
        data = next(d for d, m in ad.seeds() if m['name'] == 'j2k-gray8-ll')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as t:
            p = Path(t) / 'x.jp2'; p.write_bytes(data)
            res, rels = ad.evaluate(p, 10, t, 'x')
            self.assertEqual(c.classify(res, rels), 'all-accept-identical-pixels', res)
    def test_truncated_seed_is_rejected_by_everyone(self):
        ad = h.JP2()
        data = next(d for d, m in ad.seeds() if m['name'] == 'j2k-gray8-ll')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as t:
            p = Path(t) / 'x.jp2'; p.write_bytes(data[:10])
            res, rels = ad.evaluate(p, 10, t, 'x')
            self.assertNotIn('ok', {r['status'] for r in res.values()}, res)

if __name__ == '__main__': unittest.main()
