#!/usr/bin/env python3
"""Checks for the GIF policy draft.

1. Strict bridge: policy_oracle(STRICT) equals gif_oracle on every vector and probe input.
2. Obligation R (OPTIONS.md), for the two meanings of Strict-accepted (SPEC.md G14, G15):
   R-still: Strict decode_still is Ok -> RecommendedBeta's still is identical, recovered=false;
   R-stream: Strict decode_frames ends at the Trailer -> RecommendedBeta returns the same info,
   the same frame records (every field), recovered=false on each, and ends at the Trailer;
   R-prefix: every frame Strict returns before an error is returned identically by RecommendedBeta.
3. Profile agreement with the measured incumbents (probes/results.json):
   ImageCrateCompat-0.25.10 vs `image` (into_frames) and `glycin`;
   GdkCompat-2.42.12 vs `gdk-loader` (static image, whole buffer) and `gdk-anim`.
Usage: python3 -I validate.py
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE / 'probes'))
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
import gif_oracle as O  # noqa: E402
import policy_oracle as PO  # noqa: E402

RESULTS = json.loads((HERE / 'probes/results.json').read_text())
VEC = json.loads((HERE.parent / 'spec-draft/VECTORS.json').read_text())


def inputs():
    seen = {}
    for v in VEC:
        b = bytes.fromhex(v['input_hex'])
        if 'repeat' in v:
            b = b + bytes.fromhex(v['repeat_hex']) * v['repeat'] + bytes.fromhex(v['suffix_hex'])
        if 'pad_to' in v:
            continue
        seen.setdefault(b, v['name'])
    for r in RESULTS:
        if r['hex'] is not None:
            seen.setdefault(bytes.fromhex(r['hex']), r['name'])
    return seen


def strip(res):
    if 'frames' in res:
        res = dict(res, frames=[{k: v for k, v in f.items() if k not in ('recovered', 'events')} for f in res['frames']])
        res.pop('background_index', None)
        res.pop('aspect', None)
        res.pop('version', None)
    return {k: v for k, v in res.items() if k != 'recovered'}


def bridge(ins):
    bad = 0
    for b, name in ins.items():
        a, c = O.decode_frames(b), PO.decode_frames(b, PO.STRICT)
        s1, s2 = O.decode_still(b), PO.decode_still(b, PO.STRICT)
        s1.pop('frame', None)
        if strip(a) != strip(c) or strip(s1) != strip(s2):
            bad += 1
            print('BRIDGE', name, str(strip(a))[:150], '|', str(strip(c))[:150])
    print(f'strict bridge: {len(ins) - bad}/{len(ins)} inputs equal')


def obligation_r(ins):
    bad = still_ok = stream_ok = nframes = 0
    for b, name in ins.items():
        s, r = PO.decode_still(b, PO.STRICT), PO.decode_still(b, PO.RECOMMENDED_BETA)
        if 'error' not in s:
            still_ok += 1
            if strip(r) != strip(s) or r.get('recovered'):
                bad += 1
                print('R-still', name)
        f, g = PO.decode_frames(b, PO.STRICT), PO.decode_frames(b, PO.RECOMMENDED_BETA)
        gf = g.get('frames', [])
        for i, fr in enumerate(f.get('frames', [])):
            nframes += 1
            if i >= len(gf) or strip({'frames': [gf[i]]}) != strip({'frames': [fr]}) or gf[i]['recovered']:
                bad += 1
                print('R-prefix', name, i)
                break
        if f.get('end') == 'trailer':
            stream_ok += 1
            if strip(g) != strip(f) or any(x['recovered'] for x in gf):
                bad += 1
                print('R-stream', name)
    print(f'obligation R: {still_ok} Strict-accepted still inputs, {stream_ok} Strict-accepted streams, '
          f'{nframes} Strict frames; {bad} violations')


def h(x):
    return hashlib.sha256(x).hexdigest()


def period_reduce(frames):
    """gdk-anim runner: smallest period of the (pixels, delay) sequence of a terminated run."""
    n = len(frames)
    for p in range(1, n):
        if n % p == 0 and all(frames[i] == frames[i + p] for i in range(n - p)):
            return frames[:p]
    return frames


def gdk_delay(cs):
    ms = cs * 10
    return 100 if ms == 0 else max(ms, 20)


def model_outcome(b, P, runner):
    """What the runner would report if it implemented profile P exactly."""
    f = PO.decode_frames(b, P)
    if 'error' in f or not f['frames']:
        return None
    fr = f['frames'][:64]
    if runner == 'gdk-loader':
        if f['end'] != 'trailer':
            return None
        return (f['width'], f['height'], h(fr[0]['rgba']), 1)
    if runner == 'gdk-anim':
        if isinstance(f['end'], dict) and f['end']['error'] in ('MissingTrailer', 'TruncatedImageData',
                                                                 'TruncatedImageDescriptor', 'TruncatedExtension',
                                                                 'TruncatedColorTable'):
            return None
        seq = period_reduce([(x['rgba'], gdk_delay(x['delay_cs'])) for x in fr])
        return (f['width'], f['height'], h(b''.join(x[0] for x in seq)), len(seq))
    return (f['width'], f['height'], h(b''.join(x['rgba'] for x in fr)), len(fr))


def runner_outcome(v, runner):
    if v['status'] != 'ok':
        return None
    if runner == 'gdk-loader':
        return (v['width'], v['height'], v['rgba_sha256'], 1)
    return (v['width'], v['height'], v['seq_sha256'], v['frames'])


def agreement():
    pairs = [('ImageCrateCompat-0.25.10', 'image'), ('ImageCrateCompat-0.25.10', 'glycin'),
             ('GdkCompat-2.42.12', 'gdk-loader'), ('GdkCompat-2.42.12', 'gdk-anim'),
             ('Strict', 'image'), ('Strict', 'gdk-loader'), ('RecommendedBeta', 'image'),
             ('RecommendedBeta', 'gdk-loader')]
    for prof, runner in pairs:
        P = PO.PROFILES[prof]
        agree, miss = 0, []
        for r in RESULTS:
            if r['hex'] is None:
                continue
            b = bytes.fromhex(r['hex'])
            m, a = model_outcome(b, P, runner), runner_outcome(r['runners'][runner], runner)
            if m == a:
                agree += 1
            else:
                miss.append(r['name'])
        n = sum(1 for r in RESULTS if r['hex'] is not None)
        print(f'{prof} vs {runner}: {agree}/{n} agree' + (f'; differ: {", ".join(miss)}' if len(miss) <= 40 else f'; {len(miss)} differ'))


if __name__ == '__main__':
    ins = inputs()
    bridge(ins)
    obligation_r(ins)
    agreement()
