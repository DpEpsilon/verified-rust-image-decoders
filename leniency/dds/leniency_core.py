"""Format-neutral driver shared by the DDS and HDR adapters (dds/ and hdr/).

It does not modify engine.py: it imports engine.invoke / classify / relation_signature and adds
(1) per-input records that keep adapter-specific fields (mip count, cube, float stats ...),
(2) a refine hook so an adapter can canonicalise pixel hashes (HDR float tolerance) before classification,
(3) per-decoder acceptance statistics, provenance-family tables, ddmin witnesses, LENIENCY(-PUBLIC).md.

An adapter passes a Config. Inputs/results/anomalies live under /tmp/claude-0/leniency-<fmt>/.
"""
import collections, concurrent.futures, hashlib, json, shutil, sys, time
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
import engine  # noqa: E402

KEEP = ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'alpha_present', 'orig_color_type',
        'mips', 'layers', 'cube', 'depth', 'policy', 'channels', 'orientation', 'exposure', 'nan_inf', 'max_rel_err_vs_ref', 'max_abs_diff_vs_ref', 'alt_sha256')


class Config:
    fmt = ''; ext = ''; adapter = None
    runners = {}            # name -> (exe, args)
    unavailable = {}        # name -> reason
    meta_keys = ()          # keys compared across decoders ("surface metadata")
    def generate(self, limit, seed): raise NotImplementedError
    alt_description = ''
    alt_hash = False        # adapter fills result['alt_sha256'] (convention-adjusted view); core reports a second classification
    def refine(self, path, results, timeout): return results     # may canonicalise hashes
    def pixel_difference(self, path, results, timeout, folder): return ''
    def intro(self): return ''
    def notes(self): return ''


def compact(results):
    return {k: {x: v[x] for x in KEEP if x in v} for k, v in results.items()}


def eff(results):
    """Drop columns that declared status 'na' (candidate does not cover this input) before classification."""
    return {k: v for k, v in results.items() if v['status'] != 'na'}


def sig(results):
    results = eff(results)
    return tuple((k, v['status'], v.get('width'), v.get('height'), v.get('rgba_sha256')) for k, v in results.items())


def run_all(cfg, path, timeout, dump_for=None):
    return {n: engine.invoke(path, r, timeout) for n, r in cfg.runners.items()}


def minimize(cfg, original, target, folder, timeout, checks):
    data = original; used = 0; n = 2; tmp = folder / ('minimize.input')
    while len(data) > 1 and used < checks:
        step = max(1, (len(data) + n - 1) // n); changed = False
        for start in range(0, len(data), step):
            cand = data[:start] + data[start + step:]
            if not cand: continue
            tmp.write_bytes(cand); used += 1
            res = cfg.refine(tmp, run_all(cfg, tmp, timeout), timeout)
            if sig(res) == target: data = cand; n = max(2, n - 1); changed = True; break
            if used >= checks: break
        if not changed:
            if n >= len(data): break
            n = min(len(data), n * 2)
    tmp.unlink(missing_ok=True)
    return data, used


def cell(v):
    st = v['status']
    if st == 'ok': return f"{v['width']}x{v['height']} `{v.get('rgba_sha256','')[:8]}`"
    if st == 'na': return 'n/a'
    if st == 'error': return 'reject `' + v.get('error_class', '?').replace('|', '/')[:26] + '`'
    return st


def describe(results):
    """Group decoders by identical outcome so a finding reads 'a, b accept X; c rejects (Y)'."""
    g = collections.OrderedDict()
    for k, v in results.items():
        key = cell(v) if v['status'] == 'ok' else (v['status'] if v['status'] != 'error' else 'reject (' + v.get('error_class', '?')[:30] + ')')
        if v['status'] == 'ok': key = 'accept ' + key
        g.setdefault(key, []).append(k)
    return '; '.join(f"{', '.join(ks)}: {key}" for key, ks in g.items())


def run(cfg, args):
    folder = Path('/tmp/claude-0/leniency-' + cfg.fmt); inputs = folder / 'inputs'
    if inputs.exists(): shutil.rmtree(inputs)
    inputs.mkdir(parents=True, exist_ok=True)
    items, nseeds = cfg.generate(args.inputs, args.seed)
    names = list(cfg.runners)
    manifest = folder / 'manifest.jsonl'; output = folder / 'results.jsonl'
    with manifest.open('w') as f:
        for i, (b, m) in enumerate(items):
            (inputs / f'{i:05}.{cfg.ext}').write_bytes(b)
            f.write(json.dumps({'id': i, 'length': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **m}) + '\n')
    started = time.monotonic()
    def job(item):
        i, _ = item; p = inputs / f'{i:05}.{cfg.ext}'
        return i, cfg.refine(p, run_all(cfg, p, args.timeout), args.timeout)
    records = []; counts = collections.Counter(); counts_alt = collections.Counter(); baselines = {}; changed = 0
    stats = {n: collections.Counter() for n in names}; families = collections.defaultdict(collections.Counter)
    seed_acc = collections.defaultdict(lambda: collections.defaultdict(collections.Counter)); rules = collections.defaultdict(list); anomalies = []; meta_dis = 0; pixel_cases = collections.defaultdict(list)
    with output.open('w') as out, concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for i, res in pool.map(job, list(enumerate(items))):
            m = items[i][1]; full = res; res = eff(res); cls = engine.classify(res); counts[cls] += 1
            if cfg.alt_hash:
                counts_alt[engine.classify({k: (dict(v, rgba_sha256=v['alt_sha256']) if v['status'] == 'ok' else v) for k, v in res.items()})] += 1
            rs = engine.relation_signature(res)
            if m['source'] == 'seed': baselines[m['name']] = rs
            mchanged = m['source'] == 'mutation' and rs != baselines.get(m['seed'])
            changed += mchanged
            for n, v in full.items(): stats[n][v['status']] += 1
            ok = [n for n, v in res.items() if v['status'] == 'ok']
            if len(ok) < len(res) and ok and not any(v['status'] in ('crash', 'timeout') for v in res.values()):
                for n in ok:
                    if len(ok) == 1: stats[n]['only-acceptor'] += 1
                for n, v in res.items():
                    if v['status'] == 'error' and len(ok) == len(res) - 1: stats[n]['only-rejector'] += 1
            for k in cfg.meta_keys:
                if len({json.dumps(res[n].get(k)) for n in ok if k in res[n]}) > 1: meta_dis += 1; break
            families[m.get('family', '?')][cls] += 1
            if m['source'] == 'seed':
                for n, v in full.items(): seed_acc[m['family']][n][v['status']] += 1
            if cls == 'crash-or-timeout': anomalies.append(i)
            elif cls not in ('all-reject', 'all-accept-identical-pixels') and (m['source'] == 'seed' or mchanged):
                rules[m['rule']].append((i, m, full, cls))
                if cls == 'accept-different-pixels': pixel_cases[m['rule']].append((i, m, full, cls))
            out.write(json.dumps({'id': i, 'rule': m['rule'], 'family': m.get('family'), 'classification': cls, 'results': compact(full)}, separators=(',', ':')) + '\n')
            if (i + 1) % 1000 == 0: print(f'{cfg.fmt} {i+1}/{len(items)} {time.monotonic()-started:.0f}s', flush=True)
    elapsed = round(time.monotonic() - started, 1)
    adapter = cfg.adapter; wit = adapter / 'witnesses'
    if wit.exists(): shutil.rmtree(wit)
    wit.mkdir()
    def witness(i, res, tag, preserve=sig):
        src = inputs / f'{i:05}.{cfg.ext}'
        shrunk, used = minimize(cfg, src.read_bytes(), sig(res), folder, args.timeout, args.min_checks)
        p = wit / f'{tag}.{cfg.ext}'; p.write_bytes(shrunk)
        return p, used, len(shrunk)
    rows = []
    for rule, entries in sorted(rules.items(), key=lambda kv: -len(kv[1])):
        pats = collections.defaultdict(list)
        for e in entries: pats[sig(e[2])].append(e)
        i, m, res, cls = max(pats.values(), key=len)[0]
        p, used, ln = witness(i, res, f'{len(rows)+1:02}-' + rule.replace(':', '-').replace('/', '-')[:40])
        rows.append({'rule': rule, 'count': len(entries), 'patterns': len(pats), 'original_id': i, 'witness': 'witnesses/' + p.name,
                     'mutation': m, 'classification': cls, 'results': compact(res), 'description': describe(res),
                     'pixel_difference': cfg.pixel_difference(p, res, args.timeout, folder) if cls == 'accept-different-pixels' else '',
                     'min_checks': used, 'original_length': len(items[i][0]), 'witness_length': ln})
    prow = []
    for rule, cases in sorted(pixel_cases.items(), key=lambda kv: -len(kv[1]))[:10]:
        i, m, res, cls = cases[0]
        p, used, ln = witness(i, res, f'pixel-{len(prow)+1:02}')
        prow.append({'rule': rule, 'count': len(cases), 'id': i, 'witness': 'witnesses/' + p.name, 'results': compact(res),
                     'description': describe(res), 'difference': cfg.pixel_difference(p, res, args.timeout, folder)})
    adir = folder / 'anomalies'; adir.mkdir(exist_ok=True)
    for s in adir.glob('*'): s.unlink()
    for i in anomalies[:1000]: shutil.copyfile(inputs / f'{i:05}.{cfg.ext}', adir / f'{i:05}.{cfg.ext}')
    anomaly_by_decoder = collections.Counter()
    for line in output.open():
        r = json.loads(line)
        if r['classification'] == 'crash-or-timeout':
            for n, v in r['results'].items():
                if v['status'] in ('crash', 'timeout'): anomaly_by_decoder[f"{n}:{v['status']}"] += 1
    s = {'format': cfg.fmt, 'inputs': len(items), 'seed_variants': nseeds, 'seed': args.seed, 'counts': dict(counts), 'counts_convention_adjusted': dict(counts_alt),
         'changed_mutations': changed, 'elapsed_seconds': elapsed, 'decoders': names, 'unavailable': cfg.unavailable,
         'decoder_status': {n: dict(c) for n, c in stats.items()}, 'families': {k: dict(v) for k, v in sorted(families.items())},
         'seed_acceptance': {f: {n: dict(c) for n, c in d.items()} for f, d in sorted(seed_acc.items())}, 'surface_metadata_disagreements': meta_dis, 'rows': rows, 'pixel_rows': prow,
         'anomaly_count': len(anomalies), 'anomaly_by_decoder': dict(anomaly_by_decoder),
         'manifest': str(manifest), 'raw_results': str(output)}
    (adapter / 'summary.json').write_text(json.dumps(s, indent=2) + '\n')
    render(cfg, s, adapter / 'LENIENCY.md', False)
    pub = public(s, output)
    (adapter / 'summary-public.json').write_text(json.dumps(pub, indent=2) + '\n')
    render(cfg, pub, adapter / 'LENIENCY-PUBLIC.md', True)
    print(json.dumps({'format': cfg.fmt, 'inputs': len(items), 'counts': counts, 'elapsed_seconds': elapsed, 'rows': len(rows), 'anomalies': len(anomalies)}, indent=2))


def public(s, results_path):
    """Recount without crash/timeout inputs; drop paths, anomaly ids and crash cells."""
    p = {k: v for k, v in s.items() if k not in ('manifest', 'raw_results', 'anomaly_by_decoder', 'anomaly_count')}
    adjc = collections.Counter(); stats = {n: collections.Counter() for n in s['decoders']}; counts = collections.Counter(); fam = collections.defaultdict(collections.Counter); n = 0
    for line in results_path.open():
        r = json.loads(line)
        if r['classification'] == 'crash-or-timeout': continue
        r['results'] = {k: v for k, v in r['results'].items()}
        n += 1; counts[r['classification']] += 1
        if s.get('counts_convention_adjusted'):
            adj = engine.classify({k: (dict(v, rgba_sha256=v['alt_sha256']) if v['status'] == 'ok' else v) for k, v in r['results'].items() if v['status'] != 'na'})
            adjc[adj] += 1
        fam[r['family']][r['classification']] += 1
        ok = [k for k, v in r['results'].items() if v['status'] == 'ok']
        for k, v in r['results'].items(): stats[k][v['status']] += 1
        if len(ok) == 1: stats[ok[0]]['only-acceptor'] += 1
        if len(ok) == len([v for v in r['results'].values() if v['status'] != 'na']) - 1:
            for k, v in r['results'].items():
                if v['status'] == 'error': stats[k]['only-rejector'] += 1
    p.update(public_subset=True, inputs=n, counts=dict(counts), counts_convention_adjusted=dict(adjc), decoder_status={k: dict(v) for k, v in stats.items()},
             families={k: dict(v) for k, v in sorted(fam.items())}, excluded_crash_or_timeout_inputs=s['anomaly_count'])
    p.pop('changed_mutations', None); p.pop('surface_metadata_disagreements', None)
    return p


def adjusted_class(row):
    res = {k: (dict(v, rgba_sha256=v['alt_sha256']) if v['status'] == 'ok' and 'alt_sha256' in v else v) for k, v in row['results'].items() if v['status'] != 'na'}
    return engine.classify(res)


def significance(row):
    c = adjusted_class(row)
    rank = {'accept-different-pixels': 3, 'accept-vs-reject': 2}.get(c, 1)
    return (rank, row['count'])


def render(cfg, s, path, pub):
    names = s['decoders']; folder = f'/tmp/claude-0/leniency-{cfg.fmt}'
    L = [f'# {cfg.fmt.upper()} leniency matrix' + (' (public: synthetic inputs, crash/timeout inputs excluded)' if pub else ''), '',
         f"Seed {s['seed']}; {s['inputs']} inputs (synthetic only; no third-party files); {s['seed_variants']} valid seeds; "
         + ('' if pub else f"{s['changed_mutations']} mutations changed the acceptance/pixel relation versus their seed; ")
         + f"{s['elapsed_seconds']} s for the decoder pass.", '', cfg.intro(), '', 'Classification | Count', '--- | ---:']
    for c in ('all-accept-identical-pixels', 'accept-different-pixels', 'accept-vs-reject', 'all-reject', 'crash-or-timeout'):
        if pub and c == 'crash-or-timeout': L.append(f"crash-or-timeout (excluded from this file) | {s['excluded_crash_or_timeout_inputs']}")
        else: L.append(f"{c} | {s['counts'].get(c, 0)}")
    if s.get('counts_convention_adjusted'):
        L += ['', 'Convention-adjusted view (' + cfg.alt_description + '):', '', 'Classification (adjusted) | Count', '--- | ---:']
        for c in ('all-accept-identical-pixels', 'accept-different-pixels', 'accept-vs-reject', 'all-reject', 'crash-or-timeout'):
            if not (pub and c == 'crash-or-timeout'): L.append(f"{c} | {s['counts_convention_adjusted'].get(c, 0)}")
    L += ['', '## Per decoder', '', '| Decoder | ok | error | n/a | crash | timeout | only acceptor | only rejector |', '| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
    for n in names:
        c = s['decoder_status'][n]; L.append(f"| {n} | {c.get('ok',0)} | {c.get('error',0)} | {c.get('na',0)} | {'-' if pub else c.get('crash',0)} | {'-' if pub else c.get('timeout',0)} | {c.get('only-acceptor',0)} | {c.get('only-rejector',0)} |")
    L += ['', 'Unavailable candidates: ' + ('; '.join(f'`{k}` ({v})' for k, v in s['unavailable'].items()) or 'none') + '.', '',
          'only acceptor: the decoder accepted while every other decoder rejected. only rejector: it rejected while every other decoder accepted. Both exclude inputs with a crash or timeout.', '',
          '## Provenance families (seed variant family, all mutations of its seeds)', '', '| Family | inputs | all-accept-identical | accept-different-pixels | accept-vs-reject | all-reject |' + ('' if pub else ' crash-or-timeout |'),
          '| --- | ---: | ---: | ---: | ---: | ---: |' + ('' if pub else ' ---: |')]
    for fam, c in s['families'].items():
        L.append(f"| {fam} | {sum(c.values())} | {c.get('all-accept-identical-pixels',0)} | {c.get('accept-different-pixels',0)} | {c.get('accept-vs-reject',0)} | {c.get('all-reject',0)} |" + ('' if pub else f" {c.get('crash-or-timeout',0)} |"))
    L += ['', '## Unmutated valid seeds: acceptance per decoder (ok / seeds in family)', '', '| Family | seeds | ' + ' | '.join(names) + ' |', '| --- | ---: | ' + ' | '.join(['---:'] * len(names)) + ' |']
    for f, d in s['seed_acceptance'].items():
        tot = sum(next(iter(d.values())).values())
        L.append(f"| {f} | {tot} | " + ' | '.join(f"{d[n].get('ok', 0)}" + (f" ({d[n]['na']} n/a)" if d[n].get('na') else '') + (f" ({d[n]['crash']} crash)" if d[n].get('crash') and not pub else '') for n in names) + ' |')
    L += ['', '## Representative matrix (one row per rule; modal outcome among inputs that changed the seed relation)', '',
          '| Rule (inputs; patterns) | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['rows']:
        d = '; ' + r['pixel_difference'] if r.get('pixel_difference') else ''
        L.append(f"| {r['rule']} ({r['count']}; {r['patterns']}{d}) | [#{r['original_id']}]({r['witness']}) | " + ' | '.join(cell(r['results'][n]) for n in names) + ' |')
    L += ['', '## Same acceptance, different pixels', '', '| Rule | Witness | Difference | Outcomes |', '| --- | --- | --- | --- |']
    for r in s['pixel_rows']: L.append(f"| {r['rule']} ({r['count']}) | [#{r['id']}]({r['witness']}) | {r['difference']} | {r['description']} |")
    if not s['pixel_rows']: L.append('| none | | | |')
    L += ['', '## Ten most significant leniency findings (ranked: pixel disagreement, then accept-vs-reject, by input count' + ('; rows whose only difference is the convention-adjusted equivalence are excluded' if s.get('counts_convention_adjusted') else '') + ')', '']
    top = sorted((r for r in s['rows'] if adjusted_class(r) in ('accept-different-pixels', 'accept-vs-reject')), key=significance, reverse=True)[:10]
    for r in top: L.append(f"- `{r['rule']}` ({r['count']} inputs, {adjusted_class(r)}): {r['description']}. Witness {r['witness']}.")
    if not pub: L += ['', f"Surface-metadata disagreements (decoders reporting different mip/layer/cube/depth values): {s['surface_metadata_disagreements']}."]
    L += ['', cfg.notes(), '']
    if not pub:
        L += ['## Crashes, timeouts, aborts', '', f"{s['anomaly_count']} inputs ({', '.join(f'{k} x{v}' for k, v in s['anomaly_by_decoder'].items()) or 'none'}); inputs saved under `{folder}/anomalies/`. Counted only: not investigated, minimised or reported.", '']
    L += ['## Run', '', f'`./run.sh --inputs 10000 --seed 731 --jobs 4` from `{cfg.adapter}`. `python3 reproduce.py FILE` runs one file through every decoder. Per-input results and manifest: `{folder}`.', '']
    Path(path).write_text('\n'.join(L))


def reproduce(cfg, files):
    for f in files:
        res = cfg.refine(Path(f), run_all(cfg, Path(f), 5), 5)
        print(f, engine.classify(eff(res)))
        for n, v in res.items(): print('  ', n, json.dumps({k: v[k] for k in KEEP if k in v}))


def interleave_by_rule(cands, rng):
    """Round-robin over rules (each rule's candidates shuffled) so a bounded run touches every rule."""
    groups = collections.OrderedDict()
    for c in cands: groups.setdefault(c[1]['rule'], []).append(c)
    keys = list(groups); rng.shuffle(keys)
    for g in groups.values(): rng.shuffle(g)
    out = []
    while any(groups.values()):
        for k in keys:
            if groups[k]: out.append(groups[k].pop())
    return out
