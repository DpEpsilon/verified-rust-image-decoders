# Real-world corpora: acquisition scripts

This directory holds only scripts, repo lists, pins and manifests (text). The corpus bytes live in `~/corpora/<format>/`, outside every repository and outside `/tmp`, which is not durable.

**Rule: corpus bytes never go in a repo.** The files are third-party (mixed licences, many unknown), used only for local measurement. Never copy them into `~/projects`, commit them, or publish witnesses derived from them. `fetch_corpus.py` refuses a `CORPUS_DIR` inside a git work tree.

## Layout

```
~/corpora/<fmt>/files/<sha256>.<ext>   one file per distinct content (fmt = icns, tga, ico, bmp, gif)
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

## GIF corpus

```sh
cd ~/projects/verified-decoders/leniency/corpora
python3 -I fetch_gif.py [--list] [--only REPO] [--sleep 0.15]   # ~25 min, one blobless clone + raw downloads per repo
python3 -I fetch_gif.py --verify                                 # offline re-hash against MANIFEST.tsv
python3 -I gif_incumbents.py                                     # needs ../gif/build-min.sh (or build.sh); writes gif-incumbents.json
python3 -I gif_stats.py                                          # writes gif-STATS.md and gif-stats.json
```

`fetch_gif.py` takes every `*.gif` (files over 5 MiB and empty files dropped) from the repos in `repos-gif.tsv` at the commits in `pins-gif.tsv`, checks each against the git blob SHA-1, and stores it as `~/corpora/gif/files/<sha256>.gif`. It is resumable (rows already in `MANIFEST.tsv` with a matching file are not downloaded again) and sleeps between downloads. `MANIFEST.tsv` has one extra column, `repo_licence`; the `licence` column is NOASSERTION for every file because the repos license their code, not their test images. `typecheck=fail` marks files that do not start with `GIF87a`/`GIF89a` (kept: they are broken-input cases).

Not obtainable from this sandbox (network allowlist): Wikimedia Commons (commons.wikimedia.org, upload.wikimedia.org), chromium.googlesource.com, wpt.live, and Git LFS storage (github-cloud.githubusercontent.com, media.githubusercontent.com). The last one is why `SixLabors/ImageSharp` contributes 0 files: its 107 GIFs (including `issue1668_invalidcolorindex.gif` and `lzw-code-outside-table.gif`) are LFS pointers and are logged as `LFS-POINTER` in `manifests/gif.FETCH.log`. No GIF torture-style conformance set was found on GitHub; `libgd/libgd` `tests/gif/gif-conformance/` and the gdk-pixbuf `gif-test-suite/` are the closest.

`gif_stats.py` and `gif_incumbents.py` write only file names and numbers. The prose in `gif-STATS.md` between the generated tables comes from `gif-STATS-interpretation.md`.
