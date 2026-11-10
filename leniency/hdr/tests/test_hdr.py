import random, struct, sys, unittest
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(HERE))
import harness, leniency_core as core  # noqa: E402

BUILT = harness.RUST.exists() and harness.STB.exists()


class Generator(unittest.TestCase):
    def test_deterministic(self):
        a, _ = harness.generate(400, 5); b, _ = harness.generate(400, 5); c, _ = harness.generate(400, 6)
        self.assertEqual(a, b); self.assertNotEqual(a, c)

    def test_rle_roundtrip_shape(self):
        b, info = harness.build(random.Random(1), 16, 2, mode='new')
        self.assertTrue(b.startswith(b'#?RADIANCE\n')); self.assertIn(b'-Y 2 +X 16\n', b)
        self.assertEqual(b[info['rows'][0]:info['rows'][0] + 4], bytes([2, 2, 0, 16]))

    def test_all_eight_orientations_have_seeds(self):
        names = {i['name'] for _, i in harness.seeds(random.Random(1))}
        for o in range(1, 8): self.assertIn(f'orient-{o}-rle-8x4', names)

    def test_close(self):
        self.assertTrue(harness.close(1.0, 1.0 + 5e-7)); self.assertFalse(harness.close(1.0, 1.0 + 4e-3))
        self.assertTrue(harness.close(float('nan'), float('nan'))); self.assertFalse(harness.close(float('inf'), 1.0))


@unittest.skipUnless(BUILT, 'run ./build.sh first')
class Decoders(unittest.TestCase):
    cfg = harness.HdrConfig()
    def run_seed(self, name):
        for b, info in harness.seeds(random.Random(731)):
            if info['name'] == name:
                p = Path('/tmp/claude-0/test-hdr.hdr'); p.write_bytes(b)
                return self.cfg.refine(p, core.run_all(self.cfg, p, 10), 10)
        raise KeyError(name)

    def test_image_family_is_one_vote_and_matches_stb_bits(self):
        r = self.run_seed('rle-new-16x16-mixed')
        for n in ('image', 'image-nonstrict', 'glycin', 'stb'): self.assertEqual(r[n]['status'], 'ok', n)
        self.assertEqual(len({r[n]['rgba_sha256'] for n in ('image', 'image-nonstrict', 'glycin', 'stb')}), 1)

    def test_flipcode_decoders_differ_by_256_over_255(self):
        r = self.run_seed('flat-16x4')
        self.assertNotEqual(r['image']['rgba_sha256'], r['hdrldr']['rgba_sha256'])
        self.assertEqual(r['image']['alt_sha256'], r['hdrldr']['alt_sha256'])
        self.assertAlmostEqual(r['hdrldr']['max_rel_err_vs_ref'], 1 / 256, places=4)

    def test_nonstandard_orientation_rejected_by_all(self):
        r = self.run_seed('orient-1-rle-8x4')
        self.assertTrue(all(v['status'] == 'error' for v in r.values()), r)


if __name__ == '__main__':
    unittest.main()
