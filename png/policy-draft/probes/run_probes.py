#!/usr/bin/env python3
"""Run the incumbent PNG decoders on the spec-draft vectors plus the targeted probes.

Runners (build: see ../../BRIEF.md "Evidence base"; binaries are regenerable, not durable):
  Rust  /tmp/claude-0/png-probe-target/release/png-policy-probe   (probes/runner: image 0.25.10 + png 0.18.1)
  Rust  /tmp/claude-0/png-glycin-target/release/png-glycin-probe  (probes/glycin-runner: glycin 7344b95's
        image-rs loader code for PNG, image 0.25.10 + png git e917d6a; build:
        CARGO_TARGET_DIR=/tmp/claude-0/png-glycin-target cargo build --release --offline -j1)
  C     /tmp/claude-0/png-probe-bin/{gdk-anim,gdk-formats,png-native} from ~/projects/leniency/gif/runners
        and ~/projects/leniency/shared/gdk.c (gdk-pixbuf 2.42.12 Debian build, libpng 1.6.48, libspng 46c81dc, stb 4e9b368)
Every runner process runs under prlimit --as (default 768 MiB, --as-kib to change), --core=0, 30 s timeout.

Input sets (--set): main (vectors, probes, witnesses; default; writes results.json), heldout and
heldoutB (heldout_def.py sets A and B; write results-heldout.json, results-heldoutB.json). Inputs above 4 KiB are stored by reference:
'hex' is null and 'ref' names the generator (probe:/vec:/wit:/held:) and 'sha256' the bytes;
resolve(record) regenerates and checks them.

Usage: python3 -I run_probes.py [--set main|heldout|heldoutB] [--only NAME_SUBSTRING] [--large]
                                [--runners r1,r2] [--out FILE] [--as-kib N]
  --large   only inputs above 4 KiB plus the beyond-cap dims probes
"""
import hashlib
import json
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
PNGDIR = HERE.parent.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(PNGDIR / 'spec-draft'))
import png_oracle as O  # noqa: E402
from probes_def import PROBES  # noqa: E402

RUST = Path('/tmp/claude-0/png-probe-target/release/png-policy-probe')
GLYCIN = Path('/tmp/claude-0/png-glycin-target/release/png-glycin-probe')
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
    'glycin': (GLYCIN, ['glycin']),             # glycin's PNG loader path (mime image/png), frame loop
    'image-load': (RUST, ['image-load']),       # image::load_from_memory, default Limits
}
DEFAULT_RUNNERS = [r for r in RUNNERS if r not in ('glycin', 'image-load')]   # the columns of results.json
LARGE_EXTRA = {'probe:dims-100000x1', 'probe:dims-16385x16'}
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


def invoke(path, spec, timeout, as_kib):
    """engine.invoke with a configurable address-space limit."""
    exe, args = spec
    try:
        p = subprocess.run(['prlimit', f'--as={as_kib * 1024}', '--core=0', '--fsize=67108864', '--', str(exe), *args, str(path)],
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout, check=False)
        lines = p.stdout.decode('utf-8', 'replace').splitlines()
        if p.returncode and not lines:
            return {'status': 'crash', 'error_class': 'signal' if p.returncode < 0 else 'exit',
                    'message': str(p.returncode) + ' ' + p.stderr.decode('utf-8', 'replace')[-160:]}
        obj = json.loads(lines[-1]) if lines else {'status': 'crash', 'error_class': 'empty-output'}
        if p.returncode and obj.get('status') == 'ok':
            return {'status': 'crash', 'error_class': 'post-output', 'message': str(p.returncode)}
        return obj
    except subprocess.TimeoutExpired:
        return {'status': 'timeout', 'error_class': 'timeout'}
    except (OSError, ValueError, json.JSONDecodeError) as e:
        return {'status': 'crash', 'error_class': type(e).__name__, 'message': str(e)[:150]}


_GEN = {}


def resolve(rec):
    """The input bytes of a results record: from 'hex', else regenerated from its generator, checked by SHA-256."""
    if rec.get('hex') is not None:
        return bytes.fromhex(rec['hex'])
    if not _GEN:
        for b, (n, _) in {b: (n, w) for n, b, w in main_inputs()}.items():
            _GEN[n] = b
        from heldout_def import heldout
        for n, b, _ in heldout('A') + heldout('B'):
            _GEN[n] = b
    b = _GEN[rec.get('ref', rec['name'])]
    if hashlib.sha256(b).hexdigest() != rec['sha256']:
        raise ValueError(f"{rec['name']}: regenerated input does not match the recorded SHA-256")
    return b


def main_inputs():
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


def inputs(which='main'):
    if which in ('heldout', 'heldoutB'):
        from heldout_def import heldout
        return heldout('A' if which == 'heldout' else 'B')
    return main_inputs()


def run_one(item, runners, as_kib):
    name, b, why = item
    p = WORK / (name.replace('/', '_').replace(':', '_') + '.png')
    p.write_bytes(b)
    res = {r: norm(invoke(p, RUNNERS[r], 30, as_kib)) for r in runners}
    p.unlink()
    rec = {'name': name, 'why': why, 'hex': b.hex() if len(b) <= 4096 else None,
           'sha256': hashlib.sha256(b).hexdigest(), 'len': len(b), 'oracle': oracle_summary(b), 'runners': res}
    if rec['hex'] is None:
        rec['ref'] = name
    return rec


def arg(flag, default=None):
    return sys.argv[sys.argv.index(flag) + 1] if flag in sys.argv else default


def main():
    only = arg('--only')
    which = arg('--set', 'main')
    runners = arg('--runners', ','.join(DEFAULT_RUNNERS)).split(',')
    as_kib = int(arg('--as-kib', 786432))
    WORK.mkdir(parents=True, exist_ok=True)
    items = [i for i in inputs(which) if not only or only in i[0]]
    if '--large' in sys.argv:
        items = [i for i in items if len(i[1]) > 4096 or i[0] in LARGE_EXTRA]
    workers = 1 if '--large' in sys.argv else 2
    with ThreadPoolExecutor(workers) as ex:
        results = list(ex.map(lambda i: run_one(i, runners, as_kib), items))
    default = 'results.json' if which == 'main' else f'results-{which}.json'
    out = HERE / arg('--out', default if not only else 'results-partial.json')
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
