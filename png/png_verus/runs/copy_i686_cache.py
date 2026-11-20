#!/usr/bin/env python3
"""Copy the pinned existing i686 cache into PNG-owned output, without links."""
from pathlib import Path
import fcntl
import hashlib
import json
import shutil
import time

ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
plan=json.loads((ROOT/'evidence/i686-cache-preflight-0310.json').read_text())
src=Path(plan['source'])
dst=ROOT/'target'
excluded={'.rustc_info.json'}
assert dst.resolve()==ROOT.resolve()/'target', 'Destination must be PNG-owned target'
lock=Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a')
fcntl.flock(lock.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
# Validate every source and destination before writing any file.
for row in plan['files']:
    p=src/row['path'];st=p.stat()
    assert p.is_file() and not p.is_symlink()
    assert sha(p)==row['sha256'] and st.st_size==row['bytes'] and st.st_mtime_ns==row['mtime_ns'],p
    if row['path'] in excluded:continue
    q=dst/row['path']
    assert q.resolve().is_relative_to(dst.resolve()), q
    if q.exists():
        assert q.is_file() and not q.is_symlink() and not q.samefile(p),q
        assert sha(q)==row['sha256'],('Different existing destination; refusing overwrite',str(q))
rows=[]
for row in plan['files']:
    if row['path'] in excluded:continue
    p=src/row['path'];q=dst/row['path']
    q.parent.mkdir(parents=True,exist_ok=True)
    shutil.copy2(p,q)
    assert not q.samefile(p) and sha(q)==row['sha256'],q
    rows.append(dict(source=str(p),copy=str(q),sha256=row['sha256'],source_mtime_ns=row['mtime_ns']))
for row in plan['files']:
    p=src/row['path'];st=p.stat()
    assert sha(p)==row['sha256'] and st.st_size==row['bytes'] and st.st_mtime_ns==row['mtime_ns'],p
record=dict(kind='copy-existing-pinned-i686-cache',completed_time_ns=time.time_ns(),source_root=str(src),copy_root=str(dst),files=rows,excluded=sorted(excluded),source_files_unchanged=len(plan['files']),no_hardlinks=True,no_vstd_build=True,replay_performed=False)
(ROOT/'runs/i686-cache-copy.json').write_text(json.dumps(record,indent=2)+'\n')
print('Copied',len(rows),'cache files into PNG-owned target; all',len(plan['files']),'source files unchanged; no hardlinks or builds.')
