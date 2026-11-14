#!/usr/bin/env python3
"""Gap 4: what image 0.25.10 / png 0.18.1 (and glycin's png revision) return for damaged iCCP chunks,
and whether image::load_from_memory's default limits stop the 65,535 x 65,535 probe.

Every runner process runs under an address-space limit of 3,000,000 KiB (`ulimit -v 3000000`).
Writes probes/results-icc.json and prints one line per input and runner.
Usage: python3 -I icc_probes.py
"""
import json
import sys
import zlib
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent.parent / 'spec-draft'))
import run_probes as RP  # noqa: E402
from probes_def import PROBES, RAMPB, RAMPD  # noqa: E402
from gen_vectors import SIG, IEND, chunk, ihdr, z  # noqa: E402
import gen_vectors as G  # noqa: E402

AS_KIB = 3000000
PZ = z(G.ICC)


def zbomb(n):
    """zlib stream of n zero bytes, built in 1 MiB pieces (never holds n bytes)."""
    c = zlib.compressobj(9)
    piece = bytes(1 << 20)
    return b''.join(c.compress(piece) for _ in range(n >> 20)) + c.flush()


def icc(payload):
    return RAMPB + chunk(b'iCCP', payload) + RAMPD + IEND


EXTRA = [
    ('iccp-name-empty', icc(b'\0\0' + PZ), 'profile name of length 0'),
    ('iccp-name-79', icc(b'n' * 79 + b'\0\0' + PZ), 'profile name of 79 bytes (the maximum)'),
    ('iccp-name-80', icc(b'n' * 80 + b'\0\0' + PZ), 'profile name of 80 bytes (one too long)'),
    ('iccp-name-leading-space', icc(b' n\0\0' + PZ), 'profile name with a leading space'),
    ('iccp-name-control-char', icc(b'n\x01\0\0' + PZ), 'profile name with a control character'),
    ('iccp-no-nul', icc(b'n' * 20), 'no NUL after the name'),
    ('iccp-no-method', icc(b'n\0'), 'name, NUL and nothing else'),
    ('iccp-truncated-zlib', icc(b'n\0\0' + PZ[:len(PZ) // 2]), 'zlib stream cut in half'),
    ('iccp-bad-adler', icc(b'n\0\0' + PZ[:-1] + bytes([PZ[-1] ^ 1])), 'profile zlib stream with a wrong Adler-32'),
    ('iccp-trailing-after-zlib', icc(b'n\0\0' + PZ + b'junk'), 'four bytes after the zlib stream'),
    ('iccp-empty-profile', icc(b'n\0\0' + z(b'')), 'profile inflating to 0 bytes'),
    ('iccp-bomb-1g', icc(b'n\0\0' + zbomb(1 << 30)), 'profile inflating to 1 GiB (overlong)'),
]
NAMES = ('iccp-valid', 'iccp-bad-zlib', 'iccp-method-1', 'iccp-bomb-100m', 'dims-65535-square-tiny-data')
RUNNERS = {'image-icc': (RP.RUST, ['image-icc']), 'pngcrate': (RP.RUST, ['pngcrate']), 'glycin': (RP.GLYCIN, ['glycin']),
           'image-load': (RP.RUST, ['image-load']), 'image-still': (RP.RUST, ['image-still']),
           'gdk-loader': (RP.BIN / 'gdk-anim', ['png', 'forced-whole'])}


def main():
    items = [(n, b, w) for n, b, w in PROBES if n in NAMES] + EXTRA
    RP.WORK.mkdir(parents=True, exist_ok=True)
    out = []
    for name, b, why in items:
        p = RP.WORK / f'icc-{name}.png'
        p.write_bytes(b)
        res = {}
        for r, spec in RUNNERS.items():
            o = RP.invoke(p, spec, 60, AS_KIB)
            res[r] = {k: o.get(k) for k in ('status', 'icc', 'still', 'error', 'error_class', 'message', 'w', 'h', 'sha', 'info') if o.get(k) is not None}
            if r == 'pngcrate' and isinstance(o.get('info'), dict):
                res[r] = {'status': o['status'], 'iccp_len': o['info'].get('iccp')}
        p.unlink()
        out.append({'name': 'icc:' + name, 'why': why, 'len': len(b), 'runners': res})
        print(name, len(b), json.dumps(res)[:600])
    (HERE / 'results-icc.json').write_text(json.dumps(out, indent=0) + '\n')


if __name__ == '__main__':
    main()
