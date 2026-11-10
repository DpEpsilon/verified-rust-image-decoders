"""Generator determinism, LZW round trip, seed validity across decoders, animation classification."""
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
import anim_harness as H  # noqa: E402
import gifgen  # noqa: E402
from gif_adapter import FORMAT  # noqa: E402


def lzw_decode(data, min_size):
    """Minimal reference GIF LZW decoder used only to check the encoder."""
    clear, end = 1 << min_size, (1 << min_size) + 1
    size, table, prev, out, acc, n, pos = min_size + 1, None, None, [], 0, 0, 0

    def reset():
        return {i: [i] for i in range(clear)}
    table = reset()
    nxt = end + 1
    while True:
        while n < size:
            if pos >= len(data):
                return out
            acc |= data[pos] << n
            pos += 1
            n += 8
        code = acc & ((1 << size) - 1)
        acc >>= size
        n -= size
        if code == clear:
            table, size, nxt, prev = reset(), min_size + 1, end + 1, None
            continue
        if code == end:
            return out
        entry = table[code] if code in table else table[prev] + [table[prev][0]]
        out += entry
        if prev is not None and nxt < 4096:
            table[nxt] = table[prev] + [entry[0]]
            nxt += 1
            if nxt == 1 << size and size < 12:
                size += 1
        prev = code


class GifTests(unittest.TestCase):
    def test_seeded_generation_is_deterministic(self):
        a, base, seeds = gifgen.generate(400, 731)
        b, _, _ = gifgen.generate(400, 731)
        self.assertEqual([(hashlib.sha256(x).digest(), m) for x, m in a], [(hashlib.sha256(x).digest(), m) for x, m in b])
        self.assertGreater(base, 40)
        self.assertTrue(all('rule' in m and 'source' in m for _, m in a))

    def test_lzw_round_trip(self):
        import random
        rng = random.Random(5)
        for mc in range(2, 9):
            pix = [rng.randrange(1 << mc) for _ in range(9000)]
            for opts in ({}, {'reset_full': False}, {'clear_every': 3}):
                self.assertEqual(lzw_decode(gifgen.lzw_encode(pix, mc, **opts), mc), pix, (mc, opts))

    def test_seeds_agree_across_independent_decoders(self):
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            for b, m in gifgen.generate(60, 731)[0]:
                if m['source'] != 'synthetic' or m['name'].startswith(('interlace', 'frame-inset', 'ext-unknown', 'big-dict-deferred')):
                    continue
                p = Path(d) / 'x.gif'
                p.write_bytes(b)
                r = {n: FORMAT.invoke(p, n, 5) for n in ('glycin', 'giflib', 'libnsgif', 'gdk-forced-whole')}
                self.assertEqual(len({(v['status'], v.get('rgba_sha256')) for v in r.values()}), 1, (m['name'], r))

    def test_classification(self):
        a = {'status': 'ok', 'width': 1, 'height': 1, 'rgba_sha256': 'a', 'frames': 2, 'seq_sha256': 'x'}
        b = dict(a, seq_sha256='y')
        res = {n: a for n in FORMAT.runners}
        self.assertEqual(H.animation_class(FORMAT, res), 'identical')
        res['giflib'] = b
        self.assertEqual(H.animation_class(FORMAT, res), 'sequence-pixels-differ')
        res['giflib'] = dict(a, frames=3)
        self.assertEqual(H.animation_class(FORMAT, res), 'frame-count-differs')
        self.assertNotIn('tinygif', H.family_votes(FORMAT, res))


if __name__ == '__main__':
    unittest.main()
