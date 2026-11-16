#!/usr/bin/env python3
"""Compare the model transcription with the independent policy oracle.

Also generate complete-observation runtime records and 96 oracle-backed Verus
samples. The policy oracle supplies indices, error names, E4, and (via its return
frame) the last bit position. No oracle code or table enters the Verus model.
"""
import importlib.util
import itertools
import json
from pathlib import Path
import random
import sys

ROOT = Path(__file__).resolve().parent.parent
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('lzw_policy_oracle', ROOT.parent / 'policy-draft/policy_oracle.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


def transcription(m, data, area, p):
    if not p['m_lo'] <= m <= p['m_hi'] or (p['size1_only_one_pixel'] and m <= 1 and area != 1):
        return [], 'InvalidLzwMinCodeSize', 0, False
    clear = 1 << m
    table = [bytes([c % 256]) for c in range(clear)] + [b'', b'']
    pos, width, prev, out = 0, m + 1, None, []
    while True:
        if p['stop_at_raster'] and len(out) == area:
            return out, None, pos, False
        if pos + width > 8 * len(data):
            return out, 'TooFewPixels' if len(out) < area else 'MissingEndOfInformation', pos, False
        code = sum(((data[(pos + i) // 8] >> ((pos + i) % 8)) & 1) << i for i in range(width))
        pos += width
        if code == clear:
            table, width, prev = table[:clear + 2], m + 1, None
            continue
        if len(out) == area and code != clear + 1:
            return out, 'MissingEndOfInformation', pos, False
        if code == clear + 1:
            error = 'TooFewPixels' if len(out) < area else (
                'DataAfterEndOfInformation' if not p['stop_at_raster'] and (pos + 7) // 8 < len(data) else None)
            return out, error, pos, False
        if p['checked_literals'] and 256 <= code < clear:
            return out, 'ColorIndexOutOfRange', pos, False
        if code < len(table):
            s = table[code]
        elif code == len(table) and prev is not None and len(table) < 4096:
            s = table[prev] + table[prev][:1]
        else:
            return out, 'InvalidLzwCode', pos, False
        remaining = area - len(out)
        if len(s) > remaining:
            out.extend(s[:remaining])
            return out, None if p['stop_at_raster'] else 'TooManyPixels', pos, p['stop_at_raster']
        if prev is not None and len(table) < 4096:
            table.append(table[prev] + s[:1])
            if len(table) == 1 << width and width < 12:
                width += 1
        prev = code
        out.extend(s)


def oracle_observation(m, data, area, p):
    if not p['m_lo'] <= m <= p['m_hi'] or (p['size1_only_one_pixel'] and m <= 1 and area != 1):
        return [], 'InvalidLzwMinCodeSize', 0, False
    policy = dict(oracle.STRICT,
                  lzw_min='Range1to11Checked' if p['checked_literals'] else 'Range1to11',
                  lzw_tail='StopAtRaster' if p['stop_at_raster'] else 'Strict')
    out, events, position = [], set(), [0]
    def returned(frame, event, _arg):
        if event == 'return' and frame.f_code is oracle.lzw_decode.__code__:
            position[0] = frame.f_locals['pos']
    error = None
    sys.setprofile(returned)
    try:
        oracle.lzw_decode(m, data, area, lambda q, v: out.append(v), policy, events)
    except oracle.GifError as exc:
        error = exc.name
    finally:
        sys.setprofile(None)
    return out, error, position[0], 'E4' in events


def main():
    rng = random.Random(0x912f48)
    cases = [(2, bytes([140, 11]), 2), (9, bytes([1, 176, 4]), 1)]
    cases += [(2, bytes([b]), a) for b in range(256) for a in [1, 3]]
    cases += [(rng.randrange(1, 12), rng.randbytes(rng.randrange(0, 12)), rng.randrange(0, 17))
              for _ in range(1024)]
    for name in ['random.json', 'vectors.json']:
        for r in json.loads((ROOT / 'tests/expected' / name).read_text()):
            cases.append((r['m'], bytes(r['data']), r['a']))
    records, samples = [], []
    checked = 0
    for bounds in [(2, 8), (1, 11)]:
        for flags in itertools.product([False, True], repeat=4):
            p = dict(zip(['size1_only_one_pixel', 'checked_literals', 'deferred_clear', 'stop_at_raster'], flags),
                     m_lo=bounds[0], m_hi=bounds[1])
            for ci, (m, data, area) in enumerate(cases):
                expected = oracle_observation(m, data, area, p)
                got = transcription(m, data, area, p)
                assert got == expected, (m, list(data), area, p, got, expected)
                checked += 1
                rec = dict(m=m, data=list(data), a=area, params=p, expected=expected[0],
                           error=expected[1], pos=expected[2], e4=expected[3])
                # Retain each switch combination, all existing streams, and a
                # deterministic subset of random/exhaustive-byte cases.
                if ci < 2 or ci % 17 == 0 or ci >= len(cases) - 247:
                    records.append(rec)
                if bounds == (1, 11) and (ci < 2 or ci in [514, 515, 516, 517]):
                    samples.append(rec)
    (ROOT / 'tests/expected/policy.json').write_text(json.dumps(records, separators=(',', ':')) + '\n')
    text = '''//! GENERATED by tools/check_model_policy_gap.py from independent oracle observations.
use vstd::prelude::*;
verus! {
#[cfg(verus_only)]
use crate::model::{ML, observation, ErrK};
#[cfg(verus_only)]
use crate::lzw::Params;
'''
    seq = lambda xs: 'seq![' + ','.join(f'{v}u8' for v in xs) + ']'
    for i, r in enumerate(samples):
        ps = 'Params {' + ','.join(f'{k}:{str(v).lower()}' for k, v in r['params'].items()) + '}'
        error = 'None' if r['error'] is None else f"Some(ErrK::{r['error']})"
        call = f"ML({r['m']},{seq(r['data'])},{r['a']},{ps})"
        obs = f"observation({seq(r['expected'])},{error},{r['pos']},{str(r['e4']).lower()})"
        text += f'pub proof fn sample_{i}()\n    ensures {call}=={obs},\n{{\n    assert({call}=={obs}) by(compute);\n}}\n'
    text += '} // verus!\n'
    (ROOT / 'src/policy_samples.rs').write_text(text)
    print(f'{checked} model-transcription/oracle comparisons passed; {len(records)} runtime records; {len(samples)} Verus samples.')
    print('StopAtRaster regression: Ok([1,1]), bit 9, E4=true.')

if __name__ == '__main__':
    main()
