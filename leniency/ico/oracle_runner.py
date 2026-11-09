#!/usr/bin/env python3
"""Independent ICO 1.1 + pinned DIB 1.1 oracle; no image-rs dependency.

The oracle explicitly delegates PNG, so PngDelegation means 'not decoded here',
not evidence that the PNG image violates either specification.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

SRC=Path('/home/ws/projects/ico/spec/check_vectors.py')
spec=importlib.util.spec_from_file_location('ico_spec_oracle',SRC)
oracle=importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

def output(mode,b,dump):
    try:
        entries=oracle.parse(b)
        if mode=='first':i=0
        elif mode.startswith('entry-'):i=int(mode[6:])
        else:i=max(range(len(entries)),key=lambda x:(entries[x]['width']*entries[x]['height'],entries[x]['bit_count'] or 0))
        if i>=len(entries):return {'status':'error','error_class':'IndexOutOfRange','message':'IndexOutOfRange'}
        v=oracle.actual(b,'decode',i,entries=entries)
        if 'error' in v:return {'status':'error','error_class':v['error'],'message':v['error'],'selection':f'entry:{i}'}
        rgba=b''.join(bytes.fromhex(run['hex'])*run['count'] for run in v['rgba_runs'])
        if dump:Path(dump).write_bytes(rgba)
        return {'status':'ok','width':v['width'],'height':v['height'],
                'orig_color_type':'DIB RGBA8','alpha_present':any(rgba[k]!=255 for k in range(3,len(rgba),4)),
                'rgba_sha256':hashlib.sha256(rgba).hexdigest(),'selection':f'entry:{i}:dib'}
    except oracle.Invalid as e:
        return {'status':'error','error_class':str(e),'message':str(e)}
    except (OSError,ValueError) as e:
        return {'status':'error','error_class':type(e).__name__,'message':str(e)[:300]}

if __name__=='__main__':
    try:print(json.dumps(output(sys.argv[1],Path(sys.argv[2]).read_bytes(),sys.argv[3] if len(sys.argv)>3 else None)))
    except Exception as e:print(json.dumps({'status':'crash','error_class':type(e).__name__,'message':str(e)[:300]}))
