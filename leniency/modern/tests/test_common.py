"""Unit tests for the shared classification, tolerance and mutation helpers."""
import random, sys, unittest
from pathlib import Path
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import common as c

def ok(w=1, h=1, hs='a', frames=1, fs='f'):
    return {'status': 'ok', 'width': w, 'height': h, 'rgba_sha256': hs, 'frames': frames, 'frames_sha256': fs}

class T(unittest.TestCase):
    def test_pair_relations(self):
        a, b = bytes([10, 10, 10, 255]) * 4, bytes([12, 10, 10, 255]) * 4
        self.assertEqual(c.pair_relation(ok(2, 2, 'x'), ok(2, 2, 'y'), a, b, 2)[0], 'within-tol')
        self.assertEqual(c.pair_relation(ok(2, 2, 'x'), ok(2, 2, 'y'), a, b, 1)[0], 'different-pixels')
        self.assertEqual(c.pair_relation(ok(2, 2, 'x'), ok(2, 2, 'x'), a, a, 0)[0], 'equal')
        self.assertEqual(c.pair_relation(ok(2, 2), ok(4, 1), a, a, 0)[0], 'different-dimensions')
        self.assertEqual(c.pair_relation(ok(frames=2), ok(frames=3), a, a, 0)[0], 'different-frame-count')
    def test_transparent_rgb_only(self):
        a, b = bytes([1, 2, 3, 0]), bytes([9, 9, 9, 0])
        self.assertEqual(c.pair_relation(ok(1, 1, 'x'), ok(1, 1, 'y'), a, b, 2)[0], 'transparent-rgb')
    def test_classify_skip_is_not_a_vote(self):
        res = {'a': ok(), 'b': ok(), 'c': {'status': 'skip'}}
        self.assertEqual(c.classify(res, {('a', 'b'): ('equal', None)}), 'all-accept-identical-pixels')
        res['c'] = {'status': 'error'}
        self.assertEqual(c.classify(res, {}), 'accept-vs-reject')
        res['a'] = {'status': 'crash'}
        self.assertEqual(c.classify(res, {}), 'crash-or-timeout')
    def test_boundary_values_and_fields(self):
        v = c.boundary_values(5, 4)
        self.assertIn(0xffffffff, v); self.assertNotIn(5, v)
        out = list(c.field_mutations(bytes(8), [('f', 2, 2, 'big')]))
        self.assertTrue(all(len(b) == 8 for b, _ in out))
        self.assertEqual(out[0][1]['rule'], 'field:f')
    def test_generic_mutations_deterministic(self):
        d = bytes(range(64))
        a = list(c.generic_mutations(d, random.Random(1)))
        b = list(c.generic_mutations(d, random.Random(1)))
        self.assertEqual(a, b)
        self.assertTrue({'truncation', 'trailing-bytes', 'bit-flip'} <= {p['rule'] for _, p in a})

if __name__ == '__main__': unittest.main()
