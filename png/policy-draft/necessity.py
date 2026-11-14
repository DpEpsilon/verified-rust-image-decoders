#!/usr/bin/env python3
"""One-field-at-a-time necessity check (as ../../gif/policy-draft/necessity.py does for GIF).

For each named profile and each field whose value differs from Strict, revert that field to Strict and
count the inputs on which the profile agreed with its incumbent runner and the reverted profile no
longer does. Inputs: the union of the fitted set, the inputs above 4 KiB and the held-out sets A and B
(the same agreement rule and cap exclusions as validate.py). A field that breaks nothing has no measured
support on these inputs; it may still be kept because the library's source does it (see OPTIONS.md).
Pairs: GdkCompat-2.42.12 still vs gdk-loader; ImageCrateCompat-0.25.10 still vs image-still and frames vs
image; RecommendedBeta still vs image-still and vs gdk-loader (an input counts once if either breaks).
Usage: python3 -I necessity.py
"""
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import policy_oracle as PO  # noqa: E402
import validate as V  # noqa: E402

PAIRS = {'GdkCompat-2.42.12': (('still', 'gdk-loader'),),
         'ImageCrateCompat-0.25.10': (('still', 'image-still'), ('frames', 'image')),
         'RecommendedBeta': (('still', 'image-still'), ('still', 'gdk-loader'))}


def rows():
    out = []
    for r in V.RESULTS:
        out.append((r['name'], V.RP.resolve(r), r['runners']))
    for data in (V.HELDOUT, V.HELDOUT_B):
        for r in data or []:
            out.append((r['name'], V.RP.resolve(r), r['runners']))
    return out


def agrees(b, P, kind, theirs):
    if kind == 'still':
        mine = V.still_key(PO.decode_still(b, P))
        return mine == V.runner_key(theirs) if mine[0] == 'ok' else theirs['status'] != 'ok'
    mine = V.frames_key(PO.decode_frames(b, P))
    t = ('err',) if theirs['status'] != 'ok' else ('ok', int(theirs.get('frames', 1)), theirs.get('seq'))
    return mine == t if mine[0] == 'ok' else t[0] == 'err'


def main():
    R = rows()
    usable = []
    for name, b, run in R:
        e = V.O.decode_frames(b).get('error') or V.O.decode_still(b).get('error')
        if e not in V.CAPS:
            usable.append((name, b, run))
    print(f'{len(usable)} inputs (fitted, above 4 KiB, held-out A and B; {len(R) - len(usable)} beyond a P1 cap excluded)')
    for prof, pairs in PAIRS.items():
        P = PO.PROFILES[prof]
        base = {}
        for name, b, run in usable:
            for kind, rn in pairs:
                v = run.get(rn)
                if v is not None and v['status'] not in ('crash', 'timeout'):
                    base[(name, kind, rn)] = agrees(b, P, kind, v)
        print(f'{prof}: agrees on {sum(base.values())} of {len(base)} (input, runner) pairs')
        for field, val in P.items():
            if val == PO.STRICT.get(field, 'On'):
                continue
            Q = dict(P, **{field: PO.STRICT.get(field, 'On')})
            broke, fixed = set(), set()
            for name, b, run in usable:
                for kind, rn in pairs:
                    k = (name, kind, rn)
                    if k not in base:
                        continue
                    now = agrees(b, Q, kind, run[rn])
                    if base[k] and not now:
                        broke.add(name)
                    if not base[k] and now:
                        fixed.add(name)
            ex = ', '.join(sorted(broke)[:4])
            print(f'  {field}={val} -> {PO.STRICT.get(field, "On")}: breaks {len(broke)}'
                  + (f' (e.g. {ex})' if broke else '') + (f'; would fix {len(fixed)}: {", ".join(sorted(fixed))}' if fixed else ''))


if __name__ == '__main__':
    main()
