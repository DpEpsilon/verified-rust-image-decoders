#!/usr/bin/env python3
"""For each named compatibility profile, revert one field at a time to its Strict value and
list the probe/witness inputs whose modelled outcome then disagrees with the runner.
Usage: python3 -I necessity.py"""
import sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import policy_oracle as PO  # noqa: E402
import validate as V  # noqa: E402

for prof, runners in (('GdkCompat-2.42.12', ('gdk-loader', 'gdk-anim')), ('ImageCrateCompat-0.25.10', ('image',)),
                      ('Recommended', ())):
    P = PO.PROFILES[prof]
    for field, val in P.items():
        if val == PO.STRICT[field]:
            continue
        Q = dict(P, **{field: PO.STRICT[field]})
        broke = []
        for r in V.RESULTS:
            if r['hex'] is None:
                continue
            b = bytes.fromhex(r['hex'])
            for rn in runners:
                good = V.model_outcome(b, P, rn) == V.runner_outcome(r['runners'][rn], rn)
                if good and V.model_outcome(b, Q, rn) != V.runner_outcome(r['runners'][rn], rn):
                    broke.append(f'{r["name"]}@{rn}')
        print(f'{prof}: {field}={val} -> {PO.STRICT[field]} breaks {len(broke)}: {", ".join(sorted(set(broke))[:6])}')
