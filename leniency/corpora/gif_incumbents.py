#!/usr/bin/env python3
"""Run the GIF corpus through the incumbent decoders and compare with the strict oracle.

  python3 -I gif_incumbents.py [--limit N] [--bin DIR] [--timeout S]

Runners (built by ../gif/build-min.sh or ../gif/build.sh into /tmp/claude-0/leniency-gifpng-bin):
  gdk-anim gif anim FILE DUMP         gdk-pixbuf 2.42.12, gdk_pixbuf_animation_new_from_file + iterator
  gdk-anim gif forced-whole FILE DUMP gdk-pixbuf loader forced to GIF, whole-buffer write, first frame
  rust-image gif:image FILE DUMP      `image` crate 0.25.10 GifDecoder::into_frames (or `rust` from build.sh)
Each prints one JSON line; DUMP receives the first composited frame as RGBA8.

Comparison with the oracle uses gif_oracle.decode_still (first frame composited on the logical
screen) and decode_frames (accept = whole stream valid).  Pixels are compared after setting RGB to 0
where alpha is 0 (decoders disagree about the colour under transparent pixels, which is invisible).
Frame counts are compared only when neither side hit the runner's cap (64 frames / 16 Mpx).

Writes gif-incumbents.json beside this script: per-file statuses and agreement flags, no file bytes.
"""
import argparse
import hashlib
import json
import os
import resource
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1] / 'gif' / 'spec-draft'))
import gif_oracle  # noqa: E402


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (3 << 30, 3 << 30))


def run(cmd, timeout):
    try:
        p = subprocess.run(cmd, capture_output=True, timeout=timeout, preexec_fn=limits)
    except subprocess.TimeoutExpired:
        return {'status': 'timeout'}
    out = p.stdout.decode('utf-8', 'replace').strip().splitlines()
    for line in reversed(out):
        if line.startswith('{'):
            try:
                return json.loads(line)
            except ValueError:
                break
    return {'status': 'crash', 'error_class': f'exit{p.returncode}'}


def norm(rgba):
    b = bytearray(rgba)
    for i in range(0, len(b), 4):
        if b[i + 3] == 0:
            b[i] = b[i + 1] = b[i + 2] = 0
    return hashlib.sha256(b).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--bin', default='/tmp/claude-0/leniency-gifpng-bin')
    ap.add_argument('--timeout', type=float, default=60.0)
    ap.add_argument('--out', default=str(HERE / 'gif-incumbents.json'))
    a = ap.parse_args()
    root = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'gif'
    rows = [l.split('\t') for l in (root / 'MANIFEST.tsv').read_text().splitlines()[1:]]
    if a.limit:
        rows = rows[:a.limit]
    bindir = Path(a.bin)
    rust = bindir / ('rust-image' if (bindir / 'rust-image').exists() else 'rust')
    sel = 'gif:image'
    libs = {
        'gdk_anim': [str(bindir / 'gdk-anim'), 'gif', 'anim'],
        'gdk_forced': [str(bindir / 'gdk-anim'), 'gif', 'forced-whole'],
        'image': [str(rust), sel],
    }
    scratch = Path(tempfile.mkdtemp(prefix='gifinc-', dir=os.environ.get('TMPDIR', '/tmp/claude-0')))
    recs = []
    for i, r in enumerate(rows):
        path = root / r[0]
        b = path.read_bytes()
        st = gif_oracle.decode_still(b)
        fr = gif_oracle.decode_frames(b)
        end = fr.get('end') if 'error' not in fr else {'error': fr['error']}
        strict_ok = end == 'trailer'
        rec = {'sha256': r[1], 'repo': r[4], 'repo_path': r[6], 'size': len(b), 'strict_accept': strict_ok,
               'strict_error': None if strict_ok else (end['error'] if isinstance(end, dict) else str(end)),
               'still_ok': 'error' not in st, 'oracle_frames': len(fr.get('frames', []))}
        want = norm(st['rgba']) if 'error' not in st else None
        if 'error' not in st:
            rec['dims'] = [st['width'], st['height']]
        for name, cmd in libs.items():
            dump = scratch / 'd.rgba'
            if dump.exists():
                dump.unlink()
            j = run(cmd + [str(path), str(dump)], a.timeout)
            o = {'status': j.get('status'), 'error': j.get('error_class') or j.get('message', '')[:60] if j.get('status') != 'ok' else None}
            if j.get('status') == 'ok':
                o['frames'] = j.get('frames')
                o['capped'] = bool(j.get('frames_capped'))
                o['dims'] = [j.get('width'), j.get('height')]
                if dump.exists():
                    o['first'] = norm(dump.read_bytes())
                    o['match_oracle_first'] = (want == o['first']) if want else None
                    o['dims_match_oracle'] = (o['dims'] == rec.get('dims')) if want else None
            rec[name] = o
        recs.append(rec)
        if (i + 1) % 200 == 0:
            print(i + 1, 'of', len(rows), flush=True)
    for f in scratch.glob('*'):
        f.unlink()
    scratch.rmdir()

    def ok(rc, k):
        return rc[k].get('status') == 'ok'

    summ = {'files': len(recs), 'runner_image': str(rust.name), 'strict_accept': sum(r['strict_accept'] for r in recs)}
    for k in libs:
        summ[k] = {'ok': sum(ok(r, k) for r in recs), 'status': dict(Counter(r[k].get('status') for r in recs)),
                   'errors': dict(Counter(r[k].get('error') for r in recs if not ok(r, k)).most_common(12))}
        rej = [r for r in recs if not r['strict_accept']]
        summ[k]['accepts_strict_rejected'] = sum(ok(r, k) for r in rej)
        summ[k]['accepts_strict_rejected_by_error'] = dict(Counter(r['strict_error'] for r in rej if ok(r, k)).most_common())
        summ[k]['rejects_strict_accepted'] = [f"{r['repo']}:{r['repo_path']}" for r in recs if r['strict_accept'] and not ok(r, k)][:20]
        summ[k]['n_rejects_strict_accepted'] = sum(r['strict_accept'] and not ok(r, k) for r in recs)
        both = [r for r in recs if r['still_ok'] and ok(r, k) and 'first' in r[k]]
        summ[k]['first_frame_vs_oracle'] = {'compared': len(both), 'equal': sum(bool(r[k]['match_oracle_first']) for r in both),
                                             'differ': [f"{r['repo']}:{r['repo_path']}" for r in both if not r[k]['match_oracle_first']][:20],
                                             'n_differ': sum(not r[k]['match_oracle_first'] for r in both)}
        summ[k]['oracle_still_ok_but_lib_rejects'] = sum(r['still_ok'] and not ok(r, k) for r in recs)
    pairs = [('gdk_anim', 'image'), ('gdk_forced', 'image'), ('gdk_anim', 'gdk_forced')]
    summ['pairwise'] = {}
    for x, y in pairs:
        d = {'both_ok': 0, 'only_first_ok': 0, 'only_second_ok': 0, 'both_reject': 0, 'first_frame_equal': 0,
             'first_frame_differ': 0, 'frame_count_equal': 0, 'frame_count_differ': 0, 'differ_examples': [], 'only_first_examples': [], 'only_second_examples': []}
        for r in recs:
            ax, ay = ok(r, x), ok(r, y)
            if ax and ay:
                d['both_ok'] += 1
                if r[x].get('first') and r[y].get('first'):
                    if r[x]['first'] == r[y]['first'] and r[x]['dims'] == r[y]['dims']:
                        d['first_frame_equal'] += 1
                    else:
                        d['first_frame_differ'] += 1
                        d['differ_examples'].append(f"{r['repo']}:{r['repo_path']}")
                if not r[x]['capped'] and not r[y]['capped']:
                    if r[x]['frames'] == r[y]['frames']:
                        d['frame_count_equal'] += 1
                    else:
                        d['frame_count_differ'] += 1
            elif ax:
                d['only_first_ok'] += 1
                d['only_first_examples'].append(f"{r['repo']}:{r['repo_path']}")
            elif ay:
                d['only_second_ok'] += 1
                d['only_second_examples'].append(f"{r['repo']}:{r['repo_path']}")
            else:
                d['both_reject'] += 1
        for k in ('differ_examples', 'only_first_examples', 'only_second_examples'):
            d[k] = d[k][:15]
        summ['pairwise'][f'{x} vs {y}'] = d
    # accepted-by-both-libs but Strict-rejected
    summ['both_gdk_anim_and_image_accept_strict_rejected'] = sum((not r['strict_accept']) and ok(r, 'gdk_anim') and ok(r, 'image') for r in recs)
    summ['strict_accept_and_both_libs_accept'] = sum(r['strict_accept'] and ok(r, 'gdk_anim') and ok(r, 'image') for r in recs)
    Path(a.out).write_text(json.dumps({'summary': summ, 'files': recs}, indent=1, sort_keys=True) + '\n')
    print(json.dumps(summ, indent=1, sort_keys=True)[:6000])


if __name__ == '__main__':
    main()
