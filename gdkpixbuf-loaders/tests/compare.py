#!/usr/bin/env python3
"""End-to-end check of the verified BMP loader module inside real gdk-pixbuf.

Builds the module, generates loaders.cache files for it with
gdk-pixbuf-query-loaders, then for every *.bmp in the corpus compares:

  core      the verified core called directly (examples/core_dump.rs)
  ours      gdk-pixbuf with GDK_PIXBUF_MODULE_FILE = cache listing only our module,
            through the leniency runner (file, stream, whole-buffer loader) and
            through tests/chunk_driver.c (loader writes of 1 byte, 7 bytes, whole)
  combined  cache listing ours first, then every stock module (runner, file)
  stock     the system's default cache: the leniency runner (file; it applies its
            own output-size limit, so it is not used for accept/reject) and
            tests/chunk_driver.c (stock-chunk0, a real GdkPixbufLoader; this is
            the reference for the accept/reject, RGBA and has_alpha comparison)

Nothing is written inside the repository; all output goes to WORK.
Usage: compare.py [--corpus DIR] [--runner PATH] [--work DIR]
"""
import argparse, hashlib, json, os, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
CRATE = HERE.parent
QUERY = subprocess.run(['pkg-config', '--variable=gdk_pixbuf_query_loaders', 'gdk-pixbuf-2.0'],
                       capture_output=True, text=True, check=True).stdout.strip()
MODDIR = subprocess.run(['pkg-config', '--variable=gdk_pixbuf_moduledir', 'gdk-pixbuf-2.0'],
                        capture_output=True, text=True, check=True).stdout.strip()

ap = argparse.ArgumentParser()
ap.add_argument('--corpus', default=str(CRATE.parent / 'dib' / 'corpus'))
ap.add_argument('--runner', default='/tmp/claude-0/leniency-target/gdk-runner')
ap.add_argument('--work', default='/tmp/claude-0/gdkpixbuf-compare')
args = ap.parse_args()

target = Path(os.environ.get('CARGO_TARGET_DIR', '/tmp/claude-0/gdkpixbuf-target'))
env_build = dict(os.environ, CARGO_TARGET_DIR=str(target),
                 PATH='/opt/cargo/bin:' + os.environ.get('PATH', ''))
subprocess.run(['cargo', 'build', '--offline', '--release', '--lib', '--examples'],
               cwd=CRATE, env=env_build, check=True, stderr=subprocess.DEVNULL)
so = target / 'release' / 'libpixbufloader_verified_bmp.so'
core_dump = target / 'release' / 'examples' / 'core_dump'

work = Path(args.work); (work / 'rgba').mkdir(parents=True, exist_ok=True)
driver = work / 'chunk_driver'
cflags = subprocess.run(['pkg-config', '--cflags', '--libs', 'gdk-pixbuf-2.0'],
                        capture_output=True, text=True, check=True).stdout.split()
subprocess.run(['cc', '-O1', '-o', str(driver), str(HERE / 'chunk_driver.c')] + cflags, check=True)

ours_cache = work / 'loaders-ours.cache'
ours_cache.write_text(subprocess.run([QUERY, str(so)], capture_output=True, text=True, check=True).stdout)
stock_mods = sorted(str(p) for p in Path(MODDIR).glob('*.so'))
# gdk-pixbuf builds its module list by prepending while it reads the cache, so
# on a tie (same signature score, same name) the module listed LAST wins.
comb_text = subprocess.run([QUERY] + stock_mods + [str(so)], capture_output=True, text=True, check=True).stdout
combined_cache = work / 'loaders-combined.cache'
combined_cache.write_text(comb_text)
mods = [l.strip('"') for l in comb_text.splitlines() if l.startswith('"') and l.endswith('.so"')]
assert mods and mods[-1] == str(so), 'combined cache must list our module last'

def env_with(cache):
    e = {k: v for k, v in os.environ.items() if not k.startswith('GDK_PIXBUF_')}
    if cache:
        e['GDK_PIXBUF_MODULE_FILE'] = str(cache)
    return e

def run_json(cmd, cache):
    p = subprocess.run(cmd, capture_output=True, text=True, env=env_with(cache), timeout=600)
    try:
        return json.loads(p.stdout.strip().splitlines()[-1])
    except Exception:
        return {'status': 'crash', 'rc': p.returncode, 'stderr': p.stderr[-300:]}

def key(r):
    """Accept/reject plus pixels: what must agree between paths."""
    return ('ok', r['width'], r['height'], r['rgba_sha256']) if r.get('status') == 'ok' else (r.get('status'),)

files = sorted(Path(args.corpus).glob('*.bmp'))
core_lines = subprocess.run([str(core_dump), str(work / 'rgba')] + [str(f) for f in files],
                            capture_output=True, text=True, check=True).stdout.splitlines()
core = {}
for line in core_lines:
    r = json.loads(line)
    if r['status'] == 'ok':
        r['rgba_sha256'] = hashlib.sha256((work / 'rgba' / (r['file'] + '.rgba')).read_bytes()).hexdigest()
    core[r['file']] = r

rows, problems = [], []
for f in files:
    n = f.name
    r = {'core': core[n]}
    r['stock'] = run_json([args.runner, 'file', str(f)], None)
    for mode in ('file', 'stream', 'whole'):
        r['ours-' + mode] = run_json([args.runner, mode, str(f)], ours_cache)
    r['combined-file'] = run_json([args.runner, 'file', str(f)], combined_cache)
    for c in ('1', '7', '0'):
        r['ours-chunk' + c] = run_json([str(driver), c, str(f)], ours_cache)
    r['stock-chunk0'] = run_json([str(driver), '0', str(f)], None)
    r['ours-info'] = run_json([str(driver), 'info', str(f)], ours_cache)
    r['combined-type'] = run_json([str(driver), 'type', str(f)], combined_cache)
    ours_keys = {k: key(v) for k, v in r.items() if k.startswith(('ours-', 'combined-')) and k != 'ours-info'}
    if len(set(ours_keys.values())) != 1:
        problems.append(f'{n}: our module differs between paths: {ours_keys}')
    if key(r['ours-file']) != key(r['core']):
        problems.append(f'{n}: our module {key(r["ours-file"])} != core {key(r["core"])}')
    for k in ('ours-chunk0', 'ours-chunk1', 'ours-chunk7', 'combined-type'):
        if r[k].get('status') == 'ok' and r[k].get('format') != 'BMP (verified core, GdkCompat)':
            problems.append(f'{n}: {k} was handled by {r[k].get("format")!r}')
    if r['core']['status'] == 'ok' and r['ours-chunk0'].get('status') == 'ok' \
            and r['ours-chunk0']['has_alpha'] != r['core']['has_alpha']:
        problems.append(f'{n}: has_alpha differs from core')
    rows.append((n, r))

def cat(r):
    # Reference is stock's real GdkPixbufLoader result (whole-buffer write), not
    # the runner's, so files that only our size caps reject are not "both reject".
    o, s = r['ours-chunk0'], r['stock-chunk0']
    if o['status'] == 'ok' and s['status'] == 'ok':
        return 'both accept, same RGBA' if key(o) == key(s) else 'both accept, different RGBA'
    if o['status'] == 'ok':
        return 'ours only'
    if s['status'] == 'ok':
        return 'stock only'
    return 'both reject'

cats = {}
for n, r in rows:
    cats.setdefault(cat(r), []).append(n)
acc = lambda k: sum(r[k]['status'] == 'ok' for _, r in rows)
both_acc = [(n, r) for n, r in rows if cat(r).startswith('both accept')]
alpha_agree = sum(r['ours-chunk0']['has_alpha'] == r['stock-chunk0']['has_alpha'] for _, r in both_acc)
alpha_diff = [n for n, r in both_acc if r['ours-chunk0']['has_alpha'] != r['stock-chunk0']['has_alpha']]
info_ok = sum(r['ours-info'].get('status') == 'ok' and r['ours-info'].get('width', 0) > 0 for _, r in rows)

(work / 'results.json').write_text(json.dumps(dict(rows), indent=1))
print(f'files: {len(rows)}')
print(f'accepted: core {acc("core")}, ours via gdk-pixbuf {acc("ours-file")}, '
      f'combined cache {acc("combined-file")}, stock runner {acc("stock")}, '
      f'stock GdkPixbufLoader {acc("stock-chunk0")}')
print('| outcome | files |')
print('|---|---|')
for c in ('both accept, same RGBA', 'both accept, different RGBA', 'stock only', 'ours only', 'both reject'):
    print(f'| {c} | {len(cats.get(c, []))} |')
for c in ('both accept, different RGBA', 'stock only', 'ours only'):
    if cats.get(c):
        print(f'{c}: {" ".join(cats[c])}')
print(f'has_alpha agrees with stock on {alpha_agree} of {len(both_acc)} files both accept; differs on {len(alpha_diff)}: {" ".join(alpha_diff)}')
print(f'has_alpha true on {sum(r["stock-chunk0"]["has_alpha"] for _, r in both_acc)} files (stock), '
      f'{sum(r["ours-chunk0"]["has_alpha"] for _, r in both_acc)} (ours)')
print(f'gdk_pixbuf_get_file_info reports a size for {info_ok} files')
both = [(n, r) for n, r in rows if r['ours-chunk1'].get('status') == 'ok' and r['stock-chunk0'].get('status') == 'ok']
ex = max(both, key=lambda nr: nr[1]['ours-chunk1']['len'], default=None)
if ex:
    n, r = ex
    s1 = run_json([str(driver), '1', str(Path(args.corpus) / n)], None)
    o1 = r['ours-chunk1']
    print(f'signal timing, 1-byte writes, {n} ({o1["len"]} bytes): '
          f'stock size@{s1.get("size_at")} prepared@{s1.get("prepared_at")} updates={s1.get("n_updated")}; '
          f'ours size@{o1["size_at"]} prepared@{o1["prepared_at"]} updates={o1["n_updated"]} order={o1["order"]}')
for c in (None, combined_cache, ours_cache):
    print(f'save as "bmp" with {c.name if c else "stock cache"}: {run_json([str(driver), "save", "-"], c)}')
print(f'problems: {len(problems)}')
for p in problems:
    print('  ' + p)
sys.exit(1 if problems else 0)
