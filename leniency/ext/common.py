#!/usr/bin/env python3
"""Shared pipeline for the PNM / QOI / farbfeld adapters (the older format_harness.py is not modified).

An adapter module exposes: NAME, EXT, DECODERS {name: (exe, args)}, UNAVAILABLE {name: reason},
GROUPS {name: independent-implementation group}, seeds() -> [(bytes, meta)],
mutations(data, meta, rng) -> iterator of (bytes, {'rule','field','old','new'}) with finite, deterministic,
format-specific header/body mutations; generic truncation / trailing-byte / byte-flip rules come from here.
Every input is synthetic (generated from code); no third-party corpus bytes are read.
"""
import collections, concurrent.futures, hashlib, json, random, re, shutil, sys, time
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
import engine  # noqa: E402  (shared, unmodified)

BIN = Path('/tmp/claude-0/leniency-ext-bin')
RUNNER = BIN / 'leniency-ext-runner'
GDK = BIN / 'gdk-formats'
CLASSES = ('all-accept-identical-pixels', 'accept-identical-rgba8-different-rgba16', 'accept-different-pixels',
           'accept-vs-reject', 'all-reject', 'crash-or-timeout')


def folder(name):
    return Path('/tmp/claude-0/leniency-' + name)


# ---------------------------------------------------------------- generic mutators
def generic_mutations(data, hdr_len, rng):
    n = len(data)
    cuts = sorted({0, 1, 2, 4, 8, min(hdr_len, n), max(hdr_len - 1, 0), hdr_len + 1, n // 4, n // 2, n - 1, n - 2, n - 3, n - 4, n - 8}
                  - {n}) if n else []
    for c in cuts:
        if 0 <= c < n:
            yield data[:c], {'rule': 'truncation', 'field': 'length', 'old': n, 'new': c}
    for t in (b'\0', b'\n', b'\xff', b'\0' * 7 + b'\1', b'\0' * 16, b'P6 1 1 255\n', bytes(rng.randrange(256) for _ in range(33))):
        yield data + t, {'rule': 'trailing-bytes', 'field': 'tail', 'old': 0, 'new': len(t)}
    yield data + data, {'rule': 'trailing-bytes', 'field': 'tail', 'old': 0, 'new': n}


def flips(data, hdr_len, rng):
    """Infinite stream of seeded byte/bit flips; two thirds target the header."""
    while len(data) > 0:
        b = bytearray(data)
        for _ in range(rng.choice((1, 1, 1, 2, 3))):
            hi = max(1, min(hdr_len, len(b))) if rng.random() < 2 / 3 else len(b)
            i = rng.randrange(hi)
            old = b[i]
            b[i] = b[i] ^ (1 << rng.randrange(8)) if rng.random() < 0.5 else rng.randrange(256)
        yield bytes(b), {'rule': 'random-flip', 'field': f'byte {i}', 'old': old, 'new': b[i]}


def generate(adapter, limit, seed):
    rng = random.Random(seed)
    seeds = list(adapter.seeds())
    seen = {hashlib.sha256(b).digest() for b, _ in seeds}
    out = [(b, dict(m, source='seed', rule='seed', seed=m['name'])) for b, m in seeds][:limit]
    series = []
    for b, m in seeds:
        r = random.Random(f'{seed}/{m["name"]}')
        hdr = m.get('header_len', 16)
        spec = list(adapter.mutations(b, m, r))
        r.shuffle(spec)
        gen = [*spec, *generic_mutations(b, hdr, r)]
        series.append((m, iter(gen), flips(b, hdr, r), len(spec)))
    # round-robin: format-specific and generic mutations first, then random flips fill the remainder
    active = list(range(len(series)))
    phase = 0
    while len(out) < limit and active:
        progressed = False
        for i in list(active):
            m, it, fl, _ = series[i]
            nxt = next(it, None) if phase == 0 else next(fl, None)
            while nxt is not None and hashlib.sha256(nxt[0]).digest() in seen:
                nxt = next(it, None) if phase == 0 else next(fl, None)
            if nxt is None:
                if phase == 0:
                    series[i] = (m, iter(()), fl, 0)
                else:
                    active.remove(i)
                continue
            seen.add(hashlib.sha256(nxt[0]).digest())
            out.append((nxt[0], dict(nxt[1], source='mutation', seed=m['name'], seed_header=m.get('header_len'))))
            progressed = True
            if len(out) >= limit:
                break
        if phase == 0 and not progressed:
            phase = 1
    return out, len(seeds)


# ---------------------------------------------------------------- classification helpers
def classify16(results):
    base = engine.classify(results)
    if base != 'all-accept-identical-pixels':
        return base
    hs = {v.get('rgba16_sha256') for v in results.values() if v['status'] == 'ok' and v.get('rgba16_sha256')}
    return 'accept-identical-rgba8-different-rgba16' if len(hs) > 1 else base


def rel_sig(results):
    """Statuses plus pairwise equality of dimensions and both pixel hashes (independent of actual values)."""
    keys = list(results)
    ok = [k for k in keys if results[k]['status'] == 'ok']
    g = lambda k, f: results[k].get(f)
    return (tuple(results[k]['status'] for k in keys),
            tuple((a, b, g(a, 'width') == g(b, 'width') and g(a, 'height') == g(b, 'height'),
                   g(a, 'rgba_sha256') == g(b, 'rgba_sha256'), g(a, 'rgba16_sha256') == g(b, 'rgba16_sha256'))
                  for i, a in enumerate(ok) for b in ok[i + 1:]))


def exact_sig(results):
    return tuple((k, v['status'], v.get('width'), v.get('height'), v.get('rgba_sha256'), v.get('rgba16_sha256')) for k, v in results.items())


def compact(results):
    keep = ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'rgba16_sha256', 'alpha_present', 'orig_color_type')
    return {k: {x: v[x] for x in keep if x in v} for k, v in results.items()}


def invoke_all(adapter, path, timeout, dump_for=None):
    return {n: engine.invoke(path, r, timeout, dump_for) for n, r in adapter.DECODERS.items()}


def pixel_difference(adapter, path, results, timeout, work):
    good = [k for k, v in results.items() if v['status'] == 'ok']
    if len(good) < 2:
        return ''
    if len({(results[k]['width'], results[k]['height']) for k in good}) > 1:
        return 'dimensions'
    dumps = {}
    for k in good:
        tmp = work / f'px-{k}.rgba'
        v = engine.invoke(path, adapter.DECODERS[k], timeout, tmp)
        if v['status'] != 'ok' or not tmp.exists():
            return 'rgba-dump-unavailable'
        dumps[k] = tmp.read_bytes()
        tmp.unlink()
    ref = dumps[good[0]]
    notes = set()
    for k in good[1:]:
        d = dumps[k]
        if d == ref or len(d) != len(ref):
            continue
        px = list(zip(range(0, len(d), 4)))
        diffs = [(i, [abs(ref[i + c] - d[i + c]) for c in range(4)]) for (i,) in px if ref[i:i + 4] != d[i:i + 4]]
        if all(x[1][:3] == [0, 0, 0] for x in diffs):
            notes.add('alpha-only')
        elif all(max(x[1]) <= 1 for x in diffs):
            notes.add('rounding(<=1)')
        elif all(all(ref[i + c] + d[i + c] == 255 for c in range(3)) for i, _ in diffs):
            notes.add('inverted-rgb')
        elif sorted(ref[i + c] for i, _ in diffs for c in range(3)) and False:
            pass
        else:
            notes.add(f'values(max|delta|={max(max(x[1]) for x in diffs)},{len(diffs)}/{len(d) // 4}px)')
    return ','.join(sorted(notes))


# ---------------------------------------------------------------- minimisation
def minimize(adapter, data, target, timeout, work, checks):
    used, n, tmp = 0, 2, work / 'minimize.input'
    while len(data) > 1 and used < checks:
        step = max(1, (len(data) + n - 1) // n)
        changed = False
        for start in range(0, len(data), step):
            cand = data[:start] + data[start + step:]
            if not cand:
                continue
            tmp.write_bytes(cand)
            used += 1
            if exact_sig(invoke_all(adapter, tmp, timeout)) == target:
                data, n, changed = cand, max(2, n - 1), True
                break
            if used >= checks:
                break
        if not changed:
            if n >= len(data):
                break
            n = min(len(data), n * 2)
    tmp.unlink(missing_ok=True)
    return data, used


# ---------------------------------------------------------------- run
def vote_groups(adapter, results):
    """Collapse decoders that share an implementation into one vote (majority of their statuses)."""
    g = collections.defaultdict(list)
    for k, v in results.items():
        g[adapter.GROUPS.get(k, k)].append(v['status'] == 'ok')
    return {grp: (sum(x) * 2 > len(x)) for grp, x in g.items()}


def run(adapter, args):
    if args.reanalyse:
        return reanalyse(adapter, args)
    fol = folder(adapter.NAME)
    inputs = fol / 'inputs'
    shutil.rmtree(inputs, ignore_errors=True)
    inputs.mkdir(parents=True, exist_ok=True)
    work = fol / 'work'
    work.mkdir(exist_ok=True)
    items, nseeds = generate(adapter, args.inputs, args.seed)
    ext = adapter.EXT
    with (fol / 'manifest.jsonl').open('w') as f:
        for i, (b, m) in enumerate(items):
            (inputs / f'{i:05}.{ext}').write_bytes(b)
            f.write(json.dumps({'id': i, 'length': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **{k: v for k, v in m.items() if k != 'header_len'}}, default=str) + '\n')
    names = list(adapter.DECODERS)
    started = time.monotonic()

    def job(it):
        i = it[0]
        return i, invoke_all(adapter, inputs / f'{i:05}.{ext}', args.timeout)

    records = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool, (fol / 'results.jsonl').open('w') as out:
        for n, (i, res) in enumerate(pool.map(job, enumerate(items))):
            cls = classify16(res)
            rec = {'id': i, 'rule': items[i][1]['rule'], 'classification': cls, 'results': compact(res)}
            records.append(rec)
            out.write(json.dumps(rec, separators=(',', ':')) + '\n')
            if (n + 1) % 500 == 0:
                print(f'{adapter.NAME} {n + 1}/{len(items)} {time.monotonic() - started:.0f}s', flush=True)
    elapsed = round(time.monotonic() - started, 1)
    summary = analyse(adapter, args, items, records, nseeds, elapsed, fol, work)
    write_outputs(adapter, summary)


def reanalyse(adapter, args):
    """Rebuild summaries from the saved inputs, manifest and results.jsonl without re-running the decoder pass."""
    fol = folder(adapter.NAME)
    items, records = [], []
    for line in (fol / 'manifest.jsonl').open():
        m = json.loads(line)
        items.append(((fol / 'inputs' / f'{m["id"]:05}.{adapter.EXT}').read_bytes(), m))
    for line in (fol / 'results.jsonl').open():
        records.append(json.loads(line))
    old = json.loads((ROOT / adapter.NAME / 'summary.json').read_text())
    work = fol / 'work'
    work.mkdir(exist_ok=True)
    summary = analyse(adapter, args, items, records, old['seed_files'], old['elapsed_seconds'], fol, work)
    write_outputs(adapter, summary)


def write_outputs(adapter, summary):
    ad = ROOT / adapter.NAME
    (ad / 'summary.json').write_text(json.dumps({k: v for k, v in summary.items() if not k.startswith('_')}, indent=2, default=str) + '\n')
    (ad / 'LENIENCY.md').write_text(render(adapter, summary, False))
    pub = public_view(adapter, summary)
    (ad / 'summary-public.json').write_text(json.dumps(pub, indent=2, default=str) + '\n')
    (ad / 'LENIENCY-PUBLIC.md').write_text(render(adapter, pub, True))
    print(json.dumps({'format': adapter.NAME, 'inputs': summary['inputs'], 'counts': summary['counts'], 'elapsed_seconds': summary['elapsed_seconds'],
                      'rows': len(summary['rows']), 'crash_timeout_inputs': summary['anomaly_count']}, indent=1))


def analyse(adapter, args, items, records, nseeds, elapsed, fol, work):
    names = list(adapter.DECODERS)
    counts = collections.Counter(r['classification'] for r in records)
    base = {}
    for r in records:
        m = items[r['id']][1]
        if m['source'] == 'seed':
            base[m['name']] = rel_sig(r['results'])
    changed = []
    rules = collections.defaultdict(list)
    accept = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0]))   # rule -> decoder -> [ok, total]
    for r in records:
        m = items[r['id']][1]
        ch = m['source'] == 'seed' or rel_sig(r['results']) != base.get(m['seed'])
        r['changed'] = bool(ch)
        if m['source'] == 'mutation' and ch:
            changed.append(r['id'])
        crash = r['classification'] == 'crash-or-timeout'
        r['crash'] = crash
        if m['source'] == 'mutation':
            for k, v in r['results'].items():
                if v['status'] in ('ok', 'error'):
                    accept[m['rule']][k][1] += 1
                    accept[m['rule']][k][0] += v['status'] == 'ok'
        if r['classification'] not in ('all-reject', 'all-accept-identical-pixels') and ch and not crash:
            rules[m['rule'] + (':' + re.sub(r'\d+', 'N', str(m['field'])) if m['source'] == 'mutation' and m.get('field') else '')].append(r)
    # decoder-level counts
    per = {k: collections.Counter() for k in names}
    lone = {k: collections.Counter() for k in names}
    for r in records:
        votes = vote_groups(adapter, r['results'])
        for k, v in r['results'].items():
            per[k][v['status']] += 1
            grp = adapter.GROUPS.get(k, k)
            others = [x for g, x in votes.items() if g != grp]
            if v['status'] == 'ok' and others and not any(others):
                lone[k]['lone-accept'] += 1
            if v['status'] == 'error' and others and all(others) and len(others) >= 2:
                lone[k]['lone-reject'] += 1
    # witnesses
    wit = ROOT / adapter.NAME / 'witnesses'
    shutil.rmtree(wit, ignore_errors=True)
    wit.mkdir()
    rows = []
    cand = []
    for rule, rs in rules.items():
        pats = collections.defaultdict(list)
        for r in rs:
            pats[rel_sig(r['results'])].append(r)
        for grp in sorted(pats.values(), key=len, reverse=True)[:args.patterns_per_rule]:
            cand.append((rule, len(rs), len(pats), grp))
    cand.sort(key=lambda c: -len(c[3]))
    for rule, total, npat, grp in cand[:args.max_rows]:
        pick = grp[0]
        i = pick['id']
        data = items[i][0]
        shrunk, used = minimize(adapter, data, exact_sig(pick['results']), args.timeout, work, args.min_checks)
        path = wit / f'{len(rows) + 1:03}-{rule.replace(":", "-").replace("/", "_").replace(" ", "_")}.{adapter.EXT}'
        path.write_bytes(shrunk)
        rows.append({'rule': rule, 'count': len(grp), 'rule_total': total, 'patterns': npat, 'original_id': i, 'witness': 'witnesses/' + path.name,
                     'mutation': {k: v for k, v in items[i][1].items() if k != 'header_len'}, 'classification': pick['classification'],
                     'results': pick['results'], 'min_checks': used, 'original_length': len(data), 'witness_length': len(shrunk),
                     'crash_in_row': any(r['crash'] for r in grp),
                     'pixel_difference': pixel_difference(adapter, path, pick['results'], args.timeout, work) if pick['classification'] == 'accept-different-pixels' else ''})
    anomalies = [r for r in records if r['crash']]
    adir = fol / 'anomalies'
    shutil.rmtree(adir, ignore_errors=True)
    adir.mkdir()
    for r in anomalies:
        shutil.copyfile(fol / 'inputs' / f'{r["id"]:05}.{adapter.EXT}', adir / f'{r["id"]:05}.{adapter.EXT}')
    crash_by = {k: {s: per[k][s] for s in ('crash', 'timeout') if per[k][s]} for k in names}
    shutil.rmtree(work, ignore_errors=True)
    return {'format': adapter.NAME, 'inputs': len(items), 'seed_files': nseeds, 'seed': args.seed, 'counts': dict(counts),
            'changed_mutations': len(changed), 'elapsed_seconds': elapsed, 'decoders': names, 'unavailable': adapter.UNAVAILABLE,
            'groups': adapter.GROUPS, 'per_decoder': {k: dict(v) for k, v in per.items()},
            'lone': {k: dict(v) for k, v in lone.items()}, 'rows': rows,
            'rule_acceptance': {r: {k: v for k, v in d.items()} for r, d in accept.items()},
            'anomaly_count': len(anomalies), 'crash_by_decoder': crash_by,
            'anomaly_ids': [r['id'] for r in anomalies], 'manifest': str(fol / 'manifest.jsonl'), 'raw_results': str(fol / 'results.jsonl'),
            '_records': [{'id': r['id'], 'rule': r['rule'], 'classification': r['classification'], 'changed': r['changed'], 'crash': r['crash'],
                          'results': r['results']} for r in records], '_meta': [{k: v for k, v in m.items() if k != 'header_len'} for _, m in items]}


def public_view(adapter, s):
    """Same analysis restricted to inputs where no decoder crashed or timed out; all inputs are synthetic."""
    names = s['decoders']
    recs = [r for r in s['_records'] if not r['crash']]
    meta = s['_meta']
    counts = collections.Counter(r['classification'] for r in recs)
    accept = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0]))
    for r in recs:
        if meta[r['id']]['source'] == 'mutation':
            for k, v in r['results'].items():
                accept[meta[r['id']]['rule']][k][1] += 1
                accept[meta[r['id']]['rule']][k][0] += v['status'] == 'ok'
    per = {k: collections.Counter(v['results'][k]['status'] for v in recs) for k in names}
    p = {k: v for k, v in s.items() if k not in ('_records', '_meta', 'anomaly_ids', 'crash_by_decoder', 'anomaly_count', 'manifest', 'raw_results')}
    p.update(public_subset=True, inputs=len(recs), counts={k: v for k, v in counts.items() if k != 'crash-or-timeout'},
             per_decoder={k: {a: b for a, b in v.items() if a in ('ok', 'error')} for k, v in per.items()},
             rows=[r for r in s['rows'] if not r['crash_in_row'] and not any(v['status'] in ('crash', 'timeout') for v in r['results'].values())],
             rule_acceptance={r: dict(d) for r, d in accept.items()})
    p['lone'] = {k: v for k, v in s['lone'].items()}
    return p


def cell(v):
    st = v['status']
    if st == 'ok':
        return f"ok {v['width']}x{v['height']} `{v['rgba_sha256'][:8]}`"
    if st == 'error':
        return 'reject `' + v.get('error_class', '?').replace('|', '/')[:24] + '`'
    return st


def render(adapter, s, public):
    names = s['decoders']
    fol = f'/tmp/claude-0/leniency-{adapter.NAME}'
    L = [f'# {adapter.NAME.upper()} leniency matrix' + (' (public view)' if public else ''), '',
         f"Seed {s['seed']}; {s['inputs']} synthetic inputs from {s['seed_files']} generated valid seed files; "
         f"{'' if public else str(s['changed_mutations']) + ' mutations changed the acceptance/pixel pattern relative to their seed; '}"
         f"{s['elapsed_seconds']} s for the decoder pass. Every input is generated by code; no third-party corpus bytes are used. "
         + ('Inputs on which any decoder crashed or timed out are excluded from this public view.' if public else
            f"{s['anomaly_count']} inputs made at least one decoder crash or time out; they are saved under `{fol}/anomalies/` and only counted here."), '',
         'Classification | Inputs', '--- | ---:']
    for c in CLASSES:
        if public and c == 'crash-or-timeout':
            continue
        L.append(f"{c} | {s['counts'].get(c, 0)}")
    L += ['', '## Decoders', '', '| Decoder | Votes as | Accepted | Rejected |' + ('' if public else ' Crash/timeout | Lone accept | Lone reject |'),
          '| --- | --- | ---: | ---: |' + ('' if public else ' ---: | ---: | ---: |')]
    for k in names:
        d = s['per_decoder'][k]
        extra = '' if public else f" {sum(s['crash_by_decoder'][k].values())} | {s['lone'][k].get('lone-accept', 0)} | {s['lone'][k].get('lone-reject', 0)} |"
        L.append(f"| `{k}` | {s['groups'].get(k, k)} | {d.get('ok', 0)} | {d.get('error', 0)} |{extra}")
    for k, why in s['unavailable'].items():
        L.append(f'| `{k}` | unavailable | - | - |' + ('' if public else ' - | - | - |'))
    L += ['', 'Unavailable candidates (not counted as rejecting): ' + '; '.join(f'**{k}**: {w}' for k, w in s['unavailable'].items()) if s['unavailable'] else '', '',
          'Lone accept / lone reject: the decoder differs from every other independent implementation group (decoders that share an implementation, e.g. `image` and `glycin`, form one group and one vote). Lone reject needs at least two other groups.' if not public else '',
          '', 'Normalisation: every output is expanded to RGBA16 (8-bit v -> v*257; grey -> r=g=b; no alpha -> 65535) and RGBA8 = (v+128)/257. The cell hash is RGBA8; classification `accept-identical-rgba8-different-rgba16` means 8-bit hashes agree but 16-bit hashes do not. Decoders that return unscaled low-maxval samples are hashed as returned. `harness-limit` means the decoder\'s header parse succeeded and the image exceeded 16,000,000 pixels; it was not decoded.', '',
          '## Representative matrix (one minimised witness per rule)', '',
          '| Rule: field (inputs with this outcome pattern; patterns in rule) | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['rows']:
        det = '; ' + r['pixel_difference'] if r.get('pixel_difference') else ''
        L.append(f"| {r['rule']} ({r['count']}/{r['rule_total']}; {r['patterns']}{det}) | [#{r['original_id']}]({r['witness']}) | " + ' | '.join(cell(r['results'][k]) for k in names) + ' |')
    if not s['rows']:
        L.append('| No rule produced a disagreement among the counted decoders (crash/timeout inputs excluded) | - | ' + ' | '.join(['-'] * len(names)) + ' |')
    L += ['', '## Acceptance by mutation rule', '', 'Fraction of mutated inputs each decoder accepted, per mutation family (crash/timeout inputs counted only in the local view).', '',
          '| Rule | Inputs | ' + ' | '.join(names) + ' |', '| --- | ---: | ' + ' | '.join(['---:'] * len(names)) + ' |']
    for rule, d in sorted(s['rule_acceptance'].items(), key=lambda kv: -max(v[1] for v in kv[1].values())):
        tot = max(v[1] for v in d.values())
        L.append(f'| {rule} | {tot} | ' + ' | '.join(f"{d[k][0]}/{d[k][1]}" if k in d else '-' for k in names) + ' |')
    L += ['', '## Most significant observations (generated from the rows above)', '']
    scored = [r for r in s['rows'] if r['classification'] in ('accept-vs-reject', 'accept-different-pixels', 'accept-identical-rgba8-different-rgba16')]
    scored.sort(key=lambda r: -r['count'])
    for r in scored[:10]:
        res = r['results']
        acc = [k for k in names if res[k]['status'] == 'ok']
        rej = [k for k in names if res[k]['status'] == 'error']
        if r['classification'] == 'accept-vs-reject':
            txt = f"accepted by {', '.join(acc)}; rejected by {', '.join(rej)}"
        else:
            txt = f"all of {', '.join(acc)} accept but pixels differ" + (f' ({r["pixel_difference"]})' if r['pixel_difference'] else '') + (f"; rejected by {', '.join(rej)}" if rej else '')
        m = r['mutation']
        L.append(f"- `{r['rule']}` ({r['count']} inputs; {m.get('field')} {m.get('old')} -> {m.get('new')} in the witness's source): {txt}. [witness]({r['witness']})")
    L += ['', '## Crashes and timeouts', '']
    if public:
        L.append('Excluded from this view.')
    else:
        L.append(f"{s['anomaly_count']} inputs; per decoder: {s['crash_by_decoder']}. Inputs retained under `{fol}/anomalies/`; not investigated (handled separately).")
    L += ['', '## Run', '', f'`./run.sh --inputs 10000 --seed 731 --jobs 4` from `{ROOT}/{adapter.NAME}`; `python3 -I reproduce.py FILE` runs one file through every decoder.', '']
    return '\n'.join(L)


def reproduce(adapter, path):
    p = Path(path)
    res = invoke_all(adapter, p, 5)
    for k, v in res.items():
        print(f'{k:16} {cell(v)}' + (f"  16bit={v['rgba16_sha256'][:8]}" if v.get('rgba16_sha256') else '') + (f"  [{v.get('message', '')[:90]}]" if v['status'] != 'ok' else ''))
    for k, why in adapter.UNAVAILABLE.items():
        print(f'{k:16} unavailable: {why}')
    print('classification:', classify16(res))


def main(adapter):
    import argparse
    p = argparse.ArgumentParser()
    p.add_argument('--inputs', type=int, default=10000)
    p.add_argument('--seed', type=int, default=731)
    p.add_argument('--jobs', type=int, default=4)
    p.add_argument('--timeout', type=float, default=3.0)
    p.add_argument('--min-checks', type=int, default=10)
    p.add_argument('--patterns-per-rule', type=int, default=3)
    p.add_argument('--max-rows', type=int, default=90)
    p.add_argument('--reanalyse', action='store_true', help='rebuild summaries from saved results without re-running decoders')
    run(adapter, p.parse_args())
