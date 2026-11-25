#!/usr/bin/env python3
"""Independent frozen frame literal audit; no generator or decoder imports."""
import argparse
import hashlib
import json
from pathlib import Path
import re

def wire(text):
    alphabet='0123456789abcdef'
    result=bytearray()
    for i in range(0,len(text),2):
        result.append(alphabet.index(text[i].lower())*16+alphabet.index(text[i+1].lower()))
    return bytes(result)
def expanded(frame):
    result=bytearray()
    for run in frame['rgba_runs']:
        block=wire(run['hex'])
        remaining=run['count']
        while remaining:
            for octet in block:
                result.append(octet)
            remaining-=1
    return bytes(result)
def body(source,name):
    found=re.search(r'\bfn '+re.escape(name)+r'\([^\n]*?\)\s*(?:->[^\n{]+)?\s*\{',source)
    assert found,name
    start=found.end();depth=1;at=start
    while depth:
        if source[at]=='{':depth+=1
        if source[at]=='}':depth-=1
        at+=1
    return source[start:at-1]
def sequence(source,name):
    raw=body(source,name).strip();assert raw.startswith('seq![') and raw.endswith(']'),name
    parts=raw[5:-1].split(',') if raw[5:-1] else []
    result=[]
    for part in parts:
        assert re.fullmatch(r'\d+u8',part),name
        value=int(part[:-2]);assert 0<=value<=255;result.append(value)
    return bytes(result)
def check(source,record):
    ident=re.sub(r'[^A-Za-z0-9]','_',record['name']).lower();e=record['expected']
    assert set(e)=={'width','height','num_plays','end','frames'}
    assert sequence(source,'e6_bytes_'+ident)==wire(record['input_hex'])
    frame_source=body(source,'expected_frames_'+ident)
    chunks=re.findall(r'fm::Frame\s*\{([^{}]+)\}',frame_source)
    assert len(chunks)==len(e['frames'])
    skeleton=re.sub(r'fm::Frame\s*\{[^{}]+\}','FRAME',frame_source)
    assert re.sub(r'\s+','',skeleton)=='seq!['+','.join('FRAME' for _ in e['frames'])+']'
    assert frame_source.strip().startswith('seq![') and frame_source.strip().endswith(']')
    parsed=[]
    for k,(chunk,frame) in enumerate(zip(chunks,e['frames'])):
        assert set(frame)=={'rect','delay','dispose','blend','rgba_runs'}
        canvas=sequence(source,f'e6_expected_{ident}_{k}')
        assert canvas==expanded(frame)
        assert len(canvas)==4*e['width']*e['height']
        value=re.fullmatch(r'rgba:e6_expected_'+re.escape(ident)+r'_(\d+)\(\),rect:crate::composite_model::rect\((\d+)u32,(\d+)u32,(\d+)u32,(\d+)u32\),delay:(None|Some\(\((\d+)u16,(\d+)u16\)\)),dispose:(\d+)u8,blend:(\d+)u8',chunk)
        assert value
        g=value.groups();assert int(g[0])==k
        assert list(map(int,g[1:5]))==frame['rect']
        delay=None if g[5]=='None' else [int(g[6]),int(g[7])]
        assert delay==frame['delay']
        assert int(g[8])==frame['dispose'] and int(g[9])==frame['blend']
        parsed.append(dict(bytes=len(canvas),sha256=hashlib.sha256(canvas).hexdigest(),rect=frame['rect'],delay=delay,dispose=int(g[8]),blend=int(g[9])))
    pred=re.sub(r'\s+','',body(source,'expected_value_'+ident))
    plays='None' if e['num_plays'] is None else 'Some('+str(e['num_plays'])+'u32)'
    end='id::End::Iend' if e['end']=='iend' else 'id::End::Format(Error::'+e['end']['error']+')'
    wanted=(f'value.info.header.width=={e["width"]}u32&&value.info.header.height=={e["height"]}u32'
            f'&&value.num_plays=={plays}&&value.info.num_plays=={plays}'
            f'&&value.frames==expected_frames_{ident}()&&value.end=={end}')
    assert pred==wanted
    return dict(name=record['name'],frames=parsed,num_plays=e['num_plays'],end=e['end'])
def main():
    p=argparse.ArgumentParser();p.add_argument('--root',type=Path,required=True);a=p.parse_args();root=a.root
    rs=[r for r in json.loads((root.parent/'spec-frozen/VECTORS.json').read_text()) if r['entry']=='frames' and r['name'].startswith('apng/')]
    source=(root/'src/witness_m6_m5_frames.rs').read_text();assert len(rs)==13
    audited=[check(source,r) for r in rs]
    r=rs[0];ident=re.sub(r'[^A-Za-z0-9]','_',r['name']).lower();changed=[]
    literal=body(source,'e6_expected_'+ident+'_0');at=literal.index('255u8')
    variants={'canvas_byte':source.replace(literal,literal[:at]+'254u8'+literal[at+5:],1),
              'raw_delay_denominator':source.replace('delay:Some((5u16,0u16))','delay:Some((5u16,100u16))',1),
              'rectangle':source.replace('rect:crate::composite_model::rect(0u32,0u32,4u32,3u32)','rect:crate::composite_model::rect(0u32,0u32,3u32,3u32)',1),
              'dispose':source.replace('dispose:0u8','dispose:1u8',1),
              'blend':source.replace('blend:0u8','blend:1u8',1),
              'plays':source.replace('value.num_plays==Some(2u32)','value.num_plays==Some(3u32)',1),
              'terminal':source.replace('value.end==id::End::Iend','value.end==id::End::Format(Error::AdlerMismatch)',1)}
    for name,mutated in variants.items():
        assert mutated!=source
        try:check(mutated,r)
        except AssertionError:changed.append(name)
        else:raise AssertionError('Auditor accepted changed '+name)
    print(json.dumps(dict(status='PASS',records=len(audited),frames=sum(len(x['frames']) for x in audited),canvas_bytes=sum(f['bytes'] for x in audited for f in x['frames']),negative_literal_controls_rejected=changed,scope='Independent literal audit only; not a decoder mutation campaign',audited=audited),indent=2))
if __name__=='__main__':main()
