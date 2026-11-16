#!/usr/bin/env python3
"""Rebuild the real-world PNG corpus outside any repository.

  python3 -I fetch_png.py [--verify] [--refresh-pins] [--only REPO] [--list] [--sleep S]

Same method as fetch_gif.py (whose helpers, via fetch_corpus.py, it reuses): repos-png.tsv lists
the source repos, pins-png.tsv the commit fetched per repo (written on the first run and
reused, so rebuilds fetch identical trees).  Every *.png / *.apng in the pinned tree (subject to
the per-repo caps) is downloaded from the host's raw URL at that commit and its git blob SHA-1
is compared with the tree entry.  Writes $CORPUS_DIR/png/ (default ~/corpora):
  files/<sha256>.png     one file per distinct content
  MANIFEST.tsv           path, sha256, source_url, licence, repo, commit, repo_path, size,
                         typecheck, repo_licence, repo_kind
  FETCH.log              per-repo outcome, including failed downloads
  DUPLICATES.tsv         repo, repo_path, sha256 of every file dropped as an exact duplicate
                         (first occurrence in repos-png.tsv order is the one kept)
and copies MANIFEST.tsv, FETCH.log and DUPLICATES.tsv to manifests/png.* beside this script (text only).

Differences from fetch_gif.py:
  * per-file cap 64 MiB (GIF: 5 MiB), so very large screenshots are kept.  The download is
    refused on a Content-Length above the cap and truncated at cap+1 bytes otherwise; either is
    logged as TOO-LARGE.  Files between 5 and 64 MiB are stored; the stats report how many.
  * per-repo caps: repos-png.tsv column 4 = maximum files (0 = unlimited), column 7 = maximum
    MiB stored per repo (0 = unlimited).  A repo with a cap is read in a fixed pseudo-random
    order (SHA-1 of the path), not alphabetical, so the cap samples the whole tree
    deterministically; an uncapped repo is read in path order.
  * repo_kind (column 6 of repos-png.tsv: tests | mixed | assets | fuzz) is copied into the
    manifest; png_stats.py combines it with path tokens to classify every file.
  * the manifest is checkpointed after each repo, so an interrupted run resumes.
  * 'typecheck' is ok when the file starts with the 8-byte PNG signature; others are kept
    (broken-input cases) and marked fail.
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
import urllib.error
import urllib.request
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location('fetch_corpus', HERE / 'fetch_corpus.py')
fc = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(fc)

MAX_BYTES = 64 * 1024 * 1024
SIG = b'\x89PNG\r\n\x1a\n'
HEADER = '\t'.join(['path', 'sha256', 'source_url', 'licence', 'repo', 'commit', 'repo_path', 'size', 'typecheck',
                    'repo_licence', 'repo_kind'])
EXTS = ('.png', '.apng')


class TooLarge(Exception):
    pass


def get_capped(url, limit, tries=4):
    """Like fc.http_get, but raises TooLarge instead of reading more than limit bytes."""
    last = None
    for i in range(tries):
        try:
            req = urllib.request.Request(url, headers={'User-Agent': 'corpus-rebuild'})
            with urllib.request.urlopen(req, timeout=120) as r:
                cl = r.headers.get('Content-Length')
                if cl and int(cl) > limit:
                    raise TooLarge(f'Content-Length {cl}')
                data = r.read(limit + 1)
                if len(data) > limit:
                    raise TooLarge(f'more than {limit} bytes')
                return data
        except TooLarge:
            raise
        except urllib.error.HTTPError as e:
            last = e
            if e.code in (404, 410, 451):
                break
        except Exception as e:
            last = e
        time.sleep(1.5 * (i + 1))
    raise RuntimeError(f'{url}: {last}')


def write_outputs(out, manifest, log, dups, final):
    manifest = sorted(manifest, key=lambda r: (r[4], r[6]))
    text = HEADER + '\n' + ''.join('\t'.join(r) + '\n' for r in manifest)
    log_text = '\n'.join(log) + '\n'
    dup_text = 'repo\trepo_path\tsha256\n' + ''.join('\t'.join(r) + '\n' for r in dups)
    (out / 'MANIFEST.tsv').write_text(text)
    (out / 'FETCH.log').write_text(log_text)
    (out / 'DUPLICATES.tsv').write_text(dup_text)
    if final:
        (HERE / 'manifests').mkdir(exist_ok=True)
        (HERE / 'manifests' / 'png.MANIFEST.tsv').write_text(text)
        (HERE / 'manifests' / 'png.FETCH.log').write_text(log_text)
        (HERE / 'manifests' / 'png.DUPLICATES.tsv').write_text(dup_text)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--verify', action='store_true')
    ap.add_argument('--refresh-pins', action='store_true')
    ap.add_argument('--only')
    ap.add_argument('--list', action='store_true')
    ap.add_argument('--sleep', type=float, default=0.05)
    a = ap.parse_args()
    root = fc.corpus_root()
    fc.refuse_inside_repo(root)
    if a.verify:
        sys.exit(1 if fc.verify('png', root) else 0)
    repos = fc.read_tsv(HERE / 'repos-png.tsv')
    pins_path = HERE / 'pins-png.tsv'
    pins = {} if a.refresh_pins else {r[0]: r[1] for r in fc.read_tsv(pins_path)}
    out = root / 'png'
    (out / 'files').mkdir(parents=True, exist_ok=True)
    prior = {}
    for r in fc.read_tsv(out / 'MANIFEST.tsv'):
        if r[0] != 'path':
            prior[(r[4], r[6])] = r
    scratch = Path(tempfile.mkdtemp(prefix='corpus-scratch-', dir=root))
    seen, first_of, manifest, log, dups = set(), {}, [], [], []
    try:
        for row in repos:
            repo, licence = row[0], row[2]
            cap = int(row[3])
            repo_licence = row[4] if len(row) > 4 else 'NOASSERTION'
            kind = row[5] if len(row) > 5 else 'mixed'
            cap_bytes = int(row[6]) * 1024 * 1024 if len(row) > 6 else 0
            if a.only and a.only != repo:
                continue
            try:
                commit, tree = fc.list_tree(repo, pins.get(repo), scratch)
            except Exception as e:
                log.append(f'{repo}\tTREE-FAILED\t{str(e)[:200]}')
                print('tree failed', repo, file=sys.stderr, flush=True)
                continue
            pins[repo] = commit
            cands = [(sha, path) for _, sha, _, path in tree if path.lower().endswith(EXTS)]
            if cap or cap_bytes:
                cands.sort(key=lambda c: hashlib.sha1(c[1].encode('utf-8', 'surrogateescape')).hexdigest())
            else:
                cands.sort(key=lambda c: c[1])
            if a.list:
                print(f'{repo}\t{commit[:10]}\t{len(cands)}\tkind={kind}', flush=True)
                continue
            got = fail = dup = reused = big = lfs = nbytes = 0
            capped = ''
            for sha, path in cands:
                if cap and got + dup >= cap:
                    capped = 'FILECAP'
                    break
                if cap_bytes and nbytes >= cap_bytes:
                    capped = 'BYTECAP'
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
                        data = get_capped(url, MAX_BYTES)
                    except TooLarge as e:
                        log.append(f'{repo}\tTOO-LARGE\t{path}\t{e}')
                        big += 1
                        continue
                    except Exception as e:
                        log.append(f'{repo}\tFILE-FAILED\t{path}\t{str(e)[:150]}')
                        fail += 1
                        continue
                    if fc.git_blob_sha(data) != sha:
                        log.append(f'{repo}\tBLOB-MISMATCH\t{path}')
                        fail += 1
                        continue
                if not data:
                    log.append(f'{repo}\tEMPTY\t{path}')
                    continue
                if fc.is_lfs_pointer(data):
                    log.append(f'{repo}\tLFS-POINTER\t{path}')
                    lfs += 1
                    continue
                nbytes += len(data)
                digest = hashlib.sha256(data).hexdigest()
                if digest in seen:
                    dup += 1
                    dups.append([repo, path, digest])
                    continue
                seen.add(digest)
                rel = f'files/{digest}.png'
                if not (out / rel).exists():
                    (out / rel).write_bytes(data)
                manifest.append([rel, digest, url, licence, repo, commit, path, str(len(data)),
                                 'ok' if data.startswith(SIG) else 'fail', repo_licence, kind])
                got += 1
            log.append(f'{repo}\tOK\tcommit={commit}\tcandidates={len(cands)}\tstored={got}\treused={reused}\tduplicates={dup}'
                       f'\tfailed={fail}\ttoo_large={big}\tlfs_pointers={lfs}\tbytes={nbytes}' + (f'\tstopped_by={capped}' if capped else ''))
            print(f'{repo}: stored {got} (reused {reused}), candidates {len(cands)}, dup {dup}, failed {fail}, '
                  f'too large {big}, lfs {lfs}, {nbytes} bytes {capped}', flush=True)
            if not a.only:  # checkpoint so an interrupted run resumes
                pins_path.write_text('# repo\tcommit (anonymous fetch pins; keep so rebuilds are identical)\n'
                                     + ''.join(f'{r}\t{c}\n' for r, c in sorted(pins.items())))
                write_outputs(out, manifest, log, dups, False)
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    if a.only or a.list:
        return
    write_outputs(out, manifest, log, dups, True)
    nb = sum(int(r[7]) for r in manifest)
    print(f'png: {len(manifest)} distinct files, {sum(r[8] == "ok" for r in manifest)} start with a PNG signature, '
          f'{nb} bytes, {len(dups)} exact duplicates dropped, into {out}')


if __name__ == '__main__':
    main()
