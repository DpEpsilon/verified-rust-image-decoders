#!/usr/bin/env python3
"""Driver: equality check, then interleaved timing rounds, for verified vs incumbent decoders.

  python3 -I bench.py FORMAT PART [--rounds 3] [--passes 9] [--warmup 2]
  FORMAT = bmp|tga|icns|ico, PART = corpus|large
Corpus files are read from $CORPUS_DIR/<format>/files (default ~/corpora; see
verified-decoders/leniency/corpora/README.md).
Binaries come from /tmp/claude-0/bench-target/release/bench and /tmp/claude-0/bench-bin/gdkbench.
Every benchmark process is pinned to one core with `taskset -c $BENCH_CPU` (default 2); decoder names with
prefix `old:` run the pre-migration runner ($BENCH_RS_OLD, used by ab.py).
Results (no input bytes) go to results/FORMAT-PART.json and results/FORMAT-PART.md.
"""
import argparse, collections, glob, hashlib, json, os, statistics, subprocess, sys, time
sys.dont_write_bytecode = True

HERE = os.path.dirname(os.path.abspath(__file__))
RS = '/tmp/claude-0/bench-target/release/bench'
RS_OLD = os.environ.get('BENCH_RS_OLD', '/tmp/claude-0/bench-old-target/release/bench')
GDK = '/tmp/claude-0/bench-bin/gdkbench'
PIN = ['taskset', '-c', os.environ.get('BENCH_CPU', '2')]
ICNS_TIMED_CAP = 835  # deterministic cap on the timed ICNS subset: first N files by SHA-256 of content
LARGE = '/tmp/claude-0/bench-large'
CORPORA = os.path.expanduser(os.environ.get('CORPUS_DIR', '~/corpora'))
DIB_CORPUS = os.path.expanduser('~/projects/verified-decoders/dib/corpus')
WORK = '/tmp/claude-0/bench-out'
CFG = {
    'bmp': dict(ver='bmp-verified', variants=['bmp-verified-policy-strict', 'bmp-verified-policy-recommended'],
                extras=['bmp-verified-policy-gdk', 'bmp-verified-policy-imagecompat'],
                inc=['bmp-image', 'gdk:bmp'],
                corpus=lambda: dedupe(sorted(glob.glob(CORPORA + '/bmp/files/*')) + sorted(glob.glob(DIB_CORPUS + '/*.bmp'))),
                large=lambda: sorted(glob.glob(LARGE + '/*bmp*.bmp'))),
    'tga': dict(ver='tga-verified', variants=['tga-verified-policy-strict', 'tga-verified-policy-recommended'],
                extras=['tga-verified-policy-gdk', 'tga-verified-policy-imagecompat'], inc=['tga-image', 'tga-tinytga', 'gdk:tga'],
                corpus=lambda: sorted(glob.glob(CORPORA + '/tga/files/*')),
                large=lambda: sorted(glob.glob(LARGE + '/*tga*.tga'))),
    'icns': dict(ver='icns-verified', variants=[], inc=['icns-crate', 'gdk:icns'],
                 corpus=lambda: sorted(glob.glob(CORPORA + '/icns/files/*')),
                 large=lambda: sorted(glob.glob(LARGE + '/*.icns'))),
    'ico': dict(ver='ico-verified', variants=[], inc=['ico-crate', 'ico-image', 'gdk:ico'],
                corpus=lambda: dedupe(sorted(glob.glob(CORPORA + '/ico/files/*')) + sorted(glob.glob(DIB_CORPUS + '/*.ico'))),
                large=lambda: sorted(glob.glob(LARGE + '/*.ico'))),
}

def cmd(dec, mode, lst, *extra):
    if dec.startswith('gdk:'):
        return PIN + [GDK, dec[4:], mode, lst, *map(str, extra)]
    if dec.startswith('old:'):
        return PIN + [RS_OLD, dec[4:], mode, lst, *map(str, extra)]
    return PIN + [RS, dec, mode, lst, *map(str, extra)]

def sha256_file(f):
    return hashlib.sha256(open(f, 'rb').read()).hexdigest()

def dedupe(files):
    """Drop later files whose content duplicates an earlier one (the dib corpus is a subset of $CORPUS_DIR/bmp)."""
    seen, out = set(), []
    for f in files:
        h = sha256_file(f)
        if h not in seen:
            seen.add(h); out.append(f)
    return out

def busy():
    """Other processes using >=25% CPU (ps %cpu is lifetime-averaged, so this catches sustained jobs)."""
    out = subprocess.run(['ps', '-eo', 'pid,pcpu,args', '--sort=-pcpu'], capture_output=True, text=True).stdout.splitlines()[1:6]
    me = str(os.getpid())
    return [l.strip()[:110] for l in out if float(l.split()[1]) >= 25 and l.split()[0] != me and 'bench' not in l.split(None, 2)[2][:40] and 'ps -eo' not in l]

def run(c, timeout=900):
    return subprocess.run(c, capture_output=True, text=True, timeout=timeout)

def check(dec, lst):
    try:
        r = run(cmd(dec, 'check', lst), timeout=3600)  # first read of a cold 668 MB ICNS corpus took ~9 min
    except subprocess.TimeoutExpired:
        print('!!! check timed out', dec, file=sys.stderr); raise SystemExit(87)
    res = {}
    for l in r.stdout.splitlines():
        p = l.split('\t')
        if p[0] == 'C':
            res[p[1]] = dict(status=p[2], w=int(p[3]), h=int(p[4]), sha=p[5] if p[2] == 'ok' else None, msg=p[5] if p[2] != 'ok' else '')
    if r.returncode == 86:
        print('!!! HARD FAILURE (OutOfMemory from a verified core) while checking', dec, r.stderr[:300], file=sys.stderr)
        raise SystemExit(86)
    if len(res) == 0 and r.returncode != 0:
        print('check failed', dec, r.stderr[:200], file=sys.stderr)
    return res

def write_list(name, paths):
    os.makedirs(WORK + '/lists', exist_ok=True)
    p = f'{WORK}/lists/{name}.txt'
    open(p, 'w').write('\n'.join(paths) + '\n')
    return p

def load_snapshot():
    """1-minute load average (reads 0 in this gVisor sandbox) and the number of other runnable (state R) processes."""
    ps = subprocess.run(['ps', '-eo', 'stat,args'], capture_output=True, text=True).stdout.splitlines()[1:]
    other = [l for l in ps if l.startswith('R') and 'ps -eo' not in l and 'release/bench' not in l and 'gdkbench' not in l]
    return dict(load1=float(open('/proc/loadavg').read().split()[0]), runnable_others=len(other))

def timed(dec, lst, rounds_state, passes, warmup):
    la0 = load_snapshot()
    r = run(cmd(dec, 'time', lst, passes, warmup), timeout=3600)
    la1 = load_snapshot()
    if r.returncode == 86:
        print('!!! HARD FAILURE (OutOfMemory from a verified core) timing', dec, r.stderr[:300], file=sys.stderr)
        raise SystemExit(86)
    for l in r.stdout.splitlines():
        p = l.split('\t')
        if p[0] == 'T':
            return dict(ns=[int(x) for x in p[1].split(',')], cpu=[int(x) for x in p[2].split(',')], pixels=int(p[3]), files=int(p[4]),
                        base_kb=int(p[5]), hwm_kb=int(p[6]), total_cpu=int(p[7]), load=[la0, la1])
    return dict(error=(r.stderr or r.stdout)[:300], rc=r.returncode)

def measure(decs, lst, rounds, passes, warmup, tag):
    """Interleave decoders: round 1 runs every decoder, then round 2, ... Each run is a fresh process."""
    runs = collections.defaultdict(list)
    load0 = open('/proc/loadavg').read().split()[:3]
    flags = []
    for rd in range(rounds):
        for d in decs:
            b = busy()
            if b:
                time.sleep(5); b = busy()
            if b: flags.append(f'round {rd+1} {d}: other CPU load: {b[0]}')
            runs[d].append(timed(d, lst, rd, passes, warmup))
    out = {}
    for d, rs in runs.items():
        if any('error' in r for r in rs):
            out[d] = dict(error=[r.get('error') for r in rs if 'error' in r][0]); continue
        allw = [n for r in rs for n in r['ns']]
        # Primary metric: CPU time. The sandbox clock has 10 ms granularity, so per-pass CPU values are quantised;
        # use each process's total CPU over all its timed passes / passes (one reading, error < 10 ms in seconds).
        per_proc = [r['total_cpu'] / len(r['ns']) for r in rs]
        meds = per_proc
        px = rs[0]['pixels']; med = statistics.median(per_proc); wmed = statistics.median(allw)
        allns = per_proc
        out[d] = dict(pixels=px, files=rs[0]['files'], samples=len(allw), median_ns=med, min_ns=min(per_proc), max_ns=max(per_proc),
                      ns_per_px=med / px, min_ns_per_px=min(per_proc) / px, mbps=px * 4 / med * 1000,
                      cpu_over_wall=statistics.median(per_proc) / wmed,
                      wall_ns_per_px=wmed / px, wall_min_ns_per_px=min(allw) / px, wall_spread=(max(allw) - min(allw)) / wmed,
                      load=[r['load'] for r in rs],
                      spread=(max(allns) - min(allns)) / med, round_spread=max(meds) / min(meds) - 1,
                      hwm_mb=statistics.median(r['hwm_kb'] for r in rs) / 1024, base_mb=statistics.median(r['base_kb'] for r in rs) / 1024)
    return dict(decoders=out, load_before=load0, load_after=open('/proc/loadavg').read().split()[:3], flags=flags)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('format'); ap.add_argument('part')
    ap.add_argument('--rounds', type=int, default=5); ap.add_argument('--passes', type=int, default=None)
    ap.add_argument('--warmup', type=int, default=None); ap.add_argument('--no-time', action='store_true')
    a = ap.parse_args()
    cfg = CFG[a.format]; large = a.part == 'large'
    passes = a.passes or (5 if large else 7); warmup = a.warmup if a.warmup is not None else (1 if large else 2)
    files = cfg[a.part]()
    allp = write_list(f'{a.format}-{a.part}-all', files)
    extras = cfg.get('extras', [])
    decs = [cfg['ver']] + cfg['variants'] + extras + cfg['inc']
    chk = {d: check(d, allp) for d in decs}
    result = dict(format=a.format, part=a.part, n_files=len(files), passes=passes, warmup=warmup, rounds=a.rounds, agreement={}, timing={})
    ver = chk[cfg['ver']]
    def ok(d, f): return chk[d].get(f, {}).get('status') == 'ok'
    def same(d, f): return ok(d, f) and chk[d][f]['sha'] == ver[f]['sha'] and (chk[d][f]['w'], chk[d][f]['h']) == (ver[f]['w'], ver[f]['h'])
    print(f"verified ok on {sum(ok(cfg['ver'], f) for f in files)}/{len(files)} files", flush=True)
    result['verified_ok'] = sum(ok(cfg['ver'], f) for f in files)
    result['verified_errors'] = dict(collections.Counter(ver[f]['msg'][:40] for f in files if not ok(cfg['ver'], f)).most_common(8))
    result['agreement_profiles'] = {}
    for x in cfg['variants'] + extras:
        ag = collections.Counter(('same' if same(x, f) else 'DIFFERENT-output') if ok(x, f) and ok(cfg['ver'], f) else 'ok-only-here' if ok(x, f) else 'ok-only-plain' if ok(cfg['ver'], f) else 'both-error' for f in files)
        result['agreement_profiles'][x] = dict(ag); print('profile', x, 'vs', cfg['ver'], dict(ag), flush=True)
    for inc in cfg['inc']:
        ag = collections.Counter(); examples = []
        for f in files:
            v, i = ok(cfg['ver'], f), ok(inc, f)
            k = ('both-ok-equal' if same(inc, f) else 'both-ok-DIFFERENT') if v and i else 'verified-only-ok' if v else 'incumbent-only-ok' if i else 'both-error'
            ag[k] += 1
            if k == 'both-ok-DIFFERENT' and len(examples) < 5:
                examples.append(dict(file=os.path.basename(f), ver=[ver[f]['w'], ver[f]['h']], inc=[chk[inc][f]['w'], chk[inc][f]['h']]))
        result['agreement'][inc] = dict(counts=dict(ag), examples=examples)
        print(inc, dict(ag), flush=True)
    if a.no_time:
        json.dump(result, open(f'{HERE}/results/{a.format}-{a.part}.json', 'w'), indent=1); return
    if large:
        for f in files:
            base = os.path.basename(f)
            verok = ok(cfg['ver'], f)
            decl = [d for d in decs if (same(d, f) if verok else (ok(d, f) and d in cfg['inc']))]
            if not decl: result['timing'][base] = dict(note='no decoder succeeded', ver_msg=ver[f]['msg'], size=os.path.getsize(f)); continue
            if '16384' in base: result['timing'][base] = dict(note='probe, not timed', size=os.path.getsize(f), decoders_ok=decl, ver_msg=ver[f]['msg']); continue
            lst = write_list(f'{a.format}-large-{base}', [f])
            print('timing', base, decl, flush=True)
            t = measure(decl, lst, a.rounds, passes, warmup, base)
            t.update(size=os.path.getsize(f), verified_ok=verok, ver_msg=ver[f]['msg'], skipped_disagree=[d for d in decs if verok and ok(d, f) and not same(d, f)])
            result['timing'][base] = t
    else:
        if a.format == 'icns':
            # Applied to each pair's agreed set (files both decoders decode identically): only 43 and 101 files
            # agree out of 15,701, so a cap applied before the gate would leave ~2-6 timed files.
            result['timed_subset'] = f'per pair: first {ICNS_TIMED_CAP} files by SHA-256 of content among the files both decoders decode identically (gate ran on all {len(files)})'
        for inc in cfg['inc']:
            sel = [f for f in files if same(inc, f) and all(same(v, f) for v in cfg['variants'])]
            if a.format == 'icns': sel = sorted(sel, key=sha256_file)[:ICNS_TIMED_CAP]
            if not sel: continue
            sel2 = [f for f in sel if all(same(v, f) for v in extras)]
            full = len(sel2) == len(sel)
            lst = write_list(f'{a.format}-corpus-{inc.replace(":", "_")}', sel)
            print('timing pair', inc, len(sel), 'extras on same set' if full else f'extras on {len(sel2)}-file subset in a second call', flush=True)
            t = measure([cfg['ver']] + cfg['variants'] + (extras if full else []) + [inc], lst, a.rounds, passes, warmup, inc)
            t['n_files'] = len(sel); t['bytes'] = sum(os.path.getsize(f) for f in sel)
            result['timing'][inc] = t
            if extras and not full and sel2:
                lst2 = write_list(f'{a.format}-corpus-{inc.replace(":", "_")}-allprofiles', sel2)
                t2 = measure([cfg['ver']] + extras, lst2, a.rounds, passes, warmup, inc)
                t2['n_files'] = len(sel2); t2['bytes'] = sum(os.path.getsize(f) for f in sel2)
                result['timing'][inc + ' [subset where gdk+imagecompat profiles also match]'] = t2
    os.makedirs(f'{HERE}/results', exist_ok=True)
    json.dump(result, open(f'{HERE}/results/{a.format}-{a.part}.json', 'w'), indent=1)

if __name__ == '__main__':
    main()
