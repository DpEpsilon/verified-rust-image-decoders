#!/usr/bin/env python3
"""Reject allocator references in release libraries and instantiated entry probes.

This falsification check supplements, not replaces, the production call-graph
allocation audit. The probe prevents a generic-only library's empty symbol table
from making the check vacuous.
"""
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
NM = Path('/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-nm')
BAD = re.compile(r'alloc::|_ZN5alloc|__rust_(?:alloc|dealloc|realloc)|__rdl_|__rg_')

for target in ('', 'thumbv7m-none-eabi'):
    release = ROOT / 'target' / target / 'release'
    for path in (release / 'libinflate_verus.rlib', release / 'examples/liballocator_probe.rlib'):
        assert path.is_file(), path
        r = subprocess.run([str(NM), '--demangle', str(path)], text=True,
                           capture_output=True, check=True, timeout=30)
        assert not BAD.search(r.stdout), f'allocator symbol in {path}: {r.stdout}'
        if 'allocator_probe' in path.name:
            assert 'allocator_probe::probe_into' in r.stdout, 'missing actual writing instantiation'
            assert 'allocator_probe::probe_measure' in r.stdout, 'missing actual measuring instantiation'
        print(f'no alloc:: or allocator symbols: {path}; {len(r.stdout.splitlines())} symbol lines')
print('Both public entry points instantiated without allocator references on host and thumbv7m')
