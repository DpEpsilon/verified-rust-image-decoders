#!/usr/bin/env python3
"""Run every ICNS selector against one existing file."""
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from format_harness import runners,invoke
if len(sys.argv)!=2:raise SystemExit('usage: python3 reproduce.py FILE.icns')
path=Path(sys.argv[1]).resolve()
for name in runners('icns'):print(json.dumps({'decoder':name,**invoke('icns',path,name,5)},sort_keys=True))
