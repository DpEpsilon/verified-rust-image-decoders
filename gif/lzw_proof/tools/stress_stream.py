#!/usr/bin/env python3
"""Generate, oracle-check and stream an >=8 MiB LZW payload; retain no data files."""
import importlib.util
import json
from pathlib import Path
import random
import sys

sys.dont_write_bytecode = True
spec_dir = Path(__file__).resolve().parents[2] / 'spec-draft'
def load(name):
    spec = importlib.util.spec_from_file_location(name, spec_dir / (name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

gen = load('gen_vectors')
oracle = load('gif_oracle')
mode = sys.argv[1]
assert mode in ['clear', 'deferred']
raster = random.Random(0x8f0021).randbytes(8 * 1024 * 1024)
codes = gen.lzw_codes(raster, 8, clear_on_full=(mode == 'clear'))
widths = gen.widths_for(codes, 8)
assert max(widths) == 12
assert codes.count(256) > 1 if mode == 'clear' else codes.count(256) == 1
assert len(codes) > 8192
payload = gen.pack(codes, widths)
assert len(payload) >= 8 * 1024 * 1024
position = sum(widths)
def emit(q, value):
    assert value == raster[q], (q, value, raster[q])
oracle.lzw_decode(8, payload, len(raster), emit)
header = dict(area=len(raster), payload=len(payload), pos=position, mode=mode, max_width=12)
sys.stdout.buffer.write(json.dumps(header).encode() + b'\n')
sys.stdout.buffer.write(raster)
sys.stdout.buffer.write(payload)
