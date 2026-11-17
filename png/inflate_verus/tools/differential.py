#!/usr/bin/env python3
"""M1a runtime evidence, not a mathematical model or a Verus witness proof.

Use python3 -I. Inputs are generated or read from the frozen synthetic vectors;
no corpus or generated stream bytes are written to disk. The Rust adapter checks
into/measure equality, untouched suffix, last-window contents, and observed work.
"""
import argparse
import json
import os
from pathlib import Path
import random
import struct
import subprocess
import sys
import zlib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
PNG = ROOT.parent
sys.path.insert(0, str(PNG / 'spec-frozen'))
import check_vectors as V
O = V.O

INFLATE_ERRORS = {
    'TruncatedZlib', 'InvalidZlibHeader', 'UnsupportedZlibMethod', 'ZlibPresetDictionary',
    'InvalidBlockType', 'StoredLengthMismatch', 'InvalidCodeLengths', 'InvalidSymbol',
    'DistanceTooFar', 'AdlerMismatch', 'TrailingZlibData',
}
D4 = {
    'deflate/single-literal-code-other-bit-then-eof': 'InvalidSymbol',
    'deflate/length-with-no-distance-codes-then-eof': 'InvalidSymbol',
    'deflate/single-distance-code-other-bit-then-eof': 'InvalidSymbol',
    'deflate/single-literal-code-live-prefix-eof': 'TruncatedZlib',
    'deflate/single-distance-code-live-prefix-eof': 'TruncatedZlib',
}


def oracle(data, limit, options='S'):
    """Observe the frozen inflate's written prefix even when an exception escapes.

    This P9 observation wrapper is test scaffolding only; O.inflate and O.adler32
    remain frozen. IgnoreTail's exactly-L restriction follows THEOREMS/OPTIONS.
    """
    out = bytearray()
    try:
        if len(data) < 2:
            O.fail('TruncatedZlib')
        cmf, flg = data[:2]
        if (cmf * 256 + flg) % 31:
            O.fail('InvalidZlibHeader')
        if cmf & 15 != 8 or cmf >> 4 > 7:
            O.fail('UnsupportedZlibMethod')
        if flg & 0x20:
            O.fail('ZlibPresetDictionary')
        window = 32768 if options in ('B', 'W') else 1 << ((cmf >> 4) + 8)
        out, br = O.inflate(data, 2, limit, window, out)
        br.align()
        if br.byte + 4 > len(data):
            O.fail('TruncatedZlib')
        if O.be32(data, br.byte) != O.adler32(out):
            O.fail('AdlerMismatch')
        if br.byte + 4 != len(data) and not (options in ('B', 'T') and len(out) == limit):
            O.fail('TrailingZlibData')
        return 'Done', bytes(out)
    except O.OutputLimit:
        return 'Full', bytes(out)
    except O.PngError as e:
        return e.name, bytes(out)


class Runner:
    def __init__(self, binary):
        self.proc = subprocess.Popen([str(binary)], cwd=ROOT, stdin=subprocess.PIPE,
                                     stdout=subprocess.PIPE, text=True, bufsize=1)
        self.calls = 0

    def run(self, data, limit, options='S', layout=0, expected=None, name=''):
        self.proc.stdin.write(f'{limit}\t{options}\t{layout}\t{data.hex()}\n')
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        assert line, f'Rust adapter exited on {name}; status {self.proc.poll()}'
        status, k, raw, core, seg, pos = line.rstrip('\n').split('\t')
        output = bytes.fromhex(raw)
        assert int(k) == len(output), name
        result = status, output
        if expected is not None:
            assert result == expected, (name, limit, options, layout,
                status, len(output), expected[0], len(expected[1]),
                output[:40].hex(), expected[1][:40].hex())
        self.calls += 1
        return result

    def close(self):
        self.proc.stdin.close()
        # The outer logged command also bounds the complete harness run.
        try:
            rc = self.proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            self.proc.wait(timeout=10)
            raise
        self.proc.stdout.close()
        assert rc == 0, rc


def chunks(data):
    p = 8
    while p + 12 <= len(data):
        n = int.from_bytes(data[p:p + 4], 'big')
        if p + 12 + n > len(data) or n > 1 << 28:
            return
        yield data[p + 4:p + 8], data[p + 8:p + 8 + n]
        p += 12 + n


def extracted(data):
    """Independently recover the extractor's stream and segment presence rules."""
    idat = None
    frames = []
    for kind, payload in chunks(data):
        if kind == b'IDAT':
            if idat is None:
                idat = []
            idat.append(payload)
        elif kind == b'fcTL':
            frames.append(None)
        elif kind == b'fdAT' and frames and len(payload) >= 4:
            if frames[-1] is None:
                frames[-1] = []
            frames[-1].append(payload[4:])
    parts = ([] if idat is None else [idat]) + [f for f in frames if f is not None]
    streams = [b''.join(p) for p in parts]
    assert streams == V.zlib_streams(data)
    return list(zip(streams, parts))


def chunk(kind, payload):
    # The frozen extractor intentionally does not inspect CRC or signature.
    return struct.pack('>I', len(payload)) + kind + payload + bytes(4)


def extractor_tests():
    prefix = bytes(8)
    cases = [
        (prefix, []),
        (prefix + chunk(b'IDAT', b''), [b'']),
        (prefix + chunk(b'IDAT', b'') * 3, [b'']),
        (prefix + chunk(b'fcTL', b''), []),
        (prefix + chunk(b'fcTL', b'') + chunk(b'fdAT', bytes(4)), [b'']),
        (prefix + chunk(b'fcTL', b'') + chunk(b'fdAT', bytes(3)), []),
        (prefix + chunk(b'IDAT', b'a') + chunk(b'IDAT', b'') + chunk(b'IDAT', b'b'), [b'ab']),
        (prefix + chunk(b'fcTL', b'') + chunk(b'fdAT', bytes(4))
         + chunk(b'fdAT', bytes(4) + b'x') + chunk(b'fcTL', b''), [b'x']),
    ]
    for data, expected in cases:
        assert [s for s, _ in extracted(data)] == expected
    return len(cases)


def image_limit(data, stream_index):
    """Image R without asking the oracle's structural walk to accept the input."""
    ihdr = next((p for k, p in chunks(data) if k == b'IHDR' and len(p) == 13), None)
    if ihdr is None:
        return 8192, False
    width, height, depth, ct, _, _, interlace = struct.unpack('>IIBBBBB', ihdr)
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}.get(ct)
    if channels is None or depth not in (1, 2, 4, 8, 16) or interlace not in (0, 1):
        return 8192, False
    frames = []
    for kind, payload in chunks(data):
        if kind == b'fcTL':
            frames.append([None, False])
            if len(payload) >= 12:
                frames[-1][0] = struct.unpack('>II', payload[4:12])
        elif kind == b'fdAT' and frames and len(payload) >= 4:
            frames[-1][1] = True
    sizes = [(width, height)] if any(k == b'IDAT' for k, _ in chunks(data)) else []
    sizes += [f[0] or (width, height) for f in frames if f[1]]
    width, height = sizes[stream_index]
    passes = [(0, 0, 1, 1)] if interlace == 0 else [
        (0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4),
        (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)]
    size = 0
    for x, y, dx, dy in passes:
        pw = max(0, (width + dx - 1 - x) // dx)
        ph = max(0, (height + dy - 1 - y) // dy)
        if pw and ph:
            size += ph * (1 + (pw * channels * depth + 7) // 8)
    # Malformed IHDR and cap-boundary records may describe enormous rasters.
    # Their extractor calls remain real calls, with a bounded harness buffer;
    # the log separately counts these non-R calls, never credits them as gates.
    return min(size, 1 << 20), size <= 1 << 20


def vectors(runner):
    count = 0
    bounded = 0
    d4_seen = set()
    empty_z = 0
    authored = 0
    paths = [PNG / 'spec-frozen/VECTORS.json', PNG / 'limits-draft/VECTORS-CAPS.json',
             PNG / 'policy-draft/VECTORS-POLICY.json']
    for path in paths:
        records = json.loads(path.read_text())
        streams = 0
        for v in records:
            if 'pad_to' in v:
                continue  # Container-only cap tests have no inflate call.
            data = bytes.fromhex(v['input_hex'])
            for i, (s, parts) in enumerate(extracted(data)):
                streams += 1
                limit, exact_r = image_limit(data, i)
                bounded += not exact_r
                options = 'B' if v.get('profile') == 'RecommendedBeta' else 'S'
                expected = oracle(s, limit, options)
                layouts = [0, 1, 2, 'p:' + ','.join(str(len(p)) for p in parts)]
                for layout in layouts:
                    runner.run(s, limit, options, layout, expected, v['name'])
                # Authored first errors of the frozen inflate-only witnesses do
                # not depend on the Python oracle's expected-value calculation.
                if path == paths[0] and v['name'].startswith(('deflate/', 'zlib/', 'image/')):
                    error = v['expected'].get('error')
                    if error in INFLATE_ERRORS or error == 'ExcessImageData':
                        assert expected[0] == ('Full' if error == 'ExcessImageData' else error), v['name']
                        authored += 1
                if v['name'] in D4:
                    assert expected == (D4[v['name']], b'') or expected[0] == D4[v['name']]
                    d4_seen.add(v['name'])
                if v['name'] == 'zlib/empty-idat-only':
                    assert s == b'' and expected == ('TruncatedZlib', b'')
                    empty_z += 1
                count += 1
        if path == paths[0]:
            assert streams == 631, streams
        print(f'{path.name}: {len(records)} records, {streams} extracted streams; four layouts each', flush=True)
    assert d4_seen == set(D4) and empty_z == 1, (d4_seen, empty_z)
    print(f'authored inflate-error expectations checked: {authored}; D4 five, empty-Z one; '
          f'non-R bounded harness calls: {bounded}', flush=True)
    return count


def differential(runner, count, seed):
    rng = random.Random(seed)
    accepted_rule_differences = {}
    for i in range(count):
        n = rng.randrange(20000)
        alphabet = rng.randrange(1, 257)
        table = bytes(0 if x % 10 < 3 else x % alphabet for x in range(256))
        data = rng.randbytes(n).translate(table)
        if i % 7 == 0 and n:
            period = rng.randrange(1, min(n, 100) + 1)
            data = (data[:period] * ((n + period - 1) // period))[:n]
        c = zlib.compressobj(rng.randrange(10), 8, rng.randrange(9, 16), rng.randrange(1, 10), rng.randrange(5))
        zs = c.compress(data) + c.flush()
        runner.run(zs, len(data), layout=i % 3, expected=('Done', data), name=f'zlib-valid-{i}')
        k = rng.randrange(len(zs))
        bad = zs[:k] + bytes([zs[k] ^ (1 << rng.randrange(8))]) + zs[k + 1:]
        mine = runner.run(bad, max(len(data) + 4096, 65536), layout=i % 3, name=f'zlib-corrupt-{i}')
        ref = V.zlib_ref(bad)
        if isinstance(ref, tuple):
            raw, tail = ref
            if tail:
                assert mine[0] == 'TrailingZlibData', (i, mine[0], 'tail')
            elif len(raw) > max(len(data) + 4096, 65536):
                assert mine == ('Full', raw[:max(len(data) + 4096, 65536)]), i
            elif mine != ('Done', raw):
                assert mine[0] in V.EXPECTED_ZLIB_DIFFERENCES, (i, mine[0], len(raw))
                accepted_rule_differences[mine[0]] = accepted_rule_differences.get(mine[0], 0) + 1
        else:
            assert mine[0] != 'Done', (i, mine[0], ref)
        # Exact prefix and first-error comparison on every corruption, not just
        # accept/reject agreement with stock zlib's different error vocabulary.
        assert mine == oracle(bad, max(len(data) + 4096, 65536)), ('corrupt prefix', i)
        if i < 1000:
            limit = rng.randrange(len(data) + 1)
            expected = ('Done', data) if limit == len(data) else ('Full', data[:limit])
            runner.run(zs, limit, layout=i % 3, expected=expected, name=f'limit-{i}')
            cut = rng.randrange(len(zs) + 1)
            runner.run(zs[:cut], len(data), layout=i % 3,
                       expected=oracle(zs[:cut], len(data)), name=f'cut-{i}')
        if (i + 1) % 1000 == 0:
            print(f'differential {i + 1}/{count} streams + corruptions', flush=True)
    print(f'Python zlib differential: {count} generated streams + {count} single-bit corruptions; '
          f'0 unexpected differences, allowed={accepted_rule_differences}', flush=True)


def ring_stress(runner):
    for data in (b'a' * 200000, bytes(range(256)) * 1000,
                 random.Random(2).randbytes(40000) * 3):
        for strategy in (zlib.Z_DEFAULT_STRATEGY, zlib.Z_FIXED):
            c = zlib.compressobj(9, zlib.DEFLATED, 15, 8, strategy)
            zs = c.compress(data) + c.flush()
            for limit in (0, 1, 32767, 32768, 32769, len(data) - 1, len(data), len(data) + 1):
                expected = ('Full', data[:limit]) if limit < len(data) else ('Done', data)
                runner.run(zs, limit, layout=2, expected=expected, name='ring-wrap-match')


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--fuzz', type=int, default=10000)
    p.add_argument('--seed', type=int, default=1)
    p.add_argument('--binary', type=Path, default=ROOT / 'target/release/examples/inflate_runner')
    a = p.parse_args()
    assert a.fuzz >= 0
    print(f'extractor presence cases: {extractor_tests()}', flush=True)
    runner = Runner(a.binary)
    try:
        total = vectors(runner)
        ring_stress(runner)
        differential(runner, a.fuzz, a.seed)
        print(f'PASS: {total} vector streams, {runner.calls} public-entry pairs; '
              'each pair checks status/count, prefix/suffix, ring, observed work', flush=True)
    finally:
        runner.close()

if __name__ == '__main__':
    main()
