#!/usr/bin/env python3
"""Checks for the PNG policy draft.

1. Strict bridge: policy_oracle(STRICT) equals png_oracle on every vector, probe and witness input
   (still and frames, including the first error).
2. Obligation R: wherever Strict accepts (still: Ok; frames: every frame Strict returns), RecommendedBeta
   returns the same bytes with recovered=false.
3. Profile agreement with the measured incumbents (probes/results.json):
   ImageCrateCompat-0.25.10: still vs `image-still`, frames vs `image` (APNG frame loop);
   GdkCompat-2.42.12: still vs `gdk-loader` (GdkPixbufLoader, whole buffer), also reported vs `gdk-file`;
   ImageCrateCompat frames also vs `glycin` (probes/results-glycin.json, glycin's PNG loader path).
   An input counts as agreeing when both reject, or both accept with equal dimensions and SHA-256.
   Sets: the fitted set (inputs of at most 4 KiB in results.json), the inputs above 4 KiB (stored by
   reference, regenerated and checked by SHA-256), and the held-out sets A and B (results-heldout*.json).
Usage: python3 -I validate.py [--list] [--no-large] [--no-heldout]
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / 'spec-draft'))
sys.path.insert(0, str(HERE / 'probes'))
import png_oracle as O  # noqa: E402
import policy_oracle as PO  # noqa: E402
import run_probes as RP  # noqa: E402

RESULTS = json.loads((HERE / 'probes/results.json').read_text())


def _load(name):
    f = HERE / 'probes' / name
    return json.loads(f.read_text()) if f.exists() else None


GLYCIN = {r['name']: r['runners'] for r in (_load('results-glycin.json') or [])}
HELDOUT = _load('results-heldout.json')
HELDOUT_B = _load('results-heldoutB.json')
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
    if 'error' in r or not r['frames']:             # an iterator whose first item is an error: the runners report an error
        return ('err',)
    return ('ok', len(r['frames']), hashlib.sha256(b''.join(f['rgba'] for f in r['frames'])).hexdigest())


CAPS = {'FileTooLarge', 'InvalidDimensions', 'OutputTooLarge', 'TooManyFrames', 'PixelBudgetExceeded'}


def agreement(rows, profiles=None):
    """{label: [agree, disagree, [names], excluded]} for the profile/runner pairs of section 3."""
    profiles = profiles or {'ImageCrateCompat': PO.IMAGE, 'GdkCompat': PO.GDK, 'RecommendedBeta': PO.RECOMMENDED_BETA,
                            'Strict': PO.STRICT}
    stats = {}
    beyond_caps = set()
    for name, b, run in rows:
        e = O.decode_frames(b).get('error') or O.decode_still(b).get('error')
        if e in CAPS:
            beyond_caps.add(name)

    def agree(label, mine, theirs, name, frames=False):
        st = stats.setdefault(label, [0, 0, [], 0])
        if theirs is None:
            return
        if theirs['status'] in ('crash', 'timeout') or name in beyond_caps:
            st[3] += 1
            return
        if frames:
            t = ('err',) if theirs['status'] != 'ok' else ('ok', int(theirs.get('frames', 1)), theirs.get('seq'))
            ok = (mine == t) if mine[0] == 'ok' else t[0] == 'err'
        else:
            ok = mine == runner_key(theirs) if mine[0] == 'ok' else theirs['status'] != 'ok'
        st[0 if ok else 1] += 1
        if not ok:
            st[2].append(name)

    for name, b, run in rows:
        img = profiles.get('ImageCrateCompat')
        if img is not None:
            agree('ImageCrateCompat still vs image-still', still_key(PO.decode_still(b, img)), run.get('image-still'), name)
            img_f = frames_key(PO.decode_frames(b, img))
            agree('ImageCrateCompat frames vs image', img_f, run.get('image'), name, True)
            agree('ImageCrateCompat frames vs glycin', img_f, run.get('glycin'), name, True)
        gdk = profiles.get('GdkCompat')
        if gdk is not None:
            gdk_s = still_key(PO.decode_still(b, gdk))
            agree('GdkCompat still vs gdk-loader', gdk_s, run.get('gdk-loader'), name)
            agree('GdkCompat still vs gdk-file (not targeted)', gdk_s, run.get('gdk-file'), name)
        if 'RecommendedBeta' in profiles:
            rec = still_key(PO.decode_still(b, profiles['RecommendedBeta']))
            agree('RecommendedBeta still vs image-still', rec, run.get('image-still'), name)
            agree('RecommendedBeta still vs gdk-loader', rec, run.get('gdk-loader'), name)
        if 'Strict' in profiles:
            strict = still_key(PO.decode_still(b, profiles['Strict']))
            agree('Strict still vs image-still', strict, run.get('image-still'), name)
            agree('Strict still vs gdk-loader', strict, run.get('gdk-loader'), name)
    return stats


def report(stats, listing, title):
    print(f'-- {title}')
    for label, (ok, bad, names, skipped) in stats.items():
        if ok + bad + skipped == 0:
            continue
        print(f'{label}: {ok}/{ok + bad} agree ({skipped} excluded: runner crash/timeout, or Strict rejects for a '
              f'resource cap: FileTooLarge, InvalidDimensions, OutputTooLarge, TooManyFrames, PixelBudgetExceeded)')
        if listing or label.startswith(('ImageCrateCompat', 'GdkCompat still vs gdk-loader')):
            for n in names:
                print('    ', n, '(second round)' if n.split(':', 1)[-1] in SECOND_ROUND else '')


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
    large = []
    for r in RESULTS:
        run = dict(r['runners'], **GLYCIN.get(r['name'], {}))
        if r['hex'] is None:
            large.append(r)
            continue
        rows.append((r['name'], bytes.fromhex(r['hex']), run))
    stats = agreement(rows)
    report(stats, listing, 'fitted set (at most 4 KiB)')
    if '--no-large' not in sys.argv:
        lrows = [(r['name'], RP.resolve(r), dict(r['runners'], **GLYCIN.get(r['name'], {}))) for r in large]
        report(agreement(lrows), listing, f'inputs above 4 KiB ({len(lrows)}, regenerated from their generators, SHA-256 checked)')
    if HELDOUT is not None and '--no-heldout' not in sys.argv:
        hrows = [(r['name'], RP.resolve(r), r['runners']) for r in HELDOUT]
        report(agreement(hrows), listing, f'held-out set A ({len(hrows)} mutations, heldout_def.py; seen while modelling gap 2)')
    if HELDOUT_B is not None and '--no-heldout' not in sys.argv:
        hrows = [(r['name'], RP.resolve(r), r['runners']) for r in HELDOUT_B]
        report(agreement(hrows), listing, f'held-out set B ({len(hrows)} mutations, frozen after the gap-2/gap-5 changes; the stored-tail fix came after it)')
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
