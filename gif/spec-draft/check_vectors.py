#!/usr/bin/env python3
"""Run gif_oracle.py against every VECTORS.json record (SPEC.md G17 schema); records
carrying a `profile` run ../policy-draft/policy_oracle.py with that named profile.
Usage: python3 -I check_vectors.py [VECTORS.json]"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / 'policy-draft'))
import gif_oracle as O  # noqa: E402
import policy_oracle as PO  # noqa: E402


def runs(rgba):
    out = []
    for i in range(0, len(rgba), 4):
        h = rgba[i:i + 4].hex()
        if out and out[-1]['hex'] == h:
            out[-1]['count'] += 1
        else:
            out.append({'count': 1, 'hex': h})
    return out


def materialize(v):
    b = bytes.fromhex(v['input_hex'])
    if 'repeat' in v:
        b = b + bytes.fromhex(v['repeat_hex']) * v['repeat'] + bytes.fromhex(v['suffix_hex'])
    if 'pad_to' in v:
        b = b + bytes(v['pad_to'] - len(b))
    return b


def plays_json(p):
    return p[0] if p[0] != 'Repeats' else {'Repeats': p[1]}


def actual(v):
    b = materialize(v)
    prof = v.get('profile')
    P = PO.PROFILES[prof] if prof else None
    if v['entry'] == 'plays':
        r = O.decode_frames(b)
        if 'error' in r:
            return {'error': r['error']}
        p = O.plays(r['loop_count'])
        return {'loop_count': r['loop_count'], 'plays': plays_json(p), 'total_plays': O.total_plays(p)}
    if v['entry'] == 'still':
        r = PO.decode_still(b, P) if P else O.decode_still(b)
        if 'error' in r:
            return {'error': r['error']}
        out = {'width': r['width'], 'height': r['height'], 'rgba_runs': runs(r['rgba'])}
        if P:
            out['recovered'] = r['recovered']
        return out
    r = PO.decode_frames(b, P) if P else O.decode_frames(b)
    if 'error' in r:
        return {'error': r['error']}
    if v['expected'].get('summary'):
        return {'summary': True, 'width': r['width'], 'height': r['height'], 'frame_count': len(r['frames']),
                'last_rgba_runs': runs(r['frames'][-1]['rgba']), 'end': r['end']}
    frames = [{'rect': list(f['rect']), 'delay_cs': f['delay_cs'], 'disposal': f['disposal'],
               'user_input': f['user_input'], 'transparent': f['transparent'],
               'interlaced': f['interlaced'], 'rgba_runs': runs(f['rgba'])} for f in r['frames']]
    if P:
        for x, f in zip(frames, r['frames']):
            x['recovered'] = f['recovered']
    return {'width': r['width'], 'height': r['height'], 'loop_count': r['loop_count'],
            'frames': frames, 'end': r['end']}


def main():
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / 'VECTORS.json'
    vs = json.loads(path.read_text())
    names = [v['name'] for v in vs]
    assert len(names) == len(set(names)), 'duplicate vector names'
    bad = 0
    for v in vs:
        a = actual(v)
        if a != v['expected']:
            bad += 1
            print('MISMATCH', v['name'], json.dumps(a)[:300], 'expected', json.dumps(v['expected'])[:300])
    seen = set()
    for v in vs:
        if v.get('profile'):
            continue                        # coverage of the core's error list counts Strict records only
        e = v['expected']
        if 'error' in e:
            seen.add(e['error'])
        elif isinstance(e.get('end'), dict):
            seen.add(e['end']['error'])
    errs = sorted(seen)
    unused = sorted(set(O.ERRORS) - set(errs))
    prof = sum(1 for v in vs if v.get('profile'))
    print(f'{len(vs) - bad}/{len(vs)} vectors agree ({len(vs) - prof} Strict, {prof} profile records); '
          f'errors exercised by Strict records {len(errs)}/{len(O.ERRORS)}; unexercised: {unused}')
    return 1 if bad else 0


if __name__ == '__main__':
    sys.exit(main())
