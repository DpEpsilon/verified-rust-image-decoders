"""Generic seeded token-level and byte-level mutators for C-source-like text formats (shared by XBM and XPM)."""
import random, re

TOKEN = re.compile(rb'\s+|/\*.*?\*/|//[^\n]*|0[xX][0-9A-Fa-f]+|#[0-9A-Fa-f]+|[0-9]+|[A-Za-z_][A-Za-z_0-9]*|.', re.S)
BAD = [b'\x00', b'\xff', b'\x80', b'\x01', b'\x7f', b'\x1a', b'\x0b', b'\x0c', b'\\', b'\xc3\xa9', b'\xef\xbb\xbf', b'\r', b'\xe2\x80\xa8', b'`', b'\xa0']
LONG = [b' ', b'A', b'0', b'/', b'x', b',', b'\t']


def tokens(b):
    return [(m.start(), m.end()) for m in TOKEN.finditer(b)]


def truncations(b, rng, n=14):
    cuts = {0, 1, len(b) - 1, len(b) - 2, len(b) // 2} | {rng.randrange(len(b)) for _ in range(n - 5)}
    t = tokens(b); cuts |= {t[rng.randrange(len(t))][1] for _ in range(4)} if t else set()
    for c in sorted(x for x in cuts if 0 <= x < len(b)):
        yield b[:c], {'rule': 'truncate', 'field': 'length', 'value': c}


def byte_flips(b, rng, n=10):
    for _ in range(n):
        p = rng.randrange(len(b)); a = bytearray(b); bit = rng.randrange(8); a[p] ^= 1 << bit
        yield bytes(a), {'rule': 'byte-flip', 'field': 'offset', 'value': p}


def bad_chars(b, rng, n=10):
    for i in range(n):
        p = 0 if i == 0 else rng.randrange(len(b) + 1); ch = BAD[i % len(BAD)] if i < len(BAD) else rng.choice(BAD)
        if i % 2: yield b[:p] + ch + b[p + 1:], {'rule': 'bad-char-replace', 'field': 'offset', 'value': f'{p}:{ch.hex()}'}
        else: yield b[:p] + ch + b[p:], {'rule': 'bad-char-insert', 'field': 'offset', 'value': f'{p}:{ch.hex()}'}


def long_lines(b, rng, n=4):
    for i in range(n):
        p = [0, len(b) // 2, len(b), rng.randrange(len(b) + 1)][i % 4]; ch = LONG[(i + rng.randrange(7)) % len(LONG)]
        size = rng.choice([4095, 4096, 65537, 70000])
        yield b[:p] + ch * size + b[p:], {'rule': 'long-run', 'field': 'offset', 'value': f'{p}:{ch!r}x{size}'}


def token_ops(b, rng, n=16, nonspace=True):
    t = [x for x in tokens(b) if not b[x[0]:x[1]].isspace()] if nonspace else tokens(b)
    if not t: return
    pool = [b[s:e] for s, e in t[:200]]
    for i in range(n):
        k = rng.randrange(len(t)); s, e = t[k]; op = ('delete', 'duplicate', 'swap-next', 'replace')[i % 4]
        if op == 'delete': out = b[:s] + b[e:]
        elif op == 'duplicate': out = b[:e] + b[s:e] + b[e:]
        elif op == 'swap-next' and k + 1 < len(t):
            s2, e2 = t[k + 1]; out = b[:s] + b[s2:e2] + b[e:s2] + b[s:e] + b[e2:]
        else: out = b[:s] + rng.choice(pool) + b[e:]
        yield out, {'rule': 'token-' + op, 'field': 'token', 'value': k}


def newline_styles(b):
    flat = b.replace(b'\r\n', b'\n').replace(b'\r', b'\n')
    for name, nl in (('crlf', b'\r\n'), ('cr', b'\r'), ('none', b' '), ('double', b'\n\n')):
        yield flat.replace(b'\n', nl), {'rule': 'newlines', 'field': 'style', 'value': name}


def interleave(streams, limit, taken):
    """Round-robin over per-seed generators, deduplicating bytes, until limit items exist."""
    seen = set(x[0] for x in taken); out = list(taken); live = list(range(len(streams)))
    while len(out) < limit and live:
        for i in live[:]:
            for b, m in streams[i][1]:
                if b in seen or len(b) > 400000: continue
                seen.add(b); m = dict(m, source='mutation', seed=streams[i][0]); out.append((b, m)); break
            else: live.remove(i)
            if len(out) >= limit: break
    return out
