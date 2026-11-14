#!/usr/bin/env python3
"""Rebuild the real-world GIF corpus outside any repository.

  python3 -I fetch_gif.py [--verify] [--refresh-pins] [--only REPO] [--list] [--sleep S]

Same method as fetch_corpus.py (whose helpers it imports): repos-gif.tsv lists the
source repos, pins-gif.tsv the commit fetched per repo (written on the first run and
reused, so rebuilds fetch identical trees).  Every *.gif in the pinned tree is
downloaded from the host's raw URL at that commit and its git blob SHA-1 is compared
with the tree entry.  Writes $CORPUS_DIR/gif/ (default ~/corpora):
  files/<sha256>.gif     one file per distinct content
  MANIFEST.tsv           path, sha256, source_url, licence, repo, commit, repo_path,
                         size, typecheck, repo_licence
  FETCH.log              per-repo outcome, including failed downloads
and copies MANIFEST.tsv / FETCH.log to manifests/gif.* beside this script (text only).

Differences from fetch_corpus.py:
  * resumable: a (repo, path) whose row is already in MANIFEST.tsv and whose file is
    present with the recorded SHA-256 is not downloaded again (the tree is still listed,
    and the blob SHA-1 of the stored bytes is checked against it);
  * polite: --sleep seconds (default 0.15) between downloads;
  * 'typecheck' is ok when the file starts with GIF87a or GIF89a. Files that do not are
    kept (they are "known broken" test cases) and marked fail.
  * the licence column is the claim for the test file; NOASSERTION unless the repo
    states a licence that plainly covers its test images. repo_licence is informational.
Third-party bytes are never written under a repository (refused if CORPUS_DIR is in a work tree).
--verify re-hashes files/ against MANIFEST.tsv, offline.
--list prints candidate counts per repo (tree listing only, no file downloads).
"""
import argparse
import hashlib
import importlib.util
import shutil
import sys
import tempfile
import time
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location('fetch_corpus', HERE / 'fetch_corpus.py')
fc = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(fc)

MAX_BYTES = 5 * 1024 * 1024
HEADER = '\t'.join(['path', 'sha256', 'source_url', 'licence', 'repo', 'commit', 'repo_path', 'size', 'typecheck', 'repo_licence'])


def typecheck(b):
    return b[:6] in (b'GIF87a', b'GIF89a')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--verify', action='store_true')
    ap.add_argument('--refresh-pins', action='store_true')
    ap.add_argument('--only')
    ap.add_argument('--list', action='store_true')
    ap.add_argument('--sleep', type=float, default=0.15)
    a = ap.parse_args()
    root = fc.corpus_root()
    fc.refuse_inside_repo(root)
    if a.verify:
        sys.exit(1 if fc.verify('gif', root) else 0)
    repos = fc.read_tsv(HERE / 'repos-gif.tsv')
    pins_path = HERE / 'pins-gif.tsv'
    pins = {} if a.refresh_pins else {r[0]: r[1] for r in fc.read_tsv(pins_path)}
    out = root / 'gif'
    (out / 'files').mkdir(parents=True, exist_ok=True)
    prior = {}
    for r in fc.read_tsv(out / 'MANIFEST.tsv'):
        if r[0] != 'path':
            prior[(r[4], r[6])] = r
    scratch = Path(tempfile.mkdtemp(prefix='corpus-scratch-', dir=root))
    seen, manifest, log = set(), [], []
    try:
        for row in repos:
            repo, licence = row[0], row[2]
            cap = int(row[3])
            repo_licence = row[4] if len(row) > 4 else 'NOASSERTION'
            if a.only and a.only != repo:
                continue
            try:
                commit, tree = fc.list_tree(repo, pins.get(repo), scratch)
            except Exception as e:
                log.append(f'{repo}\tTREE-FAILED\t{str(e)[:200]}')
                print('tree failed', repo, file=sys.stderr)
                continue
            pins[repo] = commit
            cands = [(sha, path) for _, sha, _, path in sorted(tree, key=lambda t: t[3])
                     if path.lower().endswith('.gif')]
            if a.list:
                print(f'{repo}\t{commit[:10]}\t{len(cands)}')
                continue
            got = fail = dup = reused = 0
            for sha, path in cands:
                if cap and got + dup >= cap:
                    break
                url = fc.raw_url(repo, commit, path)
                data = None
                pr = prior.get((repo, path))
                if pr and pr[5] == commit:
                    p = out / pr[0]
                    if p.exists():
                        d = p.read_bytes()
                        if hashlib.sha256(d).hexdigest() == pr[1] and fc.git_blob_sha(d) == sha:
                            data = d
                            reused += 1
                if data is None:
                    time.sleep(a.sleep)
                    try:
                        data = fc.http_get(url)
                    except Exception as e:
                        log.append(f'{repo}\tFILE-FAILED\t{path}\t{str(e)[:150]}')
                        fail += 1
                        continue
                    if fc.git_blob_sha(data) != sha:
                        log.append(f'{repo}\tBLOB-MISMATCH\t{path}')
                        fail += 1
                        continue
                if len(data) > MAX_BYTES or not data:
                    log.append(f'{repo}\tTOO-LARGE-OR-EMPTY\t{path}\t{len(data)}')
                    continue
                if fc.is_lfs_pointer(data):
                    log.append(f'{repo}\tLFS-POINTER\t{path}')
                    continue
                digest = hashlib.sha256(data).hexdigest()
                if digest in seen:
                    dup += 1
                    continue
                seen.add(digest)
                rel = f'files/{digest}.gif'
                if not (out / rel).exists():
                    (out / rel).write_bytes(data)
                manifest.append([rel, digest, url, licence, repo, commit, path, str(len(data)),
                                 'ok' if typecheck(data) else 'fail', repo_licence])
                got += 1
            log.append(f'{repo}\tOK\tcommit={commit}\tcandidates={len(cands)}\tstored={got}\treused={reused}\tduplicates={dup}\tfailed={fail}')
            print(f'{repo}: stored {got} (reused {reused}), candidates {len(cands)}, dup {dup}, failed {fail}', flush=True)
            if not a.only:  # checkpoint so an interrupted run resumes
                pins_path.write_text('# repo\tcommit (anonymous fetch pins; keep so rebuilds are identical)\n'
                                     + ''.join(f'{r}\t{c}\n' for r, c in sorted(pins.items())))
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    if a.only or a.list:
        return
    manifest.sort(key=lambda r: (r[4], r[6]))
    text = HEADER + '\n' + ''.join('\t'.join(r) + '\n' for r in manifest)
    log_text = '\n'.join(log) + '\n'
    (out / 'MANIFEST.tsv').write_text(text)
    (out / 'FETCH.log').write_text(log_text)
    (HERE / 'manifests').mkdir(exist_ok=True)
    (HERE / 'manifests' / 'gif.MANIFEST.tsv').write_text(text)
    (HERE / 'manifests' / 'gif.FETCH.log').write_text(log_text)
    nb = sum(int(r[7]) for r in manifest)
    print(f'gif: {len(manifest)} distinct files, {sum(r[8] == "ok" for r in manifest)} start with a GIF signature, {nb} bytes, into {out}')


if __name__ == '__main__':
    main()
