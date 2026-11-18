#!/usr/bin/env python3
"""Group frozen streams under independently authored literal/stored traces.

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

def scan_intent(px,w,h,ct,d,il):
    channels = {0:1,2:3,3:1,4:2,6:4}[ct]; bpp = max(1,channels*d//8)
    grids = [(0,0,1,1)] if il == 0 else [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
    out = []; rownum = 0
    for x0,y0,dx,dy in grids:
        xs = list(range(x0,w,dx)); ys = list(range(y0,h,dy))
        if not xs or not ys: continue
        prev = bytes(len(pack([0]*(len(xs)*channels),d)))
        for y in ys:
            row = pack([v for x in xs for v in px[y*w+x]],d); ft = rownum%5; out.append(ft)
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

# Independently replay only the frozen encoder's initial RNG/sample schedule.
# No frozen script is executed or imported, and no decoder is called.
rng = random.Random(20261114)
combos = [(0,1),(0,2),(0,4),(0,8),(0,16),(2,8),(2,16),(3,1),(3,2),(3,4),(3,8),(4,8),(4,16),(6,8),(6,16)]
sizes = [(1,1),(2,3),(7,5),(9,10),(17,2)]
known_wires = {streams(bytes.fromhex(next(v for v in V if v['name'] == name)['input_hex']))[0] for _,name,_ in CASES}
for ct,d in combos:
    channels = {0:1,2:3,3:1,4:2,6:4}[ct]
    for il in (0,1):
        for w,h in sizes:
            n = min(1<<d,1+rng.randrange(256)) if ct == 3 else None
            if ct == 3:
                for _ in range(3*n): rng.randrange(256)
            px = [tuple(rng.randrange(n if ct == 3 else 1<<d) for _ in range(channels)) for _ in range(w*h)]
            raw = scan_intent(px,w,h,ct,d,il)
            name = f'ok/ct{ct}-d{d}-il{il}-{w}x{h}'
            z = streams(bytes.fromhex(next(v for v in V if v['name'] == name)['input_hex']))[0]
            if len(raw) <= 128 and z[2:-4] in (literal_body(raw), stored_body(raw)) and z not in known_wires:
                CASES.append((f'intent_ct{ct}_d{d}_il{il}_{w}x{h}',name,raw)); known_wires.add(z)

lines = ['//! Frozen byte-identical classes; membership is checked by audit_witnesses.py.', 'use crate::{model as m, witness_literal_class::*, witnesses::check_at, Status};', 'use vstd::prelude::*;', 'verus! {']
manifest = []
for ident,name,raw in CASES:
    start = len(lines)
    rep = next(v for v in V if v['name'] == name); data = bytes.fromhex(rep['input_hex']); z = streams(data)[0]
    # Membership requires identical Z and the independently calculated image R;
    # malformed IHDR shapes and non-IDAT frame streams receive no class credit.
    ihdr = next(b for k,b in chunks(data) if k == b'IHDR' and len(b) == 13)
    members = []
    for v in V:
        d = bytes.fromhex(v['input_hex'])
        if image_r(d) != len(raw): continue
        for index,s in enumerate(streams(d)):
            if index == 0 and s == z: members.append([v['name'],index])
    pos = [19]
    for s in raw+[256]: pos.append(pos[-1]+(8 if s<144 else 9 if s<256 else 7))
    zs = seq(z); rs = seq(raw); ps = seq(pos,'nat'); effect = f'(m::StoredEffect::End{{out:{rs},position:{pos[-1]}}})'
    lines += [f'// Class {ident}: {len(members)} independently audited frozen streams; R={len(raw)}.', '#[verifier::spinoff_prover]', f'fn {ident}() {{', '    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);', f'    let wire = [{", ".join(hex(x)+("u8" if i == 0 else "") for i,x in enumerate(z))}];', '    proof {', f'        let z = {zs}; let raw = {rs}; let positions = {ps};', f'        assert(literal_words({zs},{rs},{ps},0,{len(raw)+1})) by (compute);', f'        literal_decodes(z,raw,positions,0,{len(raw)+1}); literal_expand(z,raw,positions,0);', '        assert(raw.take(0) =~= Seq::<u8>::empty());', f'        assert(m::MCompressed(z,19,Seq::empty(),{len(raw)},32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == {effect});', f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 1) by (compute);', '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);', f'        assert(m::MBlk(z,16,Seq::empty(),{len(raw)},32768,true) == {effect});', f'        assert(m::MFinish({zs},{pos[-1]},{rs},{len(raw)},false) == (m::StoredEffect::End{{out:{rs},position:{8*len(z)}}})) by (compute);', f'        assert(m::Inflate(z,{len(raw)},true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,{len(raw)},true,false).out == raw);', f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);', '    }', f'    let ghost raw = {rs}; let mut output = [0xa5u8;{len(raw)}]; check_at(&wire,&mut output,Status::Done,Ghost(raw));', '}']
    encoding = 'fixed-literals'
    if stored_body(raw) == z[2:-4]:
        encoding = 'stored'
        end = 8*(7+len(raw))
        lines = lines[:start]+[f'// Class {ident}: {len(members)} frozen stored streams; R={len(raw)}.', '#[verifier::spinoff_prover]', f'fn {ident}() {{', '    hide(m::MStoredPayload);', f'    let wire = [{", ".join(hex(x)+("u8" if i == 0 else "") for i,x in enumerate(z))}];', '    proof {', f'        let z = {zs}; let raw = {rs};', f'        assert(z.subrange(7,{7+len(raw)}) =~= raw);', '        stored_payload(z,raw);', '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);', f'        assert(m::field_value({zs},16,1) == 1) by (compute); assert(m::field_value({zs},17,2) == 0) by (compute);', f'        assert(m::MStoredBlock(z,19,Seq::empty(),{len(raw)}) == (m::StoredEffect::End{{out:raw,position:{end}}}));', f'        assert(m::MOneBlock(z,16,Seq::empty(),{len(raw)},32768,true) == (true,m::StoredEffect::End{{out:raw,position:{end}}}));', f'        assert(m::MBlk(z,16,Seq::empty(),{len(raw)},32768,true) == (m::StoredEffect::End{{out:raw,position:{end}}}));', f'        assert(m::MFinish({zs},{end},{rs},{len(raw)},false) == (m::StoredEffect::End{{out:{rs},position:{8*len(z)}}})) by (compute);', '        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);', f'        assert(m::Inflate(z,{len(raw)},true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,{len(raw)},true,false).out == raw);', f'        assert(crate::limits::SEGMENT_BYTES_MAX >= {len(z)}) by (compute);', '    }', f'    let ghost raw = {rs}; let mut output = [0xa5u8;{len(raw)}]; check_at(&wire,&mut output,Status::Done,Ghost(raw));', '}']
    manifest.append({'class':ident,'encoding':encoding,'raw':raw,'ihdr_hex':ihdr.hex(),'members':members})
lines += ['} // verus!', '']
(ROOT/'src/witness_classes.rs').write_text('\n'.join(lines))
(ROOT/'tools/witness_classes.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Authored',len(CASES),'stream classes covering',sum(len(c['members']) for c in manifest),'frozen streams')
