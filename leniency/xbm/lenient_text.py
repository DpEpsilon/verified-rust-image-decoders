"""Differential-leniency framework for C-source-like text image formats (XBM, XPM).

Used by xbm/harness.py and xpm/harness.py. Builds on engine.invoke/classify/relation_signature and leaves
engine.py, format_harness.py and other adapters untouched. Everything generated is synthetic; all inputs,
per-input results and anomaly inputs live under /tmp/claude-0/leniency-<fmt>/.

A format config (dict) supplies: name, ext, runners {decoder: (exe, args)}, unavailable {decoder: reason},
independent {decoder: group} (decoders in one group share code and are not independent votes),
generate(limit, seed) -> [(bytes, meta)], and optional notes (markdown appended to LENIENCY.md).
"""
import collections, concurrent.futures, hashlib, json, re, shutil, sys, time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
import engine  # noqa: E402

KEEP = ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'alpha_present', 'orig_color_type', 'hotspot')
PASS_THROUGH = ('all-reject', 'all-accept-identical-pixels')


def compact(results):
    return {k: {x: v[x] for x in KEEP if x in v} for k, v in results.items()}


def sig(results):
    return tuple((k, v['status'], v.get('width'), v.get('height'), v.get('rgba_sha256')) for k, v in results.items())


def is_panic(v):
    return v.get('error_class') == 'rust_panic'


def folder_of(cfg):
    return Path('/tmp/claude-0/leniency-' + cfg['name'])


def invoke(cfg, path, name, timeout=2.0, dump=None):
    return engine.invoke(path, cfg['runners'][name], timeout, dump)


def minimize(cfg, data, target_sig, names, timeout, tmpdir, budget, call=None):
    """Delete whole lines first, then byte ranges, while every named runner keeps status/dimensions/hash."""
    call = call or (lambda path, n: invoke(cfg, path, n, timeout))
    tmp = tmpdir / ('minimize.' + cfg['ext']); used = 0

    def same(candidate):
        nonlocal used
        tmp.write_bytes(candidate); used += 1
        return sig({n: call(tmp, n) for n in names}) == target_sig
    lines = data.splitlines(keepends=True); i = 0
    while i < len(lines) and used < budget and len(lines) > 1:
        cand = lines[:i] + lines[i + 1:]
        if same(b''.join(cand)): lines = cand
        else: i += 1
    data = b''.join(lines); n = 2
    while len(data) > 1 and used < budget:
        step = max(1, (len(data) + n - 1) // n); changed = False
        for start in range(0, len(data), step):
            cand = data[:start] + data[start + step:]
            if cand and same(cand): data = cand; n = max(2, n - 1); changed = True; break
            if used >= budget: break
        if not changed:
            if n >= len(data): break
            n = min(len(data), n * 2)
    tmp.unlink(missing_ok=True)
    return data, used


def pixel_difference(cfg, path, results, tmpdir, timeout=2.0):
    good = [k for k, v in results.items() if v['status'] == 'ok']
    if len({(results[k]['width'], results[k]['height']) for k in good}) > 1: return 'dimensions'
    dumps = {}
    for k in good:
        t = tmpdir / f'px-{k}.rgba'; v = invoke(cfg, path, k, timeout, t)
        if v['status'] != 'ok' or not t.exists(): return 'rgba-dump-unavailable'
        dumps[k] = t.read_bytes(); t.unlink()
    ref = good[0]; out = []
    for k in good[1:]:
        a, b = dumps[ref], dumps[k]
        if a == b or len(a) != len(b): continue
        px = [i for i in range(0, len(a), 4) if a[i:i + 4] != b[i:i + 4]]
        alpha_only = all(a[i:i + 3] == b[i:i + 3] for i in px)
        out.append(f'{k} vs {ref}: {len(px)}/{len(a) // 4} px' + (' alpha-only' if alpha_only else ''))
    return '; '.join(out[:3])


def classify_all(results):
    return engine.classify(results)


def build_stats(cfg, rows_iter):
    names = list(cfg['runners']); st = {n: collections.Counter() for n in names}
    errs = {n: collections.Counter() for n in names}
    pair_acc = collections.Counter(); pair_pix = collections.Counter(); n_pairs = collections.Counter()
    lenient = collections.Counter(); strict = collections.Counter()
    for r in rows_iter:
        res = r['results']
        for n in names:
            v = res[n]
            if is_panic(v): st[n]['panic-reject'] += 1
            st[n][v['status']] += 1
            if v['status'] == 'error': errs[n][v.get('error_class', '?')[:40]] += 1
        sound = [n for n in names if res[n]['status'] in ('ok', 'error')]
        gs = group_status(cfg, res) if 'independent' in cfg else {}
        goks = [g for g, v in gs.items() if v == 'ok']; gers = [g for g, v in gs.items() if v == 'error']
        if len(gs) >= 3 and not has_crash(res):
            if len(goks) == 1 and len(gers) >= 2: lenient[goks[0]] += 1
            if len(gers) == 1 and len(goks) >= 2: strict[gers[0]] += 1
        for i, a in enumerate(names):
            for b in names[i + 1:]:
                if a in sound and b in sound:
                    n_pairs[(a, b)] += 1
                    if res[a]['status'] != res[b]['status']: pair_acc[(a, b)] += 1
                    elif res[a]['status'] == 'ok' and (res[a]['width'], res[a]['height'], res[a]['rgba_sha256']) != (res[b]['width'], res[b]['height'], res[b]['rgba_sha256']):
                        pair_pix[(a, b)] += 1
    pk = lambda c: {f'{a}|{b}': v for (a, b), v in sorted(c.items())}
    return {'per_decoder': {n: dict(st[n]) for n in names}, 'top_error_classes': {n: errs[n].most_common(4) for n in names},
            'group_only_accepts': dict(lenient), 'group_only_rejects': dict(strict),
            'pair_accept_disagreements': pk(pair_acc), 'pair_pixel_disagreements': pk(pair_pix), 'pair_comparable': pk(n_pairs)}


def cluster_fn(items, limit=40):
    vals = collections.defaultdict(set)
    for _, m in items:
        if m['source'] == 'mutation': vals[m['rule']].add(str(m.get('value')))
    def cluster(m):
        if m['source'] == 'mutation' and len(vals[m['rule']]) <= limit: return f"{m['rule']}={str(m.get('value'))[:40]}"
        return m['rule']
    return cluster


def group_status(cfg, res):
    g = collections.defaultdict(list)
    for n, grp in cfg['independent'].items():
        if res[n]['status'] in ('ok', 'error'): g[grp].append(res[n]['status'] == 'ok')
    return {grp: ('ok' if sum(v) * 2 > len(v) else 'error') for grp, v in g.items()}


def pixel_partition(cfg, res):
    oks = [n for n in res if res[n]['status'] == 'ok']
    classes = collections.defaultdict(set)
    for n in oks: classes[(res[n]['width'], res[n]['height'], res[n]['rgba_sha256'])].add(cfg['independent'][n])
    if len(classes) < 2: return None
    return ' || '.join(sorted('+'.join(sorted(v)) for v in classes.values()))


def run(cfg, args):
    fmt = cfg['name']; ext = cfg['ext']; folder = folder_of(cfg); inputs = folder / 'inputs'
    shutil.rmtree(inputs, ignore_errors=True); inputs.mkdir(parents=True, exist_ok=True)
    adapter = ROOT / fmt; names = list(cfg['runners'])
    items, n_seeds = cfg['generate'](args.inputs, args.seed)
    manifest = folder / 'manifest.jsonl'; resfile = folder / 'results.jsonl'
    with manifest.open('w') as f:
        for i, (b, m) in enumerate(items):
            (inputs / f'{i:05}.{ext}').write_bytes(b)
            f.write(json.dumps({'id': i, 'length': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **m}) + '\n')
    cluster = cluster_fn(items)
    started = time.monotonic(); counts = collections.Counter(); baselines = {}; per_rule = collections.defaultdict(list)
    changed_total = 0; anomalies = []

    def job(i):
        p = inputs / f'{i:05}.{ext}'
        return i, {n: invoke(cfg, p, n, args.timeout) for n in names}
    with resfile.open('w') as out, concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for i, res in pool.map(job, range(len(items))):
            m = items[i][1]; cls = classify_all(res); counts[cls] += 1
            rel = engine.relation_signature(res)
            if m['source'] == 'seed': baselines[m['name']] = rel
            changed = m['source'] == 'mutation' and rel != baselines.get(m['seed'])
            changed_total += changed
            if cls not in PASS_THROUGH and (m['source'] == 'seed' or changed): per_rule[cluster(m)].append(i)
            if cls == 'crash-or-timeout': anomalies.append(i)
            out.write(json.dumps({'id': i, 'rule': m['rule'], 'seed': m.get('seed') or m['name'], 'source': m['source'], 'cls': cls,
                                  'changed': bool(changed), 'cluster': cluster(m), 'results': compact(res)}, separators=(',', ':')) + '\n')
            if (i + 1) % 1000 == 0: print(f'{fmt} {i + 1}/{len(items)} {time.monotonic() - started:.0f}s', flush=True)
    elapsed = round(time.monotonic() - started, 1)
    allrows = [json.loads(l) for l in resfile.open()]
    byid = {r['id']: r for r in allrows}
    wdir = adapter / 'witnesses'; shutil.rmtree(wdir, ignore_errors=True); wdir.mkdir()
    adir = folder / 'anomalies'; shutil.rmtree(adir, ignore_errors=True); adir.mkdir()
    for i in anomalies: shutil.copyfile(inputs / f'{i:05}.{ext}', adir / f'{i:05}.{ext}')
    mf = {r['id']: r for r in map(json.loads, manifest.open())}
    tmpdir = folder; wcount = [0]

    def witness(i, label):
        res = byid[i]['results']; data = (inputs / f'{i:05}.{ext}').read_bytes()
        shrunk, used = minimize(cfg, data, sig(res), names, args.timeout, tmpdir, args.min_checks)
        wcount[0] += 1; fn = f'{wcount[0]:02}-' + re.sub(r'[^A-Za-z0-9_.-]+', '-', label)[:48] + '.' + ext
        (wdir / fn).write_bytes(shrunk)
        return 'witnesses/' + fn, len(data), len(shrunk)
    # --- rule rows: most frequent outcome pattern among changed/seed inputs per rule
    rows = []
    for rule, ids in sorted(per_rule.items(), key=lambda kv: -len(kv[1]))[:args.max_rows]:
        pats = collections.defaultdict(list)
        for i in ids: pats[sig(byid[i]['results'])].append(i)
        main = max(pats.values(), key=len); i = main[0]; res = byid[i]['results']
        if any(v['status'] in ('crash', 'timeout') for v in res.values()):  # keep crash rows out of the leniency table
            clean = [p for p in pats.values() if not any(v['status'] in ('crash', 'timeout') for v in byid[p[0]]['results'].values())]
            if not clean: continue
            main = max(clean, key=len); i = main[0]; res = byid[i]['results']
        w, lo, ls = witness(i, rule)
        cls = byid[i]['cls']
        pd = pixel_difference(cfg, wdir / Path(w).name, res, folder) if cls == 'accept-different-pixels' else ''
        rows.append({'rule': rule, 'count': len(ids), 'patterns': len(pats), 'id': i, 'witness': w, 'classification': cls,
                     'pixel_difference': pd, 'meta': {k: v for k, v in mf[i].items() if k not in ('sha256',)}, 'results': res,
                     'original_length': lo, 'witness_length': ls})
    # --- outlier rows: an independence group that alone accepts (or alone rejects) while at least two others agree
    outl = collections.defaultdict(list); pix = collections.defaultdict(list); intra = collections.defaultdict(list)
    for r in allrows:
        res = r['results']
        if r['cls'] == 'crash-or-timeout': continue
        gs = group_status(cfg, res); oks = [g for g, v in gs.items() if v == 'ok']; ers = [g for g, v in gs.items() if v == 'error']
        c = r.get('cluster', r['rule'])
        if len(gs) >= 3 and len(oks) == 1 and len(ers) >= 2: outl[(oks[0], 'only-accepts', c)].append(r['id'])
        if len(gs) >= 3 and len(ers) == 1 and len(oks) >= 2: outl[(ers[0], 'only-rejects', c)].append(r['id'])
        part = pixel_partition(cfg, res)
        if part: pix[part].append(r['id'])
        for grp in set(cfg['independent'].values()):
            mem = [n for n in res if cfg['independent'][n] == grp]
            if len(mem) > 1 and len({res[n]['status'] for n in mem}) > 1: intra[(grp, c)].append(r['id'])
    picked = collections.Counter(); outliers = []
    for (d, kind, c), ids in sorted(outl.items(), key=lambda kv: -len(kv[1])):
        if picked[(d, kind)] >= args.outlier_rows: continue
        picked[(d, kind)] += 1
        w, lo, ls = witness(ids[0], f'{d}-{kind}-{c}')
        outliers.append({'decoder': d, 'kind': kind, 'rule': c, 'count': len(ids), 'id': ids[0], 'witness': w,
                         'witness_length': ls, 'results': byid[ids[0]]['results']})
    pixel_rows = []
    for part, ids in sorted(pix.items(), key=lambda kv: -len(kv[1]))[:12]:
        rc = collections.Counter(byid[i]['cluster'] for i in ids).most_common(3); i = ids[0]
        w, lo, ls = witness(i, 'pixels-' + part)
        pixel_rows.append({'partition': part, 'count': len(ids), 'top_rules': rc, 'id': i, 'witness': w, 'results': byid[i]['results'],
                           'pixel_difference': pixel_difference(cfg, wdir / Path(w).name, byid[i]['results'], folder)})
    intra_rows = []; ipicked = collections.Counter()
    for (grp, c), ids in sorted(intra.items(), key=lambda kv: -len(kv[1])):
        if ipicked[grp] >= 3: continue
        ipicked[grp] += 1; w, lo, ls = witness(ids[0], f'intra-{grp}-{c}')
        intra_rows.append({'group': grp, 'rule': c, 'count': len(ids), 'id': ids[0], 'witness': w, 'results': byid[ids[0]]['results']})
    npix = sum(len(v) for v in pix.values()); nintra = collections.Counter()
    for (grp, c), ids in intra.items(): nintra[grp] += len(ids)
    stats = build_stats(cfg, (r for r in allrows))
    summary = {'format': fmt, 'seed': args.seed, 'inputs': len(items), 'seed_inputs': sum(m['source'] == 'seed' for _, m in items),
               'distinct_mutation_seeds': n_seeds, 'counts': dict(counts), 'changed_mutations': changed_total,
               'elapsed_seconds': elapsed, 'decoders': names, 'unavailable': cfg['unavailable'], 'independence_groups': cfg['independent'],
               'anomaly_count': len(anomalies), 'anomaly_by_decoder': dict(collections.Counter(
                   n for r in allrows for n, v in r['results'].items() if v['status'] in ('crash', 'timeout'))),
               'stats': stats, 'rows': rows, 'outlier_rows': outliers, 'pixel_rows': pixel_rows, 'intra_group_rows': intra_rows,
               'inputs_with_pixel_disagreement_among_acceptors': npix, 'intra_group_status_disagreements': dict(nintra), 'tool': 'see README in xbm/ and xpm/'}
    (adapter / 'summary.json').write_text(json.dumps(summary, indent=1) + '\n')
    render(cfg, adapter, summary, 'LENIENCY.md')
    pub = public_summary(cfg, summary, allrows, mf)
    (adapter / 'summary-public.json').write_text(json.dumps(pub, indent=1) + '\n')
    render(cfg, adapter, pub, 'LENIENCY-PUBLIC.md')
    write_sync(cfg, adapter, wdir, rows, outliers, pub)
    print(json.dumps({'format': fmt, 'inputs': len(items), 'counts': counts, 'elapsed_seconds': elapsed, 'rows': len(rows),
                      'outlier_rows': len(outliers), 'anomalies': len(anomalies)}, indent=1))


def has_crash(res):
    return any(v['status'] in ('crash', 'timeout') for v in res.values())


def public_summary(cfg, s, allrows, mf):
    """Same synthetic inputs, but rows involving a crash/timeout are excluded and no anomaly details appear."""
    good = [r for r in allrows if not has_crash(r['results'])]
    p = {k: v for k, v in s.items() if k not in ('anomaly_by_decoder',)}
    p['public'] = True; p['inputs'] = len(good); p['counts'] = dict(collections.Counter(r['cls'] for r in good))
    p['changed_mutations'] = sum(r['changed'] for r in good); p['seed_inputs'] = sum(r['source'] == 'seed' for r in good)
    p['rows'] = [r for r in s['rows'] if not has_crash(r['results'])]
    p['outlier_rows'] = [r for r in s['outlier_rows'] if not has_crash(r['results'])]
    p['pixel_rows'] = [r for r in s['pixel_rows'] if not has_crash(r['results'])]; p['intra_group_rows'] = [r for r in s['intra_group_rows'] if not has_crash(r['results'])]
    p['inputs_with_pixel_disagreement_among_acceptors'] = sum(1 for r in good if pixel_partition(cfg, r['results']))
    p['stats'] = build_stats(cfg, iter(good)); p['anomaly_count'] = 'excluded'
    return p


def cell(v):
    st = v['status']
    if st == 'ok': return f"{v['width']}x{v['height']}{'a' if v.get('alpha_present') else ''} `{v['rgba_sha256'][:6]}`"
    if st == 'error': return 'rej `' + v.get('error_class', '?').replace('|', '/')[:20] + '`'
    return st.upper()


def render(cfg, adapter, s, filename):
    names = s['decoders']; fmt = s['format']; pub = s.get('public'); folder = f'/tmp/claude-0/leniency-{fmt}'
    L = [f'# {fmt.upper()} leniency matrix' + (' (public subset)' if pub else ''), '',
         f"Seed {s['seed']}; {s['inputs']} synthetic inputs ({s['seed_inputs']} valid/unmutated seeds from {s['distinct_mutation_seeds']} seed files; "
         f"{s['changed_mutations']} mutations changed the acceptance/pixel-equality pattern relative to their seed); "
         f"{s['elapsed_seconds']} s for the comparison pass. "
         + ('Inputs whose run involved a crash or timeout are excluded from every count and row below.' if pub else
            'Crash/timeout inputs are counted only; they are saved under `' + folder + '/anomalies/` and not analysed.'), '',
         'Classification | Count', '--- | ---:']
    for c in ('all-accept-identical-pixels', 'accept-different-pixels', 'accept-vs-reject', 'all-reject', 'crash-or-timeout'):
        L.append(f"{c} | {s['counts'].get(c, 0)}")
    L += ['', '## Decoders', '', '| Decoder | ok | error | of which caught Rust panic | crash | timeout | top error classes |',
          '| --- | ---: | ---: | ---: | ---: | ---: | --- |']
    st = s['stats']
    for n in names:
        d = st['per_decoder'][n]
        L.append(f"| {n} | {d.get('ok', 0)} | {d.get('error', 0)} | {d.get('panic-reject', 0)} | {d.get('crash', 0) if not pub else '-'} | {d.get('timeout', 0) if not pub else '-'} | "
                 + ', '.join(f'{k} ({v})' for k, v in st['top_error_classes'][n][:3]).replace('|', '/') + ' |')
    for n, why in s['unavailable'].items(): L.append(f'| {n} | unavailable: {why} | | | | | |')
    grp = collections.defaultdict(list)
    for n, g in s['independence_groups'].items(): grp[g].append(n)
    L += ['', 'A caught Rust panic is reported as a rejection (`rust_panic`), not as a crash. Independence groups (decoders in one group share code and are not independent votes): ' + '; '.join(
        '{' + ', '.join(v) + '}' for v in grp.values()) + '.', '',
        '### Votes per independence group', '',
        'A group votes accept when a strict majority of its members accept. "Alone accepts" = this group accepts while at least two other groups reject (needs 3 or more groups).', '',
        '| Group | alone accepts | alone rejects | intra-group accept/reject disagreements |', '| --- | ---: | ---: | ---: |']
    for g in grp:
        L.append(f"| {g} | {st['group_only_accepts'].get(g, 0)} | {st['group_only_rejects'].get(g, 0)} | {s.get('intra_group_status_disagreements', {}).get(g, 0)} |")
    L += ['', '## Pairwise disagreement (inputs where both ran without crash)', '', '| Pair | comparable | accept/reject differs | both accept, pixels differ |', '| --- | ---: | ---: | ---: |']
    for k, v in st['pair_comparable'].items():
        L.append(f"| {k.replace('|', ' vs ')} | {v} | {st['pair_accept_disagreements'].get(k, 0)} | {st['pair_pixel_disagreements'].get(k, 0)} |")
    L += ['', f"## Pixel disagreement among decoders that accept ({s.get('inputs_with_pixel_disagreement_among_acceptors', 0)} inputs)", '',
          'Same bytes, two or more decoders accept, RGBA8 (or dimensions) differ. The partition shows which groups produced identical output (`||` separates output classes). These inputs are often classed `accept-vs-reject` overall because a third decoder rejects.', '',
          '| Partition | inputs | top clusters | witness | pixel difference | ' + ' | '.join(names) + ' |', '| --- | ---: | --- | --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s.get('pixel_rows', []):
        L.append(f"| {r['partition']} | {r['count']} | " + ', '.join(f'{a} ({b})' for a, b in r['top_rules']) + f" | [{r['id']}]({r['witness']}) | {r['pixel_difference']} | " + ' | '.join(cell(r['results'][n]) for n in names) + ' |')
    L += ['', '## Intra-group disagreement', '', 'Members of one library family given the same bytes by different entry points (GDK: filename-sniffed file vs forced module, whole vs byte-wise writes; glycin vs direct image-extras).', '',
          '| Group | Cluster (inputs) | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s.get('intra_group_rows', []):
        L.append(f"| {r['group']} | {r['rule']} ({r['count']}) | [{r['id']}]({r['witness']}) | " + ' | '.join(cell(r['results'][n]) for n in names) + ' |')
    L += ['', '## Representative matrix (most frequent outcome per mutation rule)', '',
          'Cluster = mutation rule, split by mutated value when the rule has at most 40 distinct values. Cell: `WxH` (a = some alpha<255) and the first six hex digits of the RGBA8 SHA-256, or `rej` + error class. RGB under alpha=0 is zeroed before hashing.', '',
          '| Rule (inputs; patterns) | Witness | class | ' + ' | '.join(names) + ' |', '| --- | --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['rows']:
        d = '; ' + r['pixel_difference'] if r.get('pixel_difference') else ''
        L.append(f"| {r['rule']} ({r['count']}; {r['patterns']}{d}) | [{r['id']}]({r['witness']}) | {r['classification']} | " + ' | '.join(cell(r['results'][n]) for n in names) + ' |')
    L += ['', '## Single-decoder outliers', '',
          'An independence group that alone accepts (or alone rejects) while at least two other groups disagree with it, grouped by cluster; witnesses are minimised preserving every decoder outcome.', '',
          '| Group | Kind | Cluster (inputs) | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['outlier_rows']:
        L.append(f"| {r['decoder']} | {r['kind']} | {r['rule']} ({r['count']}) | [{r['id']}]({r['witness']}) | " + ' | '.join(cell(r['results'][n]) for n in names) + ' |')
    L += ['', '## Ten most frequent outlier behaviours', '']
    seen = collections.Counter(); picks = []
    for r in sorted(s['outlier_rows'], key=lambda r: -r['count']):
        if seen[(r['decoder'], r['kind'])] < 2 and len(picks) < 10: seen[(r['decoder'], r['kind'])] += 1; picks.append(r)
    for r in picks: L.append(f"- `{r['decoder']}` {r['kind'].replace('-', ' ')} on rule `{r['rule']}` ({r['count']} inputs): [{r['witness']}]({r['witness']})")
    notes = cfg.get('notes')
    if notes and Path(notes).exists(): L += ['', Path(notes).read_text().strip()]
    if not pub:
        L += ['', '## Crashes, timeouts', '', f"{s['anomaly_count']} inputs with a crash or timeout in at least one decoder; counts by decoder: {s['anomaly_by_decoder']}. Inputs saved under `{folder}/anomalies/`; not investigated."]
    L += ['', '## Run', '', f'`./run.sh --inputs 10000 --seed {s["seed"]} --jobs 6` from `{adapter}`; outputs in `{folder}`.', '']
    (adapter / filename).write_text('\n'.join(L))


def write_sync(cfg, adapter, wdir, rows, outliers, pub):
    fmt = cfg['name']; keep = {r['witness'] for r in pub['rows'] + pub['outlier_rows'] + pub['pixel_rows'] + pub['intra_group_rows']}
    for f in wdir.iterdir():
        if 'witnesses/' + f.name not in keep:  # witnesses of crash rows stay out of the repo tree
            dest = folder_of(cfg) / 'witnesses-nonpublic'; dest.mkdir(exist_ok=True); shutil.move(str(f), str(dest / f.name))
    files = [f'{fmt}/{x}' for x in ('run.sh', 'build.sh', 'harness.py', 'reproduce.py', 'Cargo.toml', 'Cargo.lock', 'src/main.rs',
                                    'LENIENCY-PUBLIC.md', 'summary-public.json', 'notes.md') if (adapter / x).exists()]
    files += [f'{fmt}/{p.relative_to(adapter)}' for p in sorted(adapter.glob('runners/*.c')) + sorted(adapter.glob('tests/*.py')) + sorted(adapter.glob('*.py')) if f'{fmt}/{p.relative_to(adapter)}' not in files]
    files += [f'{fmt}/witnesses/{p.name}' for p in sorted(wdir.iterdir())]
    text = (f'# Files safe to copy to the public repository for {fmt.upper()}\n\nPaths relative to `/home/ws/projects/leniency/`. All inputs, witnesses and seeds are synthetic '
            f'(generated by `{fmt}/harness.py` from seed {pub["seed"]}); no third-party file bytes were used. Witnesses listed are exactly those referenced by `LENIENCY-PUBLIC.md`; '
            f'witnesses for rows involving a crash or timeout were removed. Do not copy `summary.json`, `LENIENCY.md` or anything under `/tmp/claude-0/`.\n\n'
            + ''.join(f'- `{f}`\n' for f in sorted(set(files))) + '\nShared (copy once): `xbm/lenient_text.py` (framework, imported by xpm/harness.py), `shared/gdk.c` (used unmodified by xbm/build.sh), root `engine.py`.\n')
    (adapter / 'SYNC-TO-REPO.md').write_text(text)


def main(cfg):
    import argparse
    p = argparse.ArgumentParser(); p.add_argument('--inputs', type=int, default=10000); p.add_argument('--seed', type=int, default=731)
    p.add_argument('--jobs', type=int, default=6); p.add_argument('--timeout', type=float, default=2.0)
    p.add_argument('--min-checks', type=int, default=40); p.add_argument('--max-rows', type=int, default=90)
    p.add_argument('--outlier-rows', type=int, default=3); p.add_argument('--render-only', action='store_true')
    a = p.parse_args()
    if a.render_only:  # re-render markdown (e.g. after editing notes.md) from the saved summaries
        ad = ROOT / cfg['name']
        for src, dst in (('summary.json', 'LENIENCY.md'), ('summary-public.json', 'LENIENCY-PUBLIC.md')):
            render(cfg, ad, json.loads((ad / src).read_text()), dst)
        pub = json.loads((ad / 'summary-public.json').read_text())
        write_sync(cfg, ad, ad / 'witnesses', pub['rows'], pub['outlier_rows'], pub)
        return
    run(cfg, a)
