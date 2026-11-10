#!/usr/bin/env python3
"""Run every gif runner against one existing file: python3 reproduce.py FILE"""
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
from gif_adapter import FORMAT
if len(sys.argv) != 2:
    raise SystemExit('usage: python3 reproduce.py FILE')
path = Path(sys.argv[1]).resolve()
for name in FORMAT.runners:
    print(json.dumps({'decoder': name, **FORMAT.invoke(path, name, 5)}, sort_keys=True))
