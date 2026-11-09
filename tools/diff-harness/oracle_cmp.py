#!/usr/bin/env python3
"""Compare out/oracle_sample.txt (verus V / creusot C lines) with check_vectors.py's implementation."""
import sys, importlib.util, collections, random
spec = importlib.util.spec_from_file_location('cv', '../../icns/spec-history/check_vectors.py')
cv = importlib.util.module_from_spec(spec); spec.loader.exec_module(cv)
PER_KIND = int(sys.argv[1]) if len(sys.argv) > 1 else 3000

def fnv(d):
    h = 0xcbf29ce484222325
    for b in d:
        h ^= b; h = (h * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h

def pix(side, data): return 'OK:%d,%d,%d,%016x' % (side, side, len(data), fnv(data))
def run(f):
    try: return f()
    except cv.DecodeError as e: return 'ERR:' + str(e)

def parse_txt(es):
    return 'OK:' + ''.join('%s:%d-%d:%d:%s;' % (e['type_hex'], e['payload'][0], e['payload'][1], int(e['opaque_candidate']),
                           '-' if e['nominal_side'] is None else e['nominal_side']) for e in es)

cases = []
cur = None
for line in open('out/oracle_sample.txt'):
    line = line.rstrip('\n')
    if line.startswith('CASE '):
        _, kind, h = line.split(' ', 2)
        cur = {'kind': kind, 'hex': h, 'V': {}, 'C': {}}
        cases.append(cur)
    else:
        side, rest = line.split(' ', 1)
        key, val = rest.rsplit(' ', 1) if not rest.startswith('parse') else ('parse', rest[6:])
        cur[side][key] = val
bykind = collections.defaultdict(list)
for c in cases: bykind[c['kind']].append(c)
random.seed(12345)
chosen = []
for k, v in sorted(bykind.items()):
    random.shuffle(v); chosen += v[:PER_KIND]
cnt = collections.Counter(); hist = collections.Counter(); div = collections.defaultdict(list)
for c in chosen:
    data = bytes.fromhex(c['hex']); cnt[c['kind']] += 1
    try: ents = cv.parse(data); pt = parse_txt(ents)
    except cv.DecodeError as e: ents = None; pt = 'ERR:' + str(e)
    exp = {'parse': pt}
    if ents is not None:
        for key in c['V']:
            if key == 'parse': continue
            p = key.split(); a = lambda s: None if s == '-' else int(s)
            if p[0] == 'rgb': exp[key] = run(lambda: pix(*cv.rgb(data, ents, int(p[1]))))
            elif p[0] == 'mask': exp[key] = run(lambda: pix(*cv.mask(data, ents, int(p[1]))))
            else: exp[key] = run(lambda: pix(*cv.combine(data, ents, a(p[1]), a(p[2]))))
    for side in 'VC':
        for key, val in c[side].items():
            hist[(c['kind'], key.split()[0], val.split(':')[0] if not val.startswith('ERR') else val) ] += 1 if side == 'V' else 0
            e = exp.get(key, 'NOENTRIES')
            if e != val:
                div[(side, key.split()[0], e[:40], val[:40])].append(c['hex'])
print('cases per kind:', dict(cnt), 'total', sum(cnt.values()))
print('divergences vs oracle (side, op, oracle, impl): count')
for k, v in div.items(): print(k, len(v), min(v, key=len)[:200])
print('no divergences' if not div else '')
print('\nVerus result-class histogram over the oracle sample (kind, op, class): count')
agg = collections.Counter()
for (k, op, cl), n in hist.items(): agg[(op, cl)] += n
for (op, cl), n in sorted(agg.items()): print(' ', op, cl, n)
