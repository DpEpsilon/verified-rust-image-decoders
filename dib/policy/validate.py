#!/usr/bin/env python3
"""Compare pure-byte policy results with independent native library observations.

Reads seed-731 observations unchanged; fresh seeds reuse harness.generate/invoke.
The novel slice excludes distinct bytes from both tuning seeds; seed 9091
also excludes review seeds 4242 and 81573, and 31337 excludes 9091 too.
Reports exact RGBA agreement among native-accepted inputs separately from
both-reject agreement. Native observations cover BMP files, not BareDib.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
MATRIX=Path('/tmp/claude-0/leniency-bmp')
LENIENCY=Path('/home/ws/projects/leniency/bmp')
sys.path.insert(0,str(HERE))
sys.path.insert(0,str(LENIENCY))
import harness  # imports shared engine, source_inputs, generate and native invoke
import policy_oracle as oracle

TARGETS={'GdkCompat-2.42.12':['gdk-file','gdk-stream','gdk-whole','gdk-byte','gdk-chunk'],
         'ImageCrateCompat-0.25.10':['image-explicit']}
RULE_OPTION={'bfSize':'file_size','bfOffBits':'offset','biSizeImage':'image_size',
             'planes':'planes','biClrUsed':'indexed_count/high_count',
             'biClrImportant':'important','truncation':'short_pixels',
             'bitfield-mask':'masks','rle-escape':'rle_eol/rle_delta/rle_eob',
             'bitCount':'unsupported','compression':'unsupported',
             'biSize':'headers/header_bytes','trailing-bytes':'tail',
             'width':'dimensions/short_pixels','height':'dimensions/short_pixels',
             'random-bitflip':'mixed','inconsistent-pair':'mixed'}

def observation(b,profile):
    try:
        w,h,rgba,_=oracle.decode_with_policy(b,'BmpFile',profile)
        return ('ok',w,h,hashlib.sha256(rgba).hexdigest())
    except oracle.Invalid:return ('error',)

def signature(r):
    return ('ok',r['width'],r['height'],r['rgba_sha256']) if r['status']=='ok' else (r['status'],)

def reason(b,rule,ours,theirs,target,path_difference=False):
    """Label an observation, never infer a root cause from generator provenance."""
    if path_difference:
        return 'filename/stream observations differ on identical bytes'
    if theirs[0]=='ok' and (theirs[1]>oracle.MAX_DIMENSION or theirs[2]>oracle.MAX_DIMENSION or 4*theirs[1]*theirs[2]>oracle.MAX_OUTPUT):
        return 'native accepted dimensions/output beyond universal caps'
    if ours[0]=='ok' and theirs[0]=='ok':
        return 'both accepted; RGBA bytes or dimensions differ'
    if ours[0]=='ok':return 'only policy oracle accepted'
    if theirs[0]=='ok':return 'only native library accepted'
    return 'both rejected with distinct native status; inspect invocation'

def tally(rows,manifest,paths,profiles=TARGETS,include_ids=None):
    report={p:{t:{'total':0,'agree':0,'both_reject':0,'native_accepted':0,
                    'accepted_same_pixels':0,'native_only':0,'oracle_only':0,
                    'both_accept_different':0,'residuals':[],
                    'options':collections.defaultdict(lambda:{'total':0,'agree':0}),
                    'clusters':collections.defaultdict(lambda:{'total':0,'agree':0,'reasons':collections.Counter(),'examples':[]})}
               for t in targets} for p,targets in profiles.items()}
    for item in rows:
        i=item['id']
        if include_ids is not None and i not in include_ids:continue
        b=paths(i);meta=manifest[i];rule=meta['rule']
        for profile,targets in profiles.items():
            ours=observation(b,profile)
            for target in targets:
                theirs=signature(item['results'][target]);cell=report[profile][target]
                same=ours==theirs;cell['total']+=1;cell['agree']+=same
                cell['both_reject']+=ours[0]!='ok' and theirs[0]!='ok'
                cell['native_accepted']+=theirs[0]=='ok'
                cell['accepted_same_pixels']+=theirs[0]=='ok' and same
                cell['native_only']+=theirs[0]=='ok' and ours[0]!='ok'
                cell['oracle_only']+=theirs[0]!='ok' and ours[0]=='ok'
                cell['both_accept_different']+=theirs[0]=='ok' and ours[0]=='ok' and not same
                cluster=cell['clusters'][rule];cluster['total']+=1;cluster['agree']+=same
                category=RULE_OPTION.get(rule,rule)
                option=cell['options'][category];option['total']+=1;option['agree']+=same
                if not same:
                    path_difference=(target.startswith('gdk-') and
                        ours in (signature(item['results']['gdk-file']),signature(item['results']['gdk-stream'])) and
                        signature(item['results']['gdk-file'])!=signature(item['results']['gdk-stream']))
                    why=reason(b,rule,ours,theirs,target,path_difference)
                    cluster['reasons'][why]+=1
                    cell['residuals'].append({'id':i,'rule':rule,'classification':why})
                    if len(cluster['examples'])<5:cluster['examples'].append({'id':i,'ours':ours,'library':theirs})
    result={}
    for p,targets in report.items():
        result[p]={}
        for t,c in targets.items():
            result[p][t]={k:c[k] for k in ('total','agree','both_reject','native_accepted',
                'accepted_same_pixels','native_only','oracle_only','both_accept_different')}
            result[p][t].update({
                'options':dict(c['options']),
                'clusters':{r:{'total':v['total'],'agree':v['agree'],
                    'reasons':dict(v['reasons']),'examples':v['examples']}
                    for r,v in c['clusters'].items() if v['agree']!=v['total']},
                'residuals':c['residuals']})
    return result

def existing():
    rows=map(json.loads,(MATRIX/'results.jsonl').open())
    metadata={m['id']:m for m in map(json.loads,(MATRIX/'manifest.jsonl').open())}
    assert len(metadata)==10000
    return tally(rows,metadata,lambda i:(MATRIX/'inputs'/f'{i:05d}.bmp').read_bytes())

def fresh(seed,count,timeout,reuse=False):
    items,_,_=harness.generate(count,seed)
    folder=Path(f'/tmp/claude-0/dib-policy/seed-{seed}')
    folder.mkdir(parents=True,exist_ok=True)
    selected=list({t for targets in TARGETS.values() for t in targets})
    metadata={i:m for i,(_,m) in enumerate(items)}
    output=folder/'native-results.jsonl'
    if reuse and output.exists():
        def replay():
            seen=0
            for row in map(json.loads,output.open()):
                seen+=1
                yield row
            if seen!=count:raise ValueError('incomplete cached native result set')
        return tally(replay(),metadata,lambda i:items[i][0])
    def records():
        with output.open('w') as saved:
            for i,(b,m) in enumerate(items):
                path=folder/f'{i:05d}.bmp';path.write_bytes(b)
                results={t:harness.invoke(path,t,timeout) for t in selected}
                if (i+1)%500==0:print(f'fresh {i+1}/{count}',flush=True)
                row={'id':i,'results':results}
                saved.write(json.dumps(row)+'\n')
                yield row
    return tally(records(),metadata,lambda i:items[i][0])

def novel(seed=4242,count=10000):
    """Use distinct bytes absent from tuning and earlier reviewed seeds."""
    native=Path(f'/tmp/claude-0/dib-policy/seed-{seed}/native-results.jsonl')
    if not native.exists() and seed==4242:
        native=Path('/tmp/claude-0/rev/s4242/native_0_10000.jsonl')
    if not native.exists() and seed in (9091,31337):
        native=Path(f'/tmp/claude-0/rev/s{seed}/native.jsonl')
    if not native.exists():
        raise FileNotFoundError(f'collect native observations for novel seed {seed} before validation')
    training={hashlib.sha256((MATRIX/'inputs'/f'{i:05d}.bmp').read_bytes()).digest()
              for i in range(10000)}
    training.update(hashlib.sha256(b).digest() for b,_ in harness.generate(count,20261109)[0])
    if seed in (9091,31337):
        for earlier in ((4242,81573) if seed==9091 else (4242,81573,9091)):
            training.update(hashlib.sha256(b).digest() for b,_ in harness.generate(count,earlier)[0])
    items,_,_=harness.generate(count,seed)
    unseen=set();chosen=set()
    for i,(b,_) in enumerate(items):
        digest=hashlib.sha256(b).digest()
        if digest not in training and digest not in unseen:
            chosen.add(i);unseen.add(digest)
    metadata={i:m for i,(_,m) in enumerate(items)}
    def records():
        seen_ids=set()
        for row in map(json.loads,native.open()):
            seen_ids.add(row['id'])
            yield row
        if seed in (9091,31337) and native.name=='native.jsonl':
            if seen_ids!=chosen:raise ValueError('reviewer novel native IDs differ from independently selected IDs')
        elif len(seen_ids)!=count:raise ValueError('incomplete cached novel native result set')
    return tally(records(),metadata,lambda i:items[i][0],include_ids=chosen),len(training),len(chosen)

def main():
    p=argparse.ArgumentParser();p.add_argument('--fresh-seed',type=int,default=20261109)
    p.add_argument('--fresh-inputs',type=int,default=10000)
    p.add_argument('--timeout',type=float,default=10)
    p.add_argument('--reuse-native',action='store_true',help='reuse complete fresh native observations')
    p.add_argument('--only-existing',action='store_true')
    p.add_argument('--novel-seed',type=int,default=4242,
                   help='measure only unique bytes absent from seeds 731 and 20261109')
    p.add_argument('--skip-novel',action='store_true')
    args=p.parse_args()
    report={'seed-731':existing()}
    if not args.only_existing:report[f'seed-{args.fresh_seed}']=fresh(args.fresh_seed,args.fresh_inputs,args.timeout,args.reuse_native)
    if not args.skip_novel and not args.only_existing:
        report[f'novel-seed-{args.novel_seed}'],training_count,novel_count=novel(args.novel_seed)
        print(f'novel bytes: {novel_count}; distinct training bytes: {training_count}')
    output=Path('/tmp/claude-0/dib-policy/agreements.json')
    output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(report,indent=2)+'\n')
    for seed,profiles in report.items():
        for profile,targets in profiles.items():
            for target,c in targets.items():
                print(f'{seed} {profile}/{target}: {c["agree"]}/{c["total"]}; '
                      f'native-accepted exact {c["accepted_same_pixels"]}/{c["native_accepted"]}; '
                      f'both reject {c["both_reject"]}; residual clusters '+
                      ', '.join(f'{r}={v["total"]-v["agree"]}' for r,v in sorted(c['clusters'].items(),key=lambda a:a[1]['agree']-a[1]['total'])[:8]))
    print(output)

if __name__=='__main__':main()
