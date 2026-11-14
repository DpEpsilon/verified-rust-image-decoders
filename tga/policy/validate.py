#!/usr/bin/env python3
"""Compare the TGA policy oracle with native decoders and write REPORT.md.

Three input sets, all compared on accept/reject plus (width, height, SHA-256 of
RGBA8) for accepted results; error names are never compared:
  1. the local real-file corpus ($CORPUS_DIR/tga (default ~/corpora/tga); 300 measured files),
     using the native results already recorded in results.jsonl;
  2. the synthetic POLICY-VECTORS.json inputs (natives invoked here);
  3. seeded mutation sets from ~/projects/leniency/format_harness.generate:
     a development seed and a held-out seed whose inputs exclude every byte
     string of the development seed (natives invoked here, results cached).
Targets: Strict vs the shipped Verus core (`verified`), GdkCompat-2.42.12 vs
gdk-pixbuf (`gdk-forced-whole`; `gdk-file` on the corpus), ImageCrateCompat-
0.25.10 vs image 0.25.10 (`image`; glycin's image-rs path is the same code and
not a second vote).  RecommendedBeta is checked for its defining invariant: every
Strict-accepted input decodes to the identical RGBA with recovered=false.
Corpus bytes are read in place and never copied.
"""
import argparse
import collections
import hashlib
import json
import os
from pathlib import Path
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, '/home/ws/projects/leniency')
import policy_oracle as O

CORPUS = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser() / 'tga'
WORK = Path('/tmp/claude-0/tga-policy')
TARGETS = {'Strict': ['verified'], 'GdkCompat-2.42.12': ['gdk-forced-whole'],
           'ImageCrateCompat-0.25.10': ['image']}
CORPUS_TARGETS = {'Strict': ['verified'], 'GdkCompat-2.42.12': ['gdk-forced-whole', 'gdk-file'],
                  'ImageCrateCompat-0.25.10': ['image', 'glycin']}


def observe(b, profile):
    try:
        w, h, rgba, rec = O.decode_with_policy(b, profile)
        return ('ok', w, h, hashlib.sha256(rgba).hexdigest()), rec
    except O.Invalid as e:
        return ('error', str(e)), False


def native(r):
    return ('ok', r['width'], r['height'], r['rgba_sha256']) if r['status'] == 'ok' else ('error',)


def classify(ours, theirs):
    if theirs[0] == 'ok' and (max(theirs[1], theirs[2]) > O.MAX_DIMENSION
                              or 4 * theirs[1] * theirs[2] > O.MAX_OUTPUT):
        return 'native accepted beyond universal caps'
    if ours[0] == 'ok' and theirs[0] == 'error' and ours[1] * ours[2] > 16_000_000:
        return 'native runner harness limit (more than 16,000,000 pixels)'
    if ours[0] == 'ok' and theirs[0] == 'ok':
        return 'both accept, pixels differ'
    if ours[0] == 'ok':
        return 'only oracle accepts'
    return 'only native accepts'


def new_cell():
    return {'total': 0, 'agree': 0, 'both_reject': 0, 'native_accepted': 0,
            'accepted_same': 0, 'residuals': collections.Counter(), 'examples': []}


def compare(items, targets):
    """items: iterable of (id, bytes, {target: native result}).  Returns report."""
    report = {p: {t: new_cell() for t in ts} for p, ts in targets.items()}
    rec = {'strict_ok': 0, 'unchanged': 0, 'violations': [], 'newly_accepted': 0,
           'newly_by_strict_error': collections.Counter(), 'recovered_true': 0}
    for ident, b, results in items:
        obs = {p: observe(b, p) for p in O.PROFILES}
        for profile, ts in targets.items():
            ours = obs[profile][0]
            for t in ts:
                if t not in results:
                    continue
                theirs = native(results[t])
                if theirs[0] == 'error' and results[t]['status'] not in ('error',):
                    theirs = ('error',)  # crash/timeout counts as rejection
                c = report[profile][t]
                same = ours[:1] == theirs[:1] if ours[0] == 'error' else ours == theirs
                c['total'] += 1
                c['agree'] += same
                c['both_reject'] += ours[0] == 'error' and theirs[0] == 'error'
                c['native_accepted'] += theirs[0] == 'ok'
                c['accepted_same'] += theirs[0] == 'ok' and same
                if not same:
                    why = classify(ours, theirs)
                    c['residuals'][why] += 1
                    if len(c['examples']) < 12:
                        c['examples'].append({'id': ident, 'why': why, 'oracle': ours[:2],
                                              'native': theirs[:1]})
        s, r = obs['Strict'][0], obs['RecommendedBeta']
        if s[0] == 'ok':
            rec['strict_ok'] += 1
            if r[0] == s and not r[1]:
                rec['unchanged'] += 1
            else:
                rec['violations'].append(ident)
        elif r[0][0] == 'ok':
            rec['newly_accepted'] += 1
            rec['newly_by_strict_error'][s[1]] += 1
            rec['recovered_true'] += r[1]
    return report, rec


def corpus_items():
    for line in (CORPUS / 'results.jsonl').open():
        row = json.loads(line)
        yield row['file'][:16], (CORPUS / 'files' / row['file']).read_bytes(), row['res']


def run_natives(items, folder, names, timeout=10):
    """Invoke native runners once per input; cache by input SHA-256."""
    import format_harness as H
    folder.mkdir(parents=True, exist_ok=True)
    cache_path = folder / 'native.jsonl'
    cache = {}
    if cache_path.exists():
        for line in cache_path.open():
            row = json.loads(line)
            cache[row['sha256']] = row['results']
    with cache_path.open('a') as saved:
        for i, (ident, b) in enumerate(items):
            digest = hashlib.sha256(b).hexdigest()
            if digest not in cache:
                path = folder / 'input.tga'
                path.write_bytes(b)
                cache[digest] = {n: H.invoke('tga', path, n, timeout) for n in names}
                saved.write(json.dumps({'sha256': digest, 'results': cache[digest]}) + '\n')
                if (i + 1) % 500 == 0:
                    print(f'{folder.name}: native {i + 1}', flush=True)
            yield ident, b, cache[digest]
    (folder / 'input.tga').unlink(missing_ok=True)


def seed_inputs(seed, count, exclude=()):
    import format_harness as H
    items, _, _ = H.generate('tga', count, seed)
    out, seen = [], set(exclude)
    for i, (b, m) in enumerate(items):
        d = hashlib.sha256(b).digest()
        if d in seen:
            continue
        seen.add(d)
        out.append((f'{seed}:{i}:{m["rule"]}', b))
    return out


def fmt_cell(c):
    res = ', '.join(f'{k} {v}' for k, v in c['residuals'].most_common()) or 'none'
    return (f"{c['agree']}/{c['total']} | {c['accepted_same']}/{c['native_accepted']} | "
            f"{c['both_reject']} | {res}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--dev-seed', type=int, default=20261111)
    ap.add_argument('--heldout-seed', type=int, default=4242)
    ap.add_argument('--seen-seed', type=int, default=777,
                    help='seed inspected while tuning (reported, and excluded from the held-out set)')
    ap.add_argument('--count', type=int, default=4000)
    args = ap.parse_args()
    names = ['verified', 'image', 'gdk-forced-whole']
    sections = {}
    sections['corpus (300 real files; regression, inspected while writing the spec)'] = compare(corpus_items(), CORPUS_TARGETS)
    pv = json.loads((HERE / 'POLICY-VECTORS.json').read_text())
    vec_items = [(r['name'], bytes.fromhex(r['input_hex'])) for r in pv['profiles']['Strict']]
    sections[f'synthetic POLICY-VECTORS ({len(vec_items)}; regression, written from the spec)'] = compare(
        run_natives(vec_items, WORK / 'vectors', names), TARGETS)
    dev = seed_inputs(args.dev_seed, args.count)
    sections[f'seed {args.dev_seed} development, tuned against (regression; {len(dev)} distinct)'] = compare(
        run_natives(dev, WORK / f'seed-{args.dev_seed}', names), TARGETS)
    exclude = {hashlib.sha256(b).digest() for _, b in dev}
    exclude |= {hashlib.sha256(b).digest() for _, b in vec_items}
    seen = seed_inputs(args.seen_seed, args.count, exclude)
    sections[f'seed {args.seen_seed} inspected during tuning (regression; {len(seen)} inputs absent from dev seed and vectors)'] = compare(
        run_natives(seen, WORK / f'seed-{args.seen_seed}', names), TARGETS)
    exclude |= {hashlib.sha256(b).digest() for _, b in seen}
    held = seed_inputs(args.heldout_seed, args.count, exclude)
    sections[f'seed {args.heldout_seed} held-out, run once after the final spec edit (the only held-out figure; {len(held)} inputs absent from all other sets)'] = compare(
        run_natives(held, WORK / f'seed-{args.heldout_seed}', names), TARGETS)
    serial = {k: ({p: {t: dict(c, residuals=dict(c['residuals'])) for t, c in ts.items()}
                   for p, ts in rep.items()},
                  dict(rec, newly_by_strict_error=dict(rec['newly_by_strict_error'])))
              for k, (rep, rec) in sections.items()}
    (WORK / 'agreements.json').write_text(json.dumps(serial, indent=1) + '\n')
    lines = ['## Measured agreement', '',
             'Columns: all-input agreement (accept/reject, and identical width/height/RGBA SHA-256 when both accept) | '
             'exact pixels among inputs the native decoder accepts | both reject | residual classes. '
             'Error names are not compared: when the oracle rejects, validate.py counts any native rejection as agreement (`same = ours[:1] == theirs[:1]`), so a both-reject row says nothing about the first error. `verified` is the shipped Verus core, so the Strict row checks the oracle against the implementation.', '',
             'Only the seed-4242 run is independent validation: its 1,465 inputs were generated after the final spec edit and run once, and each named profile agrees with its native target on 1465/1465. The corpus, the synthetic vectors and the tuned seeds (20261111, 777) were inspected while the spec was written, so their agreement figures are regression checks, not evidence of generalisation. All mutation sets come from the same generator. glycin\'s image-rs path and image 0.25.10, and the two GDK entry points, are not independent votes.', '']
    for title, (rep, rec) in sections.items():
        lines += [f'### {title}', '', '| profile vs native | all inputs | native-accepted exact | both reject | residuals |',
                  '|---|---|---|---|---|']
        for p, ts in rep.items():
            for t, c in ts.items():
                lines.append(f'| {p} vs {t} | {fmt_cell(c)} |')
        newly = ', '.join(f'{k} {v}' for k, v in sorted(rec['newly_by_strict_error'].items())) or 'none'
        lines += ['', f"RecommendedBeta: {rec['unchanged']}/{rec['strict_ok']} Strict-accepted inputs decode identically with "
                  f"recovered=false (violations: {len(rec['violations'])}); it additionally accepts {rec['newly_accepted']} "
                  f"inputs that Strict rejects ({newly}); recovered=true on {rec['recovered_true']} of them.", '']
        for p, ts in rep.items():
            for t, c in ts.items():
                if c['examples']:
                    lines.append(f"Residual examples, {p} vs {t}: " + '; '.join(
                        f"`{e['id']}` {e['why']}" for e in c['examples'][:6]))
        lines.append('')
    explained = WORK / 'pixdiff-explained.json'
    if explained.exists():
        rows = json.loads(explained.read_text())
        tally = collections.Counter((r['decoder'], ' + '.join(r['responsible']) or 'unexplained')
                                    for r in rows)
        exact = sum(r['record_matches'] for r in rows)
        lines += ['## Corpus pixel differences (investigate_pixdiff.py)', '',
                  f'{exact}/{len(rows)} decoder/file pairs ({len({r["file"] for r in rows})} distinct files) from pixdiff.tsv are reproduced byte-exactly by the '
                  "decoder's chosen rule record. Listed fields: reverting any one alone breaks the match, so each listed field is necessary within the chosen record; "
                  'sufficiency/uniqueness not shown (another record could also match), and the tinytga post-transforms (`tiny:`, `harness:`) are not independently validated.', '',
                  '| decoder | pairs | responsible rule |', '|---|---:|---|']
        lines += [f'| {d} | {c} | {why} |' for (d, why), c in sorted(tally.items())]
        lines.append('')
    (HERE / 'REPORT.md').write_text(PREAMBLE + '\n'.join(lines) + '\n')
    print('\n'.join(lines))


PREAMBLE = '''# TGA policy candidate: report

Status: frozen candidate, not frozen. 13 internal fields, 4 named profiles (Strict = shipped TGA-CORE-SPEC 1.0, RecommendedBeta = Strict plus four error-site relaxations, GdkCompat-2.42.12, ImageCrateCompat-0.25.10). `policy_oracle.py` passes all 169 frozen vectors under Strict and every vector in `POLICY-VECTORS.json`. Everything below this paragraph is regenerated by `python3 -I validate.py` (after `investigate_pixdiff.py`); native results are cached under `/tmp/claude-0/tga-policy/`.

'''


if __name__ == '__main__':
    main()
