#!/usr/bin/env python3
"""Run the incumbent PNG decoders on the spec-draft vectors plus the targeted probes.

Runners (build: see ../../BRIEF.md "Evidence base"; binaries are regenerable, not durable):
  Rust  /tmp/claude-0/png-probe-target/release/png-policy-probe   (probes/runner: image 0.25.10 + png 0.18.1)
  C     /tmp/claude-0/png-probe-bin/{gdk-anim,gdk-formats,png-native} from ~/projects/leniency/gif/runners
        and ~/projects/leniency/shared/gdk.c (gdk-pixbuf 2.42.12 Debian build, libpng 1.6.48, libspng 46c81dc, stb 4e9b368)
Writes probes/results.json (durable) and prints one line per input.

Usage: python3 -I run_probes.py [--only NAME_SUBSTRING]
"""
import hashlib
import json
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
PNGDIR = HERE.parent.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(PNGDIR / 'spec-draft'))
sys.path.insert(0, str(Path.home() / 'projects/leniency'))
import engine  # noqa: E402
import png_oracle as O  # noqa: E402
from probes_def import PROBES  # noqa: E402

RUST = Path('/tmp/claude-0/png-probe-target/release/png-policy-probe')
BIN = Path('/tmp/claude-0/png-probe-bin')
RUNNERS = {
    'image': (RUST, ['image']),                 # what image::load_from_memory / glycin-style frame loop sees
    'image-still': (RUST, ['image-still']),     # DynamicImage::from_decoder, never the APNG path
    'pngcrate': (RUST, ['pngcrate']),           # png 0.18.1 directly, EXPAND, then finish()
    'libpng': (BIN / 'png-native', ['libpng']),
    'spng': (BIN / 'png-native', ['spng']),
    'stb': (BIN / 'png-native', ['stb']),
    'gdk-loader': (BIN / 'gdk-anim', ['png', 'forced-whole']),
    'gdk-loader-byte': (BIN / 'gdk-anim', ['png', 'forced-byte']),
    'gdk-file': (BIN / 'gdk-formats', ['png', 'file']),
    'gdk-anim': (BIN / 'gdk-anim', ['png', 'anim']),
}
WORK = Path('/tmp/claude-0/png-probes')


def norm(o):
    """One schema for both runner families: status, w, h, sha (RGBA8 first frame), frames, seq,
    sha16 (exact RGBA16 when exposed), tail (error after pixels), err."""
    if o.get('status') != 'ok':
        return {'status': o.get('status', 'crash'), 'err': str(o.get('error') or o.get('message') or o.get('error_class'))[:160]}
    r = {'status': 'ok', 'w': o.get('w', o.get('width')), 'h': o.get('h', o.get('height'))}
    sha = o.get('sha') or o.get('rgba_sha256')
    if sha:
        r['sha'] = sha
    for k_in, k_out in (('frames', 'frames'), ('seq', 'seq'), ('seq_sha256', 'seq'), ('rgba16', 'sha16'),
                        ('rgba16_sha256', 'sha16'), ('tail_error', 'tail'), ('rgba', 'rgba_hex'), ('info', 'info'),
                        ('delays_ms', 'delays_ms'), ('frame_info', 'frame_info'), ('color', 'color')):
        if o.get(k_in) not in (None, ''):
            r[k_out] = o[k_in]
    return r


def oracle_summary(b):
    s = O.decode_still(b)
    out = {'still': s['error'] if 'error' in s else 'ok:' + hashlib.sha256(s['rgba']).hexdigest()}
    if 'error' not in s and s['info']['depth'] == 16:
        s16 = O.decode_still16(b)
        out['still16'] = hashlib.sha256(b''.join(v.to_bytes(2, 'big') for v in s16['rgba16'])).hexdigest()
    f = O.decode_frames(b)
    if 'error' in f:
        out['frames'] = {'error': f['error']}
    else:
        fr = f['frames']
        out['frames'] = {'n': len(fr), 'seq': hashlib.sha256(b''.join(x['rgba'] for x in fr)).hexdigest(),
                         'end': f['end'], 'plays': f['num_plays'], 'delays': [x['delay'] for x in fr[:16]]}
    return out


def inputs():
    seen = {}
    for v in json.loads((PNGDIR / 'spec-draft/VECTORS.json').read_text()):
        if 'pad_to' in v:
            continue
        seen.setdefault(bytes.fromhex(v['input_hex']), ('vec:' + v['name'], ''))
    for name, b, why in PROBES:
        seen[b] = ('probe:' + name, why)
    wit = Path.home() / 'projects/leniency/png/witnesses'
    for f in sorted(wit.glob('*.png')):          # held out: produced by the earlier leniency run, not tuned on
        seen.setdefault(f.read_bytes(), ('wit:' + f.stem, 'leniency-harness witness'))
    return [(n, b, why) for b, (n, why) in seen.items()]


def run_one(item):
    name, b, why = item
    p = WORK / (name.replace('/', '_').replace(':', '_') + '.png')
    p.write_bytes(b)
    res = {r: norm(engine.invoke(p, spec, 30)) for r, spec in RUNNERS.items()}
    p.unlink()
    return {'name': name, 'why': why, 'hex': b.hex() if len(b) <= 4096 else None,
            'sha256': hashlib.sha256(b).hexdigest(), 'len': len(b), 'oracle': oracle_summary(b), 'runners': res}


def main():
    only = sys.argv[sys.argv.index('--only') + 1] if '--only' in sys.argv else None
    WORK.mkdir(parents=True, exist_ok=True)
    items = [i for i in inputs() if not only or only in i[0]]
    with ThreadPoolExecutor(2) as ex:
        results = list(ex.map(run_one, items))
    out = HERE / ('results.json' if not only else 'results-partial.json')
    out.write_text(json.dumps(results, indent=0) + '\n')
    for r in results:
        o = r['oracle']['still']
        cells = []
        for k, v in r['runners'].items():
            if v['status'] != 'ok':
                cells.append(f"{k}=✗{v.get('err', v['status'])[:18]}")
            else:
                same = ('ok:' + v.get('sha', '')) == o
                cells.append(f"{k}={v['w']}x{v['h']}{'=' if same else '≠'}{v.get('sha', '')[:6]}/{v.get('frames', '')}")
        print(r['name'], '| strict', o[:14], '|', ' '.join(cells))


if __name__ == '__main__':
    main()
