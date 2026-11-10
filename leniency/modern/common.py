#!/usr/bin/env python3
"""Shared differential harness for the WebP, JPEG 2000 and JPEG XL adapters.

Same engine contract as format_harness.py (engine.invoke, one JSON line per input, identical bytes
through separate subprocesses) with additions needed for lossy codecs and animation:

* pixel comparison has a tolerance tier (max abs channel difference over straight RGBA8) next to the
  exact hash, and a frame-count/animation tier;
* runners belong to implementation groups; members of one group are not independent votes;
* crash/timeout inputs are saved under <out>/anomalies and only counted.

engine.py and format_harness.py are imported unchanged / not used.
"""
import argparse, collections, concurrent.futures, hashlib, json, os, random, shutil, struct, sys, time
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
import engine

FRAME_BYTES = None


# ---------------------------------------------------------------- generic mutation helpers
def u(b, p, n, endian='little'):
    return int.from_bytes(b[p:p + n], endian) if len(b) >= p + n else 0


def put(b, p, n, v, endian='little'):
    v &= (1 << (8 * n)) - 1
    return b[:p] + v.to_bytes(n, endian) + b[p + n:]


def boundary_values(old, size):
    bits = 8 * size
    mx = (1 << bits) - 1
    vals = {0, 1, 2, 3, 7, 8, 15, 16, 0x7f, 0x80, 0xff, old - 1, old + 1, old ^ 1, old * 2, old // 2, mx, mx - 1}
    if size >= 2:
        vals |= {0x100, 0x7fff, 0x8000, 0xffff, 0x101}
    if size >= 4:
        vals |= {0x10000, 0xffffff, 0x1000000, 0x7fffffff, 0x80000000, 0xfffffffe, 0xffffffff}
    if size >= 8:
        vals |= {0x7fffffffffffffff, 0x8000000000000000, 0x100000000}
    return sorted(v for v in vals if 0 <= v <= mx and v != old)


def field_mutations(b, fields, rule_prefix='field', rng=None, per_field=None):
    """fields: list of (name, offset, size, endian). Yields (bytes, prov) for boundary values."""
    for name, off, size, endian in fields:
        if off + size > len(b):
            continue
        old = int.from_bytes(b[off:off + size], endian)
        vals = boundary_values(old, size)
        if per_field and rng and len(vals) > per_field:
            vals = sorted(rng.sample(vals, per_field))
        for v in vals:
            yield put(b, off, size, v, endian), {'rule': f'{rule_prefix}:{name}', 'field': name, 'offset': off,
                                                  'old': old, 'new': v}


def generic_mutations(b, rng, cut_points=(), nflip=40, header=64):
    n = len(b)
    cuts = {0, 1, 2, 3, 4, 8, 11, 12, 16, 20, 24, 28, 32, 40, n - 1, n - 2, n - 3, n - 4, n - 8, n // 2}
    cuts |= {c for c in cut_points} | {c - 1 for c in cut_points} | {c + 1 for c in cut_points}
    cuts |= {rng.randrange(1, max(n, 2)) for _ in range(6)}
    for c in sorted(x for x in cuts if 0 <= x < n):
        yield b[:c], {'rule': 'truncation', 'field': 'length', 'old': n, 'new': c}
    for name, tail in [('zero1', b'\0'), ('zero2', b'\0\0'), ('zero3', b'\0\0\0'), ('zero4', b'\0' * 4),
                       ('ff1', b'\xff'), ('zero16', b'\0' * 16), ('junk-text', b'JUNK trailing text\n'),
                       ('zero4k', b'\0' * 4096), ('repeat-head', b[:12]), ('repeat-all', b)]:
        yield b + tail, {'rule': 'trailing-bytes', 'field': name, 'old': n, 'new': n + len(tail)}
    for _ in range(nflip):
        i = rng.randrange(min(n, header)) if rng.random() < 0.5 else rng.randrange(n)
        if rng.random() < 0.6:
            bit = rng.randrange(8)
            yield b[:i] + bytes([b[i] ^ (1 << bit)]) + b[i + 1:], {'rule': 'bit-flip', 'field': f'byte{i}', 'offset': i, 'old': b[i], 'new': b[i] ^ (1 << bit)}
        else:
            v = rng.randrange(256)
            yield b[:i] + bytes([v]) + b[i + 1:], {'rule': 'byte-replace', 'field': f'byte{i}', 'offset': i, 'old': b[i], 'new': v}


# ---------------------------------------------------------------- pixel comparison
def read_dump(path):
    try:
        d = Path(path).read_bytes()
        Path(path).unlink()
        return d
    except OSError:
        return None


def pixel_stats(a, b, tol):
    """a,b raw RGBA8 (all frames concatenated), same length. -> (max_abs_diff, mean_abs_diff, visible_max)
    visible_max ignores RGB differences where both pixels have alpha 0."""
    mx = vis = tot = 0
    for i in range(0, len(a), 4):
        pa, pb = a[i:i + 4], b[i:i + 4]
        if pa == pb:
            continue
        d = max(abs(pa[0] - pb[0]), abs(pa[1] - pb[1]), abs(pa[2] - pb[2]), abs(pa[3] - pb[3]))
        mx = max(mx, d)
        tot += abs(pa[0] - pb[0]) + abs(pa[1] - pb[1]) + abs(pa[2] - pb[2]) + abs(pa[3] - pb[3])
        if not (pa[3] == 0 and pb[3] == 0):
            vis = max(vis, d)
    return mx, tot / max(len(a), 1), vis


def pair_relation(ra, rb, da, db, tol):
    """Relation of two successful decodes: equal | transparent-rgb | within-tol | different-pixels |
    different-dimensions | different-frame-count; later-frame differences are appended with '+later'."""
    if (ra['width'], ra['height']) != (rb['width'], rb['height']):
        return 'different-dimensions', None
    if ra.get('frames', 1) != rb.get('frames', 1):
        return 'different-frame-count', None
    if ra['rgba_sha256'] == rb['rgba_sha256'] and ra.get('frames_sha256') == rb.get('frames_sha256'):
        return 'equal', (0, 0, 0)
    if da is None or db is None or len(da) != len(db):
        return 'different-pixels', None
    mx, mean, vis = pixel_stats(da, db, tol)
    if mx == 0:
        return 'equal', (0, 0, 0)
    if vis == 0:
        return 'transparent-rgb', (mx, mean, vis)
    if vis <= tol:
        return 'within-tol', (mx, mean, vis)
    return 'different-pixels', (mx, mean, vis)


def classify(results, rels):
    st = [v['status'] for v in results.values() if v['status'] != 'skip']  # 'skip' = API not applicable, no vote
    if 'crash' in st or 'timeout' in st:
        return 'crash-or-timeout'
    acc = [k for k, v in results.items() if v['status'] == 'ok']
    if not st:
        return 'all-skipped'
    if not acc:
        return 'all-reject'
    if len(acc) < len(st):
        return 'accept-vs-reject'
    rs = {r for r, _ in rels.values()}
    if rs == {'equal'} or not rs:
        return 'all-accept-identical-pixels'
    bad = rs - {'equal', 'transparent-rgb', 'within-tol'}
    if bad:
        return 'accept-different-pixels'
    if rs <= {'equal', 'transparent-rgb'}:
        return 'all-accept-transparent-rgb-differs'
    return 'all-accept-within-tolerance'


def rel_signature(results, rels):
    names = list(results)
    return (tuple(results[k]['status'] for k in names),
            tuple((a, b, rels[(a, b)][0]) for (a, b) in sorted(rels)),
            tuple((k, results[k].get('width'), results[k].get('height'), results[k].get('frames'))
                  for k in names if results[k]['status'] == 'ok'))


# ---------------------------------------------------------------- adapter run
class Adapter:
    """Fields: fmt, ext, title, runners {name:(exe,args)}, groups {name:group}, tol, seeds() -> [(bytes,meta)],
    mutations(bytes, meta, rng) -> iterable of (bytes, prov), unavailable [(what, why)], notes str,
    decoders_doc {name: description}, dir (adapter directory), findings_file."""
    tol = 2

    def invoke_all(self, path, timeout, dump_dir, tag):
        res, dumps = {}, {}
        for name, runner in self.runners.items():
            d = Path(dump_dir) / f'{tag}.{name}.rgba'
            r = engine.invoke(path, runner, timeout, d)
            res[name] = r
            if r.get('status') == 'ok':
                dumps[name] = read_dump(d)
            else:
                try:
                    d.unlink()
                except OSError:
                    pass
        return res, dumps

    def evaluate(self, path, timeout, dump_dir, tag):
        res, dumps = self.invoke_all(path, timeout, dump_dir, tag)
        names = [n for n in res if res[n]['status'] == 'ok']
        rels = {}
        for i, a in enumerate(names):
            for b in names[i + 1:]:
                rels[(a, b)] = pair_relation(res[a], res[b], dumps.get(a), dumps.get(b), self.tol)
        return res, rels

    def group_divergence(self, res, rels):
        """Members of one implementation group that disagree (status or pixels)."""
        out = []
        by = collections.defaultdict(list)
        for n, g in self.groups.items():
            by[g].append(n)
        for g, ms in by.items():
            for i, a in enumerate(ms):
                for b in ms[i + 1:]:
                    if a not in res or b not in res or 'skip' in (res[a]['status'], res[b]['status']):
                        continue
                    if res[a]['status'] != res[b]['status']:
                        out.append(f'{a}/{b}:status')
                    elif res[a]['status'] == 'ok':
                        r = rels.get((a, b)) or rels.get((b, a))
                        if r and r[0] != 'equal':
                            out.append(f'{a}/{b}:{r[0]}')
        return out


def cell(r):
    if r['status'] == 'ok':
        return f"{r['width']}×{r['height']}" + (f"×{r['frames']}f" if r.get('frames', 1) > 1 else '') + f" `{r['rgba_sha256'][:8]}`"
    if r['status'] == 'skip':
        return 'n/a'
    if r['status'] == 'error':
        return 'reject `' + (r.get('error_class') or '?')[:22].replace('|', '/') + '`'
    return r['status']


def generate(ad, limit, seed):
    rng = random.Random(seed)
    seeds = ad.seeds()
    items, seen = [], set()
    per_rule = collections.defaultdict(list)
    for data, meta in seeds:
        h = hashlib.sha256(data).digest()
        if h in seen:
            continue
        seen.add(h)
        items.append((data, {'rule': 'seed', 'seed': meta['name'], 'seed_meta': meta}))
    for data, meta in seeds:
        r2 = random.Random(f'{seed}:{meta["name"]}')
        for mb, prov in ad.mutations(data, meta, r2):
            h = hashlib.sha256(mb).digest()
            if h in seen or mb == data:
                continue
            seen.add(h)
            prov = dict(prov)
            prov['seed'] = meta['name']
            per_rule[prov['rule']].append((mb, prov))
    for v in per_rule.values():
        rng.shuffle(v)
    rules = sorted(per_rule)
    out = list(items)
    # round-robin across rules for balanced coverage
    idx = collections.defaultdict(int)
    while len(out) < limit and any(idx[r] < len(per_rule[r]) for r in rules):
        for r in rules:
            if idx[r] < len(per_rule[r]) and len(out) < limit:
                out.append(per_rule[r][idx[r]])
                idx[r] += 1
    return out[:max(limit, len(items))], len(items), {r: len(v) for r, v in per_rule.items()}


def minimize(ad, data, target, timeout, work, checks=48):
    """Byte-range delta debugger preserving the relation signature (statuses, pair relations, dimensions)."""
    path = Path(work) / ('min.' + ad.ext)
    count = [0]

    def same(b):
        if count[0] >= checks:
            return False
        count[0] += 1
        path.write_bytes(b)
        res, rels = ad.evaluate(path, timeout, work, 'min')
        return rel_signature(res, rels) == target

    cur, chunk = data, max(len(data) // 2, 1)
    while chunk >= 1 and count[0] < checks:
        i, changed = 0, False
        while i < len(cur) and count[0] < checks:
            cand = cur[:i] + cur[i + chunk:]
            if cand and same(cand):
                cur, changed = cand, True
            else:
                i += chunk
        if not changed:
            chunk //= 2
    return cur


def pattern_label(res):
    return ', '.join(f'{k}:{"OK" if v["status"]=="ok" else v["status"]}' for k, v in res.items())


def run(ad, argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument('--inputs', type=int, default=7000)
    ap.add_argument('--seed', type=int, default=731)
    ap.add_argument('--jobs', type=int, default=2)
    ap.add_argument('--timeout', type=float, default=10)
    ap.add_argument('--out', default=f'/tmp/claude-0/leniency-{ad.fmt}')
    ap.add_argument('--no-minimize', action='store_true')
    ap.add_argument('--render-only', action='store_true', help='re-render LENIENCY*.md from the existing summary*.json (after editing NOTES.md/findings*.md)')
    args = ap.parse_args(argv)
    if args.render_only:
        ad.notes = (ad.dir / 'NOTES.md').read_text() if (ad.dir / 'NOTES.md').exists() else ad.notes
        for name, pub in (('summary.json', False), ('summary-public.json', True)):
            s = json.loads((ad.dir / name).read_text())
            (ad.dir / ('LENIENCY-PUBLIC.md' if pub else 'LENIENCY.md')).write_text(render(ad, s, public=pub))
        return
    out = Path(args.out)
    for sub in ('inputs', 'dump', 'anomalies', 'witnesses', 'work'):
        shutil.rmtree(out / sub, ignore_errors=True)
        (out / sub).mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    items, nseed, rule_pool = generate(ad, args.inputs, args.seed)
    manifest = open(out / 'manifest.jsonl', 'w')
    paths = []
    for i, (data, prov) in enumerate(items):
        p = out / 'inputs' / f'{i:05d}.{ad.ext}'
        p.write_bytes(data)
        paths.append(p)
        manifest.write(json.dumps({'id': i, 'sha256': hashlib.sha256(data).hexdigest(), 'length': len(data), 'prov': prov}, sort_keys=True) + '\n')
    manifest.close()
    gen_s = time.time() - t0

    t1 = time.time()

    def work(i):
        res, rels = ad.evaluate(paths[i], args.timeout, out / 'dump', f'{i}')
        return i, res, rels

    records = [None] * len(items)
    with concurrent.futures.ThreadPoolExecutor(args.jobs) as ex:
        for n, (i, res, rels) in enumerate(ex.map(work, range(len(items)))):
            records[i] = (res, rels)
            if n % 500 == 0:
                print(f'{n}/{len(items)} {time.time()-t1:.0f}s', file=sys.stderr, flush=True)
    cmp_s = time.time() - t1

    # per-input analysis
    recs = []
    resf = open(out / 'results.jsonl', 'w')
    seed_sig = {}
    for i, (res, rels) in enumerate(records):
        prov = items[i][1]
        cls = classify(res, rels)
        sig = rel_signature(res, rels)
        if prov['rule'] == 'seed':
            seed_sig[prov['seed']] = sig
        recs.append({'id': i, 'prov': prov, 'class': cls, 'sig': sig, 'res': res, 'rels': rels,
                     'gdiv': ad.group_divergence(res, rels), 'len': len(items[i][0])})
        resf.write(json.dumps({'id': i, 'class': cls, 'prov': {k: v for k, v in prov.items() if k != 'seed_meta'},
                               'rels': {f'{a}|{b}': [r[0]] + (list(r[1]) if r[1] else []) for (a, b), r in rels.items()},
                               'res': engine.compact(res)}, sort_keys=True) + '\n')
    resf.close()
    for r in recs:
        r['changed'] = r['prov']['rule'] != 'seed' and r['sig'] != seed_sig.get(r['prov'].get('seed'))

    # anomalies: save and count only
    anom = collections.Counter()
    for r in recs:
        bad = [k for k, v in r['res'].items() if v['status'] in ('crash', 'timeout')]
        if bad:
            shutil.copy(paths[r['id']], out / 'anomalies' / f'{r["id"]:05d}.{ad.ext}')
            for k in bad:
                anom[f"{k}:{r['res'][k]['status']}"] += 1

    summary = build_summary(ad, args, recs, items, nseed, rule_pool, anom, gen_s, cmp_s, public=False)
    public_recs = [r for r in recs if r['class'] != 'crash-or-timeout']
    public = build_summary(ad, args, public_recs, items, nseed, rule_pool, collections.Counter(), gen_s, cmp_s, public=True)
    if not args.no_minimize:
        t2 = time.time()
        for s, subset in ((summary, recs), (public, public_recs)):
            witnesses(ad, s, subset, items, args, out)
        print(f'minimisation {time.time()-t2:.0f}s', file=sys.stderr)
    summary['timing']['total_seconds'] = round(time.time() - t0, 1)
    public['timing']['total_seconds'] = summary['timing']['total_seconds']
    d = ad.dir
    (d / 'summary.json').write_text(json.dumps(summary, indent=1, sort_keys=True, default=list))
    (d / 'summary-public.json').write_text(json.dumps(public, indent=1, sort_keys=True, default=list))
    (d / 'LENIENCY.md').write_text(render(ad, summary, public=False))
    (d / 'LENIENCY-PUBLIC.md').write_text(render(ad, public, public=True))
    print(json.dumps({'inputs': len(items), 'classes': summary['classes'], 'anomalies': dict(anom),
                      'seconds': summary['timing']}, default=list))


def build_summary(ad, args, recs, items, nseed, rule_pool, anom, gen_s, cmp_s, public):
    names = list(ad.runners)
    classes = collections.Counter(r['class'] for r in recs)
    per_dec = {n: collections.Counter(r['res'][n]['status'] for r in recs) for n in names}
    # acceptance disagreement matrix
    dis = {a: {b: sum(1 for r in recs if (r['res'][a]['status'] == 'ok') != (r['res'][b]['status'] == 'ok')
                      and 'crash' not in (r['res'][a]['status'], r['res'][b]['status']) and 'skip' not in (r['res'][a]['status'], r['res'][b]['status'])
                      and 'timeout' not in (r['res'][a]['status'], r['res'][b]['status'])) for b in names} for a in names}
    # lone acceptor / lone rejecter counted per implementation group
    groups = collections.defaultdict(list)
    for n, g in ad.groups.items():
        groups[g].append(n)
    lone_acc, lone_rej = collections.Counter(), collections.Counter()
    gv = collections.Counter()
    for r in recs:
        if r['class'] == 'crash-or-timeout':
            continue
        gst = {g: ('ok' if any(r['res'][m]['status'] == 'ok' for m in ms) else 'error') for g, ms in groups.items()
               if any(r['res'][m]['status'] != 'skip' for m in ms)}
        oks = [g for g, s in gst.items() if s == 'ok']
        if len(gst) >= 3:
            if len(oks) == 1:
                lone_acc[oks[0]] += 1
            if len(oks) == len(gst) - 1:
                lone_rej[[g for g, s in gst.items() if s != 'ok'][0]] += 1
        for x in r['gdiv']:
            gv[x] += 1
    # clustering by provenance
    rules = collections.defaultdict(list)
    for r in recs:
        if r['changed']:
            rules[r['prov']['rule']].append(r)
    rows = []
    for rule, rs in rules.items():
        sigs = collections.Counter(r['sig'] for r in rs)
        top, cnt = sigs.most_common(1)[0]
        rep = min((r for r in rs if r['sig'] == top), key=lambda r: (r['len'], r['id']))
        rows.append({'rule': rule, 'changed': len(rs), 'patterns': len(sigs), 'top_pattern_count': cnt,
                     'classes': dict(collections.Counter(r['class'] for r in rs)), 'witness_id': rep['id'],
                     'cells': {n: cell(rep['res'][n]) for n in names}, 'prov': {k: v for k, v in rep['prov'].items() if k != 'seed_meta'},
                     'class': rep['class']})
    rows.sort(key=lambda x: -x['changed'])
    # leniency partitions: which decoders accept vs reject
    parts = collections.defaultdict(list)
    for r in recs:
        if r['class'] == 'accept-vs-reject':
            acc = tuple(n for n in names if r['res'][n]['status'] == 'ok')
            parts[(acc, tuple(n for n in names if r['res'][n]['status'] == 'skip'))].append(r)
    prow = []
    for (acc, skp), rs in parts.items():
        rr = collections.Counter(r['prov']['rule'] for r in rs)
        rep = min(rs, key=lambda r: (r['len'], r['id']))
        prow.append({'accepting': list(acc), 'rejecting': [n for n in names if n not in acc and n not in skp], 'count': len(rs),
                     'top_rules': rr.most_common(4), 'witness_id': rep['id'],
                     'cells': {n: cell(rep['res'][n]) for n in names},
                     'unchanged_seed': sum(1 for r in rs if r['prov']['rule'] == 'seed')})
    prow.sort(key=lambda x: -x['count'])
    # pixel rows
    pix = collections.defaultdict(list)
    for r in recs:
        if r['class'] in ('accept-different-pixels', 'all-accept-transparent-rgb-differs', 'all-accept-within-tolerance'):
            key = (r['class'], tuple(sorted({(a, b, v[0]) for (a, b), v in r['rels'].items() if v[0] != 'equal'})))
            pix[key].append(r)
    xrow = []
    for (cls, diffs), rs in pix.items():
        rep = min(rs, key=lambda r: (r['len'], r['id']))
        mx = max((v[1][0] for v in rep['rels'].values() if v[1]), default=None)
        xrow.append({'class': cls, 'differing_pairs': [list(d) for d in diffs], 'count': len(rs),
                     'rules': collections.Counter(r['prov']['rule'] for r in rs).most_common(4),
                     'witness_id': rep['id'], 'max_abs_diff': mx, 'cells': {n: cell(rep['res'][n]) for n in names}})
    xrow.sort(key=lambda x: (-x['count']))
    tol_hist = collections.Counter()
    for r in recs:
        for v in r['rels'].values():
            if v[1] and v[0] in ('within-tol', 'different-pixels'):
                tol_hist[min(v[1][2], 64)] += 1
    seeds_only = [r for r in recs if r['prov']['rule'] == 'seed']
    return {
        'format': ad.fmt, 'public': public, 'seed': args.seed, 'tolerance_max_abs_channel_diff': ad.tol,
        'inputs': len(recs), 'seed_inputs': len(seeds_only), 'distinct_seeds': nseed, 'rule_pool': rule_pool,
        'changed_relations': sum(1 for r in recs if r['changed']),
        'decoders': {n: {'group': ad.groups[n], 'status_counts': dict(per_dec[n]), 'doc': ad.decoders_doc.get(n, '')} for n in names},
        'unavailable': ad.unavailable, 'classes': dict(classes), 'acceptance_disagreements': dis,
        'lone_acceptor_by_group': dict(lone_acc), 'lone_rejecter_by_group': dict(lone_rej),
        'same_group_divergence': dict(gv),
        'anomaly_counts': dict(anom), 'rules': rows, 'partitions': prow, 'pixel_rows': xrow,
        'visible_diff_histogram_pairs': dict(sorted(tol_hist.items())),
        'seed_outcomes': [{'seed': r['prov']['seed'], 'class': r['class'], 'cells': {n: cell(r['res'][n]) for n in names}} for r in seeds_only],
        'timing': {'generation_seconds': round(gen_s, 1), 'comparison_seconds': round(cmp_s, 1)},
        'witnesses': {},
    }


def witnesses(ad, s, recs, items, args, out):
    byid = {r['id']: r for r in recs}
    tag = 'public' if s['public'] else 'full'
    wd = out / 'witnesses' / tag
    wd.mkdir(parents=True, exist_ok=True)
    want = []
    for rows, kind in ((s['partitions'][:14], 'lenient'), (s['rules'][:14], 'rule'), (s['pixel_rows'][:8], 'pixel')):
        for k, row in enumerate(rows):
            want.append((kind, k, row['witness_id'], row))
    for kind, k, wid, row in want:
        r = byid[wid]
        data = items[wid][0]
        small = minimize(ad, data, r['sig'], args.timeout, out / 'work', checks=40)
        name = f'{kind}-{k+1:02d}-{wid}.{ad.ext}'
        (wd / name).write_bytes(small)
        res, rels = ad.evaluate(wd / name, args.timeout, out / 'work', 'w')
        row['witness'] = {'file': name, 'original_length': len(data), 'minimised_length': len(small),
                          'cells_minimised': {n: cell(v) for n, v in res.items()},
                          'same_signature_as_original': rel_signature(res, rels) == r['sig']}
    if not s['public']:
        pass
    # public witnesses are copied into the adapter directory (synthetic inputs only, no crash rows)
    if s['public']:
        dest = ad.dir / 'witnesses'
        shutil.rmtree(dest, ignore_errors=True)
        dest.mkdir()
        for f in wd.iterdir():
            shutil.copy(f, dest / f.name)


def render(ad, s, public):
    names = list(ad.runners)
    L = []
    w = L.append
    base = 'witnesses/' if public else f'/tmp/claude-0/leniency-{ad.fmt}/witnesses/full/'
    w(f"# {ad.title} leniency matrix{' (public: synthetic inputs, crash rows excluded)' if public else ''}\n")
    t = s['timing']
    w(f"Seed {s['seed']}; {s['inputs']} inputs ({s['seed_inputs']} unmutated synthetic seeds from {s['distinct_seeds']} distinct seed files, the rest mutations); "
      f"{s['changed_relations']} mutations changed the acceptance or cross-decoder pixel relation relative to their seed; "
      f"{t['comparison_seconds']} s comparison pass, {t['total_seconds']} s total including witness minimisation.\n")
    w('All inputs are synthetic (produced by the seed tool in this directory from closed-form pixel patterns, then mutated); no third-party file is involved.'
      + (' Inputs on which any decoder crashed or timed out are excluded from this public summary.' if public else '') + '\n')
    w('## Decoders\n')
    w('Column | Implementation group (one vote per group) | Status counts | What it is')
    w('--- | --- | --- | ---')
    for n in names:
        d = s['decoders'][n]
        w(f"{n} | {d['group']} | " + ', '.join(f'{k} {v}' for k, v in sorted(d['status_counts'].items())) + f" | {d['doc']}")
    if s['unavailable']:
        w('\nUnavailable (not counted as rejecting):\n')
        for a, b in s['unavailable']:
            w(f'- **{a}**: {b}')
    w(f"\nPixel comparison: exact RGBA8 SHA-256 first; otherwise the maximum absolute channel difference over all frames (alpha included) "
      f"decides: at most {s['tolerance_max_abs_channel_diff']} counts as within tolerance (documented per format), larger is a pixel disagreement. "
      "Differences confined to RGB under alpha 0 on both sides are reported separately (`transparent-rgb`). Dimension and frame-count differences are never tolerated.\n")
    w('## Classification\n')
    w('Classification | Count\n--- | ---:')
    for k, v in sorted(s['classes'].items(), key=lambda x: -x[1]):
        w(f'{k} | {v}')
    if not public:
        w('\nCrash/timeout counts per decoder (inputs saved under `/tmp/claude-0/leniency-' + ad.fmt + '/anomalies/`, not investigated): '
          + (', '.join(f'{k} {v}' for k, v in sorted(s['anomaly_counts'].items())) or 'none') + '\n')
    w('\n## Seeds (unmutated)\n')
    w('Seed | Class | ' + ' | '.join(names))
    w('--- | --- | ' + ' | '.join('---' for _ in names))
    for x in s['seed_outcomes']:
        w(f"{x['seed']} | {x['class']} | " + ' | '.join(x['cells'][n] for n in names))
    w('\n## Who accepts what (accept-vs-reject partitions)\n')
    w('Accepting decoders | Rejecting | Count | Top rules | Witness | ' + ' | '.join(names))
    w('--- | --- | ---: | --- | --- | ' + ' | '.join('---' for _ in names))
    for p in s['partitions'][:25]:
        wit = p.get('witness', {}).get('file')
        w(f"{', '.join(p['accepting']) or '-'} | {', '.join(p['rejecting']) or '-'} | {p['count']} | "
          + '; '.join(f'{r} {c}' for r, c in p['top_rules']) + f" | {('[' + wit + '](' + base + wit + ')') if wit else '#' + str(p['witness_id'])} | "
          + ' | '.join(p['cells'][n] for n in names))
    w('\n## Provenance clusters (mutation rule; changed-relation counts)\n')
    w('A mutation joins a cluster only if it changed acceptance or the cross-decoder pixel relation relative to its unmutated seed. The row shows the most frequent outcome pattern.\n')
    w('Rule (changed; patterns) | Witness | ' + ' | '.join(names))
    w('--- | --- | ' + ' | '.join('---' for _ in names))
    for r in s['rules'][:40]:
        wit = r.get('witness', {}).get('file')
        w(f"{r['rule']} ({r['changed']}; {r['patterns']}) | {('[' + wit + '](' + base + wit + ')') if wit else '#' + str(r['witness_id'])} | "
          + ' | '.join(r['cells'][n] for n in names))
    w('\n## Same acceptance, different pixels or animation\n')
    w('Class | Differing decoder pairs | Count | Max abs diff (witness) | Top rules | Witness | ' + ' | '.join(names))
    w('--- | --- | ---: | --- | --- | --- | ' + ' | '.join('---' for _ in names))
    for x in s['pixel_rows'][:20]:
        wit = x.get('witness', {}).get('file')
        pairs = '; '.join(f'{a}~{b}: {c}' for a, b, c in x['differing_pairs'][:4])
        w(f"{x['class']} | {pairs} | {x['count']} | {x['max_abs_diff']} | " + '; '.join(f'{r} {c}' for r, c in x['rules'][:3])
          + f" | {('[' + wit + '](' + base + wit + ')') if wit else '#' + str(x['witness_id'])} | " + ' | '.join(x['cells'][n] for n in names))
    w('\n## Per-group lone votes and same-group divergence\n')
    w(f"Lone acceptor (only this implementation group accepts): {s['lone_acceptor_by_group'] or 'none'}\n")
    w(f"Lone rejecter (only this group rejects): {s['lone_rejecter_by_group'] or 'none'}\n")
    w(f"Same-group divergence (columns that share an implementation but disagree; indicates glue or API-path differences): {s['same_group_divergence'] or 'none'}\n")
    w('\n## Acceptance disagreement counts (inputs where the two columns differ in accept/reject)\n')
    w('| | ' + ' | '.join(names))
    w('--- | ' + ' | '.join('---' for _ in names))
    for a in names:
        w(f'{a} | ' + ' | '.join(str(s['acceptance_disagreements'][a][b]) for b in names))
    ff = ad.dir / ('findings-public.md' if public else 'findings.md')
    if ff.exists():
        w('\n' + ff.read_text())
    w('\n## Notes\n')
    w(ad.notes)
    return '\n'.join(L) + '\n'
