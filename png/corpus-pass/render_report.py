#!/usr/bin/env python3
"""Write REPORT.md: the hand-written conclusions and method below, then the tables generated from results.json
(corpus_pass.py report) and counts.json (reconcile_counts.py --json).

  python3 -I render_report.py
"""
import json
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BEGIN, END = '<!-- tables:begin -->', '<!-- tables:end -->'

CONCLUSIONS = """## Conclusions

Under the 2026-11-17 caps, Strict and RecommendedBeta ran to completion on all 21,966 files of the pinned real corpus and on 25 project-authored synthetic files: no timeout, crash or SHA-256 mismatch. RecommendedBeta rejects no file that Strict accepts, and obligation R holds with 0 violations over 21,401 Strict-accepted real stills and 22,555 Strict frames. No real file reaches a reservation above 464.9 MiB, and no capped input in the synthetic set reaches one above 576 MiB, so `MAX_BYTES` = 2^30 is never the binding limit. Counts are files. Tables T1 to T13 follow the method.

**Composition.** There are 8,296 real incidental assets (web/doc images 4,750, icons 2,109, wallpapers 1,210, screenshots 227; this split uses a path heuristic and no decision relies on it), 13,323 test files and 347 fuzz files. The hold-out (`HOLDOUT.tsv`, PNG-Q14) holds 5,059 of those files from 10 whole repositories: 3,403 incidental, 1,626 test and 30 fuzz. The other 16,907 files are the development set. The 25 synthetic files are separate and never counted as real: 9 multi-monitor screenshots, 4 cap-boundary images, 7 long thin images, 3 stored near-cap images and 2 files at the input-cap boundary. Every synthetic file's outcome matched the outcome written into its manifest before the run.

**Strict errors on real files (T2).** Strict rejects 565 real files: 179 incidental, 280 test and 106 fuzz. 44 of the 62 errors occur on at least one real file. On incidental assets only four errors occur: `TrailingBytes` 107, `ChunkOutOfOrder` 64 (every one an eXIf after IDAT), `BadSignature` 7 (files that are not PNGs) and `InvalidExif` 1. Over the whole corpus the most frequent are `TrailingBytes` 147, `TruncatedChunk` 133, `ChunkOutOfOrder` 99, `CrcMismatch` 40 and `BadSignature` 22. Eighteen errors occur on no real file, among them all the deflate code errors (`InvalidSymbol`, `InvalidCodeLengths`, `InvalidBlockType`), `NonConsecutiveIdat`, `UnknownCriticalChunk`, `PixelBudgetExceeded` and `IccProfileTooLarge`; the vectors still exercise them.

**RecommendedBeta fields (T5 still, T6 stream).** RecommendedBeta accepts 313 files that Strict rejects (172 incidental, 107 test, 34 fuzz; 22 of them in the hold-out). Every one of these flips is explained by the field reversions: 308 need a single field, 5 need two or three fields, and none is unexplained. A field "acts" on a file when reverting that field alone to its Strict value changes RecommendedBeta's outcome. Final outcomes per field on real files:
- `trailing`: acts on 147 (107 incidental), all accepted.
- `ancillary_order`: acts on 97 (64 incidental), all accepted.
- `ancillary_values`: acts on 26 (1 incidental); 14 accepted, 12 rejected later, 6 of them by `CrcMismatch`.
- `apng_in_still` (still) and `apng_errors=Static` (stream): act on 20 and 19 test files; 18 and 17 accepted (R4 on the 17 streams).
- `crc_ancillary=SkipExceptFdat`: acts on 13 test and fuzz files; 9 accepted.
- `ancillary_duplicate`: acts on 12; 11 accepted in the still view (12 in the stream view).
- `palette_index`: acts on 11 test files, all accepted with R3.
- `palette_size`: acts on 7; 1 accepted.
- `missing_iend=AtEof`: acts on 7 fuzz files; 5 accepted.
- `window` and `zlib_tail=IgnoreTail`: act on 2 test files each, all accepted.
- `plte_unexpected`, `trns_invalid`: act on 2 each; 1 accepted each.
- `iend=Lenient`: acts on no file.

Only `trailing`, `ancillary_order` and `ancillary_values` act on any incidental asset. The 252 files that RecommendedBeta still rejects contain no incidental asset except the 7 non-PNG files.

**Caps and `MAX_BYTES` (T8 to T10).** Raising the caps changed the outcome of 45 real files and of no other real file: 49 real files exceed a 2026-11-16 cap, and 4 of them keep their outcome. Of the 45, 5 changed only the error name, from `InvalidDimensions` to `OutputTooLarge`. File by file against the 2026-11-16 pass (T11), 20,497 of the 20,502 files that both passes ran have the same outcome and the same Strict RGBA. The 5 that changed are the test files with a side above 16,384 (65,535 × 1, 1 × 65,535, 32,768 × 100, 524,288 × 2, 2 × 524,288), which Strict now accepts; all of them have long been decoded by every incumbent. The oracle repairs of 2026-11-17 (D4, D5, D20) changed no real file. 1,464 files ran for the first time (the old pass's unsampled 1 to 16.7 Mpx files and the files over the old caps).
- **Output.** Strict now accepts 35 real files that the old caps rejected (30 over the 64 MiB output cap, 5 over the side cap); RecommendedBeta accepts 40, the extra 5 being `TrailingBytes` or `ChunkOutOfOrder` wallpapers. The largest accepted real image is 60.9 Mpx, 232.4 MiB as RGBA8 (an incidental asset in the hold-out). The largest in the development set is a 10,000 × 5,625 wallpaper, 214.6 MiB. Two real files exceed 128 MiB and none exceeds 256 MiB. Under the 256 MiB cap Strict rejects six real files with `OutputTooLarge`. Four of them declare more than 1 Gpx and are bombs or crashers. The other two are valid test images that the incumbents decode: `gdk-pixbuf tests/large.png` (839 Mpx) and the hold-out file `opencv_extra read.png` (181 Mpx). No permitted `MAX_OUTPUT` covers either one: the envelope's upper bound, 429,496,728 bytes, is 107 Mpx.
- **Side.** The longest accepted side is 524,288 pixels (2 × 524,288, test). Among incidental assets it is 16,384. Of 41 thin real files (aspect at least 32:1), 37 are accepted. The other four fail for non-cap reasons (`CrcMismatch` 2, `InvalidChunkType` 1) or for the intended `OutputTooLarge` of `malformed-size.apng` (524,288 × 2,048). The synthetic files show that the boundary sits where the caps put it: 67,108,864 × 1 and 1 × 67,108,864 RGBA8 are accepted and 1 × 67,108,865 is `OutputTooLarge`; 2^31 − 1 × 1 is `OutputTooLarge` and 2^31 × 1 is `InvalidDimensions`.
- **Input.** The largest real file is 44.2 MiB (46,397,113 bytes), so no real file comes near either input cap. Stored synthetic images within the output cap are 256.1 MiB (8,192 × 8,192 RGBA8), 512.1 MiB (RGBA16) and 576.1 MiB (1 × 2^26 RGBA16 Adam7). The old 256 MiB `MAX_INPUT` rejected all three with `FileTooLarge`; 2^30 accepts them. A file of exactly 2^30 bytes is accepted and one of 2^30 + 1 bytes is `FileTooLarge`.
- **Multi-monitor screenshots.** The real corpus has none; its 227 files with "screen" in the path are at most 4K. All 8 synthetic layouts up to 15,360 × 4,320 (4 × 2 4K panels, 253.1 MiB RGBA8) are accepted. The 23,040 × 4,320 layout (3 × 8K, 379.7 MiB) is `OutputTooLarge`. The synthetic screenshots repeat every 256 rows, so they test sizes, not compression ratios.
- **`MAX_BYTES`.** Among accepted real files the largest single reservations are `R` = 232.4 MiB and the RGBA16 output (`decode_still16`, 8N) = 464.9 MiB. Among synthetic files they are `R` = 576.0 MiB (1 × 2^26 RGBA16 Adam7, the worst shape in `LIMITS.md`) and the RGBA16 output = 512.0 MiB. No reservation exceeds 2^30. The live-heap sum `R + 8N` for that worst shape is 1,088 MiB, which matches `LIMITS.md`. It exceeds 2^30 as an aggregate, but `MAX_BYTES` bounds each reservation, not the sum.
- **Other caps.** The largest values among accepted real files are 128 frames (`MAX_FRAMES` 65,536), an fcTL pixel sum of 112.5 Mpx (about a tenth of `MAX_TOTAL_PIXELS` = 2^30) and an inflated iCCP of 1.4 MB (about a twelfth of `MAX_ICC` = 16 MiB). One test file declares 2,147,483,649 frames, and Strict rejects it with `TooManyFrames`, as intended. These three caps were not changed on 2026-11-17 and were not stressed by synthetic files.

**Count reconciliation (T13).** All six numbers recompute exactly from the files on disk. 748 is the number of records in `probes/results.json` (516 vector, 177 probe, 55 witness; 14 above 4 KiB stored by reference). The Strict bridge counts distinct non-padded inputs: 737 = 734 inline + 533 vectors − 530 shared; then +3 OVER corner vectors = 740; +198 profile vectors, of which only 27 are new byte strings, = 767; +6 cap-boundary vectors = 773; +14 review and council vectors, 11 of them new byte strings, = 784. The step from 740 to 767 adds 27 inputs, not 198, because most profile vectors repeat bytes the bridge already holds. PLAN.md's wording "767 after the 198 profile vectors" is correct about the vectors and silent on that.

## Recommendations for the Stage-2 items (recommendations only; nothing in spec-frozen, limits-draft, policy-draft, THEOREMS.md or ERRATA.md was edited)

These recommendations are for the owner and the council. The cap evidence is quoted for the development set, with hold-out files named where they would change a number. The hold-out was run through Strict and RecommendedBeta only, as on 2026-11-16, and not through any compatibility profile.

1. *Recommendation:* freeze `MAX_OUTPUT` = 256 MiB. It covers every accepted development file with 41.4 MiB of headroom (214.6 MiB at most), and every accepted real file including the hold-out (232.4 MiB). The two valid images it rejects (839 Mpx and 181 Mpx test files) are beyond the largest permitted value, so raising the cap within the envelope would gain nothing. A real multi-monitor screenshot beyond 4 × 2 4K (more than 67 Mpx) would still be rejected; no corpus file shows one.
2. *Recommendation:* freeze `MAX_DIMENSION` = 2^31 − 1. It admits 5 valid test files that 16,384 rejected, and changes no other outcome. Because `MAX_OUTPUT` bounds the area, the effective longest side is 2^26.
3. *Recommendation:* freeze `MAX_INPUT` = 2^30, confirming the coordinator's call of 2026-11-17. Real files need only 44.2 MiB, but stored images within the output cap need up to 576.1 MiB, which 256 MiB rejects.
4. *Recommendation:* the corpus asks for no change to `MAX_BYTES` = 2^30 (E16, already approved). The largest reservation is 576 MiB (synthetic) and 464.9 MiB (real).
5. *Recommendation:* freeze `MAX_FRAMES`, `MAX_TOTAL_PIXELS` and `MAX_ICC` at their draft values. The corpus maxima are 128 frames, 112.5 Mpx and 1.4 MB, and nothing argues for another value.
6. *Recommendation:* freeze RecommendedBeta's 15 fields as drafted (policy draft 0.2). No field rejects a Strict-accepted file, obligation R holds, and every field except `iend` and `apng_errors` (in the still view) acts on at least one real file. Small calls for the council:
   - `iend=Lenient` acts on no real file. Keeping it costs nothing measurable here, and the probes show every incumbent accepts a damaged IEND. The corpus neither supports nor contradicts it.
   - `missing_iend=AtEof` acts only on 7 fuzz files (5 accepted). It is already recorded as a design choice that `image` does not share.
   - `palette_size`, `plte_unexpected` and `trns_invalid` each lead to acceptance of one real file, and none is incidental.
   - The incidental-asset case rests on `trailing` and `ancillary_order`. Their incidental flips are concentrated: on 2026-11-16, 95 of the `trailing` flips came from one wallpaper repository and 51 of the `ancillary_order` flips from another.
7. The hold-out is still unspent for the compatibility profiles. GdkCompat and ImageCrateCompat have not been run on any file, so PNG-Q14's re-measurement can use `HOLDOUT.tsv` as it stands.
"""

METHOD = """## Method

- **Corpus.** The pinned real corpus of 2026-11-16: `~/corpora/png/MANIFEST.tsv` (SHA-256 `f73984e8…62996`, checked on every run), 21,966 distinct files from 87 repository trees; every file's SHA-256 is re-checked against the manifest when it is read. Classes and the hold-out are those of `../../leniency/corpora/png_stats.py` and `holdout-png.tsv`, unchanged: *incidental* (asset or mixed repositories, path without a test word), *test*, *fuzz*. A path heuristic (`corpus_pass.incidental_sub`: icon, screenshot, wallpaper, other web/doc image) splits the incidental class further as a reading aid; no decision uses it. No file bytes are in this directory; files are named `repo:path` and by SHA-256.
- **Synthetic set.** The real corpus has no multi-monitor screenshot, no file above 64 MiB (the 2026-11-16 fetcher capped files at 64 MiB and dropped none) and no stored image near the output cap. `gen_synthetic.py` (standard library only, deterministic) writes 25 project-authored files to `~/corpora/png-synthetic/` (3.6 GiB, outside the repository because the largest is 1 GiB): nine multi-monitor screenshot layouts, four cap-boundary images, seven long thin images, three stored (uncompressed-deflate) images near the output cap and two files at the input-cap boundary. Licence: project-authored, MIT; `MANIFEST.tsv` there records origin, licence, SHA-256 and the outcome intended before any oracle run. Synthetic files have their own column and never enter the real counts. Their pixels are screenshot-like but repeat every 256 rows, so they compress far better than real screenshots (3.9 MB for 66 Mpx); the stored files carry the worst-case input size instead. A scaled-down variant of every generated file decodes under the oracle to exactly the pixels the generator intended (checked before the full-size run).
- **Oracles.** `../spec-frozen/png_oracle.py` (asserted byte-identical to `../spec-draft/png_oracle.py`, which `policy_oracle.py` imports) and `../policy-draft/policy_oracle.py`, under `python3 -I`, with the caps of `../limits-draft/limits.py` (the 2026-11-17 values). No oracle, specification or limits file was edited.
- **What ran.** `corpus_pass.py oracle` runs Strict and RecommendedBeta on every file at its own size (the 2026-11-16 pass sampled 1 in 8 files between 1 Mpx and 16.7 Mpx and ran files over the old caps only in a raised-cap side run): `decode_still`, plus `decode_frames` for files with APNG chunks; obligation R in the still, stream and prefix forms; single-field reversion of each of RecommendedBeta's 15 non-Strict fields on every Strict-rejected file at every size (the old pass ablated only flips, and files above 1 Mpx only for the fields matching Strict's first error), with pairs for flips that no single field explains. Each file whose header exceeds a 2026-11-16 cap (16,384 per side, 64 MiB RGBA8 output, 256 MiB input) is run a second time with the old caps patched into both oracle modules, so the effect of the raise is measured rather than inferred. The still, stream and ablation functions are imported from the 2026-11-16 script (`../../leniency/corpora/png_recommendedbeta_pass.py`), not copied.
- **Final outcome of a field.** Field *f* *acts* on a file when reverting *f* alone to its Strict value changes RecommendedBeta's outcome (acceptance, first error, or output). Its final outcome on that file is RecommendedBeta's own outcome: *accept*, or the error that still rejects the file. *Needed* means the file is a flip (Strict rejects, RecommendedBeta accepts) that becomes rejected when *f* is reverted.
- **Allocation sizes.** For every file the reservation sizes of SPEC P20 come from IHDR through the oracle's own `raw_size`: Raw `R`, RGBA8 output `4N`, RGBA16 output `8N`, the APNG canvas and PREVIOUS buffer `4N`; they are compared with the guard's `MAX_BYTES` = 2^30 (and the old 2^26). `acTL num_frames`, the fcTL pixel sum and the inflated iCCP length (standard-library zlib, statistics only) are scanned for the other three caps.
- **Incumbents.** The 2026-11-16 runner binaries lived under `/tmp` and are gone; their cached results (`~/corpora/png/pass-cache/incumbents*.jsonl`: libpng 1.6.48, gdk-pixbuf 2.42.12, png 0.18.1 via image 0.25.10, glycin) are joined to the new oracle results. Neither the runners nor their inputs changed, so the cached results stand; the synthetic files have no incumbent results.
- **Compatibility profiles were not run** on any file: PNG-Q14 defers GdkCompat and ImageCrateCompat until Strict and RecommendedBeta are proved, and the hold-out must stay unseen by their fitting.
- **Reproduce.** `python3 -I gen_synthetic.py`; `python3 -I corpus_pass.py oracle --set synthetic`; `python3 -I corpus_pass.py oracle --set real --shard 0/1 --workers 3` (the machine has two cores; the first run of 2026-11-19 used `--shard K/2 --workers 7`, whose two cache files are kept and read); `python3 -I corpus_pass.py oldfix --set synthetic`; `python3 -I corpus_pass.py report`; `python3 -I reconcile_counts.py --json counts.json`; `python3 -I render_report.py`. Each oracle phase resumes from its cache under `~/corpora/png/pass-cache-2026-11-19/`.
- **Resuming after an interruption.** The sandbox can stop without warning, so the oracle phase writes one JSON line per finished file to `~/corpora/png/pass-cache-2026-11-19/oracle.<set>.shard<K>of<N>.jsonl` and flushes it at once (the cache is derived from third-party bytes, so it lives beside the corpus, not here). On start, `corpus_pass.py oracle` reads every `oracle.<set>*.jsonl` in that directory, whatever shard wrote it, skips each file whose SHA-256 already has a line, and appends to its own shard file; a line cut off by a crash fails to parse and is ignored, so that file simply runs again. A restart therefore loses only the files that were in progress (at most one per worker; the largest real file took about half an hour). To resume, re-run the same command, wrapped in `timeout`, e.g. `timeout 9h python3 -I corpus_pass.py oracle --set real --shard 0/1 --workers 3 --timeout 3600`; when it prints `0 files this call`, run `report`, then `render_report.py`. `report` refuses to run while any file lacks a record unless given `--partial`.

## Files

- `gen_synthetic.py`, `corpus_pass.py`, `reconcile_counts.py`, `render_report.py`: the scripts (this report's prose is in `render_report.py`).
- `results.json`: machine-readable results (a summary, then one record per file: outcomes, acting and needed fields, events, old-cap outcome, `R`).
- `counts.json`: the count reconciliation.
- `HOLDOUT.tsv`: the hold-out list (PNG-Q14).

## Tables
"""

G = ['incidental', 'test', 'fuzz', 'real', 'development', 'holdout', 'synthetic']


def mib(n):
    return f'{n / 2**20:,.1f} MiB' if n is not None else '-'


def main():
    res = json.loads(Path(os.environ.get('RESULTS', str(HERE / 'results.json'))).read_text())
    L = res['summary']
    C = json.loads((HERE / 'counts.json').read_text())
    M = []
    w = M.append
    w(BEGIN)
    w('')
    w(f"Generated by `render_report.py` from `results.json` ({L['date']}). Real corpus manifest SHA-256 `{L['manifest_sha256']}`; "
      f"synthetic manifest `{L['synthetic_manifest_sha256']}`; `png_oracle.py` (spec-frozen) `{L['png_oracle_sha256']}`; "
      f"`policy_oracle.py` `{L['policy_oracle_sha256']}`; `limits.py` `{L['limits_py_sha256']}`.")
    w('')
    w('### T1. Composition and coverage')
    w('')
    w('| | ' + ' | '.join(G) + ' |')
    w('|---|' + '---|' * len(G))
    for k, lab in (('files', 'files'),):
        w(f'| {lab} | ' + ' | '.join(str(L['composition']['files'][g]) for g in G) + ' |')
    for k in ('ran', 'S_still', 'RB_still', 'S_stream', 'RB_stream', 'flip_still', 'flip_stream',
              'rb_rejects_strict_accepts_still', 'rb_rejects_strict_accepts_stream'):
        w(f'| {k} | ' + ' | '.join(str(L['accept'][k][g]) for g in G) + ' |')
    w('')
    w('Incidental assets by kind (path heuristic, `corpus_pass.incidental_sub`): ' +
      ', '.join(f'{k} {v}' for k, v in sorted(L['composition']['incidental_sub'].items(), key=lambda t: -t[1])) +
      '; of them in the hold-out: ' + ', '.join(f'{k} {v}' for k, v in sorted(L['composition']['incidental_sub_holdout'].items(), key=lambda t: -t[1])) + '.')
    w('')
    w(f"Synthetic kinds: " + ', '.join(f'{k} {v}' for k, v in L['composition']['synthetic_kinds'].items()) +
      f". Coverage: {L['coverage'].get('real')} real, {L['coverage'].get('synthetic')} synthetic; failures: {L['coverage']['failures'] or 'none'}; "
      f"SHA-256 mismatches against the manifests: {len(L['composition']['sha256_mismatch'])}.")
    w('')
    w(f"Hold-out (`HOLDOUT.tsv`, SHA-256 `{L['holdout']['list_sha256']}`): {L['holdout']['files']} files, " +
      ', '.join(f'{k} {v}' for k, v in L['holdout']['by_class'].items()) + '; repositories: ' + ', '.join(f'`{r}`' for r in L['holdout']['repos']) + '.')
    w('')
    for key, title in (('errors_S_still', 'T2. Strict first error, still (decode_still), files per error'),
                       ('errors_S_stream', 'T3. Strict first error, whole stream (decode_frames to IEND); differs from T2 only on APNG files'),
                       ('errors_RB_still', 'T4. RecommendedBeta first error, still')):
        E = L[key]
        w(f'### {title}')
        w('')
        w('| error | ' + ' | '.join(G) + ' |')
        w('|---|' + '---|' * len(G))
        hit = [e for e in E if E[e]['real'] or E[e]['synthetic']]
        for e in sorted(hit, key=lambda e: -E[e]['real']):
            w(f'| {e} | ' + ' | '.join(str(E[e][g]) for g in G) + ' |')
        zero = [e for e in E if not (E[e]['real'] or E[e]['synthetic'])]
        w('')
        w(f'Errors no file hits ({len(zero)} of {len(E)}): ' + ', '.join(zero) + '.')
        w('')
    for kind in ('still', 'stream'):
        F = L['fields'][kind]
        fl = L['flips_' + kind]
        w(f'### T{5 if kind == "still" else 6}. RecommendedBeta fields, final outcome ({kind})')
        w('')
        w('"acts" = files on which reverting the field alone to Strict changes RecommendedBeta\'s outcome; "final accept" / "final reject" = RecommendedBeta\'s own outcome on those files; "needed" = flip files (Strict rejects, RecommendedBeta accepts) that become rejected when the field alone is reverted (or, marked, only with a second field). Cells are real files (incidental / test / fuzz, hold-out) and synthetic files.')
        w('')
        w('| field | acts: inc / test / fuzz (hold-out) | final accept (inc) | final reject: errors | needed: inc / test / fuzz (hold-out) | needed only as pair | revert gives | synthetic acts | example |')
        w('|---|---|---|---|---|---|---|---|---|')
        for f, v in F.items():
            a, n = v['acts'], v['needed_for_acceptance']
            ex = (v['examples_incidental'] or v['examples'] or [''])[0]
            w(f"| `{f}` | {a['incidental']} / {a['test']} / {a['fuzz']} ({a['holdout']}) | {v['final_accept']['real']} ({v['final_accept']['incidental']}) | "
              f"{v['final_reject']['real']}" + (': ' + ', '.join(f'{e} {c}' for e, c in v['final_reject_errors'].items()) if v['final_reject_errors'] else '') +
              f" | {n['incidental']} / {n['test']} / {n['fuzz']} ({n['holdout']}) | {v['needed_only_as_pair']} | " +
              ', '.join(f'{e} {c}' for e, c in v['revert_gives'].items()) + f" | {a['synthetic']} | `{ex}` |")
        w('')
        w(f"Flips ({kind}): " + ', '.join(f'{g} {fl["n"][g]}' for g in G) + f"; files by number of needed fields {fl['by_number_of_fields']}; unexplained {len(fl['unexplained'])}; pair-only {fl['pair_only']}.")
        w('')
    ob = L['obligation_R']
    w('### T7. Obligation R and recovered events')
    w('')
    w(f"Strict-accepted stills {ob['strict_accepted_stills']['real']} real + {ob['strict_accepted_stills']['synthetic']} synthetic; streams {ob['strict_accepted_streams']['real']} + {ob['strict_accepted_streams']['synthetic']}; Strict frames checked (prefix form) {ob['strict_frames_checked']}; **violations: {len(ob['violations'])}**" + (f": {ob['violations'][:10]}" if ob['violations'] else '') + '.')
    w('')
    w('| entry | event | ' + ' | '.join(G) + ' |')
    w('|---|---|' + '---|' * len(G))
    for kind, evs in L['recovered_events'].items():
        for e, v in evs.items():
            w(f'| {kind} | {e} | ' + ' | '.join(str(v[g]) for g in G) + ' |')
    w('')
    K = L['caps']
    w('### T8. Caps: every file whose header exceeds a 2026-11-16 cap (16,384 per side, 64 MiB RGBA8 output, 256 MiB input)')
    w('')
    w('"old" = the oracle re-run with the old caps patched in (Strict / RecommendedBeta); "now" = under the current caps. R = Raw (filtered scanlines) reservation.')
    w('')
    w('| file | set / class | dims | ct/depth/il | size | R | old Strict / RB | now Strict / RB | oracle s |')
    w('|---|---|---|---|---|---|---|---|---|')
    for d in K['over_old_caps']:
        old = d['old'] or {}
        w(f"| `{d['name']}` | {d['set']} / {d['cls']}{' (hold-out)' if d['holdout'] else ''} | {d['w']} x {d['h']} | {d['ct']}/{d['depth']}/{d['interlace']} | {mib(d['size'])} | {mib(d['R'])} | "
          f"{old.get('strict', '-')} / {old.get('rb', '-')} | {d['strict']} / {d['rb']} | {d['seconds']} |")
    w('')
    w(f"Summary, Strict old -> now: real {K['over_old_caps_summary']['real']}; synthetic {K['over_old_caps_summary']['synthetic']}. RecommendedBeta old -> now: real {K['over_old_caps_rb_summary']['real']}; synthetic {K['over_old_caps_rb_summary']['synthetic']}.")
    w('')
    w('Cap errors under the current caps (Strict, still): ' + '; '.join(f"{k} {v['real']} real / {v['synthetic']} synthetic" for k, v in K['cap_rejects_now'].items()) + '.')
    w('')
    for f in K['cap_rejects_now_files']:
        w(f'- `{f[0]}` ({f[1]}): {f[2]} x {f[3]}, {f[4]}')
    w('')
    w('### T9. Synthetic files (project-authored, `gen_synthetic.py`)')
    w('')
    w('| file | kind | dims | ct/depth/il | encoding | size | R | out16 | expected | Strict / RB | old caps (measured) | oracle s |')
    w('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for d in sorted(K['synthetic'], key=lambda d: d['name']):
        old = d['old'] or {}
        w(f"| `{d['name'].split(':', 1)[1]}` | {d['kind']} | {d['w']} x {d['h']} | {d['ct']}/{d['depth']}/{d['interlace']} | {d['encoding']} | {mib(d['size'])} | {mib(d['R'])} | {mib(d['out16'])} | "
          f"{d['expect']} | {d['strict']} / {d['rb']} | {old.get('strict', d['old_caps_by_intent'] + ' (intent)')} | {d['seconds']} |")
    w('')
    w('### T10. Largest quantities among RecommendedBeta-accepted files, and reservations against MAX_BYTES')
    w('')
    w('| quantity | real (all classes) | real incidental |')
    w('|---|---|---|')
    A, I = K['accepted_real_max'], K['incidental_accepted_max']
    for k in ('area_px', 'side', 'size_bytes', 'R', 'out16_bytes_depth16', 'num_frames', 'fctl_pixel_sum', 'iccp_inflated', 'aspect'):
        a = A.get(k)
        i = I.get(k)
        fmt = lambda v: '-' if not v or v[0] is None else (f"{v[0]:,.1f}" if isinstance(v[0], float) else f"{v[0]:,}") + f" (`{v[1]}`)"
        w(f'| {k} | {fmt(a)} | {fmt(i)} |')
    w('')
    R = K['reservations']
    w('Largest single reservation among accepted files: ' + '; '.join(
        f"{s}: " + ', '.join(f"{k} {mib(v[0])} (`{v[1]}`)" for k, v in R[s].items() if v and v[0]) for s in R) + '.')
    w('')
    w(f"Accepted files with a reservation above the old guard (64 MiB): {K['reservations_over_old_max_bytes']}; above MAX_BYTES (1 GiB): {K['reservations_over_max_bytes']}. "
      f"Largest R + RGBA16 output (decode_still16 peak): {K['peak_live_heap_still16_max']}.")
    w('')
    w('Largest real files by declared area (all outcomes):')
    w('')
    w('| area | dims | file | class | Strict | RB |')
    w('|---|---|---|---|---|---|')
    for a, ww, hh, n, c, s, r in K['largest_real_area'][:15]:
        w(f'| {a / 1e6:,.1f} Mpx | {ww} x {hh} | `{n}` | {c} | {s} | {r} |')
    w('')
    t = L['thin_real']
    w(f"Thin real files (aspect at least 32:1, long side at least 1,024): {t['n']['real']} (incidental {t['n']['incidental']}); Strict {t['strict']}; RecommendedBeta {t['rb']}.")
    w('')
    w('### T11. Against the 2026-11-16 pass (same manifest; that pass ran the old oracles at the old caps, 20,502 files)')
    w('')
    v = L['vs_2026_11_16']
    w(f"Files run in both passes with the same Strict and RecommendedBeta outcome (and the same Strict RGBA): {v['counts'].get('same', 0)}; changed: {v['counts'].get('changed', 0)}; not run on 2026-11-16: {v['counts'].get('not run on 2026-11-16', 0)}.")
    w('')
    for c in v['changed'][:60]:
        w(f'- `{c[0]}` ({c[1]}): Strict/RB {c[2][0]} / {c[2][1]} then, {c[3][0]} / {c[3][1]} now')
    w('')
    w('### T12. Incumbents (cached 2026-11-16 runner results, real files only), against RecommendedBeta still')
    w('')
    w('| runner | both accept | RB only | incumbent only (incidental) | incumbent only, RB error | first frame equal | first frame differs, by depth |')
    w('|---|---|---|---|---|---|---|')
    for k, d in L['incumbents'].items():
        w(f"| {k} | {d['both_accept']} | {d['rb_only']} | {d['incumbent_only']} ({d['incumbent_only_incidental']}) | " + ', '.join(f'{e} {c}' for e, c in d['incumbent_only_by_rb_error'].items()) +
          f" | {d['first_frame_equal']} | {d['first_frame_differ_by_depth']} |")
    w('')
    w('### T13. Input-count reconciliation (`reconcile_counts.py`)')
    w('')
    w('| count | recomputed | definition |')
    w('|---|---|---|')
    w(f"| 748 | {C['results_json_records']} | records in `policy-draft/probes/results.json`: {C['results_json_by_kind']['vec']} vector, {C['results_json_by_kind']['probe']} probe, {C['results_json_by_kind']['wit']} witness; {C['results_json_by_reference_over_4KiB']} above 4 KiB stored by reference, {C['results_json_inline']} inline |")
    for s in reversed(C['bridge_steps']):
        w(f"| {s['claimed']} | {s['recomputed']} | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), {s['what']} |")
    w('')
    w(f"Parts: {C['inline_distinct']} distinct inline records; {C['distinct_nonpadded_vectors_before_2026_11_16']} distinct non-padded vectors of today's core and caps files without the 17 later vectors, {C['overlap_inline_and_those_vectors']} of them also inline (734 + 533 - 530 = 737); the 198 profile vectors add {C['policy_vectors_distinct_new_inputs']} distinct inputs; the 14 review/council vectors add {C['review_vectors_distinct_new_inputs']}; {C['vector_records_padded']} vector records are padded (`pad_to`) and never bridged. Vector records now: {C['records_now']} (" + ', '.join(f'{k} {v}' for k, v in C['vector_records'].items()) + ').')
    w('')
    w(f"Timing: {L['timing']['oracle_cpu_seconds']:,} s of oracle wall time summed over workers; slowest: " + '; '.join(f'{s:,.0f} s `{n}`' for s, n in L['timing']['slowest'][:5]) + '.')
    w('')
    w(END)
    p = Path(os.environ.get('REPORT', str(HERE / 'REPORT.md')))
    p.write_text('# M0-corpus: the real-corpus pass under the 2026-11-17 caps (PNG-Q7)\n\n' + CONCLUSIONS.strip() + '\n\n'
                 + METHOD.strip() + '\n\n' + '\n'.join(M) + '\n')
    print('wrote', p)


if __name__ == '__main__':
    sys.exit(main())
