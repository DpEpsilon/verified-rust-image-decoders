#!/usr/bin/env python3
"""Run every webp decoder column against one existing file: python3 reproduce.py FILE"""
import json, sys
from pathlib import Path
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
import harness as h, engine
if len(sys.argv) != 2:
    raise SystemExit('usage: python3 reproduce.py FILE')
ad = h.WebP()
res, rels = ad.evaluate(Path(sys.argv[1]).resolve(), 10, '/tmp/claude-0', 'repro')
for n, r in res.items():
    print(json.dumps({'decoder': n, **r}, sort_keys=True))
for k, v in rels.items():
    print(json.dumps({'pair': list(k), 'relation': v[0], 'diff(max,mean,visible)': v[1]}))
