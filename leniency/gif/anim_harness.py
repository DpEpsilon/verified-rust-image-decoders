#!/usr/bin/env python3
"""Shared differential harness for the GIF and PNG/APNG adapters.

Each format module (gif/harness.py, png/harness.py) supplies a `Format` object:
runners, implementation families, a seeded generator yielding (bytes, provenance)
and hand-written findings. This module runs every input through every runner as an
isolated subprocess via `engine.invoke`, classifies with `engine.classify` (first
displayed frame) plus an animation comparison, clusters changed mutations by rule,
minimises witnesses, and renders LENIENCY.md / summary.json and their public
variants. Inputs, manifests and per-input results live in /tmp/claude-0/leniency-<fmt>/.
"""
import collections
import concurrent.futures
import hashlib
import json
from pathlib import Path
import shutil
import sys
import time

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(ROOT))
import engine  # noqa: E402

BIN = Path('/tmp/claude-0/leniency-gifpng-bin')
KEEP = ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'alpha_present', 'orig_color_type',
        'selection', 'frames', 'frames_capped', 'seq_sha256', 'frame_info', 'rgba16_sha256', 'tail_error', 'loop_period')
CLASSES = ('all-accept-identical-pixels', 'accept-different-pixels', 'accept-vs-reject', 'all-reject', 'crash-or-timeout')


class Format:
    def __init__(self, name, ext, runners, families, animating, generate, findings, notes, unavailable, reference, observe_only=()):
        self.name, self.ext, self.runners, self.families = name, ext, runners, families
        self.animating, self.generate, self.findings, self.notes = animating, generate, findings, notes
        self.unavailable, self.reference = unavailable, reference
        self.observe_only = set(observe_only)
        self.folder = Path('/tmp/claude-0/leniency-' + name)
        self.adapter = ROOT / name

    def voting(self, results):
        return {k: v for k, v in results.items() if k not in self.observe_only}

    def invoke(self, path, name, timeout, dump=None):
        return engine.invoke(path, self.runners[name], timeout, dump)

    def run_all(self, path, timeout, names=None):
        return {n: self.invoke(path, n, timeout) for n in (names or self.runners)}


def compact(results):
    out = {}
    for k, v in results.items():
        c = {x: v[x] for x in KEEP if x in v}
        if 'message' in c:
            c['message'] = str(c['message'])[:160]
        out[k] = c
    return out


def sig(results):
    return tuple((k, v['status'], v.get('width'), v.get('height'), v.get('rgba_sha256')) for k, v in results.items())


def anim_key(v):
    return (v.get('width'), v.get('height'), v.get('frames'), v.get('seq_sha256'))


def animation_class(fmt, results):
    """Among accepting animation-capable runners: do frame count or composited sequence differ?"""
    good = [results[n] for n in fmt.animating if results[n]['status'] == 'ok' and n not in fmt.observe_only]
    if len(good) < 2:
        return 'n/a'
    if len({(v['width'], v['height']) for v in good}) > 1:
        return 'canvas-differs'
    if len({v.get('frames') for v in good}) > 1:
        return 'frame-count-differs'
    if len({v.get('seq_sha256') for v in good}) > 1:
        return 'sequence-pixels-differ'
    return 'identical'


def family_votes(fmt, results):
    """One vote per independent implementation family: its first listed runner."""
    seen = {}
    for name, fam in fmt.families.items():
        if fam not in seen and name in results and name not in fmt.observe_only:
            seen[fam] = results[name]
    return seen


def minimize(fmt, original, target, names, timeout, folder, checks):
    """Bounded delta debugging that preserves every runner's status, dimensions and first-frame hash."""
    data, used, n, tmp = original, 0, 2, folder / ('minimize.' + fmt.ext)
    while len(data) > 1 and used < checks:
        step = max(1, (len(data) + n - 1) // n)
        changed = False
        for start in range(0, len(data), step):
            cand = data[:start] + data[start + step:]
            if not cand:
                continue
            tmp.write_bytes(cand)
            used += 1
            if sig(fmt.run_all(tmp, timeout, names)) == target:
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


def pixel_difference(fmt, path, results, timeout, folder):
    """Describe how accepted first frames differ, from raw straight-RGBA dumps."""
    good = [k for k, v in results.items() if v['status'] == 'ok']
    if len(good) < 2:
        return ''
    if len({(results[k]['width'], results[k]['height']) for k in good}) > 1:
        return 'dimensions'
    groups = collections.defaultdict(list)
    for k in good:
        groups[results[k]['rgba_sha256']].append(k)
    reps = [v[0] for v in sorted(groups.values(), key=len, reverse=True)]
    dumps = []
    try:
        for k in reps[:2]:
            p = folder / f'pixel-{k}.rgba'
            if fmt.invoke(path, k, timeout, p)['status'] != 'ok' or not p.exists():
                return 'rgba-dump-unavailable'
            dumps.append(p.read_bytes())
            p.unlink()
    except OSError:
        return 'rgba-dump-unavailable'
    a, b = dumps
    if len(a) != len(b):
        return 'output-length'
    rgb = sum(a[i:i + 3] != b[i:i + 3] for i in range(0, len(a), 4))
    alpha = sum(a[i + 3] != b[i + 3] for i in range(0, len(a), 4))
    kind = 'alpha-only' if not rgb else ('rgb-only' if not alpha else 'rgb-and-alpha')
    return f'{kind}: {max(rgb, alpha)} of {len(a)//4} px ({"+".join(groups[results[reps[0]]["rgba_sha256"]][:3])} vs {"+".join(groups[results[reps[1]]["rgba_sha256"]][:3])})'


def run(fmt, args):
    folder = fmt.folder
    inputs = folder / 'inputs'
    if inputs.exists():
        shutil.rmtree(inputs)
    inputs.mkdir(parents=True)
    items, base, seeds = fmt.generate(args.inputs, args.seed)
    names = list(fmt.runners)
    manifest, output = folder / 'manifest.jsonl', folder / 'results.jsonl'
    with manifest.open('w') as f:
        for i, (b, m) in enumerate(items):
            (inputs / f'{i:05}.{fmt.ext}').write_bytes(b)
            f.write(json.dumps({'id': i, 'length': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **m}) + '\n')
    started = time.monotonic()

    def job(item):
        i, (_, m) = item
        return i, m, fmt.run_all(inputs / f'{i:05}.{fmt.ext}', args.timeout)

    with output.open('w') as out, concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for i, m, result in pool.map(job, enumerate(items)):
            row = {'id': i, 'rule': m['rule'], 'seed': m.get('seed', m.get('name')), 'kind': m.get('kind'),
                   'source': m['source'], 'classification': engine.classify(fmt.voting(result)),
                   'animation': animation_class(fmt, result), 'results': compact(result)}
            out.write(json.dumps(row, separators=(',', ':')) + '\n')
            if (i + 1) % 1000 == 0:
                print(f'{fmt.name} {i + 1}/{len(items)} {time.monotonic() - started:.0f}s', flush=True)
    elapsed = round(time.monotonic() - started, 1)
    finish(fmt, args, base, seeds, elapsed)


def finish(fmt, args, base, seeds, elapsed):
    summary = analyse(fmt, args, base, seeds, elapsed)
    summary['jobs'] = args.jobs
    (fmt.adapter / 'summary.json').write_text(json.dumps(summary, indent=1) + '\n')
    render(fmt, summary, 'LENIENCY.md')
    pub = public_subset(fmt, summary)
    (fmt.adapter / 'summary-public.json').write_text(json.dumps(pub, indent=1) + '\n')
    render(fmt, pub, 'LENIENCY-PUBLIC.md')
    print(json.dumps({'format': fmt.name, 'inputs': summary['inputs'], 'counts': summary['counts'],
                      'elapsed_seconds': elapsed, 'rows': len(summary['rows']), 'anomalies': summary['anomaly_count']}, indent=1))


def load(fmt):
    meta = {m['id']: m for m in map(json.loads, (fmt.folder / 'manifest.jsonl').open())}
    rows = [json.loads(x) for x in (fmt.folder / 'results.jsonl').open()]
    return meta, rows


def tallies(fmt, rows):
    """Per-decoder acceptance, independent-majority leniency/strictness and pixel outliers."""
    names = list(fmt.runners)
    per = {n: collections.Counter() for n in names}
    by_rule = collections.defaultdict(lambda: collections.defaultdict(int))
    rule_total = collections.Counter()
    for r in rows:
        res = r['results']
        rule_total[r['rule']] += 1
        votes = family_votes(fmt, res)
        clean = [v for v in votes.values() if v['status'] in ('ok', 'error')]
        acc = sum(v['status'] == 'ok' for v in clean)
        majority_ok = acc * 2 > len(clean)
        majority_err = (len(clean) - acc) * 2 > len(clean)
        hashes = collections.Counter((v['width'], v['height'], v['rgba_sha256']) for v in clean if v['status'] == 'ok')
        top = hashes.most_common(1)[0] if hashes else None
        for n in names:
            v = res[n]
            per[n][v['status']] += 1
            by_rule[r['rule']][n] += v['status'] == 'ok'
            if v['status'] == 'ok' and majority_err:
                per[n]['lenient'] += 1
            if v['status'] == 'error' and majority_ok:
                per[n]['strict'] += 1
            if v['status'] == 'ok' and top and top[1] * 2 > acc and (v['width'], v['height'], v['rgba_sha256']) != top[0]:
                per[n]['pixel-outlier'] += 1
            if v.get('tail_error'):
                per[n]['tail-error'] += 1
            if v['status'] == 'ok' and n in fmt.animating:
                ref = res.get(fmt.reference)
                if ref and ref['status'] == 'ok' and ref.get('frames', 0) > 1 and ref.get('frames') == v.get('frames'):
                    if [x[0] for x in ref.get('frame_info', [])] != [x[0] for x in v.get('frame_info', [])]:
                        per[n]['delay-differs-from-' + fmt.reference] += 1
    matrix = {rule: {'total': rule_total[rule], **{n: by_rule[rule][n] for n in names}} for rule in sorted(rule_total)}
    return {n: dict(c) for n, c in per.items()}, matrix


def analyse(fmt, args, base, seeds, elapsed):
    meta, rows = load(fmt)
    names = list(fmt.runners)
    counts = collections.Counter(r['classification'] for r in rows)
    anim = collections.Counter(r['animation'] for r in rows)
    baselines = {r['seed']: engine.relation_signature(fmt.voting(r['results'])) for r in rows if r['source'] != 'mutation'}
    banims = {r['seed']: r['animation'] for r in rows if r['source'] != 'mutation'}
    rules = collections.defaultdict(list)
    anim_rules = collections.defaultdict(list)
    changed = 0
    anomalies = []
    for r in rows:
        m = meta[r['id']]
        ch = m['source'] == 'mutation' and engine.relation_signature(fmt.voting(r['results'])) != baselines.get(m['seed'])
        changed += ch
        if r['classification'] == 'crash-or-timeout' or any(v['status'] in ('crash', 'timeout') for v in r['results'].values()):
            anomalies.append(r)
            continue
        if r['classification'] not in ('all-reject', 'all-accept-identical-pixels') and (m['source'] != 'mutation' or ch):
            rules[r['rule']].append(r)
        if r['animation'] not in ('identical', 'n/a') and (m['source'] != 'mutation' or banims.get(m['seed']) != r['animation']):
            anim_rules[r['rule']].append(r)
    adir = fmt.folder / 'anomalies'
    if adir.exists():
        shutil.rmtree(adir)
    adir.mkdir()
    for r in anomalies:
        shutil.copyfile(fmt.folder / 'inputs' / f"{r['id']:05}.{fmt.ext}", adir / f"{r['id']:05}.{fmt.ext}")
    wdir = fmt.adapter / 'witnesses'
    if wdir.exists():
        shutil.rmtree(wdir)
    wdir.mkdir()
    out_rows = []
    for rule, entries in sorted(rules.items(), key=lambda kv: -len(kv[1]))[:args.max_rows]:
        patterns = collections.defaultdict(list)
        for e in entries:
            patterns[sig(e['results'])].append(e)
        main = max(patterns.values(), key=len)
        e = min(main, key=lambda x: meta[x['id']]['length'])  # smallest input with the modal outcome
        src = fmt.folder / 'inputs' / f"{e['id']:05}.{fmt.ext}"
        shrunk, used = minimize(fmt, src.read_bytes(), sig(e['results']), names, args.timeout, fmt.folder, args.min_checks)
        w = wdir / f"{len(out_rows) + 1:02}-{rule.replace(':', '-').replace('/', '-')}.{fmt.ext}"
        w.write_bytes(shrunk)
        kinds = collections.Counter(x.get('kind') or meta[x['id']].get('kind') for x in entries).most_common(3)
        out_rows.append({'rule': rule, 'count': len(entries), 'patterns': len(patterns), 'id': e['id'],
                         'witness': 'witnesses/' + w.name, 'mutation': meta[e['id']], 'classification': e['classification'],
                         'seed_kinds': kinds, 'results': e['results'], 'original_length': src.stat().st_size,
                         'witness_length': len(shrunk), 'min_checks': used,
                         'pixel_difference': pixel_difference(fmt, w, e['results'], args.timeout, fmt.folder)
                         if e['classification'] == 'accept-different-pixels' else ''})
    pixel_groups = collections.defaultdict(list)
    for r in rows:
        if r['classification'] == 'accept-different-pixels' and r['id'] not in {a['id'] for a in anomalies}:
            pixel_groups[r['rule']].append(r)
    pixel_rows = []
    for rule, entries in sorted(pixel_groups.items(), key=lambda kv: -len(kv[1]))[:15]:
        e = min(entries, key=lambda x: meta[x['id']]['length'])
        w = wdir / f"pixel-{len(pixel_rows) + 1:02}-{rule.replace(':', '-').replace('/', '-')}.{fmt.ext}"
        shutil.copyfile(fmt.folder / 'inputs' / f"{e['id']:05}.{fmt.ext}", w)
        groups = collections.defaultdict(list)
        for n, v in e['results'].items():
            if v['status'] == 'ok':
                groups[(v['width'], v['height'], v['rgba_sha256'][:8])].append(n)
        pixel_rows.append({'rule': rule, 'count': len(entries), 'id': e['id'], 'witness': 'witnesses/' + w.name,
                           'difference': pixel_difference(fmt, w, e['results'], args.timeout, fmt.folder),
                           'groups': [[f'{k[0]}×{k[1]} {k[2]}', v] for k, v in groups.items()]})
    anim_rows = []
    for rule, entries in sorted(anim_rules.items(), key=lambda kv: -len(kv[1]))[:20]:
        e = min(entries, key=lambda x: meta[x['id']]['length'])
        src = fmt.folder / 'inputs' / f"{e['id']:05}.{fmt.ext}"
        w = wdir / f"anim-{len(anim_rows) + 1:02}-{rule.replace(':', '-').replace('/', '-')}.{fmt.ext}"
        shutil.copyfile(src, w)
        anim_rows.append({'rule': rule, 'count': len(entries), 'id': e['id'], 'witness': 'witnesses/' + w.name,
                          'animation': collections.Counter(x['animation'] for x in entries).most_common(),
                          'results': {n: {k: e['results'][n].get(k) for k in ('status', 'frames', 'seq_sha256', 'frame_info', 'error_class')}
                                      for n in fmt.animating}})
    anomaly_ids = {r['id'] for r in anomalies}
    per, matrix = tallies(fmt, [r for r in rows if r['id'] not in anomaly_ids])
    recheck = collections.Counter()
    for r in anomalies:  # serial re-run with a generous timeout: separates real crashes from load timeouts
        for k, v in r['results'].items():
            if v['status'] in ('crash', 'timeout'):
                again = fmt.invoke(fmt.folder / 'inputs' / f"{r['id']:05}.{fmt.ext}", k, max(20.0, args.timeout * 4))
                recheck[f"{k}:{v['status']}->{again['status']}"] += 1
    return {'format': fmt.name, 'seed': args.seed, 'inputs': len(rows), 'base_inputs': base, 'seed_variants': seeds,
            'counts': {c: counts.get(c, 0) for c in CLASSES}, 'animation_counts': dict(anim), 'changed_mutations': changed,
            'elapsed_seconds': elapsed, 'decoders': names, 'families': fmt.families, 'unavailable': fmt.unavailable,
            'per_decoder': per, 'rule_matrix': matrix, 'rows': out_rows, 'pixel_rows': pixel_rows, 'animation_rows': anim_rows,
            'anomaly_count': len(anomalies),
            'counts_without_anomalies': {c: n for c, n in collections.Counter(r['classification'] for r in rows if r['id'] not in anomaly_ids).items()},
            'anomaly_by_decoder': dict(collections.Counter(k + ':' + v['status'] for r in anomalies for k, v in r['results'].items()
                                                           if v['status'] in ('crash', 'timeout'))),
            'anomaly_recheck_serial': dict(recheck), 'anomaly_ids': [r['id'] for r in anomalies][:200], 'manifest': str(fmt.folder / 'manifest.jsonl'),
            'raw_results': str(fmt.folder / 'results.jsonl')}


def public_subset(fmt, full):
    """All inputs are synthetic; the public view drops crash/timeout inputs and every anomaly reference."""
    s = json.loads(json.dumps(full))
    s['public_subset'] = True
    s['inputs'] = full['inputs'] - full['anomaly_count']
    s['counts'] = dict(full['counts_without_anomalies'])
    s['excluded_inputs'] = 'inputs on which any runner crashed or timed out'
    for k in ('anomaly_ids', 'anomaly_by_decoder', 'anomaly_recheck_serial', 'manifest', 'raw_results'):
        s.pop(k, None)
    s['anomaly_count'] = 'omitted'
    for v in s['per_decoder'].values():
        v.pop('crash', None)
        v.pop('timeout', None)
    return s


def render(fmt, s, filename):
    names = s['decoders']
    pub = s.get('public_subset')
    L = [f'# {fmt.name.upper()} leniency matrix', '',
         f"Seed {s['seed']}; {s['inputs']} synthetic inputs{' (crash/timeout inputs excluded)' if pub else ''}; "
         f"{s['base_inputs']} unmutated generator seeds; {s['seed_variants']} seeds received mutations; "
         f"{s['changed_mutations']} mutations changed the acceptance/pixel relation relative to their seed; "
         f"decoder pass {s['elapsed_seconds']} s (one CPU), witness minimisation afterwards.", '',
         'Classification (first displayed frame, top-first straight RGBA8) | Count', '--- | ---:']
    for c in CLASSES:
        if c in s['counts']:
            L.append(f"{c} | {s['counts'][c]}")
    L += ['', 'Animation comparison among accepting animation-capable runners (' + ', '.join(fmt.animating) + ') | Count', '--- | ---:']
    for k, v in sorted(s['animation_counts'].items()):
        L.append(f'{k} | {v}')
    L += ['', '## Decoders', '', fmt.notes, '', '| Runner | Family (one vote) | ok | error | lenient (accepts, independent majority rejects) | strict (rejects, majority accepts) | first-frame pixel outlier | tail error after pixels |',
          '| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |']
    for n in names:
        p = s['per_decoder'][n]
        L.append(f"| {n} | {s['families'][n]} | {p.get('ok', 0)} | {p.get('error', 0)} | {p.get('lenient', 0)} | {p.get('strict', 0)} | {p.get('pixel-outlier', 0)} | {p.get('tail-error', 0)} |")
    delays = {n: v for n in names for k, v in s['per_decoder'][n].items() if k.startswith('delay-differs')}
    if delays:
        L += ['', f'Frame-delay lists differing from `{fmt.reference}` (raw file delays) when frame counts agree: ' +
              ', '.join(f'{n} {v}' for n, v in delays.items()) + '.']
    L += ['', 'Unavailable or not integrated candidates:', ''] + [f'- {u}' for u in s['unavailable']]
    L += ['', '## Ten significant observations', ''] + [f'- {x}' for x in fmt.findings]
    L += ['', '## Representative leniency rows', '',
          'A row is the most frequent full outcome signature among *changed* mutations of one rule (a mutation whose acceptance/pixel-equality pattern equals its seed\'s is not counted). '
          '`n; p` = changed inputs; distinct patterns. Cells: `W×H hash8` or `reject class`. Witnesses are delta-debugged under the same signature (bounded, not globally minimal).', '',
          ('Runners outside the classification and majority votes (observed only): ' + ', '.join(sorted(fmt.observe_only)) + '.' if fmt.observe_only else ''), '',
          '| Rule (n; p) | Witness | ' + ' | '.join(names) + ' |', '| --- | --- | ' + ' | '.join(['---'] * len(names)) + ' |']
    for row in s['rows']:
        cells = []
        for n in names:
            v = row['results'][n]
            if v['status'] == 'ok':
                cells.append(f"{v['width']}×{v['height']} `{v['rgba_sha256'][:6]}`")
            elif v['status'] == 'error':
                cells.append('✗ `' + str(v.get('error_class', '?')).replace('|', '/')[:18] + '`')
            else:
                cells.append(v['status'])
        d = '; ' + row['pixel_difference'] if row.get('pixel_difference') else ''
        L.append(f"| {row['rule']} ({row['count']}; {row['patterns']}{d}) | [#{row['id']}]({row['witness']}) {row['witness_length']}B | " + ' | '.join(cells) + ' |')
    L += ['', '## Same acceptance, different first-frame pixels', '', 'Rules with the most `accept-different-pixels` inputs (smallest input per rule, not minimised; difference from raw RGBA dumps of the two largest hash groups).', '',
          '| Rule (n) | Witness | Difference | Hash groups |', '| --- | --- | --- | --- |']
    for row in s.get('pixel_rows', []):
        L.append(f"| {row['rule']} ({row['count']}) | [#{row['id']}]({row['witness']}) | {row['difference']} | " + '; '.join(f"{k}: {'+'.join(v)}" for k, v in row['groups']) + ' |')
    L += ['', '## Animation disagreements', '', 'Inputs where accepting animation-capable runners agree on the first frame or not, but disagree on canvas, frame count or composited sequence, clustered by rule (changed relative to the seed). Witnesses are not minimised.', '',
          '| Rule (n) | Witness | Kinds | Frames per runner |', '| --- | --- | --- | --- |']
    for row in s['animation_rows']:
        fr = '; '.join(f"{n} {v['frames'] if v['status'] == 'ok' else '✗'}" for n, v in row['results'].items())
        L.append(f"| {row['rule']} ({row['count']}) | [#{row['id']}]({row['witness']}) | {', '.join(f'{k} {v}' for k, v in row['animation'])} | {fr} |")
    L += ['', '## Acceptance by rule', '', 'Accepted inputs per rule (crash/timeout inputs excluded).', '',
          '| Rule | n | ' + ' | '.join(names) + ' |', '| --- | ---: | ' + ' | '.join(['---:'] * len(names)) + ' |']
    for rule, m in s['rule_matrix'].items():
        L.append(f"| {rule} | {m['total']} | " + ' | '.join(str(m[n]) for n in names) + ' |')
    L += ['', '## Crashes, timeouts and anomalies', '']
    if pub:
        L.append('Crash and timeout inputs are excluded from this public view; they are retained and counted only in the local matrix.')
    else:
        L.append(f"{s['anomaly_count']} inputs had at least one crash or timeout in some runner (counts under load: one CPU shared by about eight agents, {s.get('jobs', 3)} parallel inputs); inputs copied to `{fmt.folder}/anomalies/`. Not investigated here (the coordinator handles them). Per runner: " +
                 (', '.join(f'{k} {v}' for k, v in sorted(s['anomaly_by_decoder'].items())) or 'none') + '. Serial re-run of each failing runner (20 s timeout): ' +
                 (', '.join(f'{k} {v}' for k, v in sorted(s.get('anomaly_recheck_serial', {}).items())) or 'none') + '.')
    L += ['', '## Run', '', f'`./run.sh --inputs 10000 --seed {s["seed"]}` in `{fmt.adapter}` builds every runner under /tmp/claude-0 and regenerates this file; '
          f'`python3 reproduce.py FILE` runs one file through every runner. Per-input JSONL and the manifest (provenance: seed, kind, rule, field, old/new value) live in `{fmt.folder}`.', '']
    (fmt.adapter / filename).write_text('\n'.join(L))


def rerender(fmt):
    s = json.loads((fmt.adapter / 'summary.json').read_text())
    render(fmt, s, 'LENIENCY.md')
    pub = public_subset(fmt, s)
    (fmt.adapter / 'summary-public.json').write_text(json.dumps(pub, indent=1) + '\n')
    render(fmt, pub, 'LENIENCY-PUBLIC.md')


def main(fmt):
    import argparse
    p = argparse.ArgumentParser()
    p.add_argument('--inputs', type=int, default=10000)
    p.add_argument('--seed', type=int, default=731)
    p.add_argument('--jobs', type=int, default=3)
    p.add_argument('--timeout', type=float, default=5.0)
    p.add_argument('--min-checks', type=int, default=12)
    p.add_argument('--max-rows', type=int, default=40)
    p.add_argument('--render-only', action='store_true')
    p.add_argument('--reanalyse', action='store_true', help='recompute summary, witnesses and reports from the existing per-input results')
    a = p.parse_args()
    if a.render_only:
        rerender(fmt)
    elif a.reanalyse:
        old = json.loads((fmt.adapter / 'summary.json').read_text())
        finish(fmt, a, old['base_inputs'], old['seed_variants'], old['elapsed_seconds'])
    else:
        run(fmt, a)
