#!/usr/bin/env python3
"""Measure the PNG corpus with a header-level scanner (standard library only).

  python3 -I png_stats.py [--limit N] [--out-md png-STATS.md] [--out-json png-stats.json]

Reads $CORPUS_DIR/png/MANIFEST.tsv (default ~/corpora), files/*.png, FETCH.log and
DUPLICATES.tsv; writes png-STATS.md and png-stats.json beside this script.  Output holds file
names (repo paths) and numbers only, never file bytes.  The prose between the generated tables
comes from png-STATS-interpretation.md.

Scan (no pixel decoding, no oracle; the oracle and incumbent runs are Phase 2):
  * chunk walk that does not stop at the first anomaly: chunk kinds, CRC-32 of every chunk,
    IHDR fields, IEND present, bytes after IEND, truncated final chunk;
  * IDAT stream: all IDAT payloads are concatenated and inflated with zlib.decompressobj in
    1 MiB steps without keeping the output; the scan records whether the stream ended, bytes
    after its end (surplus inside the IDAT payload), a zlib error class (header, Adler-32,
    block, distance ...), and the inflated size against the size the header implies, R(w,h);
  * APNG: acTL, fcTL, fdAT counts, sequence numbers, frame rectangles, default image in or
    outside the animation;
  * dimension thresholds from PNG-Q5: side above 16,384; area above 16,777,216 pixels
    (4*W*H above 64 MiB, RGBA8) and above 33,554,432 pixels (8*W*H above 256 MiB, RGBA16:
    128 MiB is the RGBA16 size at 16.7 Mpx); long thin images by aspect ratio.

Class (every file has exactly one): fuzz, test, incidental.
  fuzz        repo kind 'fuzz', or a path token afl, fuzz, crash, clusterfuzz, ossfuzz, mutant, poc, oss-fuzz
  incidental  repo kind 'assets' or 'mixed', no fuzz token, and no test token in the path
  test        everything else (repo kind 'tests', or a test token in the path)
Tokens match at the start of a path word (preceded by a non-letter), so 'debugging' is not 'bug' but 'bugfix' is.
This differs from gif_stats.py, which matched substrings.  The class is a heuristic on paths.
"""
import argparse
import json
import os
import re
import struct
import sys
import zlib
from collections import Counter, defaultdict
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
SIG = b'\x89PNG\r\n\x1a\n'
MiB = 1 << 20
SIDE_CAP = 16384
AREA_RGBA8 = 1 << 24          # 4*W*H = 64 MiB
AREA_RGBA16 = 1 << 25         # 33,554,432 px: RGBA8 128 MiB, RGBA16 256 MiB
VALID_DEPTHS = {0: (1, 2, 4, 8, 16), 2: (8, 16), 3: (1, 2, 4, 8), 4: (8, 16), 6: (8, 16)}
CHANNELS = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}
KNOWN_CRITICAL = {b'IHDR', b'PLTE', b'IDAT', b'IEND'}
KNOWN_ANC = {b'acTL', b'fcTL', b'fdAT', b'bKGD', b'cHRM', b'cICP', b'dSIG', b'eXIf', b'gAMA', b'hIST', b'iCCP', b'iTXt',
             b'mDCV', b'cLLI', b'pHYs', b'sBIT', b'sPLT', b'sRGB', b'sTER', b'tEXt', b'tIME', b'tRNS', b'zTXt'}
SINGLE = {b'IHDR', b'PLTE', b'IEND', b'acTL', b'bKGD', b'cHRM', b'cICP', b'eXIf', b'gAMA', b'hIST', b'iCCP', b'mDCV',
          b'cLLI', b'pHYs', b'sBIT', b'sRGB', b'tIME', b'tRNS'}
FUZZ_RE = re.compile(r'(?<![a-z])(afl|fuzz|crash|clusterfuzz|ossfuzz|mutant|poc)')
TEST_RE = re.compile(r'(?<![a-z])(test|fuzz|afl|crash|invalid|bad|broken|fail|repro|corrupt|trunc|error|conformance|'
                     r'regression|artificial|suite|resources/|reftest|crashtest|samples|bug|expected|reference|'
                     r'baseline|golden|pngsuite|bugfix|testdata|fixtures?)')


def classify(repo_kind, path):
    p = path.lower()
    if repo_kind == 'fuzz' or FUZZ_RE.search(p):
        return 'fuzz'
    if repo_kind in ('assets', 'mixed') and not TEST_RE.search(p):
        return 'incidental'
    return 'test'


def raw_size(w, h, ct, depth, il):
    bpp = CHANNELS[ct] * depth
    if not il:
        return h * (1 + (w * bpp + 7) // 8)
    tot = 0
    for x0, y0, dx, dy in ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)):
        pw, ph = (w - x0 + dx - 1) // dx, (h - y0 + dy - 1) // dy
        if pw > 0 and ph > 0:
            tot += ph * (1 + (pw * bpp + 7) // 8)
    return tot


def inflate_stats(parts, expected):
    """Inflate the concatenated IDAT payloads; returns dict (no output kept)."""
    r = {'zlib': 'ok', 'inflated': 0, 'ended': False, 'surplus': 0}
    if not parts:
        r['zlib'] = 'no_idat'
        return r
    d = zlib.decompressobj()
    limit = max(2 * expected, expected + MiB) if expected is not None else 1 << 30
    pieces = [c[i:i + MiB] for c in parts for i in range(0, len(c), MiB)]
    try:
        for idx, piece in enumerate(pieces):
            data = piece
            while data and not d.eof:
                out = d.decompress(data, MiB)
                r['inflated'] += len(out)
                data = d.unconsumed_tail
                if r['inflated'] > limit:
                    r['zlib'] = 'inflate_exceeds_2x_expected'
                    return r
            if d.eof:
                r['ended'] = True
                r['surplus'] = len(d.unused_data) + sum(len(p) for p in pieces[idx + 1:])
                break
    except zlib.error as e:
        m = re.search(r'Error -?\d+ while decompressing data: (.*)', str(e))
        r['zlib'] = (m.group(1) if m else str(e))[:60]
    return r


def scan(b):
    """Header-level scan; never raises."""
    r = {'size': len(b), 'sig': b.startswith(SIG)}
    flags = set()
    kinds = Counter()
    r['flags'] = flags
    r['kinds'] = kinds
    if not r['sig']:
        flags.add('no_signature')
        return r
    p = 8
    n = len(b)
    idat = []
    ihdr = None
    order = []
    seen_idat = False
    iend = False
    first = True
    actl = None
    fctl = []
    fdat_seqs = []
    fdat_n = 0
    seqs = []
    crc_bad_crit = crc_bad_anc = 0
    crc_first = None
    while p < n:
        if p + 8 > n:
            flags.add('truncated_chunk_header')
            break
        ln = struct.unpack('>I', b[p:p + 4])[0]
        typ = b[p + 4:p + 8]
        if ln > 0x7FFFFFFF:
            flags.add('chunk_length_over_2^31')
        end = p + 8 + ln
        if end + 4 > n:
            flags.add('truncated_chunk')
            r['truncated_type'] = typ.decode('latin-1')
            if seen_idat and typ != b'IDAT':
                flags.add('truncated_inside_trailing_chunk')
            data = b[p + 8:n]
            kinds[typ.decode('latin-1')] += 1
            if typ == b'IDAT':
                idat.append(data)
                seen_idat = True
            elif typ == b'IHDR' and first:
                flags.add('ihdr_truncated')
            p = n
            break
        data = b[p + 8:end]
        crc = struct.unpack('>I', b[end:end + 4])[0]
        name = typ.decode('latin-1')
        kinds[name] += 1
        if zlib.crc32(typ + data) & 0xFFFFFFFF != crc:
            if typ[0:1].isupper():
                crc_bad_crit += 1
            else:
                crc_bad_anc += 1
            crc_first = crc_first or name
        if first:
            if typ != b'IHDR':
                flags.add('first_chunk_not_ihdr')
            first = False
        if not all(65 <= c <= 90 or 97 <= c <= 122 for c in typ):
            flags.add('invalid_chunk_type_bytes')
        if typ[2] >= 97:
            flags.add('reserved_bit_set')
        if typ not in KNOWN_CRITICAL and typ not in KNOWN_ANC:
            if typ[0] < 97:
                flags.add('unknown_critical_chunk')
            else:
                flags.add('unknown_ancillary_chunk')
        if typ == b'IHDR':
            if ihdr is None and ln == 13:
                w, h, depth, ct, comp, filt, il = struct.unpack('>IIBBBBB', data)
                ihdr = (w, h, depth, ct, comp, filt, il)
            elif ln != 13:
                flags.add('ihdr_bad_length')
        elif typ == b'IDAT':
            if seen_idat and order and order[-1] != 'IDAT':
                flags.add('idat_not_consecutive')
            idat.append(data)
            seen_idat = True
        elif typ == b'IEND':
            iend = True
            if ln:
                flags.add('iend_nonzero_length')
        elif typ == b'acTL' and ln == 8:
            actl = struct.unpack('>II', data)
            if seen_idat:
                flags.add('actl_after_idat')
        elif typ == b'fcTL' and ln == 26:
            v = struct.unpack('>IIIIIHHBB', data)
            fctl.append((seen_idat, v))
            seqs.append(v[0])
        elif typ == b'fdAT' and ln >= 4:
            fdat_n += 1
            seqs.append(struct.unpack('>I', data[:4])[0])
        elif typ == b'PLTE':
            r['plte_len'] = ln
            if seen_idat:
                flags.add('plte_after_idat')
        elif typ == b'tRNS':
            r['trns_len'] = ln
            if seen_idat:
                flags.add('trns_after_idat')
        elif typ in (b'gAMA', b'cHRM', b'sRGB', b'iCCP', b'sBIT', b'cICP', b'mDCV', b'cLLI') and seen_idat:
            flags.add('colour_chunk_after_idat')
        elif typ in (b'tEXt', b'zTXt', b'iTXt', b'tIME', b'eXIf', b'pHYs') and seen_idat:
            flags.add('other_ancillary_after_idat')
        if typ == b'iCCP' and ln >= 2:
            nm_end = data.find(b'\0')
            name_bytes = data[:nm_end] if nm_end >= 0 else data
            if nm_end < 0 or nm_end > 79 or nm_end == 0:
                flags.add('iccp_bad_name_length')
            if any(c < 32 or 126 < c < 161 for c in name_bytes):
                flags.add('iccp_name_bad_chars')
            if nm_end >= 0 and nm_end + 1 < ln and data[nm_end + 1] != 0:
                flags.add('iccp_bad_method')
        order.append(name if typ == b'IDAT' else 'x')
        p = end + 4
        if iend:
            break
    r['iend'] = iend
    if not iend:
        flags.add('missing_iend')
    tail = n - p if iend else 0
    r['trailing'] = tail
    if tail:
        flags.add('trailing_bytes_after_iend')
        r['trailing_nonzero'] = any(b[p:p + 4096])
    if crc_bad_crit or crc_bad_anc:
        flags.add('bad_crc')
        if crc_bad_crit:
            flags.add('bad_crc_critical')
        if crc_bad_anc:
            flags.add('bad_crc_ancillary_only' if not crc_bad_crit else 'bad_crc_anc_too')
    r['crc_first'] = crc_first
    r['dup_single'] = sorted(k for k in SINGLE if kinds.get(k.decode(), 0) > 1)
    if r['dup_single']:
        flags.add('duplicate_single_chunk')
    if 'iCCP' in kinds and 'sRGB' in kinds:
        flags.add('iccp_and_srgb')
    if ihdr is None:
        flags.add('no_ihdr')
        return r
    w, h, depth, ct, comp, filt, il = ihdr
    r.update(width=w, height=h, depth=depth, ct=ct, interlace=il)
    if ct not in VALID_DEPTHS or depth not in VALID_DEPTHS[ct]:
        flags.add('invalid_ct_depth')
    if w == 0 or h == 0:
        flags.add('zero_dimension')
    if w > 0x7FFFFFFF or h > 0x7FFFFFFF:
        flags.add('dimension_over_2^31')
    if comp != 0 or filt != 0 or il not in (0, 1):
        flags.add('bad_method_bytes')
    if ct == 3 and 'PLTE' not in kinds:
        flags.add('indexed_without_plte')
    if ct == 3 and 'PLTE' in kinds and r.get('plte_len', 0) // 3 > (1 << depth if depth <= 8 else 256):
        flags.add('plte_larger_than_depth')
    if 'PLTE' in kinds and r.get('plte_len', 0) % 3:
        flags.add('plte_length_not_multiple_of_3')
    if ct in (4, 6) and 'tRNS' in kinds:
        flags.add('trns_with_alpha_type')
    if ct == 3 and 'tRNS' in kinds and r.get('trns_len', 0) > r.get('plte_len', 0) // 3:
        flags.add('trns_longer_than_palette')
    valid = ct in VALID_DEPTHS and depth in VALID_DEPTHS[ct] and w and h and il in (0, 1)
    expected = raw_size(w, h, ct, depth, il) if valid else None
    r['expected_raw'] = expected
    if 'IDAT' not in kinds:
        flags.add('no_idat')
    z = inflate_stats(idat, expected)
    r['zlib'] = z['zlib']
    r['inflated'] = z['inflated']
    if z['zlib'] != 'ok':
        flags.add('zlib_error')
    elif not z['ended']:
        flags.add('zlib_stream_unterminated')
    if z['surplus']:
        flags.add('surplus_after_zlib_stream')
        r['surplus'] = z['surplus']
    if expected is not None and z['zlib'] == 'ok' and z['ended']:
        if z['inflated'] < expected:
            flags.add('inflated_too_short')
        elif z['inflated'] > expected:
            flags.add('inflated_too_long')
    if expected is not None and z['zlib'] == 'ok' and not z['ended'] and z['inflated'] < expected:
        flags.add('inflated_too_short')
    r['idat_chunks'] = len(idat)
    if any(len(c) == 0 for c in idat):
        flags.add('empty_idat_chunk')
    # APNG
    if actl is not None or fctl or fdat_n:
        r['apng'] = True
        a = {'num_frames': actl[0] if actl else None, 'num_plays': actl[1] if actl else None,
             'fctl': len(fctl), 'fdat': fdat_n}
        if actl is None:
            flags.add('apng_chunks_without_actl')
        default_in_anim = bool(fctl) and not fctl[0][0]
        a['default_in_animation'] = default_in_anim
        if actl and actl[0] != len(fctl):
            flags.add('apng_frame_count_mismatch')
        if seqs != list(range(len(seqs))):
            flags.add('apng_sequence_not_contiguous')
        outside = 0
        disp = Counter()
        blend = Counter()
        for _, v in fctl:
            _, fw, fh, fx, fy, dn, dd, dop, bop = v
            if fw == 0 or fh == 0 or fx + fw > w or fy + fh > h:
                outside += 1
            disp[dop] += 1
            blend[bop] += 1
        if outside:
            flags.add('apng_frame_outside_canvas')
        if any(d > 2 for d in disp):
            flags.add('apng_bad_dispose_op')
        if any(x > 1 for x in blend):
            flags.add('apng_bad_blend_op')
        if fctl and fdat_n == 0 and len(fctl) > 1:
            flags.add('apng_no_fdat')
        a['dispose'] = dict(disp)
        a['blend'] = dict(blend)
        r['apng_info'] = a
    return r


def pct(xs, q):
    if not xs:
        return 0
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))]


def read_tsv(path):
    return [l.split('\t') for l in path.read_text().splitlines() if l and not l.startswith('#')]


def load_holdout():
    p = HERE / 'holdout-png.tsv'
    return {r[0] for r in read_tsv(p)} if p.exists() else set()


def fmt_size(n):
    return f'{n / MiB:.1f} MiB' if n >= MiB else f'{n / 1024:.1f} KiB'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--limit', type=int, default=0)
    ap.add_argument('--out-md', default=str(HERE / 'png-STATS.md'))
    ap.add_argument('--out-json', default=str(HERE / 'png-stats.json'))
    a = ap.parse_args()
    root = Path(os.environ.get('CORPUS_DIR', str(Path.home() / 'corpora'))).expanduser().resolve() / 'png'
    rows = [r for r in read_tsv(root / 'MANIFEST.tsv') if r[0] != 'path']
    if a.limit:
        rows = rows[:a.limit]
    holdout = load_holdout()
    files = []
    for i, r in enumerate(rows):
        b = (root / r[0]).read_bytes()
        s = scan(b)
        kind = r[10] if len(r) > 10 else 'mixed'
        rec = {'sha256': r[1], 'repo': r[4], 'repo_path': r[6], 'size': int(r[7]), 'kind': kind,
               'cls': classify(kind, r[6]), 'holdout': r[4] in holdout, 'scan': s}
        files.append(rec)
        if (i + 1) % 1000 == 0:
            print(i + 1, 'of', len(rows), flush=True)
    log = (root / 'FETCH.log').read_text().splitlines() if (root / 'FETCH.log').exists() else []
    dups = read_tsv(root / 'DUPLICATES.tsv')[1:] if (root / 'DUPLICATES.tsv').exists() else []
    write(files, log, dups, a)


def write(files, log, dups, a):
    n = len(files)
    total = sum(f['size'] for f in files)
    classes = ['incidental', 'test', 'fuzz']
    byc = {c: [f for f in files if f['cls'] == c] for c in classes}
    sig = [f for f in files if f['scan']['sig']]
    hdr = [f for f in sig if 'width' in f['scan']]
    dev = [f for f in files if not f['holdout']]
    hold = [f for f in files if f['holdout']]
    W = lambda f: f['scan']['width']
    H = lambda f: f['scan']['height']
    area = lambda f: W(f) * H(f)
    S = lambda f: f['scan']['flags']

    # fetch log parse
    per_repo = {}
    tree_failed, lfs_names, toolarge, failed_files = [], [], [], []
    for line in log:
        p = line.split('\t')
        if len(p) < 2:
            continue
        if p[1] == 'OK':
            d = dict(x.split('=', 1) for x in p[2:] if '=' in x)
            per_repo[p[0]] = d
        elif p[1] == 'TREE-FAILED':
            tree_failed.append((p[0], p[2] if len(p) > 2 else ''))
        elif p[1] == 'LFS-POINTER':
            lfs_names.append(p[0])
        elif p[1] == 'TOO-LARGE':
            toolarge.append((p[0], p[2], p[3] if len(p) > 3 else ''))
        elif p[1] in ('FILE-FAILED', 'BLOB-MISMATCH'):
            failed_files.append((p[0], p[2]))
    cand_total = sum(int(d['candidates']) for d in per_repo.values())
    dup_total = len(dups)
    dup_by_repo = Counter(d[0] for d in dups)
    stored_total = sum(int(d['stored']) for d in per_repo.values())
    reuse_total = sum(int(d.get('reused', 0)) for d in per_repo.values())
    lfs_by_repo = Counter(lfs_names)
    big_ct = Counter(t[0] for t in toolarge)

    def accounting(d):
        return int(d['stored']) + int(d['duplicates'])

    # size distribution
    bins = [('< 1 KiB', 0, 1 << 10), ('1-16 KiB', 1 << 10, 16 << 10), ('16-256 KiB', 16 << 10, 256 << 10),
            ('256 KiB-1 MiB', 256 << 10, 1 * MiB), ('1-5 MiB', 1 * MiB, 5 * MiB), ('5-16 MiB', 5 * MiB, 16 * MiB),
            ('16-64 MiB', 16 * MiB, 64 * MiB + 1)]
    size_hist = {c: {k: sum(1 for f in fs if lo <= f['size'] < hi) for k, lo, hi in bins}
                 for c, fs in list(byc.items()) + [('all', files)]}
    over5 = sorted([f for f in files if f['size'] > 5 * MiB], key=lambda f: -f['size'])
    sizes = [f['size'] for f in files]

    # dimension thresholds
    def thr(fs):
        d = [f for f in fs if 'width' in f['scan']]
        t = {
            'with_ihdr': len(d),
            'side > 16,384': sum(1 for f in d if max(W(f), H(f)) > SIDE_CAP),
            'area > 16.7 Mpx (RGBA8 > 64 MiB, RGBA16 > 128 MiB)': sum(1 for f in d if area(f) > AREA_RGBA8),
            'area > 33.5 Mpx (RGBA8 > 128 MiB, RGBA16 > 256 MiB)': sum(1 for f in d if area(f) > AREA_RGBA16),
            'area > 4 Mpx': sum(1 for f in d if area(f) > 4_000_000),
            'area > 8.3 Mpx (3840x2160)': sum(1 for f in d if area(f) > 3840 * 2160),
            'aspect_ge_8': sum(1 for f in d if min(W(f), H(f)) and max(W(f), H(f)) / min(W(f), H(f)) >= 8),
            'aspect_ge_32': sum(1 for f in d if min(W(f), H(f)) and max(W(f), H(f)) / min(W(f), H(f)) >= 32),
            'aspect_ge_100': sum(1 for f in d if min(W(f), H(f)) and max(W(f), H(f)) / min(W(f), H(f)) >= 100),
            'one_pixel_wide_or_high': sum(1 for f in d if min(W(f), H(f)) == 1),
            'long_thin(aspect>=32 and long side>=1024)': sum(1 for f in d if min(W(f), H(f)) and max(W(f), H(f)) / min(W(f), H(f)) >= 32 and max(W(f), H(f)) >= 1024),
            'multi-monitor-like(width>=5120, 2.5<=w/h<=8)': sum(1 for f in d if H(f) and W(f) >= 5120 and 2.5 <= W(f) / H(f) <= 8),
            'width_ge_3840_and_height_ge_2160': sum(1 for f in d if W(f) >= 3840 and H(f) >= 2160),
        }
        return t
    thr_all = {c: thr(fs) for c, fs in list(byc.items()) + [('all', files), ('non-fuzz', [f for f in files if f['cls'] != 'fuzz'])]}
    big_files = sorted([f for f in hdr if max(W(f), H(f)) > SIDE_CAP or area(f) > AREA_RGBA8], key=lambda f: -area(f))
    thin_files = sorted([f for f in hdr if min(W(f), H(f)) and max(W(f), H(f)) / min(W(f), H(f)) >= 32 and max(W(f), H(f)) >= 1024],
                        key=lambda f: -(max(W(f), H(f)) / min(W(f), H(f))))
    wide_screens = sorted([f for f in hdr if H(f) and W(f) >= 3840 and H(f) >= 1080 and W(f) / H(f) >= 2.2],
                          key=lambda f: -area(f))
    top_area = sorted(hdr, key=lambda f: -area(f))[:10]
    # dimension histogram by log2 of max side
    def side_bucket(f):
        m = max(W(f), H(f))
        for k, hi in (('<=64', 64), ('65-256', 256), ('257-1024', 1024), ('1025-2048', 2048), ('2049-4096', 4096),
                      ('4097-8192', 8192), ('8193-16384', 16384)):
            if m <= hi:
                return k
        return '>16384'
    side_order = ['<=64', '65-256', '257-1024', '1025-2048', '2049-4096', '4097-8192', '8193-16384', '>16384']
    side_hist = {c: Counter(side_bucket(f) for f in fs if 'width' in f['scan']) for c, fs in list(byc.items()) + [('all', files)]}
    # memory-relevant: filtered raw size and RGBA sizes
    raw_big = [f for f in hdr if f['scan'].get('expected_raw') and f['scan']['expected_raw'] > 64 * MiB]
    # colour types / depths
    ctd = {c: Counter((f['scan']['ct'], f['scan']['depth']) for f in fs if 'ct' in f['scan']) for c, fs in list(byc.items()) + [('all', files)]}
    ctd_all = ctd['all']
    n16 = {c: sum(v for (ct, d), v in cc.items() if d == 16) for c, cc in ctd.items()}
    nct = {c: sum(cc.values()) for c, cc in ctd.items()}
    il = {c: sum(1 for f in fs if f['scan'].get('interlace') == 1) for c, fs in list(byc.items()) + [('all', files)]}
    ilbig = [f for f in hdr if f['scan'].get('interlace') == 1 and area(f) > AREA_RGBA8]
    # APNG
    apng = [f for f in files if f['scan'].get('apng')]
    apng_ok = [f for f in apng if f['scan'].get('apng_info', {}).get('num_frames') is not None]
    frames_hist = Counter()
    for f in apng_ok:
        k = f['scan']['apng_info']['num_frames']
        frames_hist['1' if k == 1 else '2-9' if k < 10 else '10-99' if k < 100 else '100-999' if k < 1000 else '1000+'] += 1
    apng_kinds = Counter()
    for f in apng:
        ai = f['scan'].get('apng_info', {})
        apng_kinds['default image in animation' if ai.get('default_in_animation') else 'default image outside animation'] += 1
        if ai.get('num_plays') == 0:
            apng_kinds['num_plays = 0 (loops forever)'] += 1
        for k in ai.get('dispose', {}):
            apng_kinds[f'dispose_op {k}'] += 1
        for k in ai.get('blend', {}):
            apng_kinds[f'blend_op {k}'] += 1
    apng_flags = Counter()
    for f in apng:
        for k in S(f):
            if k.startswith('apng_'):
                apng_flags[k] += 1
    top_apng = sorted(apng_ok, key=lambda f: -f['scan']['apng_info']['num_frames'])[:5]
    # anomalies
    anomaly_names = ['no_signature', 'no_ihdr', 'first_chunk_not_ihdr', 'ihdr_bad_length', 'invalid_ct_depth', 'zero_dimension',
                     'bad_method_bytes', 'missing_iend', 'truncated_chunk', 'truncated_inside_trailing_chunk', 'truncated_chunk_header', 'trailing_bytes_after_iend',
                     'bad_crc', 'bad_crc_critical', 'bad_crc_ancillary_only', 'surplus_after_zlib_stream', 'zlib_error',
                     'zlib_stream_unterminated', 'inflated_too_short', 'inflated_too_long', 'no_idat', 'empty_idat_chunk',
                     'idat_not_consecutive', 'indexed_without_plte', 'plte_length_not_multiple_of_3', 'plte_larger_than_depth',
                     'trns_with_alpha_type', 'trns_longer_than_palette', 'plte_after_idat', 'trns_after_idat',
                     'colour_chunk_after_idat', 'other_ancillary_after_idat', 'duplicate_single_chunk', 'unknown_critical_chunk',
                     'unknown_ancillary_chunk', 'reserved_bit_set', 'iccp_bad_name_length', 'iccp_name_bad_chars',
                     'iccp_bad_method', 'iccp_and_srgb', 'actl_after_idat', 'apng_chunks_without_actl',
                     'apng_frame_count_mismatch', 'apng_sequence_not_contiguous', 'apng_frame_outside_canvas',
                     'apng_bad_dispose_op', 'apng_bad_blend_op', 'apng_no_fdat']
    anom = {k: {c: sum(1 for f in fs if k in S(f)) for c, fs in list(byc.items()) + [('all', files)]} for k in anomaly_names}
    anom_names = {k: [f for f in files if k in S(f)] for k in anomaly_names}
    clean = {c: sum(1 for f in fs if not (S(f) & {'no_signature', 'no_ihdr', 'first_chunk_not_ihdr', 'invalid_ct_depth', 'zero_dimension',
             'bad_method_bytes', 'missing_iend', 'truncated_chunk', 'truncated_chunk_header', 'trailing_bytes_after_iend', 'bad_crc',
             'surplus_after_zlib_stream', 'zlib_error', 'zlib_stream_unterminated', 'inflated_too_short', 'inflated_too_long',
             'no_idat', 'ihdr_bad_length', 'indexed_without_plte', 'unknown_critical_chunk', 'duplicate_single_chunk'})) for c, fs in list(byc.items()) + [('all', files)]}
    zerr = Counter(f['scan']['zlib'] for f in files if f['scan'].get('zlib') not in (None, 'ok'))
    crc_first = Counter(f['scan'].get('crc_first') for f in files if f['scan'].get('crc_first'))
    trail_sizes = [f['scan']['trailing'] for f in files if f['scan'].get('trailing')]
    surplus_sizes = [f['scan']['surplus'] for f in files if f['scan'].get('surplus')]
    # ancillary kinds
    kind_files = {c: Counter() for c in classes + ['all']}
    for f in files:
        for k in f['scan']['kinds']:
            kind_files[f['cls']][k] += 1
            kind_files['all'][k] += 1
    chunks_total = Counter()
    for f in files:
        for k, v in f['scan']['kinds'].items():
            chunks_total[k] += v
    kinds_order = [k for k, _ in kind_files['all'].most_common()]

    def ex(fs, k=3):
        return ', '.join(f"`{f['repo']}:{f['repo_path']}`" for f in fs[:k])

    def share(a, b):
        return f'{a} ({100 * a / b:.1f}%)' if b else f'{a} (n/a)'

    # JSON
    summary = {
        'files': n, 'bytes': total, 'png_signature': len(sig), 'with_ihdr': len(hdr),
        'class_counts': {c: len(v) for c, v in byc.items()},
        'class_bytes': {c: sum(f['size'] for f in v) for c, v in byc.items()},
        'dev_files': len(dev), 'holdout_files': len(hold),
        'holdout_class_counts': dict(Counter(f['cls'] for f in hold)),
        'candidates_in_trees': cand_total, 'stored_distinct': stored_total, 'exact_duplicates_dropped': dup_total,
        'duplicates_by_repo': dict(dup_by_repo), 'reused_from_previous_run': reuse_total,
        'too_large_over_64MiB': len(toolarge), 'lfs_pointers': dict(lfs_by_repo), 'failed_downloads': len(failed_files),
        'tree_failed': tree_failed,
        'size_hist': size_hist, 'size_percentiles': {q: pct(sizes, q) for q in (0.5, 0.9, 0.99)}, 'size_max': max(sizes) if sizes else 0,
        'files_over_5MiB': len(over5), 'files_over_5MiB_by_class': dict(Counter(f['cls'] for f in over5)),
        'thresholds': thr_all, 'side_hist': {c: dict(v) for c, v in side_hist.items()},
        'colour_depth_all': {f'ct{ct}/d{d}': v for (ct, d), v in sorted(ctd_all.items())},
        'depth16_by_class': {c: {'files': n16[c], 'of': nct[c]} for c in n16},
        'interlaced_by_class': {c: {'files': il[c], 'of': nct[c]} for c in il},
        'apng_files': len(apng), 'apng_frames_hist': dict(frames_hist), 'apng_flags': dict(apng_flags),
        'anomalies': anom, 'anomaly_clean_by_class': clean, 'zlib_errors': dict(zerr),
        'chunk_kind_files_all': dict(kind_files['all']), 'chunk_kind_chunks_total': dict(chunks_total),
        'anomaly_names': {k: [f"{x['repo']}:{x['repo_path']}" for x in v] for k, v in anom_names.items()
                          if k in ('missing_iend', 'trailing_bytes_after_iend', 'bad_crc', 'surplus_after_zlib_stream', 'zlib_error',
                                   'inflated_too_short', 'inflated_too_long', 'iccp_bad_name_length', 'iccp_name_bad_chars',
                                   'apng_frame_count_mismatch', 'apng_sequence_not_contiguous', 'apng_frame_outside_canvas',
                                   'apng_chunks_without_actl', 'trns_longer_than_palette', 'indexed_without_plte', 'no_signature')},
        'big_files': [{'repo': f['repo'], 'path': f['repo_path'], 'w': W(f), 'h': H(f), 'size': f['size'], 'cls': f['cls'],
                       'ct': f['scan']['ct'], 'depth': f['scan']['depth'], 'interlace': f['scan']['interlace']} for f in big_files],
        'thin_files': [{'repo': f['repo'], 'path': f['repo_path'], 'w': W(f), 'h': H(f), 'size': f['size'], 'cls': f['cls']} for f in thin_files[:200]],
        'wide_screens': [{'repo': f['repo'], 'path': f['repo_path'], 'w': W(f), 'h': H(f), 'size': f['size'], 'cls': f['cls']} for f in wide_screens[:200]],
        'over_5MiB_files': [{'repo': f['repo'], 'path': f['repo_path'], 'size': f['size'], 'w': f['scan'].get('width'), 'h': f['scan'].get('height'), 'cls': f['cls']} for f in over5],
        'repos': {r: {'files': sum(1 for f in files if f['repo'] == r), 'bytes': sum(f['size'] for f in files if f['repo'] == r),
                      'holdout': r in {f['repo'] for f in hold}} for r in sorted({f['repo'] for f in files})},
        'files_detail': [{'sha256': f['sha256'], 'repo': f['repo'], 'repo_path': f['repo_path'], 'size': f['size'], 'cls': f['cls'],
                          'holdout': f['holdout'], 'w': f['scan'].get('width'), 'h': f['scan'].get('height'),
                          'ct': f['scan'].get('ct'), 'depth': f['scan'].get('depth'), 'interlace': f['scan'].get('interlace'),
                          'apng': bool(f['scan'].get('apng')), 'flags': sorted(f['scan']['flags']),
                          'zlib': f['scan'].get('zlib'), 'chunks': sorted(f['scan']['kinds'])} for f in files],
    }
    Path(a.out_json).write_text(json.dumps(summary, indent=1, sort_keys=True) + '\n')

    L = []
    w = L.append
    w('# PNG corpus statistics')
    w('')
    w('Generated by `png_stats.py` (standard library only, header-level scan; no oracle or incumbent has been run on these files yet, that is Phase 2). Machine-readable copy: `png-stats.json` (no file bytes). Corpus bytes live in `~/corpora/png/files/`, outside every repo; provenance per file is in `manifests/png.MANIFEST.tsv`.')
    w('')
    w('## Population')
    w('')
    w(f'- {n} distinct files, {total:,} bytes ({total / 2**30:.2f} GiB). {len(sig)} carry the PNG signature; {n - len(sig)} do not (kept as known-bad inputs). {len(hdr)} have a readable IHDR.')
    w('- Classes (path heuristic, see `png_stats.py` docstring): ' + '; '.join(f"{c} {len(byc[c])} files / {sum(f['size'] for f in byc[c]) / MiB:.0f} MiB" for c in classes) + '.')
    w(f'- Hold-out (`holdout-png.tsv`, whole repos chosen by name before any statistic was computed): {len(hold)} files ({", ".join(f"{c} {v}" for c, v in Counter(f["cls"] for f in hold).most_common())}); development set {len(dev)} files.')
    w(f'- Sources: {len(summary["repos"])} repos with at least one stored file; ' + '; '.join(f'{k} {v["files"]}' for k, v in sorted(summary['repos'].items(), key=lambda kv: -kv[1]['files'])) + '.')
    w('')
    w('### Fetch accounting')
    w('')
    w(f'- PNG paths in the pinned trees of the {len(per_repo)} repos that were read: {cand_total:,}. Not all were taken: capped repos stop at their file or MiB cap (`stopped_by` in the table).')
    w(f'- Exact duplicates dropped (same SHA-256 as a file already stored from an earlier repo or an earlier path): {dup_total}. Distinct files stored: {stored_total}. Files above the 64 MiB per-file cap: {len(toolarge)}. Git LFS pointers: {sum(lfs_by_repo.values())}. Failed downloads or blob SHA-1 mismatches: {len(failed_files)}. Trees that could not be read: {len(tree_failed)}' + (' (' + ', '.join(t[0] for t in tree_failed) + ')' if tree_failed else '') + '.')
    w('')
    w('| repo | kind | PNG paths in tree | stored | duplicates | too large | LFS | MiB read | stopped by |')
    w('|---|---|---|---|---|---|---|---|---|')
    kinds_by_repo = {f['repo']: f['kind'] for f in files}
    for rp, d in per_repo.items():
        w(f"| {rp} | {kinds_by_repo.get(rp, '')} | {d['candidates']} | {d['stored']} | {d['duplicates']} | {d.get('too_large', 0)} | {d.get('lfs_pointers', 0)} | {int(d.get('bytes', 0)) / MiB:.1f} | {d.get('stopped_by', '')} |")
    w('')
    w('## Size distribution')
    w('')
    w('| size | ' + ' | '.join(classes + ['all']) + ' |')
    w('|---|' + '---|' * (len(classes) + 1))
    for k, _, _ in bins:
        w(f'| {k} | ' + ' | '.join(str(size_hist[c][k]) for c in classes + ['all']) + ' |')
    w('')
    w(f'Median {fmt_size(pct(sizes, .5))}, p90 {fmt_size(pct(sizes, .9))}, p99 {fmt_size(pct(sizes, .99))}, largest {fmt_size(max(sizes) if sizes else 0)}.')
    w('')
    w(f"**Between 5 MiB and 64 MiB (stored only because the per-file cap is 64 MiB; the GIF script's 5 MiB cap would have dropped them): {len(over5)} files** ({', '.join(f'{c} {v}' for c, v in Counter(f['cls'] for f in over5).most_common()) or 'none'}), {sum(f['size'] for f in over5) / MiB:.0f} MiB. Above 64 MiB (dropped, logged `TOO-LARGE`): {len(toolarge)}" + (' (' + ', '.join(f'{t[0]}:{t[1]}' for t in toolarge[:10]) + ')' if toolarge else '') + '.')
    w('')
    if over5:
        w('| file | class | dimensions | size | colour type / depth |')
        w('|---|---|---|---|---|')
        for f in over5[:40]:
            s = f['scan']
            w(f"| `{f['repo']}:{f['repo_path']}` | {f['cls']} | {s.get('width')}x{s.get('height')} | {fmt_size(f['size'])} | {s.get('ct')}/{s.get('depth')} |")
        if len(over5) > 40:
            w(f'(first 40 of {len(over5)}; all in `png-stats.json` `over_5MiB_files`)')
        w('')
    w('## Dimensions')
    w('')
    w('Thresholds from PNG-Q5: side above 16,384; area above 16,777,216 pixels (RGBA8 output above 64 MiB, the cap; RGBA16 above 128 MiB); area above 33,554,432 pixels (RGBA8 above 128 MiB, RGBA16 above 256 MiB). Counts are files with a readable IHDR; the IHDR is the file\'s claim, whether or not the data decode.')
    w('')
    cols = classes + ['non-fuzz', 'all']
    w('| measure | ' + ' | '.join(cols) + ' |')
    w('|---|' + '---|' * len(cols))
    for k in thr_all['all']:
        w(f'| {k} | ' + ' | '.join(str(thr_all[c][k]) for c in cols) + ' |')
    w('')
    w('Longest side:')
    w('')
    w('| longest side | ' + ' | '.join(classes + ['all']) + ' |')
    w('|---|' + '---|' * (len(classes) + 1))
    for k in side_order:
        w(f'| {k} | ' + ' | '.join(str(side_hist[c].get(k, 0)) for c in classes + ['all']) + ' |')
    w('')
    w('Largest files by pixel area:')
    w('')
    w('| file | class | dimensions | Mpx | file size | ct/depth | interlace |')
    w('|---|---|---|---|---|---|---|')
    for f in top_area:
        s = f['scan']
        w(f"| `{f['repo']}:{f['repo_path']}` | {f['cls']} | {W(f)}x{H(f)} | {area(f) / 1e6:.1f} | {fmt_size(f['size'])} | {s['ct']}/{s['depth']} | {s['interlace']} |")
    w('')
    w(f'Files over the PNG-Q5 caps (side above 16,384 or area above 16.7 Mpx): {len(big_files)}.')
    w('')
    if big_files:
        w('| file | class | dimensions | Mpx | file size | ct/depth | interlace |')
        w('|---|---|---|---|---|---|---|')
        for f in big_files[:40]:
            s = f['scan']
            w(f"| `{f['repo']}:{f['repo_path']}` | {f['cls']} | {W(f)}x{H(f)} | {area(f) / 1e6:.1f} | {fmt_size(f['size'])} | {s['ct']}/{s['depth']} | {s['interlace']} |")
        w('')
    w(f'Wide screenshots (width at least 3840, height at least 1080, width/height at least 2.2; ultrawide or multi-monitor): {len(wide_screens)}' + (': ' + ex(wide_screens, 8) if wide_screens else '') + '.')
    w('')
    w(f'Long thin images (aspect ratio at least 32:1 and long side at least 1024): {len(thin_files)}' + (': ' + ', '.join(f"`{f['repo']}:{f['repo_path']}` {W(f)}x{H(f)}" for f in thin_files[:10]) if thin_files else '') + '.')
    w('')
    w(f'Files whose filtered data R(w,h) exceeds 64 MiB (inflate output the decoder must hold): {len(raw_big)}. Interlaced files above 16.7 Mpx: {len(ilbig)}.')
    w('')
    w('## Colour types, bit depths, interlacing')
    w('')
    w('| colour type / depth | files | share |')
    w('|---|---|---|')
    names = {0: 'greyscale', 2: 'truecolour', 3: 'indexed', 4: 'grey+alpha', 6: 'RGBA'}
    for (ct, d), v in sorted(ctd_all.items(), key=lambda kv: -kv[1]):
        w(f'| {ct} ({names.get(ct, "invalid")}) / {d} | {v} | {100 * v / max(1, nct["all"]):.1f}% |')
    w('')
    w('16-bit files: ' + '; '.join(f'{c} {share(n16[c], nct[c])}' for c in classes + ['all']) + '.')
    w('')
    w('Adam7 interlaced files: ' + '; '.join(f'{c} {share(il[c], nct[c])}' for c in classes + ['all']) + '.')
    w('')
    w('## APNG')
    w('')
    w(f'{len(apng)} files carry acTL, fcTL or fdAT chunks ({len(apng_ok)} with a well-formed acTL). Frame counts (by acTL): ' + (', '.join(f'{k} {v}' for k, v in sorted(frames_hist.items())) or 'none') + '. By class: ' + ', '.join(f"{c} {sum(1 for f in apng if f['cls'] == c)}" for c in classes) + '.')
    w('')
    if apng:
        w('Structure: ' + '; '.join(f'{k} {v}' for k, v in sorted(apng_kinds.items())) + '.')
        w('')
        w('Anomalies among APNG files: ' + ('; '.join(f'{k} {v}' for k, v in apng_flags.most_common()) or 'none') + '.')
        w('')
        w('Most frames: ' + ', '.join(f"`{f['repo']}:{f['repo_path']}` {f['scan']['apng_info']['num_frames']}" for f in top_apng) + '.')
        w('')
    w('## Damage and anomalies (header-level)')
    w('')
    w('Files with the feature, by class. "Clean" means none of: bad signature, IHDR damage, invalid colour type/depth, missing IEND, truncated chunk, trailing bytes after IEND, bad CRC, surplus after the zlib stream, a zlib error, unterminated stream, inflated size different from R(w,h), no IDAT, indexed without PLTE, unknown critical chunk, duplicated single-instance chunk. These are scan flags, not the Strict oracle\'s verdicts; the Strict first error needs Phase 2.')
    w('')
    w('| feature | ' + ' | '.join(classes + ['all']) + ' | examples (not fuzz first) |')
    w('|---|' + '---|' * (len(classes) + 1) + '---|')
    w('| **clean** | ' + ' | '.join(share(clean[c], len(byc[c]) if c != 'all' else n) for c in classes + ['all']) + ' | |')
    for k in anomaly_names:
        if anom[k]['all']:
            fl = sorted(anom_names[k], key=lambda f: f['cls'] == 'fuzz')
            w(f'| {k} | ' + ' | '.join(str(anom[k][c]) for c in classes + ['all']) + f' | {ex(fl)} |')
    w('')
    w('Anomalies with zero files in the corpus: ' + ', '.join(k for k in anomaly_names if not anom[k]['all']) + '.')
    w('')
    if zerr:
        w('zlib error classes (inflate of the concatenated IDAT payloads): ' + '; '.join(f'`{k}` {v}' for k, v in zerr.most_common()) + '.')
        w('')
    if crc_first:
        w('Chunk type of the first bad CRC: ' + '; '.join(f'{k} {v}' for k, v in crc_first.most_common()) + '.')
        w('')
    if trail_sizes:
        w(f'Bytes after IEND: {len(trail_sizes)} files; median {pct(trail_sizes, .5)} bytes, p90 {pct(trail_sizes, .9)}, max {max(trail_sizes)}.')
        w('')
    if surplus_sizes:
        w(f'Surplus bytes after the end of the zlib stream inside the IDAT payload: {len(surplus_sizes)} files; median {pct(surplus_sizes, .5)} bytes, max {max(surplus_sizes)}.')
        w('')
    w('## Chunk kinds')
    w('')
    w('Files containing at least one chunk of the kind (chunks in total in the last column).')
    w('')
    w('| chunk | ' + ' | '.join(classes + ['all']) + ' | chunks |')
    w('|---|' + '---|' * (len(classes) + 1) + '---|')
    for k in kinds_order:
        w(f'| `{k}` | ' + ' | '.join(str(kind_files[c][k]) for c in classes + ['all']) + f' | {chunks_total[k]} |')
    w('')
    w('## Duplicates and the 737 versus 748 counts')
    w('')
    w(f'The corpus has {dup_total} exact duplicates removed (same SHA-256; the first occurrence in the order of `repos-png.tsv` is kept). Per repo: ' + (', '.join(f'{k} {v}' for k, v in dup_by_repo.most_common(15)) or 'none') + f'. Listing: `manifests/png.DUPLICATES.tsv`. Candidates read in the trees: {cand_total:,}; stored {stored_total}; duplicates {dup_total}; too large {len(toolarge)}; LFS pointers {sum(lfs_by_repo.values())}; failed {len(failed_files)}; the remainder were not read because a per-repo cap stopped the repo or the file was empty.')
    w('')
    w('The earlier packet\'s two counts concern the synthetic inputs, not this corpus: 748 is the number of inputs in `png/policy-draft/probes/results.json` (516 vectors, 177 probes, 55 witnesses, all with distinct SHA-256, every one run through ten runners); 737 is the number of distinct inputs that `png/policy-draft/validate.py` feeds to its Strict-bridge and obligation-R checks. `validate.py` builds that set from `spec-draft/VECTORS.json` (591 entries without `pad_to`, 536 distinct byte strings) plus the 734 results entries whose bytes are stored as hex in `results.json`, deduplicated by bytes (201 are not vectors), giving 737. The 11 missing from 748 are the inputs above 4 KiB that `results.json` stores as a generator name instead of hex and that are not vectors: probes `iccp-bomb-100m`, `ztxt-bomb-64m`, `itxt-bomb-64m`, `text-many-20000`, `text-large-8m` and witnesses `03-length-IEND`, `04-length-IDAT`, `25-type-bit-safe-to-copy`, `26-type-bit-private`, `27-apng-actl-without-fctl`, `33-apng-add-single-frame`. (Three other above-4 KiB inputs, `idat/split-1-byte`, `icc/too-large`, `icc/exactly-max`, are vectors and so are in the 737.) A third figure, 721, is the fitted comparison set against the runners: results with stored hex that no runner crashed on or that are not beyond a resource cap. PNG-Q7 in `QUESTIONS.md` says "737 synthetic inputs (vectors, 177 targeted probes and 55 distinct witnesses)", which adds a wrong total to that composition; the composition sums to 748.')
    w('')
    w('## What this shows and what it cannot show')
    w('')
    ip = HERE / 'png-STATS-interpretation.md'
    w(ip.read_text().rstrip() if ip.exists() else '(interpretation file missing)')
    Path(a.out_md).write_text('\n'.join(L) + '\n')


if __name__ == '__main__':
    main()
