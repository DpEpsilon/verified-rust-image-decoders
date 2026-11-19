#!/usr/bin/env python3
"""Group frozen streams under independently authored fixed-match and larger stored traces.

RGB pixels are the frozen encoder's TPX, recovered by replaying its RNG/encoder
prefix (not a decoder): (201,73,63),(202,75,151),(225,26,23),(252,246,129).
Row 0 uses filter 0; row 1 uses Sub (filter 1), giving the raw below.
Palette samples are authored (0,1),(2,1): filter-0 [0,0,1], Sub [1,2,255].
The one-pixel packed rows have filter 0 and authored packed byte 0 or 128.
No expected result field, decompressor or executable decode table is consulted.
"""
import json
import random
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
V = json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())
def chunks(data):
    p = 8
    while p+12 <= len(data):
        n = int.from_bytes(data[p:p+4],'big')
        if p+12+n > len(data) or n > 1<<28: return
        yield data[p+4:p+8], data[p+8:p+8+n]
        p += n+12
def streams(data):
    idat = None; frames = []
    for k,b in chunks(data):
        if k == b'IDAT':
            if idat is None: idat = []
            idat.append(b)
        elif k == b'fcTL': frames.append(None)
        elif k == b'fdAT' and frames and len(b) >= 4:
            if frames[-1] is None: frames[-1] = []
            frames[-1].append(b[4:])
    return [b''.join(p) for p in ([] if idat is None else [idat])+[f for f in frames if f is not None]]
def seq(xs,ty='u8'):
    return 'seq!['+', '.join(str(x)+(ty if i == 0 else '') for i,x in enumerate(xs))+']'
# Representative names select input bytes only. Expected raw bytes are authored.
CASES = [('rgb_2x2','container/minimal-ok',[0,201,73,63,202,75,151,1,225,26,23,27,220,106]),
         ('palette_2x2','plte/ok',[0,0,1,1,2,255]),
         ('packed_zero','ok/ct0-d1-il0-1x1',[0,0]),
         ('packed_high','ok/ct0-d1-il1-1x1',[0,128])]
def image_r(data):
    hs = [b for k,b in chunks(data) if k == b'IHDR' and len(b) == 13]
    if len(hs) != 1: return None
    h = hs[0]; w = int.from_bytes(h[:4],'big'); height = int.from_bytes(h[4:8],'big')
    d,ct,cm,fm,il = h[8:]; channels = {0:1,2:3,3:1,4:2,6:4}.get(ct)
    legal = {0:(1,2,4,8,16),2:(8,16),3:(1,2,4,8),4:(8,16),6:(8,16)}
    if not (0 < w < 1<<31 and 0 < height < 1<<31 and ct in legal and d in legal[ct] and cm == fm == 0 and il in (0,1)): return None
    grids = [(0,0,1,1)] if il == 0 else [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
    size = 0
    for x,y,dx,dy in grids:
        pw = max(0,(w+dx-1-x)//dx); ph = max(0,(height+dy-1-y)//dy)
        if pw and ph: size += ph*(1+(pw*channels*d+7)//8)
    return size

def pack(values,d):
    if d == 16: return b''.join(v.to_bytes(2,'big') for v in values)
    if d == 8: return bytes(values)
    out = []; a = n = 0
    for v in values:
        a = (a<<d)|v; n += d
        if n == 8: out.append(a); a = n = 0
    if n: out.append(a<<(8-n))
    return bytes(out)

def scan_intent(px,w,h,ct,d,il,filters=None):
    channels = {0:1,2:3,3:1,4:2,6:4}[ct]; bpp = max(1,channels*d//8)
    grids = [(0,0,1,1)] if il == 0 else [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
    out = []; rownum = 0
    for x0,y0,dx,dy in grids:
        xs = list(range(x0,w,dx)); ys = list(range(y0,h,dy))
        if not xs or not ys: continue
        prev = bytes(len(pack([0]*(len(xs)*channels),d)))
        for y in ys:
            row = pack([v for x in xs for v in px[y*w+x]],d); ft = rownum%5 if filters is None else filters; out.append(ft)
            for i,v in enumerate(row):
                a = row[i-bpp] if i >= bpp else 0; b = prev[i]; c = prev[i-bpp] if i >= bpp else 0
                pa,pb,pc = abs(b-c),abs(a-c),abs(a+b-2*c)
                predictor = [0,a,b,(a+b)//2,a if pa <= pb and pa <= pc else b if pb <= pc else c][ft]
                out.append((v-predictor)&255)
            prev = row; rownum += 1
    return out

def literal_body(raw):
    # Forward RFC fixed-code encoding is an eligibility check, not a decoder.
    bits = [1,1,0]
    for s in raw+[256]:
        width = 8 if s < 144 else 9 if s < 256 else 7
        word = 48+s if s < 144 else 256+s if s < 256 else 0
        bits += [(word>>i)&1 for i in range(width-1,-1,-1)]
    return bytes(sum(v<<j for j,v in enumerate(bits[i:i+8])) for i in range(0,len(bits),8))

def stored_body(raw):
    n = len(raw)
    return bytes([1])+n.to_bytes(2,'little')+(65535-n).to_bytes(2,'little')+bytes(raw)


# RFC forward encoder intent, not executable inflate tables.
LBASE=[3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258]
LEXTRA=[0]*8+[1]*4+[2]*4+[3]*4+[4]*4+[5]*4+[0]
DBASE=[1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577]
DEXTRA=[0]*4+[i for i in range(1,14) for _ in range(2)]
def codes(n,d):
    li=max(i for i,b in enumerate(LBASE) if b<=n);di=max(i for i,b in enumerate(DBASE) if b<=d)
    return 257+li,n-LBASE[li],LEXTRA[li],di,d-DBASE[di],DEXTRA[di]
def encode(tokens):
    bits=[1,1,0]
    def field(v,n):bits.extend((v>>i)&1 for i in range(n))
    def code(s):
        n=8 if s<144 else 9 if s<256 else 7 if s<280 else 8
        w=48+s if s<144 else 256+s if s<256 else s-256 if s<280 else s-88
        bits.extend((w>>i)&1 for i in range(n-1,-1,-1))
    for t in tokens:
        if isinstance(t,int):code(t)
        else:
            s,le,ln,ds,de,dn=codes(*t);code(s);field(le,ln)
            bits.extend((ds>>i)&1 for i in range(4,-1,-1));field(de,dn)
    code(256)
    return bytes(sum(v<<j for j,v in enumerate(bits[i:i+8])) for i in range(0,len(bits),8))
def match(raw,p,oldest,origin=0):
    best=(0,0)
    for d in range(1,min(p-origin,32768)+1):
        n=0
        while n<258 and p+n<len(raw) and raw[p+n]==raw[p+n-d]:n+=1
        if n>=3 and (n>best[0] or oldest and n==best[0]):best=(n,d)
    return best

def greedy(raw,prefix,lazy,oldest,origin=0):
    ts=[];p=0
    while p<len(raw):
        n,d=match(raw,p,oldest,origin) if p>=prefix else (0,0)
        if n and lazy and match(raw,p+1,oldest,origin)[0]>n:n=0
        if n:ts.append((n,d));p+=n
        else:ts.append(raw[p]);p+=1
    return ts

def rle(raw):
    ts=[];p=0
    while p<len(raw):
        n=0
        while p>0 and n<258 and p+n<len(raw) and raw[p+n]==raw[p+n-1]:n+=1
        if n>=3:ts.append((n,1));p+=n
        else:ts.append(raw[p]);p+=1
    return ts

# Each fdAT stream uses the dimensions of its own enclosing fcTL, not the canvas.
def stream_rs(data):
    hs = [b for k,b in chunks(data) if k == b'IHDR' and len(b) == 13]
    if len(hs) != 1: return [None]*len(streams(data))
    head = hs[0]; has_idat = False; frames = []
    for k,b in chunks(data):
        if k == b'IDAT': has_idat = True
        elif k == b'fcTL': frames.append([b,False])
        elif k == b'fdAT' and len(b) >= 4 and frames: frames[-1][1] = True
    rs = [image_r(data)] if has_idat else []
    for ctl,present in frames:
        if not present: continue
        if len(ctl) != 26: rs.append(None); continue
        # Reuse only the shape calculation, with an independently formed IHDR.
        shape = ctl[4:12]+head[8:]
        rs.append(image_r(bytes(8)+bytes([0,0,0,13])+b'IHDR'+shape+bytes(4)))
    return rs

CASES = []; TRACES = {}; REPS = {}; INTENTS = {}
base = json.loads((ROOT/'tools/witness_classes.json').read_text())
covered = {tuple(member) for c in base for member in c['members']}
for source in ('witnesses.rs','witness_cases.rs','witness_dynamic_cases.rs','witness_d4.rs','witness_overlap.rs','witness_invalid_dynamic.rs','witness_repeats.rs','witness_single_literal.rs'):
    for line in (ROOT/'src'/source).read_text().splitlines():
        if line.startswith('/// ') and ':' in line: covered.add((line[4:].split(':')[0],0))
known = set()
for c in base:
    name,index = c['members'][0]
    known.add(streams(bytes.fromhex(next(v['input_hex'] for v in V if v['name']==name)))[index])

def consider(ident,name,raw,intent,index=0):
    data = bytes.fromhex(next(v['input_hex'] for v in V if v['name']==name)); z = streams(data)[index]
    if z in known: return
    tokens = None
    if z[2:-4] != stored_body(raw):
        # Each candidate depends on raw and these fixed forward rules only.
        # Comparing its complete candidate DEFLATE-body encoding with Z never supplies expected bytes.
        for lazy in (False,True):
            for origin in (0,1):
                trial = greedy(raw,1,lazy,False,origin)
                if encode(trial) == z[2:-4]: tokens = trial; break
            if tokens is not None: break
        if tokens is None:
            trial = rle(raw)
            if encode(trial) == z[2:-4]: tokens = trial
        if tokens is None: return
    members = [(v['name'],i) for v in V for i,s in enumerate(streams(bytes.fromhex(v['input_hex'])))
               if s == z and (v['name'],i) not in covered and stream_rs(bytes.fromhex(v['input_hex']))[i] == len(raw)]
    if not members: return
    CASES.append((ident,name,raw)); TRACES[ident]=tokens; REPS[ident]=index; INTENTS[ident]=intent
    known.add(z); covered.update(members)

def rand_px(w,h,ct,d,n=None):
    ch = {0:1,2:3,3:1,4:2,6:4}[ct]
    return [tuple(rng.randrange(n if ct==3 else 1<<d) for _ in range(ch)) for _ in range(w*h)]

# Replay the complete initial sample schedule, then filters and tRNS. No frozen
# encoder code is imported or executed; all sample/filter operations are above.
rng = random.Random(20261114)
combos = [(0,1),(0,2),(0,4),(0,8),(0,16),(2,8),(2,16),(3,1),(3,2),(3,4),(3,8),(4,8),(4,16),(6,8),(6,16)]
for ct,d in combos:
    for il in (0,1):
        for w,h in [(1,1),(2,3),(7,5),(9,10),(17,2)]:
            n = min(1<<d,1+rng.randrange(256)) if ct==3 else None
            if ct==3:
                for _ in range(3*n): rng.randrange(256)
            px=rand_px(w,h,ct,d,n); name=f'ok/ct{ct}-d{d}-il{il}-{w}x{h}'
            consider(f'extended_ct{ct}_d{d}_il{il}_{w}x{h}',name,scan_intent(px,w,h,ct,d,il),{'schedule':'initial','shape':[w,h,ct,d,il]})
for ft in range(5):
    for ct,d in [(0,1),(0,8),(0,16),(2,8),(2,16),(4,8),(6,16)]:
        px=rand_px(6,4,ct,d); name=f'filter/{ft}-ct{ct}-d{d}'
        consider(f'filter_{ft}_ct{ct}_d{d}',name,scan_intent(px,6,4,ct,d,0,ft),{'schedule':'filters','filter':ft,'shape':[6,4,ct,d,0]})
for d in (1,2,4,8,16):
    px=rand_px(5,3,0,d)
    consider(f'trns_grey_d{d}',f'trns/grey-d{d}',scan_intent(px,5,3,0,d,0),{'schedule':'trns-grey','shape':[5,3,0,d,0]})
for d in (8,16):
    px=rand_px(4,4,2,d); px[5]=px[9]=px[0]
    consider(f'trns_rgb_d{d}',f'trns/rgb-d{d}',scan_intent(px,4,4,2,d,0),{'schedule':'trns-rgb','shape':[4,4,2,d,0]})
for d,n,tn in [(8,16,3),(8,16,16),(4,16,1),(1,2,2)]:
    for _ in range(3*n+tn):rng.randrange(256)
    px=rand_px(4,3,3,d,n)
    consider(f'trns_palette_d{d}_n{tn}',f'trns/palette-d{d}-{n}-entries-{tn}-alpha',scan_intent(px,4,3,3,d,0),{'schedule':'trns-palette','shape':[4,3,3,d,0]})
vals=[0,127,128,383,384,32767,32768,32895,32896,65407,65408,65535]
consider('rounding_boundaries','depth16/rounding-boundaries',scan_intent([(v,) for v in vals],12,1,0,16,0,0),{'schedule':'literal-boundaries','samples':vals})
# Continue the encoder RNG through BASE_PX and rawW to the edge-case schedule.
base_px=rand_px(23,11,6,8); base_px[40:80]=[base_px[40]]*40
base_raw=scan_intent(base_px,23,11,6,8,0)
# A single stored BASE_RAW encoding also covers all byte-identical level-zero
# and Huffman-only records; large queried instances stay subject to default limits.
consider('base_stored','zlib/level0-default',base_raw,{'schedule':'base-image','shape':[23,11,6,8,0]})
for ident,name in [('base_fixed','zlib/level6-default'),('base_level9','zlib/level9-default'),('base_rle','zlib/level6-rle')]:
    consider(ident,name,base_raw,{'schedule':'base-image','shape':[23,11,6,8,0]})
window_raw = [0]+[rng.randrange(256) for _ in range(600)]
for _ in range(12+9):rng.randrange(256)
for w,h in [(1,1),(1,2),(2,1),(1,5),(5,1),(2,2),(3,3),(4,4),(5,5),(8,8),(9,1),(1,9)]:
    for ct,d in [(0,1),(6,16),(3,4)]:
        n=16 if ct==3 else None
        if ct==3:
            for _ in range(48):rng.randrange(256)
        px=rand_px(w,h,ct,d,n)
        consider(f'edge_ct{ct}_d{d}_{w}x{h}',f'interlace/ct{ct}-d{d}-{w}x{h}',scan_intent(px,w,h,ct,d,1),{'schedule':'interlace-edge','shape':[w,h,ct,d,1]})
# Explicit sample/filter intentions from malformed palette containers; the zlib
# entry still has a legal image-size R even though the PNG call rejects indices.
for ident,name,px,w,h,ct,d in [
    ('palette_bad_index','image/palette-index-out-of-range',[(0,),(1,),(2,),(3,),(4,),(0,)],3,2,3,8),
    ('palette_bad_index_d2','image/palette-index-out-of-range-d2',[(0,),(1,),(3,),(0,),(0,),(0,)],3,2,3,2),
    ('grey_ones','plte/in-greyscale',[(1,)]*4,2,2,0,8),
    ('grey_alpha_constant','plte/in-grey-alpha',[(1,2)]*4,2,2,4,8),
    ('grey_trns_length','trns/grey-length-6',[(1,)]*4,2,2,0,8),
    ('rgba_zero','trns/in-rgba',[(0,0,0,0)],1,1,6,8),
    ('ga_zero','trns/in-grey-alpha',[(0,0)],1,1,4,8)]:
    consider(ident,name,scan_intent(px,w,h,ct,d,0),{'schedule':'explicit-samples','shape':[w,h,ct,d,0],'samples':px})
consider('bad_index_before_filter','precedence/index-row0-before-filter-row1',[0,0,9,7,0,0],{'schedule':'explicit-raw','raw-intent':[0,0,9,7,0,0]})

# Repeated APNG streams: samples are authored directly from frame intent.
name='apng/default-is-frame0-dispose-bg-blend-over'
for ident,px,w,h,index in [('red_canvas',[(255,0,0,255)]*12,4,3,0),('green_frame',[(0,255,0,255)]*4,2,2,1),('half_blue_frame',[(0,0,255,128)]*2,2,1,2)]:
    consider(ident,name,scan_intent(px,w,h,6,8,0),{'schedule':'apng-authored','shape':[w,h,6,8,0],'samples':px},index)
FA=[(10,200,30,0),(10,200,30,1),(10,200,30,127),(10,200,30,254),(200,10,99,64),(1,2,3,200)]
FB=[(250,5,77,a) for a in (0,255,128,3,200,90)]
for ident,px,index in [('blend_operand_a',FA,2),('blend_operand_b',FB,1)]:
    consider(ident,'apng/blend-over-arithmetic',scan_intent(px,3,2,6,8,0),{'schedule':'apng-authored','shape':[3,2,6,8,0],'samples':px},index)

# All remaining successful APNG operands are authored before composition.
ts=[(200,100,50,0),(0,0,0,0),(255,255,255,0),(1,2,3,0)]
dest=[(10,20,30,255),(10,20,30,128),(10,20,30,1),(255,255,255,255)]
operands=[('transparent_source',[(dest[i%4],ts[i//3%4]) for i in range(12)]),
 ('transparent_destination',[((10*(i+1)%256,99-i,7*i,0),(200-i,50+i,3*i,a)) for i,a in enumerate((1,2,64,127,128,129,200,253,254,1,128,254))]),
 ('zero_result_alpha',[((10*(i+1)%256,99-i,7*i,0),(200-i,50+i,3*i,0)) for i in range(9)]+[((0,0,0,0),(0,0,0,0)),((255,255,255,0),(255,255,255,0)),((0,0,0,0),(9,9,9,0))])]
for ident,pairs in operands:
    for side in (0,1):
        px=[p[side] for p in pairs]
        consider(f'apng_{ident}_{side}',f'apng/blend-over-{ident.replace("_","-")}',scan_intent(px,4,3,6,8,0),{'schedule':'apng-authored','shape':[4,3,6,8,0],'samples':px},side+1)
for ident,px,w,h,index in [('interlaced_red',[(255,0,0,255)]*12,4,3,0),('interlaced_subframe',[(255,0,0,255),(0,255,0,255),(0,0,255,255),(0,0,255,128)]*2+[(255,0,0,255)],3,3,1)]:
    consider(ident,'apng/interlaced-subframe',scan_intent(px,w,h,6,8,1),{'schedule':'apng-authored','shape':[w,h,6,8,1],'samples':px},index)
consider('one_pixel_apng','apng/empty-fdat-payload',scan_intent([(10,20,30,255)],1,1,6,8,0),{'schedule':'apng-authored','shape':[1,1,6,8,0],'samples':[(10,20,30,255)]})

def emit():
    # Emit separate constructors so large literal-array proofs never share a solver
    # query with the public-call refinement. Expected raw remains a ghost sequence.
    lines=['//! Independently authored fixed-match and larger stored stream classes.',
           'use crate::{model as m, witness_match as t, witness_checksum as c, witness_literal_class::stored_payload, witness_layout::replay_layout, witnesses::check_at, Status};',
           'use vstd::prelude::*;', 'verus! {']
    claims=[]
    base_keys={tuple(member) for c in base for member in c['members']}
    # Manual witness keys are excluded independently of this generated manifest.
    base_keys.update((line[4:].split(':')[0],0) for source in ('witnesses.rs','witness_cases.rs','witness_dynamic_cases.rs','witness_d4.rs','witness_overlap.rs','witness_invalid_dynamic.rs','witness_repeats.rs','witness_single_literal.rs') for line in (ROOT/'src'/source).read_text().splitlines() if line.startswith('/// ') and ':' in line)
    for ident,name,raw in CASES:
        index=REPS[ident]; data=bytes.fromhex(next(v['input_hex'] for v in V if v['name']==name));z=streams(data)[index]
        tokens=TRACES[ident]; zs=seq(z);rs=seq(raw);n=len(raw)
        members=[]
        for v in V:
            d=bytes.fromhex(v['input_hex']); ss=streams(d);rr=stream_rs(d)
            for i,s in enumerate(ss):
                if s==z and rr[i]==n and (v['name'],i) not in base_keys:members.append([v['name'],i]);base_keys.add((v['name'],i))
        if tokens is not None and n > 527:
            steps=[];pos=19;before=0
            for token in tokens+[256]:
                if isinstance(token,int):s=token;le=ds=de=length=distance=0;nextpos=pos+(8 if s<144 else 9 if s<256 else 7);after=before+(s<256)
                else:
                    length,distance=token;s,le,ln,ds,de,dn=codes(length,distance);nextpos=pos+(7 if s<280 else 8)+ln+5+dn;after=before+length
                steps.append({'pos':pos,'next':nextpos,'before':before,'after':after,'symbol':s,'le':le,'ds':ds,'de':de,'length':length,'distance':distance})
                pos=nextpos;before=after
            end=pos;ts='seq!['+', '.join('t::Step {'+', '.join(f'{k}:{v}' for k,v in step.items())+'}' for step in steps)+']'
            sb=sum(raw);sp=sum((n-i)*v for i,v in enumerate(raw))+n;adler=(sp%65521)*65536+(1+sb)%65521
            lines += [f'spec fn {ident}_z() -> Seq<u8> {{ {zs} }}',f'spec fn {ident}_raw() -> Seq<u8> {{ {rs} }}',
                      f'spec fn {ident}_ts() -> Seq<t::Step> {{ {ts} }}',
                      f'fn {ident}_bytes() -> (wire: [u8;{len(z)}])',f'    ensures wire@ == {ident}_z(),','{',
                      '    ['+', '.join(hex(x)+('u8' if i==0 else '') for i,x in enumerate(z))+']','}',
                      '#[verifier::spinoff_prover]',f'proof fn {ident}_trace()',
                      f'    ensures {ident}_z().len() == {len(z)}, {ident}_raw().len() == {n}, {ident}_ts().len() == {len(steps)},',
                      f'        {ident}_ts()[0].pos == 19, {ident}_ts()[0].before == 0, {ident}_ts().last().next == {end},',
                      f'        t::trace({ident}_z(),{ident}_raw(),{ident}_ts(),0,{len(steps)}),','{',
                      f'    assert(t::trace({zs},{rs},{ts},0,{len(steps)})) by (compute);','}',
                      '#[verifier::spinoff_prover]',f'proof fn {ident}_fields()',
                      f'    ensures {ident}_z()[0] == 120, {ident}_z()[1] == {z[1]},',
                      f'        m::field_value({ident}_z(),16,1) == 1, m::field_value({ident}_z(),17,2) == 1,',
                      f'        m::be_value({ident}_z(),{(end+7)//8},4) == {adler},','{',
                      f'    assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 1) by (compute);',
                      f'    assert(m::be_value({zs},{(end+7)//8},4) == {adler}) by (compute);','}',
                      '#[verifier::spinoff_prover]',f'proof fn {ident}_checksum()',f'    ensures m::Adler({ident}_raw()) == {adler},','{',
                      '    hide(m::sum_bytes); hide(m::sum_prefixes);',f'    let raw = {ident}_raw();',
                      f'    assert(c::sums({rs},0,{n}) == ({sb}nat,{sp}nat)) by (compute);',
                      f'    c::sums_correct(raw,0,{n}); assert(raw.subrange(0,{n}) =~= raw);',
                      '    assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',
                      f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute);','}',
                      '#[verifier::spinoff_prover]',f'fn {ident}() {{',f'    hide({ident}_z); hide({ident}_raw); hide({ident}_ts);',
                      f'    let wire = {ident}_bytes();','    proof {',f'        {ident}_trace(); {ident}_fields(); {ident}_checksum();',
                      f'        crate::witness_stored_bridge::fixed_stream({ident}_z(),{ident}_raw(),{ident}_ts(),{n},{z[1]},{adler});',
                      f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',
                      f'    let ghost raw = {ident}_raw(); let mut output = [0xa5u8;{n}]; check_at(&wire,&mut output,Status::Done,Ghost(raw));','}']
            claims.append({'class':ident,'encoding':'fixed-matches','raw':raw,'representative':[name,index],'intent':INTENTS[ident],'steps':steps,'members':members,'decomposed':'fixed'})
            continue
        if tokens is None and n > 527:
            # Separate concrete payload and checksum facts from model/public calls.
            end = 8*(7+n)
            sb = sum(raw); sp = sum((n-i)*v for i,v in enumerate(raw))+n
            adler = (sp%65521)*65536+(1+sb)%65521
            lines += [f'spec fn {ident}_z() -> Seq<u8> {{ {zs} }}',
                      f'spec fn {ident}_raw() -> Seq<u8> {{ {rs} }}',
                      f'fn {ident}_bytes() -> (wire: [u8;{len(z)}])',
                      f'    ensures wire@ == {ident}_z(),', '{',
                      '    ['+', '.join(hex(x)+('u8' if i==0 else '') for i,x in enumerate(z))+']', '}',
                      '#[verifier::spinoff_prover]', f'proof fn {ident}_payload()',
                      f'    ensures {ident}_z().len() == {len(z)}, {ident}_raw().len() == {n},',
                      f'        {ident}_z().subrange(7,{7+n}) == {ident}_raw(),', '{',
                      f'    assert({zs}.subrange(7,{7+n}) == {rs}) by (compute);', '}',
                      '#[verifier::spinoff_prover]', f'proof fn {ident}_fields()',
                      f'    ensures {ident}_z()[3] as nat + 256*{ident}_z()[4] as nat == {n},',
                      f'        {ident}_z()[5] as nat + 256*{ident}_z()[6] as nat == {65535-n},',
                      f'        m::field_value({ident}_z(),24,16) == {n},',
                      f'        m::field_value({ident}_z(),40,16) == {65535-n},',
                      f'        m::field_value({ident}_z(),16,1) == 1, m::field_value({ident}_z(),17,2) == 0,',
                      f'        m::be_value({ident}_z(),0,1) == {z[0]}, m::be_value({ident}_z(),1,1) == {z[1]},',
                      f'        m::be_value({ident}_z(),{7+n},4) == {adler},', '{']
            for start,width,value in [(24,16,n),(40,16,65535-n),(16,1,1),(17,2,0)]:
                lines.append(f'    assert(m::field_value({zs},{start},{width}) == {value}) by (compute);')
            for start,width,value in [(0,1,z[0]),(1,1,z[1]),(7+n,4,adler)]:
                lines.append(f'    assert(m::be_value({zs},{start},{width}) == {value}) by (compute);')
            lines += ['}', '#[verifier::spinoff_prover]', f'proof fn {ident}_checksum()',
                      f'    ensures m::Adler({ident}_raw()) == {adler},', '{',
                      '    hide(m::sum_bytes); hide(m::sum_prefixes);',
                      f'    let raw = {ident}_raw();',
                      f'    assert(c::sums({rs},0,{n}) == ({sb}nat,{sp}nat)) by (compute);',
                      f'    c::sums_correct(raw,0,{n}); assert(raw.subrange(0,{n}) =~= raw);',
                      '    assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',
                      f'    assert(({sp}nat % 65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute);', '}',
                      '#[verifier::spinoff_prover]', f'fn {ident}() {{',
                      f'    hide({ident}_z); hide({ident}_raw); hide(m::sum_bytes); hide(m::sum_prefixes);',
                      f'    let wire = {ident}_bytes();', '    proof {',
                      f'        {ident}_payload(); {ident}_fields(); {ident}_checksum();',
                      f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);',
                      f'        crate::witness_stored_bridge::stored_stream({ident}_z(),{ident}_raw(),{n},{z[0]},{z[1]},{adler});',
                      '    }', f'    let ghost raw = {ident}_raw();',
                      f'    let mut output = [0xa5u8;{n}]; check_at(&wire,&mut output,Status::Done,Ghost(raw));', '}']
            claims.append({'class':ident,'encoding':'stored','raw':raw,'representative':[name,index],'intent':INTENTS[ident],'steps':[],'members':members,'decomposed':True})
            continue
        lines += [f'fn {ident}_bytes() -> (wire: [u8;{len(z)}])', f'    ensures wire@ == {zs},', '{', '    ['+', '.join(hex(x)+('u8' if i==0 else '') for i,x in enumerate(z))+']','}',
                  '#[verifier::spinoff_prover]',f'fn {ident}() {{',
                  '    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep); hide(m::MStoredPayload); hide(m::sum_bytes); hide(m::sum_prefixes);',
                  f'    let wire = {ident}_bytes();', '    proof {',f'        let z = {zs}; let raw = {rs};',
                  '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);']
        steps=[]
        if tokens is None:
            encoding='stored';end=8*(7+n)
            lines += [f'        assert(z.subrange(7,{7+n}) =~= raw);','        stored_payload(z,raw);',
                      f'        assert(m::field_value({zs},24,16) == {n}) by (compute);',
                      f'        assert(m::field_value({zs},40,16) == {65535-n}) by (compute);',
                      f'        assert(m::MStoredBlock(z,19,Seq::empty(),{n}) == (m::StoredEffect::End{{out:raw,position:{end}}}));']
        else:
            encoding='fixed-matches' if any(isinstance(t,tuple) for t in tokens) else 'fixed-literals';pos=19;before=0
            for token in tokens+[256]:
                if isinstance(token,int):s=token;le=ds=de=length=distance=0;nextpos=pos+(8 if s<144 else 9 if s<256 else 7);after=before+(s<256)
                else:
                    length,distance=token;s,le,ln,ds,de,dn=codes(length,distance);nextpos=pos+(7 if s<280 else 8)+ln+5+dn;after=before+length
                steps.append({'pos':pos,'next':nextpos,'before':before,'after':after,'symbol':s,'le':le,'ds':ds,'de':de,'length':length,'distance':distance})
                pos=nextpos;before=after
            end=pos;ts='seq!['+', '.join('t::Step {'+', '.join(f'{k}:{v}' for k,v in step.items())+'}' for step in steps)+']'
            lines += [f'        let ts = {ts};',f'        assert(t::trace({zs},{rs},{ts},0,{len(steps)})) by (compute);',
                      f'        t::effects(z,raw,ts,0,{len(steps)}); t::expand(z,raw,ts,0);','        assert(raw.take(0) =~= Seq::<u8>::empty());',
                      f'        assert(m::MCompressed(z,19,Seq::empty(),{n},32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{{out:raw,position:{end}}}));']
        sb=sum(raw);sp=sum((n-i)*v for i,v in enumerate(raw))+n;adler=(sp%65521)*65536+(1+sb)%65521
        lines += [f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == {0 if tokens is None else 1}) by (compute);',
                  f'        assert(m::MBlk(z,16,Seq::empty(),{n},32768,true) == (m::StoredEffect::End{{out:raw,position:{end}}}));',
                  f'        assert(c::sums({rs},0,{n}) == ({sb}nat,{sp}nat)) by (compute);',f'        c::sums_correct(raw,0,{n});',f'        assert(raw.subrange(0,{n}) =~= raw);',
                  '        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX+1) by (compute);',f'        assert(m::sum_bytes(raw) == {sb}); assert(m::sum_prefixes(raw) == {sp});', f'        assert(({sp}nat % 65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521 == {adler}) by (compute);', f'        assert(m::Adler(raw) == {adler});',
                  f'        assert(m::be_value({zs},{(end+7)//8},4) == {adler}) by (compute);',
                  f'        assert(m::MFinish(z,{end},raw,{n},false) == (m::StoredEffect::End{{out:raw,position:{8*len(z)}}}));',
                  f'        assert(m::Inflate(z,{n},true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,{n},true,false).out == raw);',
                  f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);','    }',
                  f'    let ghost raw = {rs}; let mut output = [0xa5u8;{n}]; check_at(&wire,&mut output,Status::Done,Ghost(raw));','}']
        claims.append({'class':ident,'encoding':encoding,'raw':raw,'representative':[name,index],'intent':INTENTS[ident],'steps':steps,'members':members})
    layouts=[]
    for suffix in ('split-1-byte','split-irregular','zero-length-idats'):
        name='idat/'+suffix; ident='layout_'+suffix.replace('-','_')
        data=bytes.fromhex(next(v['input_hex'] for v in V if v['name']==name))
        parts=[b for k,b in chunks(data) if k==b'IDAT']; cuts=[0]
        for part in parts:cuts.append(cuts[-1]+len(part))
        assert b''.join(parts) == streams(bytes.fromhex(next(v['input_hex'] for v in V if v['name']=='zlib/level6-default')))[0]
        base_claim=next(c for c in claims if c['class']=='base_fixed');raw=base_claim['raw'];n=len(raw);cs=seq(cuts,'usize')
        checksum=(sum((n-i)*v for i,v in enumerate(raw))+n)%65521*65536+(1+sum(raw))%65521
        lines += [f'spec fn {ident}_cuts() -> Seq<usize> {{ {cs} }}',
                  f'fn {ident}_cuts_bytes() -> (cuts: [usize;{len(cuts)}])',f'    ensures cuts@ == {ident}_cuts(),','{',
                  '    ['+', '.join(str(x)+('usize' if i==0 else '') for i,x in enumerate(cuts))+']','}',
                  '#[verifier::spinoff_prover]',f'proof fn {ident}_cut_facts()',
                  f'    ensures {ident}_cuts().len() == {len(cuts)}, {ident}_cuts()[0] == 0, {ident}_cuts().last() == {cuts[-1]},',
                  f'        forall|j:int| 0 <= j < {len(parts)} ==> #[trigger] {ident}_cuts()[j] <= {ident}_cuts()[j+1] <= {cuts[-1]},','{',
                  f'    assert(crate::witness_layout::cuts_ok({cs},0,{len(parts)},{cuts[-1]})) by (compute);',
                  f'    crate::witness_layout::cuts_quant({ident}_cuts(),0,{len(parts)},{cuts[-1]});','}',
                  '#[verifier::spinoff_prover]',f'fn {ident}() {{',
                  f'    hide(base_fixed_z); hide(base_fixed_raw); hide(base_fixed_ts); hide({ident}_cuts);',
                  '    let wire = base_fixed_bytes();',f'    let cuts = {ident}_cuts_bytes();',
                  '    proof {',f'        {ident}_cut_facts(); base_fixed_trace(); base_fixed_fields(); base_fixed_checksum();',
                  f'        crate::witness_stored_bridge::fixed_stream(base_fixed_z(),base_fixed_raw(),base_fixed_ts(),{n},156,{checksum});',
                  f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(b"".join(parts))}) by (compute);','    }',
                  f'    let mut parts: [&[u8];{len(parts)}] = [&wire[0..0];{len(parts)}];',
                  f'    let ghost raw = base_fixed_raw(); let mut output = [0xa5u8;{n}];',
                  '    replay_layout(&wire,&cuts,&mut parts,&mut output,Ghost(raw));','}']
        layouts.append({'class':ident,'record':name,'r':n,'raw':raw,'cuts':cuts,'segment_lengths':[len(p) for p in parts]})
    (ROOT/'tools/witness_layouts.json').write_text(json.dumps(layouts,indent=2)+'\n')
    lines+=['} // verus!','']
    (ROOT/'src/witness_extended.rs').write_text('\n'.join(lines))
    (ROOT/'tools/witness_extended.json').write_text(json.dumps(claims,indent=2)+'\n')
    print('Authored',len(claims),'extended classes covering',sum(len(c['members']) for c in claims),'new frozen streams')
    print('Kinds', {kind:sum(c['encoding']==kind for c in claims) for kind in ('stored','fixed-matches','fixed-literals')})

if __name__ == "__main__":
    emit()
