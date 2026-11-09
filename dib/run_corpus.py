#!/usr/bin/env python3
"""Local Pillow corpus diagnostic; container selection is a measurement shim, not the ICO spec."""
import collections
import importlib.util
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('dib_oracle',ROOT/'spec'/'check_vectors.py')
oracle=importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

def entry(data, cursor=False):
    if len(data)<6 or data[:2]!=b'\x00\x00' or data[2:4] not in (b'\x01\x00',b'\x02\x00'):
        return None,'ContainerInvalid'
    n=int.from_bytes(data[4:6],'little')
    if n==0 or len(data)<6+16*n: return None,'ContainerInvalid'
    png=0
    for i in range(n):
        p=6+16*i
        length=int.from_bytes(data[p+8:p+12],'little')
        offset=int.from_bytes(data[p+12:p+16],'little')
        if length==0 or offset<6+16*n or offset+length>len(data): return None,'ContainerInvalid'
        # Some Pillow cursor fixtures incorrectly declare eight bytes; measure their
        # actual DIB from the offset to EOF, without changing the core's contract.
        if cursor and length==8 and n==1 and offset+length<len(data):
            length=len(data)-offset
        content=data[offset:offset+length]
        if content.startswith(b'\x89PNG\r\n\x1a\n'):
            png+=1; continue
        return content,None
    return None,'ContainerPngOnly' if png else 'ContainerNoDib'

def run(directory):
    counts=collections.Counter(); rows=[]
    for path in sorted(directory.iterdir()):
        if path.suffix.lower() not in ('.bmp','.ico','.cur') or not path.is_file(): continue
        data=path.read_bytes()
        if path.suffix.lower()=='.bmp':
            mode='BmpFile'; kind=None
        else:
            data,kind=entry(data, cursor=path.suffix.lower()=='.cur'); mode='IcoEntryDib'
        if kind:
            outcome=kind
        else:
            try:
                oracle.decode(data,mode)
                outcome='Accepted'
            except oracle.Invalid as e:
                outcome=str(e)
        rows.append({'file':path.name,'outcome':outcome})
        counts[outcome]+=1
        print(f'{path.name}\t{outcome}')
    print(json.dumps({'total':len(rows),'counts':dict(sorted(counts.items()))},sort_keys=True))
    return rows,counts

if __name__=='__main__':
    if len(sys.argv)>2: raise SystemExit('usage: python3 -I run_corpus.py [directory]')
    run(Path(sys.argv[1]) if len(sys.argv)==2 else ROOT/'corpus')
