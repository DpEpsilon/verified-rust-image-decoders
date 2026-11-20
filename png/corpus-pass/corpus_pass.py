#!/usr/bin/env python3
"""M0-corpus (PNG-Q7): Strict and RecommendedBeta over the pinned real corpus and the project-authored synthetic set,
under the cap values of ../limits-draft/limits.py (2026-11-17: MAX_DIMENSION 2^31 - 1, MAX_OUTPUT 256 MiB, MAX_INPUT
2^30; guard MAX_BYTES 2^30). Re-run of the 2026-11-16 pass (../../leniency/corpora/png_recommendedbeta_pass.py, whose
still/stream/ablation functions are imported and reused) with these differences:

  * every file is run through the oracle at its own size (no 1-in-8 sample, no area tiers, no raised-cap side run): the
    new caps admit every real file up to 2^26 pixels, and the files beyond them are rejected at IHDR;
  * ablation is exhaustive at every size (the old pass ablated files above 1 Mpx in targeted mode), and is run on every
    Strict-rejected file, not only on flips, so each RecommendedBeta field gets a final outcome (below);
  * each file whose header quantities exceed an old cap (16,384 per side, 64 MiB RGBA8 output, 256 MiB input) is also
    run with the old caps patched into png_oracle, so the effect of the raise is measured, not inferred;
  * per file: the allocation sizes the decoder would reserve (SPEC P20): Raw R, RGBA8 output 4N, RGBA16 output 8N, an
    APNG canvas and PREVIOUS buffer, against the guard's MAX_BYTES; and header statistics for the other three caps
    (acTL num_frames, the fcTL pixel sum, the inflated iCCP length via the standard library's zlib, labelled as such).

  python3 -I corpus_pass.py oracle --set real|synthetic [--workers 16] [--shard K/N] [--timeout S] [--budget S] [--limit N]
  python3 -I corpus_pass.py report [--out-json results.json] [--out-holdout HOLDOUT.tsv]

The oracles are imported from ../spec-frozen/png_oracle.py (asserted byte-identical to ../spec-draft/png_oracle.py, which
policy_oracle.py names) and ../policy-draft/policy_oracle.py. Caches (one JSON line per file, resumable) live under
~/corpora/png/pass-cache-2026-11-19/, outside every repository; the outputs here carry names, hashes, error names and
numbers, never file bytes. Real files: ~/corpora/png/MANIFEST.tsv (SHA-256 pinned). Synthetic files:
~/corpora/png-synthetic/MANIFEST.tsv (written by gen_synthetic.py; SHA-256 checked per file).

Final outcome of a RecommendedBeta field f on a file: run RecommendedBeta with f alone reverted to its Strict value. f
"acts" on the file when that changes the outcome (acceptance, first error, or output). The final outcome is
RecommendedBeta's own outcome on that file: `accept` (f is needed for the file to be accepted) or the error that still
rejects it (f moved past one problem, another rejects the file). Flips (Strict rejects, RecommendedBeta accepts) that no
single reversion explains are tried with pairs, as in the 2026-11-16 pass.
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
import zlib
from collections import Counter, defaultdict
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
PNG = HERE.parent
ROOT = PNG.parent
LEN = ROOT / 'leniency' / 'corpora'
assert (PNG / 'spec-frozen/png_oracle.py').read_bytes() == (PNG / 'spec-draft/png_oracle.py').read_bytes(), \
    'spec-frozen and spec-draft png_oracle.py differ'
sys.path.insert(0, str(PNG / 'spec-frozen'))
import png_oracle as O  # noqa: E402  (the frozen oracle; policy_oracle's own import then reuses this module)
sys.path.insert(0, str(PNG / 'policy-draft'))
import policy_oracle as PO  # noqa: E402
sys.path.insert(0, str(LEN))
import png_recommendedbeta_pass as LP  # noqa: E402  (the 2026-11-16 pass: still_of, stream_of, ablate, header)
import png_stats  # noqa: E402  (classify, load_holdout: the class and hold-out definitions of 2026-11-16)
assert O.__file__ == str(PNG / 'spec-frozen' / 'png_oracle.py') and PO.O is O and LP.O is O

STRICT, RB, FIELDS = PO.STRICT, PO.RECOMMENDED_BETA, LP.FIELDS
CACHE = Path(os.environ.get('PASS_CACHE', str(Path.home() / 'corpora' / 'png' / 'pass-cache-2026-11-19')))
OLD_CACHE = Path.home() / 'corpora' / 'png' / 'pass-cache'
SYN = Path.home() / 'corpora' / 'png-synthetic'
NEW_CAPS = dict(MAX_INPUT=O.MAX_INPUT, MAX_DIMENSION=O.MAX_DIMENSION, MAX_OUTPUT=O.MAX_OUTPUT)
OLD_CAPS = dict(MAX_INPUT=256 << 20, MAX_DIMENSION=16384, MAX_OUTPUT=64 << 20)
MAX_BYTES = 1 << 30
OLD_MAX_BYTES = 1 << 26
SIG = LP.SIG
assert NEW_CAPS == dict(MAX_INPUT=1 << 30, MAX_DIMENSION=(1 << 31) - 1, MAX_OUTPUT=256 << 20), NEW_CAPS


class Timeout(Exception):
    pass


def _alarm(signum, frame):
    raise Timeout()


# ---------------------------------------------------------------- per-file analysis

def header_stats(b):
    """Light chunk scan (no oracle): IHDR fields, acTL num_frames, fcTL pixel sum and largest frame, iCCP compressed
    and inflated length (zlib, capped at 2^31), first-chunk positions. Never raises."""
    d = {'size': len(b)}
    try:
        if not b.startswith(SIG) or len(b) < 33 or b[12:16] != b'IHDR':
            return d
        w, h, depth, ct, _, _, il = struct.unpack('>IIBBBBB', b[16:29])
        d.update(w=w, h=h, depth=depth, ct=ct, interlace=il)
        p, fsum, fmax, nfctl = 8, 0, 0, 0
        while p + 8 <= len(b):
            ln = struct.unpack('>I', b[p:p + 4])[0]
            t = b[p + 4:p + 8]
            body = b[p + 8:p + 8 + ln]
            if t == b'acTL' and ln >= 8:
                d['num_frames'] = struct.unpack('>I', body[:4])[0]
            elif t == b'fcTL' and ln >= 26:
                fw, fh = struct.unpack('>II', body[4:12])
                fsum += fw * fh
                fmax = max(fmax, fw * fh)
                nfctl += 1
                if body[24] == 2:
                    d['dispose_previous'] = True
            elif t == b'iCCP':
                k = body.find(b'\0')
                if k >= 0 and k + 2 <= len(body):
                    comp = body[k + 2:]
                    d['iccp_compressed'] = len(comp)
                    try:
                        z = zlib.decompressobj()
                        out = z.decompress(comp, 1 << 31)
                        d['iccp_inflated'] = len(out)
                    except zlib.error:
                        d['iccp_inflated'] = None
            elif t == b'IEND':
                break
            p += 12 + ln
        if nfctl:
            d.update(fctl=nfctl, fctl_pixel_sum=fsum, fctl_max_pixels=fmax)
    except Exception as e:  # statistics only
        d['scan_error'] = type(e).__name__
    return d


def reservations(hs):
    """SPEC P20 reservation sizes from the header (exact formulas of the oracle): Raw R (raw_size), RGBA8 output 4N,
    RGBA16 output 8N; for APNG the canvas is 4N and a PREVIOUS disposal adds 4N (Saved)."""
    if 'w' not in hs or hs.get('ct') not in O.CHANNELS or hs.get('depth') not in O.DEPTHS.get(hs.get('ct'), ()) \
            or not hs['w'] or not hs['h'] or hs.get('interlace') not in (0, 1):
        return None
    doc = {'color_type': hs['ct'], 'depth': hs['depth'], 'interlace': hs['interlace']}
    N = hs['w'] * hs['h']
    if N > 1 << 40:
        return {'N': N, 'R': None}
    R = O.raw_size(doc, hs['w'], hs['h'])
    return {'N': N, 'R': R, 'out8': 4 * N, 'out16': 8 * N, 'saved': 4 * N if hs.get('dispose_previous') else 0}


def same(kind, s, base):
    if s['ok'] and base['ok']:
        return (s['whole'] == base['whole']) if kind == 'stream' else (s['rgba'] == base['rgba'] and s['recovered'] == base['recovered'])
    return s['ok'] == base['ok'] and s['error'] == base['error']


# Fields that the still entry reads after the walk (decode_image and zlib_decompress); every other RecommendedBeta field
# is read only by the walk (policy_oracle.py: P[...] uses by function).
DECODE_FIELDS = {'palette_index', 'window', 'zlib_tail'}


def still_walk(b, P):
    ev = set()
    try:
        return PO.walk_entry(b, P, ev, 'still'), ev
    except O.PngError as e:
        return e.name, None


def zlib_of(doc, P):
    ev = set()
    try:
        raw = PO.zlib_decompress(bytes(doc['idat']), O.raw_size(doc, doc['width'], doc['height']), 'ExcessImageData', P, ev,
                                 bytearray(), None)
        return hashlib.sha256(raw).hexdigest(), ev
    except O.PngError as e:
        return e.name, ev


def acts(b, kind, base):
    """Single-field reversion on a Strict-rejected file: {field: outcome after reverting it} where it differs.
    Exact shortcuts for the still entry (decode_still = walk, then decode_image of the walk's document; decode_image =
    zlib_decompress, then unfiltering and pixel_row, which reads only palette_index among the 15 fields):
      * a walk-only field: if the reverted walk raises, that error is the outcome; if it returns the same document and
        events as RecommendedBeta's walk, the rest is the same computation, so the outcome equals `base`;
      * window, zlib_tail (read only in zlib_decompress), when RecommendedBeta accepts: their Strict values only add a
        rejection (a smaller window, a check for bytes after the Adler-32), so the reverted inflate either raises, and
        that error is the outcome, or returns RecommendedBeta's bytes, and the outcome equals `base`; with CINFO = 7
        the window values coincide;
      * palette_index (read only for colour type 3 at an out-of-range index, which sets R3), when RecommendedBeta
        accepts without R3: reverting it changes nothing.
    Otherwise the full decode runs."""
    out = {}
    bw = still_walk(b, RB) if kind == 'still' else None
    for f in FIELDS:
        P = dict(RB, **{f: STRICT[f]})
        s = None
        if kind == 'still' and f not in DECODE_FIELDS:
            w = still_walk(b, P)
            if w[1] is None:
                s = {'ok': False, 'error': w[0]}
            elif bw[1] is not None and w[0] == bw[0] and w[1] == bw[1]:
                continue
        elif kind == 'still' and base['ok'] and bw[1] is not None:
            if f == 'palette_index' and 'R3' not in base['events']:
                continue
            if f == 'window' and bw[0]['idat'][:1] and bw[0]['idat'][0] >> 4 == 7:
                continue                    # CINFO = 7: the declared window is 2^15, the window Ignore uses
            if f in ('window', 'zlib_tail'):
                z = zlib_of(bw[0], P)
                if z[0] in O.ERRORS:
                    s = {'ok': False, 'error': z[0]}
                else:
                    continue                # same inflate, same Adler-32; the reverted value only adds a rejection
        if s is None:
            s = LP.stream_of(b, P) if kind == 'stream' else LP.still_of(b, P)
        if not same(kind, s, base):
            out[f] = 'accept' if s['ok'] else s['error']
            if s['ok'] and base['ok']:
                out[f] = 'DIFFERS'
    return out


def stream_from_still(sl):
    return {'ok': sl['ok'], 'error': sl['error'], 'frames': 1 if sl['ok'] else 0, 'dig': [sl.get('rgba')] if sl['ok'] else [],
            'rec': [sl.get('recovered')] if sl['ok'] else [], 'rec_frames': int(bool(sl.get('recovered'))),
            'events': sl.get('events', []), 'whole': sl.get('rgba')}


def analyse(b):
    """As LP.analyse (2026-11-16) with exhaustive ablation at every size and `acts` on every Strict-rejected file."""
    t0 = time.time()
    w, h, apng = LP.header(b)
    rec = {'w': w, 'h': h, 'apng_chunks': apng}
    S_sl, R_sl = LP.still_of(b, STRICT), LP.still_of(b, RB)
    if apng:
        S_st, R_st = LP.stream_of(b, STRICT), LP.stream_of(b, RB)
        rec['stream_derived'] = False
    else:
        S_st, R_st = stream_from_still(S_sl), stream_from_still(R_sl)
        rec['stream_derived'] = True
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
    rec['S_still'], rec['RB_still'] = S_sl, R_sl
    rec['flip_still'] = (not S_sl['ok']) and R_sl['ok']
    rec['flip_stream'] = (not S_st['ok']) and R_st['ok']
    rec['ablate'], rec['acts'] = {}, {}
    if not S_sl['ok']:
        rec['acts']['still'] = acts(b, 'still', R_sl)
        if rec['flip_still'] and not rec['acts']['still']:
            rec['ablate']['still'] = LP.ablate(b, 'still', R_sl)       # pairs
        elif rec['flip_still']:
            rec['ablate']['still'] = {f: e for f, e in rec['acts']['still'].items()}
    if not S_st['ok']:
        if rec['stream_derived']:
            rec['acts']['stream'] = rec['acts'].get('still', {})
            if rec['flip_stream']:
                rec['ablate']['stream'] = rec['ablate'].get('still', {})
        else:
            rec['acts']['stream'] = acts(b, 'stream', R_st)
            if rec['flip_stream'] and not rec['acts']['stream']:
                rec['ablate']['stream'] = LP.ablate(b, 'stream', R_st)
            elif rec['flip_stream']:
                rec['ablate']['stream'] = dict(rec['acts']['stream'])
    # in `ablate` (flips) the value is the error that the reverted field produces, as in the 2026-11-16 report
    rec['seconds'] = round(time.time() - t0, 2)
    return rec


def exceeds_old_caps(hs):
    if hs['size'] > OLD_CAPS['MAX_INPUT']:
        return True
    w, h = hs.get('w'), hs.get('h')
    return bool(w is not None and (max(w, h) > OLD_CAPS['MAX_DIMENSION'] or 4 * w * h > OLD_CAPS['MAX_OUTPUT']))


def old_caps_run(b):
    """png_oracle reads its caps at call time; policy_oracle imported MAX_INPUT by value, so it is patched there too."""
    saved = {k: getattr(O, k) for k in OLD_CAPS}
    saved_po = PO.MAX_INPUT
    try:
        for k, v in OLD_CAPS.items():
            setattr(O, k, v)
        PO.MAX_INPUT = OLD_CAPS['MAX_INPUT']
        s, r = LP.still_of(b, STRICT), LP.still_of(b, RB)
        return {'strict': 'accept' if s['ok'] else s['error'], 'rb': 'accept' if r['ok'] else r['error']}
    finally:
        for k, v in saved.items():
            setattr(O, k, v)
        PO.MAX_INPUT = saved_po


def phase_oldfix(a):
    """Recompute `old_caps` for cached records of files above the old MAX_INPUT (the first synthetic run patched only
    png_oracle's MAX_INPUT, which policy_oracle had imported by value). Writes merged records to oracle.<set>.zoldfix.jsonl,
    which load_cache reads last."""
    rows = {r['sha256']: r for r in (real_rows() if a.set == 'real' else synthetic_rows())}
    done = load_cache('oracle.' + a.set)
    n = 0
    with (CACHE / f'oracle.{a.set}.zoldfix.jsonl').open('a') as out:
        for sha, rec in done.items():
            if rec.get('hdr', {}).get('size', 0) > OLD_CAPS['MAX_INPUT'] and 'old_caps' in rec and not rec.get('old_caps_fixed'):
                rec['old_caps'] = old_caps_run(Path(rows[sha]['file']).read_bytes())
                rec['old_caps_fixed'] = True
                out.write(json.dumps(rec) + '\n')
                n += 1
    print('oldfix:', n, 'records')


def work(args):
    path, sha, timeout = args
    signal.signal(signal.SIGALRM, _alarm)
    resource.setrlimit(resource.RLIMIT_AS, (256 << 30, 256 << 30))
    t0 = time.time()
    b = Path(path).read_bytes()
    out = {'sha256_ok': hashlib.sha256(b).hexdigest() == sha}
    hs = header_stats(b)
    out['hdr'] = hs
    out['res'] = reservations(hs)
    try:
        signal.alarm(int(timeout))
        try:
            out.update(analyse(b))
            if exceeds_old_caps(hs):
                out['old_caps'] = old_caps_run(b)
        except Timeout:
            out['timeout'] = True
        except MemoryError:
            out['memory_error'] = True
        finally:
            signal.alarm(0)
    except Exception as e:  # record, never lose a file
        out['exception'] = f'{type(e).__name__}: {str(e)[:160]}'
    out['wall'] = round(time.time() - t0, 2)
    return out


# ---------------------------------------------------------------- sets

def real_rows():
    root, rows = LP.read_manifest(True)
    holdout = png_stats.load_holdout()
    for r in rows:
        r['cls'] = png_stats.classify(r['kind'], r['repo_path'])
        r['holdout'] = r['repo'] in holdout
        r['name'] = f"{r['repo']}:{r['repo_path']}"
        r['file'] = str(root / r['path'])
        r['set'] = 'real'
    return rows


def synthetic_rows():
    man = SYN / 'MANIFEST.tsv'
    lines = man.read_text().splitlines()
    cols = lines[0].split('\t')
    rows = []
    for line in lines[1:]:
        r = dict(zip(cols, line.split('\t')))
        r.update(file=str(SYN / r['path']), set='synthetic', cls='synthetic', holdout=False, name='synthetic:' + r['name'],
                 size=int(r['size']))
        rows.append(r)
    return rows


def area_of(r):
    with open(r['file'], 'rb') as f:
        b = f.read(29)
    if b.startswith(SIG) and len(b) >= 24 and b[12:16] == b'IHDR':
        w, h = struct.unpack('>II', b[16:24])
        return w * h
    return 0


def load_cache(stem):
    done = {}
    for p in sorted(CACHE.glob(stem + '*.jsonl')):
        for line in p.read_text().splitlines():
            try:
                r = json.loads(line)
            except ValueError:
                continue
            done[r['sha256']] = r
    return done


def phase_oracle(a):
    CACHE.mkdir(parents=True, exist_ok=True)
    rows = real_rows() if a.set == 'real' else synthetic_rows()
    k, n = (int(x) for x in a.shard.split('/'))
    done = load_cache('oracle.' + a.set)
    rows = [r for r in rows if r['sha256'] not in done and int(r['sha256'][-4:], 16) % n == k]
    if a.limit:
        rows = rows[:a.limit]
    # largest work first (bounded by the area cap: files beyond it are rejected at IHDR), so the tail is short
    # areas from the 2026-11-16 header statistics (png-stats.json, same manifest) where present: opening 22k files on
    # this volume just to read IHDR costs about 0.13 s each, serially
    st = {f['sha256']: (f['w'] or 0) * (f['h'] or 0) for f in json.loads((LEN / 'png-stats.json').read_text())['files_detail']} \
        if a.set == 'real' else {}
    ar = {r['sha256']: st[r['sha256']] if r['sha256'] in st else area_of(r) for r in rows}
    rows.sort(key=lambda r: -(ar[r['sha256']] if ar[r['sha256']] <= 1 << 26 else 0) - r['size'] / 1e9)
    cpath = CACHE / f'oracle.{a.set}.shard{k}of{n}.jsonl'
    t_end = time.time() + a.budget
    m = 0
    with cpath.open('a') as out, ProcessPoolExecutor(a.workers) as ex:
        futs = {}
        it = iter(rows)
        while True:
            while len(futs) < a.workers and time.time() < t_end:
                r = next(it, None)
                if r is None:
                    break
                futs[ex.submit(work, (r['file'], r['sha256'], a.timeout))] = r
            if not futs:
                break
            fut = next(as_completed(futs))
            r = futs.pop(fut)
            rec = fut.result()
            rec.update(sha256=r['sha256'], name=r['name'])
            out.write(json.dumps(rec) + '\n')
            out.flush()
            m += 1
            if m % 500 == 0:
                print(time.strftime('%H:%M:%S'), m, 'of', len(rows), flush=True)
    print(f'oracle {a.set} shard {k}/{n}: {m} files this call, {len(done) + m} cached')


# ---------------------------------------------------------------- report

INCIDENTAL_SUB = [('icon', ('icon', 'favicon', 'emoji', 'cursor', 'glyph', '/apps/', 'scalable', 'hicolor')),
                  ('screenshot', ('screenshot', 'screen', 'shot', 'capture', 'snapshot')),
                  ('wallpaper', ('wallpaper', 'wall', 'background', 'backdrop'))]


def incidental_sub(r, hs):
    """Path heuristic for the incidental class (icons, screenshots, wallpapers, other web/doc images); a square file of at
    most 256 x 256 counts as an icon. Repositories whose name says wallpaper count as wallpapers."""
    p = (r['repo'] + '/' + r['repo_path']).lower()
    for sub, keys in INCIDENTAL_SUB:
        if any(k in p for k in keys):
            return sub
    if hs.get('w') and hs['w'] == hs['h'] and hs['w'] <= 256:
        return 'icon'
    return 'web/doc image'


def outc(s):
    return 'accept' if s['ok'] else s['error']


def phase_report(a):
    real = real_rows()
    syn = synthetic_rows() if (SYN / 'MANIFEST.tsv').exists() else []
    orc_r, orc_s = load_cache('oracle.real'), load_cache('oracle.synthetic')
    missing = [r['name'] for r in real if r['sha256'] not in orc_r] + [r['name'] for r in syn if r['sha256'] not in orc_s]
    if missing and not a.partial:
        raise SystemExit(f'{len(missing)} files lack oracle records, e.g. {missing[:3]}')
    inc = LP.cache_load_all(OLD_CACHE, 'incumbents')
    old = LP.cache_load_all(OLD_CACHE, 'oracle')
    recs = []
    for r in real + syn:
        o = (orc_r if r['set'] == 'real' else orc_s).get(r['sha256'])
        if o is None:
            continue
        r = dict(r)
        if r['set'] == 'real':
            r['sub'] = incidental_sub(r, o['hdr']) if r['cls'] == 'incidental' else r['cls']
        else:
            r['sub'] = r['kind']
        recs.append((r, o))
    ran = lambda o: 'S_still' in o
    REAL = [(r, o) for r, o in recs if r['set'] == 'real']
    SYNR = [(r, o) for r, o in recs if r['set'] == 'synthetic']
    GROUPS = {'real': lambda r: r['set'] == 'real', 'incidental': lambda r: r['cls'] == 'incidental',
              'test': lambda r: r['cls'] == 'test', 'fuzz': lambda r: r['cls'] == 'fuzz',
              'development': lambda r: r['set'] == 'real' and not r['holdout'], 'holdout': lambda r: r['holdout'],
              'synthetic': lambda r: r['set'] == 'synthetic'}

    def by(pred, items=recs):
        return {g: sum(1 for r, o in items if f(r) and pred(r, o)) for g, f in GROUPS.items()}

    L = {'date': time.strftime('%Y-%m-%d'),
         'manifest_sha256': hashlib.sha256((LP.corpus_root() / 'MANIFEST.tsv').read_bytes()).hexdigest(),
         'synthetic_manifest_sha256': hashlib.sha256((SYN / 'MANIFEST.tsv').read_bytes()).hexdigest() if syn else None,
         'png_oracle_sha256': hashlib.sha256((PNG / 'spec-frozen/png_oracle.py').read_bytes()).hexdigest(),
         'policy_oracle_sha256': hashlib.sha256((PNG / 'policy-draft/policy_oracle.py').read_bytes()).hexdigest(),
         'limits_py_sha256': hashlib.sha256((PNG / 'limits-draft/limits.py').read_bytes()).hexdigest(),
         'caps': dict(NEW_CAPS, MAX_FRAMES=O.MAX_FRAMES, MAX_TOTAL_PIXELS=O.MAX_TOTAL_PIXELS, MAX_ICC=O.MAX_ICC, MAX_BYTES=MAX_BYTES),
         'old_caps': dict(OLD_CAPS, MAX_BYTES=OLD_MAX_BYTES), 'fields': FIELDS}
    L['composition'] = {'files': by(lambda r, o: True),
                        'incidental_sub': dict(Counter(r['sub'] for r, o in REAL if r['cls'] == 'incidental')),
                        'incidental_sub_holdout': dict(Counter(r['sub'] for r, o in REAL if r['cls'] == 'incidental' and r['holdout'])),
                        'synthetic_kinds': dict(Counter(r['kind'] for r, o in SYNR)),
                        'sha256_mismatch': [r['name'] for r, o in recs if not o.get('sha256_ok', True)]}
    cov = defaultdict(Counter)
    for r, o in recs:
        k = 'ran' if ran(o) else 'timeout' if o.get('timeout') else 'memory_error' if o.get('memory_error') else 'exception' if o.get('exception') else 'other'
        cov[r['set']][k] += 1
    L['coverage'] = {k: dict(v) for k, v in cov.items()}
    L['coverage']['failures'] = [[r['name'], o.get('exception') or ('timeout' if o.get('timeout') else 'memory_error')]
                                 for r, o in recs if not ran(o)]
    good = [(r, o) for r, o in recs if ran(o)]
    L['accept'] = {k: by(lambda r, o, k=k: ran(o) and o[k]['ok']) for k in ('S_still', 'RB_still', 'S_stream', 'RB_stream')}
    L['accept']['ran'] = by(lambda r, o: ran(o))
    L['accept']['flip_still'] = by(lambda r, o: ran(o) and o['flip_still'])
    L['accept']['flip_stream'] = by(lambda r, o: ran(o) and o['flip_stream'])
    L['accept']['rb_rejects_strict_accepts_still'] = by(lambda r, o: ran(o) and o['S_still']['ok'] and not o['RB_still']['ok'])
    L['accept']['rb_rejects_strict_accepts_stream'] = by(lambda r, o: ran(o) and o['S_stream']['ok'] and not o['RB_stream']['ok'])
    # ---- per Strict error (all 62 names, zeros included)
    for key in ('S_still', 'S_stream', 'RB_still', 'RB_stream'):
        L['errors_' + key] = {e: by(lambda r, o, e=e: ran(o) and not o[key]['ok'] and o[key]['error'] == e, good) for e in O.ERRORS}
    # ---- per field
    fields = {}
    for kind in ('still', 'stream'):
        F = {}
        for f in FIELDS:
            act = [(r, o) for r, o in good if f in o.get('acts', {}).get(kind, {})]
            need = [(r, o) for r, o in good if f in o.get('ablate', {}).get(kind, {}) or any(f in k.split('+') for k in o.get('ablate', {}).get(kind, {}) if '+' in k)]
            key = 'RB_' + kind
            F[f] = {'acts': by(lambda r, o: True, act),
                    'final_accept': by(lambda r, o: o[key]['ok'], act),
                    'final_reject': by(lambda r, o: not o[key]['ok'], act),
                    'final_reject_errors': dict(Counter(o[key]['error'] for r, o in act if not o[key]['ok']).most_common()),
                    'needed_for_acceptance': by(lambda r, o: True, need),
                    'needed_only_as_pair': sum(1 for r, o in need if f not in o['ablate'][kind]),
                    'error_replaced': dict(Counter(o['ablate'][kind].get(f, 'pair') for r, o in need).most_common()),
                    'revert_gives': dict(Counter(o['acts'][kind][f] for r, o in act).most_common()),
                    'examples_incidental': [r['name'] for r, o in act if r['cls'] == 'incidental'][:3],
                    'examples': [r['name'] for r, o in sorted(act, key=lambda t: (t[0]['cls'] == 'fuzz', t[0]['set'] == 'synthetic'))][:3]}
            inc_acc = Counter()
            for r, o in need:
                i = inc.get(r['sha256'], {})
                for k in ('image-still', 'libpng', 'gdk-loader', 'gdk-file', 'glycin'):
                    inc_acc[k] += i.get(k, {}).get('status') == 'ok'
            F[f]['incumbents_accepting_needed_files'] = dict(inc_acc)
        fields[kind] = F
        flips = [(r, o) for r, o in good if o['flip_' + kind]]
        L['flips_' + kind] = {'n': by(lambda r, o: True, flips),
                              'by_number_of_fields': dict(Counter(len(o['ablate'][kind]) for r, o in flips)),
                              'unexplained': [r['name'] for r, o in flips if not o['ablate'][kind]],
                              'pair_only': [[r['name'], sorted(o['ablate'][kind])] for r, o in flips if any('+' in k for k in o['ablate'][kind])]}
    L['fields'] = fields
    # ---- obligation R and events
    L['obligation_R'] = {'strict_accepted_stills': by(lambda r, o: o['S_still']['ok'], good),
                         'strict_accepted_streams': by(lambda r, o: o['S_stream']['ok'], good),
                         'strict_frames_checked': sum(o['R_frames_checked'] for r, o in good),
                         'violations': [[r['name'], o['R_violations']] for r, o in good if o['R_violations']]}
    ev = {}
    for kind in ('still', 'stream'):
        e = defaultdict(list)
        for r, o in good:
            if o['RB_' + kind]['ok']:
                for x in o['RB_' + kind].get('events', []):
                    e[x].append(r)
        ev[kind] = {x: {g: sum(1 for r in v if f(r)) for g, f in GROUPS.items()} for x, v in sorted(e.items())}
    L['recovered_events'] = ev
    # ---- caps
    def q(r, o):
        return o['hdr']
    caps = {}
    rl = [(r, o) for r, o in REAL if 'w' in o['hdr']]
    caps['largest_real_area'] = sorted([[o['hdr']['w'] * o['hdr']['h'], o['hdr']['w'], o['hdr']['h'], r['name'], r['cls'],
                                         outc(o['S_still']) if ran(o) else 'not run', outc(o['RB_still']) if ran(o) else 'not run']
                                        for r, o in rl], reverse=True)[:25]
    caps['largest_real_side'] = sorted([[max(o['hdr']['w'], o['hdr']['h']), o['hdr']['w'], o['hdr']['h'], r['name'], r['cls'],
                                         outc(o['S_still']) if ran(o) else 'not run'] for r, o in rl], reverse=True)[:15]
    acc = [(r, o) for r, o in REAL if ran(o) and o['RB_still']['ok']]
    def mx(items, f):
        best = max(items, key=lambda t: f(t[1]) or 0, default=None)
        return [f(best[1]), best[0]['name'], best[0]['cls']] if best else None
    caps['accepted_real_max'] = {
        'area_px': mx(acc, lambda o: o['hdr']['w'] * o['hdr']['h']),
        'side': mx(acc, lambda o: max(o['hdr']['w'], o['hdr']['h'])),
        'size_bytes': mx(acc, lambda o: o['hdr']['size']),
        'R': mx(acc, lambda o: (o['res'] or {}).get('R')),
        'out16_bytes_depth16': mx([t for t in acc if t[1]['hdr'].get('depth') == 16], lambda o: o['res']['out16']),
        'num_frames': mx(acc, lambda o: o['hdr'].get('num_frames')),
        'fctl_pixel_sum': mx(acc, lambda o: o['hdr'].get('fctl_pixel_sum')),
        'iccp_inflated': mx(acc, lambda o: o['hdr'].get('iccp_inflated')),
        'aspect': mx(acc, lambda o: max(o['hdr']['w'], o['hdr']['h']) / min(o['hdr']['w'], o['hdr']['h']))}
    caps['incidental_accepted_max'] = {
        'area_px': mx([t for t in acc if t[0]['cls'] == 'incidental'], lambda o: o['hdr']['w'] * o['hdr']['h']),
        'side': mx([t for t in acc if t[0]['cls'] == 'incidental'], lambda o: max(o['hdr']['w'], o['hdr']['h'])),
        'size_bytes': mx([t for t in acc if t[0]['cls'] == 'incidental'], lambda o: o['hdr']['size'])}
    # every file whose header quantities exceed an old cap: outcome now and under the old caps
    caps['over_old_caps'] = []
    for r, o in recs:
        if 'old_caps' in o or (ran(o) and exceeds_old_caps(o['hdr'])):
            hs = o['hdr']
            caps['over_old_caps'].append({'name': r['name'], 'set': r['set'], 'cls': r['cls'], 'holdout': r['holdout'],
                                          'w': hs.get('w'), 'h': hs.get('h'), 'ct': hs.get('ct'), 'depth': hs.get('depth'),
                                          'interlace': hs.get('interlace'), 'size': hs['size'],
                                          'strict': outc(o['S_still']) if ran(o) else 'not run',
                                          'rb': outc(o['RB_still']) if ran(o) else 'not run',
                                          'old': o.get('old_caps'), 'seconds': o.get('seconds'),
                                          'expect': r.get('expect'), 'R': (o['res'] or {}).get('R'),
                                          'incumbents': {k: v.get('status') for k, v in inc.get(r['sha256'], {}).items() if isinstance(v, dict) and 'status' in v}})
    caps['over_old_caps'].sort(key=lambda d: (d['set'], -((d['w'] or 0) * (d['h'] or 0))))
    caps['over_old_caps_summary'] = {
        s: dict(Counter((d['old']['strict'] if d['old'] else '?') + ' -> ' + d['strict'] for d in caps['over_old_caps'] if d['set'] == s))
        for s in ('real', 'synthetic')}
    caps['over_old_caps_rb_summary'] = {
        s: dict(Counter((d['old']['rb'] if d['old'] else '?') + ' -> ' + d['rb'] for d in caps['over_old_caps'] if d['set'] == s))
        for s in ('real', 'synthetic')}
    caps['cap_rejects_now'] = {k: by(lambda r, o, k=k: ran(o) and o['S_still'].get('error') == k, good)
                               for k in ('FileTooLarge', 'InvalidDimensions', 'OutputTooLarge', 'TooManyFrames', 'PixelBudgetExceeded', 'IccProfileTooLarge')}
    caps['cap_rejects_now_files'] = [[r['name'], r['cls'], o['hdr'].get('w'), o['hdr'].get('h'), o['S_still']['error']]
                                     for r, o in good if o['S_still'].get('error') in ('FileTooLarge', 'InvalidDimensions', 'OutputTooLarge', 'TooManyFrames', 'PixelBudgetExceeded')]
    # reservations of the files each profile accepts, against MAX_BYTES
    def resmax(items, key):
        vals = [((o['res'] or {}).get(key) or 0, r['name'], r['set']) for r, o in items]
        return max(vals, default=None)
    accS = [(r, o) for r, o in recs if ran(o) and o['RB_still']['ok']]
    caps['reservations'] = {s: {k: resmax([t for t in accS if t[0]['set'] == s], k) for k in ('R', 'out8', 'out16', 'saved')}
                            for s in ('real', 'synthetic')}
    caps['reservations_over_old_max_bytes'] = {
        s: {k: sum(1 for r, o in accS if r['set'] == s and ((o['res'] or {}).get(k) or 0) > OLD_MAX_BYTES and (k != 'out16' or o['hdr'].get('depth') == 16 or True))
            for k in ('R', 'out8', 'out16')} for s in ('real', 'synthetic')}
    caps['reservations_over_max_bytes'] = {s: sum(1 for r, o in accS if r['set'] == s and any(((o['res'] or {}).get(k) or 0) > MAX_BYTES for k in ('R', 'out8', 'out16')))
                                           for s in ('real', 'synthetic')}
    caps['peak_live_heap_still16_max'] = {s: max((((o['res'] or {}).get('R') or 0) + ((o['res'] or {}).get('out16') or 0), r['name']) for r, o in accS if r['set'] == s)
                                          if any(t[0]['set'] == s for t in accS) else None for s in ('real', 'synthetic')}
    caps['synthetic'] = [{'name': r['name'], 'kind': r['kind'], 'w': int(r['w']), 'h': int(r['h']), 'ct': int(r['ct']), 'depth': int(r['depth']),
                          'interlace': int(r['interlace']), 'encoding': r['encoding'], 'size': r['size'], 'expect': r['expect'],
                          'old_caps_by_intent': r['old_caps'], 'strict': outc(o['S_still']) if ran(o) else o.get('exception') or 'timeout',
                          'rb': outc(o['RB_still']) if ran(o) else '-', 'old': o.get('old_caps'),
                          'R': (o['res'] or {}).get('R'), 'out8': (o['res'] or {}).get('out8'), 'out16': (o['res'] or {}).get('out16'),
                          'seconds': o.get('seconds'), 'wall': o.get('wall'), 'R_violations': o.get('R_violations')}
                         for r, o in SYNR]
    L['caps'] = caps
    # ---- thin images
    thin = [(r, o) for r, o in REAL if o['hdr'].get('w') and o['hdr'].get('h') and max(o['hdr']['w'], o['hdr']['h']) >= 1024
            and max(o['hdr']['w'], o['hdr']['h']) / min(o['hdr']['w'], o['hdr']['h']) >= 32]
    L['thin_real'] = {'n': by(lambda r, o: True, thin),
                      'strict': dict(Counter(outc(o['S_still']) if ran(o) else 'not run' for r, o in thin)),
                      'rb': dict(Counter(outc(o['RB_still']) if ran(o) else 'not run' for r, o in thin))}
    # ---- the 2026-11-16 pass compared file by file (same manifest, old oracle and old caps, tiers)
    cmp_ = Counter()
    changed = []
    for r, o in REAL:
        p = old.get(r['sha256'])
        if not ran(o):
            continue
        if p is None or 'S_still' not in p:
            cmp_['not run on 2026-11-16'] += 1
            continue
        a_, b_ = (outc(p['S_still']), outc(p['RB_still'])), (outc(o['S_still']), outc(o['RB_still']))
        if a_ == b_ and (not o['S_still']['ok'] or p['S_still'].get('rgba') == o['S_still'].get('rgba')):
            cmp_['same'] += 1
        else:
            cmp_['changed'] += 1
            changed.append([r['name'], r['cls'], list(a_), list(b_)])
    L['vs_2026_11_16'] = {'counts': dict(cmp_), 'changed': changed[:200]}
    # ---- incumbents (cached results of 2026-11-16; binaries unchanged; synthetic files have none)
    runners = ['image-still', 'image', 'libpng', 'gdk-loader', 'gdk-file', 'glycin']
    ic = {}
    for k in runners:
        both = [(r, o) for r, o in REAL if ran(o) and o['RB_still']['ok'] and inc.get(r['sha256'], {}).get(k, {}).get('status') == 'ok']
        rbo = [(r, o) for r, o in REAL if ran(o) and o['RB_still']['ok'] and inc.get(r['sha256'], {}).get(k, {}).get('status') != 'ok']
        ino = [(r, o) for r, o in REAL if ran(o) and not o['RB_still']['ok'] and inc.get(r['sha256'], {}).get(k, {}).get('status') == 'ok']
        eq = sum(1 for r, o in both if inc[r['sha256']][k].get('sha') == o['RB_still']['rgba'])
        ic[k] = {'both_accept': len(both), 'rb_only': len(rbo), 'incumbent_only': len(ino),
                 'incumbent_only_by_rb_error': dict(Counter(o['RB_still']['error'] for r, o in ino).most_common(8)),
                 'incumbent_only_incidental': sum(1 for r, o in ino if r['cls'] == 'incidental'),
                 'first_frame_equal': eq,
                 'first_frame_differ_by_depth': dict(Counter(o['hdr'].get('depth') for r, o in both if inc[r['sha256']][k].get('sha') != o['RB_still']['rgba']))}
    L['incumbents'] = ic
    L['timing'] = {'oracle_cpu_seconds': round(sum(o.get('wall', 0) for r, o in recs)),
                   'slowest': sorted([[o.get('wall', 0), r['name']] for r, o in recs], reverse=True)[:8]}
    # ---- hold-out list (PNG-Q14)
    hold = [r for r, o in REAL if r['holdout']]
    with open(a.out_holdout, 'w') as f:
        f.write('# M0-corpus hold-out (PNG-Q7, PNG-Q14): every file of the 10 repositories of ../../leniency/corpora/holdout-png.tsv\n'
                '# (chosen by name on 2026-11-16 before any statistic). No compatibility profile has been run on these files; GdkCompat\n'
                '# and ImageCrateCompat are re-measured on them after Strict and RecommendedBeta are proved. Fitting any profile or\n'
                '# cap value to them spends the hold-out. Columns: manifest sha256, repo, path in repo, class.\n')
        f.write('sha256\trepo\trepo_path\tclass\n')
        for r in sorted(hold, key=lambda r: (r['repo'], r['repo_path'])):
            f.write(f"{r['sha256']}\t{r['repo']}\t{r['repo_path']}\t{r['cls']}\n")
    L['holdout'] = {'files': len(hold), 'by_class': dict(Counter(r['cls'] for r in hold)),
                    'repos': sorted({r['repo'] for r in hold}),
                    'list_sha256': hashlib.sha256(Path(a.out_holdout).read_bytes()).hexdigest()}
    per = []
    for r, o in recs:
        d = {'name': r['name'], 'sha256': r['sha256'], 'set': r['set'], 'cls': r['cls'], 'sub': r['sub'], 'holdout': r['holdout'],
             'w': o['hdr'].get('w'), 'h': o['hdr'].get('h'), 'ct': o['hdr'].get('ct'), 'depth': o['hdr'].get('depth'),
             'interlace': o['hdr'].get('interlace'), 'size': o['hdr']['size'], 'apng': o.get('apng_chunks')}
        if ran(o):
            d.update(strict_still=outc(o['S_still']), rb_still=outc(o['RB_still']), strict_stream=outc(o['S_stream']),
                     rb_stream=outc(o['RB_stream']), frames=o['RB_stream']['frames'],
                     events=sorted(set(o['RB_still'].get('events', [])) | set(o['RB_stream'].get('events', []))),
                     acts={k: v for k, v in o.get('acts', {}).items() if v}, needed={k: sorted(v) for k, v in o.get('ablate', {}).items() if v})
        else:
            d['oracle'] = o.get('exception') or ('timeout' if o.get('timeout') else 'memory_error')
        if o.get('old_caps'):
            d['old_caps'] = o['old_caps']
        if o.get('res'):
            d['R'] = o['res'].get('R')
        per.append(d)
    Path(a.out_json).write_text('{"summary": ' + json.dumps(L, indent=1, sort_keys=True) + ',\n "files": [\n'
                                + ',\n'.join(json.dumps(d, sort_keys=True) for d in per) + '\n]}\n')
    print('wrote', a.out_json, a.out_holdout)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('phase', choices=['oracle', 'report', 'oldfix'])
    ap.add_argument('--set', choices=['real', 'synthetic'], default='real')
    ap.add_argument('--workers', type=int, default=16)
    ap.add_argument('--shard', default='0/1')
    ap.add_argument('--timeout', type=float, default=4 * 3600)
    ap.add_argument('--budget', type=float, default=1e9)
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--partial', action='store_true')
    ap.add_argument('--out-json', default=str(HERE / 'results.json'))
    ap.add_argument('--out-holdout', default=str(HERE / 'HOLDOUT.tsv'))
    a = ap.parse_args()
    {'oracle': phase_oracle, 'report': phase_report, 'oldfix': phase_oldfix}[a.phase](a)


if __name__ == '__main__':
    main()
