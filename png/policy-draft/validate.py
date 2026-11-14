#!/usr/bin/env python3
"""Checks for the PNG policy draft.

1. Strict bridge: policy_oracle(STRICT) equals png_oracle on every vector, probe and witness input
   (still and frames, including the first error).
2. Obligation R: wherever Strict accepts (still: Ok; frames: every frame Strict returns), RecommendedBeta
   returns the same bytes with recovered=false.
3. Profile agreement with the measured incumbents (probes/results.json):
   ImageCrateCompat-0.25.10: still vs `image-still`, frames vs `image` (APNG frame loop);
   GdkCompat-2.42.12: still vs `gdk-loader` (GdkPixbufLoader, whole buffer), also reported vs `gdk-file`.
   An input counts as agreeing when both reject, or both accept with equal dimensions and SHA-256.
Usage: python3 -I validate.py [--list]
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
import png_oracle as O  # noqa: E402
import policy_oracle as PO  # noqa: E402

RESULTS = json.loads((HERE / 'probes/results.json').read_text())
VEC = json.loads((HERE.parent / 'spec-draft/VECTORS.json').read_text())
SECOND_ROUND = {'apng-bg-then-previous-4', 'apng-previous-chain', 'idat-then-4-bytes', 'idat-then-iend-no-crc',
                'crc-extra-idat-after-stream', 'unknown-critical-after-idat', 'crc-text-after-idat-image',
                'trns-rgb16-lowbyte', 'index-oob-2bit-trns'}


def inputs():
    seen = {}
    for v in VEC:
        if 'pad_to' in v:
            continue
        seen.setdefault(bytes.fromhex(v['input_hex']), 'vec:' + v['name'])
    by_sha = {r['sha256']: r for r in RESULTS}
    for r in RESULTS:
        if r['hex'] is not None:
            seen.setdefault(bytes.fromhex(r['hex']), r['name'])
    return seen, by_sha


def strip_frames(r):
    if 'error' in r:
        return r
    return dict(r, frames=[{k: v for k, v in f.items() if k not in ('recovered', 'events')} for f in r['frames']])


def still_key(r):
    return ('err',) if 'error' in r else ('ok', r['width'], r['height'], hashlib.sha256(r['rgba']).hexdigest())


def runner_key(v):
    return ('err',) if v['status'] != 'ok' else ('ok', v.get('w'), v.get('h'), v.get('sha'))


def frames_key(r):
    if 'error' in r:
        return ('err',)
    return ('ok', len(r['frames']), hashlib.sha256(b''.join(f['rgba'] for f in r['frames'])).hexdigest())


def main():
    seen, by_sha = inputs()
    listing = '--list' in sys.argv
    # 1. Strict bridge and 2. obligation R
    bridge_bad = r_bad = r_checked = 0
    for b, name in seen.items():
        s0, s1 = O.decode_still(b), PO.decode_still(b, PO.STRICT)
        a = {k: v for k, v in s0.items() if k != 'info'}
        c = {k: v for k, v in s1.items() if k not in ('recovered', 'events')}
        if a != c or s1.get('recovered'):
            bridge_bad += 1
            print('BRIDGE still', name, str(a)[:120], str(c)[:120])
        f0, f1 = O.decode_frames(b), PO.decode_frames(b, PO.STRICT)
        if f0 != strip_frames(f1) or any(f.get('recovered') for f in f1.get('frames', [])):
            bridge_bad += 1
            print('BRIDGE frames', name, str(f0)[:160], str(strip_frames(f1))[:160])
        if 'error' not in s0:
            r_checked += 1
            r = PO.decode_still(b, PO.RECOMMENDED_BETA)
            if 'error' in r or r['rgba'] != s0['rgba'] or r['recovered']:
                r_bad += 1
                print('OBLIGATION-R still', name, r.get('error'), r.get('events'))
        if 'error' not in f0:
            r2 = PO.decode_frames(b, PO.RECOMMENDED_BETA)
            for k, f in enumerate(f0['frames']):
                if 'error' in r2 or k >= len(r2['frames']) or r2['frames'][k]['rgba'] != f['rgba'] or r2['frames'][k]['recovered']:
                    r_bad += 1
                    print('OBLIGATION-R frames', name, k)
                    break
    print(f'Strict bridge: {len(seen)} inputs, {bridge_bad} differences. '
          f'Obligation R: {r_checked} Strict-accepted still inputs and every Strict frame, {r_bad} violations.')
    # 3. agreement
    rows = []
    for r in RESULTS:
        if r['hex'] is None:
            continue
        b = bytes.fromhex(r['hex'])
        rows.append((r['name'], b, r['runners']))
    stats = {}

    CAPS = {'FileTooLarge', 'InvalidDimensions', 'OutputTooLarge', 'TooManyFrames', 'PixelBudgetExceeded'}

    def agree(label, mine, theirs, name):
        st = stats.setdefault(label, [0, 0, [], 0])
        if theirs['status'] in ('crash', 'timeout') or name in beyond_caps:
            st[3] += 1
            return
        ok = mine == runner_key(theirs) if mine[0] == 'ok' else theirs['status'] != 'ok'
        st[0 if ok else 1] += 1
        if not ok:
            st[2].append(name)

    beyond_caps = set()
    for name, b, run in rows:
        e = O.decode_frames(b).get('error') or O.decode_still(b).get('error')
        if e in CAPS:
            beyond_caps.add(name)
    for name, b, run in rows:
        img_s = still_key(PO.decode_still(b, PO.IMAGE))
        agree('ImageCrateCompat still vs image-still', img_s, run['image-still'], name)
        img_f = frames_key(PO.decode_frames(b, PO.IMAGE))
        v = run['image']
        theirs = ('err',) if v['status'] != 'ok' else ('ok', int(v.get('frames', 1)), v.get('seq'))
        st = stats.setdefault('ImageCrateCompat frames vs image', [0, 0, [], 0])
        if v['status'] in ('crash', 'timeout') or name in beyond_caps:
            st[3] += 1
        else:
            ok = (img_f == theirs) if img_f[0] == 'ok' else theirs[0] == 'err'
            st[0 if ok else 1] += 1
            if not ok:
                st[2].append(name)
        gdk_s = still_key(PO.decode_still(b, PO.GDK))
        agree('GdkCompat still vs gdk-loader', gdk_s, run['gdk-loader'], name)
        agree('GdkCompat still vs gdk-file (not targeted)', gdk_s, run['gdk-file'], name)
        rec = still_key(PO.decode_still(b, PO.RECOMMENDED_BETA))
        agree('RecommendedBeta still vs image-still', rec, run['image-still'], name)
        agree('RecommendedBeta still vs gdk-loader', rec, run['gdk-loader'], name)
        strict = still_key(PO.decode_still(b, PO.STRICT))
        agree('Strict still vs image-still', strict, run['image-still'], name)
        agree('Strict still vs gdk-loader', strict, run['gdk-loader'], name)
    for label, (ok, bad, names, skipped) in stats.items():
        print(f'{label}: {ok}/{ok + bad} agree ({skipped} excluded: runner crash/timeout, or Strict rejects for a '
              f'resource cap: FileTooLarge, InvalidDimensions, OutputTooLarge, TooManyFrames, PixelBudgetExceeded)')
        if listing or label.startswith(('ImageCrateCompat', 'GdkCompat still vs gdk-loader')):
            for n in names:
                print('    ', n, '(second round)' if n.split(':', 1)[-1] in SECOND_ROUND else '')
    # acceptance gain of RecommendedBeta among inputs Strict rejects and an incumbent accepts
    gain = tot = 0
    for name, b, run in rows:
        if 'error' in O.decode_still(b) and (run['image-still']['status'] == 'ok' or run['gdk-loader']['status'] == 'ok'):
            tot += 1
            gain += 'error' not in PO.decode_still(b, PO.RECOMMENDED_BETA)
    print(f'RecommendedBeta accepts {gain} of the {tot} probe/vector/witness inputs that Strict rejects and image-still '
          f'or the gdk-pixbuf loader accepts.')
    return 1 if bridge_bad or r_bad else 0


if __name__ == '__main__':
    sys.exit(main())
