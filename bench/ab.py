#!/usr/bin/env python3
"""A/B on the large files: migrated cores (try_*, runner src/main.rs) vs pre-migration cores (~/projects/vd-commit,
runner built from a copy of the old src/main.rs in /tmp/claude-0/bench-old). Same session, interleaved fresh
processes, both pinned with taskset (bench.PIN). Writes results/ab-large.json. Usage: python3 -I ab.py [--rounds 9]"""
import argparse, glob, json, os, sys
sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bench as B
ap = argparse.ArgumentParser(); ap.add_argument('--rounds', type=int, default=9); ap.add_argument('--passes', type=int, default=5)
a = ap.parse_args()
out = {}
for fmt, pat in [('bmp', '/*bmp*.bmp'), ('tga', '/*tga*.tga'), ('icns', '/*.icns'), ('ico', '/*.ico')]:
    ver = f'{fmt}-verified'
    for f in sorted(glob.glob(B.LARGE + pat)):
        base = os.path.basename(f)
        if 'probe' in base: continue
        lst = B.write_list(f'ab-{base}', [f])
        c_new, c_old = B.check(ver, lst), B.check('old:' + ver, lst)
        n, o = c_new.get(f), c_old.get(f)
        if not (n and o and n['status'] == 'ok' and o['status'] == 'ok'):
            out[base] = dict(note='not decoded by both', new=n and n['status'], old=o and o['status']); continue
        same = (n['sha'], n['w'], n['h']) == (o['sha'], o['w'], o['h'])
        print('A/B', base, 'output identical' if same else 'OUTPUT DIFFERS', flush=True)
        t = B.measure([ver, 'old:' + ver], lst, a.rounds, a.passes, 1, base)
        t['output_identical'] = same
        out[base] = t
json.dump(out, open(os.path.join(B.HERE, 'results', 'ab-large.json'), 'w'), indent=1)
