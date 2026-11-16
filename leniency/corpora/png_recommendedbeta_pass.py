#!/usr/bin/env python3
"""PNG-Q7 corpus pass: run Strict and RecommendedBeta (png/policy-draft/policy_oracle.py, draft 0.2) and the
incumbent decoders over the pinned PNG corpus.  Mirrors gif_recommendedbeta_pass.py.

  python3 -I png_recommendedbeta_pass.py oracle     [--limit N] [--timeout S] [--max-px N] [--workers 2] [--cache DIR] [--budget S]
  python3 -I png_recommendedbeta_pass.py incumbents [--limit N] [--timeout S] [--cache DIR] [--bin DIR]
  python3 -I png_recommendedbeta_pass.py report     [--cache DIR] [--out-json png-recommendedbeta-pass.json]

Reads $CORPUS_DIR/png/MANIFEST.tsv (default ~/corpora; its SHA-256 must equal MANIFEST_SHA256) and files/<sha256>.png.
Each phase is resumable: it appends one JSON line per file to <cache>/oracle.jsonl or <cache>/incumbents.jsonl and
skips files already there.  The cache lives under ~/corpora/png/pass-cache (outside every repository); the report
writes only file names, hashes, error names and numbers, never file bytes.

oracle phase, per file, for Strict and RecommendedBeta (policy_oracle.PROFILES):
  still  = decode_still; stream = decode_frames, accepted when `end == 'iend'` (every frame decoded).  A file without
  APNG chunks gets its stream outcome from the still outcome (`stream_derived`), because decode_frames then returns the
  default image as the single frame after the same walk; this halves the cost of the pure-Python oracle.
  obligation R in three forms (as validate.py): still, stream, prefix;
  for every file where RecommendedBeta accepts and Strict rejects (a "flip"), ablation: revert one of RecommendedBeta's
  fifteen non-Strict fields at a time to its Strict value and record whether the outcome changes and which error appears
  instead (the error that field replaced); if no single field explains the flip, pairs are tried.
  Files whose Strict error is a resource cap (InvalidDimensions, OutputTooLarge) are run again with the caps raised
  (MAX_DIMENSION 2^31-1, MAX_OUTPUT 2^40) so the oracle can say what RecommendedBeta would do at the dimensions the owner
  intends to allow; those results are kept apart ('raised_caps').
  Tiers (the pure-Python oracle costs about 1.9 s per megapixel per decode): files up to --max-px (default 1,000,000
  pixels) always run; files above it up to 2^24 pixels (the area cap) run if png-stats.json flags them (any anomaly flag,
  APNG, 16-bit) or they fall in a deterministic 1-in-SAMPLE hash sample (first byte of the SHA-256 modulo SAMPLE, default
  8); the others are recorded as `not_sampled`.  Files above 2^24 pixels (and, up to 2^27 pixels, files with a Strict cap
  error) get only the raised-cap run, still outcomes only, no frames and no ablation unless the still flips.
incumbents phase: runners from png/policy-draft/probes (run_probes.RUNNERS): image-still (image 0.25.10 still path),
  image (image's frame loop), libpng 1.6.48 (png-native), gdk-loader (gdk-pixbuf 2.42.12 incremental loader),
  gdk-file (gdk_pixbuf_new_from_file) and glycin's loader code when its binary exists.  RGBA8 first frame hash.
report phase: joins both caches with the manifest and png-stats.json and writes the JSON summary.
"""
import argparse
import hashlib
import json
import os
import resource
import signal
import struct
import sys
import time
from collections import Counter, defaultdict
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PNG = ROOT / 'png'
for p in (PNG / 'policy-draft', PNG / 'spec-draft', PNG / 'policy-draft' / 'probes', HERE):
    sys.path.insert(0, str(p))
import png_oracle as O  # noqa: E402
import policy_oracle as PO  # noqa: E402
import png_stats  # noqa: E402  (classify(): the class definitions of the statistics)

MANIFEST_SHA256 = 'f73984e8c5f30fb959d6fae697ad267be4da3431330714ba111e2fd9baf62996'
STRICT, RB = PO.STRICT, PO.RECOMMENDED_BETA
FIELDS = [k for k in STRICT if RB[k] != STRICT[k]]
assert len(FIELDS) == 15, FIELDS
IGNORE = ('recovered', 'events')
CAP_ERRORS = {'InvalidDimensions', 'OutputTooLarge'}
RUNNERS = ['image-still', 'image', 'libpng', 'gdk-loader', 'gdk-file', 'glycin']
AS_KIB = 16 << 20
RAISED_MAX_PX = 1 << 27     # 134 Mpx: the raised-cap oracle run is not attempted above this
SIG = b'\x89PNG\r\n\x1a\n'


class Timeout(Exception):
    pass


def _alarm(signum, frame):
    raise Timeout()


def corpus_root():
    return Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'png'


def read_manifest(check=True):
    root = corpus_root()
    raw = (root / 'MANIFEST.tsv').read_bytes()
    if check and MANIFEST_SHA256:
        assert hashlib.sha256(raw).hexdigest() == MANIFEST_SHA256, 'manifest differs from the pinned one'
    rows = [l.split('\t') for l in raw.decode().splitlines()[1:]]
    return root, [{'i': i, 'path': r[0], 'sha256': r[1], 'repo': r[4], 'repo_path': r[6], 'size': int(r[7]),
                   'kind': r[10] if len(r) > 10 else 'mixed'} for i, r in enumerate(rows)]


def header(b):
    """(w, h, has_apng_chunks) by a light chunk walk; w, h None if no readable IHDR."""
    if not b.startswith(SIG) or len(b) < 33 or b[12:16] != b'IHDR':
        return None, None, False
    w, h = struct.unpack('>II', b[16:24])
    p, apng = 8, False
    while p + 8 <= len(b):
        ln = struct.unpack('>I', b[p:p + 4])[0]
        t = b[p + 4:p + 8]
        if t in (b'acTL', b'fcTL', b'fdAT'):
            apng = True
            break
        if t in (b'IEND',):
            break
        p += 12 + ln
    return w, h, apng


# ---------------------------------------------------------------- oracle phase

def frame_digest(f):
    h = hashlib.sha256()
    h.update(repr(tuple((k, f[k]) for k in sorted(f) if k not in IGNORE + ('rgba',))).encode())
    h.update(f['rgba'])
    return h.hexdigest()


def stream_of(b, P):
    res = PO.decode_frames(b, P)
    if 'error' in res:
        return {'ok': False, 'error': res['error'], 'ctor': True, 'frames': 0, 'dig': [], 'rec_frames': 0, 'events': [],
                'whole': None, 'rec': []}
    fr = res['frames']
    end = res['end']
    ok = end == 'iend'
    dig = [frame_digest(f) for f in fr]
    ev = sorted({e for f in fr for e in f['events']})
    whole = hashlib.sha256(json.dumps([res['width'], res['height'], res['num_plays'], dig, ok]).encode()).hexdigest()
    return {'ok': ok, 'error': None if ok else end['error'], 'ctor': False, 'frames': len(fr), 'dig': dig,
            'rec_frames': sum(1 for f in fr if f['recovered']), 'events': ev, 'whole': whole,
            'rec': [bool(f['recovered']) for f in fr]}


def still_of(b, P):
    res = PO.decode_still(b, P)
    if 'error' in res:
        return {'ok': False, 'error': res['error']}
    return {'ok': True, 'error': None, 'dims': [res['width'], res['height']],
            'rgba': hashlib.sha256(res['rgba']).hexdigest(), 'recovered': bool(res['recovered']),
            'events': res['events']}


TARGETED = {'TrailingBytes': ['trailing'], 'ChunkOutOfOrder': ['ancillary_order', 'plte_unexpected', 'apng_in_still', 'apng_errors'],
            'DuplicateChunk': ['ancillary_duplicate', 'apng_in_still', 'apng_errors'], 'CrcMismatch': ['crc_ancillary'],
            'PaletteIndexOutOfRange': ['palette_index'], 'TrailingZlibData': ['zlib_tail'], 'DistanceTooFar': ['window'],
            'MissingIend': ['missing_iend', 'iend'], 'InvalidPalette': ['palette_size'], 'InvalidTransparency': ['trns_invalid']}


def ablate(b, kind, base, strict_error=None, targeted=False):
    """One-field-at-a-time reversion of RecommendedBeta; {field: replaced error or 'DIFFERS'}."""
    out = {}

    def run(P):
        return stream_of(b, P) if kind == 'stream' else still_of(b, P)

    def same(s):
        if s['ok'] and base['ok']:
            return (s['whole'] == base['whole']) if kind == 'stream' else (s['rgba'] == base['rgba'] and s['recovered'] == base['recovered'])
        return s['ok'] == base['ok'] and s['error'] == base['error']

    if targeted and strict_error:
        # large files (above 1 Mpx): try only the fields that can raise Strict's first error; if one of them explains the
        # flip, stop (a second firing field would be missed; recorded as ablation_mode targeted)
        cands = TARGETED.get(strict_error) or (['ancillary_values'] if strict_error.startswith('Invalid') else [])
        for f in cands:
            s = run(dict(RB, **{f: STRICT[f]}))
            if not same(s):
                out[f] = s['error'] or 'DIFFERS'
        if out:
            out['_targeted'] = True
            return out
    for f in FIELDS:
        s = run(dict(RB, **{f: STRICT[f]}))
        if not same(s):
            out[f] = s['error'] or 'DIFFERS'
    if not out:
        for i, f in enumerate(FIELDS):
            for g in FIELDS[i + 1:]:
                s = run(dict(RB, **{f: STRICT[f], g: STRICT[g]}))
                if s['ok'] != base['ok']:
                    out[f + '+' + g] = s['error'] or 'DIFFERS'
    return out


def analyse(b, light=False):
    t0 = time.time()
    w, h, apng = header(b)
    rec = {'w': w, 'h': h, 'apng_chunks': apng}
    S_sl, R_sl = still_of(b, STRICT), still_of(b, RB)
    if apng and not light:
        S_st, R_st = stream_of(b, STRICT), stream_of(b, RB)
        rec['stream_derived'] = False
    else:
        # one default-image frame after the same walk
        S_st = {'ok': S_sl['ok'], 'error': S_sl['error'], 'frames': 1 if S_sl['ok'] else 0, 'dig': [S_sl.get('rgba')] if S_sl['ok'] else [],
                'rec': [S_sl.get('recovered')] if S_sl['ok'] else [], 'rec_frames': int(bool(S_sl.get('recovered'))),
                'events': S_sl.get('events', []), 'whole': S_sl.get('rgba')}
        R_st = {'ok': R_sl['ok'], 'error': R_sl['error'], 'frames': 1 if R_sl['ok'] else 0, 'dig': [R_sl.get('rgba')] if R_sl['ok'] else [],
                'rec': [R_sl.get('recovered')] if R_sl['ok'] else [], 'rec_frames': int(bool(R_sl.get('recovered'))),
                'events': R_sl.get('events', []), 'whole': R_sl.get('rgba')}
        rec['stream_derived'] = True
        rec['light'] = light
    viol = []
    if S_sl['ok'] and not (R_sl['ok'] and R_sl['rgba'] == S_sl['rgba'] and not R_sl['recovered']):
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
        rec[tag + '_stream'] = {k: v for k, v in s.items() if k not in ('dig', 'rec', 'whole')}
    rec['S_still'] = S_sl
    rec['RB_still'] = R_sl
    rec['flip_stream'] = (not S_st['ok']) and R_st['ok']
    rec['flip_still'] = (not S_sl['ok']) and R_sl['ok']
    rec['ablate'] = {}
    targeted = bool(w and h and w * h > 1_000_000)
    if rec['flip_still']:
        rec['ablate']['still'] = ablate(b, 'still', R_sl, S_sl['error'], targeted)
    if rec['flip_stream']:
        if rec['stream_derived']:
            rec['ablate']['stream'] = rec['ablate'].get('still') or ablate(b, 'still', R_sl, S_sl['error'], targeted)
        else:
            rec['ablate']['stream'] = ablate(b, 'stream', R_st, S_st['error'], targeted)
    rec['seconds'] = round(time.time() - t0, 2)
    return rec


def work(args):
    """Runs in a worker process.  mode: 'full' (normal caps), 'raised' (the file is over the caps or the area ceiling:
    only the raised-cap run, still outcomes only), 'skip' (not sampled)."""
    path, mode, timeout = args
    signal.signal(signal.SIGALRM, _alarm)
    resource.setrlimit(resource.RLIMIT_AS, (64 << 30, 64 << 30))
    if mode == 'skip':
        return {'skipped': 'not_sampled'}
    b = Path(path).read_bytes()
    w, h, _ = header(b)
    out = {}
    base_caps = (O.MAX_DIMENSION, O.MAX_OUTPUT)
    try:
        if mode == 'raised':
            out['skipped'] = 'area_over_16.7Mpx'
        else:
            signal.alarm(int(timeout))
            try:
                out.update(analyse(b))
            except Timeout:
                out['timeout'] = True
            except MemoryError:
                out['memory_error'] = True
            finally:
                signal.alarm(0)
        s = out.get('S_still', {}).get('error')
        if w is not None and (out.get('skipped') == 'area_over_16.7Mpx' or s in CAP_ERRORS) and w * h <= RAISED_MAX_PX:
            O.MAX_DIMENSION, O.MAX_OUTPUT = (1 << 31) - 1, 1 << 40
            signal.alarm(int(timeout) * 3)
            try:
                r = analyse(b, light=True)
                r['caps'] = 'raised'
                out['raised_caps'] = r
            except Timeout:
                out['raised_caps'] = {'timeout': True}
            except MemoryError:
                out['raised_caps'] = {'memory_error': True}
            finally:
                signal.alarm(0)
                O.MAX_DIMENSION, O.MAX_OUTPUT = base_caps
    except Exception as e:  # never lose a file to a crash in the oracle: record it
        out['exception'] = f'{type(e).__name__}: {str(e)[:120]}'
    return out


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


def cache_load_all(cache, stem):
    done = {}
    for p in sorted(cache.glob(stem + '*.jsonl')):
        done.update(cache_load(p))
    return done


def area_of(root, r):
    with (root / r['path']).open('rb') as f:
        b = f.read(64)
    if b.startswith(SIG) and len(b) >= 24 and b[12:16] == b'IHDR':
        w, h = struct.unpack('>II', b[16:24])
        return w * h
    return 0


def phase_oracle(a):
    root, rows = read_manifest(not a.no_pin)
    cache = Path(a.cache)
    cache.mkdir(parents=True, exist_ok=True)
    shard_k, shard_n = (int(x) for x in a.shard.split('/'))
    cpath = cache / ('oracle.zfix.jsonl' if a.force_thin else 'oracle.jsonl' if shard_n == 1 else f'oracle.shard{shard_k}of{shard_n}.jsonl')
    done = cache_load_all(cache, 'oracle')
    if a.limit:
        rows = rows[:a.limit]
    flags = {f['sha256']: set(f['flags']) | ({'apng'} if f['apng'] else set()) | ({'depth16'} if f['depth'] == 16 else set())
             for f in json.loads((HERE / 'png-stats.json').read_text())['files_detail']} if (HERE / 'png-stats.json').exists() else {}
    CLEAN_IGNORE = {'unknown_ancillary_chunk', 'duplicate_single_chunk', 'iccp_and_srgb'}
    areas = {r['sha256']: area_of(root, r) for r in rows}
    # over-cap files first (few, slow, the ones the owner asked about), then ascending area
    rows = sorted(rows, key=lambda r: (areas[r['sha256']] <= 1 << 24, areas[r['sha256']], r['size']))
    def dims(r):
        with (root / r['path']).open('rb') as fh:
            hb = fh.read(24)
        return struct.unpack('>II', hb[16:24]) if hb.startswith(SIG) and len(hb) >= 24 and hb[12:16] == b'IHDR' else (0, 0)

    def thin_or_long(r):
        w_, h_ = dims(r)
        return bool(w_ and h_ and (max(w_, h_) > 16384 or (max(w_, h_) >= 1024 and max(w_, h_) / min(w_, h_) >= 32)))

    if a.force_thin:
        # records written as not_sampled for files that carry a per-side cap or long-thin question are redone (cheap: small area)
        todo = [r for r in rows if done.get(r['sha256'], {}).get('skipped') == 'not_sampled' and thin_or_long(r)]
    else:
        todo = [r for r in rows if r['sha256'] not in done and int(r['sha256'][-4:], 16) % shard_n == shard_k]

    def mode_of(r):
        ar = areas[r['sha256']]
        if a.force_thin:
            return 'full'
        if ar <= a.max_px:
            return 'full'
        if ar <= 1 << 24:
            fl = flags.get(r['sha256'], {'unknown'})
            if (fl - CLEAN_IGNORE - {'unknown_ancillary_chunk'}) or int(r['sha256'][:2], 16) % a.sample == 0:
                return 'full'
            return 'skip'
        return 'raised'
    t_end = time.time() + a.budget
    n = 0
    with cpath.open('a') as out, ProcessPoolExecutor(a.workers) as ex:
        it = iter(todo)
        pending = []
        while True:
            while len(pending) < a.workers * 2 and time.time() < t_end:
                r = next(it, None)
                if r is None:
                    break
                pending.append((r, ex.submit(work, (str(root / r['path']), mode_of(r), a.timeout))))
            if not pending:
                break
            r, fut = pending.pop(0)
            rec = fut.result()
            rec.update({'sha256': r['sha256'], 'i': r['i']})
            out.write(json.dumps(rec) + '\n')
            out.flush()
            n += 1
            if n % 200 == 0:
                print(n, 'of', len(todo), flush=True)
    print(f'oracle: {n} files analysed this call, {len(done) + n} of {len(rows)} cached')


# ---------------------------------------------------------------- incumbents phase

def phase_incumbents(a):
    import run_probes as RP
    root, rows = read_manifest(not a.no_pin)
    cache = Path(a.cache)
    cache.mkdir(parents=True, exist_ok=True)
    cpath = cache / 'incumbents.jsonl'
    done = cache_load(cpath)
    if a.limit:
        rows = rows[:a.limit]
    runners = [r for r in RUNNERS if Path(str(RP.RUNNERS[r][0])).exists()]
    missing = [r for r in RUNNERS if r not in runners]
    if missing:
        print('runners not available:', missing, file=sys.stderr)
    todo = [r for r in rows if r['sha256'] not in done]
    t_end = time.time() + a.budget
    n = 0

    def one(r):
        rec = {'sha256': r['sha256'], 'i': r['i']}
        for k in runners:
            rec[k] = RP.norm(RP.invoke(root / r['path'], RP.RUNNERS[k], a.timeout, AS_KIB))
        return rec

    with cpath.open('a') as out:
        from concurrent.futures import ThreadPoolExecutor
        with ThreadPoolExecutor(2) as ex:
            for i in range(0, len(todo), 40):
                if time.time() > t_end:
                    break
                for rec in ex.map(one, todo[i:i + 40]):
                    rec['runners'] = runners
                    out.write(json.dumps(rec) + '\n')
                    n += 1
                out.flush()
                if (i // 40) % 10 == 0:
                    print(n, 'of', len(todo), flush=True)
    print(f'incumbents: {n} files run this call, {len(done) + n} of {len(rows)} cached')


def phase_incumbents_big(a):
    """Re-run the three C runners on files whose result was cut short by the runners' own 16,000,000-pixel harness limit
    ('outside harness limit'), with copies of the runners built with the limits raised to 2,000,000,000 pixels (gdk.c: the
    same literal; acc.h: ACC_MAX_PIXELS and ACC_MAX_TOTAL).  The copies live under /tmp/claude-0/png-probe-bin-big; the
    sources are the unmodified runner sources with those constants changed by sed (see the README).  Records get
    `harness_limit_raised: true`."""
    import run_probes as RP
    root, rows = read_manifest(not a.no_pin)
    cache = Path(a.cache)
    inc = cache_load_all(cache, 'incumbents')
    big = Path('/tmp/claude-0/png-probe-bin-big')
    spec = {'libpng': (big / 'png-native', ['libpng']), 'gdk-loader': (big / 'gdk-anim', ['png', 'forced-whole']),
            'gdk-file': (big / 'gdk-formats', ['png', 'file'])}
    todo = [r for r in rows if any('harness limit' in str(inc[r['sha256']].get(k, {}).get('err', '')) for k in spec)
            and not inc[r['sha256']].get('harness_limit_raised')]
    n = 0
    with (cache / 'incumbents.zbig.jsonl').open('a') as out:
        for r in todo:
            rec = dict(inc[r['sha256']])
            for k, sp in spec.items():
                rec[k] = RP.norm(RP.invoke(root / r['path'], sp, a.timeout, AS_KIB))
            rec['harness_limit_raised'] = True
            out.write(json.dumps(rec) + '\n')
            out.flush()
            n += 1
    print(f'incumbents-big: {n} files re-run')


# ---------------------------------------------------------------- report phase

def name(r):
    return f"{r['repo']}:{r['repo_path']}"


def phase_report(a):
    root, rows = read_manifest(not a.no_pin)
    cache = Path(a.cache)
    orc = cache_load_all(cache, 'oracle')
    inc = cache_load_all(cache, 'incumbents')
    for o in orc.values():
        for kind, ab in o.get('ablate', {}).items():
            if ab.pop('_targeted', False):
                o.setdefault('targeted', []).append(kind)
    stats = {f['sha256']: f for f in json.loads((HERE / 'png-stats.json').read_text())['files_detail']}
    holdout = png_stats.load_holdout()
    if a.partial:
        rows = [r for r in rows if r['sha256'] in orc and r['sha256'] in inc]
    for r in rows:
        if r['sha256'] not in orc and a.allow_unreached:
            orc[r['sha256']] = {'skipped': 'not_reached'}
    missing = [r for r in rows if r['sha256'] not in orc or r['sha256'] not in inc or r['sha256'] not in stats]
    assert not missing, f'{len(missing)} files lack oracle, incumbent or stats records, e.g. {name(missing[0])}'
    recs = []
    for r in rows:
        r = dict(r, cls=png_stats.classify(r['kind'], r['repo_path']), holdout=r['repo'] in holdout)
        recs.append((r, orc[r['sha256']], inc[r['sha256']], stats[r['sha256']]))
    SUBS = {'all': lambda r: True, 'incidental': lambda r: r['cls'] == 'incidental', 'test': lambda r: r['cls'] == 'test',
            'fuzz': lambda r: r['cls'] == 'fuzz', 'non-fuzz': lambda r: r['cls'] != 'fuzz',
            'development': lambda r: not r['holdout'], 'holdout': lambda r: r['holdout']}

    def by_sub(pred, items=None):
        items = recs if items is None else items
        return {k: sum(1 for t in items if f(t[0]) and pred(*t)) for k, f in SUBS.items()}

    def ran(o):
        return 'S_still' in o

    def capped(o):
        return ran(o) and o['S_still'].get('error') in CAP_ERRORS

    L = {'files': len(recs), 'manifest_sha256': hashlib.sha256((root / 'MANIFEST.tsv').read_bytes()).hexdigest(),
         'fields': FIELDS, 'policy_oracle_sha256': hashlib.sha256((PNG / 'policy-draft' / 'policy_oracle.py').read_bytes()).hexdigest(),
         'png_oracle_sha256': hashlib.sha256((PNG / 'spec-draft' / 'png_oracle.py').read_bytes()).hexdigest()}
    L['subset_sizes'] = by_sub(lambda r, o, i, s: True)
    # ---- coverage
    cov = Counter()
    for r, o, i, s in recs:
        if o.get('timeout'):
            cov['timeout'] += 1
        elif o.get('memory_error'):
            cov['memory_error'] += 1
        elif o.get('exception'):
            cov['exception'] += 1
        elif o.get('skipped'):
            cov['skipped_' + o['skipped']] += 1
        else:
            cov['ran'] += 1
        if 'raised_caps' in o:
            rc = o['raised_caps']
            cov['raised_caps_' + ('timeout' if rc.get('timeout') else 'memory_error' if rc.get('memory_error') else 'ran')] += 1
    L['coverage'] = dict(cov)
    L['coverage']['strict_cap_rejects'] = sum(1 for t in recs if capped(t[1]))
    L['coverage']['strict_cap_error_names'] = dict(Counter(t[1]['S_still']['error'] for t in recs if capped(t[1])))
    L['coverage']['oracle_failures'] = [name(r) for r, o, i, s in recs if o.get('timeout') or o.get('memory_error') or o.get('exception')]
    L['coverage']['skipped_files'] = [[name(r), s['w'], s['h']] for r, o, i, s in recs if o.get('skipped')]
    good = [t for t in recs if ran(t[1])]
    # ---- accept counts
    L['accept'] = {}
    for tag, key in (('strict_still', ('S_still',)), ('rb_still', ('RB_still',)), ('strict_stream', ('S_stream',)), ('rb_stream', ('RB_stream',))):
        L['accept'][tag] = by_sub(lambda r, o, i, s, key=key: ran(o) and o[key[0]]['ok'], recs)
    L['accept']['ran'] = by_sub(lambda r, o, i, s: ran(o), recs)
    L['accept']['cap_rejects_both'] = by_sub(lambda r, o, i, s: capped(o), recs)
    # ---- first errors
    def err_table(key):
        out = {}
        for k, f in SUBS.items():
            c = Counter(o[key]['error'] for r, o, i, s in good if f(r) and not o[key]['ok'])
            out[k] = dict(c.most_common())
        return out
    L['strict_first_error_still'] = err_table('S_still')
    L['rb_first_error_still'] = err_table('RB_still')
    L['strict_first_error_stream'] = err_table('S_stream')
    L['rb_first_error_stream'] = err_table('RB_stream')
    # ---- flips and fields
    L['flips'] = {'still': by_sub(lambda r, o, i, s: o.get('flip_still', False)),
                  'stream': by_sub(lambda r, o, i, s: o.get('flip_stream', False)),
                  'rb_rejects_what_strict_accepts_still': by_sub(lambda r, o, i, s: ran(o) and o['S_still']['ok'] and not o['RB_still']['ok']),
                  'rb_rejects_what_strict_accepts_stream': by_sub(lambda r, o, i, s: ran(o) and o['S_stream']['ok'] and not o['RB_stream']['ok'])}

    def okI(i, k, o=None):
        return i.get(k, {}).get('status') == 'ok'

    for kind in ('still', 'stream'):
        flips = [t for t in good if t[1]['flip_' + kind]]
        by_field = defaultdict(list)
        replaced = defaultdict(Counter)
        unexplained, pair_only = [], []
        strict_err = defaultdict(Counter)
        supp = defaultdict(Counter)
        for r, o, i, s in flips:
            ab = o['ablate'][kind]
            for k in ('all', r['cls'], 'holdout' if r['holdout'] else 'development'):
                strict_err[k][o['S_' + kind]['error']] += 1
            if not ab:
                unexplained.append(name(r))
            if any('+' in k for k in ab):
                pair_only.append([name(r), sorted(ab)])
            for f, e in ab.items():
                by_field[f].append(r)
                replaced[f][e] += 1
                for k in ('image-still', 'libpng', 'gdk-loader', 'gdk-file', 'glycin'):
                    supp[f][k + '_accepts'] += okI(i, k)
        L['flip_' + kind] = {
            'n': len(flips), 'strict_first_error': {k: dict(v.most_common()) for k, v in strict_err.items()},
            'unexplained': unexplained, 'pair_only': pair_only,
            'files_by_number_of_fields': dict(Counter(len(o['ablate'][kind]) for _, o, _, _ in flips)),
            'fields': {f: {'files': len(by_field[f]),
                           **{k: sum(1 for x in by_field[f] if SUBS[k](x)) for k in ('incidental', 'test', 'fuzz', 'development', 'holdout')},
                           'replaced_errors': dict(replaced[f].most_common()),
                           'examples': [name(x) for x in sorted(by_field[f], key=lambda x: x['cls'] == 'fuzz')[:3]],
                           'incumbents_accepting': dict(supp[f])}
                       for f in sorted(by_field, key=lambda k: -len(by_field[k]))},
            'fields_never_firing': [f for f in FIELDS if f not in by_field]}
    # ---- obligation R
    viol = [[name(r), o['R_violations']] for r, o, i, s in good if o['R_violations']]
    L['ablation_targeted_files'] = sum(1 for _, o, _, _ in good if o.get('targeted'))
    L['obligation_R'] = {'strict_accepted_stills': sum(1 for t in good if t[1]['S_still']['ok']),
                         'strict_accepted_streams': sum(1 for t in good if t[1]['S_stream']['ok']),
                         'strict_frames_checked': sum(o['R_frames_checked'] for _, o, _, _ in good),
                         'violations': len(viol), 'violating_files': viol}
    # ---- recovered events among RB accepted
    ev = {}
    for kind, key in (('still', 'RB_still'), ('stream', 'RB_stream')):
        c = defaultdict(lambda: Counter())
        files_ev = defaultdict(list)
        for r, o, i, s in good:
            if o[key]['ok']:
                for e in o[key].get('events', []):
                    files_ev[e].append(r)
        ev[kind] = {e: {'files': len(v), **{k: sum(1 for x in v if SUBS[k](x)) for k in ('incidental', 'test', 'fuzz', 'holdout')},
                        'examples': [name(x) for x in sorted(v, key=lambda x: x['cls'] == 'fuzz')[:3]]} for e, v in sorted(files_ev.items())}
    L['recovered_events'] = ev
    # ---- incumbents
    runners = ['image-still', 'image', 'libpng', 'gdk-loader', 'gdk-file', 'glycin']
    L['incumbents'] = {}
    for k in runners:
        if not any(k in i for _, _, i, _ in recs):
            continue
        d = {'status': dict(Counter(i.get(k, {}).get('status', 'missing') for _, _, i, _ in recs)),
             'ok_by_subset': by_sub(lambda r, o, i, s, k=k: okI(i, k))}
        for lvl, key in (('still', 'RB_still'), ('stream', 'RB_stream')):
            if lvl == 'stream' and k not in ('image', 'glycin', 'gdk-loader'):
                continue
            both = [t for t in good if t[1][key]['ok'] and okI(t[2], k)]
            only_rb = [t for t in good if t[1][key]['ok'] and not okI(t[2], k)]
            only_inc = [t for t in good if not t[1][key]['ok'] and okI(t[2], k) and not capped(t[1])]
            d[lvl] = {'both_accept': len(both), 'rb_only': len(only_rb), 'incumbent_only_excluding_cap_rejects': len(only_inc),
                      'rb_only_by_strict_error': dict(Counter(t[1]['S_' + lvl]['error'] or 'accept' for t in only_rb).most_common(12)),
                      'incumbent_only_by_rb_error': dict(Counter(t[1][key]['error'] for t in only_inc).most_common(12)),
                      'rb_only_by_class': dict(Counter(t[0]['cls'] for t in only_rb)),
                      'incumbent_only_by_class': dict(Counter(t[0]['cls'] for t in only_inc)),
                      'incumbent_only_examples': [name(t[0]) for t in sorted(only_inc, key=lambda t: t[0]['cls'] == 'fuzz')[:5]]}
        both = [t for t in good if t[1]['RB_still']['ok'] and okI(t[2], k) and 'sha' in t[2][k]]
        eq = [t for t in both if t[2][k]['sha'] == t[1]['RB_still']['rgba'] and [t[2][k].get('w'), t[2][k].get('h')] == t[1]['RB_still']['dims']]
        eqset = {id(t) for t in eq}
        diff = [t for t in both if id(t) not in eqset]
        d['first_frame'] = {'compared': len(both), 'equal': len(eq), 'differ': len(diff),
                            'differ_by_class': dict(Counter(t[0]['cls'] for t in diff)),
                            'differ_by_depth': dict(Counter(f"depth{t[3]['depth']}" for t in diff)),
                            'differ_by_interlace_or_apng': {'interlaced': sum(1 for t in diff if t[3]['interlace'] == 1),
                                                           'apng': sum(1 for t in diff if t[3]['apng'])},
                            'differ_dims': sum(1 for t in diff if [t[2][k].get('w'), t[2][k].get('h')] != t[1]['RB_still']['dims']),
                            'differ_files': [name(t[0]) for t in sorted(diff, key=lambda t: t[0]['cls'] == 'fuzz')[:40]]}
        sboth = [t for t in good if t[1]['S_still']['ok'] and okI(t[2], k) and 'sha' in t[2][k]]
        d['strict_first_frame'] = {'compared': len(sboth), 'equal': sum(1 for t in sboth if t[2][k]['sha'] == t[1]['S_still']['rgba'])}
        L['incumbents'][k] = d
    # frame counts: RB stream versus image's frame loop
    ap = [t for t in good if t[3]['apng'] and t[1]['RB_stream']['ok'] and okI(t[2], 'image')]
    L['apng_frame_counts'] = {'compared': len(ap), 'equal': sum(1 for t in ap if t[1]['RB_stream']['frames'] == t[2]['image'].get('frames')),
                              'differ': [[name(t[0]), t[1]['RB_stream']['frames'], t[2]['image'].get('frames')] for t in ap
                                         if t[1]['RB_stream']['frames'] != t[2]['image'].get('frames')][:30]}
    # ---- council counts
    def outcome(o):
        if not ran(o):
            return 'not run'
        return 'strict accept' if o['S_still']['ok'] else ('rb accept' if o['RB_still']['ok'] else 'both reject')

    def council(flag_pred, label, stream=False):
        sel = [t for t in recs if flag_pred(t)]
        return {'files': by_sub(lambda r, o, i, s: flag_pred((r, o, i, s))),
                'outcomes': dict(Counter((t[1]['RB_stream'] if stream else t[1].get('RB_still', {})).get('error') or 'rb accept' if ran(t[1]) else 'not run' for t in sel)),
                'strict_errors': dict(Counter(t[1]['S_still'].get('error') or 'strict accept' for t in sel if ran(t[1]))),
                'examples': [name(t[0]) for t in sorted(sel, key=lambda t: t[0]['cls'] == 'fuzz')[:4]]}
    fl = lambda k: (lambda t: k in t[3]['flags'])
    L['council'] = {
        'surplus_decompressed_output (inflated_too_long)': council(fl('inflated_too_long'), ''),
        'surplus_compressed_bytes_after_zlib_end (surplus_after_zlib_stream)': council(fl('surplus_after_zlib_stream'), ''),
        'missing_iend': council(fl('missing_iend'), ''),
        'truncated_inside_trailing_ancillary_chunk': council(fl('truncated_inside_trailing_chunk'), ''),
        'trailing_bytes_after_iend': council(fl('trailing_bytes_after_iend'), ''),
        'bad_crc_ancillary_only': council(fl('bad_crc_ancillary_only'), ''),
        'bad_crc_critical': council(fl('bad_crc_critical'), ''),
        'depth16_files': council(lambda t: t[3]['depth'] == 16, ''),
        'r4_fallback_structural_apng_error': {
            'files': by_sub(lambda r, o, i, s: ran(o) and 'R4' in o['RB_stream'].get('events', [])),
            'strict_errors': dict(Counter(o['S_stream']['error'] for r, o, i, s in good if 'R4' in o['RB_stream'].get('events', []))),
            'examples': [name(r) for r, o, i, s in good if 'R4' in o['RB_stream'].get('events', [])][:10]},
        'palette_index_out_of_range (R3)': {
            'files': by_sub(lambda r, o, i, s: ran(o) and ('R3' in o['RB_still'].get('events', []) or 'R3' in o['RB_stream'].get('events', []))),
            'strict_errors': dict(Counter(o['S_still'].get('error') for r, o, i, s in good if 'R3' in o['RB_still'].get('events', []))),
            'examples': [name(r) for r, o, i, s in good if 'R3' in o['RB_still'].get('events', [])][:10]},
    }
    # ---- large images
    big = [t for t in recs if t[3]['w'] and (max(t[3]['w'], t[3]['h']) > 16384 or t[3]['w'] * t[3]['h'] > 1 << 24)]
    thin = [t for t in recs if t[3]['w'] and min(t[3]['w'], t[3]['h']) and max(t[3]['w'], t[3]['h']) / min(t[3]['w'], t[3]['h']) >= 32 and max(t[3]['w'], t[3]['h']) >= 1024]

    def outc(t):
        r, o, i, s = t
        rc = o.get('raised_caps')
        d = {'name': name(r), 'cls': r['cls'], 'holdout': r['holdout'], 'w': s['w'], 'h': s['h'], 'size': r['size'],
             'ct': s['ct'], 'depth': s['depth'], 'interlace': s['interlace'],
             'strict': ('accept' if o['S_still']['ok'] else o['S_still']['error']) if ran(o) else 'not run (' + str(o.get('skipped', o.get('timeout') and 'timeout')) + ')'}
        if rc is not None:
            d['raised_caps'] = ('timeout' if rc.get('timeout') else 'memory_error' if rc.get('memory_error') else
                                {'strict': 'accept' if rc['S_still']['ok'] else rc['S_still']['error'],
                                 'rb': 'accept' if rc['RB_still']['ok'] else rc['RB_still']['error'],
                                 'rb_events': rc['RB_still'].get('events', []), 'seconds': rc.get('seconds')})
        d['incumbents'] = {k: (i[k]['status'] if i.get(k, {}).get('status') != 'ok' else 'ok') for k in runners if k in i}
        if rc and isinstance(rc, dict) and rc.get('RB_still', {}).get('ok'):
            d['first_frame_equal'] = {k: i[k].get('sha') == rc['RB_still']['rgba'] for k in runners if okI(i, k) and 'sha' in i.get(k, {})}
        return d
    L['large'] = {'side_over_16384': by_sub(lambda r, o, i, s: s['w'] and max(s['w'], s['h']) > 16384),
                  'area_over_16.7Mpx': by_sub(lambda r, o, i, s: s['w'] and s['w'] * s['h'] > 1 << 24),
                  'area_over_33.5Mpx': by_sub(lambda r, o, i, s: s['w'] and s['w'] * s['h'] > 1 << 25),
                  'thin_ge_32_to_1_and_long_ge_1024': by_sub(lambda r, o, i, s: s['w'] and min(s['w'], s['h']) and max(s['w'], s['h']) / min(s['w'], s['h']) >= 32 and max(s['w'], s['h']) >= 1024),
                  'over_caps_files': [outc(t) for t in sorted(big, key=lambda t: -(t[3]['w'] * t[3]['h']))],
                  'thin_files': [outc(t) for t in sorted(thin, key=lambda t: -(max(t[3]['w'], t[3]['h']) / min(t[3]['w'], t[3]['h'])))[:60]],
                  'thin_outcomes': dict(Counter(('accept' if t[1]['S_still']['ok'] else t[1]['S_still']['error']) if ran(t[1]) else 'not run' for t in thin))}
    big_inc = {k: Counter() for k in runners}
    for t in big:
        for k in runners:
            if k in t[2]:
                big_inc[k][t[2][k]['status']] += 1
    L['large']['over_caps_incumbent_status'] = {k: dict(v) for k, v in big_inc.items() if v}
    L['timing'] = {'oracle_seconds_total': round(sum(o.get('seconds', 0) for _, o, _, _ in good), 1),
                   'slowest': sorted([[o['seconds'], name(r)] for r, o, _, _ in good], reverse=True)[:5]}
    per_file = []
    for r, o, i, s in recs:
        d = {'name': name(r), 'sha256': r['sha256'], 'size': r['size'], 'cls': r['cls'], 'holdout': r['holdout'],
             'w': s['w'], 'h': s['h'], 'ct': s['ct'], 'depth': s['depth'], 'interlace': s['interlace'], 'apng': s['apng'],
             'oracle': 'ran' if ran(o) else ('timeout' if o.get('timeout') else 'skipped' if o.get('skipped') else 'other')}
        if ran(o):
            d.update({'strict_still': 'accept' if o['S_still']['ok'] else o['S_still']['error'],
                      'rb_still': 'accept' if o['RB_still']['ok'] else o['RB_still']['error'],
                      'strict_stream': 'accept' if o['S_stream']['ok'] else o['S_stream']['error'],
                      'rb_stream': 'accept' if o['RB_stream']['ok'] else o['RB_stream']['error'],
                      'frames': o['RB_stream']['frames'], 'fields_still': sorted(o['ablate'].get('still', {})),
                      'fields_stream': sorted(o['ablate'].get('stream', {})),
                      'events_still': o['RB_still'].get('events', []), 'events_stream': o['RB_stream'].get('events', [])})
        if 'raised_caps' in o:
            d['raised_caps'] = outc((r, o, i, s)).get('raised_caps')
        d['incumbents'] = {k: i[k]['status'] for k in runners if k in i}
        per_file.append(d)
    Path(a.out_json).write_text(json.dumps({'summary': L, 'files': per_file}, indent=1, sort_keys=True) + '\n')
    write_md(L, a)
    print('wrote', a.out_json, 'and', a.out_md)


def write_md(L, a):
    M = []
    w = M.append
    SUBS = ['all', 'incidental', 'test', 'fuzz', 'non-fuzz', 'development', 'holdout']
    w('# PNG corpus pass: Strict and RecommendedBeta (draft 0.2) on the pinned corpus')
    w('')
    w(f"Generated by `png_recommendedbeta_pass.py report` from the caches under `~/corpora/png/pass-cache/`; machine-readable copy `png-recommendedbeta-pass.json` (names, hashes, error names, numbers; no file bytes). Corpus: {L['files']} files, manifest SHA-256 `{L['manifest_sha256']}`; `policy_oracle.py` SHA-256 `{L['policy_oracle_sha256']}`; `png_oracle.py` SHA-256 `{L['png_oracle_sha256']}`. Classes, hold-out and the header-level statistics are defined in `png-STATS.md`.")
    w('')
    ip = HERE / 'png-RECOMMENDEDBETA-PASS-interpretation.md'
    w(ip.read_text().rstrip() if ip.exists() else '(interpretation file missing)')
    w('')
    w('## Coverage')
    w('')
    w('| | files |')
    w('|---|---|')
    for k, v in sorted(L['coverage'].items()):
        if isinstance(v, (int, float)):
            w(f'| {k} | {v} |')
    w('')
    w('Tiers of the oracle run (the pure-Python oracle costs about 1.9 s per megapixel per decode): see the docstring of `png_recommendedbeta_pass.py`. `ran` = Strict and RecommendedBeta still outcomes exist (streams of non-APNG files are derived from the still outcome); `skipped_not_sampled` = between 1 Mpx and the 16.7 Mpx cap, no anomaly flag, not in the 1-in-8 hash sample; `skipped_area_over_16.7Mpx` = over the area cap, only the raised-cap run was attempted (below). Incumbent results exist for every file.')
    w('')
    w('## Acceptance by class (files the oracle ran)')
    w('')
    w('| outcome | ' + ' | '.join(SUBS) + ' |')
    w('|---|' + '---|' * len(SUBS))
    for k in ('ran', 'strict_still', 'rb_still', 'strict_stream', 'rb_stream', 'cap_rejects_both'):
        w(f'| {k} | ' + ' | '.join(str(L['accept'][k][s]) for s in SUBS) + ' |')
    w('')
    for tag, key in (('RecommendedBeta accepts what Strict rejects (flips), still', 'still'), ('same, whole stream', 'stream')):
        w(f'{tag}: ' + ', '.join(f"{s} {L['flips'][key][s]}" for s in SUBS) + '.')
        w('')
    w('RecommendedBeta rejects what Strict accepts: ' + ', '.join(f"{s} {L['flips']['rb_rejects_what_strict_accepts_still'][s]}" for s in SUBS) + ' (still); ' + ', '.join(f"{s} {L['flips']['rb_rejects_what_strict_accepts_stream'][s]}" for s in SUBS) + ' (stream).')
    w('')
    for kind in ('still', 'stream'):
        fl = L['flip_' + kind]
        w(f'## Which RecommendedBeta field fires ({kind}, one-field-at-a-time reversion)')
        w('')
        w(f"{fl['n']} flip files. A field \"fires\" for a file when reverting it alone to its Strict value makes the file rejected (or changes its output); the error is the one that field replaced. Files by number of firing fields: {fl['files_by_number_of_fields']} ({L['ablation_targeted_files']} files above 1 Mpx were ablated in targeted mode: only the fields that can raise the first Strict error were tried, so a second firing field would not be listed). Unexplained by a single field or a pair: {len(fl['unexplained'])}; explained only by a pair: {len(fl['pair_only'])}" + (' (' + '; '.join(f'{n} {f}' for n, f in fl['pair_only'][:5]) + ')' if fl['pair_only'] else '') + '.')
        w('')
        w('| field | files | incidental | test | fuzz | development | holdout | error replaced | image-still accepts | libpng accepts | gdk-loader accepts | example |')
        w('|---|---|---|---|---|---|---|---|---|---|---|---|')
        for f, v in fl['fields'].items():
            inc = v['incumbents_accepting']
            w(f"| {f} | {v['files']} | {v['incidental']} | {v['test']} | {v['fuzz']} | {v['development']} | {v['holdout']} | " + ', '.join(f'{e} {c}' for e, c in v['replaced_errors'].items()) + f" | {inc.get('image-still_accepts', 0)} | {inc.get('libpng_accepts', 0)} | {inc.get('gdk-loader_accepts', 0)} | `{v['examples'][0] if v['examples'] else ''}` |")
        w('')
        w('Fields that fire on no file: ' + (', '.join(fl['fields_never_firing']) or 'none') + '.')
        w('')
        w('Strict first error of the flip files: ' + '; '.join(f"{k}: " + ', '.join(f'{e} {c}' for e, c in v.items()) for k, v in fl['strict_first_error'].items() if k in ('all', 'incidental', 'test', 'fuzz')) + '.')
        w('')
    w('## First errors')
    w('')
    for tag, key in (('Strict first error, still', 'strict_first_error_still'), ('RecommendedBeta first error, still', 'rb_first_error_still'),
                     ('Strict first error, whole stream (APNG files differ)', 'strict_first_error_stream'), ('RecommendedBeta first error, whole stream', 'rb_first_error_stream')):
        w(f'**{tag}**')
        w('')
        names = sorted({e for s in ('incidental', 'test', 'fuzz') for e in L[key][s]}, key=lambda e: -L[key]['all'].get(e, 0))
        w('| error | incidental | test | fuzz | all |')
        w('|---|---|---|---|---|')
        for e in names:
            w(f"| {e} | {L[key]['incidental'].get(e, 0)} | {L[key]['test'].get(e, 0)} | {L[key]['fuzz'].get(e, 0)} | {L[key]['all'].get(e, 0)} |")
        w('')
    ob = L['obligation_R']
    w('## Obligation R')
    w('')
    w(f"RecommendedBeta equals Strict (same RGBA, `recovered` false) on every Strict-accepted file: {ob['strict_accepted_stills']} Strict-accepted stills, {ob['strict_accepted_streams']} Strict-accepted streams, {ob['strict_frames_checked']} Strict frames checked (prefix form), **{ob['violations']} violations**" + (': ' + '; '.join(f'{n} {v}' for n, v in ob['violating_files'][:10]) if ob['violations'] else '') + '.')
    w('')
    w('## Recovered events among RecommendedBeta-accepted files')
    w('')
    w('| kind | event | files | incidental | test | fuzz | example |')
    w('|---|---|---|---|---|---|---|')
    for kind, evs in L['recovered_events'].items():
        for e, v in evs.items():
            w(f"| {kind} | {e} | {v['files']} | {v['incidental']} | {v['test']} | {v['fuzz']} | `{v['examples'][0] if v['examples'] else ''}` |")
    w('')
    w('## Incumbents')
    w('')
    w('Runner status over all files, then agreement with RecommendedBeta (still) on the files the oracle ran. "RB only" = RecommendedBeta accepts and the incumbent does not; "incumbent only" = the incumbent decodes a file RecommendedBeta rejects (files that Strict rejects for a resource cap excluded). First frame = RGBA8 of the default image compared byte for byte with RecommendedBeta\'s still.')
    w('')
    w('| runner | ok | error | crash/timeout | both accept | RB only | incumbent only | first frame equal | differ |')
    w('|---|---|---|---|---|---|---|---|---|')
    for k, v in L['incumbents'].items():
        st = v['status']
        s = v.get('still', {})
        ff = v['first_frame']
        w(f"| {k} | {st.get('ok', 0)} | {st.get('error', 0)} | {st.get('crash', 0) + st.get('timeout', 0)} | {s.get('both_accept', '')} | {s.get('rb_only', '')} | {s.get('incumbent_only_excluding_cap_rejects', '')} | {ff['equal']} of {ff['compared']} | {ff['differ']} |")
    w('')
    for k, v in L['incumbents'].items():
        ff = v['first_frame']
        if ff['differ']:
            w(f"- {k}: {ff['differ']} first-frame differences: by class {ff['differ_by_class']}; by bit depth {ff['differ_by_depth']}; {ff['differ_by_interlace_or_apng']}; different dimensions {ff['differ_dims']}. First listed: " + ', '.join(f'`{n}`' for n in ff['differ_files'][:6]) + '.')
    w('')
    ac = L['apng_frame_counts']
    w(f"APNG frame counts (RecommendedBeta stream accepted and `image` decoded): {ac['equal']} of {ac['compared']} equal.")
    w('')
    w('## The council\'s counts')
    w('')
    w('| count | files | incidental | test | fuzz | holdout | RecommendedBeta outcome | Strict first error |')
    w('|---|---|---|---|---|---|---|---|')
    for k, v in L['council'].items():
        f = v['files']
        w(f"| {k} | {f['all']} | {f['incidental']} | {f['test']} | {f['fuzz']} | {f['holdout']} | " + (', '.join(f'{e} {c}' for e, c in v.get('outcomes', {}).items()) or '') + ' | ' + (', '.join(f'{e} {c}' for e, c in v.get('strict_errors', {}).items()) or '') + ' |')
    w('')
    lg = L['large']
    w('## Large and thin images')
    w('')
    w(f"Counts (IHDR claims, all files): side above 16,384: {lg['side_over_16384']['all']} (incidental {lg['side_over_16384']['incidental']}); area above 16.7 Mpx: {lg['area_over_16.7Mpx']['all']} (incidental {lg['area_over_16.7Mpx']['incidental']}); area above 33.5 Mpx: {lg['area_over_33.5Mpx']['all']} (incidental {lg['area_over_33.5Mpx']['incidental']}); thin (aspect at least 32:1, long side at least 1024): {lg['thin_ge_32_to_1_and_long_ge_1024']['all']} (incidental {lg['thin_ge_32_to_1_and_long_ge_1024']['incidental']}). Strict outcome of the thin files: {lg['thin_outcomes']}.")
    w('')
    w('Files over the caps, with what the oracle says when the caps are raised (`raised`: strict / RecommendedBeta still, same bytes; `-` = not attempted) and what each incumbent does (ok, error, crash, timeout):')
    w('')
    w('| file | class | dims | size | ct/depth | Strict (caps) | raised strict / RB | image-still | image | libpng | gdk-loader | gdk-file | glycin |')
    w('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for d in lg['over_caps_files']:
        rc = d.get('raised_caps')
        rcs = '-' if rc is None else (rc if isinstance(rc, str) else f"{rc['strict']} / {rc['rb']}")
        inc = d['incumbents']
        w(f"| `{d['name']}` | {d['cls']} | {d['w']}x{d['h']} | {d['size'] / 2**20:.1f} MiB | {d['ct']}/{d['depth']} | {d['strict']} | {rcs} | " + ' | '.join(inc.get(k, '') for k in ('image-still', 'image', 'libpng', 'gdk-loader', 'gdk-file', 'glycin')) + ' |')
    w('')
    w('Incumbent status over those files: ' + '; '.join(f'{k} {v}' for k, v in lg['over_caps_incumbent_status'].items()) + '.')
    w('')
    w(f"Timing: {L['timing']['oracle_seconds_total']} s of oracle time over {L['accept']['ran']['all']} files; slowest {L['timing']['slowest'][:3]}.")
    Path(a.out_md).write_text('\n'.join(M) + '\n')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('phase', choices=['oracle', 'incumbents', 'incumbents-big', 'report'])
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--timeout', type=float, default=600.0)
    ap.add_argument('--max-px', type=int, default=1_000_000, help='files up to this area are always run through the oracle; larger ones up to 2^24 only if flagged by png-stats or in the 1-in-SAMPLE hash sample')
    ap.add_argument('--sample', type=int, default=8)
    ap.add_argument('--force-thin', action='store_true', help='re-run, into oracle.zfix.jsonl, files recorded as not_sampled that have a side above 16,384 or aspect at least 32:1 with a side of at least 1024')
    ap.add_argument('--shard', default='0/1', help='K/N: process only files with int(sha256[-4:], 16) %% N == K; each shard has its own cache file')
    ap.add_argument('--workers', type=int, default=2)
    ap.add_argument('--budget', type=float, default=1e9, help='stop starting new files after this many seconds')
    ap.add_argument('--allow-unreached', action='store_true', help='report files the oracle budget did not reach as skipped (not_reached)')
    ap.add_argument('--partial', action='store_true', help='report only on files present in both caches (development)')
    ap.add_argument('--no-pin', action='store_true', help='skip the manifest SHA-256 check (development only)')
    ap.add_argument('--cache', default=str(Path.home() / 'corpora' / 'png' / 'pass-cache'))
    ap.add_argument('--out-json', default=str(HERE / 'png-recommendedbeta-pass.json'))
    ap.add_argument('--out-md', default=str(HERE / 'png-RECOMMENDEDBETA-PASS.md'))
    a = ap.parse_args()
    {'oracle': phase_oracle, 'incumbents': phase_incumbents, 'incumbents-big': phase_incumbents_big, 'report': phase_report}[a.phase](a)


if __name__ == '__main__':
    main()
