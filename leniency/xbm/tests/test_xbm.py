"""Run: PYTHONDONTWRITEBYTECODE=1 python3 -I xbm/tests/test_xbm.py  (decoder tests are skipped until ./build.sh has run)"""
import hashlib, sys, tempfile, unittest
from pathlib import Path
HERE = Path(__file__).resolve().parent.parent
sys.dont_write_bytecode = True
sys.path.insert(0, str(HERE)); sys.path.insert(0, str(HERE.parent))
import engine, lenient_text, xbm_gen, harness


def digest(items): return hashlib.sha256(b'\0'.join(b for b, _ in items)).hexdigest()


class Generator(unittest.TestCase):
    def test_deterministic_and_seed_sensitive(self):
        a, _ = xbm_gen.generate(700, 731); b, _ = xbm_gen.generate(700, 731); c, _ = xbm_gen.generate(700, 732)
        self.assertEqual(digest(a), digest(b)); self.assertNotEqual(digest(a), digest(c))

    def test_seeds_first_and_have_provenance(self):
        items, n = xbm_gen.generate(400, 731)
        self.assertTrue(all(m['source'] == 'seed' for _, m in items[:n]))
        self.assertTrue(all(m['source'] == 'mutation' and m['seed'] and m['rule'] for _, m in items[n:]))
        self.assertEqual(len({b for b, _ in items}), len(items))  # deduplicated

    def test_seed_syntax_variants_present(self):
        items, n = xbm_gen.generate(n_ := 200, 731); texts = [b for b, _ in items[:n]]
        self.assertTrue(any(b'short' in t for t in texts)); self.assertTrue(any(b'_x_hot' in t for t in texts))
        self.assertTrue(any(b'\r\n' in t for t in texts)); self.assertTrue(any(b'unsigned char' in t for t in texts))
        self.assertTrue(any(b'static' not in t for t in texts)); self.assertTrue(any(b'//' in t for t in texts))

    def test_roles(self):
        import random
        b = xbm_gen.build(dict(xbm_gen.BASE, hot=(1, 2)), random.Random(1)); r = xbm_gen.roles(b)
        self.assertEqual(set(r), {'width', 'height', 'x_hot', 'y_hot'}); self.assertEqual(b[slice(*r['width'])], b'13')

    def test_x10_word_count(self):
        import random
        b = xbm_gen.build(dict(xbm_gen.BASE, style='x10', w=17, h=3), random.Random(1)); s, e = xbm_gen.data_span(b)
        self.assertEqual(b[s:e].count(b'0x'), 2 * 3)  # 17 px -> two 16-bit words per row


class Framework(unittest.TestCase):
    def test_classify(self):
        ok = lambda h: {'status': 'ok', 'width': 1, 'height': 1, 'rgba_sha256': h}; er = {'status': 'error'}
        self.assertEqual(engine.classify({'a': ok('x'), 'b': ok('x')}), 'all-accept-identical-pixels')
        self.assertEqual(engine.classify({'a': ok('x'), 'b': ok('y')}), 'accept-different-pixels')
        self.assertEqual(engine.classify({'a': ok('x'), 'b': er}), 'accept-vs-reject')
        self.assertEqual(engine.classify({'a': er, 'b': {'status': 'crash'}}), 'crash-or-timeout')

    def test_minimize_preserves_signature(self):
        def fake(path, name): return {'status': 'ok' if b'KEEP' in Path(path).read_bytes() else 'error'}
        with tempfile.TemporaryDirectory() as d:
            out, used = lenient_text.minimize(harness.CFG, b'a\nbbb KEEP ccc\nd\n', (('x', 'ok', None, None, None),), ['x'], 1, Path(d), 200, call=fake)
        self.assertIn(b'KEEP', out); self.assertLess(len(out), 10)

    def test_stats_outliers(self):
        r = lambda s: {'status': s}
        rows = [{'results': {'a': r('ok'), 'b': r('error'), 'c': r('error'), 'd': r('error')}}]
        cfg = {'runners': dict.fromkeys('abcd'), 'independent': {n: n for n in 'abcd'}}
        st = lenient_text.build_stats(cfg, iter(rows)); self.assertEqual(st['group_only_accepts'], {'a': 1})


@unittest.skipUnless(harness.BIN.exists() and (harness.BIN / 'x11').exists(), 'decoders not built')
class Decoders(unittest.TestCase):
    def test_base_seed_agrees(self):
        items, _ = xbm_gen.generate(1, 731)
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / 'a.xbm'; p.write_bytes(items[0][0])
            res = {n: lenient_text.invoke(harness.CFG, p, n) for n in harness.RUNNERS}
        self.assertEqual(engine.classify(res), 'all-accept-identical-pixels', res)


if __name__ == '__main__':
    unittest.main()
