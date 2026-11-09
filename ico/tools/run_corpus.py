#!/usr/bin/env python3
"""Measure the 13 local ICO/CUR files without copying their bytes."""
import importlib.util
from collections import Counter
from pathlib import Path

HERE=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('ico_oracle',HERE/'spec/check_vectors.py')
ico=importlib.util.module_from_spec(spec);spec.loader.exec_module(ico)
dib=ico.load_dib()
CORPUS=Path('/home/ws/projects/dib-verus/corpus')

def main():
    files=sorted((*CORPUS.glob('*.ico'),*CORPUS.glob('*.cur')))
    totals=Counter()
    for path in files:
        raw=path.read_bytes()
        try:entries=ico.parse(raw)
        except ico.Invalid as exc:
            totals['rejected']+=1;totals['parse:'+str(exc)]+=1
            print(f'{path.name}: reject {exc}')
            continue
        totals['accepted']+=1
        print(f'{path.name}: accept {len(entries)} entries')
        for index,e in enumerate(entries):
            status=e['status']
            delegated=ico.actual(raw,'decode',index,dib,entries)
            if 'invalid' in status:
                totals['invalid_entry']+=1
                assert delegated=={'error':status['invalid']},path.name
                print(f'  {index}: invalid {status["invalid"]}; decode agrees')
                continue
            a,z=status['valid']['payload']
            if status['valid']['kind']=='Png':
                totals['png']+=1
                assert delegated=={'error':'PngDelegation'},path.name
                print(f'  {index}: PNG delegate [{a},{z}) nominal {e["width"]}x{e["height"]}')
                continue
            totals['dib']+=1
            independent=dib.actual(raw[a:z],'IcoEntryDibSlack')
            expected=independent if 'error' not in independent else {'error':'Dib('+independent['error']+')'}
            assert delegated==expected,path.name
            if 'error' in delegated:
                totals['dib_reject']+=1
                print(f'  {index}: DIB {delegated["error"]}; 1.1 oracle agreement')
            else:
                totals['dib_ok']+=1
                print(f'  {index}: DIB {delegated["width"]}x{delegated["height"]} RGBA; 1.1 oracle agreement')
    print('TOTAL',len(files),dict(sorted(totals.items())))

if __name__=='__main__':main()
