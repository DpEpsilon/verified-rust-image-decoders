#!/usr/bin/env python3
"""Strict reference decoder for PNG-CORE-SPEC draft 0.1 (SPEC.md, sections P1-P21).

Every function names the SPEC section it implements. The zlib/DEFLATE layer (P9, P10),
CRC-32 (P3) and Adler-32 (P9) are written here by hand; Python's zlib is never imported
by this file (check_vectors.py cross-checks against it).

Entry points (P1):
  decode_still(b)          -> {'width','height','rgba': bytes, 'info': {...}} | {'error'}
  decode_still16(b)        -> as decode_still with 'rgba16': list of 16-bit samples
  decode_frames(b)         -> {'width','height','num_plays','frames': [...], 'end'} | {'error'}
  icc_profile(b)           -> {'icc': bytes|None} | {'error'}
"""

# ------------------------------------------------------------------ P1 limits
MAX_INPUT = 256 * 1024 * 1024          # bytes of input
MAX_DIMENSION = 16384                  # pixels per side
MAX_OUTPUT = 64 * 1024 * 1024          # bytes of one RGBA8 canvas, 4*W*H
MAX_FRAMES = 65536                     # acTL num_frames
MAX_TOTAL_PIXELS = 1 << 30             # sum of w*h over the fcTL frames of one stream
MAX_ICC = 16 * 1024 * 1024             # bytes of a decompressed ICC profile

ERRORS = [
    'FileTooLarge', 'BadSignature', 'TruncatedChunk', 'ChunkTooLong', 'InvalidChunkType', 'CrcMismatch',
    'MissingIhdr', 'InvalidChunkLength', 'InvalidDimensions', 'OutputTooLarge', 'InvalidColorTypeDepth',
    'UnsupportedCompressionMethod', 'UnsupportedFilterMethod', 'UnsupportedInterlaceMethod',
    'UnknownCriticalChunk', 'DuplicateChunk', 'ChunkOutOfOrder', 'MissingPalette', 'UnexpectedPalette',
    'InvalidPalette', 'NonConsecutiveIdat', 'MissingImageData', 'MissingIend', 'TrailingBytes',
    'InvalidTransparency', 'InvalidGamma', 'InvalidSrgb', 'InvalidIccp', 'InvalidSbit', 'InvalidCicp',
    'InvalidBackground', 'InvalidHistogram', 'InvalidPhys', 'InvalidSplt', 'InvalidExif', 'InvalidTime',
    'InvalidText', 'MissingAnimationControl', 'InvalidAnimationControl', 'TooManyFrames',
    'InvalidFrameControl', 'SequenceError', 'UnexpectedFrameData', 'MissingFrameData', 'FrameCountMismatch',
    'PixelBudgetExceeded', 'InvalidZlibHeader', 'UnsupportedZlibMethod', 'ZlibPresetDictionary',
    'InvalidBlockType', 'StoredLengthMismatch', 'InvalidCodeLengths', 'InvalidSymbol', 'DistanceTooFar',
    'TruncatedZlib', 'AdlerMismatch', 'TrailingZlibData', 'TruncatedImageData', 'ExcessImageData',
    'InvalidFilterType', 'PaletteIndexOutOfRange', 'IccProfileTooLarge',
]


class PngError(Exception):
    def __init__(self, name):
        assert name in ERRORS, name
        super().__init__(name)
        self.name = name


def fail(name):
    raise PngError(name)


def be32(b, p):
    return (b[p] << 24) | (b[p + 1] << 16) | (b[p + 2] << 8) | b[p + 3]


def be16(b, p):
    return (b[p] << 8) | b[p + 1]


# ------------------------------------------------------------------ P3 CRC-32 (ISO 3309 / ITU-T V.42)
def _crc_table():
    t = []
    for n in range(256):
        c = n
        for _ in range(8):
            c = (0xEDB88320 ^ (c >> 1)) if c & 1 else (c >> 1)
        t.append(c)
    return t


CRC_TABLE = _crc_table()


def crc32(data):
    """P3: CRC over chunk type and data; register preset to all ones, result complemented."""
    c = 0xFFFFFFFF
    for x in data:
        c = CRC_TABLE[(c ^ x) & 0xFF] ^ (c >> 8)
    return c ^ 0xFFFFFFFF


# ------------------------------------------------------------------ P9 Adler-32 (RFC 1950 s.8.2)
def adler32(data):
    a, s = 1, 0
    for i in range(0, len(data), 5552):
        for x in data[i:i + 5552]:
            a += x
            s += a
        a %= 65521
        s %= 65521
    return (s << 16) | a


# ------------------------------------------------------------------ P10 DEFLATE (RFC 1951)
class OutputLimit(Exception):
    """P10.6: the stream would produce more than `limit` bytes; the caller names the error."""


class Bits:
    """P10.1: LSB-first bit reader over a byte string (the concatenated zlib payload)."""

    def __init__(self, data, pos):
        self.d, self.byte, self.bit = data, pos, 0

    def get(self, n):
        v = 0
        for i in range(n):
            if self.byte >= len(self.d):
                fail('TruncatedZlib')
            v |= ((self.d[self.byte] >> self.bit) & 1) << i
            self.bit += 1
            if self.bit == 8:
                self.bit, self.byte = 0, self.byte + 1
        return v

    def align(self):
        if self.bit:
            self.bit, self.byte = 0, self.byte + 1


class Huffman:
    """P10.3: canonical code from code lengths (RFC 1951 s.3.2.2) as count[len] and symbols
    sorted by (length, symbol). `complete`: every bit string of the maximal length is decodable."""

    def __init__(self, lengths):
        self.count = [0] * 16
        for n in lengths:
            self.count[n] += 1
        self.count[0] = 0
        left = 1
        self.over = False
        for n in range(1, 16):
            left = (left << 1) - self.count[n]
            if left < 0:
                self.over = True
        self.left = left            # unused codes at length 15 scale; 0 iff complete
        self.nsyms = sum(self.count)
        self.single_len1 = self.nsyms == 1 and self.count[1] == 1
        offs = [0] * 16
        for n in range(1, 15):
            offs[n + 1] = offs[n] + self.count[n]
        self.symbol = [0] * max(1, self.nsyms)
        for s, n in enumerate(lengths):
            if n:
                self.symbol[offs[n]] = s
                offs[n] += 1

    def decode(self, br):
        """Read one code; a bit string that matches no code is InvalidSymbol (only possible for
        the incomplete codes P10.3 admits)."""
        code = first = index = 0
        for n in range(1, 16):
            code |= br.get(1)
            cnt = self.count[n]
            if code - cnt < first:
                return self.symbol[index + (code - first)]
            index += cnt
            first = (first + cnt) << 1
            code <<= 1
        fail('InvalidSymbol')


LEN_BASE = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115,
            131, 163, 195, 227, 258]
LEN_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0]
DIST_BASE = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
             2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577]
DIST_EXTRA = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13]
CL_ORDER = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]

FIXED_LIT = Huffman([8] * 144 + [9] * 112 + [7] * 24 + [8] * 8)
FIXED_DIST = Huffman([5] * 30)   # codes 30, 31 exist in the fixed code but are invalid (P10.4)


def acceptable(h, kind):
    """P10.3: which codes a dynamic header may define. 'cl': complete. 'lit'/'dist': complete, or
    exactly one code of length 1; 'dist' may also be empty (no distance codes)."""
    if h.over:
        return False
    if h.left == 0:
        return True
    if kind == 'cl':
        return False
    if h.nsyms == 0:
        return kind == 'dist'
    return h.single_len1


def inflate(data, pos, limit, window, out=None):
    """P10: decode DEFLATE blocks starting at byte `pos` of `data`. Returns (output, bit reader).
    Raises PngError for format errors, OutputLimit when byte limit+1 would be produced.
    `window`: largest admissible distance (P9: 2**(CINFO+8))."""
    br = Bits(data, pos)
    out = bytearray() if out is None else out    # a caller-supplied buffer keeps partial output on error
    while True:
        final = br.get(1)
        btype = br.get(2)
        if btype == 0:                                    # P10.2 stored
            br.align()
            if br.byte + 4 > len(data):
                fail('TruncatedZlib')
            ln = data[br.byte] | (data[br.byte + 1] << 8)
            nln = data[br.byte + 2] | (data[br.byte + 3] << 8)
            if ln != (~nln & 0xFFFF):
                fail('StoredLengthMismatch')
            br.byte += 4
            # bytes are produced one at a time in stream order: the limit, then a truncation
            take = min(ln, len(data) - br.byte)
            if len(out) + take > limit:
                out += data[br.byte:br.byte + limit - len(out)]
                raise OutputLimit
            if take < ln:
                fail('TruncatedZlib')
            out += data[br.byte:br.byte + ln]
            br.byte += ln
        elif btype == 3:
            fail('InvalidBlockType')
        else:
            if btype == 1:                                # P10.4 fixed
                lit, dist = FIXED_LIT, FIXED_DIST
            else:                                         # P10.5 dynamic
                lit, dist = dynamic_tables(br)
            while True:
                sym = lit.decode(br)
                if sym < 256:
                    if len(out) + 1 > limit:
                        raise OutputLimit
                    out.append(sym)
                elif sym == 256:
                    break
                else:
                    sym -= 257
                    if sym >= 29:
                        fail('InvalidSymbol')
                    length = LEN_BASE[sym] + br.get(LEN_EXTRA[sym])
                    ds = dist.decode(br)
                    if ds >= 30:
                        fail('InvalidSymbol')
                    d = DIST_BASE[ds] + br.get(DIST_EXTRA[ds])
                    if d > len(out) or d > window:
                        fail('DistanceTooFar')
                    if len(out) + length > limit:
                        length = limit - len(out)              # copy what fits, then report the limit
                        start = len(out) - d
                        for i in range(length):
                            out.append(out[start + i])
                        raise OutputLimit
                    start = len(out) - d
                    if d >= length:
                        out += out[start:start + length]
                    else:
                        for i in range(length):
                            out.append(out[start + i])
        if final:
            return out, br


def dynamic_tables(br):
    """P10.5: HLIT, HDIST, HCLEN, the code-length code, then literal/length and distance lengths."""
    nlen = br.get(5) + 257
    ndist = br.get(5) + 1
    ncode = br.get(4) + 4
    if nlen > 286 or ndist > 30:
        fail('InvalidCodeLengths')
    cl = [0] * 19
    for i in range(ncode):
        cl[CL_ORDER[i]] = br.get(3)
    clh = Huffman(cl)
    if not acceptable(clh, 'cl'):
        fail('InvalidCodeLengths')
    lengths = []
    while len(lengths) < nlen + ndist:
        sym = clh.decode(br)
        if sym < 16:
            lengths.append(sym)
            continue
        if sym == 16:
            if not lengths:
                fail('InvalidCodeLengths')
            val, rep = lengths[-1], 3 + br.get(2)
        elif sym == 17:
            val, rep = 0, 3 + br.get(3)
        else:
            val, rep = 0, 11 + br.get(7)
        if len(lengths) + rep > nlen + ndist:
            fail('InvalidCodeLengths')
        lengths += [val] * rep
    litl, distl = lengths[:nlen], lengths[nlen:]
    if litl[256] == 0:
        fail('InvalidCodeLengths')
    lit, dist = Huffman(litl), Huffman(distl)
    if not acceptable(lit, 'lit') or not acceptable(dist, 'dist'):
        fail('InvalidCodeLengths')
    return lit, dist


# ------------------------------------------------------------------ P9 zlib (RFC 1950)
def zlib_decompress(data, limit, limit_error):
    """P9: header, DEFLATE body (P10), Adler-32, nothing after it. `data` is the concatenated
    payload; output above `limit` bytes is `limit_error`."""
    if len(data) < 2:
        fail('TruncatedZlib')
    cmf, flg = data[0], data[1]
    if (cmf * 256 + flg) % 31:
        fail('InvalidZlibHeader')
    if cmf & 15 != 8 or cmf >> 4 > 7:
        fail('UnsupportedZlibMethod')
    if flg & 0x20:
        fail('ZlibPresetDictionary')
    try:
        out, br = inflate(data, 2, limit, 1 << ((cmf >> 4) + 8))
    except OutputLimit:
        fail(limit_error)
    br.align()
    if br.byte + 4 > len(data):
        fail('TruncatedZlib')
    if be32(data, br.byte) != adler32(out):
        fail('AdlerMismatch')
    if br.byte + 4 != len(data):
        fail('TrailingZlibData')
    return bytes(out)


# ------------------------------------------------------------------ P4 IHDR
DEPTHS = {0: (1, 2, 4, 8, 16), 2: (8, 16), 3: (1, 2, 4, 8), 4: (8, 16), 6: (8, 16)}
CHANNELS = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}


def parse_ihdr(d):
    w, h = be32(d, 0), be32(d, 4)
    depth, ct, cm, fm, im = d[8], d[9], d[10], d[11], d[12]
    if not (1 <= w <= MAX_DIMENSION and 1 <= h <= MAX_DIMENSION):
        fail('InvalidDimensions')
    if 4 * w * h > MAX_OUTPUT:
        fail('OutputTooLarge')
    if ct not in DEPTHS or depth not in DEPTHS[ct]:
        fail('InvalidColorTypeDepth')
    if cm != 0:
        fail('UnsupportedCompressionMethod')
    if fm != 0:
        fail('UnsupportedFilterMethod')
    if im > 1:
        fail('UnsupportedInterlaceMethod')
    return {'width': w, 'height': h, 'depth': depth, 'color_type': ct, 'interlace': im}


# ------------------------------------------------------------------ P8 text helpers
def valid_keyword(k):
    """P8.2: 1-79 bytes of 0x20-0x7E or 0xA1-0xFF; no leading, trailing or consecutive spaces."""
    if not 1 <= len(k) <= 79:
        return False
    if any(not (0x20 <= c <= 0x7E or 0xA1 <= c <= 0xFF) for c in k):
        return False
    return k[0] != 0x20 and k[-1] != 0x20 and b'  ' not in k


def split_keyword(d, err):
    z = d.find(b'\0')
    if z < 0 or not valid_keyword(d[:z]):
        fail(err)
    return z


# ------------------------------------------------------------------ P3, P5-P8 chunk walk
BEFORE_PLTE = {b'cHRM', b'gAMA', b'iCCP', b'sBIT', b'sRGB', b'cICP', b'mDCV', b'cLLI'}
AFTER_PLTE = {b'bKGD', b'hIST', b'tRNS'}
BEFORE_IDAT = {b'eXIf', b'pHYs', b'sPLT', b'acTL'}
ANYWHERE = {b'tIME', b'tEXt', b'zTXt', b'iTXt'}
SINGLE = BEFORE_PLTE | AFTER_PLTE | {b'eXIf', b'pHYs', b'acTL', b'tIME', b'PLTE', b'IHDR'}
FIXED_LEN = {b'gAMA': 4, b'cHRM': 32, b'sRGB': 1, b'cICP': 4, b'mDCV': 24, b'cLLI': 8, b'pHYs': 9,
             b'tIME': 7, b'acTL': 8, b'IEND': 0}           # fcTL (26) and fdAT (>= 4): P14.1
SBIT_LEN = {0: 1, 2: 3, 3: 3, 4: 2, 6: 4}
BKGD_LEN = {0: 2, 4: 2, 2: 6, 6: 6, 3: 1}


def chunk_at(b, p):
    """P3: one chunk at offset p. Returns (type, data_start, data_len, next_offset)."""
    if p + 8 > len(b):
        fail('TruncatedChunk')
    n = be32(b, p)
    t = bytes(b[p + 4:p + 8])
    if n > 0x7FFFFFFF:
        fail('ChunkTooLong')
    if any(not (0x41 <= c <= 0x5A or 0x61 <= c <= 0x7A) for c in t):
        fail('InvalidChunkType')
    if p + 12 + n > len(b):
        fail('TruncatedChunk')
    if crc32(b[p + 4:p + 8 + n]) != be32(b, p + 8 + n):
        fail('CrcMismatch')
    return t, p + 8, n, p + 12 + n


def walk(b):
    """P2-P8 and P14.1: the structural pass over the whole datastream. No pixel is decoded and
    no zlib stream is inflated. Returns the document record used by P11-P17."""
    if len(b) > MAX_INPUT:
        fail('FileTooLarge')
    if len(b) < 8 or b[:8] != b'\x89PNG\r\n\x1a\n':
        fail('BadSignature')
    p = 8
    t, ds, n, p = chunk_at(b, p)
    if t != b'IHDR':
        fail('MissingIhdr')
    if n != 13:
        fail('InvalidChunkLength')
    doc = parse_ihdr(b[ds:ds + 13])
    W, H, ct, depth = doc['width'], doc['height'], doc['color_type'], doc['depth']
    doc.update(plte=None, trns=None, idat=bytearray(), meta={}, actl=None, frames=[], iccp=None,
               default_is_frame=False)
    seen = {b'IHDR'}
    idat = 0                      # 0 before the first IDAT, 1 in the IDAT run, 2 after it
    seq = 0                       # P14.1 next expected sequence number
    cur = None                    # P14.1 frame whose data is being collected
    fctl_before_idat = False
    pixels = 0
    while True:
        if p == len(b):
            fail('MissingIend')
        t, ds, n, p = chunk_at(b, p)
        d = bytes(b[ds:ds + n])
        if idat == 1 and t != b'IDAT':
            idat = 2
        if t in SINGLE and t in seen:
            fail('DuplicateChunk')
        # ---- P5 ordering
        if t in BEFORE_PLTE and (doc['plte'] is not None or idat):
            fail('ChunkOutOfOrder')
        if t in AFTER_PLTE and (idat or (doc['plte'] is None and (ct == 3 or t == b'hIST'))):
            fail('ChunkOutOfOrder')
        if t in BEFORE_IDAT and idat:
            fail('ChunkOutOfOrder')
        if t in FIXED_LEN and n != FIXED_LEN[t]:
            fail('InvalidChunkLength')
        seen.add(t)
        if t == b'IHDR':
            fail('DuplicateChunk')
        elif t == b'PLTE':                                   # P6
            if idat:
                fail('ChunkOutOfOrder')
            if seen & AFTER_PLTE:
                fail('ChunkOutOfOrder')
            if ct in (0, 4):
                fail('UnexpectedPalette')
            if n == 0 or n % 3 or n > 768 or (ct == 3 and n // 3 > (1 << depth)):
                fail('InvalidPalette')
            doc['plte'] = d
        elif t == b'IDAT':                                   # P5, P11
            if idat == 2:
                fail('NonConsecutiveIdat')
            if ct == 3 and doc['plte'] is None:
                fail('MissingPalette')
            if idat == 0 and cur is not None:
                cur['source'] = 'IDAT'
                doc['default_is_frame'] = True
            idat = 1
            doc['idat'] += d
        elif t == b'IEND':
            if idat == 0:
                fail('MissingImageData')
            if cur is not None and cur['source'] == 'fdAT' and not cur['parts']:
                fail('MissingFrameData')
            if doc['actl'] is not None and len(doc['frames']) != doc['actl']['num_frames']:
                fail('FrameCountMismatch')
            if p != len(b):
                fail('TrailingBytes')
            return doc
        elif t == b'tRNS':                                   # P7
            if ct in (4, 6):
                fail('InvalidTransparency')
            if ct == 0 and n != 2 or ct == 2 and n != 6 or ct == 3 and not 1 <= n <= len(doc['plte']) // 3:
                fail('InvalidTransparency')
            doc['trns'] = d
        elif t == b'gAMA':
            if be32(d, 0) == 0:
                fail('InvalidGamma')
            doc['meta']['gAMA'] = be32(d, 0)
        elif t == b'cHRM':
            doc['meta']['cHRM'] = [be32(d, 4 * i) for i in range(8)]
        elif t == b'sRGB':
            if d[0] > 3:
                fail('InvalidSrgb')
            doc['meta']['sRGB'] = d[0]
        elif t == b'iCCP':
            z = split_keyword(d, 'InvalidIccp')
            if z + 1 >= n or d[z + 1] != 0:
                fail('InvalidIccp')
            doc['iccp'] = d[z + 2:]
            doc['meta']['iCCP'] = d[:z].decode('latin-1')
        elif t == b'sBIT':
            if n != SBIT_LEN[ct]:
                fail('InvalidChunkLength')
            top = 8 if ct == 3 else depth
            if any(not 1 <= x <= top for x in d):
                fail('InvalidSbit')
            doc['meta']['sBIT'] = list(d)
        elif t == b'cICP':
            if d[2] != 0 or d[3] > 1:
                fail('InvalidCicp')
            doc['meta']['cICP'] = list(d)
        elif t == b'mDCV':
            doc['meta']['mDCV'] = d.hex()
        elif t == b'cLLI':
            doc['meta']['cLLI'] = [be32(d, 0), be32(d, 4)]
        elif t == b'bKGD':
            if n != BKGD_LEN[ct]:
                fail('InvalidChunkLength')
            if ct == 3:
                if d[0] >= len(doc['plte']) // 3:
                    fail('InvalidBackground')
                doc['meta']['bKGD'] = [d[0]]
            else:
                vals = [be16(d, i) for i in range(0, n, 2)]
                if any(v >> depth for v in vals):
                    fail('InvalidBackground')
                doc['meta']['bKGD'] = vals
        elif t == b'hIST':
            if n != 2 * (len(doc['plte']) // 3):
                fail('InvalidHistogram')
        elif t == b'pHYs':
            if d[8] > 1:
                fail('InvalidPhys')
            doc['meta']['pHYs'] = [be32(d, 0), be32(d, 4), d[8]]
        elif t == b'sPLT':
            z = split_keyword(d, 'InvalidSplt')
            if z + 1 >= n or d[z + 1] not in (8, 16):
                fail('InvalidSplt')
            if (n - z - 2) % (6 if d[z + 1] == 8 else 10):
                fail('InvalidSplt')
        elif t == b'eXIf':
            if n < 4 or d[:4] not in (b'II*\0', b'MM\0*'):
                fail('InvalidExif')
            doc['meta']['eXIf'] = n
        elif t == b'tIME':
            y, mo, dd, hh, mi, ss = be16(d, 0), d[2], d[3], d[4], d[5], d[6]
            if not (1 <= mo <= 12 and 1 <= dd <= 31 and hh <= 23 and mi <= 59 and ss <= 60):
                fail('InvalidTime')
            doc['meta']['tIME'] = [y, mo, dd, hh, mi, ss]
        elif t == b'tEXt':
            split_keyword(d, 'InvalidText')
        elif t == b'zTXt':
            z = split_keyword(d, 'InvalidText')
            if z + 1 >= n or d[z + 1] != 0:
                fail('InvalidText')
        elif t == b'iTXt':
            z = split_keyword(d, 'InvalidText')
            if z + 2 >= n or d[z + 1] > 1 or (d[z + 1] == 1 and d[z + 2] != 0):
                fail('InvalidText')
            z2 = d.find(b'\0', z + 3)
            if z2 < 0 or d.find(b'\0', z2 + 1) < 0:
                fail('InvalidText')
        elif t == b'acTL':                                   # P14.1
            nf, plays = be32(d, 0), be32(d, 4)
            if nf == 0:
                fail('InvalidAnimationControl')
            if nf > MAX_FRAMES:
                fail('TooManyFrames')
            doc['actl'] = {'num_frames': nf, 'num_plays': plays}
        elif t == b'fcTL':
            if doc['actl'] is None:
                fail('MissingAnimationControl')
            if idat == 0 and fctl_before_idat:
                fail('ChunkOutOfOrder')
            if n != 26:
                fail('InvalidChunkLength')
            if be32(d, 0) != seq:
                fail('SequenceError')
            seq += 1
            if cur is not None and cur['source'] == 'fdAT' and not cur['parts']:
                fail('MissingFrameData')
            fw, fh, fx, fy = be32(d, 4), be32(d, 8), be32(d, 12), be32(d, 16)
            dn, dd, dop, bop = be16(d, 20), be16(d, 22), d[24], d[25]
            if fw == 0 or fh == 0 or fx + fw > W or fy + fh > H or dop > 2 or bop > 1:
                fail('InvalidFrameControl')
            if idat == 0 and (fx or fy or fw != W or fh != H):
                fail('InvalidFrameControl')
            pixels += fw * fh
            if pixels > MAX_TOTAL_PIXELS:
                fail('PixelBudgetExceeded')
            if idat == 0:
                fctl_before_idat = True
            cur = {'rect': (fx, fy, fw, fh), 'delay': (dn, dd), 'dispose': dop, 'blend': bop,
                   'source': 'fdAT', 'parts': bytearray()}
            doc['frames'].append(cur)
        elif t == b'fdAT':
            if doc['actl'] is None:
                fail('MissingAnimationControl')
            if idat == 0:
                fail('ChunkOutOfOrder')
            if n < 4:
                fail('InvalidChunkLength')
            if be32(d, 0) != seq:
                fail('SequenceError')
            seq += 1
            if cur is None or cur['source'] != 'fdAT':
                fail('UnexpectedFrameData')
            cur['parts'] += d[4:]
            cur['nonempty'] = True
        elif not t[0] & 0x20:
            fail('UnknownCriticalChunk')
        # unknown ancillary chunk: skipped (P5)


# ------------------------------------------------------------------ P11 scanlines and filters
def bits_pp(doc):
    return CHANNELS[doc['color_type']] * doc['depth']


def row_bytes(doc, w):
    return (w * bits_pp(doc) + 7) // 8


ADAM7 = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)]


def passes(doc, w, h):
    """P12: the (x0, y0, dx, dy, pass_w, pass_h) list; one full-size pass without interlace.
    Empty passes (pass_w = 0 or pass_h = 0) are omitted and contribute no bytes."""
    if not doc['interlace']:
        return [(0, 0, 1, 1, w, h)]
    out = []
    for x0, y0, dx, dy in ADAM7:
        pw, ph = (w - x0 + dx - 1) // dx if w > x0 else 0, (h - y0 + dy - 1) // dy if h > y0 else 0
        if pw and ph:
            out.append((x0, y0, dx, dy, pw, ph))
    return out


def raw_size(doc, w, h):
    """P11: R = sum over non-empty passes of pass_h * (1 + row bytes of pass_w)."""
    return sum(ph * (1 + row_bytes(doc, pw)) for _, _, _, _, pw, ph in passes(doc, w, h))


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    return b if pb <= pc else c


def unfilter_row(ftype, line, prev, bpp):
    """P11.2: reconstruct one scanline (bytes after the filter byte) given the previous
    reconstructed scanline of the same pass (all zeros for the first)."""
    out = bytearray(len(line))
    for i, x in enumerate(line):
        a = out[i - bpp] if i >= bpp else 0
        bb = prev[i]
        c = prev[i - bpp] if i >= bpp else 0
        if ftype == 0:
            v = x
        elif ftype == 1:
            v = x + a
        elif ftype == 2:
            v = x + bb
        elif ftype == 3:
            v = x + ((a + bb) >> 1)
        else:
            v = x + paeth(a, bb, c)
        out[i] = v & 0xFF
    return out


# ------------------------------------------------------------------ P13 samples to RGBA
def samples(line, depth, count):
    """P13.1: `count` samples of `depth` bits, most significant first, big-endian for 16."""
    if depth == 16:
        return [be16(line, 2 * i) for i in range(count)]
    if depth == 8:
        return list(line[:count])
    per = 8 // depth
    mask = (1 << depth) - 1
    return [(line[i // per] >> (8 - depth * (i % per + 1))) & mask for i in range(count)]


def to8(v, depth):
    """P13.2: exact rescaling to 8 bits: v*255/(2^d-1) for d <= 8 (exact for 1,2,4,8);
    round-half-up of v*255/65535 for d = 16."""
    if depth == 16:
        return (v * 255 * 2 + 65535) // (2 * 65535)
    return v * 255 // ((1 << depth) - 1)


def to16(v, depth):
    """P13.3: exact rescaling to 16 bits, v*65535/(2^d-1) (exact for every PNG depth)."""
    return v * 65535 // ((1 << depth) - 1)


def pixel_row(doc, line, w, wide):
    """P13: one reconstructed scanline of w pixels to RGBA tuples (8 or 16 bits per channel)."""
    ct, depth = doc['color_type'], doc['depth']
    s = samples(line, depth, w * CHANNELS[ct])
    conv = (lambda v: to16(v, depth)) if wide else (lambda v: to8(v, depth))
    top = 65535 if wide else 255
    trns = doc['trns']
    out = []
    if ct == 3:
        pal = doc['plte']
        ent = len(pal) // 3
        for i in s:
            if i >= ent:
                fail('PaletteIndexOutOfRange')
            a = trns[i] if trns is not None and i < len(trns) else 255
            rgb = pal[3 * i:3 * i + 3]
            out.append(tuple(x * 257 for x in rgb) + (a * 257,) if wide else tuple(rgb) + (a,))
        return out
    key = None
    if trns is not None:
        mask = (1 << depth) - 1
        key = tuple(be16(trns, j) & mask for j in range(0, len(trns), 2))
    for x in range(w):
        if ct == 0:
            g = s[x]
            a = 0 if key == (g,) else top
            out.append((conv(g),) * 3 + (a,))
        elif ct == 2:
            rgb = tuple(s[3 * x:3 * x + 3])
            a = 0 if key == rgb else top
            out.append(tuple(conv(v) for v in rgb) + (a,))
        elif ct == 4:
            g, al = s[2 * x], s[2 * x + 1]
            out.append((conv(g),) * 3 + (conv(al),))
        else:
            out.append(tuple(conv(v) for v in s[4 * x:4 * x + 4]))
    return out


def decode_image(doc, data, w, h, wide=False):
    """P11-P13: inflate (P9) the image data of a w x h image, unfilter each row of each pass in
    stream order, convert and place pixels (P12). Returns a flat list of RGBA tuples."""
    R = raw_size(doc, w, h)
    raw = zlib_decompress(data, R, 'ExcessImageData')
    if len(raw) < R:
        fail('TruncatedImageData')
    bpp = max(1, bits_pp(doc) // 8)
    px = [None] * (w * h)
    pos = 0
    for x0, y0, dx, dy, pw, ph in passes(doc, w, h):
        rb = row_bytes(doc, pw)
        prev = bytes(rb)
        for r in range(ph):
            ft = raw[pos]
            if ft > 4:
                fail('InvalidFilterType')
            line = unfilter_row(ft, raw[pos + 1:pos + 1 + rb], prev, bpp)
            pos += 1 + rb
            prev = line
            for c, p in enumerate(pixel_row(doc, line, pw, wide)):
                px[(y0 + r * dy) * w + x0 + c * dx] = p
    return px


def flat8(px):
    return bytes(v for p in px for v in p)


def public_info(doc):
    return {'color_type': doc['color_type'], 'depth': doc['depth'], 'interlace': doc['interlace'],
            'palette_entries': len(doc['plte']) // 3 if doc['plte'] is not None else 0,
            'trns': doc['trns'] is not None, 'meta': doc['meta']}


# ------------------------------------------------------------------ P15-P17 entry points
def decode_still(b):
    """P15: the default image (IDAT) as RGBA8 after the structural pass (P5)."""
    try:
        doc = walk(b)
        px = decode_image(doc, bytes(doc['idat']), doc['width'], doc['height'])
    except PngError as e:
        return {'error': e.name}
    return {'width': doc['width'], 'height': doc['height'], 'rgba': flat8(px), 'info': public_info(doc)}


def decode_still16(b):
    """P15: as decode_still with 16 bits per channel (P13.3)."""
    try:
        doc = walk(b)
        px = decode_image(doc, bytes(doc['idat']), doc['width'], doc['height'], wide=True)
    except PngError as e:
        return {'error': e.name}
    return {'width': doc['width'], 'height': doc['height'], 'rgba16': [v for p in px for v in p]}


def over(s, d):
    """P14.4: straight-alpha OVER on 8-bit samples with exact integer rounding (half up)."""
    sa, da = s[3], d[3]
    if sa == 255:
        return s
    if sa == 0:
        return d
    A = 255 * sa + da * (255 - sa)
    out = [(2 * (255 * sa * s[i] + da * (255 - sa) * d[i]) + A) // (2 * A) for i in range(3)]
    return tuple(out) + ((2 * A + 255) // 510,)


def decode_frames(b):
    """P16: the animation. Without acTL the default image is the only frame. Errors of P2-P8,
    P14.1 and of the default image return Err; an error decoding frame k >= 1 ends the sequence."""
    try:
        doc = walk(b)
        W, H = doc['width'], doc['height']
        default = decode_image(doc, bytes(doc['idat']), W, H)
    except PngError as e:
        return {'error': e.name}
    res = {'width': W, 'height': H, 'num_plays': None, 'frames': [], 'end': 'iend'}
    if doc['actl'] is None:
        res['frames'].append({'rgba': flat8(default), 'rect': [0, 0, W, H], 'delay': None,
                              'dispose': 0, 'blend': 0})
        return res
    res['num_plays'] = doc['actl']['num_plays']
    canvas = [(0, 0, 0, 0)] * (W * H)
    prev, saved = None, None
    for k, f in enumerate(doc['frames']):
        fx, fy, fw, fh = f['rect']
        try:
            if f['source'] == 'IDAT':
                fpx = default
            else:
                fpx = decode_image(doc, bytes(f['parts']), fw, fh)
        except PngError as e:
            res['end'] = {'error': e.name}
            return res
        if prev is not None:                                # P14.3 disposal of frame k-1
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
        for y in range(fh):                                 # P14.4 blending
            for x in range(fw):
                s = fpx[y * fw + x]
                o = (fy + y) * W + fx + x
                canvas[o] = s if f['blend'] == 0 else over(s, canvas[o])
        prev = (f['rect'], op)
        res['frames'].append({'rgba': flat8(canvas), 'rect': list(f['rect']), 'delay': list(f['delay']),
                              'dispose': f['dispose'], 'blend': f['blend']})
    return res


def icc_profile(b):
    """P17: the decompressed iCCP profile after the structural pass; None without iCCP."""
    try:
        doc = walk(b)
        if doc['iccp'] is None:
            return {'icc': None}
        return {'icc': zlib_decompress(doc['iccp'], MAX_ICC, 'IccProfileTooLarge')}
    except PngError as e:
        return {'error': e.name}


if __name__ == '__main__':
    import sys
    r = decode_still(open(sys.argv[1], 'rb').read())
    print({k: (v if k != 'rgba' else len(v)) for k, v in r.items()})
