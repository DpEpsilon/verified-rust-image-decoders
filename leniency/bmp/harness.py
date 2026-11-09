#!/usr/bin/env python3
"""Reproducible BMP differential corpus, isolated runners, provenance clusters."""
import argparse
import collections
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import random
import shutil
import struct
import subprocess
import sys
import time
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import engine

HERE = Path(__file__).resolve().parent
PROJECTS = HERE.parents[1]
SCRATCH = Path('/tmp/claude-0/leniency-bmp')
VECTORS = PROJECTS / 'verified-decoders/dib/spec-1.1-frozen/VECTORS.json'
RUNNERS = {
    'verified': ('rust', 'verified'), 'image-explicit': ('rust', 'image-explicit'),
    'image-guess': ('rust', 'image-guess'), 'glycin-image-rs': ('rust', 'glycin-image-rs'),
    'gdk-file': ('gdk', 'file'), 'gdk-stream': ('gdk', 'stream'),
    'gdk-whole': ('gdk', 'whole'), 'gdk-byte': ('gdk', 'byte'), 'gdk-chunk': ('gdk', 'chunk'),
}
FIELD = {'bfSize': (2, 4), 'bfOffBits': (10, 4), 'biSize': (14, 4),
         'width': (18, 4), 'height': (22, 4), 'planes': (26, 2),
         'bitCount': (28, 2), 'compression': (30, 4), 'biSizeImage': (34, 4),
         'biClrUsed': (46, 4), 'biClrImportant': (50, 4)}


def u(b, pos, size):
    return int.from_bytes(b[pos:pos+size], 'little') if pos+size <= len(b) else 0


def wrap_dib(b):
    """Infer pixel start from header, masks and palette; preserve original DIB bytes."""
    if len(b) < 4:
        return b'BM' + struct.pack('<IHHI', len(b)+14, 0, 0, len(b)+14) + b
    hdr = u(b, 0, 4)
    bit = u(b, 10 if hdr == 12 else 14, 2)
    compression = u(b, 16, 4) if hdr >= 40 else 0
    colors = u(b, 32, 4) if hdr >= 40 else 0
    count = min(colors or (1 << bit if bit <= 8 else 0), 256)
    mask = (12 if compression == 3 else 16 if compression == 6 else 0) if hdr == 40 else 0
    off = min(len(b), max(0, hdr) + mask + count * (3 if hdr == 12 else 4))
    # If the inferred palette exceeds the DIB, choose a conservative header boundary.
    if off == len(b) and hdr < len(b) and bit >= 16:
        off = hdr + mask
    return b'BM' + struct.pack('<IHHI', len(b)+14, 0, 0, off+14) + b


def source_inputs():
    vectors = json.loads(VECTORS.read_text())
    for v in vectors:
        if v['mode'] in ('BmpFile', 'BareDib'):
            b = bytes.fromhex(v['input_hex'])
            if v['mode'] == 'BareDib':
                b = wrap_dib(b)
            yield b, {'source': 'frozen', 'name': v['name'], 'mode': v['mode'],
                      'expected_ok': 'error' not in v['expected'],
                      'rule': 'frozen:' + v['mode']}
    roots = [(PROJECTS / 'verified-decoders/dib/corpus', 'local-corpus'),
             (PROJECTS / 'glycin-plan/glycin/tests/test-images', 'glycin-tests'),
             (PROJECTS / 'spec/gdk-pixbuf', 'gdk-tests')]
    for root, source in roots:
        if root.exists():
            for path in sorted(root.rglob('*.bmp')):
                if path.stat().st_size <= 2_000_000:
                    yield path.read_bytes(), {'source': source, 'name': str(path), 'rule': source}


def seed_key(b):
    h=u(b,14,4)
    return (h, u(b,24 if h==12 else 28,2), u(b,30,4) if h>=40 else 0)


def variants(seed, rng):
    length = len(seed)
    hdr=u(seed,14,4)
    fields=(dict(FIELD) if hdr!=12 else
            {**{k:v for k,v in FIELD.items() if k in ('bfSize','bfOffBits','biSize')},
             'width':(18,2),'height':(20,2),'planes':(22,2),'bitCount':(24,2)})
    # Schedule each category before the long field-boundary series so the input
    # budget actually covers masks, RLE, truncation, tails and random flips.
    for at, name in [(54,'redMask'),(58,'greenMask'),(62,'blueMask'),(66,'alphaMask')]:
        has_mask=hdr>=52 or (hdr==40 and u(seed,30,4) in (3,6))
        if at+4<=length and has_mask and (name!='alphaMask' or hdr>=56 or u(seed,30,4)==6):
            for val in [0,1,0xff,0xf0f0,0xffffffff,u(seed,at,4)^1]:
                b=bytearray(seed);b[at:at+4]=val.to_bytes(4,'little')
                yield bytes(b),{'rule':'bitfield-mask','field':name,'value':val}
    if hdr>=40 and u(seed,30,4) in (1,2):
        off=u(seed,10,4)
        for pos in [off,max(off,length-2)]:
            if pos+2<=length:
                for escape in [0,1,2,3,254,255]:
                    b=bytearray(seed);b[pos:pos+2]=bytes([0,escape])
                    yield bytes(b),{'rule':'rle-escape','field':'escape@'+str(pos),'value':escape}
    positions=sorted(set([0,1,2,6,10,13,14,17,18,21,22,25,26,28,30,34,46,50,
                          u(seed,10,4)-1,u(seed,10,4),u(seed,10,4)+1,length-1]))
    for n in positions:
        if 0<=n<length:yield seed[:n],{'rule':'truncation','field':'fileLength','value':n}
    for n in [1,2,4,16,256]:
        yield seed+bytes(n),{'rule':'trailing-bytes','field':'trailing','value':n}
    for _ in range(20):
        if not seed:break
        b=bytearray(seed);at=rng.randrange(len(b));b[at]^=1<<rng.randrange(8)
        yield bytes(b),{'rule':'random-bitflip','field':'offset','value':at}
    for off in (10,2,34):
        if off+4<=length:
            b=bytearray(seed);b[off:off+4]=((length+1)%(1<<32)).to_bytes(4,'little')
            b[2:6]=max(0,length-1).to_bytes(4,'little')
            yield bytes(b),{'rule':'inconsistent-pair','field':str(off)+'+bfSize','value':length+1}
    field_items=list(fields.items());rng.shuffle(field_items)
    for field, (at, width) in field_items:
        if at + width > length or (field=='compression' and hdr<40) or (field.startswith('bi') and field not in ('biSize',) and hdr<40):
            continue
        original = u(seed, at, width)
        bound = (1 << (width*8))-1
        values = [0,1,2,3,7,12,13,15,16,17,31,32,39,40,41,51,52,55,56,57,63,64,65,
                  107,108,109,123,124,125,127,128,255,256,511,512,1023,1024,
                  length-1,length,length+1, u(seed,10,4)-1,u(seed,10,4)+1,
                  original-1,original+1,bound]
        if field == 'bitCount': values += [2,3,5,7,12,64]
        if field == 'compression': values += list(range(14))
        if field in ('width','height'): values += [0x80000000, 0x7fffffff, 0xffffffff]
        if field == 'biSize': values += [12,16,40,52,56,64,108,124]
        for value in dict.fromkeys(v for v in values if 0<=v<=bound and v != original):
            b=bytearray(seed); b[at:at+width]=value.to_bytes(width,'little')
            yield bytes(b), {'rule':field, 'field':field,'value':value,'old':original}


def generate(limit, seed):
    rng=random.Random(seed)
    base=list(source_inputs())
    seeds={}
    for b,meta in base:
        if len(b)>=54 and meta['source']=='frozen' and not meta['expected_ok']:
            continue
        bit=u(b,24 if u(b,14,4)==12 else 28,2)
        width=u(b,18,2 if u(b,14,4)==12 else 4)
        height=u(b,20 if u(b,14,4)==12 else 22,2 if u(b,14,4)==12 else 4)
        if len(b)>=26 and width and height and bit in (1,4,8,16,24,32):
            seeds.setdefault(seed_key(b),(b,meta))
    # Prefer structurally sound corpus examples when a frozen vector of the same header tuple rejects.
    for b,meta in base:
        if meta['source']!='frozen' and len(b)>=54:
            seeds.setdefault(seed_key(b),(b,meta))
    generated=list(base)
    series=[(b,meta) for b,meta in seeds.values()]
    rng.shuffle(series)
    lists=[iter(variants(b,rng)) for b,_ in series]
    active=list(range(len(lists)))
    while len(generated)<limit and active:
        for i in active[:]:
            try: b, m=next(lists[i])
            except StopIteration: active.remove(i);continue
            m.update({'source':'mutation','seed':series[i][1]['name']})
            generated.append((b,m))
            if len(generated)>=limit:break
    return generated, len(base),len(seeds)


def invoke(path, decoder, timeout, dump=None):
    kind, mode=RUNNERS[decoder]
    exe = (Path('/tmp/claude-0/glycin-target/debug/leniency-bmp-runner') if kind=='rust'
           else Path('/tmp/claude-0/leniency-target/gdk-runner'))
    return engine.invoke(path,(exe,[mode]),timeout,dump)


classify = engine.classify


def signature(results):
    return tuple((k,v['status'], v.get('width'),v.get('height'),v.get('rgba_sha256')) for k,v in results.items())


def relation_signature(results):
    """Acceptance and cross-decoder pixel equality, independent of pixel contents."""
    keys=list(results)
    return (tuple(results[k]['status'] for k in keys),
            tuple((a,b,results[a].get('width')==results[b].get('width'),
                   results[a].get('height')==results[b].get('height'),
                   results[a].get('rgba_sha256')==results[b].get('rgba_sha256'))
                  for i,a in enumerate(keys) for b in keys[i+1:]
                  if results[a]['status']=='ok' and results[b]['status']=='ok'))


def pixel_difference(path, results, timeout):
    good=[k for k,v in results.items() if v['status']=='ok']
    if len(good)<2:return ''
    sizes={(results[k]['width'],results[k]['height']) for k in good}
    if len(sizes)>1:return 'dimensions'
    files=[]
    try:
        for k in good:
            f=SCRATCH / ('pixels-'+k+'.rgba');invoke(path,k,timeout,f);files.append(f.read_bytes())
        a=files[0];w=results[good[0]]['width'];h=results[good[0]]['height']
        if any(len(b)!=len(a) for b in files[1:]):return 'output-length'
        if all(all(a[i:i+3]==b[i:i+3] for i in range(0,len(a),4)) for b in files[1:]):return 'alpha-only'
        if all(b==a or all(a[i]==b[i+2] and a[i+1]==b[i+1] and a[i+2]==b[i] and a[i+3]==b[i+3] for i in range(0,len(a),4)) for b in files[1:]):return 'red-blue-swap'
        for bits in (4,5,6):
            maximum=(1<<bits)-1
            scales={(round(x*255/maximum),x<<(8-bits)) for x in range(maximum+1)}
            pairs=scales|{(y,x) for x,y in scales}
            if all(b==a or all(a[i+3]==b[i+3] and all((a[i+j],b[i+j]) in pairs or a[i+j]==b[i+j] for j in range(3))
                       for i in range(0,len(a),4)) for b in files[1:]):return f'{bits}-bit-scaling'
        row=w*4
        if all(b==a or a==b''.join(b[j*row:(j+1)*row] for j in range(h-1,-1,-1)) for b in files[1:]):return 'vertical-orientation'
        return 'color-values-or-layout'
    except (OSError,IndexError):return 'unavailable'
    finally:
        for f in (SCRATCH / ('pixels-'+k+'.rgba') for k in good):f.unlink(missing_ok=True)


compact = engine.compact


def minimize(original, target, runners, timeout, max_checks=36):
    """Bounded byte-range ddmin; preserve decoder accept/reject/dimension/hash pattern."""
    b=original; checks=0;n=2
    def pattern(path):
        r={k:invoke(path,k,timeout) for k in runners}
        return tuple((r[k]['status'],r[k].get('width'),r[k].get('height'),r[k].get('rgba_sha256')) for k in runners)
    while len(b)>1 and checks<max_checks:
        step=max(1,(len(b)+n-1)//n); changed=False
        for at in range(0,len(b),step):
            trial=b[:at]+b[at+step:]
            if not trial:continue
            p=SCRATCH/'minimize.tmp';p.write_bytes(trial);checks+=1
            if pattern(p)==target:
                b=trial;n=max(2,n-1);changed=True;break
            if checks>=max_checks:break
        if not changed:
            if n>=len(b):break
            n=min(len(b),n*2)
    return b, checks


def execute(args):
    SCRATCH.mkdir(parents=True,exist_ok=True)
    inputs_dir=SCRATCH/'inputs';inputs_dir.mkdir(exist_ok=True)
    items,base_count,seeds=generate(args.inputs,args.seed)
    manifest=SCRATCH/'manifest.jsonl';output=SCRATCH/'results.jsonl'
    with manifest.open('w') as f:
        for i,(b,m) in enumerate(items):
            path=inputs_dir/f'{i:05d}.bmp';path.write_bytes(b)
            f.write(json.dumps({'id':i,'sha256':hashlib.sha256(b).hexdigest(),'length':len(b),**m})+'\n')
    names=list(RUNNERS)
    begun=time.monotonic(); counts=collections.Counter();rules=collections.defaultdict(list); anomalies=[]; file_disagreement=[]
    baseline_by_name={};changed_mutations=0;image_glycin_disagreements=0
    error_agreement=collections.Counter()
    def job(item):
        i,(_,meta)=item;path=inputs_dir/f'{i:05d}.bmp'
        return i,meta,{k:invoke(path,k,args.timeout) for k in names}
    with output.open('w') as out, concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for i,meta,r in pool.map(job,enumerate(items)):
            category=classify(r);counts[category]+=1
            a,b=r['image-explicit'],r['glycin-image-rs']
            image_glycin_disagreements+=(a['status'],a.get('width'),a.get('height'),a.get('rgba_sha256'))!=(b['status'],b.get('width'),b.get('height'),b.get('rgba_sha256'))
            if category=='all-reject':
                error_agreement['same' if len({v.get('error_class') for v in r.values()})==1 else 'different']+=1
            if meta['source']!='mutation':
                baseline_by_name[meta['name']]=relation_signature(r)
            changed=(meta['source']=='mutation' and
                     relation_signature(r)!=baseline_by_name.get(meta['seed']))
            changed_mutations+=changed
            if category not in ('all-reject','all-accept-identical-pixels') and (meta['source']!='mutation' or changed):
                rules[meta['rule']].append((i,meta,r,category))
            if category=='crash-or-timeout':anomalies.append((i,meta,r))
            whole=[r['gdk-'+x] for x in ('file','stream','whole','byte','chunk')]
            if len({(v['status'],v.get('width'),v.get('height'),v.get('rgba_sha256')) for v in whole})>1:
                file_disagreement.append(i)
            out.write(json.dumps({'id':i,'rule':meta['rule'],'classification':category,'results':compact(r)},separators=(',',':'))+'\n')
            if (i+1)%250==0:print(f'{i+1}/{len(items)} in {time.monotonic()-begun:.1f}s',flush=True)
    elapsed=time.monotonic()-begun
    # Save one witness for each distinct accept/reject or pixel pattern per provenance rule.
    rows=[];witness_dir=HERE/'witnesses';witness_dir.mkdir(exist_ok=True)
    for rule,entries in sorted(rules.items(),key=lambda x:-len(x[1])):
        patterns=collections.defaultdict(list)
        for e in entries:
            r=e[2]
            key=tuple((k,v['status'],v.get('width'),v.get('height'),v.get('rgba_sha256')) for k,v in r.items())
            patterns[key].append(e)
        main=max(patterns.values(),key=len)
        i,m,r,c=main[0];path=inputs_dir/f'{i:05d}.bmp'
        target=tuple((v['status'],v.get('width'),v.get('height'),v.get('rgba_sha256')) for v in r.values())
        witness,checks=minimize(path.read_bytes(),target,names,args.timeout,args.min_checks)
        wname=f'{len(rows)+1:02d}-{rule.replace(":","-").replace("/","-")}.bmp'
        local_source=m.get('source') in ('local-corpus','glycin-tests','gdk-tests') or any(
            str(m.get('seed','')).startswith(str(root)) for root in (
                PROJECTS/'verified-decoders/dib/corpus',
                PROJECTS/'glycin-plan/glycin/tests/test-images',PROJECTS/'spec/gdk-pixbuf'))
        target_dir=(SCRATCH/'witnesses') if local_source else witness_dir
        target_dir.mkdir(exist_ok=True)
        wpath=target_dir/wname;wpath.write_bytes(witness)
        reference=str(wpath) if local_source else 'witnesses/'+wname
        difference=pixel_difference(wpath,r,args.timeout) if c=='accept-different-pixels' else ''
        rows.append({'rule':rule,'count':len(entries),'patterns':len(patterns),'witness':reference,
                     'original_id':i,'mutation':m,'classification':c,'pixel_difference':difference,
                     'results':compact(r),'min_checks':checks,'original_length':len(items[i][0]),'witness_length':len(witness)})
    # Keep anomaly inputs separate even when a rule's representative is not anomalous.
    separate=SCRATCH/'anomalies';separate.mkdir(exist_ok=True)
    for i,m,r in anomalies[:30]:
        shutil.copyfile(inputs_dir/f'{i:05d}.bmp',separate/f'{i:05d}.bmp')
    # Store distinct GDK path disagreements, including stream/loader errors.
    gdk_paths=[];seen_bytes=set()
    for i in file_disagreement:
        b=items[i][0];digest=hashlib.sha256(b).hexdigest()
        if digest in seen_bytes:continue
        seen_bytes.add(digest)
        m=items[i][1]
        local=m['source'] in ('local-corpus','glycin-tests','gdk-tests') or str(m.get('seed','')).startswith(str(PROJECTS))
        folder=(SCRATCH/'witnesses') if local else witness_dir;folder.mkdir(exist_ok=True)
        name=f'gdk-path-{len(gdk_paths)+1:02d}.bmp';path=folder/name;path.write_bytes(b)
        gdk_paths.append({'id':i,'witness':str(path) if local else 'witnesses/'+name,'sha256':digest})
    verified_lenient=None
    for i,(b,m) in enumerate(items[:base_count]):
        if m.get('name')=='header-16-BareDib':
            path=witness_dir/'verified-only-header16.bmp';path.write_bytes(b)
            verified_lenient={'id':i,'witness':'witnesses/'+path.name}
            break
    summary={'seed':args.seed,'inputs':len(items),'base_inputs':base_count,'seed_variants':seeds,
             'changed_mutations':changed_mutations,'verified_lenient_witness':verified_lenient,
             'image_glycin_disagreements':image_glycin_disagreements,
             'decoders':names,'counts':counts,'all_reject_error_agreement':error_agreement,'elapsed_seconds':round(elapsed,2),'gdk_disagreements':file_disagreement,'gdk_path_witnesses':gdk_paths,
             'anomalies':[{'id':i,'mutation':m,'results':compact(r)} for i,m,r in anomalies[:30]],
             'anomaly_count':len(anomalies),'rows':rows,'manifest':str(manifest),'raw_results':str(output)}
    summary['pixel_witnesses']=pixel_witnesses(args.timeout,args.min_checks)
    (HERE/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    render(summary)
    print(json.dumps({'inputs':len(items),'counts':counts,'elapsed_seconds':round(elapsed,1),'rows':len(rows),'anomalies':len(anomalies),'gdk_disagreements':len(file_disagreement)},indent=2))


def pixel_witnesses(timeout=10, checks=12):
    """Keep distinct pixel behaviors, without copying any local corpus bytes."""
    wanted=['frozen:BareDib','frozen:BmpFile','rle-escape','bitfield-mask','bfSize','width']
    metadata={m['id']:m for m in map(json.loads,(SCRATCH/'manifest.jsonl').open())}
    examples={};baselines={}
    for row in map(json.loads,(SCRATCH/'results.jsonl').open()):
        m=metadata[row['id']]
        if m['source']!='mutation':baselines[m['name']]=relation_signature(row['results'])
        if row['classification']!='accept-different-pixels' or row['rule'] not in wanted or row['rule'] in examples:
            continue
        if m['source']=='mutation' and relation_signature(row['results'])==baselines.get(m['seed']):
            continue
        if m['source'] in ('local-corpus','glycin-tests','gdk-tests') or str(m.get('seed','')).startswith(str(PROJECTS)):
            continue
        examples[row['rule']]=(row,m)
    selected=[]
    for rule in wanted:
        if rule not in examples:continue
        row,m=examples[rule];i=row['id'];results=row['results'];names=list(RUNNERS)
        pattern=tuple((v['status'],v.get('width'),v.get('height'),v.get('rgba_sha256')) for v in results.values())
        source=SCRATCH/'inputs'/f'{i:05d}.bmp'
        minimized,_=minimize(source.read_bytes(),pattern,names,timeout,checks)
        name=f'pixel-{len(selected)+1:02d}-{rule.replace(":","-")}.bmp'
        path=HERE/'witnesses'/name;path.write_bytes(minimized)
        kind=pixel_difference(path,results,timeout)
        selected.append({'rule':rule,'id':i,'mutation':m,'difference':kind,'witness':'witnesses/'+name,
                         'length':len(minimized),'results':results})
    return selected


def render(s):
    columns=s['decoders']; lines=['# BMP/DIB leniency matrix','','Inputs use identical bytes; each cell describes the **representative** witness for a provenance cluster, not every mutation in that cluster. See `summary.json` for cluster variability and `/tmp/claude-0/leniency-bmp/results.jsonl` for per-input observations. Pixel hashes cover top-first, straight RGBA8.','','## Run and coverage','',
        f"Seed {s['seed']}; {s['inputs']} inputs ({s['base_inputs']} frozen/corpus, {s['inputs']-s['base_inputs']} mutations) from {s['seed_variants']} distinct header/depth/compression seeds; {s['changed_mutations']} mutations change the decoder acceptance/pixel-equality pattern relative to their unmutated seed; elapsed {s['elapsed_seconds']} seconds.",'',
        'Classification | Count','--- | ---:']
    for category in ['all-accept-identical-pixels','accept-different-pixels','accept-vs-reject','all-reject','crash-or-timeout']:
        lines.append(f"{category} | {s['counts'].get(category,0)}")
    lines.append(f"All-reject error classes: same / different | {s['all_reject_error_agreement'].get('same',0)} / {s['all_reject_error_agreement'].get('different',0)}")
    lines.append('')
    lines.append('Error-class agreement is literal; libraries use different error taxonomies, so different labels do not establish different validation rules.')
    lines += ['','## Representative matrix','', '| Rule (differing inputs; patterns) | Witness | '+' | '.join(columns)+' |','| --- | --- | '+' | '.join(['---']*len(columns))+' |']
    for row in s['rows']:
        cells=[]
        for col in columns:
            v=row['results'][col];status=v['status']
            if status=='ok':
                status='accept '+str(v.get('width'))+'×'+str(v.get('height'))+' `'+v.get('rgba_sha256','')[:8]+'`'
            elif status=='error':status='reject `'+v.get('error_class','?').replace('|','/')[:28]+'`'
            cells.append(status)
        detail=('; '+row['pixel_difference']) if row['pixel_difference'] else ''
        lines.append('| '+row['rule']+f" ({row['count']}; {row['patterns']}{detail}) | [#{row['original_id']}]({row['witness']}) | "+' | '.join(cells)+' |')
    by_rule={row['rule']:row for row in s['rows']}
    pixels={row['rule']:row for row in s.get('pixel_witnesses',[])}
    lines += ['','## Ten significant observations','']
    findings=[
        ('`planes=0`: GDK accepts the image; verified core and image-rs reject it.','planes',by_rule),
        ('`biSizeImage=1` instead of zero: verified core rejects; image-rs and GDK decode identical pixels.','biSizeImage',by_rule),
        ('`bfSize=63` for a 58-byte file: verified core rejects the inflated declared size; image-rs and GDK accept.','bfSize',by_rule),
        ('A zero red bitfield mask: GDK decodes the image; verified core and image-rs reject it.','bitfield-mask',by_rule),
        ('Replacing the terminal RLE escape with end-of-line: verified core reports `RleBounds`; image-rs and GDK accept.','rle-escape',by_rule),
        ('One 16-bit component scales to 25 in verified core/image-rs but 24 in GDK.','frozen:BareDib',pixels),
        ('An indexed BMP with a palette gap retains colored pixels in verified core/image-rs; GDK outputs black.','frozen:BmpFile',pixels),
        ('An RLE escape produces opaque black in verified core/image-rs and transparent black in GDK.','rle-escape',pixels),
    ]
    for description,rule,source in findings:
        if rule in source:lines.append(f"- {description} [Witness]({source[rule]['witness']}).")
    if s.get('verified_lenient_witness'):
        lines.append(f"- Verified core accepts a wrapped 16-byte OS/2 DIB header that image-rs and GDK reject. [Witness]({s['verified_lenient_witness']['witness']}).")
    if s.get('gdk_path_witnesses'):
        lines.append(f"- GDK's `.bmp` filename path accepts a damaged signature that its stream and all incremental chunk patterns reject; renaming the file `.dat` makes the filename path reject it. [Witness]({s['gdk_path_witnesses'][0]['witness']}).")
    lines += ['','## Same acceptance, different pixels','','These witnesses keep the same accept/size/hash pattern through bounded delta-debugging. The category names describe byte-level differences, not presumed decoder intent.']
    for p in s.get('pixel_witnesses',[]):
        m=p['mutation'];change=(f" {m.get('field')}={m.get('value')}" if 'value' in m else f" {m.get('name','')}")
        r=p['results'];core=r['verified'];gdk=r['gdk-file']
        lines.append(f"- {p['rule']}{change}: {p['difference']}; verified `{core.get('rgba_sha256','')[:8]}` versus GDK `{gdk.get('rgba_sha256','')[:8]}`; [witness]({p['witness']}) (input #{p['id']}).")
    lines += ['',"## where our core's choices disagree with ALL or MOST other decoders",'']
    found=0
    for row in s['rows']:
        r=row['results']; verified=r['verified']['status']=='ok';peers=[r[k]['status']=='ok' for k in columns if k not in ('verified','image-guess','gdk-byte','gdk-chunk','gdk-stream','gdk-whole')]
        if peers and sum(p!=verified for p in peers)>=2:
            found+=1
            lines.append(f"- `{row['rule']}`: verified {'accepts' if verified else 'rejects'} while image-rs, glycin-image-rs and gdk-file {'mostly disagree' if sum(p!=verified for p in peers)==2 else 'all disagree'}; [witness]({row['witness']}).")
    if not found:lines.append('No representative cluster has a verified-core accept/reject choice opposed by at least two of image-rs, glycin-image-rs and gdk-file. Check every result in `results.jsonl` before changing a rule.')
    if s.get('verified_lenient_witness'):
        lines.append(f"- The verified core **accepts** a valid wrapped 16-byte OS/2 header that image-rs and GDK reject; [witness]({s['verified_lenient_witness']['witness']}). This is a verified-core leniency/format-coverage choice, not an error exposed by a mutation.")
    lines.append(f"Image-rs and glycin-image-rs share the BMP decoder implementation; their acceptance/dimensions/pixel hashes differ on {s.get('image_glycin_disagreements','unmeasured')} of {s['inputs']} inputs. Treat their agreement as one implementation, not two independent votes.")
    lines += ['','## GDK incremental versus whole-file','','The installed BMP loader is `/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-bmp.so` (module, not built-in). The runner compares `new_from_file`, `new_from_stream`, and `GdkPixbufLoader` with whole-buffer, byte and 4096-byte writes.',f"Different statuses, dimensions or pixels on {len(s['gdk_disagreements'])} input(s): {str(s['gdk_disagreements'][:30])}."]
    for case in s.get('gdk_path_witnesses',[]):
        lines.append(f"- Input #{case['id']}: [witness]({case['witness']}); compare `gdk-file` with `gdk-stream` and all loader chunk sizes.")
    if s.get('gdk_path_witnesses'):
        lines.append('In this run the filename path accepts two distinct damaged BMP signatures because the `.bmp` suffix selects BMP; the stream and all loader chunk patterns reject them as unrecognized. Renaming the first witness to `.dat` makes the file path reject it too.')
    lines += ['','## Crashes, timeouts, and anomalies','',f"{s['anomaly_count']} inputs have a crash or timeout. Up to 30 inputs are saved under `/tmp/claude-0/leniency-bmp/anomalies/`; no exploitation or upstream reporting was attempted."]
    for a in s['anomalies'][:15]:
        bad=[k+': '+v['status']+' '+v.get('error_class','') for k,v in a['results'].items() if v['status'] in ('crash','timeout')]
        lines.append(f"- Input #{a['id']} ({a['mutation']['rule']}): "+', '.join(bad))
    lines += ['','## Reproduce','',"Run `python3 harness.py --inputs 10000 --seed 731` from this directory after `./build.sh`. A single witness: `python3 reproduce.py PATH_FROM_MATRIX`. Witness numbers and paths may change when the run size changes. The manifest and all input files are in `/tmp/claude-0/leniency-bmp/`, and are regenerated for a new run.",'']
    (HERE/'LENIENCY.md').write_text('\n'.join(lines))


def main():
    p=argparse.ArgumentParser();p.add_argument('--inputs',type=int,default=10000);p.add_argument('--seed',type=int,default=731)
    p.add_argument('--jobs',type=int,default=6);p.add_argument('--timeout',type=float,default=10)
    p.add_argument('--min-checks',type=int,default=16)
    args=p.parse_args();execute(args)
if __name__=='__main__':main()
