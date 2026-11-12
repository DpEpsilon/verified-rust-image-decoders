#!/usr/bin/env python3
"""Rebuild a real-world test-file corpus outside any repository.

  python3 -I fetch_corpus.py FORMAT [--verify] [--refresh-pins] [--only REPO]
  FORMAT = icns | tga | ico | bmp

Reads  repos-FORMAT.tsv  (repo, notes_count, licence, cap) and pins-FORMAT.tsv
(repo -> full commit hash; written on the first run, then reused so later
rebuilds fetch the same trees).  Writes  $CORPUS_DIR/FORMAT/  (default ~/corpora):
  files/<sha256>.<ext>   one file per distinct content
  MANIFEST.tsv           path, sha256, source URL, licence, repo, commit, repo_path, size, typecheck
  FETCH.log              per-repo outcome, including failures
and copies MANIFEST.tsv to manifests/FORMAT.MANIFEST.tsv beside this script
(text only: hashes and URLs, no third-party bytes).

How a repo is read: a blobless shallow clone into a scratch directory gives the
file tree at the pinned commit (no file contents are transferred); every
candidate file is then downloaded from the host's raw URL at that commit and its
git blob SHA-1 is compared with the tree entry, so a truncated or substituted
download is rejected.  Git LFS pointer files are dropped.  Files over 5 MiB are
dropped (BMP: over 1024 KiB, as in dib/tools/fetch_corpus.sh).

Corpus bytes are never written under a repository.  The script refuses a
CORPUS_DIR inside a git work tree.

--verify re-hashes every file in files/ against MANIFEST.tsv and exits non-zero
on any mismatch or missing file; it does no network access.
"""
import argparse
import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
MAX_BYTES = 5 * 1024 * 1024
EXTS = {'icns': {'icns'}, 'tga': {'tga', 'targa'}, 'ico': {'ico', 'cur'},
        'bmp': {'bmp', 'ico', 'cur'}}
HEADER = '\t'.join(['path', 'sha256', 'source_url', 'licence', 'repo', 'commit', 'repo_path', 'size', 'typecheck'])


def corpus_root():
    return Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve()


def refuse_inside_repo(path):
    p = path
    while not p.exists():
        p = p.parent
    r = subprocess.run(['git', '-C', str(p), 'rev-parse', '--show-toplevel'], capture_output=True, text=True)
    if r.returncode == 0:
        sys.exit(f'refusing: {path} is inside git work tree {r.stdout.strip()}; corpus bytes never go in a repo')


def host_of(repo):
    if repo.startswith('gitlab.gnome.org/'):
        return 'gitlab', repo[len('gitlab.gnome.org/'):], 'https://gitlab.gnome.org/' + repo[len('gitlab.gnome.org/'):] + '.git'
    return 'github', repo, f'https://github.com/{repo}.git'


def raw_url(repo, commit, path):
    kind, name, _ = host_of(repo)
    q = urllib.parse.quote(path)
    if kind == 'github':
        return f'https://raw.githubusercontent.com/{name}/{commit}/{q}'
    return f'https://gitlab.gnome.org/{name}/-/raw/{commit}/{q}'


def git(*args, cwd=None, check=True):
    env = dict(os.environ, GIT_TERMINAL_PROMPT='0', GIT_ASKPASS='true')
    # anonymous access only: never use a stored credential for third-party repos
    cmd = ['git', '-c', 'credential.helper=', *args]
    return subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, check=check)


def list_tree(repo, pin, scratch):
    """Return (commit, [(mode, blob_sha, size, path)]) for the pinned or default-branch tree."""
    _, _, url = host_of(repo)
    d = Path(tempfile.mkdtemp(prefix='tree-', dir=scratch))
    try:
        if pin:
            git('init', '-q', str(d))
            git('-C', str(d), 'fetch', '-q', '--depth', '1', '--filter=blob:none', url, pin)
            commit = pin
            ref = 'FETCH_HEAD'
        else:
            git('clone', '-q', '--depth', '1', '--filter=blob:none', '--no-checkout', url, str(d))
            ref = 'HEAD'
            commit = git('-C', str(d), 'rev-parse', 'HEAD').stdout.decode().strip()
        out = git('-C', str(d), 'ls-tree', '-r', '-z', ref).stdout
        rows = []
        for ent in out.split(b'\0'):
            if not ent:
                continue
            meta, path = ent.split(b'\t', 1)
            mode, typ, sha = meta.split()
            if typ == b'blob' and mode in (b'100644', b'100755'):
                rows.append((mode.decode(), sha.decode(), 0, path.decode('utf-8', 'surrogateescape')))
        return commit, rows
    finally:
        shutil.rmtree(d, ignore_errors=True)


def http_get(url, tries=4):
    last = None
    for i in range(tries):
        try:
            req = urllib.request.Request(url, headers={'User-Agent': 'corpus-rebuild'})
            with urllib.request.urlopen(req, timeout=60) as r:
                return r.read()
        except urllib.error.HTTPError as e:
            last = e
            if e.code in (404, 410, 451):
                break
        except Exception as e:
            last = e
        time.sleep(1.5 * (i + 1))
    raise RuntimeError(f'{url}: {last}')


def git_blob_sha(data):
    return hashlib.sha1(b'blob %d\0' % len(data) + data).hexdigest()


def is_lfs_pointer(data):
    return len(data) < 1024 and data.startswith(b'version https://git-lfs')


def typecheck(fmt, b):
    """Magic/header plausibility, independent of the file extension."""
    if fmt == 'icns':
        return len(b) >= 8 and b[:4] == b'icns' and 8 <= int.from_bytes(b[4:8], 'big') <= len(b)
    if fmt == 'ico':
        return (len(b) >= 22 and b[:2] == b'\0\0' and b[2:4] in (b'\1\0', b'\2\0')
                and 1 <= int.from_bytes(b[4:6], 'little') and 6 + 16 * int.from_bytes(b[4:6], 'little') <= len(b))
    if fmt == 'tga':
        if len(b) < 18:
            return False
        cm, it, depth = b[1], b[2], b[16]
        w, h = int.from_bytes(b[12:14], 'little'), int.from_bytes(b[14:16], 'little')
        return (cm in (0, 1) and it in (1, 2, 3, 9, 10, 11) and depth in (8, 15, 16, 24, 32)
                and w > 0 and h > 0 and (cm == 1) == (it in (1, 9)) or
                (cm == 0 and it in (2, 3, 10, 11) and depth in (8, 15, 16, 24, 32) and w > 0 and h > 0))
    if fmt == 'bmp':
        return len(b) >= 26 and (b[:2] == b'BM' or b[:2] == b'\0\0')
    return True


def read_tsv(path):
    rows = []
    if path.exists():
        for line in path.read_text().splitlines():
            if line and not line.startswith('#'):
                rows.append(line.split('\t'))
    return rows


def verify(fmt, root):
    d = root / fmt
    bad = 0
    n = 0
    for row in [r for r in read_tsv(d / 'MANIFEST.tsv') if r[0] != 'path']:
        n += 1
        p = d / row[0]
        if not p.exists():
            print('MISSING', row[0])
            bad += 1
        elif hashlib.sha256(p.read_bytes()).hexdigest() != row[1]:
            print('MISMATCH', row[0])
            bad += 1
    print(f'{fmt}: {n} manifest rows, {bad} bad')
    return bad


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('format', choices=sorted(EXTS))
    ap.add_argument('--verify', action='store_true')
    ap.add_argument('--refresh-pins', action='store_true', help='ignore pins-FORMAT.tsv and take current default-branch HEADs')
    ap.add_argument('--only', help='process one repo from repos-FORMAT.tsv')
    a = ap.parse_args()
    fmt = a.format
    root = corpus_root()
    refuse_inside_repo(root)
    if a.verify:
        sys.exit(1 if verify(fmt, root) else 0)

    repos = read_tsv(HERE / f'repos-{fmt}.tsv')
    pins_path = HERE / f'pins-{fmt}.tsv'
    pins = {} if a.refresh_pins else {r[0]: r[1] for r in read_tsv(pins_path)}
    out = root / fmt
    (out / 'files').mkdir(parents=True, exist_ok=True)
    scratch = Path(tempfile.mkdtemp(prefix='corpus-scratch-', dir=root))
    bmp = fmt == 'bmp'
    limit = 1024 * 1024 if bmp else MAX_BYTES
    seen = set()
    manifest = []
    log = []
    try:
        for row in repos:
            repo, licence = row[0], row[2] if len(row) > 2 else 'unknown'
            cap = int(row[3]) if len(row) > 3 else 0
            if a.only and a.only != repo:
                continue
            try:
                commit, tree = list_tree(repo, pins.get(repo), scratch)
            except Exception as e:
                log.append(f'{repo}\tTREE-FAILED\t{str(e)[:200]}')
                print('tree failed', repo, file=sys.stderr)
                continue
            pins[repo] = commit
            cands = []
            for mode, sha, size, path in sorted(tree, key=lambda t: t[3]):
                if path.rsplit('.', 1)[-1].lower() not in EXTS[fmt] or '.' not in path.rsplit('/', 1)[-1]:
                    continue
                if bmp and not path.startswith('Tests/images/'):
                    continue
                cands.append((sha, size, path))
            got = fail = dup = 0
            ok_count = 0
            for sha, size, path in cands:
                if cap and ok_count >= cap:
                    break
                url = raw_url(repo, commit, path)
                try:
                    data = http_get(url)
                except Exception as e:
                    log.append(f'{repo}\tFILE-FAILED\t{path}\t{str(e)[:150]}')
                    fail += 1
                    continue
                if git_blob_sha(data) != sha:
                    log.append(f'{repo}\tBLOB-MISMATCH\t{path}')
                    fail += 1
                    continue
                if len(data) > limit or not data:
                    log.append(f'{repo}\tTOO-LARGE-OR-EMPTY\t{path}\t{len(data)}')
                    continue
                if is_lfs_pointer(data):
                    log.append(f'{repo}\tLFS-POINTER\t{path}')
                    continue
                tc = typecheck(fmt, data)
                if tc:
                    ok_count += 1
                digest = hashlib.sha256(data).hexdigest()
                if digest in seen:
                    dup += 1
                    continue
                seen.add(digest)
                ext = path.rsplit('.', 1)[-1].lower()
                rel = f'files/{digest}.{ext}'
                (out / rel).write_bytes(data)
                manifest.append([rel, digest, url, licence, repo, commit, path, str(len(data)), 'ok' if tc else 'fail'])
                got += 1
            log.append(f'{repo}\tOK\tcommit={commit}\tcandidates={len(cands)}\tstored={got}\ttypecheck_ok={ok_count}\tduplicates={dup}\tfailed={fail}')
            print(f'{repo}: stored {got} (typecheck ok {ok_count}), candidates {len(cands)}', flush=True)
    finally:
        shutil.rmtree(scratch, ignore_errors=True)

    if a.only:
        return
    pins_path.write_text('# repo\tcommit (anonymous fetch pins; keep so rebuilds are identical)\n'
                         + ''.join(f'{r}\t{c}\n' for r, c in sorted(pins.items())))
    manifest.sort(key=lambda r: (r[4], r[6]))
    text = HEADER + '\n' + ''.join('\t'.join(r) + '\n' for r in manifest)
    (out / 'MANIFEST.tsv').write_text(text)
    (out / 'FETCH.log').write_text('\n'.join(log) + '\n')
    (HERE / 'manifests').mkdir(exist_ok=True)
    (HERE / 'manifests' / f'{fmt}.MANIFEST.tsv').write_text(text)
    (HERE / 'manifests' / f'{fmt}.FETCH.log').write_text('\n'.join(log) + '\n')
    n_ok = sum(r[8] == 'ok' for r in manifest)
    print(f'{fmt}: {len(manifest)} distinct files stored, {n_ok} pass the type check, into {out}')


if __name__ == '__main__':
    main()
