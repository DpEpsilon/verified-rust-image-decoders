#!/usr/bin/env python3
"""Shared run loop for the JPEG and TIFF adapters: tolerance-aware comparison,
classification, provenance clustering, witness minimisation and rendering.

It reuses engine.invoke (subprocess sandbox) and engine.classify (exact class) without
modifying them. Lossy decoders differ by IDCT, chroma upsampling and colour conversion,
so every accepted output is also compared numerically: two outputs with the same
dimensions are *tolerance-equal* when the maximum absolute channel difference is <= TOL_MAX
and the mean absolute channel difference is <= TOL_MEAN. Tolerance-equality is made
transitive with union-find, so a class may contain pairs that only chain together.
"""
import collections
import concurrent.futures
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import sys
import threading
import time

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
LENIENCY = HERE.parent
sys.path.insert(0, str(LENIENCY))
import engine  # noqa: E402

BIN = Path('/tmp/claude-0/leniency-jt-bin')
TOL_MAX = 4      # max |a-b| per channel for "equal within IDCT tolerance"
TOL_MEAN = 1.0   # mean |a-b| per channel
NEAR_MAX, NEAR_MEAN = 48, 4.0  # "near": upsampling / colour-conversion class
DUMP_LIMIT = 4 * 1_000_000     # bytes of RGBA compared numerically; larger outputs exact only
CLASSES = ('all-accept-identical-pixels', 'all-accept-within-tolerance', 'all-accept-orientation-differs',
           'accept-different-pixels', 'accept-vs-reject', 'all-reject', 'crash-or-timeout')
KEEP = ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'alpha_present', 'orig_color_type',
        'orientation', 'orientation_applied', 'oriented_width', 'oriented_height', 'oriented_sha256',
        'warnings', 'first_warning', 'pages', 'bit_depth', 'components')


class Adapter:
    """Format description: subclasses set fmt, ext, runners, families, unavailable, generate()."""
    fmt = ''
    ext = ''
    runners = {}       # name -> (executable, args)
    families = {}      # family -> [runner names]; one independent vote per family
    unavailable = {}   # candidate -> reason
    findings = []      # ten observations, written after inspecting a run
    public_findings = None
    notes = []

    def folder(self):
        return Path(f'/tmp/claude-0/leniency-{self.fmt}')

    def generate(self, limit, seed):
        raise NotImplementedError


def compact(v):
    out = {k: v[k] for k in KEEP if k in v}
    if 'message' in out:
        out['message'] = str(out['message'])[:160]
    for k in ('rgba_sha256', 'oriented_sha256'):
        if k in out:
            out[k] = out[k][:16]
    return out


def invoke_all(ad, path, timeout, dumpdir=None):
    res, dumps = {}, {}
    for name, runner in ad.runners.items():
        dump = dumpdir / f'{name}.rgba' if dumpdir else None
        if dump:
            dump.unlink(missing_ok=True)
        r = engine.invoke(path, runner, timeout, dump)
        res[name] = r
        if dump and r.get('status') == 'ok' and dump.exists():
            dumps[name] = dump.read_bytes()
            dump.unlink()
    return res, dumps


def diff(a, b):
    """(max, mean) absolute channel difference of two equal-length RGBA buffers."""
    if len(a) != len(b) or not a:
        return None
    mx = tot = 0
    for x, y in zip(a, b):
        d = x - y if x > y else y - x
        tot += d
        if d > mx:
            mx = d
    return mx, round(tot / len(a), 3)


def transform(buf, w, h, o):
    """Apply EXIF orientation o (1..8) to top-first RGBA; returns (buf, w, h)."""
    if o in (1, None) or not buf:
        return buf, w, h
    px = [buf[i:i + 4] for i in range(0, len(buf), 4)]
    def at(x, y): return px[y * w + x]
    if o == 2: out = [at(w - 1 - x, y) for y in range(h) for x in range(w)]; return b''.join(out), w, h
    if o == 3: return b''.join(reversed(px)), w, h
    if o == 4: out = [at(x, h - 1 - y) for y in range(h) for x in range(w)]; return b''.join(out), w, h
    # 5-8 swap axes: output row index = source column x, output column index = source row y.
    if o == 5: out = [at(x, y) for x in range(w) for y in range(h)]; return b''.join(out), h, w
    if o == 6: out = [at(x, h - 1 - y) for x in range(w) for y in range(h)]; return b''.join(out), h, w
    if o == 7: out = [at(w - 1 - x, h - 1 - y) for x in range(w) for y in range(h)]; return b''.join(out), h, w
    if o == 8: out = [at(w - 1 - x, y) for x in range(w) for y in range(h)]; return b''.join(out), h, w
    return buf, w, h


def tol_equal(d):
    return d is not None and d[0] <= TOL_MAX and d[1] <= TOL_MEAN


def compare(results, dumps):
    """Exact groups, tolerance partition, pairwise metrics and a difference diagnosis."""
    ok = [k for k, v in results.items() if v.get('status') == 'ok']
    exact = {}
    for k in ok:
        exact.setdefault((results[k]['width'], results[k]['height'], results[k]['rgba_sha256']), []).append(k)
    reps = [cols[0] for cols in exact.values()]
    parent = {r: r for r in reps}
    def find(r):
        while parent[r] != r:
            r = parent[r]
        return r
    pairs = []
    for i, a in enumerate(reps):
        for b in reps[i + 1:]:
            va, vb = results[a], results[b]
            d = None
            if (va['width'], va['height']) == (vb['width'], vb['height']) and a in dumps and b in dumps:
                d = diff(dumps[a], dumps[b])
            pairs.append([a, b, d[0] if d else None, d[1] if d else None, describe(va, vb, dumps.get(a), dumps.get(b), d)])
            if tol_equal(d):
                parent[find(a)] = find(b)
    group_of = {}
    for cols in exact.values():
        for k in cols:
            group_of[k] = find(cols[0])
    order = []
    for k in results:
        if k in group_of and group_of[k] not in order:
            order.append(group_of[k])
    groups = {k: order.index(group_of[k]) for k in group_of}
    return groups, len(exact), pairs


def describe(va, vb, a, b, d):
    if (va['width'], va['height']) != (vb['width'], vb['height']):
        if a and b and (va['width'], va['height']) == (vb['height'], vb['width']) and len(a) <= 65536 * 4:
            for o in (5, 6, 7, 8):
                t, _, _ = transform(a, va['width'], va['height'], o)
                if tol_equal(diff(t, b)):
                    return f'orientation-{o}'
        return 'dimensions'
    if d is None:
        return 'not-compared'
    if tol_equal(d):
        return 'exact' if d[0] == 0 else 'within-tolerance'
    if all(a[i:i + 3] == b[i:i + 3] for i in range(0, len(a), 4)):
        return 'alpha-only'
    if len(a) <= 65536 * 4:
        for o in (2, 3, 4):
            t, _, _ = transform(a, va['width'], va['height'], o)
            if tol_equal(diff(t, b)):
                return f'orientation-{o}'
        if va['width'] == va['height']:
            for o in (5, 6, 7, 8):
                t, _, _ = transform(a, va['width'], va['height'], o)
                if tol_equal(diff(t, b)):
                    return f'orientation-{o}'
        sw = bytes(x for i in range(0, len(a), 4) for x in (a[i + 2], a[i + 1], a[i], a[i + 3]))
        if tol_equal(diff(sw, b)):
            return 'red-blue-swap'
    if d[0] <= NEAR_MAX and d[1] <= NEAR_MEAN:
        return 'near (upsampling/colour conversion)'
    return 'large'


def classify(results, groups, pairs):
    statuses = [v.get('status') for v in results.values()]
    if 'crash' in statuses or 'timeout' in statuses:
        return 'crash-or-timeout'
    ok = [k for k in results if statuses[list(results).index(k)] == 'ok']
    if not ok:
        return 'all-reject'
    if len(ok) < len(statuses):
        return 'accept-vs-reject'
    if engine.classify(results) == 'all-accept-identical-pixels':
        return 'all-accept-identical-pixels'
    if len(set(groups.values())) == 1:
        return 'all-accept-within-tolerance'
    kinds = {p[4] for p in pairs}
    if all(k.startswith('orientation') or k in ('exact', 'within-tolerance') for k in kinds):
        return 'all-accept-orientation-differs'
    return 'accept-different-pixels'


def relation(results, groups):
    """Status per column plus the tolerance partition; ignores actual pixel values."""
    return tuple((k, v.get('status'), groups.get(k)) for k, v in results.items())


def evaluate(ad, path, timeout, dumpdir):
    res, dumps = invoke_all(ad, path, timeout, dumpdir)
    groups, n_exact, pairs = compare(res, dumps)
    return res, groups, n_exact, pairs, classify(res, groups, pairs)


def minimize(ad, data, target, timeout, folder, checks):
    """Bounded delta debugging preserving per-column status and tolerance partition."""
    tmp = folder / f'minimize.{ad.ext}'
    dumpdir = folder / 'dumps-min'
    dumpdir.mkdir(exist_ok=True)
    used, n = 0, 2
    while len(data) > 1 and used < checks:
        step = max(1, (len(data) + n - 1) // n)
        changed = False
        for start in range(0, len(data), step):
            cand = data[:start] + data[start + step:]
            if not cand:
                continue
            tmp.write_bytes(cand)
            used += 1
            res, groups, _, _, _ = evaluate(ad, tmp, timeout, dumpdir)
            if relation(res, groups) == target:
                data, changed, n = cand, True, max(2, n - 1)
                break
            if used >= checks:
                break
        if not changed:
            if n >= len(data):
                break
            n = min(len(data), n * 2)
    tmp.unlink(missing_ok=True)
    return data, used


def run(ad, args):
    folder = ad.folder()
    inputs = folder / 'inputs'
    if inputs.exists():
        shutil.rmtree(inputs)
    inputs.mkdir(parents=True)
    t0 = time.monotonic()
    items, n_base, n_seeds = ad.generate(args.inputs, args.seed)
    gen_seconds = round(time.monotonic() - t0, 1)
    with (folder / 'manifest.jsonl').open('w') as f:
        for i, (b, m) in enumerate(items):
            (inputs / f'{i:05}.{ad.ext}').write_bytes(b)
            f.write(json.dumps({'id': i, 'length': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **m}) + '\n')
    local = threading.local()
    def job(i):
        if not hasattr(local, 'dir'):
            local.dir = folder / f'dumps-{threading.get_ident()}'
            local.dir.mkdir(exist_ok=True)
        res, groups, n_exact, pairs, cls = evaluate(ad, inputs / f'{i:05}.{ad.ext}', args.timeout, local.dir)
        return i, res, groups, n_exact, pairs, cls
    started = time.monotonic()
    counts, exact_counts = collections.Counter(), collections.Counter()
    baselines, rules, anomalies = {}, collections.defaultdict(list), []
    changed_count = 0
    per_decoder = collections.defaultdict(collections.Counter)
    with (folder / 'results.jsonl').open('w') as out, concurrent.futures.ThreadPoolExecutor(args.jobs) as pool:
        for i, res, groups, n_exact, pairs, cls in pool.map(job, range(len(items))):
            m = items[i][1]
            counts[cls] += 1
            exact_counts[engine.classify(res)] += 1
            for k, v in res.items():
                per_decoder[k][v.get('status')] += 1
            rel = relation(res, groups)
            if m['source'] != 'mutation':
                baselines[m['name']] = rel
            changed = m['source'] == 'mutation' and rel != baselines.get(m['seed'])
            changed_count += changed
            if cls == 'crash-or-timeout':
                anomalies.append((i, m, res))
            elif cls not in ('all-reject', 'all-accept-identical-pixels', 'all-accept-within-tolerance') and (m['source'] != 'mutation' or changed):
                rules[m['rule']].append((i, rel, cls))
            out.write(json.dumps({'id': i, 'rule': m['rule'], 'classification': cls, 'exact_classification': engine.classify(res),
                                  'groups': groups, 'exact_groups': n_exact, 'pairs': pairs, 'changed': changed,
                                  'results': {k: compact(v) for k, v in res.items()}}, separators=(',', ':')) + '\n')
            if (i + 1) % 1000 == 0:
                print(f'{ad.fmt} {i + 1}/{len(items)} {time.monotonic() - started:.0f}s', flush=True)
    elapsed = round(time.monotonic() - started, 1)
    for d in folder.glob('dumps-*'):
        shutil.rmtree(d, ignore_errors=True)
    # Anomalies: inputs are copied for the coordinator and only counted here.
    adir = folder / 'anomalies'
    if adir.exists():
        shutil.rmtree(adir)
    adir.mkdir()
    for i, _, _ in anomalies:
        shutil.copyfile(inputs / f'{i:05}.{ad.ext}', adir / f'{i:05}.{ad.ext}')
    by_id = {}
    for line in (folder / 'results.jsonl').open():
        r = json.loads(line)
        by_id[r['id']] = r
    rows = build_rows(ad, items, rules, by_id, args)
    summary = {
        'format': ad.fmt, 'seed': args.seed, 'inputs': len(items), 'base_inputs': n_base, 'seed_variants': n_seeds,
        'generation_seconds': gen_seconds, 'elapsed_seconds': elapsed, 'jobs': args.jobs, 'timeout_seconds': args.timeout,
        'tolerance': {'max_abs': TOL_MAX, 'mean_abs': TOL_MEAN, 'near_max': NEAR_MAX, 'near_mean': NEAR_MEAN},
        'decoders': list(ad.runners), 'families': ad.families, 'unavailable': ad.unavailable,
        'counts': {c: counts.get(c, 0) for c in CLASSES}, 'exact_counts': dict(exact_counts),
        'changed_mutations': changed_count, 'per_decoder_status': {k: dict(v) for k, v in per_decoder.items()},
        'anomaly_count': len(anomalies),
        'anomaly_by_decoder': dict(collections.Counter(k for _, _, r in anomalies for k, v in r.items() if v.get('status') in ('crash', 'timeout'))),
        'anomaly_ids': [i for i, _, _ in anomalies][:200],
        'rows': rows, 'pixel_rows': pixel_rows(ad, items, by_id, args),
        'seed_table': seed_table(ad, items, by_id), 'orientation_table': orientation_table(ad, items, by_id),
        'acceptance_by_rule': acceptance_by_rule(ad, items, by_id),
        'manifest': str(folder / 'manifest.jsonl'), 'raw_results': str(folder / 'results.jsonl'),
    }
    (HERE.parent / ad.fmt / 'summary.json').write_text(json.dumps(summary, indent=1) + '\n')
    finish(ad, summary)
    print(json.dumps({'format': ad.fmt, 'inputs': len(items), 'counts': summary['counts'], 'elapsed_seconds': elapsed,
                      'rows': len(rows), 'anomalies': len(anomalies)}, indent=1))


def finish(ad, summary):
    adapter = HERE.parent / ad.fmt
    render(ad, summary, adapter / 'LENIENCY.md')
    pub = public(summary)
    (adapter / 'summary-public.json').write_text(json.dumps(pub, indent=1) + '\n')
    render(ad, pub, adapter / 'LENIENCY-PUBLIC.md')


def build_rows(ad, items, rules, by_id, args):
    """One row per provenance rule: the modal relation among changed inputs, minimised."""
    adapter = HERE.parent / ad.fmt
    wdir = adapter / 'witnesses'
    if wdir.exists():
        shutil.rmtree(wdir)
    wdir.mkdir()
    rows = []
    ranked = sorted(rules.items(), key=lambda kv: -len(kv[1]))
    for n, (rule, entries) in enumerate(ranked):
        patterns = collections.defaultdict(list)
        for i, rel, cls in entries:
            patterns[rel].append(i)
        modal = max(patterns.values(), key=len)
        i = min(modal, key=lambda j: len(items[j][0]))
        r = by_id[i]
        row = {'rule': rule, 'count': len(entries), 'patterns': len(patterns), 'id': i, 'classification': r['classification'],
               'mutation': items[i][1], 'results': r['results'], 'groups': r['groups'], 'pairs': r['pairs'],
               'classes': dict(collections.Counter(c for _, _, c in entries))}
        if n < args.max_witness_rows:
            target = relation(r['results'], r['groups'])
            shrunk, used = minimize(ad, items[i][0], target, args.timeout, ad.folder(), args.min_checks)
            name = f"{len(rows) + 1:02}-{rule.replace(':', '-').replace('/', '-')[:40]}.{ad.ext}"
            (wdir / name).write_bytes(shrunk)
            row.update(witness='witnesses/' + name, original_length=len(items[i][0]), witness_length=len(shrunk), min_checks=used)
        rows.append(row)
    return rows


def pixel_rows(ad, items, by_id, args):
    """Same acceptance, different pixels: cluster by (rule, diagnosis) beyond the tolerance."""
    grouped = collections.defaultdict(list)
    for i, r in by_id.items():
        if r['classification'] not in ('accept-different-pixels', 'all-accept-orientation-differs', 'accept-vs-reject'):
            continue
        kinds = sorted({p[4] for p in r['pairs'] if p[4] not in ('exact', 'within-tolerance')})
        if not kinds:
            continue
        split = tuple(sorted(tuple(sorted(k for k, g in r['groups'].items() if g == x)) for x in set(r['groups'].values())))
        grouped[(r['rule'], '+'.join(kinds), split)].append(i)
    out = []
    for (rule, kind, split), ids in sorted(grouped.items(), key=lambda kv: -len(kv[1]))[:30]:
        i = min(ids, key=lambda j: len(items[j][0]))
        r = by_id[i]
        out.append({'rule': rule, 'difference': kind, 'split': [list(s) for s in split], 'count': len(ids), 'id': i,
                    'mutation': items[i][1], 'pairs': r['pairs'], 'results': r['results']})
    wdir = HERE.parent / ad.fmt / 'witnesses'
    for n, row in enumerate(out[:12]):
        name = f'pixel-{n + 1:02}.{ad.ext}'
        (wdir / name).write_bytes(items[row['id']][0])
        row['witness'] = 'witnesses/' + name
    return out


def seed_table(ad, items, by_id):
    """Unmutated synthetic seeds: acceptance and tolerance partition per decoder."""
    out = []
    for i, (b, m) in enumerate(items):
        if m['source'] == 'mutation':
            continue
        r = by_id[i]
        worst = max((p[2] for p in r['pairs'] if p[2] is not None), default=0)
        out.append({'id': i, 'name': m['name'], 'classification': r['classification'], 'max_abs': worst,
                    'status': {k: (v['status'] if v['status'] != 'error' else 'reject') for k, v in r['results'].items()},
                    'groups': r['groups'], 'kinds': sorted({p[4] for p in r['pairs']})})
    return out


def orientation_table(ad, items, by_id):
    rows = []
    for i, (b, m) in enumerate(items):
        if m['source'] == 'mutation' or not m.get('orientation') or m['orientation'] == 1:
            continue
        r = by_id[i]['results']
        rows.append({'id': i, 'name': m['name'], 'tag': m['orientation'],
                     'decoders': {k: {'status': v['status'], 'reported': v.get('orientation'), 'applied': v.get('orientation_applied'),
                                      'size': f"{v.get('width')}x{v.get('height')}",
                                      'oriented_size': f"{v.get('oriented_width')}x{v.get('oriented_height')}" if 'oriented_width' in v else None}
                                  for k, v in r.items()}})
    return rows


def acceptance_by_rule(ad, items, by_id):
    """Per rule: number of inputs each decoder accepted, for every mutation rule."""
    acc = collections.defaultdict(lambda: collections.Counter())
    tot = collections.Counter()
    for i, r in by_id.items():
        rule = r['rule']
        tot[rule] += 1
        for k, v in r['results'].items():
            acc[rule][k] += v['status'] == 'ok'
    return {rule: {'inputs': tot[rule], **dict(acc[rule])} for rule in sorted(tot)}


def public(summary):
    """All inputs are synthetic; the public view drops crash/timeout detail and local paths."""
    s = json.loads(json.dumps(summary))
    s['public_subset'] = True
    for k in ('anomaly_ids', 'manifest', 'raw_results'):
        s.pop(k, None)
    s['rows'] = [r for r in s['rows'] if r['classification'] != 'crash-or-timeout']
    s['pixel_rows'] = [r for r in s['pixel_rows'] if all(v['status'] in ('ok', 'error') for v in r['results'].values())]
    for r in s['rows'] + s['pixel_rows']:
        for v in r['results'].values():
            if 'message' in v:
                v['message'] = re.sub(r'(?:/[^/\s"\u201c\u201d:]+)+/', '<path>/', v['message'])
    return s


def cell(v):
    st = v['status']
    if st == 'ok':
        return f"{v['width']}×{v['height']} `{v['rgba_sha256'][:6]}`"
    if st == 'error':
        return 'reject `' + str(v.get('error_class', '?')).replace('|', '/')[:24] + '`'
    return st


def render(ad, s, path):
    names = s['decoders']
    pub = s.get('public_subset')
    L = [f'# {ad.fmt.upper()} leniency matrix', '',
         f"Seed {s['seed']}: {s['inputs']} synthetic inputs ({s['base_inputs']} unmutated seeds, {s['seed_variants']} seeds mutated, "
         f"{s['changed_mutations']} mutations changed the acceptance/tolerance relation relative to their seed). "
         f"The comparison pass took {s['elapsed_seconds']} s with {s['jobs']} worker threads and a {s['timeout_seconds']} s per-decoder timeout; "
         f"input generation took {s['generation_seconds']} s.", '']
    if pub:
        L += ['Public view: every input is generated by the harness (no third-party image bytes); crash and timeout rows are '
              'excluded and only counted.', '']
    fam = '; '.join(f"**{f}**: {', '.join(c)}" for f, c in s['families'].items())
    L += [f'Decoder columns and independent families (one vote per family): {fam}.', '']
    if s['unavailable']:
        L += ['Unavailable candidates: ' + '; '.join(f'{k}: {v}' for k, v in s['unavailable'].items()) + '.', '']
    L += [f"Pixels are hashed as straight top-first RGBA8 before any orientation. Two outputs are **equal within IDCT tolerance** when they have the "
          f"same dimensions, max absolute channel difference ≤ {s['tolerance']['max_abs']} and mean ≤ {s['tolerance']['mean_abs']}; "
          f"\"near\" means max ≤ {s['tolerance']['near_max']} and mean ≤ {s['tolerance']['near_mean']} (chroma upsampling or colour conversion). "
          'Tolerance-equality is closed transitively (union-find).', '',
          '| Classification | Inputs |', '| --- | ---: |']
    for c in CLASSES:
        L.append(f"| {c} | {s['counts'].get(c, 0)} |")
    L += ['', 'Exact-hash classification (engine.classify, no tolerance): ' +
          ', '.join(f'{k} {v}' for k, v in sorted(s['exact_counts'].items())) + '.', '',
          '| Decoder | ok | error | crash | timeout |', '| --- | ---: | ---: | ---: | ---: |']
    for k in names:
        d = s['per_decoder_status'].get(k, {})
        L.append(f"| {k} | {d.get('ok', 0)} | {d.get('error', 0)} | {d.get('crash', 0)} | {d.get('timeout', 0)} |")
    L += ['', '## Ten significant observations', '']
    for f in (s.get('public_findings') or ad.public_findings or ad.findings) if pub else ad.findings:
        L.append('- ' + f)
    L += ['', '## Seeds', '', 'Unmutated synthetic seeds. Group numbers are the tolerance partition (same number = equal within tolerance).', '',
          '| Seed | Class | max abs | ' + ' | '.join(names) + ' |', '| --- | --- | ---: | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['seed_table']:
        cells = [(f"g{r['groups'][k]}" if r['status'][k] == 'ok' else r['status'][k]) for k in names]
        L.append(f"| {r['name']} | {r['classification']} | {r['max_abs']} | " + ' | '.join(cells) + ' |')
    L += ['', '## Representative matrix by provenance', '',
          'One row per mutation rule: the most frequent status/tolerance-partition pattern among *changed* mutations from that rule '
          '(count = changed inputs; patterns = distinct patterns). Witnesses are delta-minimised while preserving every column\'s status and '
          'the tolerance partition; minimisation is bounded, not globally minimal.', '',
          '| Rule (changed; patterns) | Class | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for r in s['rows']:
        w = f"[#{r['id']}]({r['witness']})" if r.get('witness') else f"#{r['id']}"
        L.append(f"| {r['rule']} ({r['count']}; {r['patterns']}) | {r['classification']} | {w} | " + ' | '.join(cell(r['results'][k]) for k in names) + ' |')
    L += ['', '## Same acceptance, different pixels', '',
          'Clustered by rule, diagnosis and tolerance split. Diagnoses: `dimensions`, `orientation-N` (an EXIF transform maps one output onto the other '
          'within tolerance), `alpha-only`, `red-blue-swap`, `near (...)`, `large`.', '',
          '| Rule | Diagnosis | Split | Inputs | Witness |', '| --- | --- | --- | ---: | --- |']
    for r in s['pixel_rows'][:30]:
        w = f"[#{r['id']}]({r['witness']})" if r.get('witness') else f"#{r['id']}"
        L.append(f"| {r['rule']} | {r['difference']} | {' / '.join(','.join(x) for x in r['split'])} | {r['count']} | {w} |")
    if s['orientation_table']:
        L += ['', '## Orientation handling', '', 'Seeds carrying an orientation tag ≠ 1. `reported` is what each decoder exposes; '
              'no column rotates its primary output except libtiff\'s RGBA interface (`applied`).', '',
              '| Seed | Tag | ' + ' | '.join(names) + ' |', '| --- | ---: | ' + ' | '.join(['---'] * len(names)) + ' |']
        for r in s['orientation_table']:
            cells = []
            for k in names:
                d = r['decoders'][k]
                cells.append('reject' if d['status'] != 'ok' else f"{d['reported']}{' (applied)' if d['applied'] else ''} {d['size']}" + (f" → {d['oriented_size']}" if d['oriented_size'] else ''))
            L.append(f"| {r['name']} | {r['tag']} | " + ' | '.join(cells) + ' |')
    L += ['', '## Acceptance by rule', '', '| Rule | Inputs | ' + ' | '.join(names) + ' |', '| --- | ---: | ' + ' | '.join(['---:'] * len(names)) + ' |']
    for rule, d in s['acceptance_by_rule'].items():
        L.append(f"| {rule} | {d['inputs']} | " + ' | '.join(str(d.get(k, 0)) for k in names) + ' |')
    L += ['', '## Crashes and timeouts', '',
          f"{s['anomaly_count']} inputs made at least one decoder crash or time out (" +
          (', '.join(f'{k}: {v}' for k, v in sorted(s['anomaly_by_decoder'].items())) or 'none') +
          f"). Their inputs are kept under `/tmp/claude-0/leniency-{ad.fmt}/anomalies/` for the coordinator; causes were not investigated.", '']
    for n in ad.notes:
        L.append(n)
    L += ['', '## Run', '', f'`./run.sh --inputs 10000 --seed 731 --jobs 4` from `leniency/{ad.fmt}/`; `python3 reproduce.py FILE` runs every column on one file. '
          f'Per-input JSONL and the manifest are written to `/tmp/claude-0/leniency-{ad.fmt}/`.', '']
    path.write_text('\n'.join(L))


def main(ad):
    import argparse
    p = argparse.ArgumentParser()
    p.add_argument('--inputs', type=int, default=10000)
    p.add_argument('--seed', type=int, default=731)
    p.add_argument('--jobs', type=int, default=4)
    p.add_argument('--timeout', type=float, default=5.0)
    p.add_argument('--min-checks', type=int, default=24)
    p.add_argument('--max-witness-rows', type=int, default=40)
    p.add_argument('--render-only', action='store_true', help='re-render markdown/public files from summary.json')
    a = p.parse_args()
    if a.render_only:
        finish(ad, json.loads((HERE.parent / ad.fmt / 'summary.json').read_text()))
    else:
        run(ad, a)


def reproduce(ad, argv):
    if len(argv) != 2:
        raise SystemExit(f'usage: python3 reproduce.py FILE.{ad.ext}')
    d = Path('/tmp/claude-0') / f'repro-{ad.fmt}-{os.getpid()}'
    d.mkdir(parents=True, exist_ok=True)
    res, groups, n_exact, pairs, cls = evaluate(ad, Path(argv[1]).resolve(), 10, d)
    shutil.rmtree(d, ignore_errors=True)
    for k, v in res.items():
        print(json.dumps({'decoder': k, 'tolerance_group': groups.get(k), **compact(v)}, sort_keys=True))
    print(json.dumps({'classification': cls, 'exact_groups': n_exact, 'pairs': pairs}))
