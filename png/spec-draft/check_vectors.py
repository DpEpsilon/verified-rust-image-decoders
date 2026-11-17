#!/usr/bin/env python3
"""Run png_oracle.py against every VECTORS.json record (SPEC.md P19 schema), report error
coverage, and cross-check the oracle's hand-written inflate, CRC-32 and Adler-32 against
Python's zlib (the only place zlib meets the oracle).

Usage: python3 -I check_vectors.py [VECTORS.json] [--fuzz N] [--only SUBSTRING] [--caps NAME=V,...] [--usage]
  --caps   evaluate the (Strict) records with the named caps replaced (the Stage-1 fixture envelope, REVIEW-BRIEF.md)
  --usage  print, per cap, the largest quantity any record compares with it, and the envelope's lower corner
"""
import hashlib
import json
import random
import struct
import sys
import zlib
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import png_oracle as O  # noqa: E402
_PO = None


def policy_oracle():
    """Imported only for a record whose `profile` is not Strict, so the Strict check never depends on ../policy-draft."""
    global _PO
    if _PO is None:
        sys.path.insert(0, str(HERE.parent / 'policy-draft'))
        import policy_oracle as PO  # noqa: E402
        _PO = PO
    return _PO


def runs(pix, digits=8):
    out = []
    for p in pix:
        h = ''.join(f'{v:0{digits // 4}x}' for v in p)
        if out and out[-1]['hex'] == h:
            out[-1]['count'] += 1
        else:
            out.append({'count': 1, 'hex': h})
    return out


def quads(flat):
    return [tuple(flat[i:i + 4]) for i in range(0, len(flat), 4)]


def materialize(v):
    b = bytes.fromhex(v['input_hex'])
    if 'pad_to' in v:
        b = b + bytes(v['pad_to'] - len(b))
    return b


def actual_profile(v):
    """Records with `profile` other than Strict (P19): the profile's still/frames entry point, with recovered and events."""
    PO = policy_oracle()
    b, P = materialize(v), PO.PROFILES[v['profile']]
    if v['entry'] == 'still':
        r = PO.decode_still(b, P)
        if 'error' in r:
            return {'error': r['error']}
        return {'width': r['width'], 'height': r['height'], 'rgba_runs': runs(quads(r['rgba'])),
                'recovered': r['recovered'], 'events': r['events']}
    assert v['entry'] == 'frames', v['name']
    r = PO.decode_frames(b, P)
    if 'error' in r:
        return {'error': r['error']}
    return {'width': r['width'], 'height': r['height'], 'num_plays': r['num_plays'], 'end': r['end'],
            'frames': [{'rect': f['rect'], 'delay': f['delay'], 'dispose': f['dispose'], 'blend': f['blend'],
                        'rgba_runs': runs(quads(f['rgba'])), 'recovered': f['recovered'], 'events': f['events']}
                       for f in r['frames']]}


def actual(v):
    if v.get('profile', 'Strict') != 'Strict':
        return actual_profile(v)
    b = materialize(v)
    e = v['entry']
    if e == 'still':
        r = O.decode_still(b)
        if 'error' in r:
            return {'error': r['error']}
        return {'width': r['width'], 'height': r['height'], 'rgba_runs': runs(quads(r['rgba']))}
    if e == 'still16':
        r = O.decode_still16(b)
        if 'error' in r:
            return {'error': r['error']}
        return {'width': r['width'], 'height': r['height'], 'rgba16_runs': runs(quads(r['rgba16']), 16)}
    if e == 'icc':
        r = O.icc_profile(b)
        if 'error' in r:
            return {'error': r['error']}
        if r['icc'] is None:
            return {'icc': None}
        return {'icc_len': len(r['icc']), 'icc_sha256': hashlib.sha256(r['icc']).hexdigest()}
    r = O.decode_frames(b)
    if 'error' in r:
        return {'error': r['error']}
    return {'width': r['width'], 'height': r['height'], 'num_plays': r['num_plays'], 'end': r['end'],
            'frames': [{'rect': f['rect'], 'delay': f['delay'], 'dispose': f['dispose'], 'blend': f['blend'],
                        'rgba_runs': runs(quads(f['rgba']))} for f in r['frames']]}


WINDOW_MAX = 32768


def req_icc(b):
    """P20 (three cases, D7): the requests of icc_profile when no reservation fails, as [site, elements] pairs."""
    try:
        doc = O.walk(b)
    except O.PngError:
        return []
    if doc['iccp'] is None:
        return []
    try:
        m = len(O.zlib_decompress(doc['iccp'], O.MAX_ICC, 'IccProfileTooLarge'))
    except O.PngError:
        return [['IccWindow', WINDOW_MAX]]
    return [['IccWindow', WINDOW_MAX], ['Icc', m]]


def zlib_streams(b):
    """Concatenated IDAT payload and each frame's fdAT payload of a well-framed input (no checks). An IDAT run
    whose payloads are all empty yields the empty stream b'' (it exists and is inflated, P9: TruncatedZlib); no
    IDAT chunk yields no stream. Likewise a frame with at least one fdAT chunk yields its payload, possibly b'',
    and a frame without fdAT chunks (IDAT-based, or MissingFrameData) yields none."""
    out, p, idat, frames, idat_seen = [], 8, bytearray(), [], False
    while p + 12 <= len(b):
        n = struct.unpack('>I', b[p:p + 4])[0]
        t = b[p + 4:p + 8]
        if p + 12 + n > len(b) or n > 1 << 28:
            break
        d = b[p + 8:p + 8 + n]
        if t == b'IDAT':
            idat += d
            idat_seen = True
        elif t == b'fcTL':
            frames.append(None)
        elif t == b'fdAT' and frames and n >= 4:        # a shorter fdAT is InvalidChunkLength, not frame data
            frames[-1] = (frames[-1] or bytearray()) + d[4:]
        p += 12 + n
    if idat_seen:
        out.append(bytes(idat))
    out += [bytes(f) for f in frames if f is not None]
    return out


def zlib_ref(data):
    """Python zlib: (output, unused_tail) or the exception name."""
    try:
        d = zlib.decompressobj()
        o = d.decompress(data)
        if not d.eof:
            return 'incomplete'
        return o, d.unused_data
    except zlib.error as e:
        return 'zlib.error: ' + str(e)


def ours(data):
    try:
        return O.zlib_decompress(data, 1 << 30, 'ExcessImageData')
    except O.PngError as e:
        return e.name


EXPECTED_ZLIB_DIFFERENCES = {
    # Python's zlib accepts these streams; the oracle rejects them by P9/P10 rules zlib does not enforce.
    'DistanceTooFar',       # zlib checks distance against output only, not against the CINFO window
    'TrailingZlibData',     # zlib reports trailing bytes as unused_data, not an error
}


def cross_check(vs, fuzz):
    agree = differ = 0
    notes = {}
    for v in vs:
        if 'pad_to' in v:
            continue
        for s in zlib_streams(materialize(v)):
            mine, ref = ours(s), zlib_ref(s)
            if isinstance(ref, tuple):
                out, tail = ref
                good = (mine == out) if not tail else (mine == 'TrailingZlibData')
            else:
                good = isinstance(mine, str)
            if not good and isinstance(mine, str) and mine in EXPECTED_ZLIB_DIFFERENCES:
                notes[mine] = notes.get(mine, 0) + 1
                good = True
            if good:
                agree += 1
            else:
                differ += 1
                print('INFLATE DIFFERS', v['name'], str(mine)[:60], str(ref)[:80])
    rng = random.Random(1)
    fz = 0
    for i in range(fuzz):
        n = rng.randrange(0, 20000)
        alphabet = rng.randrange(1, 257)
        data = bytes(rng.randrange(alphabet) if rng.random() < 0.7 else 0 for _ in range(n))
        lv, st, wb = rng.randrange(10), rng.choice([0, 1, 2, 3, 4]), rng.randrange(9, 16)
        c = zlib.compressobj(lv, 8, wb, rng.randrange(1, 10), st)
        zs = c.compress(data) + c.flush()
        if ours(zs) != data:
            fz += 1
            print('FUZZ MISMATCH', i, lv, st, wb)
        # random single-byte corruption: both must reject, or both accept with equal output
        k = rng.randrange(len(zs))
        bad = zs[:k] + bytes([zs[k] ^ (1 << rng.randrange(8))]) + zs[k + 1:]
        mine, ref = ours(bad), zlib_ref(bad)
        if isinstance(ref, tuple) and not ref[1]:
            if mine != ref[0] and mine not in EXPECTED_ZLIB_DIFFERENCES:
                fz += 1
                print('FUZZ CORRUPT ACCEPTED BY ZLIB ONLY', i, k, mine)
        elif not isinstance(mine, str) and not (isinstance(ref, tuple) and ref[1]):
            fz += 1
            print('FUZZ CORRUPT ACCEPTED BY ORACLE ONLY', i, k, str(ref)[:60])
    crc_ok = all(O.crc32(x) == zlib.crc32(x) for x in (b'', b'IEND', bytes(range(256)) * 3))
    adl_ok = all(O.adler32(x) == zlib.adler32(x) for x in (b'', b'a', bytes(70000), bytes(range(256)) * 300))
    print(f'inflate vs zlib on vector streams: {agree} agree, {differ} differ '
          f'(expected rule differences counted as agreement: {notes}); fuzz {fuzz} streams + {fuzz} corruptions, '
          f'{fz} mismatches; crc32 {"ok" if crc_ok else "MISMATCH"}; adler32 {"ok" if adl_ok else "MISMATCH"}')
    return differ + fz + (not crc_ok) + (not adl_ok)


CAP_NAMES = ('MAX_INPUT', 'MAX_DIMENSION', 'MAX_OUTPUT', 'MAX_FRAMES', 'MAX_TOTAL_PIXELS', 'MAX_ICC')
PNG_WIRE_MAX = (1 << 31) - 1
CAPS_OK_MIN = {'MAX_INPUT': 0, 'MAX_DIMENSION': 1, 'MAX_OUTPUT': 4, 'MAX_FRAMES': 1, 'MAX_TOTAL_PIXELS': 0, 'MAX_ICC': 1}


def usage(v):
    """The largest quantity each cap is compared with while decoding record v (an upper bound, from the chunk framing,
    no checks): |b|; max(W, H) and 4*W*H when both sides are in 1..PNG_WIRE_MAX (otherwise InvalidDimensions holds for
    every permitted MAX_DIMENSION and the area test is never reached); acTL num_frames; the sum of fcTL w*h; the
    inflated iCCP profile length for an icc record. A record keeps its outcome for every cap tuple with each cap at least
    this value (each cap enters only through a test `quantity > cap`)."""
    b = materialize(v)
    u = {'MAX_INPUT': len(b)}
    p, first, pixels = 8, True, 0
    while p + 12 <= len(b):
        n = struct.unpack('>I', b[p:p + 4])[0]
        t = b[p + 4:p + 8]
        if p + 12 + n > len(b):
            break
        d = b[p + 8:p + 8 + n]
        if first and t == b'IHDR' and n == 13:
            W, H = struct.unpack('>II', d[:8])
            if 1 <= W <= PNG_WIRE_MAX and 1 <= H <= PNG_WIRE_MAX:
                u['MAX_DIMENSION'], u['MAX_OUTPUT'] = max(W, H), 4 * W * H
        elif t == b'acTL' and n == 8:
            u['MAX_FRAMES'] = max(u.get('MAX_FRAMES', 0), struct.unpack('>I', d[:4])[0])
        elif t == b'fcTL' and n == 26:
            fw, fh = struct.unpack('>II', d[4:12])
            pixels += fw * fh
            u['MAX_TOTAL_PIXELS'] = pixels
        elif t == b'iCCP' and v['entry'] == 'icc':
            z = d.find(b'\0')
            try:
                u['MAX_ICC'] = len(zlib.decompressobj().decompress(d[z + 2:], 1 << 31))
            except zlib.error:
                pass
        first = False
        p += 12 + n
    return u


def set_caps(spec):
    """--caps NAME=VALUE,...: evaluate the Strict records at other cap values (png_oracle reads its caps at call time)."""
    for item in spec.split(','):
        k, val = item.split('=')
        assert k in CAP_NAMES, k
        setattr(O, k, int(val, 0))


def main():
    args = sys.argv[1:]
    fuzz = int(args[args.index('--fuzz') + 1]) if '--fuzz' in args else 200
    only = args[args.index('--only') + 1] if '--only' in args else None
    pos = [a for i, a in enumerate(args) if not a.startswith('--') and (i == 0 or not args[i - 1] in ('--fuzz', '--only', '--caps'))]
    path = Path(pos[0]) if pos else HERE / 'VECTORS.json'
    vs = json.loads(path.read_text())
    if '--usage' in args:
        top = {}
        for v in vs:
            for k, q in usage(v).items():
                if q > top.get(k, (-1, ''))[0]:
                    top[k] = (q, v['name'])
        for k in CAP_NAMES:
            q, nm = top.get(k, (0, '-'))
            print(f'{k}: largest use {q} ({nm})')
        print('lower corner: --caps ' + ','.join(f'{k}={max(CAPS_OK_MIN[k], top.get(k, (0,))[0])}' for k in CAP_NAMES))
        return 0
    if '--caps' in args:
        assert all(v.get('profile', 'Strict') == 'Strict' for v in vs), '--caps evaluates Strict records only'
        set_caps(args[args.index('--caps') + 1])
        print('caps: ' + ', '.join(f'{k}={getattr(O, k)}' for k in CAP_NAMES))
    if only:
        vs = [v for v in vs if only in v['name']]
    names = [v['name'] for v in vs]
    assert len(names) == len(set(names)), 'duplicate vector names'
    bad = 0
    seen = set()
    for v in vs:
        a = actual(v)
        if a != v['expected']:
            bad += 1
            print('MISMATCH', v['name'], json.dumps(a)[:300], 'expected', json.dumps(v['expected'])[:300])
        if 'req' in v:
            assert v['entry'] == 'icc', v['name']
            r = req_icc(materialize(v))
            if r != v['req']:
                bad += 1
                print('REQ MISMATCH', v['name'], r, 'expected', v['req'])
        e = v['expected']
        if v.get('profile', 'Strict') != 'Strict':
            continue            # P19: the error list counts Strict records only
        if 'error' in e:
            seen.add(e['error'])
        elif isinstance(e.get('end'), dict):
            seen.add(e['end']['error'])
    unused = sorted(set(O.ERRORS) - seen)
    print(f'{len(vs) - bad}/{len(vs)} vectors agree; errors exercised {len(seen & set(O.ERRORS))}/{len(O.ERRORS)}; '
          f'unexercised: {unused}')
    xbad = cross_check(vs, fuzz) if not only else 0
    return 1 if bad or xbad else 0


if __name__ == '__main__':
    sys.exit(main())
