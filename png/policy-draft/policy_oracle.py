#!/usr/bin/env python3
"""PNG policy draft 0.1: the strict oracle (../spec-draft/png_oracle.py) indexed by an option
record. Every option site names its OPTIONS.md field; with STRICT the results equal png_oracle
on every input (check: validate.py, Strict bridge).

decode_still(b, P) / decode_frames(b, P) return png_oracle's records plus 'recovered' (still) or
per-frame 'recovered', from the closed event list R1-R5 of OPTIONS.md.
"""
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'spec-draft'))
import png_oracle as O  # noqa: E402
from png_oracle import (PngError, fail, be16, be32, crc32, adler32, parse_ihdr, split_keyword,  # noqa: E402
                        BEFORE_PLTE, AFTER_PLTE, BEFORE_IDAT, SINGLE, FIXED_LEN, SBIT_LEN, BKGD_LEN,
                        MAX_INPUT, MAX_FRAMES, MAX_TOTAL_PIXELS, OutputLimit)

STRICT = {
    'crc_critical': 'Reject', 'crc_ancillary': 'Reject', 'iend': 'Strict', 'chunk_type': 'Reject',
    'reserved_bit': 'Accept', 'after_idat': 'Full', 'missing_iend': 'Reject', 'eof': 'Reject', 'trailing': 'Reject',
    'ancillary_order': 'Reject', 'ancillary_duplicate': 'Reject', 'ancillary_values': 'Reject',
    'trns_invalid': 'Reject', 'plte_unexpected': 'Reject', 'palette_size': 'Reject', 'palette_index': 'Reject',
    'adler': 'Check', 'stream_end': 'Require', 'zlib_tail': 'Reject', 'window': 'Enforce', 'single_code': 'Accept',
    'image_data_error': 'Reject', 'depth16': 'Round', 'apng_in_still': 'Validate', 'apng_errors': 'Reject',
    'blend': 'Exact', 'dispose_previous': 'Spec', 'apng16': 'Composite',
}
RECOMMENDED_BETA = dict(STRICT, crc_ancillary='Skip', iend='Lenient', missing_iend='AtEof', trailing='Ignore',
                        ancillary_order='Drop', ancillary_duplicate='FirstWins', ancillary_values='Drop',
                        trns_invalid='Drop', plte_unexpected='Ignore', palette_size='Allow', palette_index='Black',
                        zlib_tail='Ignore', window='Ignore', apng_in_still='Ignore')
GDK = dict(RECOMMENDED_BETA, reserved_bit='RejectBeforeIdat', zlib_tail='StopAtRaster', eof='GdkPartial', image_data_error='GdkGrey', depth16='HighByte',
           apng_errors='Static', apng_in_still='Ignore')
GDK['apng'] = 'Off'
IMAGE = dict(RECOMMENDED_BETA, chunk_type='Ancillary', after_idat='ImageRs', missing_iend='Reject',
             ancillary_order='ImageRs', ancillary_values='ImageRs', trns_invalid='ImageRs', adler='Ignore',
             stream_end='AtRaster', single_code='Reject', apng_in_still='ImageRs', apng_errors='Prefix',
             blend='ImageF32', dispose_previous='ImageRs', apng16='Reject')
for _p in (STRICT, RECOMMENDED_BETA, IMAGE):
    _p.setdefault('apng', 'On')
PROFILES = {'Strict': STRICT, 'RecommendedBeta': RECOMMENDED_BETA, 'GdkCompat-2.42.12': GDK,
            'ImageCrateCompat-0.25.10': IMAGE}
VALUES = {
    'crc_critical': ('Reject', 'Ignore'), 'crc_ancillary': ('Reject', 'Skip', 'Ignore'), 'iend': ('Strict', 'Lenient'),
    'chunk_type': ('Reject', 'Ancillary'), 'reserved_bit': ('Accept', 'Reject', 'RejectBeforeIdat'), 'after_idat': ('Full', 'ImageRs'),
    'missing_iend': ('Reject', 'AtEof'), 'eof': ('Reject', 'GdkPartial'), 'trailing': ('Reject', 'Ignore'),
    'ancillary_order': ('Reject', 'Drop', 'ImageRs'), 'ancillary_duplicate': ('Reject', 'FirstWins'),
    'ancillary_values': ('Reject', 'Drop', 'ImageRs'), 'trns_invalid': ('Reject', 'Drop', 'ImageRs'),
    'plte_unexpected': ('Reject', 'Ignore'), 'palette_size': ('Reject', 'Allow'), 'palette_index': ('Reject', 'Black'),
    'adler': ('Check', 'Ignore'), 'stream_end': ('Require', 'AtRaster'), 'zlib_tail': ('Reject', 'Ignore', 'StopAtRaster'),
    'window': ('Enforce', 'Ignore'), 'single_code': ('Accept', 'Reject'), 'image_data_error': ('Reject', 'GdkGrey'),
    'depth16': ('Round', 'HighByte'), 'apng_in_still': ('Validate', 'Ignore', 'ImageRs'),
    'apng_errors': ('Reject', 'Static', 'Prefix'), 'blend': ('Exact', 'ImageF32'), 'dispose_previous': ('Spec', 'ImageRs'),
    'apng16': ('Composite', 'Reject'), 'apng': ('On', 'Off'),
}
for _p in PROFILES.values():
    assert set(_p) == set(VALUES), set(_p) ^ set(VALUES)
    assert all(_p[k] in VALUES[k] for k in _p)

APNG_TYPES = {b'acTL', b'fcTL', b'fdAT'}
APNG_ERRORS = {'MissingAnimationControl', 'InvalidAnimationControl', 'TooManyFrames', 'InvalidFrameControl',
               'SequenceError', 'UnexpectedFrameData', 'MissingFrameData', 'FrameCountMismatch', 'PixelBudgetExceeded'}
GREY = (0x97, 0x98, 0x99, 0xFF)                     # io-png.c:41 prefill colour


class ApngError(Exception):
    """Internal: an APNG structure error at chunk offset `at` (for apng_errors=Prefix/Static)."""

    def __init__(self, name, frames_complete, actl):
        super().__init__(name)
        self.name, self.frames_complete, self.actl = name, frames_complete, actl


class Eof(Exception):
    """Internal: eof=GdkPartial, the input ended inside or after the image data."""


# ---------------------------------------------------------------- zlib with policy (P9/P10)
def zlib_decompress(data, limit, limit_error, P, ev, partial=None):
    """P9 with `adler`, `stream_end`, `zlib_tail`, `window`, `single_code`. `partial` receives the
    output produced before an error (image_data_error=GdkGrey)."""
    out = partial if partial is not None else bytearray()
    if len(data) < 2:
        fail('TruncatedZlib')
    cmf, flg = data[0], data[1]
    if (cmf * 256 + flg) % 31:
        fail('InvalidZlibHeader')
    if cmf & 15 != 8 or cmf >> 4 > 7:
        fail('UnsupportedZlibMethod')
    if flg & 0x20:
        fail('ZlibPresetDictionary')
    window = 1 << 15 if P['window'] == 'Ignore' else 1 << ((cmf >> 4) + 8)
    lim = limit + 1 if (P['zlib_tail'] == 'Ignore' or P['stream_end'] == 'AtRaster') else limit
    stop = P['zlib_tail'] == 'StopAtRaster'                 # libpng: stop inflating at the first surplus byte
    saved = O.Huffman.__init__
    if P['single_code'] == 'Reject':
        def strict_init(self, lengths):
            saved(self, lengths)
            if self.single_len1 and self.nsyms == 1 and sum(lengths) == 1 and len(lengths) > 30:
                self.over = True                     # fdeflate: a one-code literal/length code is rejected
        O.Huffman.__init__ = strict_init
    try:
        try:
            out, br = O.inflate(data, 2, lim, window, out)
        except OutputLimit:
            if stop:
                ev.add('R2')
                return bytes(out[:limit])
            if lim == limit:
                fail(limit_error)
            ev.add('R2')
            return bytes(out[:limit])
        except PngError as e:
            if P['stream_end'] == 'AtRaster' and len(out) >= limit and e.name == 'TruncatedZlib':
                if len(out) > limit:
                    ev.add('R2')
                return bytes(out[:limit])
            raise
    finally:
        O.Huffman.__init__ = saved
    br.align()
    if br.byte + 4 > len(data):
        if P['stream_end'] == 'AtRaster' and len(out) >= limit:
            if len(out) > limit:
                ev.add('R2')
            return bytes(out[:limit])
        fail('TruncatedZlib')
    if P['adler'] == 'Check' and be32(data, br.byte) != adler32(out):
        fail('AdlerMismatch')
    if br.byte + 4 != len(data):
        if P['zlib_tail'] == 'Reject' and P['stream_end'] != 'AtRaster':
            fail('TrailingZlibData')
    if len(out) > limit:
        ev.add('R2')
        return bytes(out[:limit])
    return bytes(out)


# ---------------------------------------------------------------- walk with policy (P2-P8, P14.1)
def chunk_at(b, p, P, ev, eof_ok, idat_seen):
    """P3 with `chunk_type`, `reserved_bit`, `eof`. Returns (type, data, next, crc_ok, eof_hit)."""
    if p + 8 > len(b):
        if eof_ok and idat_seen:
            raise Eof
        fail('TruncatedChunk')
    n = be32(b, p)
    t = bytes(b[p + 4:p + 8])
    if n > 0x7FFFFFFF:
        fail('ChunkTooLong')
    letters = all(0x41 <= c <= 0x5A or 0x61 <= c <= 0x7A for c in t)
    if not letters and P['chunk_type'] == 'Reject':
        fail('InvalidChunkType')
    if letters and t[2] & 0x20 and (P['reserved_bit'] == 'Reject' or (P['reserved_bit'] == 'RejectBeforeIdat' and not idat_seen)):
        fail('InvalidChunkType')
    if p + 12 + n > len(b):
        if eof_ok and (idat_seen or t == b'IDAT'):
            return t, bytes(b[p + 8:min(len(b), p + 8 + n)]), len(b), True, True
        fail('TruncatedChunk')
    ok = crc32(b[p + 4:p + 8 + n]) == be32(b, p + 8 + n)
    if not letters:
        t = b'@@@@' if not t[0] & 0x20 else b'````'          # unknown; bit 5 of byte 0 (criticality) kept
    return t, bytes(b[p + 8:p + 8 + n]), p + 12 + n, ok, False


def walk(b, P, ev, entry):
    """P2-P8, P14.1 under policy P. `entry` is 'still' or 'frames'. Under apng_errors=Prefix (frames
    entry, valid acTL), any error after the first IDAT ends the animation after the completed frames."""
    box = {}
    try:
        return _walk(b, P, ev, entry, box)
    except PngError as e:
        doc = box.get('doc')
        if (P['apng_errors'] == 'Prefix' and entry == 'frames' and doc is not None and doc.get('_seen_idat')
                and doc['actl'] is not None):
            raise ApngError(e.name, [f for f in doc['frames'] if f.get('closed')], doc['actl'])
        raise


def _walk(b, P, ev, entry, box):
    if len(b) > MAX_INPUT:
        fail('FileTooLarge')
    if len(b) < 8 or b[:8] != b'\x89PNG\r\n\x1a\n':
        fail('BadSignature')
    t, d, p, ok, _ = chunk_at(b, 8, P, ev, False, False)
    if not ok and P['crc_critical'] == 'Reject':
        fail('CrcMismatch')
    if t != b'IHDR':
        fail('MissingIhdr')
    if len(d) != 13:
        fail('InvalidChunkLength')
    doc = parse_ihdr(d)
    W, H, ct, depth = doc['width'], doc['height'], doc['color_type'], doc['depth']
    doc.update(plte=None, trns=None, idat=bytearray(), meta={}, actl=None, frames=[], iccp=None, eof=False, _P=P)
    box['doc'] = doc
    seen = {b'IHDR'}
    idat = 0
    seq = 0
    cur = None
    fctl_before_idat = False
    pixels = 0
    apng_on = P['apng'] == 'On' and not (entry == 'still' and P['apng_in_still'] == 'Ignore')
    lenient_actl = P['apng_errors'] == 'Prefix' or (P['apng_errors'] == 'Static' and entry == 'frames')
    actl_ignored = False

    def apng_fail(name):
        if P['apng_errors'] in ('Prefix', 'Static') and entry == 'frames':
            raise ApngError(name, [f for f in doc['frames'] if f.get('closed')], doc['actl'])
        fail(name)

    def drop(tt, reason_ok):
        if tt == b'tRNS' and ct in (0, 2, 3):
            ev.add('R1')

    while True:
        eof_ok = P['eof'] == 'GdkPartial'
        if idat == 1 and P['after_idat'] == 'ImageRs' and (entry == 'still' or doc['actl'] is None):
            if p + 8 > len(b):
                fail('TruncatedChunk')
            if b[p + 4:p + 8] != b'IDAT':
                return doc                                        # nothing after the IDAT run is examined
        if p == len(b):
            if idat and (P['missing_iend'] == 'AtEof' or P['eof'] == 'GdkPartial'):
                t, d, ok, p_end = b'IEND', b'', True, p
            else:
                fail('MissingIend')
        else:
            try:
                t, d, p_end, ok, hit = chunk_at(b, p, P, ev, eof_ok, idat > 0)
            except Eof:
                t, d, ok, p_end, hit = b'IEND', b'', True, len(b), True
            if hit:
                doc['eof'] = True
                if t == b'IDAT':
                    doc['idat'] += d
                    idat = 1
                t, d, ok = b'IEND', b'', True
        n = len(d)
        if idat == 1 and t != b'IDAT':
            idat = 2
        prefix = P['apng_errors'] == 'Prefix' and entry == 'frames' and doc['actl'] is not None
        if prefix and cur is not None and not cur.get('closed') and (
                (cur['source'] == 'IDAT' and idat == 2) or (cur['parts_seen'] and t != b'fdAT')):
            cur['closed'] = True
            if sum(1 for f in doc['frames'] if f.get('closed')) == doc['actl']['num_frames']:
                return doc                                         # image-rs stops reading after num_frames frames
        p = p_end
        anc = bool(t[0] & 0x20)
        if (entry == 'still' and P['apng_in_still'] == 'ImageRs' and t == b'fcTL' and idat == 0 and n == 26
                and (be32(d, 12) or be32(d, 16) or be32(d, 4) != W or be32(d, 8) != H)):
            fail('InvalidFrameControl')
        if not apng_on and t in APNG_TYPES:
            t, anc = b'\xff\xff\xff\xff', True                     # treated as an unknown ancillary chunk
        if not ok:
            if t == b'IEND':
                if P['iend'] == 'Strict':
                    fail('CrcMismatch')
            elif anc:
                if P['crc_ancillary'] == 'Reject':
                    fail('CrcMismatch')
                if P['crc_ancillary'] == 'Skip':
                    drop(t, True)
                    continue
            elif P['crc_critical'] == 'Reject':
                fail('CrcMismatch')
        # ---- duplicates and order (P5)
        if t in SINGLE and t in seen:
            if anc and P['ancillary_duplicate'] == 'FirstWins':
                drop(t, True)
                continue
            if t == b'acTL' and lenient_actl:
                continue
            if t == b'PLTE' and P['plte_unexpected'] == 'Ignore' and ct in (0, 4):
                continue
            fail('DuplicateChunk')
        misplaced = ((t in BEFORE_PLTE and (doc['plte'] is not None or idat))
                     or (t in AFTER_PLTE and (idat or (doc['plte'] is None and (ct == 3 or t == b'hIST'))))
                     or (t in BEFORE_IDAT and idat))
        if misplaced:
            if t == b'acTL' and apng_on:
                if not lenient_actl:
                    fail('ChunkOutOfOrder')
                actl_ignored = True
                continue
            if P['ancillary_order'] == 'Drop' or (P['ancillary_order'] == 'ImageRs'
                                                  and not (t == b'bKGD' and ct == 3 and doc['plte'] is None and not idat)):
                drop(t, True)
                continue
            if P['ancillary_order'] == 'ImageRs':
                fail('InvalidBackground')
            fail('ChunkOutOfOrder')
        if t == b'IEND' and n != 0 and P['iend'] == 'Lenient':
            n, d = 0, b''
        if t in FIXED_LEN and n != FIXED_LEN[t]:
            if anc and t != b'acTL' and P['ancillary_values'] in ('Drop', 'ImageRs'):
                continue
            if t == b'acTL' and lenient_actl:
                actl_ignored = True
                continue
            fail('InvalidChunkLength')
        seen.add(t)
        try:
            if t == b'IHDR':
                fail('DuplicateChunk')
            elif t == b'PLTE':
                if idat or any(x in seen for x in AFTER_PLTE):
                    if P['plte_unexpected'] == 'Ignore' and ct != 3:
                        continue
                    fail('ChunkOutOfOrder')
                if ct in (0, 4):
                    if P['plte_unexpected'] == 'Ignore':
                        continue
                    fail('UnexpectedPalette')
                if n == 0 or n % 3 or n > 768:
                    fail('InvalidPalette')
                if ct == 3 and n // 3 > (1 << depth) and P['palette_size'] == 'Reject':
                    fail('InvalidPalette')
                doc['plte'] = d
            elif t == b'IDAT':
                if idat == 2:
                    fail('NonConsecutiveIdat')
                if ct == 3 and doc['plte'] is None:
                    fail('MissingPalette')
                if idat == 0 and cur is not None:
                    cur['source'] = 'IDAT'
                idat = 1
                doc['_seen_idat'] = True
                doc['idat'] += d
            elif t == b'IEND':
                if idat == 0:
                    fail('MissingImageData')
                if cur is not None and (cur['source'] == 'IDAT' or cur['parts_seen']):
                    cur['closed'] = True
                if cur is not None and cur['source'] == 'fdAT' and not cur['parts_seen']:
                    apng_fail('MissingFrameData')
                if doc['actl'] is not None and len(doc['frames']) != doc['actl']['num_frames']:
                    if not (P['apng_errors'] == 'Prefix' and len(doc['frames']) > doc['actl']['num_frames']):
                        apng_fail('FrameCountMismatch')
                    doc['frames'] = doc['frames'][:doc['actl']['num_frames']]
                if not doc['eof'] and p != len(b) and P['trailing'] == 'Reject':
                    fail('TrailingBytes')
                return doc
            elif t == b'tRNS':
                ok_len = {0: 2, 2: 6}.get(ct)
                bad = ct in (4, 6) or (ok_len is not None and n != ok_len) or (ct == 3 and not 1 <= n <= len(doc['plte']) // 3)
                if bad:
                    if P['trns_invalid'] == 'Drop':
                        drop(t, True)
                        continue
                    if P['trns_invalid'] == 'ImageRs':
                        if ok_len is not None and n > ok_len:
                            ev.add('R1')
                            d = d[:ok_len]
                        else:
                            drop(t, True)
                            continue
                    else:
                        fail('InvalidTransparency')
                doc['trns'] = d
            elif t in (b'gAMA', b'cHRM', b'sRGB', b'iCCP', b'sBIT', b'cICP', b'mDCV', b'cLLI', b'bKGD', b'hIST',
                       b'pHYs', b'sPLT', b'eXIf', b'tIME', b'tEXt', b'zTXt', b'iTXt'):
                ancillary_values(t, d, n, doc, ct, depth)
            elif t == b'acTL':
                nf, plays = be32(d, 0), be32(d, 4)
                if nf == 0 or nf > 0x7FFFFFFF or plays > 0x7FFFFFFF:
                    if lenient_actl:
                        actl_ignored = True
                        continue
                    if nf == 0:
                        fail('InvalidAnimationControl')
                if nf > MAX_FRAMES:
                    fail('TooManyFrames')
                doc['actl'] = {'num_frames': nf, 'num_plays': plays}
            elif t == b'fcTL':
                if doc['actl'] is None:
                    if lenient_actl:
                        continue                                    # fcTL without (valid) acTL: ignored
                    fail('MissingAnimationControl')
                if cur is not None and (cur['source'] == 'IDAT' or cur['parts_seen']):
                    cur['closed'] = True
                if idat == 0 and fctl_before_idat:
                    apng_fail('ChunkOutOfOrder')
                if n != 26:
                    apng_fail('InvalidChunkLength')
                if be32(d, 0) != seq:
                    apng_fail('SequenceError')
                seq += 1
                if cur is not None and cur['source'] == 'fdAT' and not cur['parts_seen']:
                    apng_fail('MissingFrameData')
                fw, fh, fx, fy = be32(d, 4), be32(d, 8), be32(d, 12), be32(d, 16)
                dn, dd, dop, bop = be16(d, 20), be16(d, 22), d[24], d[25]
                if fw == 0 or fh == 0 or fx + fw > W or fy + fh > H or dop > 2 or bop > 1:
                    apng_fail('InvalidFrameControl')
                if idat == 0 and (fx or fy or fw != W or fh != H):
                    fail('InvalidFrameControl')                      # image rejects this even in read_info
                pixels += fw * fh
                if pixels > MAX_TOTAL_PIXELS:
                    fail('PixelBudgetExceeded')
                if idat == 0:
                    fctl_before_idat = True
                cur = {'rect': (fx, fy, fw, fh), 'delay': (dn, dd), 'dispose': dop, 'blend': bop,
                       'source': 'fdAT', 'parts': bytearray(), 'parts_seen': False}
                doc['frames'].append(cur)
            elif t == b'fdAT':
                if doc['actl'] is None:
                    if lenient_actl:
                        continue
                    fail('MissingAnimationControl')
                if idat == 0:
                    apng_fail('ChunkOutOfOrder')
                if n < 4:
                    apng_fail('InvalidChunkLength')
                if be32(d, 0) != seq:
                    apng_fail('SequenceError')
                seq += 1
                if cur is None or cur['source'] != 'fdAT':
                    if P['apng_errors'] == 'Prefix' and entry == 'frames' and not doc['frames']:
                        fail('UnexpectedFrameData')
                    apng_fail('UnexpectedFrameData')
                cur['parts'] += d[4:]
                cur['parts_seen'] = True
            elif not anc:
                if idat == 2 and P['after_idat'] == 'ImageRs':
                    continue
                fail('UnknownCriticalChunk')
        except _Dropped:
            drop(t, True)
            continue


class _Dropped(Exception):
    pass


def ancillary_values(t, d, n, doc, ct, depth):
    """P8.1 with `ancillary_values`: a violation drops the chunk (Drop/ImageRs) or is the named error."""
    P = doc['_P']
    soft = P['ancillary_values'] in ('Drop', 'ImageRs')

    def bad(name):
        if soft and not (P['ancillary_values'] == 'ImageRs' and name == 'InvalidCicp'):
            raise _Dropped
        fail(name)

    if t == b'gAMA':
        if be32(d, 0) == 0:
            bad('InvalidGamma')
    elif t == b'sRGB':
        if d[0] > 3:
            bad('InvalidSrgb')
    elif t == b'iCCP':
        z = d.find(b'\0')
        if z < 0 or not O.valid_keyword(d[:z]) or z + 1 >= n or d[z + 1] != 0:
            bad('InvalidIccp')
        doc['iccp'] = d[z + 2:]
    elif t == b'sBIT':
        if n != SBIT_LEN[ct]:
            bad('InvalidChunkLength')
        top = 8 if ct == 3 else depth
        if any(not 1 <= x <= top for x in d):
            bad('InvalidSbit')
    elif t == b'cICP':
        if d[2] != 0 or d[3] > 1:
            bad('InvalidCicp')
    elif t == b'bKGD':
        if n != BKGD_LEN[ct]:
            bad('InvalidChunkLength')
        if ct == 3:
            if d[0] >= len(doc['plte']) // 3:
                bad('InvalidBackground')
        elif any(v >> depth for v in (be16(d, i) for i in range(0, n, 2))):
            bad('InvalidBackground')
    elif t == b'hIST':
        if n != 2 * (len(doc['plte']) // 3):
            bad('InvalidHistogram')
    elif t == b'pHYs':
        if d[8] > 1:
            bad('InvalidPhys')
    elif t == b'sPLT':
        z = d.find(b'\0')
        if z < 0 or not O.valid_keyword(d[:z]) or z + 1 >= n or d[z + 1] not in (8, 16) or (n - z - 2) % (6 if d[z + 1] == 8 else 10):
            bad('InvalidSplt')
    elif t == b'eXIf':
        if n < 4 or d[:4] not in (b'II*\0', b'MM\0*'):
            bad('InvalidExif')
    elif t == b'tIME':
        if not (1 <= d[2] <= 12 and 1 <= d[3] <= 31 and d[4] <= 23 and d[5] <= 59 and d[6] <= 60):
            bad('InvalidTime')
    elif t in (b'tEXt', b'zTXt', b'iTXt'):
        z = d.find(b'\0')
        if z < 0 or not O.valid_keyword(d[:z]):
            bad('InvalidText')
        if t == b'zTXt' and (z + 1 >= n or d[z + 1] != 0):
            bad('InvalidText')
        if t == b'iTXt':
            if z + 2 >= n or d[z + 1] > 1 or (d[z + 1] == 1 and d[z + 2] != 0):
                bad('InvalidText')
            z2 = d.find(b'\0', z + 3)
            if z2 < 0 or d.find(b'\0', z2 + 1) < 0:
                bad('InvalidText')


# ---------------------------------------------------------------- pixels with policy (P11-P13)
def to8(v, depth, P):
    if depth == 16:
        return v >> 8 if P['depth16'] == 'HighByte' else (v * 255 * 2 + 65535) // (2 * 65535)
    return v * 255 // ((1 << depth) - 1)


def pixel_row(doc, line, w, P, ev):
    ct, depth = doc['color_type'], doc['depth']
    s = O.samples(line, depth, w * O.CHANNELS[ct])
    cv = lambda v: to8(v, depth, P)  # noqa: E731
    trns = doc['trns']
    out = []
    if ct == 3:
        pal = doc['plte']
        ent = len(pal) // 3
        for i in s:
            if i >= ent:
                if P['palette_index'] == 'Black':
                    ev.add('R3')
                    out.append((0, 0, 0, 255))
                    continue
                fail('PaletteIndexOutOfRange')
            a = trns[i] if trns is not None and i < len(trns) else 255
            out.append(tuple(pal[3 * i:3 * i + 3]) + (a,))
        return out
    key = None
    if trns is not None:
        mask = (1 << depth) - 1
        key = tuple(be16(trns, j) & mask for j in range(0, len(trns), 2))
    for x in range(w):
        if ct == 0:
            g = s[x]
            out.append((cv(g),) * 3 + (0 if key == (g,) else 255,))
        elif ct == 2:
            rgb = tuple(s[3 * x:3 * x + 3])
            out.append(tuple(cv(v) for v in rgb) + (0 if key == rgb else 255,))
        elif ct == 4:
            out.append((cv(s[2 * x]),) * 3 + (cv(s[2 * x + 1]),))
        else:
            out.append(tuple(cv(v) for v in s[4 * x:4 * x + 4]))
    return out


GREY_ERRORS = {'InvalidZlibHeader', 'UnsupportedZlibMethod', 'InvalidBlockType', 'StoredLengthMismatch',
               'InvalidCodeLengths', 'InvalidSymbol', 'DistanceTooFar', 'AdlerMismatch', 'TruncatedImageData'}


def decode_image(doc, data, w, h, P, ev, eof=False):
    """P11-P13 with `image_data_error` and `eof` (GdkGrey/GdkPartial: rows completed before the error
    are kept, the rest is GREY, event R5; a surrogate fitted to the gdk-pixbuf loader probes)."""
    R = O.raw_size(doc, w, h)
    partial = bytearray()
    lost_last = False
    try:
        raw = zlib_decompress(data, R, 'ExcessImageData', P, ev, partial)
        if len(raw) < R:
            fail('TruncatedImageData')
    except PngError as e:
        grey_ok = P['image_data_error'] == 'GdkGrey' and (e.name in GREY_ERRORS or (eof and e.name == 'TruncatedZlib'))
        if not grey_ok:
            raise
        raw = bytes(partial[:R])
        lost_last = e.name not in ('TruncatedImageData', 'TruncatedZlib')
    bpp = max(1, O.bits_pp(doc) // 8)
    px = [None] * (w * h)
    pos = 0
    rows = [(pa, r) for pa in O.passes(doc, w, h) for r in range(pa[5])]
    complete = []
    for x0, y0, dx, dy, pw, ph in O.passes(doc, w, h):
        rb = O.row_bytes(doc, pw)
        prev = bytes(rb)
        for r in range(ph):
            if pos + 1 + rb > len(raw):
                break
            ft = raw[pos]
            if ft > 4:
                fail('InvalidFilterType')
            line = O.unfilter_row(ft, raw[pos + 1:pos + 1 + rb], prev, bpp)
            pos += 1 + rb
            prev = line
            complete.append([((y0 + r * dy) * w + x0 + c * dx, p) for c, p in enumerate(pixel_row(doc, line, pw, P, ev))])
    if lost_last and complete:
        complete.pop()
    for row in complete:
        for i, p in row:
            px[i] = p
    if any(p is None for p in px):
        ev.add('R5')
        px = [GREY if p is None else p for p in px]
    del rows
    return px


def flat8(px):
    return bytes(v for p in px for v in p)


# ---------------------------------------------------------------- entry points
def decode_still(b, P):
    ev = set()
    try:
        doc = walk_entry(b, P, ev, 'still')
        px = decode_image(doc, bytes(doc['idat']), doc['width'], doc['height'], P, ev, doc['eof'])
    except PngError as e:
        return {'error': e.name}
    return {'width': doc['width'], 'height': doc['height'], 'rgba': flat8(px), 'recovered': bool(ev), 'events': sorted(ev)}


def walk_entry(b, P, ev, entry):
    return walk(b, P, ev, entry)


def f32(x):
    return struct.unpack('f', struct.pack('f', x))[0]


def blend_f32(bg, fg):
    """image 0.25.10 Rgba::blend (color.rs:868-925) in f32 with truncating casts."""
    if fg[3] == 0:
        return bg
    if fg[3] == 255:
        return fg
    m = f32(255.0)
    br, bgc, bb, ba = (f32(v / m) for v in bg)
    fr, fgc, fb, fa = (f32(v / m) for v in fg)
    af = f32(f32(ba + fa) - f32(ba * fa))
    if af == 0.0:
        return bg
    one = f32(1.0 - fa)
    outs = []
    for fc, bc in ((fr, br), (fgc, bgc), (fb, bb)):
        o = f32(f32(fc * fa) + f32(f32(bc * ba) * one))
        outs.append(int(f32(m * f32(o / af))))
    return tuple(outs) + (int(f32(m * af)),)


def decode_frames(b, P):
    ev = set()
    try:
        try:
            doc = walk_entry(b, P, ev, 'frames')
            tail = None
        except ApngError as e:
            if P['apng_errors'] == 'Static':
                Q = dict(P, apng='Off')
                doc = walk_entry(b, Q, ev, 'frames')
                doc['frames'] = []
                ev.add('R4')
                tail = None
            else:
                ev2 = set()
                doc = walk_entry(b, dict(P, apng='Off'), ev2, 'frames')
                doc['frames'] = list(e.frames_complete)
                doc['actl'] = e.actl or {'num_frames': len(doc['frames']), 'num_plays': 0}
                tail = e.name
                if not doc['frames']:
                    fail(e.name)
                # frames recorded by the failed walk (data collected up to the failing chunk)
        W, H = doc['width'], doc['height']
        default = decode_image(doc, bytes(doc['idat']), W, H, P, ev, doc['eof'])
    except PngError as e:
        return {'error': e.name}
    base_ev = set(ev)
    res = {'width': W, 'height': H, 'num_plays': None, 'frames': [], 'end': 'iend'}
    if doc['actl'] is None or not doc['frames'] and tail is None:
        res['frames'].append({'rgba': flat8(default), 'rect': [0, 0, W, H], 'delay': None, 'dispose': 0, 'blend': 0,
                              'recovered': bool(base_ev), 'events': sorted(base_ev)})
        return res
    if P['apng16'] == 'Reject' and doc['depth'] == 16:
        return {'error': 'InvalidColorTypeDepth'}
    res['num_plays'] = doc['actl']['num_plays']
    canvas = [(0, 0, 0, 0)] * (W * H)
    prev, saved = None, None
    img_prev = list(canvas)                      # image-rs `previous` buffer (dispose_previous=ImageRs)
    for k, f in enumerate(doc['frames']):
        fev = set(base_ev)
        fx, fy, fw, fh = f['rect']
        try:
            fpx = default if f['source'] == 'IDAT' else decode_image(doc, bytes(f['parts']), fw, fh, P, fev)
        except PngError as e:
            res['end'] = {'error': e.name}
            return res
        if P['dispose_previous'] == 'ImageRs':
            if prev is not None:
                (px0, py0, pw, ph), op = prev
                if op in (0, 1):
                    img_prev = list(canvas)
                if op == 1:
                    for y in range(py0, py0 + ph):
                        for x in range(px0, px0 + pw):
                            canvas[y * W + x] = (0, 0, 0, 0)
                elif op == 2:
                    for y in range(py0, py0 + ph):
                        for x in range(px0, px0 + pw):
                            canvas[y * W + x] = img_prev[y * W + x]
            else:
                img_prev = list(canvas)
            op = f['dispose']                         # image-rs keeps PREVIOUS; its `previous` starts transparent
        else:
            if prev is not None:
                (px0, py0, pw, ph), op = prev
                if op == 1:
                    for y in range(py0, py0 + ph):
                        for x in range(px0, px0 + pw):
                            canvas[y * W + x] = (0, 0, 0, 0)
                elif op == 2:
                    canvas = saved
            op = 1 if (k == 0 and f['dispose'] == 2) else f['dispose']
            if op == 2:
                saved = list(canvas)
        for y in range(fh):
            for x in range(fw):
                s = fpx[y * fw + x]
                o = (fy + y) * W + fx + x
                if f['blend'] == 0:
                    canvas[o] = s
                else:
                    canvas[o] = blend_f32(canvas[o], s) if P['blend'] == 'ImageF32' else O.over(s, canvas[o])
        prev = (f['rect'], op)
        res['frames'].append({'rgba': flat8(canvas), 'rect': list(f['rect']), 'delay': list(f['delay']),
                              'dispose': f['dispose'], 'blend': f['blend'], 'recovered': bool(fev), 'events': sorted(fev)})
    if tail:
        res['end'] = {'error': tail}
    return res
