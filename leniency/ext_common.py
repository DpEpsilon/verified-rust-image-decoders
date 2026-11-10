"""Shared driver for the exr/heif/svg/raw adapters (new file; engine.py and format_harness.py are untouched).

An adapter supplies: NAME, EXT, RUNNERS {name: (exe, [selector args])}, INDEPENDENCE notes,
generate(limit, seed) -> list[(bytes, meta)], relate(results, dumps) -> (relation, detail).
Each runner prints one JSON line: status ok|error|crash|timeout, error_class, message, width, height,
rgba_sha256 (hash of the format's canonical sample dump), plus format-specific fields. Optional last
argument writes the canonical dump. All subprocess isolation is delegated to engine.invoke.
"""
import collections
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import random
import shutil
import sys
import threading
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
import engine  # noqa: E402

TMP = Path('/tmp/claude-0')


def classify(results, relation):
    """results: {decoder: json}; relation: 'identical'|'tolerance'|'different'|'dims' (accepted decoders only)."""
    st = [v['status'] for v in results.values()]
    if 'crash' in st or 'timeout' in st:
        return 'crash-or-timeout'
    ok = [v for v in results.values() if v['status'] == 'ok']
    if not ok:
        return 'all-reject'
    if len(ok) < len(st):
        return 'accept-vs-reject' if len(ok) >= 1 else 'all-reject'
    return {'identical': 'all-accept-same', 'tolerance': 'accept-within-tolerance',
            'dims': 'accept-different-dimensions', 'different': 'accept-different-pixels',
            'single': 'all-accept-same'}[relation]


def read_dump(path, fmt):
    data = Path(path).read_bytes()
    return data


def pattern(results):
    """Compact accept/reject pattern, e.g. 'a=ok,b=err'."""
    return ','.join(f"{k}={'A' if v['status']=='ok' else ('R' if v['status']=='error' else v['status'][0].upper()+'!')}"
                    for k, v in results.items())


def default_relate(tolerance_fn=None):
    """Return a relate() comparing accepted decoders on (width,height,hash) then via tolerance_fn(dumpA,dumpB,detail)."""
    def relate(results, dumps):
        ok = [(k, v) for k, v in results.items() if v['status'] == 'ok']
        if len(ok) < 2:
            return 'single', ''
        if len({(v['width'], v['height']) for _, v in ok}) > 1:
            return 'dims', ';'.join(f"{k}={v['width']}x{v['height']}" for k, v in ok)
        if len({v['rgba_sha256'] for _, v in ok}) == 1:
            return 'identical', ''
        if tolerance_fn is None:
            return 'different', ''
        ref_name, _ = ok[0]
        worst, detail = 'tolerance', []
        for k, _ in ok[1:]:
            if k not in dumps or ref_name not in dumps:
                return 'different', 'no dump'
            r = tolerance_fn(dumps[ref_name], dumps[k])
            detail.append(f'{ref_name}~{k}:{r[1]}')
            if not r[0]:
                worst = 'different'
        return worst, ';'.join(detail)
    return relate


def _work(cfg, idx, data, meta, workdir, timeout, want_dump):
    ext = cfg['EXT']
    tok = f'{os.getpid()}-{threading.get_ident()}'
    f = workdir / f'w{tok}.{ext}'
    f.write_bytes(data)
    results, dumps = {}, {}
    for name, runner in cfg['RUNNERS'].items():
        d = workdir / f'd{tok}-{name}.bin'
        if d.exists():
            d.unlink()
        r = engine.invoke(f, runner, timeout, d)
        results[name] = {k: r.get(k) for k in ('status', 'error_class', 'message', 'width', 'height', 'rgba_sha256', 'selection',
                                               'orig_color_type', 'alpha_present', 'extra') if k in r}
        if r.get('status') == 'ok' and d.exists() and d.stat().st_size < 8_000_000:
            dumps[name] = d.read_bytes()
    relation, detail = cfg['relate'](results, dumps)
    klass = classify(results, relation)
    for d in workdir.glob(f'd{tok}-*.bin'):
        d.unlink()
    return {'i': idx, 'rule': meta.get('rule'), 'source': meta.get('source'), 'seed': meta.get('seed'),
            'class': klass, 'pattern': pattern(results), 'relation_detail': detail[:200],
            'results': {k: {kk: (vv[:120] if isinstance(vv, str) else vv) for kk, vv in v.items()} for k, v in results.items()}}


def summarize(cfg, records, n_seed, total_inputs):
    classes = collections.Counter(r['class'] for r in records)
    per_dec = {k: collections.Counter() for k in cfg['RUNNERS']}
    errs = {k: collections.Counter() for k in cfg['RUNNERS']}
    pair = collections.Counter()
    names = list(cfg['RUNNERS'])
    for r in records:
        for k, v in r['results'].items():
            per_dec[k][v['status']] += 1
            if v['status'] == 'error':
                errs[k][(v.get('error_class') or '')[:60]] += 1
        for i, a in enumerate(names):
            for b in names[i + 1:]:
                sa, sb = r['results'][a]['status'], r['results'][b]['status']
                if 'crash' in (sa, sb) or 'timeout' in (sa, sb):
                    continue
                if (sa == 'ok') != (sb == 'ok'):
                    pair[f'{a}|{b}:accept-disagree'] += 1
                elif sa == 'ok':
                    same = (r['results'][a]['width'], r['results'][a]['height'], r['results'][a]['rgba_sha256']) == \
                           (r['results'][b]['width'], r['results'][b]['height'], r['results'][b]['rgba_sha256'])
                    pair[f'{a}|{b}:both-accept-{"same" if same else "differ"}'] += 1
    rules = {}
    for r in records:
        key = r['rule'] or '?'
        e = rules.setdefault(key, {'n': 0, 'classes': collections.Counter(), 'patterns': collections.Counter()})
        e['n'] += 1
        e['classes'][r['class']] += 1
        e['patterns'][r['pattern']] += 1
    anomalies = sum(1 for r in records if r['class'] == 'crash-or-timeout')
    crash_counts = {k: {'crash': c.get('crash', 0), 'timeout': c.get('timeout', 0)} for k, c in per_dec.items()}
    return {
        'format': cfg['NAME'], 'seed': cfg.get('seed'), 'inputs': total_inputs, 'seed_inputs': n_seed,
        'decoders': {k: {'exe': str(v[0]), 'args': list(v[1])} for k, v in cfg['RUNNERS'].items()},
        'independence_notes': cfg.get('INDEPENDENCE', ''),
        'class_counts': dict(classes.most_common()),
        'per_decoder_status': {k: dict(v) for k, v in per_dec.items()},
        'per_decoder_top_errors': {k: dict(v.most_common(6)) for k, v in errs.items()},
        'pairwise': dict(sorted(pair.items())),
        'rules': {k: {'n': v['n'], 'classes': dict(v['classes'].most_common()),
                      'top_patterns': dict(v['patterns'].most_common(4))} for k, v in sorted(rules.items())},
        'crash_timeout_inputs': anomalies,
        'crash_timeout_by_decoder': crash_counts,
    }


def write_markdown(path, cfg, summ, title, public):
    L = [f'# {title}', '',
         f"Seed {summ['seed']}; {summ['inputs']} inputs ({summ['seed_inputs']} synthetic seeds, rest seeded mutations). "
         'All inputs are generated by this directory (no third-party files).', '']
    if public:
        L += ['Crash/timeout rows are excluded from this public recount; the number of excluded inputs is given below as a count only.', '']
    L += ['## Decoders', '']
    for k, v in summ['decoders'].items():
        L.append(f"- `{k}`: `{Path(v['exe']).name} {' '.join(v['args'])}`")
    L += ['', summ['independence_notes'], '', '## Classification counts', '', '| class | inputs |', '| --- | ---: |']
    for k, v in summ['class_counts'].items():
        L.append(f'| {k} | {v} |')
    L += ['', '## Per-decoder status', '', '| decoder | ok | error | crash | timeout |', '| --- | ---: | ---: | ---: | ---: |']
    for k, v in summ['per_decoder_status'].items():
        L.append(f"| {k} | {v.get('ok',0)} | {v.get('error',0)} | {v.get('crash',0)} | {v.get('timeout',0)} |")
    L += ['', '## Pairwise agreement (crash/timeout rows excluded)', '']
    for k, v in summ['pairwise'].items():
        L.append(f'- {k}: {v}')
    L += ['', '## Most frequent rejection classes per decoder', '']
    for k, v in summ['per_decoder_top_errors'].items():
        L.append(f'- `{k}`: ' + '; '.join(f'{c or "(empty)"} x{n}' for c, n in v.items()) if v else f'- `{k}`: none')
    L += ['', '## By mutation rule', '', '| rule | n | classes | most frequent accept/reject pattern |', '| --- | ---: | --- | --- |']
    for k, v in summ['rules'].items():
        cl = ', '.join(f'{c} {n}' for c, n in v['classes'].items())
        pat = next(iter(v['top_patterns']))
        L.append(f"| {k} | {v['n']} | {cl} | `{pat}` |")
    L += ['', f"Crash/timeout inputs (count only, saved outside the repository): {summ['crash_timeout_inputs']}", '']
    if not public:
        L += ['Per-decoder crash/timeout counts: ' + json.dumps(summ['crash_timeout_by_decoder']), '']
    Path(path).write_text('\n'.join(L))


def _append_findings(here, name):
    f = Path(here) / 'findings.md'
    if f.exists():
        p = Path(here) / name
        p.write_text(p.read_text() + '\n' + f.read_text())


def _write_docs(cfg, here, records, n_seed):
    summ = summarize(cfg, records, n_seed, len(records))
    (here / 'summary.json').write_text(json.dumps(summ, indent=1, sort_keys=True))
    write_markdown(here / 'LENIENCY.md', cfg, summ, f"{cfg['NAME'].upper()} decoder leniency matrix", False)
    _append_findings(here, 'LENIENCY.md')
    pub = [r for r in records if r['class'] != 'crash-or-timeout']
    psum = summarize(cfg, pub, n_seed, len(pub))
    psum['excluded_crash_timeout_inputs'] = len(records) - len(pub)
    (here / 'summary-public.json').write_text(json.dumps(psum, indent=1, sort_keys=True))
    write_markdown(here / 'LENIENCY-PUBLIC.md', cfg, psum, f"{cfg['NAME'].upper()} decoder leniency matrix (public recount)", True)
    _append_findings(here, 'LENIENCY-PUBLIC.md')
    return summ


def run_main(cfg, here, argv=None):
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('--inputs', type=int, default=3000)
    ap.add_argument('--seed', type=int, default=731)
    ap.add_argument('--jobs', type=int, default=2)
    ap.add_argument('--timeout', type=int, default=10)
    ap.add_argument('--redoc', action='store_true', help='rebuild summaries/markdown from the existing results.jsonl without running decoders')
    a = ap.parse_args(argv)
    here = Path(here)
    cfg['seed'] = a.seed
    out = TMP / f"leniency-{cfg['NAME']}"
    (out / 'work').mkdir(parents=True, exist_ok=True)
    if a.redoc:
        records = [json.loads(l) for l in (out / 'results.jsonl').read_text().splitlines()]
        n_seed = sum(1 for l in (out / 'manifest.jsonl').read_text().splitlines() if json.loads(l).get('source') == 'synthetic-seed')
        _write_docs(cfg, here, records, n_seed)
        return None
    if (out / 'anomalies').exists():
        shutil.rmtree(out / 'anomalies')
    (out / 'anomalies').mkdir()
    gen, n_seed = cfg['generate'](a.inputs, a.seed)
    with (out / 'manifest.jsonl').open('w') as mf:
        for i, (b, m) in enumerate(gen):
            mf.write(json.dumps({'i': i, 'len': len(b), 'sha256': hashlib.sha256(b).hexdigest(), **m}, default=str) + '\n')
    t0 = time.time()
    records = []
    with concurrent.futures.ThreadPoolExecutor(a.jobs) as ex:
        futs = [ex.submit(_work, cfg, i, b, m, out / 'work', a.timeout, True) for i, (b, m) in enumerate(gen)]
        for f in futs:
            records.append(f.result())
    with (out / 'results.jsonl').open('w') as rf:
        for r in records:
            rf.write(json.dumps(r) + '\n')
    for r in records:
        if r['class'] == 'crash-or-timeout':
            b = gen[r['i']][0]
            (out / 'anomalies' / f"{r['i']:05d}-{hashlib.sha256(b).hexdigest()[:12]}.{cfg['EXT']}").write_bytes(b)
    summ = _write_docs(cfg, here, records, n_seed)
    # compact witnesses: first synthetic input per (class, rule), max 40, each at most 64 KiB
    wd = here / 'witnesses'
    if wd.exists():
        shutil.rmtree(wd)
    wd.mkdir()
    seen, n = set(), 0
    for r in records:
        key = (r['class'], r['rule'])
        if r['class'] in ('all-all-accept-same', 'all-accept-same', 'crash-or-timeout') or key in seen or n >= 40:
            continue
        b = gen[r['i']][0]
        if len(b) > 65536:
            continue
        seen.add(key)
        n += 1
        (wd / f"{n:02d}-{r['class']}-{r['rule']}.{cfg['EXT']}".replace('/', '_')).write_bytes(b)
    print(json.dumps({'inputs': len(gen), 'classes': summ['class_counts'], 'seconds': round(time.time() - t0)}))
    return summ


# ---- mutation helpers shared by generators -------------------------------------------------

def flips(b, rng, n=12):
    for _ in range(n):
        if not b:
            return
        p = rng.randrange(len(b))
        a = bytearray(b)
        a[p] ^= 1 << rng.randrange(8)
        yield bytes(a), {'rule': 'byte-flip', 'field': 'offset', 'value': p}


def structural(b, rng, tail=(1, 8, 64)):
    n = len(b)
    for cut in sorted({0, 1, 4, 8, 16, 32, n // 4, n // 2, n - 1}):
        if 0 <= cut < n:
            yield b[:cut], {'rule': 'truncation', 'field': 'length', 'value': cut}
    for t in tail:
        yield b + bytes(t), {'rule': 'trailing-bytes', 'field': 'tail', 'value': t}


def interleave(seed_series, limit, rng):
    """seed_series: list of (bytes, meta, iterator of (bytes, meta)). Round-robin mutations after the seeds."""
    out = [(b, dict(m, source='synthetic-seed')) for b, m, _ in seed_series][:limit]
    its = [(m, it) for _, m, it in seed_series]
    active = list(range(len(its)))
    while len(out) < limit and active:
        for i in active[:]:
            try:
                b, meta = next(its[i][1])
            except StopIteration:
                active.remove(i)
                continue
            meta = dict(meta, source='mutation', seed=its[i][0]['name'])
            out.append((b, meta))
            if len(out) >= limit:
                break
    return out, len(seed_series)


def patch(b, pos, value, n, endian='little'):
    a = bytearray(b)
    if pos + n > len(a):
        return None
    a[pos:pos + n] = (value & ((1 << (8 * n)) - 1)).to_bytes(n, endian)
    return bytes(a)
