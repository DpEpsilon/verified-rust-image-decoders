"""Generator determinism, seed validity across independent decoders, 16-bit normalisation, APNG frames."""
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest
import zlib

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
import pnggen  # noqa: E402
from png_adapter import FORMAT  # noqa: E402


class PngTests(unittest.TestCase):
    def test_seeded_generation_is_deterministic(self):
        a, base, _ = pnggen.generate(500, 731)
        b, _, _ = pnggen.generate(500, 731)
        self.assertEqual([(hashlib.sha256(x).digest(), m) for x, m in a], [(hashlib.sha256(x).digest(), m) for x, m in b])
        self.assertGreater(base, 60)

    def test_crc_and_zlib_of_seeds(self):
        for b, m in pnggen.generate(80, 731)[0]:
            if m['source'] != 'synthetic':
                continue
            pos, idat = 8, b''
            while pos < len(b):
                n = int.from_bytes(b[pos:pos + 4], 'big')
                t, d = b[pos + 4:pos + 8], b[pos + 8:pos + 8 + n]
                self.assertEqual(int.from_bytes(b[pos + 8 + n:pos + 12 + n], 'big'), zlib.crc32(t + d), m['name'])
                idat += d if t == b'IDAT' else b''
                pos += 12 + n
            zlib.decompress(idat)

    def test_static_seeds_agree(self):
        """libpng, spng, stb and the image-rs path agree on every static seed, including 16-bit exact samples."""
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            for b, m in pnggen.generate(80, 731)[0]:
                if m['source'] != 'synthetic' or m['name'].startswith('apng'):
                    continue
                p = Path(d) / 'x.png'
                p.write_bytes(b)
                r = {n: FORMAT.invoke(p, n, 5) for n in ('glycin', 'libpng', 'spng', 'stb', 'lodepng')}
                self.assertEqual(len({(v['status'], v.get('rgba_sha256'), v.get('rgba16_sha256')) for v in r.values()}), 1, (m['name'], r))

    def test_apng_frames(self):
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            b = next(x for x, m in pnggen.generate(80, 731)[0] if m.get('name') == 'apng-dispose1-blend0')
            p = Path(d) / 'a.png'
            p.write_bytes(b)
            g, ref, lib = (FORMAT.invoke(p, n, 5) for n in ('glycin-apng', 'pngcrate-apng', 'libpng'))
            self.assertEqual((g['frames'], g['seq_sha256']), (ref['frames'], ref['seq_sha256']))
            self.assertEqual(g['frames'], 3)
            self.assertEqual(lib['frames'], 1)


if __name__ == '__main__':
    unittest.main()
