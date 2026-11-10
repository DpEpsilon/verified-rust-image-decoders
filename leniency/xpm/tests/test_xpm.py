"""Run: PYTHONDONTWRITEBYTECODE=1 python3 -I xpm/tests/test_xpm.py  (decoder tests are skipped until ./build.sh has run)"""
import hashlib, sys, tempfile, unittest
from pathlib import Path
HERE = Path(__file__).resolve().parent.parent
sys.dont_write_bytecode = True
sys.path.insert(0, str(HERE.parent / 'xbm')); sys.path.insert(0, str(HERE.parent)); sys.path.insert(0, str(HERE))
import engine, lenient_text, xpm_gen, harness


def digest(items): return hashlib.sha256(b'\0'.join(b for b, _ in items)).hexdigest()


class Generator(unittest.TestCase):
    def test_deterministic_and_seed_sensitive(self):
        a, _ = xpm_gen.generate(700, 731); b, _ = xpm_gen.generate(700, 731); c, _ = xpm_gen.generate(700, 732)
        self.assertEqual(digest(a), digest(b)); self.assertNotEqual(digest(a), digest(c))

    def test_provenance(self):
        items, n = xpm_gen.generate(500, 731)
        self.assertTrue(all(m['source'] == 'seed' for _, m in items[:n]))
        self.assertTrue(all(m['source'] == 'mutation' and m['seed'] and m['rule'] for _, m in items[n:]))
        self.assertEqual(len({b for b, _ in items}), len(items))

    def test_seed_variants(self):
        items, n = xpm_gen.generate(300, 731); t = [b for b, _ in items[:n]]
        self.assertTrue(any(x.startswith(b'! XPM2') for x in t)); self.assertTrue(any(b'XPMEXT' in x for x in t))
        self.assertTrue(any(b' g4 ' in x for x in t)); self.assertTrue(any(b' m ' in x for x in t)); self.assertTrue(any(b' s sym' in x for x in t))
        self.assertTrue(any(b'None' in x for x in t)); self.assertTrue(any(b'\r\n' in x for x in t))
        for cpp in (b'"5 4 3 2"', b'"5 4 3 3"', b'"5 4 3 4"'): self.assertTrue(any(cpp in x for x in t), cpp)
        digits = {len(w) - 1 for x in t for w in (y.strip(b'",') for y in x.split()) if w.startswith(b'#') and len(w) > 1 and all(c in b'0123456789ABCDEF' for c in w[1:])}
        self.assertTrue({3, 6, 9, 12} <= digits, digits)

    def test_render_shape_xpm3(self):
        import random
        m = xpm_gen.make(dict(xpm_gen.BASE), random.Random(1)); b = xpm_gen.render(m).decode()
        self.assertTrue(b.startswith('/* XPM */\nstatic char * img[] = {\n"5 4 3 1"')); self.assertTrue(b.endswith('};\n'))
        self.assertEqual(b.count('",\n"'), 3 + 4)  # header + 3 colours + 4 rows -> 7 separators


class Framework(unittest.TestCase):
    def test_minimize(self):
        def fake(path, name): return {'status': 'ok' if b'KEEP' in Path(path).read_bytes() else 'error'}
        with tempfile.TemporaryDirectory() as d:
            out, _ = lenient_text.minimize(harness.CFG, b'a\nbbb KEEP ccc\nd\n', (('x', 'ok', None, None, None),), ['x'], 1, Path(d), 200, call=fake)
        self.assertIn(b'KEEP', out); self.assertLess(len(out), 10)


@unittest.skipUnless(harness.BIN.exists(), 'decoders not built')
class Decoders(unittest.TestCase):
    def test_base_seed_agrees(self):
        items, _ = xpm_gen.generate(1, 731)
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / 'a.xpm'; p.write_bytes(items[0][0])
            res = {n: lenient_text.invoke(harness.CFG, p, n) for n in harness.RUNNERS if n != 'ez-pixmap'}  # ez-pixmap lookups are case-sensitive (`Red`)
        self.assertEqual(engine.classify(res), 'all-accept-identical-pixels', res)


if __name__ == '__main__':
    unittest.main()
