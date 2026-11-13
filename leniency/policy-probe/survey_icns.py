#!/usr/bin/env python3
"""Header-level survey of an ICNS corpus for the policy draft (reads entry headers
by seeking, plus the payloads of legacy colour/mask entries only).
Usage: python3 -I survey_icns.py CORPUS_DIR OUT_JSON"""
import collections, json, os, struct, sys
from multiprocessing import Pool

LEG = {b'is32': 16, b'il32': 32, b'ih32': 48, b'it32': 128}
MSK = {b's8mk': 16, b'l8mk': 32, b'h8mk': 48, b't8mk': 128}
PNG = b'\x89PNG\r\n\x1a\n'
JP2 = bytes.fromhex('0000000c6a5020200d0a870a')

def mr(s, n):
    cur = 0
    for _ in range(3):
        prod = 0
        while prod < n:
            if cur == len(s): return 'RleTruncated'
            c = s[cur]; cur += 1
            cnt, lit = (c + 1, True) if c < 128 else (c - 125, False)
            if cnt > n - prod: return 'RleOverrun'
            need = cnt if lit else 1
            if len(s) - cur < need: return 'RleTruncated'
            cur += need; prod += cnt
    return 'ok' if cur == len(s) else 'RleTrailingBytes'

def survey(path):
    f = {}
    size = os.stat(path).st_size
    with open(path, 'rb') as fh:
        h = fh.read(8)
        if len(h) < 8: return {'strict': 'TruncatedHeader'}
        if h[:4] != b'icns': return {'strict': 'BadMagic'}
        L = struct.unpack('>I', h[4:])[0]
        if L < 8: return {'strict': 'InvalidFileLength'}
        strict = None
        if L > size: strict = 'TruncatedFile'
        elif L < size: strict = 'TrailingFileBytes'; f['suffix_bytes'] = size - L
        end = min(L, size)
        off, entries, scan_err = 8, [], None
        while off < end:
            if end - off < 8: scan_err = 'TruncatedEntryHeader'; break
            fh.seek(off); eh = fh.read(8)
            t, q = eh[:4], struct.unpack('>I', eh[4:])[0]
            if q < 8: scan_err = 'InvalidEntryLength'; break
            if q > end - off: scan_err = 'EntryOutOfBounds'; break
            head = fh.read(min(12, q - 8))
            entries.append((t, off + 8, off + q, head)); off += q
        if strict is None and scan_err: strict = scan_err
        if strict is None and len(entries) > 4096: strict = 'TooManyEntries'
        f['strict'] = strict or 'ok'
        f['scan'] = scan_err or 'ok'
        f['n'] = len(entries)
        # pixel-level facts for legacy entries (last occurrence wins, as in GDK)
        col, msk = {}, {}
        for t, a, b, head in entries:
            if t in LEG: col[LEG[t]] = (t, a, b)
            if t in MSK: msk[MSK[t]] = (a, b)
        f['png_types'] = sorted({t.decode('latin1') for t, a, b, hd in entries if hd.startswith(PNG) and t not in LEG and t not in MSK})
        f['jp2_types'] = sorted({t.decode('latin1') for t, a, b, hd in entries if hd.startswith(JP2) and t not in LEG and t not in MSK})
        f['argb_types'] = sorted({t.decode('latin1') for t, a, b, hd in entries if t in (b'ic04', b'ic05', b'icsb') and not hd.startswith(PNG) and not hd.startswith(JP2)})
        f['old_types'] = sorted({t.decode('latin1') for t, a, b, hd in entries if t in (b'ICN#', b'icl8', b'icl4', b'ics#', b'ics8', b'ics4', b'ich#', b'ich8', b'ich4', b'icm#', b'icm8', b'icm4', b'ICON')})
        pairs = {}
        for side, (t, a, b) in col.items():
            n = side * side
            fh.seek(a); p = fh.read(b - a)
            raw = len(p) == 4 * n
            q = p[4:] if t == b'it32' and p[:4] == b'\0\0\0\0' else p
            rle = mr(q, n)
            zero_other = t != b'it32' and p[:4] == b'\0\0\0\0'
            alt = mr(p[4:], n) if zero_other else None
            m = msk.get(side)
            mlen = None if m is None else (m[1] - m[0] == n)
            pairs[side] = {'rle': rle, 'raw4n': raw, 'zero_prefix_non_it32': zero_other, 'rle_after_strip': alt, 'mask': mlen}
        f['pairs'] = pairs
    return f

def main():
    d, out = sys.argv[1], sys.argv[2]
    paths = sorted(os.path.join(d, x) for x in os.listdir(d))
    with Pool(4) as p:
        res = p.map(survey, paths, chunksize=64)
    json.dump(dict(zip([os.path.basename(x) for x in paths], res)), open(out, 'w'))

if __name__ == '__main__':
    main()
