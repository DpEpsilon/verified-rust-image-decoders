#!/usr/bin/env python3
"""Run one file through every decoder: python3 -I reproduce.py FILE"""
import json, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import harness, lenient_text
for name in harness.RUNNERS:
    v = lenient_text.invoke(harness.CFG, sys.argv[1], name)
    print(f"{name:18} " + json.dumps({k: v[k] for k in ('status', 'error_class', 'width', 'height', 'rgba_sha256', 'hotspot') if k in v}))
