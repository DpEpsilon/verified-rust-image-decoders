#!/usr/bin/env python3
"""Append 1.1-only DIB witnesses without altering the 214 frozen 1.0 records."""
import json, struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
OLD=Path('/home/ws/projects/verified-decoders/dib/spec-frozen/VECTORS.json')
v=json.loads(OLD.read_text());assert len(v)==214
pix={'width':1,'height':1,'rgba_runs':[{'count':1,'hex':'ff0000ff'}]}
def body(img=0,mask=b'\0'*4):
    return struct.pack('<IiiHHIIiiII',40,1,2,1,24,0,img,0,0,0,0)+bytes.fromhex('0000ff00')+mask

def add(name,b,mode,expected):
    v.append({'name':name,'mode':mode,'input_hex':b.hex(),'expected':expected})
for n in (0,1,2,3,4,5,6,7,8,9,16,32,63,64,65,127,128,129):
    b=body()+bytes([0x5a])*n
    add(f'slack-{n}-new',b,'IcoEntryDibSlack',pix)
    if n: add(f'slack-{n}-old',b,'IcoEntryDib',{'error':'TrailingBytes'})
for image in (0,4,8,9):
    b=body(image)+b'\x42'
    ex=pix if image!=9 else {'error':'InvalidImageSize'}
    add(f'image-size-{image}-slack',b,'IcoEntryDibSlack',ex)
    add(f'image-size-{image}-old',b,'IcoEntryDib',{'error':'TrailingBytes'} if image!=9 else ex)
for n in (0,1,2,3):
    b=body(mask=b'\0'*n)
    add(f'mask-short-{n}-new',b,'IcoEntryDibSlack',{'error':'TruncatedAndMask'})
    add(f'mask-short-{n}-old',b,'IcoEntryDib',{'error':'TruncatedAndMask'})
for name,b,expected in [('header-short',b'\x28\0\0', 'TruncatedDibHeader'),('pixels-short',body()[:43], 'TruncatedPixels'),('odd-height',body()[:8]+struct.pack('<i',3)+body()[12:], 'InvalidDimensions'),('invalid-size-and-short',body(9,mask=b'\0'*2), 'InvalidImageSize')]:
    add(name+'-new',b,'IcoEntryDibSlack',{'error':expected})
# At the 64 MiB input boundary, the entry is complete and the remainder is slack.
seed=body()+b'Z'
for mode,expect in [('IcoEntryDibSlack',pix),('IcoEntryDib',{'error':'TrailingBytes'})]:
    v.append({'name':'max-input-slack-'+mode,'mode':mode,'input_hex':seed.hex(),
              'repeat_from':len(body()),'input_repeat':67_108_864-len(body()),
              'input_suffix_hex':'','expected':expect})
v.append({'name':'over-max-input-slack-new','mode':'IcoEntryDibSlack',
          'input_hex':seed.hex(),'repeat_from':len(body()),
          'input_repeat':67_108_865-len(body()),'input_suffix_hex':'',
          'expected':{'error':'FileTooLarge'}})

# B1: prove that the new mode retains the old ICO pixel and header semantics.
red=lambda a: {'width':1,'height':1,'rgba_runs':[{'count':1,'hex':'ff0000'+f'{a:02x}'}]}
def raw(w=1,h=2,bits=32,comp=0,img=0,used=0,extra=b'',xor=b'',mask=b'\0'*4,slack=b'Z',head=40):
    base=struct.pack('<IiiHHIIiiII',head,w,h,1,bits,comp,img,0,0,used,0)
    return base+bytes(head-40)+extra+xor+mask+slack

def pair(name,b,expect):
    add(name+'-new',b,'IcoEntryDibSlack',expect)
    old={'error':'TrailingBytes'} if 'error' not in expect else expect
    add(name+'-old',b,'IcoEntryDib',old)

for alpha,mask,expected in [(0x80,b'\x80\0\0\0',0x80),
                            (0x7f,b'\x80\0\0\0',0x7f),
                            (0,b'\x80\0\0\0',0),
                            (0,b'\0'*4,255)]:
    pair(f'alpha32-{alpha:02x}-mask{mask[0]:02x}',
         raw(xor=bytes((0,0,255,alpha)),mask=mask),red(expected))
# Global alpha test: a nonzero second pixel prevents AND fallback for both.
pair('alpha32-global',raw(w=2,xor=bytes.fromhex('0000ff0000ff0080'),
    mask=bytes.fromhex('c0000000')),
    {'width':2,'height':1,'rgba_runs':[{'count':1,'hex':'ff000000'},
                                       {'count':1,'hex':'00ff0080'}]})
# BITFIELDS 32 with and without a fourth alpha mask.
external=struct.pack('<III',0x00ff0000,0x0000ff00,0x000000ff)
pair('fields32-no-alpha',raw(bits=32,comp=3,extra=external,
     xor=bytes.fromhex('0000ff00'),mask=bytes.fromhex('80000000')),red(0))
masks=external+struct.pack('<I',0xff000000)
pair('fields32-explicit-alpha',raw(bits=32,comp=3,head=56,extra=b'',
     xor=bytes.fromhex('0000ff80'),mask=bytes.fromhex('80000000'))[:40]+masks+
     raw(bits=32,comp=3,head=56,extra=b'',xor=bytes.fromhex('0000ff80'),mask=bytes.fromhex('80000000'))[56:],red(128))
# One-bit palette, 8-bit index and no-RLE rules all take the slack branch.
palette=bytes.fromhex('00000000 0000ff00')
pair('indexed1-wide8',raw(w=8,bits=1,extra=palette,
     xor=bytes.fromhex('ff000000'),slack=b'\0'*64),
     {'width':8,'height':1,'rgba_runs':[{'count':8,'hex':'ff0000ff'}]})
pair('indexed8-two',raw(w=2,bits=8,used=2,extra=palette,
     xor=bytes.fromhex('00010000')),
     {'width':2,'height':1,'rgba_runs':[{'count':1,'hex':'000000ff'},
                                       {'count':1,'hex':'ff0000ff'}]})
pair('indexed8-invalid-index',raw(bits=8,used=2,extra=palette,
     xor=bytes.fromhex('03000000')),
     {'error':'PaletteIndexOutOfRange'})
for bits,comp in ((8,1),(4,2)):
    pair(f'rle{bits}-forbidden',raw(bits=bits,comp=comp,extra=palette,
         xor=bytes.fromhex('01010001')),
         {'error':'UnsupportedOrientation'})
pair('high-depth-clrused',raw(bits=24,used=1,xor=bytes.fromhex('0000ff00')),
     {'error':'InvalidColorCount'})
pair('negative-height',raw(h=-2,bits=24,xor=bytes.fromhex('0000ff00')),
     {'error':'InvalidDimensions'})
pair('image-size-includes-slack',body(9)+b'Z',{'error':'InvalidImageSize'})
assert v[:214]==json.loads(OLD.read_text())
assert len(v)==len({x['name'] for x in v})
(ROOT/'VECTORS.json').write_text(json.dumps(v,indent=2)+'\n')
print('old',214,'new',len(v)-214,'total',len(v))
