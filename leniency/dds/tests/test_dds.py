import random, struct, sys, unittest
from pathlib import Path
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(HERE))
import harness, leniency_core as core  # noqa: E402

BUILT = harness.EXE.exists()


class Generator(unittest.TestCase):
    def test_deterministic(self):
        a, _ = harness.generate(400, 5); b, _ = harness.generate(400, 5); c, _ = harness.generate(400, 6)
        self.assertEqual(a, b); self.assertNotEqual(a, c)

    def test_header_layout(self):
        raw = harness.build(random.Random(1), 8, 8, fourcc='DXT1')
        self.assertEqual(raw[:4], b'DDS '); self.assertEqual(struct.unpack_from('<I', raw, 4)[0], 124)
        self.assertEqual(struct.unpack_from('<II', raw, 12), (8, 8)); self.assertEqual(raw[84:88], b'DXT1')
        self.assertEqual(len(raw), 128 + 4 * 8)  # 2x2 blocks of 8 bytes
        dx10 = harness.build(random.Random(1), 8, 8, dx10='BC7')
        self.assertEqual(dx10[84:88], b'DX10'); self.assertEqual(len(dx10), 148 + 4 * 16)
        mips = harness.build(random.Random(1), 16, 16, fourcc='DXT1', mips=3)
        self.assertEqual(len(mips), 128 + (16 + 4 + 1) * 8)  # 4x4, 2x2, 1x1 blocks... = 16+4+1 blocks

    def test_every_seed_family_present(self):
        fams = {f for _, f, _ in harness.seeds(random.Random(1))}
        for f in ('legacy-bcn', 'legacy-bc45', 'uncompressed-bitmask', 'dx10', 'mipmaps', 'cube', 'volume', 'array'): self.assertIn(f, fams)

    def test_mutation_provenance(self):
        items, _ = harness.generate(500, 5)
        for b, m in items:
            self.assertIn(m['source'], ('seed', 'mutation'))
            if m['source'] == 'mutation': self.assertTrue({'seed', 'rule', 'field', 'value'} <= set(m))


class Classification(unittest.TestCase):
    def test_na_dropped(self):
        r = {'a': {'status': 'ok', 'width': 1, 'height': 1, 'rgba_sha256': 'x'}, 'b': {'status': 'na'}}
        self.assertEqual(core.engine.classify(core.eff(r)), 'all-accept-identical-pixels')
        r['b'] = {'status': 'error'}
        self.assertEqual(core.engine.classify(core.eff(r)), 'accept-vs-reject')


@unittest.skipUnless(BUILT, 'run ./build.sh first')
class Decoders(unittest.TestCase):
    cfg = harness.DdsConfig()
    def decode_seed(self, name):
        for n, fam, b in harness.seeds(random.Random(731)):
            if n == name:
                p = Path('/tmp/claude-0/test-dds.dds'); p.write_bytes(b)
                return core.run_all(self.cfg, p, 10)
        raise KeyError(name)

    def test_image_and_glycin_are_one_vote(self):
        for name in ('legacy-dxt1-8x8', 'legacy-dxt5-8x8', 'dx10-bc3-8x8', 'mask-argb32-8x8'):
            r = self.decode_seed(name)
            self.assertEqual(r['image']['status'], r['glycin']['status'])
            if r['image']['status'] == 'ok': self.assertEqual(r['image']['rgba_sha256'], r['glycin']['rgba_sha256'])

    def test_valid_seeds_decode(self):
        r = self.decode_seed('legacy-dxt5-8x8')
        for n in ('image', 'glycin', 'image-dds', 'ddsfile-bcdec', 'ddsfile-texpresso'):
            self.assertEqual((r[n]['status'], r[n]['width'], r[n]['height']), ('ok', 8, 8), n)

    def test_uncompressed_only_in_independent_decoders(self):
        r = self.decode_seed('mask-argb32-8x8')
        self.assertEqual(r['image']['status'], 'error'); self.assertEqual(r['ddsfile-bcdec']['status'], 'ok'); self.assertEqual(r['ddsfile-texpresso']['status'], 'na')

    def test_bad_magic_rejected_everywhere(self):
        p = Path('/tmp/claude-0/test-dds2.dds'); p.write_bytes(b'XXXX' + bytes(200))
        r = core.run_all(self.cfg, p, 10)
        self.assertTrue(all(v['status'] in ('error', 'na') for v in r.values()), r)


if __name__ == '__main__':
    unittest.main()
