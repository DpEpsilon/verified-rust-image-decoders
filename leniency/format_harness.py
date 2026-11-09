#!/usr/bin/env python3
"""Seeded, provenance-aware, per-process differential harness for TGA/ICNS/ICO.

Inputs and third-party-derived witnesses stay under /tmp/claude-0. Only frozen-
vector-derived witnesses are copied into the adapter directory.
"""
import argparse
import collections
import concurrent.futures
import hashlib
import json
from pathlib import Path
import random
import struct
import zlib
import shutil
import sys
import time

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parent
PROJECTS=ROOT.parent
sys.path.insert(0,str(ROOT))
import engine

VECTORS={'tga':PROJECTS/'verified-decoders/tga/spec-frozen/VECTORS.json',
         'icns':PROJECTS/'verified-decoders/icns/spec-frozen/VECTORS.json',
         'ico':PROJECTS/'ico/spec/VECTORS.json'}
EXT={'tga':'tga','icns':'icns','ico':'ico'}
FIELDS={
 'tga':{'id_length':(0,1),'map_flag':(1,1),'image_type':(2,1),'map_first':(3,2),'map_length':(5,2),
        'map_depth':(7,1),'x_origin':(8,2),'y_origin':(10,2),'width':(12,2),'height':(14,2),'pixel_depth':(16,1),'descriptor':(17,1)},
 'icns':{'declared_length':(4,4),'entry_type':(8,4),'entry_length':(12,4)},
 'ico':{'reserved':(0,2),'kind':(2,2),'entry_count':(4,2),'entry_width':(6,1),'entry_height':(7,1),
        'palette_count':(8,1),'entry_reserved':(9,1),'planes_or_hotspot_x':(10,2),'depth_or_hotspot_y':(12,2),
        'payload_length':(14,4),'payload_offset':(18,4)}}
BIG={'icns':{'declared_length','entry_length'},'ico':{'payload_length','payload_offset'},'tga':set()}

def u(b,p,n,endian='little'):
    return int.from_bytes(b[p:p+n],endian) if len(b)>=p+n else 0

def format_seed(fmt,b):
    if fmt=='tga':return (u(b,2,1),u(b,16,1),u(b,17,1)&0x30,u(b,7,1))
    if fmt=='icns':return (b[8:12],b[16:20],len(b)//64)
    return (u(b,2,2),u(b,4,2),b[22:26],u(b,12,2))

def icns_synthetic(vectors):
    """Compose frozen colour/mask elements and deterministic PNG; no corpus bytes."""
    by_name={v['name']:bytes.fromhex(v['input_hex']) for v in vectors}
    def file(*elements):
        payload=b''.join(elements)
        return b'icns'+struct.pack('>I',8+len(payload))+payload
    def png(side):
        def chunk(tag,payload):
            return struct.pack('>I',len(payload))+tag+payload+struct.pack('>I',zlib.crc32(tag+payload)&0xffffffff)
        pixel=(bytes([0])+bytes([26,98,167,255])*side)*side
        return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',side,side,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(pixel,9))+chunk(b'IEND',b'')
    def element(code,payload):return code+struct.pack('>I',len(payload)+8)+payload
    pairs=[]
    for side,colour,mask in [(16,'rgb-is32','mask-s8mk'),(32,'rgb-il32','mask-l8mk'),
                             (48,'rgb-ih32','mask-h8mk'),(128,'rgb-it32','mask-t8mk')]:
        pair=(by_name[colour][8:],by_name[mask][8:])
        pairs.append(pair)
        yield file(*pair),{'source':'frozen','name':f'compose-legacy-{side}','rule':'synth:legacy','expected_ok':True}
    for side,code in [(16,b'icp4'),(32,b'icp5'),(64,b'icp6'),(128,b'ic07'),(256,b'ic08')]:
        icon=element(code,png(side))
        yield file(icon),{'source':'frozen','name':f'compose-png-{side}','rule':'synth:png','expected_ok':True}
        if side in (16,32,128):
            yield file(*pairs[0],icon),{'source':'frozen','name':f'compose-legacy-png-{side}','rule':'synth:mixed','expected_ok':True}
            yield file(icon,*pairs[0]),{'source':'frozen','name':f'compose-png-legacy-{side}','rule':'synth:mixed','expected_ok':True}
    yield file(*pairs[0],*pairs[1]),{'source':'frozen','name':'compose-two-legacy-16-32','rule':'synth:multi','expected_ok':True}
    yield file(*pairs[1],*pairs[0]),{'source':'frozen','name':'compose-two-legacy-32-16','rule':'synth:multi','expected_ok':True}


def bases(fmt):
    data=json.loads(VECTORS[fmt].read_text())
    for v in data:
        raw=bytes.fromhex(v['input_hex'])
        if fmt=='ico':
            if 'repeat_from' in v:
                cut=v['repeat_from'];size=cut+(len(raw)-cut)*v.get('input_repeat',1)+len(bytes.fromhex(v.get('input_suffix_hex','')))
                if size>2_000_000:continue
                raw=raw[:cut]+raw[cut:]*v.get('input_repeat',1)+bytes.fromhex(v.get('input_suffix_hex',''))
            else:
                if len(raw)*v.get('input_repeat',1)>2_000_000:continue
                raw*=v.get('input_repeat',1)
        if len(raw)>2_000_000:continue
        yield raw,{'source':'frozen','name':v['name'],'operation':v.get('operation','decode'),
                   'rule':'frozen:'+v.get('operation','decode'),'expected_ok':'error' not in v['expected']}
    if fmt=='icns':yield from icns_synthetic(data)
    roots=[(PROJECTS/'glycin-plan/glycin/tests/test-images','glycin-tests'),
           (PROJECTS/'spec/gdk-pixbuf/tests','gdk-tests')]
    if fmt=='ico':roots.append((PROJECTS/'dib-verus/corpus','local-ico-cur-corpus'))
    for folder,source in roots:
        if not folder.exists():continue
        for path in sorted(folder.rglob('*')):
            if not path.is_file() or path.suffix.lower() not in ({'.ico','.cur'} if fmt=='ico' else {'.'+fmt}):continue
            if path.stat().st_size>2_000_000:continue
            yield path.read_bytes(),{'source':source,'name':str(path),'rule':source}

def variants(fmt,b,rng):
    length=len(b);fields=list(FIELDS[fmt].items());rng.shuffle(fields)
    # These rules run before the field sweep so a bounded run exercises each class.
    spots={'tga':[0,2,12,14,17,18,19,length//2,length-1],
           'icns':[0,4,7,8,11,12,15,16,17,18,19,20,24,32,64,length//2,length-1],
           'ico':[0,2,4,6,9,14,18,21,22,u(b,18,4)-1,u(b,18,4),length//2,length-1]}[fmt]
    for n in sorted(set(x for x in spots if 0<=x<length)):
        yield b[:n],{'rule':'truncation','field':'length','value':n}
    for n in [1,2,8,32,256]:yield b+bytes(n),{'rule':'trailing-bytes','field':'tail','value':n}
    if fmt=='tga':
        for descriptor in [0,1,7,8,15,16,31,32,48,63,64,128,255]:
            if length>=18:
                a=bytearray(b);a[17]=descriptor;yield bytes(a),{'rule':'origin-alpha-bits','field':'descriptor','value':descriptor}
        if len(b)>19 and b[2] in (9,10,11):
            pos=18+b[0]+(b[5]+256*b[6])*((b[7]+7)//8)
            if pos<len(b):
                for control in [0,1,127,128,129,254,255]:
                    a=bytearray(b);a[pos]=control;yield bytes(a),{'rule':'rle-packet','field':'control','value':control}
    if fmt=='icns' and length>=16:
        for code in [b'is32',b'it32',b's8mk',b'icp4',b'ic08',b'XXXX']:
            a=bytearray(b);a[8:12]=code;yield bytes(a),{'rule':'entry-type','field':'type','value':code.decode()}
    if fmt=='ico' and length>=22:
        for p in [6,7,8,9,10,12,14,18]:
            a=bytearray(b);a[p]^=128;yield bytes(a),{'rule':'entry-byte','field':str(p),'value':a[p]}
        count=min(u(b,4,2),12)
        for i in range(1,count):
            p=6+16*i
            if p+16<=length:
                for field,offset,n in [('entry_width',0,1),('payload_length',8,4),('payload_offset',12,4)]:
                    for value in [v for v in [0,1,length,length+1,(1<<(8*n))-1] if v<1<<(8*n)]:
                        a=bytearray(b);a[p+offset:p+offset+n]=value.to_bytes(n,'little');yield bytes(a),{'rule':'later-entry-'+field,'field':f'entry[{i}].{field}','value':value}
    for _ in range(24):
        if not b:break
        pos=rng.randrange(length);a=bytearray(b);a[pos]^=1<<rng.randrange(8)
        yield bytes(a),{'rule':'byte-flip','field':'offset','value':pos}
    for name,(p,n) in fields:
        if p+n>length:continue
        endian='big' if fmt=='icns' and name in BIG['icns'] else 'little'
        original=u(b,p,n,endian);maximum=(1<<(n*8))-1
        values=[0,1,2,3,7,8,15,16,17,31,32,33,63,64,127,128,129,255,256,257,511,512,
                length-1,length,length+1,original-1,original+1,maximum]
        if name in ('image_type','kind','map_depth','pixel_depth','depth_or_hotspot_y'):
            values.extend([4,9,10,11,15,16,24,32])
        if name in BIG.get(fmt,set()):values.extend([8,12,16,18,22,65535,65536,0x7fffffff,0x80000000])
        for val in dict.fromkeys(v for v in values if 0<=v<=maximum and v!=original):
            a=bytearray(b);a[p:p+n]=val.to_bytes(n,endian)
            yield bytes(a),{'rule':name,'field':name,'value':val,'old':original}

def generate(fmt,limit,seed):
    rng=random.Random(seed);base=list(bases(fmt));seed_by_key={}
    for b,m in base:
        if not m.get('expected_ok') and m['source']=='frozen':continue
        if len(b)<{'tga':18,'icns':16,'ico':22}[fmt]:continue
        seed_by_key.setdefault(format_seed(fmt,b),(b,m))
    for b,m in base:
        if m['source']!='frozen' and len(b)>=22:seed_by_key.setdefault(format_seed(fmt,b),(b,m))
    series=list(seed_by_key.values());rng.shuffle(series)
    generated=base[:limit];iters=[iter(variants(fmt,b,rng)) for b,_ in series];active=list(range(len(series)))
    while len(generated)<limit and active:
        for i in active[:]:
            try:b,meta=next(iters[i])
            except StopIteration:active.remove(i);continue
            meta.update(source='mutation',seed=series[i][1]['name'],seed_source=series[i][1]['source'])
            generated.append((b,meta))
            if len(generated)>=limit:break
    return generated,len(base),len(series)

def runners(fmt):
    core={'tga':['verified','image','glycin','tiny'],'icns':['verified-legacy','verified-png','crate','glycin'],
          'ico':['oracle-largest','oracle-first','image','crate-largest','crate-first','glycin']}[fmt]
    gdk=['gdk-file','gdk-stream','gdk-auto-whole','gdk-forced-whole','gdk-forced-byte','gdk-forced-chunk']
    return core+gdk

def invoke(fmt,path,name,timeout,dump=None):
    if name.startswith('gdk-'):
        selector=name[4:]
        return engine.invoke(path,(Path('/tmp/claude-0/leniency-target/gdk-formats'),[fmt,selector]),timeout,dump)
    if name.startswith('oracle-'):
        return engine.invoke(path,(sys.executable,['-I',str(PROJECTS/'leniency/ico/oracle_runner.py'),name[7:]]),timeout,dump)
    return engine.invoke(path,(Path('/tmp/claude-0/glycin-target/debug/leniency-format-runner'),[fmt+':'+name]),timeout,dump)

def sig(results):
    return tuple((k,v['status'],v.get('width'),v.get('height'),v.get('rgba_sha256')) for k,v in results.items())

def minimize(fmt,original,target,names,timeout,folder,checks,preserve_selection=False):
    """Delta-debug while preserving exact pixels and, for selection rows, entry IDs."""
    data=original;used=0;n=2;tmp=folder/'minimize.input'
    while len(data)>1 and used<checks:
        step=max(1,(len(data)+n-1)//n);changed=False
        for start in range(0,len(data),step):
            candidate=data[:start]+data[start+step:]
            if not candidate:continue
            tmp.write_bytes(candidate);used+=1
            result={name:invoke(fmt,tmp,name,timeout) for name in names}
            observed=engine.outcome_signature(result) if preserve_selection else sig(result)
            if observed==target:data=candidate;n=max(2,n-1);changed=True;break
            if used>=checks:break
        if not changed:
            if n>=len(data):break
            n=min(len(data),n*2)
    tmp.unlink(missing_ok=True)
    return data,used

def pixel_difference(fmt,path,results,timeout,folder):
    """Describe a representative difference from raw straight RGBA, not just hash."""
    good=[name for name,v in results.items() if v['status']=='ok']
    if len(good)<2:return ''
    if len({(results[k]['width'],results[k]['height']) for k in good})>1:return 'dimensions'
    dumps=[];paths=[]
    try:
        for k in good:
            tmp=folder/f'pixel-{k}.rgba';paths.append(tmp)
            v=invoke(fmt,path,k,timeout,tmp)
            if v['status']!='ok':return 'rgba-dump-unavailable'
            dumps.append(tmp.read_bytes())
        original=dumps[0]
        if any(len(d)!=len(original) for d in dumps[1:]):return 'output-length'
        if all(all(original[i:i+3]==d[i:i+3] for i in range(0,len(d),4)) for d in dumps[1:]):return 'alpha-only'
        def swapped(d):return all(original[i]==d[i+2] and original[i+1]==d[i+1] and original[i+2]==d[i] and original[i+3]==d[i+3] for i in range(0,len(d),4))
        if all(d==original or swapped(d) for d in dumps[1:]):return 'red-blue-swap'
        row=results[good[0]]['width']*4;h=results[good[0]]['height'];w=results[good[0]]['width']
        if all(d==original or original==b''.join(d[i*row:(i+1)*row] for i in range(h-1,-1,-1)) for d in dumps[1:]):return 'vertical-orientation'
        horizontal=lambda d:b''.join(d[y*row+x*4:y*row+x*4+4] for y in range(h) for x in range(w-1,-1,-1))
        if all(d==original or original==horizontal(d) for d in dumps[1:]):return 'horizontal-orientation'
        both=lambda d:b''.join(d[y*row+x*4:y*row+x*4+4] for y in range(h-1,-1,-1) for x in range(w-1,-1,-1))
        if all(d==original or original==both(d) for d in dumps[1:]):return 'both-axes-orientation'
        return 'color-values-or-layout'
    except OSError:return 'rgba-dump-unavailable'
    finally:
        for p in paths:p.unlink(missing_ok=True)


def distinct_pixel_rows(fmt,summary,timeout=2,checks=8):
    """Expose pixel disagreements hidden by a rule's modal accept/reject row."""
    folder=Path('/tmp/claude-0/leniency-'+fmt)
    metadata={m['id']:m for m in map(json.loads,(folder/'manifest.jsonl').open())}
    grouped=collections.defaultdict(list)
    for r in map(json.loads,(folder/'results.jsonl').open()):
        if r['classification']=='accept-different-pixels':grouped[r['rule']].append(r)
    out=[];names=runners(fmt);adapter=PROJECTS/'leniency'/fmt
    for rule,cases in sorted(grouped.items(),key=lambda x:-len(x[1]))[:10]:
        r=next((case for case in cases if metadata[case['id']]['source']=='frozen' or metadata[case['id']].get('seed_source')=='frozen'),cases[0])
        i=r['id'];m=metadata[i];src=folder/'inputs'/f'{i:05}.{EXT[fmt]}'
        shrunk,_=minimize(fmt,src.read_bytes(),sig(r['results']),names,timeout,folder,checks)
        derived=m['source'] not in ('frozen','mutation') or m.get('seed_source') not in (None,'frozen')
        dest=(folder/'witnesses' if derived else adapter/'witnesses');dest.mkdir(exist_ok=True)
        path=dest/f'pixel-{len(out)+1:02}.{EXT[fmt]}';path.write_bytes(shrunk)
        out.append({'rule':rule,'count':len(cases),'id':i,'witness':str(path) if derived else 'witnesses/'+path.name,
                    'difference':pixel_difference(fmt,path,r['results'],timeout,folder),'results':r['results']})
    summary['pixel_rows']=out
    return summary


def selection_rows(fmt,summary,timeout=2,checks=8):
    """Cluster representation-choice differences, not merely pixel acceptance."""
    folder=Path('/tmp/claude-0/leniency-'+fmt)
    metadata={m['id']:m for m in map(json.loads,(folder/'manifest.jsonl').open())}
    grouped=collections.defaultdict(list);detection=[]
    for line in (folder/'results.jsonl').open():
        row=json.loads(line);r=row['results'];i=row['id']
        if fmt=='tga':
            a,b=r['gdk-file'],r['gdk-auto-whole']
            if (a['status'],a.get('rgba_sha256'))!=(b['status'],b.get('rgba_sha256')):detection.append(i)
            continue
        pairs=([('verified-legacy','verified-png'),('crate','glycin')] if fmt=='icns' else
               [('oracle-first','oracle-largest'),('crate-first','crate-largest')])
        for left,right in pairs:
            a,b=r[left],r[right]
            different_pixels=(a.get('width'),a.get('height'),a.get('rgba_sha256'))!=(b.get('width'),b.get('height'),b.get('rgba_sha256'))
            different_entry=(fmt=='ico' or left=='verified-legacy') and a.get('selection')!=b.get('selection')
            if a['status']==b['status']=='ok' and (different_pixels or different_entry):
                grouped[(left,right,row['rule'])].append((i,r))
    output=[];names=runners(fmt)
    for (left,right,rule),cases in sorted(grouped.items(),key=lambda kv:-len(kv[1]))[:16]:
        preferred=next(((i,r) for i,r in cases if metadata[i]['source']=='frozen' or metadata[i].get('seed_source')=='frozen'),cases[0])
        i,r=preferred;m=metadata[i];path=folder/'inputs'/f'{i:05}.{EXT[fmt]}'
        b,_=minimize(fmt,path.read_bytes(),engine.outcome_signature(r),names,timeout,folder,checks,True)
        derived=m['source'] not in ('frozen','mutation') or m.get('seed_source') not in (None,'frozen')
        dest=(folder/'witnesses' if derived else PROJECTS/'leniency'/fmt/'witnesses');dest.mkdir(exist_ok=True)
        witness=dest/f'selection-{len(output)+1:02}.{EXT[fmt]}';witness.write_bytes(b)
        output.append({'rule':rule,'count':len(cases),'id':i,'left':left,'right':right,
                       'witness':str(witness) if derived else 'witnesses/'+witness.name,
                       'results':{k:r[k] for k in (left,right,'crate' if fmt=='icns' else 'image','glycin','gdk-forced-whole') if k in r}})
    summary['selection_rows']=output
    summary['gdk_file_auto_disagreements']=len(detection)
    summary['gdk_file_auto_ids']=detection[:100]
    return summary


def public_subset(fmt,full):
    """Produce independently countable results using frozen-derived bytes only."""
    folder=Path('/tmp/claude-0/leniency-'+fmt)
    metadata={m['id']:m for m in map(json.loads,(folder/'manifest.jsonl').open())}
    public=lambda m:m['source']=='frozen' or m['source']=='mutation' and m.get('seed_source')=='frozen'
    cases=[r for r in map(json.loads,(folder/'results.jsonl').open()) if public(metadata[r['id']])]
    s=dict(full);s['public_subset']=True;s['inputs']=len(cases)
    s['base_inputs']=sum(metadata[r['id']]['source']=='frozen' for r in cases)
    s['seed_variants']=len({metadata[r['id']].get('seed') for r in cases if metadata[r['id']]['source']=='mutation'})
    s['counts']=dict(collections.Counter(r['classification'] for r in cases))
    s['changed_mutations']='see full local results'
    s['rows']=[dict(r) for r in full['rows'] if r['witness'].startswith('witnesses/')]
    baselines={};differing=collections.defaultdict(list);changed=0
    for r in cases:
        m=metadata[r['id']];observed=r['results']
        if m['source']=='frozen':baselines[m['name']]=engine.relation_signature(observed)
        mutation_changed=m['source']=='mutation' and engine.relation_signature(observed)!=baselines.get(m['seed'])
        changed+=mutation_changed
        if r['classification'] not in ('all-reject','all-accept-identical-pixels') and (m['source']=='frozen' or mutation_changed):
            differing[r['rule']].append(sig(observed))
    s['changed_mutations']=changed
    for row in s['rows']:
        selected=differing[row['rule']];row['count']=len(selected);row['patterns']=len(set(selected))
    s['selection_rows']=[dict(r) for r in full.get('selection_rows',[]) if r['witness'].startswith('witnesses/')]
    for row in s['selection_rows']:
        left,right=row['left'],row['right'];count=0
        for r in cases:
            if r['rule']!=row['rule']:continue
            a,b=r['results'][left],r['results'][right]
            different_pixels=(a.get('width'),a.get('height'),a.get('rgba_sha256'))!=(b.get('width'),b.get('height'),b.get('rgba_sha256'))
            different_entry=(fmt=='ico' or left=='verified-legacy') and a.get('selection')!=b.get('selection')
            if a['status']==b['status']=='ok' and (different_pixels or different_entry):count+=1
        row['count']=count
    s['pixel_rows']=[dict(r) for r in full.get('pixel_rows',[]) if r['witness'].startswith('witnesses/')]
    for row in s['pixel_rows']:
        row['count']=sum(r['rule']==row['rule'] and r['classification']=='accept-different-pixels' for r in cases)
    s['anomalies']=[a for a in full['anomalies'] if public(a['mutation'])]
    s['anomaly_count']=sum(r['classification']=='crash-or-timeout' for r in cases)
    s['selection_difference_count']=sum(len({(v.get('width'),v.get('height')) for k,v in r['results'].items() if v['status']=='ok' and not k.startswith('gdk-')})>1 for r in cases)
    s['gdk_file_auto_disagreements']=sum((r['results']['gdk-file']['status'],r['results']['gdk-file'].get('rgba_sha256'))!=(r['results']['gdk-auto-whole']['status'],r['results']['gdk-auto-whole'].get('rgba_sha256')) for r in cases)
    for k in ('manifest','raw_results','selection_difference_ids','gdk_file_auto_ids'):
        s.pop(k,None)
    return s


def run(fmt,args):
    folder=Path('/tmp/claude-0/leniency-'+fmt);inputs=folder/'inputs';inputs.mkdir(parents=True,exist_ok=True)
    items,base,seeds=generate(fmt,args.inputs,args.seed)
    manifest=folder/'manifest.jsonl';output=folder/'results.jsonl';names=runners(fmt)
    with manifest.open('w') as f:
        for i,(b,m) in enumerate(items):
            (inputs/f'{i:05}.{EXT[fmt]}').write_bytes(b)
            f.write(json.dumps({'id':i,'length':len(b),'sha256':hashlib.sha256(b).hexdigest(),**m})+'\n')
    counts=collections.Counter();rules=collections.defaultdict(list);baselines={};anomalies=[];selection_differences=[]
    started=time.monotonic();changed_count=0
    def job(item):
        i,(_,m)=item;path=inputs/f'{i:05}.{EXT[fmt]}'
        return i,m,{name:invoke(fmt,path,name,args.timeout) for name in names}
    with output.open('w') as out,concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for i,m,result in pool.map(job,enumerate(items)):
            cls=engine.classify(result);counts[cls]+=1
            if m['source']!='mutation':baselines[m['name']]=engine.relation_signature(result)
            changed=m['source']=='mutation' and engine.relation_signature(result)!=baselines.get(m['seed'])
            changed_count+=changed
            good={k:(v.get('selection'),v.get('width'),v.get('height')) for k,v in result.items() if v['status']=='ok' and not k.startswith('gdk-')}
            if len({(x[1],x[2]) for x in good.values()})>1:selection_differences.append(i)
            if cls not in ('all-reject','all-accept-identical-pixels') and (m['source']!='mutation' or changed):rules[m['rule']].append((i,m,result,cls))
            if cls=='crash-or-timeout':anomalies.append((i,m,result))
            out.write(json.dumps({'id':i,'rule':m['rule'],'classification':cls,'results':engine.compact(result)},separators=(',',':'))+'\n')
            if (i+1)%1000==0:print(f'{fmt} {i+1}/{len(items)} {time.monotonic()-started:.1f}s',flush=True)
    elapsed=round(time.monotonic()-started,2)
    local=PROJECTS/'leniency'/fmt/'witnesses';external=folder/'witnesses';local.mkdir(exist_ok=True);external.mkdir(exist_ok=True)
    rows=[]
    for rule,entries in sorted(rules.items(),key=lambda pair:-len(pair[1])):
        patterns=collections.defaultdict(list)
        for e in entries:patterns[sig(e[2])].append(e)
        main=max(patterns.values(),key=len);i,m,result,cls=main[0]
        source=inputs/f'{i:05}.{EXT[fmt]}';target=sig(result)
        shrunk,checks=minimize(fmt,source.read_bytes(),target,names,args.timeout,folder,args.min_checks)
        third_party=m['source'] not in ('frozen','mutation') or m.get('seed_source') not in (None,'frozen')
        dest=external if third_party else local;dest.mkdir(exist_ok=True)
        witness=dest/f'{len(rows)+1:02}-{rule.replace(":","-")}.{EXT[fmt]}';witness.write_bytes(shrunk)
        rows.append({'rule':rule,'count':len(entries),'patterns':len(patterns),'original_id':i,
                     'witness':str(witness) if third_party else 'witnesses/'+witness.name,
                     'mutation':m,'classification':cls,'results':engine.compact(result),
                     'pixel_difference':pixel_difference(fmt,witness,result,args.timeout,folder) if cls=='accept-different-pixels' else '',
                     'min_checks':checks,'original_length':len(items[i][0]),'witness_length':len(shrunk)})
    anomaly_dir=folder/'anomalies';anomaly_dir.mkdir(exist_ok=True)
    for stale in anomaly_dir.glob(f'*.{EXT[fmt]}'):stale.unlink()
    for i,_,_ in anomalies:shutil.copyfile(inputs/f'{i:05}.{EXT[fmt]}',anomaly_dir/f'{i:05}.{EXT[fmt]}')
    summary={'format':fmt,'inputs':len(items),'base_inputs':base,'seed_variants':seeds,'seed':args.seed,
             'counts':counts,'changed_mutations':changed_count,'elapsed_seconds':elapsed,'decoders':names,
             'selection_difference_count':len(selection_differences),'selection_difference_ids':selection_differences[:100],
             'anomaly_count':len(anomalies),'anomalies':[{'id':i,'mutation':m,'results':engine.compact(r)} for i,m,r in anomalies[:40]],
             'rows':rows,'manifest':str(manifest),'raw_results':str(output)}
    adapter=PROJECTS/'leniency'/fmt
    summary=selection_rows(fmt,summary,args.timeout,min(args.min_checks,8))
    summary=distinct_pixel_rows(fmt,summary,args.timeout,min(args.min_checks,8))
    (adapter/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    render(adapter,summary)
    subset=public_subset(fmt,summary)
    (adapter/'summary-public.json').write_text(json.dumps(subset,indent=2)+'\n')
    render(adapter,subset,'LENIENCY-PUBLIC.md')
    print(json.dumps({'format':fmt,'inputs':len(items),'counts':counts,'elapsed_seconds':elapsed,'rows':len(rows),'anomalies':len(anomalies)},indent=2))

def render(adapter,s,filename='LENIENCY.md'):
    fmt=s['format'];names=s['decoders'];folder=f'/tmp/claude-0/leniency-{fmt}'
    lines=[f'# {fmt.upper()} leniency matrix','',f"Seed {s['seed']}; {s['inputs']} inputs"+(' (only frozen-derived, no corpus bytes)' if s.get('public_subset') else '')+f"; {s['base_inputs']} {'frozen inputs' if s.get('public_subset') else 'frozen/corpus inputs'}; {s['seed_variants']} distinct mutation seeds; {s['changed_mutations']} changed acceptance/pixel relations; {s['elapsed_seconds']} seconds for the decoder-comparison pass; witness minimization follows.",'',
           'Classification | Count','--- | ---:']
    for cat in ('all-accept-identical-pixels','accept-different-pixels','accept-vs-reject','all-reject','crash-or-timeout'):
        lines.append(f"{cat} | {s['counts'].get(cat,0)}")
    lines+=['','Every input passes as identical bytes to independent subprocesses; outputs are top-first, straight RGBA8. A row represents the most common full outcome signature among changed inputs from the rule, not every outcome. Unmutated seeds stand alone. image-rs and glycin-image-rs are not independent votes. Detailed provenance and per-input JSONL are in `'+folder+'`.','',
            '## Representative matrix','', '| Rule (differing; patterns) | Witness | '+' | '.join(names)+' |',
            '| --- | --- | '+' | '.join(['---']*len(names))+' |']
    for row in s['rows']:
        cells=[]
        for name in names:
            v=row['results'][name];st=v['status']
            if st=='ok':st=f"{v['width']}×{v['height']} `{v['rgba_sha256'][:8]}`"
            elif st=='error':st='reject `'+v.get('error_class','?').replace('|','/')[:22]+'`'
            cells.append(st)
        detail='; '+row['pixel_difference'] if row.get('pixel_difference') else ''
        lines.append(f"| {row['rule']} ({row['count']}; {row['patterns']}{detail}) | [#{row['original_id']}]({row['witness']}) | "+' | '.join(cells)+' |')
    lines+=['','## Same acceptance, different pixels','',
            'These rows select a pixel-disagreement case even when a rule\'s most common outcome is accept-vs-reject. The diagnostic comes from raw straight RGBA dumps.',
            '| Rule | Witness | Pixel difference | Selected dimensions / hashes |','| --- | --- | --- | --- |']
    for row in s.get('pixel_rows',[]):
        good=[f"{k} {v['width']}×{v['height']} `{v['rgba_sha256'][:8]}`" for k,v in row['results'].items() if v['status']=='ok']
        lines.append(f"| {row['rule']} ({row['count']}) | [#{row['id']}]({row['witness']}) | {row['difference']} | "+'; '.join(good)+' |')
    if not s.get('pixel_rows'):lines.append('| No publishable frozen-derived pixel witness in this subset | — | — | — |')
    findings={
        'tga':[
            'Descriptor attribute bits: the verified core and image-rs reject one altered image that forced GDK decodes.',
            'Palette starting index 1: the verified core rejects; image-rs and forced GDK accept the representative.',
            'Colour-map flag 2: the verified core rejects; image-rs and forced GDK accept the representative.',
            'A one-byte trailer: the verified core reports TrailingBytes; image-rs and forced GDK accept.',
            'Palette entry depth 1: the verified core rejects; image-rs and forced GDK accept the representative.',
            'An RLE packet control byte 129 overruns the verified core, while image-rs and forced GDK accept the representative.',
            'A mutated palette index makes tinytga panic (caught as crash) while the verified core returns ColorIndexOutOfRange.',
            'A 2×2 origin mutation makes tinytga mirror rows horizontally; the verified core, image-rs, glycin and forced GDK agree on the unmirrored pixels.',
            'Two frozen images at the 64 MiB output boundary time out in the image crate under the two-second per-decoder budget.',
            'The filename and automatic GDK stream/loader paths disagree on format detection; forced TGA decoding separates detection from decoding.'
        ],
        'icns':[
            'A shortened declared container length is rejected by the verified parser and glycin but accepted by the icns crate.',
            'A byte after the declared container length is rejected by the verified parser and glycin but accepted by the icns crate.',
            'Malformed entry lengths can trigger a multi-gigabyte allocation failure in the icns crate under the memory limit.',
            'The verified core does not choose an image: a legacy-only file has NoCandidate in the PNG-specific column, not a parse failure.',
            'The verified core delegates PNG pixel decoding to image-rs; it only verifies entry ranges and legacy pixels.',
            'All four composed 16/32/48/128 legacy colour+mask pairs decode in the verified core, icns crate and glycin.',
            'Composed PNG entries decode in the icns crate and glycin; the installed GDK ICNS loader rejects the representative PNG-only entry.',
            'On equal-size PNG and legacy candidates, glycin prefers PNG; the icns crate can choose a legacy representation depending on entry order.',
            'The two complete legacy pairs are ranked by largest side, independent of file order, in the verified comparison selector.',
            'A damaged PNG candidate can make glycin fall back to an intact legacy colour+mask pair rather than reject the container.'
        ],
        'ico':[
            'A nonzero directory reserved word is rejected by the independent oracle and ico crate but accepted by image-rs.',
            'A nonzero per-entry reserved byte is rejected by the oracle but accepted by image-rs.',
            'An unsupported directory type zero is rejected by the oracle and ico crate while image-rs accepts the representative.',
            'The GDK ICO module accepts one zero-length entry that the oracle, image-rs and ico crate reject.',
            'A payload offset overlapping the directory is rejected by the oracle but accepted by image-rs.',
            'A directory width byte zero denotes 256 in the oracle and ico crate, but image-rs rejects some mismatched payloads.',
            'The same 256-pixel sentinel rule applies to a directory height byte zero.',
            'A CUR hotspot-X mutation remains pixel-neutral in the oracle and ico crate but can make image-rs reject the entry.',
            'The oracle explicitly reports PngDelegation for PNG entries; this is not a judgment that the PNG is invalid.',
            'Both ico-crate selectors can abort on malformed declared entry sizes under the subprocess memory limit.'
        ]}[fmt]
    if fmt=='tga' and s.get('public_subset'):
        findings[1]='An image-type byte of zero: the verified core rejects, but image-rs and glycin decode the frozen-derived representative.'
        findings[4]='An image-ID length of one exposes a truncated pixel range: the verified core and image-rs reject, while tinytga accepts the frozen-derived representative.'
    if fmt=='ico' and not s.get('public_subset'):
        findings[6]='A corpus-derived, byte-flipped ICO decodes with identical RGB but different alpha in image-rs/glycin versus the oracle, ico crate and GDK; its witness stays under /tmp.'
    lines+=['','## Ten significant observations','']
    for finding in findings:lines.append('- '+finding)
    lines+=['',"## where our core's choices disagree with ALL or MOST other decoders",'']
    if fmt=='ico':lines.append('No ICO verified core exists; the independent ICO+DIB oracles are the spec column. The oracle delegates PNG rather than rejecting it.')
    else:
        peers={'tga':['image','tiny','gdk-forced-whole'],'icns':['crate','gdk-forced-whole']}[fmt]
        found=0
        for row in s['rows']:
            v=row['results'];core=('verified' if fmt=='tga' else 'verified-legacy');status=v[core]['status']=='ok'
            eligible=[p for p in peers if v[p]['status'] not in ('crash','timeout') and
                      not (p=='tiny' and v[p].get('error_class','').startswith('UnsupportedTgaType'))]
            disagree=[p for p in eligible if (v[p]['status']=='ok')!=status]
            if len(eligible)>=2 and len(disagree)>=len(eligible)//2+1:
                lines.append(f"- `{row['rule']}`: core {'accepts' if status else 'rejects'}, {len(disagree)} of {len(eligible)} independent applicable peers ({', '.join(disagree)}) disagree; [witness]({row['witness']}). Revisit the core's rule after checking the original seed and specification.")
                found+=1
        if not found:lines.append('No representative row has a majority of independent peers disagreeing with the verified core; inspect individual results before changing a rule.')
        if fmt=='tga':lines.append('`tinytga` exposes RGB-only output and a narrower set of TGA types; exclude its unsupported formats when deciding whether a core rule should change. image-rs and glycin-image-rs share implementation, so glycin is not a second vote.')
        if fmt=='icns':lines.append('The verified legacy column deliberately rejects files without a complete legacy colour+mask pair; this is an entry-selection boundary, not an ICNS parsing judgment. The verified PNG column delegates PNG pixel decoding to image-rs; no verified PNG core exists.')
    lines+=['','## Selection and detection','']
    if fmt=='tga':lines.append('TGA has no reliable magic: GDK filename `.tga` may select its module when stream/auto-detection fails. The installed GDK 2.42.12 module is `/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-tga.so`. `gdk-forced-*` calls `gdk_pixbuf_loader_new_with_type("tga")`, writing whole, single-byte, or 4096-byte chunks; `gdk-stream` and `gdk-auto-whole` leave detection automatic.')
    elif fmt=='icns':lines.append('Verified legacy selects the largest available legacy colour+mask pair; verified PNG selects the largest PNG candidate (later entry wins ties). The `icns` crate selects its largest advertised complete icon; glycin-icns selects the largest decodable candidate and can fall back. GDK uses installed `libpixbufloader-icns.so`. Selection is observable in `selection` fields. '+str(s['selection_difference_count'])+' inputs produced different decoded dimensions across non-GDK selectors.')
    else:
        lines.append('ICO/CUR oracle and `ico` crate expose first and largest-by-area entry columns. The oracle breaks area ties by directory bit depth; the crate breaks them by bit depth. image-rs 0.25.10 ranks **bit depth before pixel area** and starts from the last entry on ties; glycin shares that image-rs decoder. GDK uses installed `libpixbufloader-ico.so`, sorts by area times a color-count-derived depth score and attempts candidates in that order. `ico:crate-entry:N` and `oracle-entry-N` compare individual entries. Cursor hotspots are metadata, not RGBA pixels. '+str(s['selection_difference_count'])+' inputs produced different decoded dimensions across non-GDK selectors.')
        lines.append('Of 217 frozen ICO vectors, 214 become files. The other three expand to at least 67 MiB (two near 2 GiB) and exceed this run\'s 2 MiB input cap; the oracle\'s own check_vectors.py still covers those declarations without feeding them to decoders.')
    lines+=['','## Entry-selection disagreements','',
            'Each row compares different selected images from the *same bytes*; it is separate from ordinary acceptance differences.',
            '| Rule | Witness | Choices | Peer choices |','| --- | --- | --- | --- |']
    for row in s.get('selection_rows',[]):
        a,b=(row['results'][x] for x in (row['left'],row['right']))
        def choice(v):return f"{v.get('selection','?')} {v.get('width')}×{v.get('height')} `{v.get('rgba_sha256','')[:8]}`" if v['status']=='ok' else v.get('error_class','error')
        others='; '.join(k+' '+choice(v) for k,v in row['results'].items() if k not in (row['left'],row['right']))
        lines.append(f"| {row['rule']} ({row['count']}) | [#{row['id']}]({row['witness']}) | {row['left']} {choice(a)}; {row['right']} {choice(b)} | {others} |")
    if not s.get('selection_rows'):lines.append('| No decoded entry-selection differences in this run | — | — | — |')
    if fmt=='tga':lines.append(f"GDK filename versus auto-stream/loader detection differs on {s.get('gdk_file_auto_disagreements',0)} input(s); forced TGA is the appropriate decoding comparison when auto-detection does not identify the format.")
    lines+=['','## Crashes, timeouts, and anomalies','',f"{s['anomaly_count']} inputs with crashes or timeouts saved separately under `{folder}/anomalies/`; neither exploitation nor upstream reporting was attempted."]
    for a in s['anomalies'][:20]:
        bad=[f'{k} {v["status"]} {v.get("error_class", "")}' for k,v in a['results'].items() if v['status'] in ('crash','timeout')]
        lines.append(f"- Input #{a['id']}: "+', '.join(bad))
    lines+=['','## Run','',f'`./run.sh --inputs 10000 --seed 731 --jobs 8` from `{adapter}`; the script builds runners before executing. `python3 reproduce.py FILE.{EXT[fmt]}` reproduces one witness. Outputs under `{folder}` are regenerated by each run.','']
    (adapter/filename).write_text('\n'.join(lines))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('format',choices=VECTORS);p.add_argument('--inputs',type=int,default=10000)
    p.add_argument('--seed',type=int,default=731);p.add_argument('--jobs',type=int,default=8)
    p.add_argument('--timeout',type=float,default=2.0);p.add_argument('--min-checks',type=int,default=12)
    a=p.parse_args();run(a.format,a)
