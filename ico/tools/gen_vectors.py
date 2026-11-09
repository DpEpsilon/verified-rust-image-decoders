#!/usr/bin/env python3
"""Build ICO draft-two fixtures. Expectations derive from wire records, not ICO oracle.

Historical draft-one inputs are retained; DIB expected pixels use the hash-pinned
DIB 1.1 oracle. This script never asks the ICO oracle for an expected record.
"""
import importlib.util
import json
import struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
pin_spec=importlib.util.spec_from_file_location('ico_dib_oracle_pin',ROOT/'spec/dib_oracle_pin.py')
pin=importlib.util.module_from_spec(pin_spec);pin_spec.loader.exec_module(pin)
spec=importlib.util.spec_from_file_location('dib_1_1_expected',pin.verified_path())
dib=importlib.util.module_from_spec(spec);spec.loader.exec_module(dib)
PNG=bytes.fromhex('89504e470d0a1a0a')
HEADER_ERRORS={'FileTooLarge','TruncatedDirectory','InvalidReserved','UnsupportedType','EmptyDirectory','TooManyEntries','TruncatedEntries'}
MAX=67_108_864
OLD=json.loads((ROOT/'spec/VECTORS-draft1.json').read_text())

def dib24(w=1,h=1,rgb=(255,0,0),suffix=b'',image_size=0):
    hdr=struct.pack('<IiiHHIIiiII',40,w,2*h,1,24,0,image_size,0,0,0,0)
    stride=((w*3+3)//4)*4
    xor=(bytes((rgb[2],rgb[1],rgb[0]))*w).ljust(stride,b'\0')*h
    return hdr+xor+b'\0'*(((w+31)//32)*4*h)+suffix
DIB24=dib24()

def header(typ=1,count=1,res=0):return struct.pack('<HHH',res,typ,count)
def ent(w=1,h=1,c=0,r=0,lo=1,hi=24,size=None,off=None):
    return struct.pack('<BBBBHHII',w,h,c,r,lo,hi,len(DIB24) if size is None else size,22 if off is None else off)

def expect_entry(b,t,count,i):
    p=6+16*i;w,h,c,res,lo,hi,size,off=struct.unpack_from('<BBBBHHII',b,p)
    directory_end=6+16*count
    if res:status={'invalid':'InvalidEntryReserved'}
    elif size==0:status={'invalid':'EmptyEntry'}
    elif off<directory_end:status={'invalid':'EntryOverlapsDirectory'}
    elif off>len(b) or size>len(b)-off:status={'invalid':'EntryOutOfBounds'}
    else:status={'valid':{'kind':'Png' if size>=8 and b[off:off+8]==PNG else 'Dib','payload':[off,off+size]}}
    return {'width':w or 256,'height':h or 256,'color_count':c,'raw_offset':off,'raw_size':size,'status':status,
            'planes':lo if t==1 else None,'bit_count':hi if t==1 else None,
            'hotspot_x':lo if t==2 else None,'hotspot_y':hi if t==2 else None}

def expected_success_dib(b,entry):
    a,z=entry['status']['valid']['payload']
    try:
        w,h,p=dib.decode(b[a:z],'IcoEntryDibSlack')
        return {'width':w,'height':h,'rgba_runs':dib.runs(p)}
    except dib.Invalid as exc:return {'error':'Dib('+str(exc)+')'}

def expanded(v):
    raw=bytes.fromhex(v['input_hex']);k=v.get('input_repeat',1)
    if 'repeat_from' in v:
        j=v['repeat_from'];tail=bytes.fromhex(v['input_suffix_hex']);n=j+(len(raw)-j)*k+len(tail)
        return n,raw[:j]+raw[j:]*k+tail if n<=MAX else None
    n=len(raw)*k;return n,raw*k if n<=MAX else None

def result(b,op='parse',index=0,override=None,head_error=None):
    if head_error:return {'error':head_error}
    if len(b)<6:return {'error':'TruncatedDirectory'}
    reserved,t,count=struct.unpack_from('<HHH',b)
    if reserved:return {'error':'InvalidReserved'}
    if t not in (1,2):return {'error':'UnsupportedType'}
    if count==0:return {'error':'EmptyDirectory'}
    if count>1024:return {'error':'TooManyEntries'}
    if len(b)<6+16*count:return {'error':'TruncatedEntries'}
    records=[expect_entry(b,t,count,i) for i in range(count)]
    if op=='parse':return {'entries':records}
    if index>=count:return {'error':'IndexOutOfRange'}
    if override is not None and (len(override)!=count or override[index]!=records[index]):return {'error':'InvalidEntryRecord'}
    e=records[index];state=e['status']
    if 'invalid' in state:return {'error':state['invalid']}
    if state['valid']['kind']=='Png':return {'error':'PngDelegation'}
    return expected_success_dib(b,e)

V=[]
for old in OLD:
    v={k:x for k,x in old.items() if k!='expected'}
    n,b=expanded(v)
    head_error=old['expected'].get('error') if old['expected'].get('error') in HEADER_ERRORS else None
    v['expected']={'error':'FileTooLarge'} if n>MAX else result(b,v['operation'],v.get('index',0),head_error=head_error)
    V.append(v)
# Replace redundant draft-one metadata permutations with discrimination fixtures.
drop=set()
drop.update('short-dir-'+str(i) for i in (1,2,3,4))
drop.update('reserved-'+str(i) for i in (2,))
drop.update('type-'+str(i) for i in (4,))
drop.update('short-table-'+str(i) for i in (1,2,7,8))
drop.update('dimensions-'+x+'-type'+t for x in ('0-0','0-1','1-0','255-255','2-3','1-255','255-1') for t in ('1','2') if (x,t) not in {('0-0','1'),('0-1','2'),('1-0','1'),('255-255','2'),('2-3','1'),('1-255','2'),('255-1','1')})
drop.update('colour-'+str(c)+'-type'+str(t) for c in (0,1,2,16,255) for t in (1,2) if (c,t) not in {(0,1),(1,2),(16,1),(255,2)})
drop.update('words-'+str(a)+'-'+str(b)+'-type'+str(t) for a,b in ((0,0),(1,1),(65535,0),(0,65535),(123,456)) for t in (1,2) if (a,b,t) not in {(0,0,1),(1,1,2),(65535,0,2),(0,65535,1),(123,456,1),(123,456,2)})
drop.update('signature-prefix-'+str(n) for n in (1,3,9,15))
drop.update('signature-decode-'+str(n) for n in (1,3,9,15))
drop.update('png-'+str(n) for n in (9,16,24,32))
drop.update('png-decode-'+str(n) for n in (9,24,32))
drop.update('payload-gap-'+str(n) for n in (23,1024))
drop.update('tail-'+str(n) for n in (23,256))
V=[x for x in V if x['name'] not in drop]

def add(name,b,op='parse',index=0,override=None,**repeat):
    v={'name':name,'input_hex':b.hex(),'operation':op}
    if op=='decode':v['index']=index
    if override is not None:v['entries_override']=override
    v.update(repeat);n,raw=expanded(v)
    v['expected']={'error':'FileTooLarge'} if n>MAX else result(raw,op,index,override)
    V.append(v)

def one(name,payload=DIB24,raw=None,tail=b'',op='parse',index=0):
    b=header()+ (raw if raw is not None else ent(size=len(payload)))+payload+tail
    add(name,b,op,index)

# B2: test the full little-endian type and reserved words, not their low bytes.
for typ in (257,513):
    add(f'type-{typ}',header(typ=typ))

# B1: the next file byte must never complete a seven-byte entry signature.
for n in (1,3,6,7):
    one(f'png-prefix-followed-by-rest-{n}',PNG[:n],raw=ent(size=n),tail=PNG[n:],op='parse')
    one(f'png-prefix-followed-by-rest-{n}-decode',PNG[:n],raw=ent(size=n),tail=PNG[n:],op='decode')
for i in (0,3,4,7):
    bad=bytearray(PNG);bad[i]^=1
    one(f'png-bit-{i}-parse',bytes(bad))
    one(f'png-bit-{i}-decode',bytes(bad),op='decode')
one('png-shifted',b'\0'+PNG)
one('png-shifted-decode',b'\0'+PNG,op='decode')
# One bounded overlapping signature; second entry starts inside first signature.
b=header(count=2)+ent(off=38,size=8)+ent(off=39,size=7)+PNG
add('second-inside-png-signature',b)
add('second-inside-png-signature-decode',b,'decode',1)

# B2: multi-entry directory is [0,38), not [0,22) for first entry.
for i,off in ((0,22),(0,37),(0,38),(1,22),(1,37),(1,38)):
    r1=ent(size=1,off=38);r2=ent(size=1,off=38)
    if i==0:r1=ent(size=1,off=off)
    else:r2=ent(size=1,off=off)
    add(f'table-boundary-record{i}-off{off}',header(count=2)+r1+r2+b'\x42')
    add(f'table-boundary-record{i}-off{off}-decode',header(count=2)+r1+r2+b'\x42','decode',i)
for field,values in [('size',(0x10000,0x10001,0x80000000,0xffffffea)),('offset',(0x10016,0x80000016,0xffffffea))]:
    for value in values:
        r=ent(**{('size' if field=='size' else 'off'):value})
        one(f'wide-{field}-{value:x}',raw=r)
        one(f'wide-{field}-{value:x}-decode',raw=r,op='decode')
# Offset+size wrap and exact EOF, with no truncation of the 32-bit fields.
one('wide-sum-wrap',raw=ent(size=0xffffffea))
one('wide-sum-at-eof',raw=ent(size=len(DIB24)))
for i in (256,65536,65537,0x1000000,0xffffffff):
    one('index-narrow-'+str(i),op='decode',index=i)
# Shifted/nested versus genuinely straddling payload ranges.
b=header(count=2)+ent(size=8,off=38)+ent(size=8,off=39)+PNG+b'Z'
add('straddling-ranges',b)
add('straddling-ranges-decode1',b,'decode',1)
# Reverse nominal dimensions and preserve original directory order.
for reverse in (False,True):
    a=ent(w=16,h=16,off=38,size=8)
    z=ent(w=32,h=32,off=38,size=8)
    add('directory-order-'+('large-first' if reverse else 'small-first'),header(count=2)+(z+a if reverse else a+z)+PNG)
# D5: slack after the complete AND mask accepted in the new DIB mode.
for n in (1,4,64,128,192):
    one(f'dib-slack-{n}-decode',payload=dib24(suffix=b'Q'*n),op='decode')
# D9: bad records remain visible; only selecting one reports its error.
for variant in ('reserved','empty','overlap','oob'):
    bad={'reserved':ent(r=7,off=38),'empty':ent(size=0,off=38),
         'overlap':ent(off=22,size=1),'oob':ent(off=0x10016,size=1)}[variant]
    b=header(count=2)+ent(size=len(DIB24),off=38)+bad+DIB24
    add('valid-then-bad-'+variant,b)
    add('valid-then-bad-'+variant+'-decode-valid',b,'decode',0)
    add('valid-then-bad-'+variant+'-decode-bad',b,'decode',1)
    b2=header(count=2)+bad+ent(size=len(DIB24),off=38)+DIB24
    add('bad-then-valid-'+variant,b2)
    add('bad-then-valid-'+variant+'-decode-valid',b2,'decode',1)
    add('bad-then-valid-'+variant+'-decode-bad',b2,'decode',0)
# D10: forged record, swapped kind, changed range and list length are errors.
good=header()+ent()+DIB24
canonical=expect_entry(good,1,1,0)
for name,mutant in [('empty',[]),('extra',[canonical,canonical]),
                    ('wrong-width',[{**canonical,'width':2}]),
                    ('wrong-height',[{**canonical,'height':2}]),
                    ('wrong-color-count',[{**canonical,'color_count':7}]),
                    ('wrong-planes',[{**canonical,'planes':2}]),
                    ('wrong-bit-count',[{**canonical,'bit_count':32}]),
                    ('wrong-raw-size',[{**canonical,'raw_size':len(DIB24)-1}]),
                    ('wrong-kind',[{**canonical,'status':{'valid':{'kind':'Png','payload':[22,22+len(DIB24)]}}}]),
                    ('wrong-end',[{**canonical,'status':{'valid':{'kind':'Dib','payload':[22,23]}}}]),
                    ('oob',[{**canonical,'raw_offset':0xffffffff}])]:
    add('forged-'+name,good,'decode',0,override=mutant)
cur=header(typ=2)+ent(lo=11,hi=12)+DIB24
cur_entry=expect_entry(cur,2,1,0)
for key in ('hotspot_x','hotspot_y'):
    add('forged-'+key.replace('_','-'),cur,'decode',0,
        override=[{**cur_entry,key:cur_entry[key]+1}])
wire_invalid=header()+ent(r=1)+DIB24
invalid_entry=expect_entry(wire_invalid,1,1,0)
add('wire-invalid-correct-record',wire_invalid,'decode',0,override=[invalid_entry])
add('wire-invalid-forged-valid',wire_invalid,'decode',0,
    override=[{**invalid_entry,'status':{'valid':{'kind':'Dib','payload':[22,22+len(DIB24)]}}}])
add('wire-invalid-forged-defect',wire_invalid,'decode',0,
    override=[{**invalid_entry,'status':{'invalid':'EmptyEntry'}}])
two=header(count=2)+ent(size=len(DIB24),off=38)+ent(size=len(DIB24),off=38)+DIB24
selected=expect_entry(two,1,2,0)
other=expect_entry(two,1,2,1)
add('forged-unselected-ignored',two,'decode',0,
    override=[selected,{**other,'width':other['width']+1}])
add('decode-parse-header-error',header(count=2),'decode',0,override=[])
add('decode-count-cap-before-index',header(count=1025),'decode',4294967295)
add('decode-short-table-before-index',header(count=2),'decode',4294967295)
add('decode-index-priority',good,'decode',1,override=[])
# Replace the identical-record count boundary with 1,024 distinct records.
V=[v for v in V if v['name'] not in ('count-exact-1024','count-exact-1024-decode-last')]
end=6+16*1024
rows=[]
for i in range(1024):
    rows.append(ent(w=(i%255)+1,h=((i//255)%255)+1,c=i%256,lo=i,hi=65535-i,
                    size=(len(DIB24) if i==1023 else 8),off=(end+8 if i==1023 else end)))
large=header(count=1024)+b''.join(rows)+PNG+DIB24
add('count-exact-1024',large)
add('count-exact-1024-decode-last',large,'decode',1023)
add('count-exact-1024-decode-first',large,'decode',0)
assert len(V)>=120,len(V)
assert all('entries_override' not in v or v['operation']=='decode' for v in V)
assert len(V)==len({v['name'] for v in V})
(ROOT/'spec/VECTORS.json').write_text(json.dumps(V,indent=2)+'\n')
print('ICO draft two vectors',len(V))
