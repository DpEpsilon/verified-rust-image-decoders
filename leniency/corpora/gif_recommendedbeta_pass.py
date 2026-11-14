#!/usr/bin/env python3
"""Q9 corpus pass: run RecommendedBeta (gif/policy-draft/policy_oracle.py) over the pinned GIF corpus.

  python3 -I gif_recommendedbeta_pass.py oracle     [--limit N] [--timeout S] [--cache DIR]
  python3 -I gif_recommendedbeta_pass.py incumbents [--limit N] [--bin DIR] [--cache DIR]
  python3 -I gif_recommendedbeta_pass.py report     [--cache DIR] [--out-json gif-recommendedbeta-pass.json]

Reads $CORPUS_DIR/gif/MANIFEST.tsv (default ~/corpora; its SHA-256 must equal the pin below) and
files/<sha256>.gif.  The three phases are resumable: each appends one JSON line per file to
<cache>/oracle.jsonl or <cache>/incumbents.jsonl and skips files already there.  Output holds file
names (repo paths), error names and numbers only, never file bytes.

oracle phase, per file, for Strict and RecommendedBeta (both via policy_oracle.PROFILES):
  still  = decode_still; stream = decode_frames, accepted when `end == 'trailer'`;
  obligation R in its three forms (validate.py): R-still, R-stream, R-prefix;
  for every file where RecommendedBeta accepts and Strict rejects (a "flip"), ablation: revert one
  of RecommendedBeta's ten non-Strict fields at a time to its Strict value and record whether the
  outcome changes and which error appears instead (the error that field replaced).
incumbents phase: gdk-pixbuf 2.42.12 (animation path, forced loader, whole buffer) and the
  `image` 0.25.10 runner (gif 0.14.2) from build-min.sh; first composited frame as RGBA8 with RGB
  zeroed where alpha is 0; the `image` runner's `tail_error` (an error after its first frame) is kept.
report phase: joins both caches with the manifest and writes the JSON summary.
"""
import argparse
import array
import hashlib
import json
import os
import resource
import signal
import subprocess
import sys
import tempfile
import time
from collections import Counter, defaultdict
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT / 'gif' / 'policy-draft'))
sys.path.insert(0, str(ROOT / 'gif' / 'spec-draft'))
sys.path.insert(0, str(HERE))
import policy_oracle as PO  # noqa: E402
import gif_stats  # noqa: E402  (is_fuzz / is_incidental: the definitions of the earlier statistics)

MANIFEST_SHA256 = '122e6ecacdbe081c7bf32209ede195833d2fcda4553381dd39695631b39ccf23'
STRICT, RB = PO.STRICT, PO.RECOMMENDED_BETA
FIELDS = [k for k in PO.PIXEL_FIELDS if RB[k] != STRICT[k]]
assert len(FIELDS) == 10, FIELDS
IGNORE = ('recovered', 'events')


class Timeout(Exception):
    pass


def _alarm(signum, frame):
    raise Timeout()


def corpus_root():
    return Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'gif'


def read_manifest():
    root = corpus_root()
    raw = (root / 'MANIFEST.tsv').read_bytes()
    assert hashlib.sha256(raw).hexdigest() == MANIFEST_SHA256, 'manifest differs from the pinned one'
    rows = [l.split('\t') for l in raw.decode().splitlines()[1:]]
    return root, [{'i': i, 'path': r[0], 'sha256': r[1], 'repo': r[4], 'repo_path': r[6], 'size': int(r[7])}
                  for i, r in enumerate(rows)]


def norm_rgba(rgba):
    """sha256 of the RGBA bytes with RGB set to 0 where alpha is 0 (little-endian words: alpha is the top byte)."""
    w = array.array('I')
    w.frombytes(bytes(rgba))
    if sys.byteorder == 'big':
        w.byteswap()
    out = array.array('I', [0 if v < 0x01000000 else v for v in w])
    return hashlib.sha256(out.tobytes()).hexdigest()


# ---------------------------------------------------------------- oracle phase

def frame_digest(f):
    h = hashlib.sha256()
    h.update(repr(tuple((k, f[k]) for k in sorted(f) if k not in IGNORE + ('rgba',))).encode())
    h.update(f['rgba'])
    return h.hexdigest()


def stream_of(b, P):
    """Summary of decode_frames: status, first error, per-frame digests (kept in memory for the R check)."""
    res = PO.decode_frames(b, P)
    if 'error' in res:
        return {'ok': False, 'error': res['error'], 'ctor': True, 'frames': 0, 'dig': [], 'rec_frames': 0,
                'events': [], 'loop': None, 'whole': None}
    fr = res['frames']
    end = res['end']
    ok = end == 'trailer'
    dig = [frame_digest(f) for f in fr]
    ev = sorted({e for f in fr for e in f['events']})
    whole = hashlib.sha256(json.dumps([res['width'], res['height'], res['loop_count'], dig, ok]).encode()).hexdigest()
    return {'ok': ok, 'error': None if ok else end['error'], 'ctor': False, 'frames': len(fr), 'dig': dig,
            'rec_frames': sum(1 for f in fr if f['recovered']), 'events': ev, 'loop': res['loop_count'],
            'whole': whole, 'rec': [bool(f['recovered']) for f in fr]}


def still_of(b, P):
    res = PO.decode_still(b, P)
    if 'error' in res:
        return {'ok': False, 'error': res['error']}
    return {'ok': True, 'error': None, 'dims': [res['width'], res['height']], 'rgba': norm_rgba(res['rgba']),
            'recovered': bool(res['recovered']),
            'raw': hashlib.sha256(res['rgba']).hexdigest()}


def same_outcome(x, y, keys):
    return all(x.get(k) == y.get(k) for k in keys)


def ablate(b, kind, base):
    """One-field-at-a-time reversion of RecommendedBeta; returns {field: replaced error or 'differs'}."""
    out = {}
    for f in FIELDS:
        P = dict(RB, **{f: STRICT[f]})
        if kind == 'stream':
            s = stream_of(b, P)
            if s['ok'] and base['ok']:
                if s['whole'] != base['whole']:
                    out[f] = 'DIFFERS'
            elif s['ok'] != base['ok'] or s['error'] != base['error']:
                out[f] = s['error'] or 'DIFFERS'
        else:
            s = still_of(b, P)
            if s['ok'] and base['ok']:
                if s['raw'] != base['raw'] or s['recovered'] != base['recovered']:
                    out[f] = 'DIFFERS'
            elif s['ok'] != base['ok'] or s['error'] != base['error']:
                out[f] = s['error'] or 'DIFFERS'
    if not out:        # no single field explains the flip: try pairs
        for i, f in enumerate(FIELDS):
            for g in FIELDS[i + 1:]:
                P = dict(RB, **{f: STRICT[f], g: STRICT[g]})
                s = stream_of(b, P) if kind == 'stream' else still_of(b, P)
                if s['ok'] != base['ok']:
                    out[f + '+' + g] = s['error'] or 'DIFFERS'
    return out


def analyse(b):
    t0 = time.time()
    rec = {}
    S_st, R_st = stream_of(b, STRICT), stream_of(b, RB)
    S_sl, R_sl = still_of(b, STRICT), still_of(b, RB)
    # obligation R (validate.py): still, stream, prefix
    viol = []
    if S_sl['ok'] and not (R_sl['ok'] and R_sl['raw'] == S_sl['raw'] and not R_sl['recovered']):
        viol.append('still')
    if S_st['ok'] and not (R_st['ok'] and R_st['whole'] == S_st['whole'] and R_st['rec_frames'] == 0):
        viol.append('stream')
    for i, d in enumerate(S_st['dig']):
        if i >= len(R_st['dig']) or R_st['dig'][i] != d or R_st['rec'][i]:
            viol.append(f'prefix@{i}')
            break
    rec['R_violations'] = viol
    rec['R_frames_checked'] = len(S_st['dig'])
    for tag, s in (('S', S_st), ('RB', R_st)):
        rec[tag + '_stream'] = {k: v for k, v in s.items() if k not in ('dig', 'rec')}
        rec[tag + '_stream']['digest_frames'] = len(s['dig'])
    rec['S_still'] = {k: v for k, v in S_sl.items() if k != 'raw'}
    rec['RB_still'] = {k: v for k, v in R_sl.items() if k != 'raw'}
    rec['flip_stream'] = (not S_st['ok']) and R_st['ok']
    rec['flip_still'] = (not S_sl['ok']) and R_sl['ok']
    rec['ablate'] = {}
    if rec['flip_stream']:
        rec['ablate']['stream'] = ablate(b, 'stream', R_st)
    if rec['flip_still']:
        rec['ablate']['still'] = ablate(b, 'still', R_sl)
    rec['seconds'] = round(time.time() - t0, 2)
    return rec


def cache_load(path):
    done = {}
    if path.exists():
        for line in path.read_text().splitlines():
            try:
                r = json.loads(line)
            except ValueError:
                continue
            done[r['sha256']] = r
    return done


def phase_oracle(a):
    root, rows = read_manifest()
    cache = Path(a.cache)
    cache.mkdir(parents=True, exist_ok=True)
    cpath = cache / 'oracle.jsonl'
    done = cache_load(cpath)
    signal.signal(signal.SIGALRM, _alarm)
    resource.setrlimit(resource.RLIMIT_AS, (3 << 30, 3 << 30))
    if a.limit:
        rows = rows[:a.limit]
    todo = [r for r in rows if r['sha256'] not in done]
    t_end = time.time() + a.budget
    n = 0
    with cpath.open('a') as out:
        for r in todo:
            if time.time() > t_end:
                break
            b = (root / r['path']).read_bytes()
            signal.alarm(int(a.timeout))
            try:
                rec = analyse(b)
            except Timeout:
                rec = {'timeout': True}
            except MemoryError:
                rec = {'memory_error': True}
            finally:
                signal.alarm(0)
            rec.update({'sha256': r['sha256'], 'i': r['i']})
            out.write(json.dumps(rec) + '\n')
            out.flush()
            n += 1
    print(f'oracle: {n} files analysed this call, {len(done) + n} of {len(rows)} cached')


# ---------------------------------------------------------------- incumbents phase

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (3 << 30, 3 << 30))


def run_runner(cmd, timeout):
    try:
        p = subprocess.run(cmd, capture_output=True, timeout=timeout, preexec_fn=limits)
    except subprocess.TimeoutExpired:
        return {'status': 'timeout'}
    for line in reversed(p.stdout.decode('utf-8', 'replace').strip().splitlines()):
        if line.startswith('{'):
            try:
                return json.loads(line)
            except ValueError:
                break
    return {'status': 'crash', 'error_class': f'exit{p.returncode}'}


def phase_incumbents(a):
    root, rows = read_manifest()
    cache = Path(a.cache)
    cache.mkdir(parents=True, exist_ok=True)
    cpath = cache / 'incumbents.jsonl'
    done = cache_load(cpath)
    if a.limit:
        rows = rows[:a.limit]
    bindir = Path(a.bin)
    libs = {'gdk_anim': [str(bindir / 'gdk-anim'), 'gif', 'anim'],
            'gdk_forced': [str(bindir / 'gdk-anim'), 'gif', 'forced-whole'],
            'image': [str(bindir / 'rust-image'), 'gif:image']}
    scratch = Path(tempfile.mkdtemp(prefix='gifrb-', dir=a.scratch))
    t_end = time.time() + a.budget
    n = 0
    with cpath.open('a') as out:
        for r in rows:
            if r['sha256'] in done:
                continue
            if time.time() > t_end:
                break
            rec = {'sha256': r['sha256'], 'i': r['i']}
            for name, cmd in libs.items():
                dump = scratch / 'd.rgba'
                if dump.exists():
                    dump.unlink()
                j = run_runner(cmd + [str(root / r['path']), str(dump)], a.timeout)
                o = {'status': j.get('status')}
                if j.get('status') == 'ok':
                    o.update({'frames': j.get('frames'), 'capped': bool(j.get('frames_capped')),
                              'dims': [j.get('width'), j.get('height')], 'tail_error': j.get('tail_error')})
                    if dump.exists():
                        o['first'] = norm_rgba(dump.read_bytes())
                else:
                    o['error'] = j.get('error_class')
                rec[name] = o
            if dump.exists():
                dump.unlink()
            out.write(json.dumps(rec) + '\n')
            out.flush()
            n += 1
    for f in scratch.glob('*'):
        f.unlink()
    scratch.rmdir()
    print(f'incumbents: {n} files run this call, {len(done) + n} of {len(rows)} cached')


# ---------------------------------------------------------------- report phase

def name(r):
    return f"{r['repo']}:{r['repo_path']}"


def phase_report(a):
    root, rows = read_manifest()
    cache = Path(a.cache)
    orc = cache_load(cache / 'oracle.jsonl')
    inc = cache_load(cache / 'incumbents.jsonl')
    assert len(orc) == len(rows) == len(inc), (len(orc), len(rows), len(inc))
    prev = {f['sha256']: f for f in json.loads((HERE / 'gif-stats.json').read_text())['files_detail']}
    prev_inc = {f['sha256']: f for f in json.loads((HERE / 'gif-incumbents.json').read_text())['files']}
    recs = []
    for r in rows:
        o, i = orc[r['sha256']], inc[r['sha256']]
        b_hdr = (root / r['path']).read_bytes()[:6]
        r = dict(r, fuzz=gif_stats.is_fuzz(r), incidental=(b_hdr[:3] == b'GIF' and b_hdr[3:6] in (b'87a', b'89a')
                                                           and gif_stats.is_incidental(r)),
                 valid_header=b_hdr[:3] == b'GIF' and b_hdr[3:6] in (b'87a', b'89a'))
        recs.append((r, o, i))
    timeouts = [name(r) for r, o, _ in recs if o.get('timeout') or o.get('memory_error')]
    good = [(r, o, i) for r, o, i in recs if 'S_stream' in o]
    L = {}
    L['files'] = len(recs)
    L['oracle_failures'] = timeouts
    L['manifest_sha256'] = MANIFEST_SHA256
    L['fields'] = FIELDS

    def cnt(pred):
        return sum(1 for t in good if pred(*t))
    L['strict'] = {'stream_accept': cnt(lambda r, o, i: o['S_stream']['ok']), 'still_accept': cnt(lambda r, o, i: o['S_still']['ok'])}
    L['recommendedbeta'] = {'stream_accept': cnt(lambda r, o, i: o['RB_stream']['ok']), 'still_accept': cnt(lambda r, o, i: o['RB_still']['ok'])}
    # draft-0.1 numbers recorded in gif-stats.json for comparison
    L['strict_previous_stats'] = {'stream_accept': sum(1 for f in prev.values() if f['strict'] == 'accept'),
                                  'still_accept': sum(1 for f in prev.values() if f['still'] == 'accept')}
    L['strict_vs_previous_stats_differ'] = [name(r) for r, o, i in good
                                            if (o['S_stream']['ok'] != (prev[r['sha256']]['strict'] == 'accept'))
                                            or (o['S_still']['ok'] != (prev[r['sha256']]['still'] == 'accept'))][:50]
    L['flips'] = {'stream': cnt(lambda r, o, i: o['flip_stream']), 'still': cnt(lambda r, o, i: o['flip_still']),
                  'stream_and_still': cnt(lambda r, o, i: o['flip_stream'] and o['flip_still']),
                  'rb_rejects_what_strict_accepts_stream': cnt(lambda r, o, i: o['S_stream']['ok'] and not o['RB_stream']['ok']),
                  'rb_rejects_what_strict_accepts_still': cnt(lambda r, o, i: o['S_still']['ok'] and not o['RB_still']['ok'])}
    # per-subset accept counts
    subsets = {'all': lambda r: True, 'fuzz': lambda r: r['fuzz'], 'nonfuzz': lambda r: not r['fuzz'],
               'valid_header': lambda r: r['valid_header'], 'incidental': lambda r: r['incidental']}
    L['subsets'] = {}
    for k, f in subsets.items():
        sel = [t for t in good if f(t[0])]
        L['subsets'][k] = {'files': len(sel),
                           'strict_stream': sum(o['S_stream']['ok'] for _, o, _ in sel),
                           'rb_stream': sum(o['RB_stream']['ok'] for _, o, _ in sel),
                           'strict_still': sum(o['S_still']['ok'] for _, o, _ in sel),
                           'rb_still': sum(o['RB_still']['ok'] for _, o, _ in sel)}
    def ok(i, k, lvl='still'):
        """Incumbent acceptance. Whole-stream level: the `image` runner also records `tail_error`, an error after its
        first frame, which counts as a rejection; gdk-pixbuf's two paths report any error in the file."""
        if i[k]['status'] != 'ok':
            return False
        return not (lvl == 'stream' and k == 'image' and i[k].get('tail_error'))

    def same_first(o, i, k):
        return (o['RB_still']['ok'] and i[k]['status'] == 'ok' and 'first' in i[k]
                and i[k]['first'] == o['RB_still']['rgba'] and i[k]['dims'] == o['RB_still']['dims'])
    # flips: Strict first error crosstab, fields fired (ablation)
    for kind in ('stream', 'still'):
        flips = [t for t in good if t[1]['flip_' + kind]]
        by_field = defaultdict(list)
        replaced = defaultdict(Counter)
        unexplained, multi = [], []
        strict_err = Counter()
        supp = defaultdict(Counter)
        for r, o, i in flips:
            ab = o['ablate'][kind]
            strict_err[o['S_' + kind]['error']] += 1
            if not ab:
                unexplained.append(name(r))
            if any('+' in k for k in ab):
                multi.append((name(r), sorted(ab)))
            for f, e in ab.items():
                by_field[f].append(r)
                replaced[f][e] += 1
                supp[f]['files'] += 1
                for k in ('gdk_anim', 'gdk_forced', 'image'):
                    supp[f][k + '_accepts'] += ok(i, k, kind)
                    supp[f][k + '_first_frame_equal_to_rb'] += same_first(o, i, k)
        L['flip_' + kind] = {
            'n': len(flips), 'strict_first_error': dict(strict_err.most_common()),
            'unexplained_by_single_or_pair': unexplained, 'pair_only': multi,
            'fields': {f: {'files': len(by_field[f]), 'examples': [name(x) for x in by_field[f][:3]],
                           'replaced_errors': dict(replaced[f].most_common()),
                           'non_fuzz_files': sum(1 for x in by_field[f] if not x['fuzz']),
                           'incidental_files': sum(1 for x in by_field[f] if x['incidental']),
                           'incumbents': dict(supp[f])}
                       for f in sorted(by_field, key=lambda k: -len(by_field[k]))},
            'files_by_number_of_fields': dict(Counter(len(o['ablate'][kind]) for _, o, _ in flips)),
        }
    # recovered events
    ev = {}
    for kind in ('stream', 'still'):
        acc = [t for t in good if t[1]['RB_' + kind]['ok']]
        if kind == 'stream':
            files_rec = [t for t in acc if t[1]['RB_stream']['rec_frames'] > 0]
            ev_c = Counter(e for t in files_rec for e in t[1]['RB_stream']['events'])
            ev_ex = {e: [name(t[0]) for t in files_rec if e in t[1]['RB_stream']['events']][:3] for e in ev_c}
            frames_rec = sum(t[1]['RB_stream']['rec_frames'] for t in files_rec)
            ev[kind] = {'accepted': len(acc), 'files_with_recovered': len(files_rec), 'recovered_frames': frames_rec,
                        'events_by_files': dict(ev_c), 'examples': ev_ex,
                        'incidental_files': sum(1 for t in files_rec if t[0]['incidental']),
                        'nonfuzz_files': sum(1 for t in files_rec if not t[0]['fuzz'])}
        else:
            files_rec = [t for t in acc if t[1]['RB_still']['recovered']]
            ev[kind] = {'accepted': len(acc), 'files_with_recovered': len(files_rec),
                        'examples': [name(t[0]) for t in files_rec[:3]]}
    # frames before an error also carry events
    pre = [t for t in good if not t[1]['RB_stream']['ok'] and t[1]['RB_stream']['rec_frames'] > 0]
    ev['rejected_streams_with_recovered_frames_before_error'] = len(pre)
    L['recovered'] = ev
    # still rejects
    for kind in ('stream', 'still'):
        rej = [t for t in good if not t[1]['RB_' + kind]['ok']]
        errs = Counter(t[1]['RB_' + kind]['error'] for t in rej)
        L['rb_rejects_' + kind] = {
            'n': len(rej), 'first_error_top15': [[e, c, [name(t[0]) for t in rej if t[1]['RB_' + kind]['error'] == e][:3]]
                                                 for e, c in errs.most_common(15)],
            'distinct_errors': len(errs),
            'first_error_all': dict(errs.most_common()),
            'non_fuzz': sum(1 for t in rej if not t[0]['fuzz']),
            'non_fuzz_valid_header': sum(1 for t in rej if not t[0]['fuzz'] and t[0]['valid_header']),
            'no_signature': sum(1 for t in rej if not t[0]['valid_header']),
            'incidental': [[name(t[0]), t[1]['RB_' + kind]['error']] for t in rej if t[0]['incidental']]}
    # obligation R
    viol = [[name(r), o['R_violations']] for r, o, i in good if o['R_violations']]
    L['obligation_R'] = {'strict_accepted_stills': cnt(lambda r, o, i: o['S_still']['ok']),
                         'strict_accepted_streams': cnt(lambda r, o, i: o['S_stream']['ok']),
                         'strict_frames_checked': sum(o['R_frames_checked'] for _, o, _ in good),
                         'violations': len(viol), 'violating_files': viol}
    # incumbents
    L['incumbents'] = {}
    L['incumbents_vs_previous_json'] = {}
    for k in ('gdk_anim', 'gdk_forced', 'image'):
        d = {'ok_first_frame': sum(ok(i, k) for _, _, i in good), 'ok_whole_stream': sum(ok(i, k, 'stream') for _, _, i in good)}
        d['image_tail_error'] = sum(1 for _, _, i in good if k == 'image' and i[k]['status'] == 'ok' and i[k].get('tail_error'))
        d['capped_at_64_frames'] = sum(1 for _, _, i in good if i[k]['status'] == 'ok' and i[k].get('capped'))
        for lvl in ('stream', 'still'):
            both = [t for t in good if t[1]['RB_' + lvl]['ok'] and ok(t[2], k, lvl)]
            only_rb = [t for t in good if t[1]['RB_' + lvl]['ok'] and not ok(t[2], k, lvl)]
            only_inc = [t for t in good if not t[1]['RB_' + lvl]['ok'] and ok(t[2], k, lvl)]
            neither = len(good) - len(both) - len(only_rb) - len(only_inc)
            d[lvl] = {'both_accept': len(both), 'rb_only': len(only_rb), 'incumbent_only': len(only_inc), 'neither': neither,
                      'rb_only_examples': [[name(t[0]), t[1]['RB_stream']['error'] if False else None] for t in only_rb[:0]],
                      'rb_only_by_strict_error': dict(Counter(t[1]['S_' + lvl]['error'] or 'accept' for t in only_rb).most_common(12)),
                      'incumbent_only_by_rb_error': dict(Counter(t[1]['RB_' + lvl]['error'] for t in only_inc).most_common(12)),
                      'incumbent_only_examples': [name(t[0]) for t in only_inc[:5]],
                      'rb_only_examples': [name(t[0]) for t in only_rb[:5]]}
        # first-frame agreement: RecommendedBeta still accepted and incumbent ok
        both = [t for t in good if t[1]['RB_still']['ok'] and ok(t[2], k) and 'first' in t[2][k]]
        eq = [t for t in both if t[2][k]['first'] == t[1]['RB_still']['rgba'] and t[2][k]['dims'] == t[1]['RB_still']['dims']]
        d['first_frame'] = {'compared': len(both), 'equal': len(eq), 'differ': len(both) - len(eq),
                            'differ_files': [name(t[0]) for t in both if t not in eq]}
        # and restricted to files both accept as whole streams
        both_s = [t for t in both if t[1]['RB_stream']['ok'] and ok(t[2], k, 'stream')]
        eq_s = [t for t in both_s if t in eq]
        d['first_frame_whole_stream_both'] = {'compared': len(both_s), 'equal': len(eq_s)}
        L['incumbents'][k] = d
        # consistency with the earlier gif-incumbents.json (same runners)
        n_same = n_chk = 0
        diff = []
        for r, o, i in good:
            p = prev_inc[r['sha256']][k]
            n_chk += 1
            same = (p.get('status') == i[k]['status']) and p.get('first') == i[k].get('first')
            n_same += same
            if not same:
                diff.append(name(r))
        L['incumbents_vs_previous_json'][k] = {'same': n_same, 'checked': n_chk, 'differ': diff[:10]}
    L['incumbents']['rb_and_gdk_anim_and_image_accept_stream'] = cnt(
        lambda r, o, i: o['RB_stream']['ok'] and ok(i, 'gdk_anim', 'stream') and ok(i, 'image', 'stream'))
    L['incumbents']['rb_stream_accept_and_both_gdk_anim_and_image_reject'] = [name(r) for r, o, i in good if
        o['RB_stream']['ok'] and not ok(i, 'gdk_anim', 'stream') and not ok(i, 'image', 'stream')]
    L['incumbents']['rb_still_accept_and_both_gdk_anim_and_image_reject'] = [name(r) for r, o, i in good if
        o['RB_still']['ok'] and not ok(i, 'gdk_anim') and not ok(i, 'image')]
    L['incumbents']['rb_stream_reject_and_both_gdk_anim_and_image_accept'] = [name(r) for r, o, i in good if
        not o['RB_stream']['ok'] and ok(i, 'gdk_anim', 'stream') and ok(i, 'image', 'stream')]
    # incidental subset listing: every incidental file Strict rejects (stream), with outcomes
    inc = [t for t in good if t[0]['incidental']]
    L['incidental_strict_rejected'] = [{
        'name': name(t[0]), 'strict_error': t[1]['S_stream']['error'], 'rb_stream': 'accept' if t[1]['RB_stream']['ok'] else t[1]['RB_stream']['error'],
        'rb_still': 'accept' if t[1]['RB_still']['ok'] else t[1]['RB_still']['error'],
        'fields': sorted(t[1]['ablate'].get('stream', {})), 'recovered_events': t[1]['RB_stream']['events'],
        'gdk_anim': t[2]['gdk_anim']['status'], 'image': t[2]['image']['status'],
        'image_tail_error': t[2]['image'].get('tail_error')}
        for t in inc if not t[1]['S_stream']['ok']]
    L['timing'] = {'oracle_seconds_total': round(sum(o.get('seconds', 0) for _, o, _ in good), 1),
                   'slowest': sorted([[o['seconds'], name(r)] for r, o, _ in good], reverse=True)[:3]}
    per_file = []
    for r, o, i in good:
        per_file.append({'name': name(r), 'sha256': r['sha256'], 'size': r['size'], 'fuzz': r['fuzz'], 'incidental': r['incidental'],
                         'strict_stream': 'accept' if o['S_stream']['ok'] else o['S_stream']['error'],
                         'strict_still': 'accept' if o['S_still']['ok'] else o['S_still']['error'],
                         'rb_stream': 'accept' if o['RB_stream']['ok'] else o['RB_stream']['error'],
                         'rb_still': 'accept' if o['RB_still']['ok'] else o['RB_still']['error'],
                         'frames': o['RB_stream']['frames'], 'fields_stream': sorted(o['ablate'].get('stream', {})),
                         'fields_still': sorted(o['ablate'].get('still', {})),
                         'recovered_events': o['RB_stream']['events'], 'recovered_still': bool(o['RB_still'].get('recovered')),
                         'gdk_anim': i['gdk_anim']['status'], 'gdk_forced': i['gdk_forced']['status'], 'image': i['image']['status'],
                         'image_tail_error': i['image'].get('tail_error')})
    Path(a.out_json).write_text(json.dumps({'summary': L, 'files': per_file}, indent=1, sort_keys=True) + '\n')
    print(json.dumps(L, indent=1, sort_keys=True)[:12000])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('phase', choices=['oracle', 'incumbents', 'report'])
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--timeout', type=float, default=300.0)
    ap.add_argument('--budget', type=float, default=1e9, help='stop starting new files after this many seconds')
    ap.add_argument('--bin', default='/tmp/claude-0/leniency-gifpng-bin')
    ap.add_argument('--cache', default='/tmp/claude-0/gif-rb-pass-cache')
    ap.add_argument('--scratch', default='/tmp/claude-0')
    ap.add_argument('--out-json', default=str(HERE / 'gif-recommendedbeta-pass.json'))
    a = ap.parse_args()
    {'oracle': phase_oracle, 'incumbents': phase_incumbents, 'report': phase_report}[a.phase](a)


if __name__ == '__main__':
    main()
