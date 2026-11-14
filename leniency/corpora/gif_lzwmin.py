#!/usr/bin/env python3
"""GIF-Q6: which LZW minimum code sizes occur in the corpus, and what would accepting 1-11 change?

  python3 -I gif_lzwmin.py [--bin DIR] [--timeout S] [--json OUT.json]   (tables go to stdout)

Reads $CORPUS_DIR/gif/MANIFEST.tsv (default ~/corpora) and files/*.gif.  Prints markdown tables and,
with --json, a machine-readable copy (file names, sha256 prefixes and numbers only; no file bytes).

Passes:
  1. Lenient scan (gif_stats.scan, which does not stop at LZW errors and walks until the block
     structure becomes impossible): histogram of the min-code-size byte of every image descriptor it
     reaches, per file class (all / fuzzer-derived / not fuzzer-derived / incidental asset; the
     classes use gif_stats.is_fuzz and gif_stats.is_incidental unchanged).
  2. Structure-intact walk: the same histogram restricted to frames the Strict block grammar (G5-G9,
     gif_oracle.parse_*) reaches without a structural error.  Garbage that a lenient scan reads as an
     image descriptor after a structural break does not appear here.
  3. Census under the policy oracle: Recommended with lzw_min=Range1to11 over every file, recording each
     min-code-size byte the walk reads.  Recommended skips malformed extension sizes, so it reaches frames
     that the lenient scan (which assumes exact extension sizes) misreads.
  4. For every file with any size outside 2..8 (lenient scan or policy walk): the Strict result, two
     hypothetical decoders, Recommended with 2..8 and with 1..11, and the three incumbents.  The hypothetical decoders are Strict with G9/G10 changed:
       checked  1 <= m <= 11; a literal code in 256..2**m-1 (m = 9..11) is ColorIndexOutOfRange
       wrap     1 <= m <= 11; such a literal yields index code mod 256 (what gdk-pixbuf and `gif`/weezl do)
     They are implemented here by replacing gif_oracle.read_image_data and gif_oracle.lzw_decode for the
     duration of one call; gif_oracle.py is not edited.
     The wrap decoder is cross-checked against policy_oracle with lzw_min=Range1to11.
  5. A synthetic stream (not corpus bytes) with a literal code above 255 under min code size 9, run through
     the incumbents, to show whether they wrap.
  Incumbent runners: the binaries gif_incumbents.py uses (build with ../gif/build-min.sh -j1).  Frames are
  compared as in gif_incumbents.py: first composited frame, RGB zeroed where alpha is 0.
"""
import argparse
import contextlib
import hashlib
import json
import os
import resource
import signal
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / 'gif' / 'spec-draft'))
sys.path.insert(0, str(ROOT / 'gif' / 'policy-draft'))
import gif_oracle  # noqa: E402
import gif_stats  # noqa: E402
import policy_oracle  # noqa: E402

GifError = gif_oracle.GifError


# --- hypothetical decoders ------------------------------------------------------------------------

class Mode:
    name = 'strict'
    high_literals = 0     # count of literal codes in 256..2**m-1 consumed while decoding (wrap mode)
    seen_sizes = None


def read_image_data_1to11(b, p):
    """gif_oracle.read_image_data with the range 1..11."""
    gif_oracle.need(b, p, 1, 'TruncatedImageData')
    m = b[p]
    if Mode.seen_sizes is not None:
        Mode.seen_sizes.append(m)
    if not 1 <= m <= 11:
        raise GifError('InvalidLzwMinCodeSize')
    subs, p = gif_oracle.read_sub_blocks(b, p + 1, 'TruncatedImageData')
    return m, b''.join(subs), p


def lzw_decode_1to11(m, data, area, emit, wrap):
    """gif_oracle.lzw_decode with a literal alphabet of 2**m codes.  Codes 256..2**m-1 are 'high
    literals': wrap=True emits code & 255, wrap=False raises ColorIndexOutOfRange.  Everything else
    (clear/EOI numbering, width growth, KwKwK, error order) is the oracle's."""
    clear, eoi = 1 << m, (1 << m) + 1
    nbits = len(data) * 8
    pos = 0
    width = m + 1
    table = [bytes([i]) if i < 256 else (bytes([i & 255]) if wrap else None) for i in range(clear)] + [b'', b'']
    prev = None
    q = 0
    while True:
        if pos + width > nbits:
            raise GifError('TooFewPixels' if q < area else 'MissingEndOfInformation')
        code = 0
        for k in range(width):
            i = pos + k
            code |= ((data[i >> 3] >> (i & 7)) & 1) << k
        pos += width
        if code == clear:
            table, width, prev = table[:clear + 2], m + 1, None
            continue
        if q == area and code != eoi:
            raise GifError('MissingEndOfInformation')
        if code == eoi:
            if q < area:
                raise GifError('TooFewPixels')
            if (pos + 7) // 8 < len(data):
                raise GifError('DataAfterEndOfInformation')
            return
        if code < len(table) and code not in (clear, eoi):
            s = table[code]
            if s is None:
                raise GifError('ColorIndexOutOfRange')
            if 256 <= code < clear:
                Mode.high_literals += 1
        elif code == len(table) and prev is not None and len(table) < 4096:
            s = prev + prev[:1]
        else:
            raise GifError('InvalidLzwCode')
        if prev is not None and len(table) < 4096:
            table.append(prev + s[:1])
            if len(table) == (1 << width) and width < 12:
                width += 1
        prev = s
        for idx in s:
            if q >= area:
                raise GifError('TooManyPixels')
            emit(q, idx)
            q += 1


@contextlib.contextmanager
def decoder(mode):
    """mode: 'strict' (no change), 'checked', 'wrap'.  Yields the Mode record."""
    orig = gif_oracle.read_image_data, gif_oracle.lzw_decode
    Mode.name, Mode.high_literals, Mode.seen_sizes = mode, 0, []
    if mode != 'strict':
        gif_oracle.read_image_data = read_image_data_1to11
        gif_oracle.lzw_decode = lambda m, data, area, emit: lzw_decode_1to11(m, data, area, emit, mode == 'wrap')
    else:
        def rec(b, p):
            if p < len(b):
                Mode.seen_sizes.append(b[p])
            return orig[0](b, p)
        gif_oracle.read_image_data = rec
    try:
        yield Mode
    finally:
        gif_oracle.read_image_data, gif_oracle.lzw_decode = orig


class Timeout(Exception):
    pass


def _alarm(sig, frm):
    raise Timeout()


def timed(fn, *a, seconds=120):
    signal.signal(signal.SIGALRM, _alarm)
    signal.setitimer(signal.ITIMER_REAL, seconds)
    try:
        return fn(*a)
    except Timeout:
        return {'error': 'timeout'}
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)


def norm(rgba):
    b = bytearray(rgba)
    for i in range(0, len(b), 4):
        if b[i + 3] == 0:
            b[i] = b[i + 1] = b[i + 2] = 0
    return hashlib.sha256(b).hexdigest()


def end_of(res):
    if 'error' in res:
        return res['error']
    e = res.get('end')
    return 'ok' if e == 'trailer' else e['error']


def oracle_view(b, mode):
    """Whole-stream and first-frame results of Strict with the given G9/G10 variant."""
    with decoder(mode) as M:
        fr = timed(gif_oracle.decode_frames, b)
        sizes_all = list(M.seen_sizes)
        hi_all = M.high_literals
    with decoder(mode) as M:
        st = timed(gif_oracle.decode_still, b)
        hi_first = M.high_literals
    v = {'whole': end_of(fr), 'frames_ok': len(fr.get('frames', [])), 'oracle_sizes': sizes_all, 'high_literals': hi_all,
         'still': 'ok' if 'error' not in st else st['error'], 'high_literals_first': hi_first}
    if 'error' not in st:
        v['first_hash'] = norm(st['rgba'])
        v['dims'] = [st['width'], st['height']]
    return v


def walk_sizes(b):
    """Min-code-size bytes of the frames that Strict's block grammar reaches (no LZW decoding, no
    range check on m).  Stops at the first structural error, as gif_oracle.run would."""
    sizes = []
    try:
        scr, p = gif_oracle.parse_header(b)
        st = {'gce': None, 'frames_seen': 0, 'total_pixels': 0, 'loop_count': None}
        while p < len(b):
            c = b[p]
            if c == 0x3B:
                break
            if c == 0x21:
                p = gif_oracle.parse_extension(b, p, st)
                continue
            if c != 0x2C:
                break
            fh, p = gif_oracle.parse_image_descriptor(b, p, scr, st)
            if p >= len(b):
                break
            sizes.append(b[p])
            _, p = gif_oracle.read_sub_blocks(b, p + 1, 'TruncatedImageData')
            st['gce'] = None
            st['frames_seen'] += 1
            st['total_pixels'] += fh['rect'][2] * fh['rect'][3]
    except GifError:
        pass
    return sizes


# --- incumbents -----------------------------------------------------------------------------------

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


def incumbents(path, bindir, scratch, timeout):
    rust = bindir / ('rust-image' if (bindir / 'rust-image').exists() else 'rust')
    libs = {'gdk_anim': [str(bindir / 'gdk-anim'), 'gif', 'anim'],
            'gdk_forced': [str(bindir / 'gdk-anim'), 'gif', 'forced-whole'],
            'image': [str(rust), 'gif:image']}
    out = {}
    for name, cmd in libs.items():
        dump = scratch / 'd.rgba'
        if dump.exists():
            dump.unlink()
        j = run_runner(cmd + [str(path), str(dump)], timeout)
        o = {'status': j.get('status')}
        if j.get('status') == 'ok':
            o['frames'] = j.get('frames')
            o['dims'] = [j.get('width'), j.get('height')]
            if dump.exists():
                o['first'] = norm(dump.read_bytes())
        else:
            o['error'] = j.get('error_class') or (j.get('message') or '')[:60]
        out[name] = o
    return out


# --- policy census ----------------------------------------------------------------------------------

REC = policy_oracle.RECOMMENDED
REC11 = dict(REC, lzw_min='Range1to11')


def policy_view(b, P):
    res = timed(policy_oracle.decode_frames, b, P)
    st = timed(policy_oracle.decode_still, b, P)
    v = {'whole': end_of(res), 'frames_ok': len(res.get('frames', [])), 'still': 'ok' if 'error' not in st else st['error']}
    if 'error' not in st:
        v['first_hash'] = norm(st['rgba'])
        v['dims'] = [st['width'], st['height']]
    return v


def policy_census(b):
    """Recommended + Range1to11 over the whole file; returns the set of min-code-size bytes the walk read."""
    seen = []
    orig = policy_oracle.read_image_data

    def rec(bb, p, P):
        if p < len(bb):
            seen.append(bb[p])
        return orig(bb, p, P)
    policy_oracle.read_image_data = rec
    try:
        timed(policy_oracle.decode_frames, b, REC11, seconds=60)
    finally:
        policy_oracle.read_image_data = orig
    return sorted(set(seen))


# --- synthetic check (not corpus bytes) -------------------------------------------------------------

def synthetic_high_literal(bindir, scratch, timeout):
    """2x1 image, 256-colour table, min code size 9, codes [clear, 300, 7, EOI].  Code 300 is a literal
    above 255.  Returns what each incumbent produced against both hypotheses (wrap: palette[300 & 255]=44)."""
    pal = bytes(v for i in range(256) for v in (i, 255 - i, (i * 7) & 255))
    codes = [512, 300, 7, 513]
    bits = ''.join(format(c, '010b')[::-1] for c in codes)
    bits += '0' * (-len(bits) % 8)
    data = bytes(int(bits[i:i + 8][::-1], 2) for i in range(0, len(bits), 8))
    gif = (b'GIF89a' + bytes([2, 0, 1, 0, 0xF7, 0, 0]) + pal + b'\x2c' + bytes([0, 0, 0, 0, 2, 0, 1, 0, 0]) +
           bytes([9, len(data)]) + data + b'\x00;')
    f = scratch / 'synthetic-m9.gif'
    f.write_bytes(gif)
    want_wrap = norm(bytes(pal[3 * 44:3 * 44 + 3]) + b'\xff' + bytes(pal[3 * 7:3 * 7 + 3]) + b'\xff')
    inc = incumbents(f, bindir, scratch, timeout)
    out = {}
    for k, o in inc.items():
        out[k] = 'rejects' if o['status'] != 'ok' else ('wraps (index 300 -> 44)' if o.get('first') == want_wrap else 'accepts, other pixels')
    for mode in ('checked', 'wrap'):
        with decoder(mode):
            r = gif_oracle.decode_still(gif)
        out['hyp_' + mode] = r['error'] if 'error' in r else ('same as wrap' if norm(r['rgba']) == want_wrap else 'other pixels')
    return out


# --- main -----------------------------------------------------------------------------------------

def odd_sizes(c):
    """True when the size map/list contains a value outside 2..8."""
    return any(not 2 <= int(k) <= 8 for k in c)


CLASSES = [('all files', lambda x: True), ('fuzzer-derived', lambda x: x['fuzz']),
           ('not fuzzer-derived', lambda x: not x['fuzz']), ('incidental asset', lambda x: x['incidental'])]


def md_table(head, rows):
    L = ['| ' + ' | '.join(head) + ' |', '|' + '|'.join('---' for _ in head) + '|']
    L += ['| ' + ' | '.join(str(c) for c in r) + ' |' for r in rows]
    return '\n'.join(L)


def label(x):
    p = x['repo_path']
    if 'ns-afl-gif/' in p:
        p = 'ns-afl-gif/' + p.split('ns-afl-gif/')[1].split(',')[0]
    return f"{x['repo'].split('/')[-1]}:{p}"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--bin', default='/tmp/claude-0/leniency-gifpng-bin')
    ap.add_argument('--timeout', type=float, default=60.0)
    ap.add_argument('--json', default='')
    ap.add_argument('--skip-census', action='store_true', help='skip the 2-3 minute policy census over all files')
    a = ap.parse_args()
    root = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'gif'
    rows = [l.split('\t') for l in (root / 'MANIFEST.tsv').read_text().splitlines()[1:]]
    bindir = Path(a.bin)
    scratch = Path(tempfile.mkdtemp(prefix='lzwmin-', dir=os.environ.get('TMPDIR', '/tmp')))

    recs = []
    for r in rows:
        b = (root / r[0]).read_bytes()
        s = gif_stats.scan(b)
        f = {'repo_path': r[6]}
        rec = {'sha': r[1], 'repo': r[4], 'repo_path': r[6], 'size': len(b), 'valid_header': s['valid_header'],
               'fuzz': gif_stats.is_fuzz(f), 'incidental': s['valid_header'] and gif_stats.is_incidental(f),
               'canvas': [s.get('width'), s.get('height')], 'frames_lenient': s['frames'], 'stop': s.get('stop'),
               'lenient': {int(k): v for k, v in s['min_code_sizes'].items()},
               'intact': dict(Counter(walk_sizes(b)))}
        rec['policy_sizes'] = [] if a.skip_census else policy_census(b)
        recs.append(rec)

    hist = {}
    for kind in ('lenient', 'intact'):
        hist[kind] = {}
        for cname, pred in CLASSES:
            fs = [x for x in recs if pred(x)]
            fr, fl = Counter(), Counter()
            for x in fs:
                for k, v in x[kind].items():
                    fr[int(k)] += v
                    fl[int(k)] += 1
            hist[kind][cname] = {'n_files': len(fs), 'frames': dict(sorted(fr.items())), 'files': dict(sorted(fl.items()))}

    sub = [x for x in recs if odd_sizes(x['lenient']) or odd_sizes(x['policy_sizes'])]
    for x in sub:
        b = (root / 'files' / (x['sha'] + '.gif')).read_bytes()
        x['strict'] = oracle_view(b, 'strict')
        x['checked'] = oracle_view(b, 'checked')
        x['wrap'] = oracle_view(b, 'wrap')
        x['rec'] = policy_view(b, REC)
        x['rec11'] = policy_view(b, REC11)
        pol = timed(policy_oracle.decode_still, b, dict(policy_oracle.STRICT, lzw_min='Range1to11'))
        x['policy_strict11_still'] = norm(pol['rgba']) if 'error' not in pol else pol['error']
        x['policy_strict11_whole'] = end_of(timed(policy_oracle.decode_frames, b, dict(policy_oracle.STRICT, lzw_min='Range1to11')))
        x['inc'] = incumbents(root / 'files' / (x['sha'] + '.gif'), bindir, scratch, a.timeout)
    synth = synthetic_high_literal(bindir, scratch, a.timeout)
    for f in scratch.glob('*'):
        f.unlink()
    scratch.rmdir()

    prev = {}
    pj = HERE / 'gif-incumbents.json'
    if pj.exists():
        prev = {r['sha256']: r for r in json.loads(pj.read_text())['files']}

    out = {'n_files': len(recs), 'hist': hist, 'subset': sub, 'synthetic': synth}
    if a.json:
        Path(a.json).write_text(json.dumps(out, indent=1, sort_keys=True, default=str) + '\n')
    report(recs, hist, sub, synth, prev)


# --- report -----------------------------------------------------------------------------------------

def sizes_str(c):
    return ','.join(f'{k}' + (f'x{v}' if v > 1 else '') for k, v in sorted(((int(k), v) for k, v in c.items())))


def inc_cell(x, k):
    o = x['inc'][k]
    if o['status'] != 'ok':
        return 'rej'
    c = x['checked']
    if 'first_hash' not in c:
        return 'acc (hyp. rejects)'
    return 'eq' if (o.get('first') == c['first_hash'] and o['dims'] == c['dims']) else 'DIFF'


def report(recs, hist, sub, synth, prev):
    P = print
    P('<!-- generated by gif_lzwmin.py: tables only -->\n')
    # summary counts per class
    P('### Summary by class\n')
    for kind, title in (('lenient', 'lenient scan'), ('intact', 'structure-intact')):
        rows = []
        for (c, pred), d in zip(CLASSES, hist[kind].values()):
            fr = d['frames']
            fs = [x for x in recs if pred(x)]
            tot = sum(fr.values())
            f28 = sum(v for k, v in fr.items() if 2 <= k <= 8)
            f1 = sum(v for k, v in fr.items() if k in (1, 9, 10, 11))
            n_odd = sum(odd_sizes(x[kind]) for x in fs)
            n_mid = sum(any(int(k) in (1, 9, 10, 11) for k in x[kind]) for x in fs)
            rows.append([c, d['n_files'], tot, f28, f1, tot - f28 - f1, n_odd, n_mid])
        P(f'**{title}**\n')
        P(md_table(['class', 'files', 'frames', 'frames, size 2..8', 'frames, size 1 or 9..11', 'frames, size 0 or 12..255',
                    'files with any size outside 2..8', 'files with a 1 or 9..11'], rows))
        P('')
    P('')
    P('### Histograms (non-zero values only; cell = frames / files)\n')
    for kind, title in (('lenient', 'Lenient scan (every image descriptor the scan reaches)'),
                        ('intact', 'Structure-intact frames (reached by the Strict block grammar)')):
        P(f'**{title}**\n')
        vals = sorted({int(k) for c in hist[kind].values() for k in c['frames']})
        P(md_table(['class'] + [str(v) for v in vals],
                   [[f"{c} ({d['n_files']})"] + [f"{d['frames'].get(v, 0)} / {d['files'].get(v, 0)}" for v in vals] for c, d in hist[kind].items()]))
        P('')

    def is_a(x):
        return x['strict']['whole'] == 'InvalidLzwMinCodeSize' and odd_sizes(x['intact'])

    head = ['sha256', 'file', 'sizes (lenient; Strict-reached)', 'canvas', 'Strict first error', 'with 1..11 checked: whole file', 'gdk anim', 'gdk forced', 'image']

    def row(x):
        reached = sizes_str(x['intact']) if x['intact'] else '-'
        return [x['sha'][:10], '`' + label(x) + '`', f"{sizes_str(x['lenient']) or '-'}; {reached}", f"{x['canvas'][0]}x{x['canvas'][1]}",
                x['strict']['whole'], x['checked']['whole'], inc_cell(x, 'gdk_anim'), inc_cell(x, 'gdk_forced'), inc_cell(x, 'image')]

    nonfuzz = [x for x in sub if not x['fuzz']]
    fuzz = [x for x in sub if x['fuzz']]
    all_acc = lambda x: all(x['inc'][k]['status'] == 'ok' for k in ('gdk_anim', 'gdk_forced', 'image'))
    nonfuzz.sort(key=lambda x: (0 if all_acc(x) else 1 if is_a(x) else 2, x['repo_path']))   # decoded by all, rejected by all at a reached frame, size read behind an invalid header
    P(f'### Files that are not fuzzer-derived with a size outside 2..8 ({len(nonfuzz)})\n')
    P(md_table(head, [row(x) for x in nonfuzz]))
    P('')
    fa = [x for x in fuzz if is_a(x)]
    fb = [x for x in fuzz if not is_a(x)]
    P(f'### Fuzzer-derived files whose size outside 2..8 sits at a Strict-reached frame ({len(fa)})\n')
    P(md_table(head, [row(x) for x in sorted(fa, key=lambda x: x['repo_path'])]))
    P('')
    P(f'### Fuzzer-derived files where the size is read only after a structural break or in an invalid header ({len(fb)})\n')
    n_acc = sum(x['inc'][k]['status'] == 'ok' for x in fb for k in ('gdk_anim', 'gdk_forced', 'image'))
    P(f'Incumbent acceptances among these {len(fb)} files (three decoders each): {n_acc}.\n')
    P(md_table(['sha256', 'file', 'sizes (lenient)', 'canvas', 'Strict first error'],
               [[x['sha'][:10], '`' + label(x) + '`', sizes_str(x['lenient']) or ('(policy walk: ' + sizes_str(Counter(x['policy_sizes'])) + ')'),
                 f"{x['canvas'][0]}x{x['canvas'][1]}", x['strict']['whole']]
                for x in sorted(fb, key=lambda x: x['repo_path'])]))
    P('')

    # derived facts
    P('### Derived facts\n')
    facts = {}
    facts['subset_files'] = len(sub)
    facts['nonfuzz'] = len(nonfuzz)
    facts['fuzz'] = len(fuzz)
    facts['strict_InvalidLzwMinCodeSize_files'] = sum(x['strict']['whole'] == 'InvalidLzwMinCodeSize' for x in recs if 'strict' in x)
    facts['intact_odd_files'] = sum(1 for x in recs if odd_sizes(x['intact']))
    facts['intact_odd_equals_strict_error_set'] = (sorted(x['sha'] for x in recs if odd_sizes(x['intact'])) ==
                                                   sorted(x['sha'] for x in sub if x['strict']['whole'] == 'InvalidLzwMinCodeSize'))
    facts['subset_not_in_lenient'] = [x['sha'][:10] + ' ' + label(x) for x in sub if not odd_sizes(x['lenient'])]
    facts['wrap_vs_checked_differ_whole_or_still'] = [x['sha'][:10] for x in sub if (x['wrap']['whole'], x['wrap']['still'], x['wrap'].get('first_hash')) != (x['checked']['whole'], x['checked']['still'], x['checked'].get('first_hash'))]
    facts['files_consuming_high_literal_wrap'] = [x['sha'][:10] for x in sub if x['wrap']['high_literals'] or x['wrap']['high_literals_first']]
    facts['wrap_vs_policy_strict11_first_frame_mismatch'] = [x['sha'][:10] for x in sub
                                                           if x['wrap'].get('first_hash', x['wrap']['still']) != x['policy_strict11_still']]
    facts['wrap_vs_policy_strict11_whole_mismatch'] = [x['sha'][:10] for x in sub if x['policy_strict11_whole'] != x['wrap']['whole']]
    # Recommended 2..8 vs 1..11 over the whole corpus (only files the policy walk read an odd size from can differ)
    changed = [x for x in sub if (x['rec']['whole'], x['rec']['still']) != (x['rec11']['whole'], x['rec11']['still'])]
    facts['recommended_1to11_changes_whole_or_still'] = [(x['sha'][:10], label(x), x['rec']['whole'] + '/' + x['rec']['still'], x['rec11']['whole'] + '/' + x['rec11']['still']) for x in changed]
    facts['recommended_1to11_reject_to_accept_whole'] = [(x['sha'][:10], label(x)) for x in changed if x['rec']['whole'] != 'ok' and x['rec11']['whole'] == 'ok']
    facts['recommended_1to11_reject_to_still_ok'] = [(x['sha'][:10], label(x), x['rec11']['whole']) for x in changed if x['rec']['still'] != 'ok' and x['rec11']['still'] == 'ok']
    # incumbents vs hypothetical
    agree = []
    for x in sub:
        st = [x['inc'][k]['status'] == 'ok' for k in ('gdk_anim', 'gdk_forced', 'image')]
        firsts = {x['inc'][k].get('first') for k in ('gdk_anim', 'gdk_forced', 'image') if x['inc'][k]['status'] == 'ok'}
        agree.append((x['sha'][:10], x['fuzz'], tuple(st), len(firsts)))
    facts['all_three_accept'] = [(a[0], a[1], a[3]) for a in agree if all(a[2])]
    facts['mixed_accept'] = [(a[0], a[1], a[2]) for a in agree if any(a[2]) and not all(a[2])]
    facts['n_all_three_reject'] = sum(1 for a in agree if not any(a[2]))
    # consistency with gif-incumbents.json
    if prev:
        mism = []
        for x in sub:
            pr = prev.get(x['sha'])
            if not pr:
                continue
            for k in ('gdk_anim', 'gdk_forced', 'image'):
                if pr[k].get('status') != x['inc'][k]['status'] or pr[k].get('first') != x['inc'][k].get('first'):
                    mism.append((x['sha'][:10], k))
        facts['mismatch_vs_gif-incumbents.json'] = mism
    facts['strict_first_error_of_structure_broken_fuzz'] = dict(Counter(x['strict']['whole'] for x in fb).most_common())
    facts['strict_first_error_of_nonfuzz_not_reached'] = dict(Counter(x['strict']['whole'] for x in nonfuzz if not is_a(x)).most_common())
    facts['hyp_checked_first_frame_ok_but_incumbent_rejects'] = [(x['sha'][:10], k) for x in sub if 'first_hash' in x['checked']
                                                                 for k in ('gdk_anim', 'gdk_forced', 'image') if x['inc'][k]['status'] != 'ok']
    facts['incumbent_accepts_but_hyp_checked_first_frame_rejects'] = [(x['sha'][:10], k, x['checked']['still']) for x in sub if 'first_hash' not in x['checked']
                                                                      for k in ('gdk_anim', 'gdk_forced', 'image') if x['inc'][k]['status'] == 'ok']
    prof = {}
    for a_ in agree:
        if any(a_[2]) and not all(a_[2]):
            x = next(y for y in sub if y['sha'][:10] == a_[0])
            bb = (Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser() / 'gif' / 'files' / (x['sha'] + '.gif')).read_bytes()
            for nm in ('GdkCompat-2.42.12', 'ImageCrateCompat-0.25.10'):
                st = timed(policy_oracle.decode_still, bb, policy_oracle.PROFILES[nm])
                prof[(a_[0], nm)] = ('error ' + st['error']) if 'error' in st else ('first frame = gdk_anim' if norm(st['rgba']) == x['inc']['gdk_anim'].get('first') else 'first frame differs from gdk_anim')
    facts['mixed_accept_vs_compat_profiles'] = {f'{k[0]} {k[1]}': v for k, v in prof.items()}
    facts['synthetic_m9_literal_300'] = synth
    for k, v in facts.items():
        P(f'- {k}: {v}')
    P('')


if __name__ == '__main__':
    main()
