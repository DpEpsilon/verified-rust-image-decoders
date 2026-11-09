#!/usr/bin/env python3
"""Reproduce one BMP witness through every isolated decoder."""
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
from harness import RUNNERS, invoke
if len(sys.argv)!=2:
    raise SystemExit('usage: python3 reproduce.py FILE.bmp')
path=Path(sys.argv[1]).resolve()
for name in RUNNERS:
    print(json.dumps({'decoder':name,**invoke(path,name,10)},sort_keys=True))
