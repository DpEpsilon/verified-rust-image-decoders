#!/usr/bin/env python3
"""Run ICO/CUR selectors, optionally each individual directory entry."""
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from format_harness import runners,invoke
if len(sys.argv) not in (2,3):raise SystemExit('usage: python3 reproduce.py FILE.ico [--entries]')
path=Path(sys.argv[1]).resolve(); names=runners('ico')
if len(sys.argv)==3:
    if sys.argv[2]!='--entries':raise SystemExit('only --entries is accepted')
    data=path.read_bytes();count=min(12,int.from_bytes(data[4:6],'little')) if len(data)>=6 else 0
    names += [name for i in range(count) for name in ('oracle-entry-'+str(i),'crate-entry:'+str(i))]
for name in names:print(json.dumps({'decoder':name,**invoke('ico',path,name,5)},sort_keys=True))
