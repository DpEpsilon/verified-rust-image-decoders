#!/usr/bin/env python3
"""Differential inputs: mutate the VECTORS.json and probe inputs (byte flips, truncations,
insertions, deletions, field tweaks) and write records with the oracles' expectations in
the VECTORS.json schema (Strict still/frames via gif_oracle.py, RecommendedBeta still/frames
via policy_oracle.py). Run them with
    GIF_RECORDS=OUT cargo test --release --test conformance extra_records_from_env

Usage: python3 -I tools/gen_mutants.py OUT [COUNT] [SEED]
"""
import json
import random
import sys
from pathlib import Path

CRATE = Path(__file__).resolve().parent.parent
GIF = CRATE.parent
sys.path.insert(0, str(GIF / 'spec-draft'))
sys.path.insert(0, str(GIF / 'policy-draft'))
import check_vectors as CV  # noqa: E402


def seeds():
    out = []
    for v in json.loads((GIF / 'spec-draft/VECTORS.json').read_text()):
        if 'pad_to' in v or 'repeat' in v:
            continue
        out.append(bytes.fromhex(v['input_hex']))
    p = CRATE / 'tests/data/PROBES.json'
    if p.exists():
        for v in json.loads(p.read_text()):
            out.append(bytes.fromhex(v['input_hex']))
    return sorted(set(out))


def mutate(rng, b):
    b = bytearray(b)
    for _ in range(rng.choice((1, 1, 1, 2, 3))):
        if not b:
            break
        k = rng.randrange(7)
        i = rng.randrange(len(b))
        if k == 0:
            b[i] ^= 1 << rng.randrange(8)
        elif k == 1:
            b[i] = rng.choice((0, 1, 2, 3, 4, 7, 8, 9, 11, 12, 0x21, 0x2C, 0x3B, 0x7F, 0x80, 0xF9, 0xFE, 0xFF, rng.randrange(256)))
        elif k == 2:
            del b[i:]
        elif k == 3:
            del b[i:i + rng.randrange(1, 8)]
        elif k == 4:
            b[i:i] = bytes(rng.randrange(256) for _ in range(rng.randrange(1, 6)))
        elif k == 5:
            j = rng.randrange(len(b))
            n = rng.randrange(1, 32)
            b[i:i] = b[j:j + n]
        else:
            b[i] = (b[i] + rng.choice((-1, 1))) & 255
    return bytes(b)


def main():
    out = Path(sys.argv[1])
    count = int(sys.argv[2]) if len(sys.argv) > 2 else 5000
    rng = random.Random(int(sys.argv[3]) if len(sys.argv) > 3 else 1)
    base = seeds()
    seen, recs = set(), []
    while len(seen) < count:
        b = mutate(rng, rng.choice(base))
        if b in seen:
            continue
        seen.add(b)
        for entry in ('still', 'frames'):
            for prof in (None, 'RecommendedBeta'):
                v = {'name': f'mut{len(seen)}' + ('/frames' if entry == 'frames' else '') + (f'@{prof}' if prof else ''),
                     'entry': entry, 'input_hex': b.hex(), 'expected': {}}
                if prof:
                    v['profile'] = prof
                v['expected'] = CV.actual(v)
                recs.append(v)
    out.write_text(json.dumps(recs))
    print(f'{len(recs)} records from {count} mutated inputs -> {out}')


if __name__ == '__main__':
    main()
