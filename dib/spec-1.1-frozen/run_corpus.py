#!/usr/bin/env python3
"""Read-only DIB 1.1 corpus diagnostic; ICO selection is a measurement shim."""
import collections
import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CORPUS = Path('/home/ws/projects/dib-verus/corpus')
spec = importlib.util.spec_from_file_location('dib_1_1_corpus_oracle', ROOT/'check_vectors.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

def first_dib(data, cursor=False):
    if len(data)<6 or data[:2]!=b'\0\0' or data[2:4] not in (b'\x01\0',b'\x02\0'):
        return None,'ContainerInvalid'
    count=int.from_bytes(data[4:6],'little')
    end=6+16*count
    if count==0 or end>len(data):return None,'ContainerInvalid'
    png=0
    for i in range(count):
        p=6+16*i
        size=int.from_bytes(data[p+8:p+12],'little')
        off=int.from_bytes(data[p+12:p+16],'little')
        if size==0 or off<end or off>len(data) or size>len(data)-off:
            return None,'ContainerInvalid'
        # This two-fixture workaround measures the DIB, not ICO's exact-range rule.
        if cursor and count==1 and size==8 and off+size<len(data):size=len(data)-off
        payload=data[off:off+size]
        if payload.startswith(b'\x89PNG\r\n\x1a\n'):
            png+=1
            continue
        return payload,None
    return None,'ContainerPngOnly' if png else 'ContainerNoDib'

def main(directory=CORPUS):
    counts=collections.Counter()
    for path in sorted(directory.iterdir()):
        if not path.is_file() or path.suffix.lower() not in ('.bmp','.ico','.cur'):
            continue
        data=path.read_bytes()
        if path.suffix.lower()=='.bmp':mode='BmpFile'; reason=None
        else:
            data,reason=first_dib(data,path.suffix.lower()=='.cur')
            mode='IcoEntryDibSlack'
        if reason:outcome=reason
        else:
            try:
                oracle.decode(data,mode)
                outcome='Accepted'
            except oracle.Invalid as exc:outcome=str(exc)
        counts[outcome]+=1
        print(f'{path.name}\t{outcome}')
    print(json.dumps({'total':sum(counts.values()),'counts':dict(sorted(counts.items()))},sort_keys=True))

if __name__=='__main__':
    if len(sys.argv)>2:raise SystemExit('usage: python3 -I run_corpus.py [directory]')
    main(Path(sys.argv[1]) if len(sys.argv)==2 else CORPUS)
