# Real-world corpora: acquisition scripts

This directory holds only scripts, repo lists, pins and manifests (text). The corpus bytes live in `~/corpora/<format>/`, outside every repository and outside `/tmp`, which is not durable.

**Rule: corpus bytes never go in a repo.** The files are third-party (mixed licences, many unknown), used only for local measurement. Never copy them into `~/projects`, commit them, or publish witnesses derived from them. `fetch_corpus.py` refuses a `CORPUS_DIR` inside a git work tree.

## Layout

```
~/corpora/<fmt>/files/<sha256>.<ext>   one file per distinct content (fmt = icns, tga, ico, bmp)
~/corpora/<fmt>/MANIFEST.tsv           path, sha256, source_url, licence, repo, commit, repo_path, size, typecheck
~/corpora/<fmt>/FETCH.log              per-repo outcome, including failed downloads
```

Here:

- `fetch_corpus.py`: the acquisition tool.
- `repos-<fmt>.tsv`: source repos (repo, count in NOTES.md, licence label, per-repo cap).
- `pins-<fmt>.tsv`: the commit fetched per repo. Written on the first run and reused afterwards, so rebuilds fetch the same trees. `--refresh-pins` takes current HEADs instead.
- `manifests/<fmt>.MANIFEST.tsv`, `manifests/<fmt>.FETCH.log`: text copies of what a run produced.
- `NOTES.md`: the original source description, counts and licences (kept at `~/projects/leniency/corpora/NOTES.md`).

## Rebuild

```sh
cd ~/projects/verified-decoders/leniency/corpora
python3 -I fetch_corpus.py tga      # likewise icns, ico, bmp; run them serially
python3 -I fetch_corpus.py tga --verify   # offline re-hash against MANIFEST.tsv
```

Each file is downloaded from the host's raw URL at the pinned commit and checked against its git blob SHA-1. LFS pointers and files over 5 MiB (bmp: 1 MiB) are dropped. The `typecheck` column records magic/header plausibility; files that fail are kept but marked `fail`.

## Using the corpora

Harnesses read `$CORPUS_DIR/<fmt>/files/` with `CORPUS_DIR` defaulting to `~/corpora`: `tga/policy/validate.py`, `tga/policy/investigate_pixdiff.py` and `leniency/bench/bench.py`. Set `CORPUS_DIR` to relocate. `bmp` is the Pillow `Tests/images` set pinned at `aa60851e` (84 files, identical to `dib/corpus`).

## Not reproducible

- The original in-the-wild files came from GitHub code search, which needs an authenticated token; only the repos named in `NOTES.md` are rebuilt, at current HEADs (original commits were not recorded), so counts differ from NOTES.md. Compare each `MANIFEST.tsv` with the counts there.
- `results.jsonl` and `pixdiff.tsv` (native decoder results that `validate.py` reads from `~/corpora/tga/`) are outputs of the harness, not of acquisition; regenerate them by running the harness on the rebuilt files.
