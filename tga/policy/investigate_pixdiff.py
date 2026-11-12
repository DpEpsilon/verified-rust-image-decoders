#!/usr/bin/env python3
"""Explain corpus pixel differences by matching native RGBA hashes to oracle rules.

Reads the local corpus ($CORPUS_DIR/tga (default ~/corpora/tga): files/, results.jsonl,
pixdiff.tsv) and never copies its bytes.  For every (file, decoder) pair that
pixdiff.tsv lists, it decodes the file under the decoder's candidate rule
record and checks the SHA-256 of the RGBA output against the native hash
recorded in results.jsonl.  It then reverts each non-Strict option of the
record one at a time: an option is *responsible* when reverting it alone
breaks the exact match.  Output: /tmp/claude-0/tga-policy/pixdiff-explained.json
and a summary on stdout.

tinytga is modelled as a rule record plus two post-transforms that the harness
imposes or that tinytga's `pixels()` iterator has: alpha forced to 255 (the
runner converts to Rgb888) and descriptor bit 4 ignored (no horizontal flip).
"""
import collections
import hashlib
import json
import os
from pathlib import Path
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import policy_oracle as O

CORPUS = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser() / 'tga'
OUT = Path('/tmp/claude-0/tga-policy/pixdiff-explained.json')
TINY = dict(O.STRICT, trailing='Ignore', rle_overrun='ClipSamples', attribute_bits='Lenient',
            map_header='IgnoreAbsent', rgb5='Round', map5='Round', map_index='ZeroBased')
RECORDS = {'image': O.IMAGE, 'glycin': O.IMAGE, 'gdk-forced-whole': O.GDK, 'gdk-file': O.GDK,
           'tiny': TINY}


def tiny_post(b, w, h, rgba, unflip=True, opaque=True):
    px = bytearray(rgba)
    if opaque:
        px[3::4] = b'\xff' * (w * h)
    if unflip and b[17] & 0x10:
        rows = [px[4 * w * y:4 * w * (y + 1)] for y in range(h)]
        px = bytearray(b''.join(b''.join(r[4 * x:4 * x + 4] for x in range(w - 1, -1, -1)) for r in rows))
    return bytes(px)


def sha(b, record, decoder, unflip=True, opaque=True):
    try:
        w, h, rgba, _ = O._decode(b, record)
    except O.Invalid as e:
        return ('error', str(e))
    if decoder == 'tiny':
        rgba = tiny_post(b, w, h, rgba, unflip, opaque)
    return ('ok', w, h, hashlib.sha256(rgba).hexdigest())


def native(r):
    return ('ok', r['width'], r['height'], r['rgba_sha256']) if r['status'] == 'ok' else ('error',)


def main():
    results = {}
    for line in (CORPUS / 'results.jsonl').open():
        row = json.loads(line)
        results[row['file']] = row['res']
    pairs = [l.rstrip('\n').split('\t') for l in (CORPUS / 'pixdiff.tsv').open()]
    explained = []
    tally = collections.Counter()
    for f, repo, decoder, cls in pairs:
        b = (CORPUS / 'files' / f).read_bytes()
        target = native(results[f][decoder])
        record = RECORDS[decoder]
        full = sha(b, record, decoder) == target
        responsible = []
        if full:
            for k, v in record.items():
                if v != O.STRICT[k] and sha(b, dict(record, **{k: O.STRICT[k]}), decoder) != target:
                    responsible.append(f'{k}={v}')
            if decoder == 'tiny':
                if sha(b, record, decoder, unflip=False) != target:
                    responsible.append('tiny:ignores-descriptor-bit4')
                if sha(b, record, decoder, opaque=False) != target:
                    responsible.append('harness:Rgb888-alpha-255')
        singles = [f'{k}={v}' for k, vals in O.OPTIONS.items() for v in vals[1:]
                   if decoder != 'tiny' and sha(b, dict(O.STRICT, **{k: v}), decoder) == target]
        h = f'type={b[2]} depth={b[16]} map={b[1]}/{b[7]} first={O.le16(b, 3)} attr={b[17] & 15} right={b[17] >> 4 & 1}'
        explained.append({'file': f, 'decoder': decoder, 'harness_class': cls, 'header': h,
                          'record_matches': full, 'responsible': responsible,
                          'single_option_matches': singles})
        tally[(decoder, cls, full, ' + '.join(responsible))] += 1
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(explained, indent=1) + '\n')
    for (decoder, cls, full, resp), c in sorted(tally.items()):
        print(f'{decoder:17} {cls:16} {c:3} exact={full} responsible: {resp or "-"}')
    print(OUT)


if __name__ == '__main__':
    main()
