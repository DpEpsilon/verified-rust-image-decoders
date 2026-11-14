#!/usr/bin/env python3
"""Measure the GIF corpus with the strict Python oracle and a lenient structural scanner.

  python3 -I gif_stats.py [--limit N] [--oracle-timeout S] [--out-md gif-STATS.md] [--out-json gif-stats.json]

Reads $CORPUS_DIR/gif/MANIFEST.tsv (default ~/corpora) and files/*.gif; writes
gif-STATS.md and gif-stats.json beside this script.  Output holds file names (repo
paths) and numbers only, never file bytes.

Two independent passes per file:
  * oracle: gif/spec-draft/gif_oracle.decode_frames (Strict).  A file is accepted when
    every frame decodes and `end` is 'trailer'.  The first error is the Strict error name.
    Files whose oracle run exceeds --oracle-timeout seconds are recorded as 'timeout'.
  * scan: a lenient walk that does not stop at the first anomaly and counts the features
    that the owner questions Q5/Q7/Q8/Q9 depend on.  LZW is decoded symbolically (code
    lengths and the set of colour indices each table string contains), so no pixels are
    materialised and the scan is fast on large files.
"""
import argparse
import json
import signal
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT / 'gif' / 'spec-draft'))
import gif_oracle  # noqa: E402

import os  # noqa: E402

KNOWN_EXT = {0x01: 'PlainText', 0xF9: 'GraphicControl', 0xFE: 'Comment', 0xFF: 'Application'}


def sub_blocks(b, p):
    """Return (end_position_after_terminator, payload_bytes_total, ok)."""
    total = 0
    while True:
        if p >= len(b):
            return p, total, False
        n = b[p]
        if n == 0:
            return p + 1, total, True
        if p + 1 + n > len(b):
            return len(b), total + (len(b) - p - 1), False
        total += n
        p += 1 + n


def sub_blocks_data(b, p):
    parts = []
    while True:
        if p >= len(b):
            return b''.join(parts), p, False
        n = b[p]
        if n == 0:
            return b''.join(parts), p + 1, True
        if p + 1 + n > len(b):
            parts.append(b[p + 1:])
            return b''.join(parts), len(b), False
        parts.append(b[p + 1:p + 1 + n])
        p += 1 + n


def lzw_symbolic(m, data, area):
    """Decode LZW without materialising pixels.
    Returns dict: pixels (indices produced, capped at area+1 for reporting),
    used (bitmask int of colour index values produced, over all produced strings),
    eoi (an End-of-Information code was read), clear_first (first code is Clear)."""
    clear = 1 << m
    eoi = clear + 1
    nbits = len(data) * 8
    width = m + 1
    nxt = clear + 2
    length = [1] * clear + [0, 0]
    first = list(range(clear)) + [0, 0]
    mask = [1 << i if i < 256 else 0 for i in range(clear)] + [0, 0]
    prev = None
    pos = 0
    pixels = 0
    used = 0
    saw_eoi = False
    first_code = True
    clear_first = False
    bad = False
    while pos + width <= nbits:
        i = pos >> 3
        code = ((int.from_bytes(data[i:i + 3], 'little') >> (pos & 7)) & ((1 << width) - 1))
        pos += width
        if first_code:
            clear_first = code == clear
            first_code = False
        if code == clear:
            nxt, width, prev = clear + 2, m + 1, None
            continue
        if code == eoi:
            saw_eoi = True
            break
        if code < nxt:
            ln, fv, mk = length[code], first[code], mask[code]
        elif code == nxt and prev is not None and nxt < 4096:
            ln, fv, mk = length[prev] + 1, first[prev], mask[prev] | (1 << first[prev])
        else:
            bad = True
            break
        if prev is not None and nxt < 4096:
            if nxt >= len(length):
                length.append(0), first.append(0), mask.append(0)
            length[nxt] = length[prev] + 1
            first[nxt] = first[prev]
            mask[nxt] = mask[prev] | (1 << fv)
            nxt += 1
            if nxt == (1 << width) and width < 12:
                width += 1
        prev = code
        pixels += ln
        used |= mk
    return {'pixels': pixels, 'used': used, 'eoi': saw_eoi, 'clear_first': clear_first, 'bad': bad}


def scan(b):
    """Lenient structural scan.  Never raises."""
    r = {'size': len(b), 'valid_header': False, 'frames': 0, 'frame_pixels': 0}
    f = Counter()   # boolean-ish feature counters (counted per file as 0/1 later)
    r['flags'] = f
    r['min_code_sizes'] = Counter()
    r['ext_labels'] = Counter()
    r['app_ids'] = Counter()
    r['disposals'] = Counter()
    r['frame_rects'] = []
    if len(b) < 13 or b[:3] != b'GIF' or b[3:6] not in (b'87a', b'89a'):
        r['stop'] = 'bad-header'
        return r
    r['valid_header'] = True
    r['version'] = b[3:6].decode()
    W, H = b[6] | b[7] << 8, b[8] | b[9] << 8
    r['width'], r['height'] = W, H
    if W == 0 or H == 0:
        f['zero_size_screen'] += 1
    flags = b[10]
    gct = 3 * (1 << ((flags & 7) + 1)) if flags & 0x80 else 0
    gcols = gct // 3
    r['global_table'] = gcols
    p = 13 + gct
    if p > len(b):
        r['stop'] = 'truncated-gct'
        return r
    pend = None            # pending GCE dict or None
    saw_trailer = False
    stop = None
    while True:
        if p >= len(b):
            f['missing_trailer'] += 1
            stop = 'eof-no-trailer'
            break
        c = b[p]
        if c == 0x3B:
            saw_trailer = True
            if p + 1 < len(b):
                f['trailing_bytes'] += 1
                r['trailing_len'] = len(b) - p - 1
            if pend is not None:
                f['gce_without_graphic'] += 1
            break
        if c == 0x21:
            if p + 2 > len(b):
                stop = 'truncated-ext'
                break
            L = b[p + 1]
            q = p + 2
            r['ext_labels'][f'0x{L:02X}'] += 1
            if L == 0xF9:
                if pend is not None:
                    f['multiple_gce_for_one_image'] += 1
                if q + 6 <= len(b) and b[q] == 4:
                    g = b[q + 1]
                    d = (g >> 2) & 7
                    r['disposals'][d] += 1
                    if d >= 4:
                        f['disposal_4_7'] += 1
                    pend = {'transparent': b[q + 4] if g & 1 else None, 'delay': b[q + 2] | b[q + 3] << 8,
                            'disposal': d}
                    if g & 1:
                        f['has_transparency'] += 1
                    p = q + 6
                    continue
                pend = {'transparent': None, 'delay': 0, 'disposal': 0}
                f['malformed_gce'] += 1
                p, _, ok = sub_blocks(b, q)
                if not ok:
                    stop = 'truncated-ext'
                    break
                continue
            if L == 0x01:
                if pend is not None:
                    f['gce_before_plain_text'] += 1
                f['plain_text'] += 1
                pend = None
                if q + 13 <= len(b):
                    p, _, ok = sub_blocks(b, q + 13)
                else:
                    ok = False
                    p = len(b)
                if not ok:
                    stop = 'truncated-ext'
                    break
                continue
            if L == 0xFF:
                f['application_ext'] += 1
                if q + 12 <= len(b) and b[q] == 11:
                    r['app_ids'][b[q + 1:q + 12].decode('latin-1')] += 1
                    p, _, ok = sub_blocks(b, q + 12)
                else:
                    p, _, ok = sub_blocks(b, q + 1)
                if not ok:
                    stop = 'truncated-ext'
                    break
                continue
            if L == 0xFE:
                f['comment_ext'] += 1
            else:
                f['unknown_ext_label'] += 1
                if L <= 0x7F:
                    pend = None
            p, _, ok = sub_blocks(b, q)
            if not ok:
                stop = 'truncated-ext'
                break
            continue
        if c != 0x2C:
            f['unknown_block'] += 1
            stop = 'unknown-block'
            break
        if p + 10 > len(b):
            stop = 'truncated-image-desc'
            break
        left, top = b[p + 1] | b[p + 2] << 8, b[p + 3] | b[p + 4] << 8
        w, h = b[p + 5] | b[p + 6] << 8, b[p + 7] | b[p + 8] << 8
        I = b[p + 9]
        p += 10
        r['frames'] += 1
        r['frame_pixels'] += w * h
        r['frame_rects'].append((left, top, w, h))
        if w == 0 or h == 0:
            f['zero_size_frame'] += 1
        if left + w > W or top + h > H:
            f['frame_crosses_screen_edge'] += 1
        if I & 0x40:
            f['interlaced'] += 1
        lcols = 0
        if I & 0x80:
            f['local_color_table'] += 1
            lt = 3 * (1 << ((I & 7) + 1))
            lcols = lt // 3
            p += lt
            if p > len(b):
                stop = 'truncated-lct'
                break
        cols = lcols or gcols
        if cols == 0:
            f['no_color_table'] += 1
        if p >= len(b):
            stop = 'truncated-image-data'
            break
        m = b[p]
        r['min_code_sizes'][m] += 1
        data, p, ok = sub_blocks_data(b, p + 1)
        if not ok:
            f['truncated_image_data'] += 1
            stop = 'truncated-image-data'
        if not ok:
            pass   # truncated sub-blocks: LZW-level counts would only describe the cut
        elif 2 <= m <= 8 and w and h:
            z = lzw_symbolic(m, data, w * h)
            if not z['eoi']:
                f['missing_eoi'] += 1
            if z['bad']:
                f['invalid_lzw_code'] += 1
            if z['pixels'] < w * h:
                f['too_few_pixels'] += 1
            elif z['pixels'] > w * h:
                f['too_many_pixels'] += 1
            tr = pend['transparent'] if pend else None
            used = z['used']
            if tr is not None and 0 <= tr < 256:
                used &= ~(1 << tr)
            if cols and used >> cols:
                f['index_beyond_color_table'] += 1
                r.setdefault('undersized', []).append((cols, used.bit_length() - 1))
            if not cols and used:
                f['index_without_table'] += 1
        elif not (2 <= m <= 8):
            f['invalid_min_code_size'] += 1
        if pend is not None and pend['transparent'] is not None and cols and pend['transparent'] >= cols:
            f['transparent_index_beyond_table'] += 1
        pend = None
        if stop:
            break
    r['trailer'] = saw_trailer
    r['stop'] = stop
    return r


class Timeout(Exception):
    pass


def _alarm(sig, frm):
    raise Timeout()


def oracle_run(b, seconds):
    signal.signal(signal.SIGALRM, _alarm)
    signal.setitimer(signal.ITIMER_REAL, seconds)
    try:
        r = gif_oracle.decode_frames(b)
    except Timeout:
        return {'status': 'timeout'}
    except MemoryError:
        return {'status': 'memory'}
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
    if 'error' in r:
        return {'status': 'error', 'error': r['error'], 'frames_ok': 0}
    end = r['end']
    out = {'frames_ok': len(r['frames'])}
    if end == 'trailer':
        out['status'] = 'accept'
    else:
        out['status'] = 'error'
        out['error'] = end['error']
    return out


def is_fuzz(f):
    p = f['repo_path'].lower()
    return 'afl' in p or 'fuzz' in p or 'crash' in p or 'clusterfuzz' in p


TEST_TOKENS = ('test', 'fuzz', 'afl', 'crash', 'invalid', 'bad', 'broken', 'fail', 'repro', 'corrupt', 'trunc',
               'error', 'conformance', 'regression', 'artificial', 'suite', 'resources/', 'reftest', 'crashtest', 'samples', 'bug')


def is_incidental(f):
    """Heuristic: the file is a site, documentation or demo asset rather than a test input (no test-like token in its path)."""
    p = f['repo_path'].lower()
    return not any(t in p for t in TEST_TOKENS)


def sub_stats(fs):
    fs = [f for f in fs if f['width'] and f['height']]
    fp = [f['frame_pixels'] for f in fs]
    op = [f['frames'] * f['width'] * f['height'] for f in fs]
    mcs = Counter()
    for f in fs:
        for k, v in f['min_code_sizes'].items():
            mcs[int(k)] += v
    top = sorted(fs, key=lambda f: -f['frames'])[:3]
    big = max(fs, key=lambda f: f['width'] * f['height']) if fs else None
    side = max(fs, key=lambda f: max(f['width'], f['height'])) if fs else None
    return {'files': len(fs), 'max_frames': max((f['frames'] for f in fs), default=0),
            'frames_p99': pct([f['frames'] for f in fs], .99),
            'max_canvas_area': big and [big['width'], big['height'], big['repo_path']],
            'max_side': side and [side['width'], side['height'], side['repo_path']],
            'frame_pixels_max': max(fp, default=0), 'frame_pixels_p99': pct(fp, .99),
            'output_pixels_max': max(op, default=0), 'output_pixels_p99': pct(op, .99),
            'max_bytes': max((f['size'] for f in fs), default=0),
            'min_code_size_frames': {str(k): v for k, v in sorted(mcs.items())}}


def pct(xs, q):
    if not xs:
        return 0
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--oracle-timeout', type=float, default=120.0)
    ap.add_argument('--out-md', default=str(HERE / 'gif-STATS.md'))
    ap.add_argument('--out-json', default=str(HERE / 'gif-stats.json'))
    a = ap.parse_args()
    root = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'gif'
    rows = [l.split('\t') for l in (root / 'MANIFEST.tsv').read_text().splitlines()[1:]]
    if a.limit:
        rows = rows[:a.limit]
    files = []
    for r in rows:
        b = (root / r[0]).read_bytes()
        s = scan(b)
        o = oracle_run(b, a.oracle_timeout)
        # decode_still on the same bytes (first frame only; trailing bytes unexamined)
        st = gif_oracle.decode_still(b) if o['status'] != 'timeout' else {'error': 'timeout'}
        rec = {'sha256': r[1], 'repo': r[4], 'repo_path': r[6], 'size': int(r[7]), 'licence': r[3],
               'url': r[2], 'oracle': o, 'still': 'accept' if 'error' not in st else st['error'],
               'frames': s['frames'], 'width': s.get('width'), 'height': s.get('height'),
               'frame_pixels': s['frame_pixels'], 'valid_header': s['valid_header'],
               'version': s.get('version'), 'stop': s['stop'] if 'stop' in s else None,
               'flags': dict(s['flags']), 'min_code_sizes': dict(s['min_code_sizes']),
               'ext_labels': dict(s['ext_labels']), 'app_ids': dict(s['app_ids']),
               'disposals': dict(s['disposals']), 'undersized': s.get('undersized', [])[:3]}
        files.append(rec)
        if len(files) % 100 == 0:
            print(len(files), 'of', len(rows), flush=True)
    write(files, a)


def incumbent_section(S):
    L = []
    w = L.append
    names = {'gdk_anim': 'gdk-pixbuf 2.42.12, animation path', 'gdk_forced': 'gdk-pixbuf 2.42.12, loader forced to GIF, first frame', 'image': '`image` crate 0.25.10 GIF decoder (`into_frames`)'}
    w('## Incumbents versus the oracle (`gif_incumbents.py`, `gif-incumbents.json`)')
    w('')
    w(f"{S['files']} files; Strict accepts {S['strict_accept']}. Pixel comparison is on the first composited frame, with RGB zeroed where alpha is 0. Runner frame counts stop at 64 frames or 16 Mpx.")
    w('')
    w('Reading notes. The runners are capped at 64 frames and 16 Mpx per canvas and run under a 3 GiB address-space limit; a runner that aborts on allocation is counted as `crash`. gdk-pixbuf exposes no frame count, so the `anim` runner infers one from the repeating (pixels, delay) sequence and its counts differ from the other decoders for files with zero delays or repeated frames; the `forced` mode returns one static frame, so its frame counts are not comparable and the third pair is only meaningful for acceptance and first frame.')
    w('')
    w('| decoder | accepts | accepts a Strict-rejected file | rejects a Strict-accepted file | first frame equals oracle (compared / equal) |')
    w('|---|---:|---:|---:|---|')
    for k, nm in names.items():
        d = S[k]
        f = d['first_frame_vs_oracle']
        w(f"| {nm} | {d['ok']} | {d['accepts_strict_rejected']} | {d['n_rejects_strict_accepted']} | {f['compared']} / {f['equal']} |")
    w('')
    w(f"Both gdk-pixbuf (animation) and `image` accept a Strict-rejected file in {S['both_gdk_anim_and_image_accept_strict_rejected']} cases; all three agree to accept a Strict-accepted file in {S['strict_accept_and_both_libs_accept']} cases.")
    w('')
    for k, nm in names.items():
        d = S[k]
        w(f"Strict-rejected files that {nm} accepts, by Strict error: " + (', '.join(f'{e} {c}' for e, c in d['accepts_strict_rejected_by_error'].items()) or 'none') + '.')
        w('')
    w('| pair | both accept | only first | only second | neither | first frame equal | first frame differ | frame count equal | frame count differ |')
    w('|---|---:|---:|---:|---:|---:|---:|---:|---:|')
    for k, d in S['pairwise'].items():
        w(f"| {k} | {d['both_ok']} | {d['only_first_ok']} | {d['only_second_ok']} | {d['both_reject']} | {d['first_frame_equal']} | {d['first_frame_differ']} | {d['frame_count_equal']} | {d['frame_count_differ']} |")
    w('')
    for k, d in S['pairwise'].items():
        if d['differ_examples']:
            w(f"First-frame differences, {k}: " + ', '.join(f'`{x}`' for x in d['differ_examples'][:6]) + '.')
            w('')
    return '\n'.join(L)


def write(files, a):
    n = len(files)
    total = sum(f['size'] for f in files)
    hdr = [f for f in files if f['valid_header']]
    accf = [f for f in files if f['oracle']['status'] == 'accept']
    fuzz = [f for f in files if is_fuzz(f)]
    nonfuzz = [f for f in files if not is_fuzz(f)]
    inc_files = [f for f in files if f['valid_header'] and is_incidental(f)]
    nonzero = [f for f in hdr if f['width'] and f['height']]
    fcnt = Counter(f['frames'] for f in hdr)
    bins = [('0', 0, 0), ('1', 1, 1), ('2-9', 2, 9), ('10-99', 10, 99), ('100-999', 100, 999), ('1000+', 1000, 10**9)]
    frame_hist = {k: sum(1 for f in hdr if lo <= f['frames'] <= hi) for k, lo, hi in bins}
    top_frames = sorted(hdr, key=lambda f: (-f['frames'], f['repo_path']))[:10]
    area = lambda f: f['width'] * f['height']
    by_area = sorted(nonzero, key=lambda f: -area(f))[:5]
    by_side = sorted(nonzero, key=lambda f: -max(f['width'], f['height']))[:5]
    fp = [f['frame_pixels'] for f in hdr]
    outpix = [f['frames'] * area(f) for f in nonzero]
    top_fp = sorted(hdr, key=lambda f: -f['frame_pixels'])[:5]
    top_out = sorted(nonzero, key=lambda f: -(f['frames'] * area(f)))[:5]
    mcs = Counter()
    for f in hdr:
        for k in f['min_code_sizes']:
            mcs[int(k)] += 1
    mcs_frames = Counter()
    for f in hdr:
        for k, v in f['min_code_sizes'].items():
            mcs_frames[int(k)] += v
    statuses = Counter(f['oracle']['status'] for f in files)
    errs = defaultdict(list)
    for f in files:
        if f['oracle']['status'] == 'error':
            errs[f['oracle']['error']].append(f)
    still_ok = sum(1 for f in files if f['still'] == 'accept')
    still_errs = Counter(f['still'] for f in files if f['still'] != 'accept')

    feats = ['gce_before_plain_text', 'index_beyond_color_table', 'missing_eoi', 'frame_crosses_screen_edge',
             'missing_trailer', 'trailing_bytes', 'unknown_ext_label', 'disposal_4_7', 'multiple_gce_for_one_image',
             'zero_size_screen', 'zero_size_frame', 'transparent_index_beyond_table', 'index_without_table',
             'too_few_pixels', 'too_many_pixels', 'truncated_image_data', 'invalid_lzw_code', 'invalid_min_code_size',
             'gce_without_graphic', 'no_color_table', 'unknown_block',
             'interlaced', 'local_color_table', 'has_transparency', 'plain_text', 'comment_ext', 'application_ext']
    fc = {k: [f for f in files if f['flags'].get(k)] for k in feats}
    extra = {'animated (frames > 1)': sum(1 for f in hdr if f['frames'] > 1),
             'single frame': sum(1 for f in hdr if f['frames'] == 1),
             'version GIF87a': sum(1 for f in hdr if f['version'] == '87a'),
             'version GIF89a': sum(1 for f in hdr if f['version'] == '89a'),
             'odd width or height': sum(1 for f in nonzero if f['width'] % 2 or f['height'] % 2),
             'no signature (not GIF)': n - len(hdr)}
    nsgif = sum(1 for f in hdr if 'NETSCAPE2.0' in f['app_ids'])
    extra['NETSCAPE2.0 loop extension'] = nsgif
    ids = Counter()
    for f in hdr:
        for k in f['app_ids']:
            ids[k] += 1

    def ex(fl, k=3):
        return ', '.join(f"`{f['repo']}:{f['repo_path']}`" for f in fl[:k])

    repos = Counter(f['repo'] for f in files)
    repo_bytes = Counter()
    for f in files:
        repo_bytes[f['repo']] += f['size']
    summary = {
        'files': n, 'bytes': total, 'oracle_status': dict(statuses),
        'strict_accept': statuses.get('accept', 0), 'strict_accept_rate': round(statuses.get('accept', 0) / n, 4) if n else 0,
        'still_accept': still_ok, 'still_errors': dict(still_errs),
        'strict_errors': {k: {'files': len(v), 'examples': [f"{x['repo']}:{x['repo_path']}" for x in v[:3]]} for k, v in sorted(errs.items(), key=lambda kv: -len(kv[1]))},
        'fuzz_derived_files': len(fuzz), 'fuzz_derived_strict_accept': sum(1 for f in fuzz if f['oracle']['status'] == 'accept'),
        'nonfuzz_files': len(nonfuzz), 'nonfuzz_strict_accept': sum(1 for f in nonfuzz if f['oracle']['status'] == 'accept'),
        'strict_accepted_subset': sub_stats(accf),
        'incidental_files': len(inc_files), 'incidental_strict_accept': sum(1 for f in inc_files if f['oracle']['status'] == 'accept'),
        'incidental_strict_errors': dict(Counter(f['oracle']['error'] for f in inc_files if f['oracle']['status'] == 'error').most_common()),
        'incidental_features': {k: sum(1 for f in inc_files if f['flags'].get(k)) for k in ('index_beyond_color_table', 'gce_before_plain_text', 'missing_eoi', 'frame_crosses_screen_edge', 'missing_trailer', 'trailing_bytes', 'unknown_ext_label', 'disposal_4_7', 'multiple_gce_for_one_image', 'zero_size_screen')},
        'frame_count_histogram': frame_hist,
        'top10_frame_counts': [{'strict_frames_decoded': f['oracle'].get('frames_ok'), 'frames': f['frames'], 'repo': f['repo'], 'path': f['repo_path'], 'size': f['size'],
                                'canvas': [f['width'], f['height']], 'strict': f['oracle']['status'] + (':' + f['oracle'].get('error', '') if f['oracle']['status'] == 'error' else '')} for f in top_frames],
        'largest_canvas_area': [{'area': area(f), 'w': f['width'], 'h': f['height'], 'repo': f['repo'], 'path': f['repo_path']} for f in by_area],
        'largest_side': [{'side': max(f['width'], f['height']), 'w': f['width'], 'h': f['height'], 'repo': f['repo'], 'path': f['repo_path']} for f in by_side],
        'frame_pixels_sum_wh': {'max': max(fp) if fp else 0, 'p99': pct(fp, 0.99), 'p50': pct(fp, 0.5)},
        'output_pixels_frames_x_canvas': {'max': max(outpix) if outpix else 0, 'p99': pct(outpix, 0.99), 'p50': pct(outpix, 0.5)},
        'min_code_size_files': {str(k): v for k, v in sorted(mcs.items())},
        'min_code_size_frames': {str(k): v for k, v in sorted(mcs_frames.items())},
        'feature_files': {k: len(v) for k, v in fc.items()},
        'feature_files_nonfuzz': {k: sum(1 for x in v if not is_fuzz(x)) for k, v in fc.items()},
        'feature_all_names': {k: [f"{x['repo']}:{x['repo_path']}" for x in fc[k]] for k in ('index_beyond_color_table', 'gce_before_plain_text', 'multiple_gce_for_one_image', 'disposal_4_7', 'missing_trailer', 'zero_size_screen') },
        'feature_examples': {k: [f"{x['repo']}:{x['repo_path']}" for x in v[:3]] for k, v in fc.items()},
        'other': extra, 'application_ids': dict(ids.most_common(15)),
        'repos': {k: {'files': repos[k], 'bytes': repo_bytes[k], 'strict_accept': sum(1 for f in files if f['repo'] == k and f['oracle']['status'] == 'accept')} for k in sorted(repos)},
        'files_detail': [{k: f[k] for k in ('sha256', 'repo', 'repo_path', 'size', 'frames', 'width', 'height', 'frame_pixels', 'still')}
                         | {'strict': f['oracle']['status'] if f['oracle']['status'] != 'error' else f['oracle']['error'],
                            'flags': sorted(f['flags'])} for f in files],
    }
    Path(a.out_json).write_text(json.dumps(summary, indent=1, sort_keys=True) + '\n')

    L = []
    w = L.append
    w('# GIF corpus statistics')
    w('')
    w(f'Generated by `gif_stats.py` (strict oracle `gif/spec-draft/gif_oracle.py` plus a lenient structural scanner). Machine-readable copy: `gif-stats.json` (no file bytes). Corpus bytes live in `~/corpora/gif/files/`, outside every repo; provenance per file is in `manifests/gif.MANIFEST.tsv`.')
    w('')
    w('## Population')
    w('')
    w(f'- {n} distinct files, {total:,} bytes ({total / 2**20:.1f} MiB). {len(hdr)} carry a GIF87a/GIF89a signature; {n - len(hdr)} do not (kept as known-bad inputs).')
    w(f"- Sources ({len(repos)} repos): " + '; '.join(f'{k} {repos[k]}' for k, _ in repos.most_common()) + '.')
    w(f"- Fuzzer-derived files (path contains afl, fuzz or crash; mostly libnsgif's `ns-afl-gif` mutants): {len(fuzz)}, of which Strict accepts {sum(1 for f in fuzz if f['oracle']['status'] == 'accept')}. All other files: {len(nonfuzz)}, Strict accepts {sum(1 for f in nonfuzz if f['oracle']['status'] == 'accept')}.")
    w(f"- Incidental assets (heuristic: path has no test-like token such as test, fuzz, invalid, bad, resources/, samples, bug; these are documentation, website and demo images and the closest thing here to files that exist for a purpose other than testing a decoder): {len(inc_files)} with a GIF signature, Strict accepts {summary['incidental_strict_accept']}; first errors of the rest: " + (', '.join(f'{k} {v}' for k, v in summary['incidental_strict_errors'].items()) or 'none') + '. Features among them: ' + ', '.join(f'{k} {v}' for k, v in summary['incidental_features'].items()) + '.')
    w('- Licence column: NOASSERTION for every file (the repos state licences for their code, not for their test images; see the README). Bytes stay outside every repo.')
    w('')
    w('')
    w('| repo | files | bytes | Strict accepts |')
    w('|---|---:|---:|---:|')
    for k, _ in repos.most_common():
        w(f"| {k} | {repos[k]} | {repo_bytes[k]:,} | {summary['repos'][k]['strict_accept']} |")
    w('')
    w('## Strict (oracle, `decode_frames`)')
    w('')
    acc = statuses.get('accept', 0)
    w(f'- Strict accepts {acc} of {n} files ({100 * acc / n:.1f}%); rejects {statuses.get("error", 0)}; oracle timeouts {statuses.get("timeout", 0)} (limit {a.oracle_timeout:.0f} s).')
    w(f'- `decode_still` (first frame only, bytes after it unexamined) accepts {still_ok} ({100 * still_ok / n:.1f}%).')
    w('')
    w('| Strict error (first error) | files | examples |')
    w('|---|---:|---|')
    for k, v in sorted(errs.items(), key=lambda kv: -len(kv[1])):
        w(f'| {k} | {len(v)} | {ex(v)} |')
    w('')
    w('`decode_still` errors: ' + (', '.join(f'{k} {v}' for k, v in still_errs.most_common()) or 'none') + '.')
    w('')
    w('## Frames')
    w('')
    w('Frame-count histogram (files with a valid signature; count of image descriptors reached by the lenient scan, not only those Strict accepts): ' + ', '.join(f'{k}: {v}' for k, v in frame_hist.items()) + '.')
    w('')
    w('Ten largest frame counts:')
    w('')
    w('| frames (lenient scan) | frames Strict decoded | canvas | bytes | Strict | file |')
    w('|---:|---:|---|---:|---|---|')
    for f in top_frames:
        s = f['oracle']['status'] + (':' + f['oracle']['error'] if f['oracle']['status'] == 'error' else '')
        w(f"| {f['frames']} | {f['oracle'].get('frames_ok')} | {f['width']}x{f['height']} | {f['size']:,} | {s} | `{f['repo']}:{f['repo_path']}` |")
    w('')
    w('## Sizes')
    w('')
    if by_area:
        w(f"- Largest canvas area: {area(by_area[0]):,} px ({by_area[0]['width']}x{by_area[0]['height']}), `{by_area[0]['repo']}:{by_area[0]['repo_path']}`.")
        w(f"- Largest side: {max(by_side[0]['width'], by_side[0]['height'])} px ({by_side[0]['width']}x{by_side[0]['height']}), `{by_side[0]['repo']}:{by_side[0]['repo_path']}`.")
    w(f"- Sum of frame area w*h per file (the quantity the spec's `T` counts against `MAX_TOTAL_PIXELS` = 2^30): max {max(fp) if fp else 0:,}, p99 {pct(fp, .99):,}, median {pct(fp, .5):,}. Max file: `{top_fp[0]['repo']}:{top_fp[0]['repo_path']}`." if top_fp else '')
    w(f"- Composited output pixels (frames x canvas area, what a decoder that returns every canvas materialises): max {max(outpix) if outpix else 0:,}, p99 {pct(outpix, .99):,}, median {pct(outpix, .5):,}. Max file: `{top_out[0]['repo']}:{top_out[0]['repo_path']}`." if top_out else '')
    ss = summary['strict_accepted_subset']
    w(f"- Among the {ss['files']} Strict-accepted files only: max frames {ss['max_frames']} (p99 {ss['frames_p99']}); largest canvas {ss['max_canvas_area'][0]}x{ss['max_canvas_area'][1]} (`{ss['max_canvas_area'][2]}`); largest side {ss['max_side'][0]}x{ss['max_side'][1]}; sum of frame area max {ss['frame_pixels_max']:,}, p99 {ss['frame_pixels_p99']:,}; composited output pixels max {ss['output_pixels_max']:,}, p99 {ss['output_pixels_p99']:,}; largest file {ss['max_bytes']:,} bytes.")
    w('- Spec limits for comparison: `MAX_DIMENSION` 8192, one canvas `MAX_OUTPUT` 64 MiB (= 16,777,216 px), `MAX_FRAMES` 65,536, `MAX_TOTAL_PIXELS` 2^30.')
    w('')
    w('LZW minimum code sizes among Strict-accepted files (frames using each): ' + ', '.join(f'{k}: {v}' for k, v in summary['strict_accepted_subset']['min_code_size_frames'].items()) + '.')
    w('')
    w('LZW minimum code sizes over all files, including garbage read past a structural error (files / frames using each; values outside 2-8 come from broken or non-GIF streams): ' + ', '.join(f'{k}: {mcs[k]} / {mcs_frames[k]}' for k in sorted(mcs)) + '.')
    w('')
    w('## Features and anomalies (lenient scan; files, not frames)')
    w('')
    w('| feature | files | of which not fuzzer-derived | examples |')
    w('|---|---:|---:|---|')
    labels = {
        'gce_before_plain_text': 'Graphic Control immediately before a Plain Text Extension (Q7)',
        'index_beyond_color_table': 'colour index beyond the active table, transparent index excluded (Q5)',
        'missing_eoi': 'frame whose LZW data ends without End-of-Information',
        'frame_crosses_screen_edge': 'frame crossing the logical screen edge',
        'missing_trailer': 'missing trailer (EOF where a block should start)',
        'trailing_bytes': 'bytes after the trailer',
        'unknown_ext_label': 'unknown extension label (not 01, F9, FE, FF)',
        'disposal_4_7': 'disposal value 4-7',
        'multiple_gce_for_one_image': 'more than one Graphic Control for one graphic',
        'zero_size_screen': 'zero-size logical screen',
        'zero_size_frame': 'zero-size frame',
        'transparent_index_beyond_table': 'transparent index beyond the table',
        'index_without_table': 'indices used with no colour table',
        'too_few_pixels': 'LZW produces fewer pixels than the frame area',
        'too_many_pixels': 'LZW produces more pixels than the frame area',
        'truncated_image_data': 'truncated image data sub-blocks',
        'invalid_lzw_code': 'invalid LZW code',
        'invalid_min_code_size': 'LZW minimum code size outside 2..8',
        'gce_without_graphic': 'Graphic Control with no following graphic',
        'no_color_table': 'frame with neither local nor global table',
        'unknown_block': 'unknown block introducer',
        'interlaced': 'interlaced frame', 'local_color_table': 'local colour table', 'has_transparency': 'transparency flag set',
        'plain_text': 'Plain Text Extension', 'comment_ext': 'Comment Extension', 'application_ext': 'Application Extension'}
    for k in feats:
        w(f'| {labels[k]} | {len(fc[k])} | {sum(1 for x in fc[k] if not is_fuzz(x))} | {ex([x for x in fc[k] if not is_fuzz(x)] + [x for x in fc[k] if is_fuzz(x)])} |')
    for k, v in extra.items():
        w(f'| {k} | {v} | | |')
    w('')
    w('Files with a colour index beyond the table (Q5), all names:')
    w('')
    for nme in summary['feature_all_names']['index_beyond_color_table']:
        w(f'- `{nme}`')
    w('')
    w('Graphic Control before Plain Text (Q7), files that are not fuzzer-derived:')
    w('')
    for nme in [x for x in summary['feature_all_names']['gce_before_plain_text'] if not any(t in x.lower() for t in ('afl', 'fuzz', 'crash'))]:
        w(f'- `{nme}`')
    w('')
    w('Application identifiers: ' + (', '.join(f'`{k}` {v}' for k, v in ids.most_common(10)) or 'none') + '.')
    w('')
    w('Notes on the scan: "missing End-of-Information" counts a frame whose code stream ends (data exhausted, or invalid code) before an EOI code is read; a frame that reaches EOI after the last pixel is not counted. "Colour index beyond the table" ORs, per frame, the set of indices contained in every table string the decoder emits, so it counts a frame once if any emitted pixel has an index at or above the table size and that index is not the transparent index. Counts are of files; a file with 100 offending frames counts once.')
    w('')
    w('## What this shows and what it cannot show')
    w('')
    ip = HERE / 'gif-STATS-interpretation.md'
    w(ip.read_text().rstrip() if ip.exists() else '(interpretation file missing)')
    inc = HERE / 'gif-incumbents.json'
    if inc.exists():
        w('')
        w(incumbent_section(json.loads(inc.read_text())['summary']))
    Path(a.out_md).write_text('\n'.join(L) + '\n')


if __name__ == '__main__':
    main()
