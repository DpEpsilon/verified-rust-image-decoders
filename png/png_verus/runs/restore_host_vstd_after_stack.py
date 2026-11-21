#!/usr/bin/env python3
"""Restore only PNG-owned vstd cache entries by copying pinned source bytes."""
from pathlib import Path
import hashlib,json,shutil,fcntl,time
ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')
source=ROOT.parent/'inflate_verus/target/debug'
dest=ROOT/'target/debug'
assert dest.resolve().is_relative_to(ROOT.resolve())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
pins=json.loads((ROOT/'evidence/vstd-cache-copy.json').read_text())
for x in pins:
 p=Path(x['source']);assert sha(p)==x['sha256'] and p.stat().st_mtime_ns==x['source_mtime_ns']
rels=[Path('.fingerprint')/('vstd-'+s) for s in ['01822e92e4f44378','4f9fb55783e359aa','cd53d8cc110e2a10']]
rels += [Path('build')/('vstd-'+s) for s in ['4f9fb55783e359aa','cd53d8cc110e2a10']]
rels += [Path(x['source']).relative_to(source) for x in pins]
files=[]
for rel in rels:
 p=source/rel
 assert p.exists(),p
 selected=sorted(p.rglob('*')) if p.is_dir() else [p]
 for f in selected:
  assert not f.is_symlink(),f
  if f.is_file():
   st=f.stat();files.append(dict(path=str(f.relative_to(source)),sha256=sha(f),bytes=st.st_size,mtime_ns=st.st_mtime_ns))
lock=Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a')
fcntl.flock(lock.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
for x in files:
 p=source/x['path'];q=dest/x['path'];assert q.resolve().is_relative_to(dest.resolve())
 if q.exists():assert not q.is_symlink() and not q.samefile(p),q
 q.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,q)
 assert not q.samefile(p) and sha(q)==x['sha256'] and q.stat().st_mtime_ns==x['mtime_ns']
for x in files:
 p=source/x['path'];st=p.stat();assert sha(p)==x['sha256'] and st.st_size==x['bytes'] and st.st_mtime_ns==x['mtime_ns']
for x in pins:assert sha(Path(x['copy']))==x['sha256']
(ROOT/'runs/host-vstd-restoration-after-0325.json').write_text(json.dumps(dict(source_root=str(source),copy_root=str(dest),files=files,pinned_artifacts_checked_before_after=True,source_bytes_sizes_mtimes_unchanged=True,copy_only=True,no_links=True,no_source_cache_writes=True,restored_ns=time.time_ns()),indent=2)+'\n')
print('Restored',len(files),'PNG-owned vstd files by copy; all source hashes/mtimes and pinned artifacts match.')
