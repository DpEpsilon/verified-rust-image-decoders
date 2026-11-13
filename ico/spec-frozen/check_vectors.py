#!/usr/bin/env python3
"""Independent ICO/CUR byte oracle; the DIB 1.1 oracle is hash-pinned.

Run python3 -I spec/check_vectors.py. Parse preserves invalid per-entry records.
Decode is total: it compares the selected supplied record to the wire record.
"""
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
def pinned_dib_oracle():
    spec=importlib.util.spec_from_file_location('ico_dib_oracle_pin',HERE/'dib_oracle_pin.py')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module.verified_path()

MAX_INPUT = 67_108_864
MAX_ENTRIES = 1024
SIGNATURE = bytes.fromhex('89504e470d0a1a0a')
ERRORS = frozenset('FileTooLarge TruncatedDirectory InvalidReserved UnsupportedType EmptyDirectory TooManyEntries TruncatedEntries InvalidEntryReserved EmptyEntry EntryOverlapsDirectory EntryOutOfBounds IndexOutOfRange InvalidEntryRecord PngDelegation'.split())
ENTRY_ERRORS = frozenset('InvalidEntryReserved EmptyEntry EntryOverlapsDirectory EntryOutOfBounds'.split())

class Invalid(Exception):
    pass

def fail(name):
    raise Invalid(name)

def word(b, p, n):
    return int.from_bytes(b[p:p+n], 'little')

def directory(b):
    if len(b) > MAX_INPUT: fail('FileTooLarge')
    if len(b) < 6: fail('TruncatedDirectory')
    if word(b, 0, 2): fail('InvalidReserved')
    typ = word(b, 2, 2)
    if typ not in (1,2): fail('UnsupportedType')
    count = word(b, 4, 2)
    if not count: fail('EmptyDirectory')
    if count > MAX_ENTRIES: fail('TooManyEntries')
    end = 6 + 16*count
    if end > len(b): fail('TruncatedEntries')
    return typ,count,end

def record(b, typ, count, end, i):
    p=6+16*i
    width,height=b[p] or 256,b[p+1] or 256
    colors,reserved=b[p+2],b[p+3]
    lo,hi=word(b,p+4,2),word(b,p+6,2)
    length,off=word(b,p+8,4),word(b,p+12,4)
    if reserved:status={'invalid':'InvalidEntryReserved'}
    elif length==0:status={'invalid':'EmptyEntry'}
    elif off<end:status={'invalid':'EntryOverlapsDirectory'}
    elif off>len(b) or length>len(b)-off:status={'invalid':'EntryOutOfBounds'}
    else:
        kind='Png' if length>=8 and b[off:off+8]==SIGNATURE else 'Dib'
        status={'valid':{'kind':kind,'payload':[off,off+length]}}
    return {'width':width,'height':height,'color_count':colors,
            'raw_offset':off,'raw_size':length,'status':status,
            'planes':lo if typ==1 else None,'bit_count':hi if typ==1 else None,
            'hotspot_x':lo if typ==2 else None,'hotspot_y':hi if typ==2 else None}

def parse(b):
    typ,count,end=directory(b)
    return [record(b,typ,count,end,i) for i in range(count)]

def load_dib():
    spec=importlib.util.spec_from_file_location('pinned_dib_1_1_oracle',pinned_dib_oracle())
    mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
    return mod

def decode_entry(b, entries, index, dib=None):
    # Revalidation uses only the selected wire record; another invalid record
    # cannot change its result. The canonical comparison rejects forged entries.
    typ,count,end=directory(b)
    if index>=count:fail('IndexOutOfRange')
    if len(entries)!=count:fail('InvalidEntryRecord')
    expected=record(b,typ,count,end,index)
    if entries[index]!=expected:fail('InvalidEntryRecord')
    status=expected['status']
    if 'invalid' in status:fail(status['invalid'])
    if status['valid']['kind']=='Png':fail('PngDelegation')
    a,z=status['valid']['payload']
    if dib is None:dib=load_dib()
    try:
        w,h,pixels=dib.decode(b[a:z],'IcoEntryDibSlack')
        return {'width':w,'height':h,'rgba_runs':dib.runs(pixels)}
    except dib.Invalid as exc:
        return {'error':'Dib('+str(exc)+')'}

def actual(b, operation, index=None, dib=None, entries=None):
    try:
        if operation=='parse':return {'entries':parse(b)}
        if operation!='decode':raise ValueError(operation)
        if index is None:raise ValueError('index required')
        if entries is None:entries=parse(b)
        return decode_entry(b, entries, index, dib)
    except Invalid as exc:return {'error':str(exc)}

def expand(v):
    seed=bytes.fromhex(v['input_hex'])
    repeat=v.get('input_repeat',1)
    assert type(repeat) is int and repeat>0
    if 'repeat_from' in v:
        assert 'input_suffix_hex' in v
        cut=v['repeat_from'];assert type(cut) is int and 0<=cut<len(seed)
        suffix=bytes.fromhex(v['input_suffix_hex'])
        length=cut+(len(seed)-cut)*repeat+len(suffix)
        return length,seed[:cut]+seed[cut:]*repeat+suffix if length<=MAX_INPUT else None
    assert 'input_suffix_hex' not in v
    length=len(seed)*repeat
    return length,seed*repeat if length<=MAX_INPUT else None

def check_vector(v,dib):
    allowed={'name','input_hex','operation','expected','index','input_repeat','repeat_from','input_suffix_hex','entries_override'}
    assert {'name','input_hex','operation','expected'} <= v.keys() <= allowed
    assert v['operation'] in ('parse','decode')
    assert ('index' in v)==(v['operation']=='decode')
    assert 'entries_override' not in v or v['operation']=='decode'
    assert v['input_hex']==v['input_hex'].lower() and len(v['input_hex'])%2==0
    length,b=expand(v)
    if length>MAX_INPUT: got={'error':'FileTooLarge'}
    else:got=actual(b,v['operation'],v.get('index'),dib,v.get('entries_override'))
    expected=v['expected']
    if 'error' in expected:
        name=expected['error'];assert name in ERRORS or name.startswith('Dib(') and name.endswith(')') and name[4:-1] in dib.ERRORS
    elif v['operation']=='parse':assert set(expected)=={'entries'}
    else:
        assert set(expected)=={'width','height','rgba_runs'}
        runs=expected['rgba_runs'];assert sum(r['count'] for r in runs)==expected['width']*expected['height']
        assert all(set(r)=={'count','hex'} and r['count']>0 and len(r['hex'])==8 and (i==0 or runs[i-1]['hex']!=r['hex']) for i,r in enumerate(runs))
    if got!=expected:
        # A compact diagnostic for the 1,024-entry boundary fixture.
        if 'entries' in got and 'entries' in expected:
            for i,(a,e) in enumerate(zip(got['entries'],expected['entries'])):
                if a!=e:raise AssertionError(f"{v['name']}: entry {i}: {e!r} != {a!r}")
        raise AssertionError(f"{v['name']}: {expected!r} != {got!r}")

def main():
    dib=load_dib()
    vectors=json.loads((HERE/'VECTORS.json').read_text())
    assert len(vectors)==len({v['name'] for v in vectors})
    for v in vectors:check_vector(v,dib)
    print(f'{len(vectors)} independent ICO conformance vectors passed')

if __name__=='__main__':main()
