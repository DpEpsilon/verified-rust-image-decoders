"""Held-out mutation set for the PNG policy draft (GAPS-CLOSED.md gap 6).

Deterministic: seeds are every distinct spec vector and probe input of 33 bytes to 2 KiB that starts with the PNG signature (taken from
../../spec-draft/VECTORS.json and probes_def.PROBES, never from any runner result); mutations are
drawn with random.Random(SEED). Every held-out input is at most 4 KiB, differs from every fitted
input (vectors, probes, witnesses) and from every other held-out input. The set was generated and its
digest recorded in ../../GAPS-CLOSED.md before any incumbent ran on it; no compat field was changed
after looking at its results.

Mutation kinds (one per input, the kind chosen uniformly):
  flip        xor one byte after the signature with a random non-zero value; CRC left stale
  flip-crc    as flip, then the CRC of the chunk containing the byte is recomputed
  zflip       xor one byte of the zlib data of a random IDAT or fdAT chunk, CRC recomputed
  delete      remove one chunk other than IHDR
  duplicate   insert a copy of one chunk other than IHDR directly after it
  move        move one chunk other than IHDR to another chunk boundary after IHDR
  swap        exchange two adjacent chunks after IHDR
  length      add a value in [-4, 4] \\ {0} to one chunk's length field (CRC untouched)
  truncate    cut the file at a random offset after the signature
  split       split one IDAT into two IDAT chunks at a random data offset (valid CRCs)
  append      append 1-16 random bytes after the last byte

Usage: python3 -I heldout_def.py [A|B]   -> prints the count and the set digest
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
sys.path.insert(0, str(HERE.parent.parent / 'spec-draft'))

SEED = 20261114          # set A: frozen before any runner saw it
SEED_B = 20261115        # set B: frozen after the gap-2 and gap-5 oracle changes, before any runner saw it
COUNT = 400
KINDS = ('flip', 'flip-crc', 'zflip', 'delete', 'duplicate', 'move', 'swap', 'length', 'truncate', 'split', 'append')


def chunks(b):
    """Chunk spans (start, end, type) of a well-framed prefix; stops at the first framing problem."""
    out, p = [], 8
    while p + 12 <= len(b):
        n = struct.unpack('>I', b[p:p + 4])[0]
        if p + 12 + n > len(b):
            break
        out.append((p, p + 12 + n, bytes(b[p + 4:p + 8])))
        p += 12 + n
    return out, p


def mk(t, d):
    return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d))


def mutate(b, kind, r):
    cs, tail = chunks(b)
    body = [c for c in cs if c[2] != b'IHDR']
    if kind in ('flip', 'flip-crc'):
        i = r.randrange(8, len(b))
        x = bytearray(b)
        x[i] ^= r.randrange(1, 256)
        if kind == 'flip-crc':
            for s, e, _ in cs:
                if s + 4 <= i < e - 4:          # inside type or data: recompute that chunk's CRC
                    x[e - 4:e] = struct.pack('>I', zlib.crc32(bytes(x[s + 4:e - 4])))
                    break
            else:
                return None
        return bytes(x)
    if kind == 'zflip':
        zs = [c for c in cs if c[2] in (b'IDAT', b'fdAT') and c[1] - c[0] > 12 + (4 if c[2] == b'fdAT' else 0)]
        if not zs:
            return None
        s, e, t = r.choice(zs)
        d = bytearray(b[s + 8:e - 4])
        i = r.randrange(4 if t == b'fdAT' else 0, len(d))
        d[i] ^= r.randrange(1, 256)
        return b[:s] + mk(t, bytes(d)) + b[e:]
    if kind == 'split':
        zs = [c for c in cs if c[2] == b'IDAT' and c[1] - c[0] > 13]
        if not zs:
            return None
        s, e, t = r.choice(zs)
        d = b[s + 8:e - 4]
        k = r.randrange(1, len(d))
        return b[:s] + mk(b'IDAT', d[:k]) + mk(b'IDAT', d[k:]) + b[e:]
    if kind == 'truncate':
        return b[:r.randrange(8, len(b))]
    if kind == 'append':
        return b + bytes(r.randrange(256) for _ in range(r.randrange(1, 17)))
    if not body:
        return None
    if kind == 'delete':
        s, e, _ = r.choice(body)
        return b[:s] + b[e:]
    if kind == 'duplicate':
        s, e, _ = r.choice(body)
        return b[:e] + b[s:e] + b[e:]
    if kind == 'length':
        s, e, _ = r.choice(body)
        n = struct.unpack('>I', b[s:s + 4])[0] + r.choice((-4, -3, -2, -1, 1, 2, 3, 4))
        if n < 0:
            return None
        return b[:s] + struct.pack('>I', n) + b[s + 4:]
    first = cs[0][1] if cs and cs[0][2] == b'IHDR' else None
    if first is None:
        return None
    if kind == 'swap':
        k = [i for i in range(len(cs) - 1) if cs[i][2] != b'IHDR']
        if not k:
            return None
        i = r.choice(k)
        (s1, e1, _), (s2, e2, _) = cs[i], cs[i + 1]
        return b[:s1] + b[s2:e2] + b[s1:e1] + b[e2:]
    if kind == 'move':
        s, e, _ = r.choice(body)
        rest = b[:s] + b[e:]
        cs2, _ = chunks(rest)
        bounds = [c[1] for c in cs2 if c[1] >= first]
        if not bounds:
            return None
        q = r.choice(bounds)
        return rest[:q] + b[s:e] + rest[q:]
    raise ValueError(kind)


def fitted_inputs():
    """Seeds and the exclusion set: vectors, probes, witnesses (bytes only, no runner results)."""
    from probes_def import PROBES
    seen = []
    for v in json.loads((HERE.parent.parent / 'spec-draft/VECTORS.json').read_text()):
        if 'pad_to' not in v:
            seen.append(bytes.fromhex(v['input_hex']))
    seen += [b for _, b, _ in PROBES]
    excl = set(seen)
    wit = Path.home() / 'projects/leniency/png/witnesses'
    excl |= {f.read_bytes() for f in sorted(wit.glob('*.png'))}
    seeds, s = [], set()
    for b in seen:
        if 33 <= len(b) <= 2048 and b[:8] == b'\x89PNG\r\n\x1a\n' and b not in s:
            s.add(b)
            seeds.append(b)
    return seeds, excl


def heldout(which='A'):
    """[(name, bytes, why)] in a fixed order. Set B excludes every input of set A."""
    seeds, excl = fitted_inputs()
    seed, tag = (SEED, 'held') if which == 'A' else (SEED_B, 'heldB')
    if which == 'B':
        excl = excl | {b for _, b, _ in heldout('A')}
    r = random.Random(seed)
    out, got = [], set()
    tries = 0
    while len(out) < COUNT:
        tries += 1
        if tries > 100 * COUNT:
            raise RuntimeError('mutation budget exhausted')
        si = r.randrange(len(seeds))
        kind = r.choice(KINDS)
        m = mutate(seeds[si], kind, r)
        if m is None or len(m) > 4096 or m in excl or m in got:
            continue
        got.add(m)
        out.append((f'{tag}:{len(out):03d}-{kind}-s{si}', m, f'{kind} of seed {si}'))
    return out


def digest(items):
    h = hashlib.sha256()
    for name, b, _ in items:
        h.update(name.encode() + b'\0' + hashlib.sha256(b).digest())
    return h.hexdigest()


if __name__ == '__main__':
    H = heldout(sys.argv[1] if len(sys.argv) > 1 else 'A')
    from collections import Counter
    print(len(H), 'inputs; digest', digest(H))
    print(sorted(Counter(n.split('-', 1)[1].rsplit('-s', 1)[0] for n, _, _ in H).items()))
    print('max len', max(len(b) for _, b, _ in H))
