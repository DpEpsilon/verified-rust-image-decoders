#!/usr/bin/env python3
"""DDS (image/vnd.ms-dds) leniency adapter.

Decoders (identical bytes, one subprocess each; output = first mip, array layer 0, volume slice 0, straight RGBA8):
  image              image 0.25.10 DdsDecoder (explicit format)         } same implementation: ONE vote
  glycin             glycin-image-rs ImgLoader, mime image/vnd.ms-dds    }
  image-dds          image_dds 0.7.2 (ddsfile + its own BCn/uncompressed decoders)
  ddsfile-bcdec      ddsfile 0.5.2 container + bcdec_rs 0.2.1 (BC1-5,7) + our bitmask expansion
  ddsfile-texpresso  ddsfile 0.5.2 container + texpresso 2.0.2 (BC1-5 only)
Unavailable: GDK (no DDS/HDR module installed), DirectXTex (C++, not fetched), stb_image (no DDS support).
All inputs are synthetic: DDS files built here from a seeded PRNG; nothing from a third-party corpus.
"""
import argparse, random, struct, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import leniency_core as core  # noqa: E402

EXE = Path('/tmp/claude-0/target-dds/release/leniency-dds-runner')

# ---- DDS builder (all offsets from the Microsoft DDS_HEADER / DDS_HEADER_DXT10 layout) ----
CAPS, HEIGHT, WIDTH, PITCH, PIXELFORMAT, MIPMAPCOUNT, LINEARSIZE, DEPTH = 1, 2, 4, 8, 0x1000, 0x20000, 0x80000, 0x800000
PF_ALPHAPIXELS, PF_ALPHA, PF_FOURCC, PF_RGB, PF_LUMINANCE = 1, 2, 4, 0x40, 0x20000
CAPS_COMPLEX, CAPS_TEXTURE, CAPS_MIPMAP = 8, 0x1000, 0x400000
CAPS2_CUBEMAP, CAPS2_ALLFACES, CAPS2_VOLUME = 0x200, 0xFC00, 0x200000

FIELDS = {  # name: (offset, size)
    'magic': (0, 4), 'header_size': (4, 4), 'flags': (8, 4), 'height': (12, 4), 'width': (16, 4), 'pitch_or_linear_size': (20, 4),
    'depth': (24, 4), 'mip_count': (28, 4), 'reserved1_0': (32, 4), 'reserved1_10': (72, 4), 'pf_size': (76, 4), 'pf_flags': (80, 4),
    'pf_fourcc': (84, 4), 'pf_bit_count': (88, 4), 'pf_r_mask': (92, 4), 'pf_g_mask': (96, 4), 'pf_b_mask': (100, 4), 'pf_a_mask': (104, 4),
    'caps': (108, 4), 'caps2': (112, 4), 'caps3': (116, 4), 'caps4': (120, 4), 'reserved2': (124, 4),
    'dx10_dxgi_format': (128, 4), 'dx10_resource_dimension': (132, 4), 'dx10_misc_flag': (136, 4), 'dx10_array_size': (140, 4), 'dx10_misc_flags2': (144, 4)}
DX10_ONLY = {k for k in FIELDS if k.startswith('dx10_')}

BLOCK = {'DXT1': 8, 'DXT3': 16, 'DXT5': 16, 'ATI1': 8, 'ATI2': 16, 'DXT2': 16, 'DXT4': 16, 'BC4U': 8, 'BC5U': 16}
DXGI_BC = {'BC1': (71, 8), 'BC2': (74, 16), 'BC3': (77, 16), 'BC4': (80, 8), 'BC5': (83, 16), 'BC7': (98, 16)}
# name: (bits, r, g, b, a, pf_flags)
MASKS = {
    'rgb24': (24, 0xff0000, 0xff00, 0xff, 0, PF_RGB), 'xrgb32': (32, 0xff0000, 0xff00, 0xff, 0, PF_RGB),
    'argb32': (32, 0xff0000, 0xff00, 0xff, 0xff000000, PF_RGB | PF_ALPHAPIXELS), 'abgr32': (32, 0xff, 0xff00, 0xff0000, 0xff000000, PF_RGB | PF_ALPHAPIXELS),
    'rgb565': (16, 0xf800, 0x7e0, 0x1f, 0, PF_RGB), 'argb4444': (16, 0xf00, 0xf0, 0xf, 0xf000, PF_RGB | PF_ALPHAPIXELS),
    'argb1555': (16, 0x7c00, 0x3e0, 0x1f, 0x8000, PF_RGB | PF_ALPHAPIXELS), 'rgb332': (8, 0xe0, 0x1c, 0x3, 0, PF_RGB),
    'l8': (8, 0xff, 0, 0, 0, PF_LUMINANCE), 'l8a8': (16, 0xff, 0, 0, 0xff00, PF_LUMINANCE | PF_ALPHAPIXELS), 'l16': (16, 0xffff, 0, 0, 0, PF_LUMINANCE),
    'a8': (8, 0, 0, 0, 0xff, PF_ALPHA), 'bgr24': (24, 0xff, 0xff00, 0xff0000, 0, PF_RGB)}
DXGI_RAW = {'R8G8B8A8': (28, 4), 'B8G8R8A8': (87, 4), 'R8': (61, 1), 'A8': (65, 1), 'B8G8R8X8': (88, 4)}


def level_dims(w, h, d, mip): return max(1, w >> mip), max(1, h >> mip), max(1, d >> mip)


def bc_kind(fourcc, dx10):
    name = {'DXT1': 'BC1', 'DXT2': 'BC2', 'DXT3': 'BC2', 'DXT4': 'BC3', 'DXT5': 'BC3', 'ATI1': 'BC4', 'BC4U': 'BC4', 'ATI2': 'BC5', 'BC5U': 'BC5'}.get(fourcc) or dx10
    return name if name in ('BC1', 'BC2', 'BC3', 'BC4', 'BC5') else None


def endpoint_blocks(rng, kind, nblocks):
    """Blocks that use only endpoint indices (0/1): decoders cannot differ by interpolation rounding."""
    def colour():
        c0 = rng.randrange(1, 65536); c1 = rng.randrange(0, c0)
        idx = sum(rng.randrange(2) << (2 * i) for i in range(16))
        return struct.pack('<HHI', c0, c1, idx)
    def alpha():
        a0 = rng.randrange(1, 256); a1 = rng.randrange(0, a0)
        bits = sum(rng.randrange(2) << (3 * i) for i in range(16))
        return bytes([a0, a1]) + bits.to_bytes(6, 'little')
    out = bytearray()
    for _ in range(nblocks):
        if kind == 'BC1': out += colour()
        elif kind == 'BC2': out += bytes(rng.randrange(256) for _ in range(8)) + colour()
        elif kind == 'BC3': out += alpha() + colour()
        elif kind == 'BC4': out += alpha()
        else: out += alpha() + alpha()
    return bytes(out)


def build(rng, w, h, *, fourcc=None, mask=None, dx10=None, mips=1, cube=False, volume=0, array=1, payload='endpoints', size_fields=True):
    """Return (bytes, expected_features). mips = number of levels stored; array only with dx10."""
    block = None; bpp = None
    if fourcc: block = BLOCK[fourcc]
    elif dx10 and dx10 in DXGI_BC: block = DXGI_BC[dx10][1]
    elif mask: bpp = MASKS[mask][0] // 8
    else: bpp = DXGI_RAW[dx10][1]
    d = volume if volume else 1
    def level_size(lw, lh, ld):
        return (((lw + 3) // 4) * ((lh + 3) // 4) * block if block else lw * lh * bpp) * ld
    data = bytearray()
    faces = 6 if cube else 1
    for _a in range(array):
        for _f in range(faces):
            for m in range(mips):
                lw, lh, ld = level_dims(w, h, d, m)
                n = level_size(lw, lh, ld)
                kind = bc_kind(fourcc, dx10)
                if payload == 'endpoints' and kind: data += endpoint_blocks(rng, kind, n // block)
                elif payload in ('random', 'endpoints'): data += bytes(rng.randrange(256) for _ in range(n))
                elif payload == 'gradient': data += bytes(((i * 7 + m * 31) & 255) for i in range(n))
                else: data += bytes(n)
    flags = CAPS | HEIGHT | WIDTH | PIXELFORMAT
    caps = CAPS_TEXTURE; caps2 = 0
    if mips > 1: flags |= MIPMAPCOUNT; caps |= CAPS_COMPLEX | CAPS_MIPMAP
    if volume: flags |= DEPTH; caps |= CAPS_COMPLEX; caps2 |= CAPS2_VOLUME
    if cube: caps |= CAPS_COMPLEX; caps2 |= CAPS2_CUBEMAP | CAPS2_ALLFACES
    if block:
        flags |= LINEARSIZE; pitch = level_size(w, h, 1)
    else:
        flags |= PITCH; pitch = w * bpp
    if not size_fields: pitch = 0
    pf = [32, 0, 0, 0, 0, 0, 0, 0]
    if dx10: pf[1] = PF_FOURCC; pf[2] = struct.unpack('<I', b'DX10')[0]
    elif fourcc: pf[1] = PF_FOURCC; pf[2] = struct.unpack('<I', fourcc.encode())[0]
    else:
        bits, r, g, b, a, pff = MASKS[mask]; pf = [32, pff, 0, bits, r, g, b, a]
    hdr = struct.pack('<4sIIIIIII44s', b'DDS ', 124, flags, h, w, pitch, d if volume else 0, mips if mips > 1 else 0, bytes(44))
    hdr += struct.pack('<8I', *pf) + struct.pack('<5I', caps, caps2, 0, 0, 0)
    if dx10:
        fmt = DXGI_BC[dx10][0] if dx10 in DXGI_BC else DXGI_RAW[dx10][0]
        hdr += struct.pack('<5I', fmt, 4 if volume else 3, 4 if cube else 0, array, 0)
    return bytes(hdr) + bytes(data)


def seeds(rng):
    """(name, family, bytes). Names are stable; payloads come from the seeded PRNG."""
    out = []
    def add(name, family, **kw):
        w, h = kw.pop('wh'); out.append((name, family, build(rng, w, h, **kw)))
    for cc, nm in [('DXT1', 'dxt1'), ('DXT3', 'dxt3'), ('DXT5', 'dxt5')]:
        for wh in [(8, 8), (7, 5), (1, 1), (4, 4)]: add(f'legacy-{nm}-{wh[0]}x{wh[1]}', 'legacy-bcn', fourcc=cc, wh=wh)
    for cc in ['DXT2', 'DXT4', 'ATI1', 'ATI2', 'BC4U', 'BC5U']: add(f'legacy-{cc.lower()}-8x8', 'legacy-bc45' if cc in ('ATI1', 'ATI2', 'BC4U', 'BC5U') else 'legacy-bcn', fourcc=cc, wh=(8, 8))
    for mk in MASKS:
        for wh in [(8, 8), (5, 3)]: add(f'mask-{mk}-{wh[0]}x{wh[1]}', 'uncompressed-bitmask', mask=mk, wh=wh)
    for f in ['BC1', 'BC2', 'BC3', 'BC4', 'BC5', 'BC7']:
        for wh in [(8, 8), (6, 6)]: add(f'dx10-{f.lower()}-{wh[0]}x{wh[1]}', 'dx10', dx10=f, payload='random' if f == 'BC7' else 'endpoints', wh=wh)
    for f in ['R8G8B8A8', 'B8G8R8A8', 'R8', 'A8', 'B8G8R8X8']: add(f'dx10-{f.lower()}-8x8', 'dx10', dx10=f, wh=(8, 8))
    for cc in ['DXT1', 'DXT5']: add(f'mips-{cc.lower()}-16x16', 'mipmaps', fourcc=cc, mips=5, wh=(16, 16))
    add('mips-argb32-8x8', 'mipmaps', mask='argb32', mips=4, wh=(8, 8))
    add('mips-dx10-bc7-16x16', 'mipmaps', dx10='BC7', mips=5, payload='random', wh=(16, 16))
    add('cube-dxt1-8x8', 'cube', fourcc='DXT1', cube=True, wh=(8, 8))
    add('cube-dxt5-mips-8x8', 'cube', fourcc='DXT5', cube=True, mips=4, wh=(8, 8))
    add('cube-dx10-bc1-8x8', 'cube', dx10='BC1', cube=True, wh=(8, 8))
    add('cube-dx10-array2-bc3-8x8', 'cube', dx10='BC3', cube=True, array=2, wh=(8, 8))
    add('volume-argb32-4x4x4', 'volume', mask='argb32', volume=4, wh=(4, 4))
    add('volume-argb32-mips-4x4x4', 'volume', mask='argb32', volume=4, mips=3, wh=(4, 4))
    add('volume-dxt1-8x8x2', 'volume', fourcc='DXT1', volume=2, wh=(8, 8))
    add('array-dx10-bc1-3-8x8', 'array', dx10='BC1', array=3, wh=(8, 8))
    add('array-dx10-rgba8-mips-2', 'array', dx10='R8G8B8A8', array=2, mips=3, wh=(8, 8))
    for cc in ['DXT1', 'DXT3', 'DXT5', 'ATI1', 'ATI2']: add(f'random-{cc.lower()}-8x8', 'legacy-random-blocks', fourcc=cc, payload='random', wh=(8, 8))
    add('random-dx10-bc7-8x8', 'dx10-random-blocks', dx10='BC7', payload='random', wh=(8, 8))
    add('nolinearsize-dxt1', 'legacy-bcn', fourcc='DXT1', size_fields=False, wh=(8, 8))
    add('nopitch-argb32', 'uncompressed-bitmask', mask='argb32', size_fields=False, wh=(8, 8))
    add('zeros-dxt1', 'legacy-bcn', fourcc='DXT1', payload='zeros', wh=(8, 8))
    add('gradient-dxt5', 'legacy-bcn', fourcc='DXT5', payload='gradient', wh=(8, 8))
    return out


VALUES = [0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 124, 127, 128, 129, 255, 256, 257, 4096, 65535, 65536,
          0x7fffffff, 0x80000000, 0xfffffffe, 0xffffffff]


def u32(b, p): return struct.unpack_from('<I', b, p)[0]


def variants(b, rng):
    n = len(b); hdr_end = 148 if u32(b, 84) == 0x30315844 and u32(b, 80) & 4 else 128
    out = []
    cuts = {0, 1, 3, 4, 8, 64, 127, 128, 131, 132, 147, 148, 149, hdr_end, hdr_end + 1, hdr_end + 4, hdr_end + 7, hdr_end + 8, n // 2, n - 1, n - 2, n - 4}
    for c in sorted(x for x in cuts if 0 <= x < n): out.append((b[:c], {'rule': 'truncation', 'field': 'length', 'value': c}))
    for k in (1, 2, 4, 16, 64, 1024): out.append((b + bytes(k), {'rule': 'trailing-bytes', 'field': 'tail', 'value': k}))
    out.append((b + b'\xff' * 5, {'rule': 'trailing-bytes', 'field': 'tail-ff', 'value': 5}))
    for _ in range(14):
        p = rng.randrange(n); a = bytearray(b); a[p] ^= 1 << rng.randrange(8)
        out.append((bytes(a), {'rule': 'byte-flip-header' if p < hdr_end else 'byte-flip-payload', 'field': 'offset', 'value': p}))
    for _ in range(10):
        p = rng.randrange(min(n, hdr_end)); a = bytearray(b); a[p] ^= 1 << rng.randrange(8)
        out.append((bytes(a), {'rule': 'byte-flip-header', 'field': 'offset', 'value': p}))
    pool = []
    for name, (p, sz) in FIELDS.items():
        if p + sz > n or (name in DX10_ONLY and hdr_end < 148): continue
        old = u32(b, p)
        vals = list(dict.fromkeys(v for v in VALUES + [old - 1, old + 1, old ^ 1, old * 2, old // 2, n, n - hdr_end, (n - hdr_end) + 1, (n - hdr_end) - 1, old | 0x80000000, old & 0x7fffffff] if 0 <= v <= 0xffffffff and v != old))
        for v in vals:
            a = bytearray(b); a[p:p + 4] = struct.pack('<I', v); pool.append((bytes(a), {'rule': name, 'field': name, 'value': v, 'old': old}))
    # flag-bit toggles for the flag words
    for name in ('flags', 'pf_flags', 'caps', 'caps2', 'dx10_misc_flag'):
        if name in DX10_ONLY and hdr_end < 148: continue
        p, _ = FIELDS[name]; old = u32(b, p)
        for bit in range(32):
            a = bytearray(b); a[p:p + 4] = struct.pack('<I', old ^ (1 << bit)); pool.append((bytes(a), {'rule': name + '-bit', 'field': name, 'value': bit, 'old': old}))
    rng.shuffle(pool)
    return out, pool


def generate(limit, seed):
    rng = random.Random(seed)
    ss = seeds(rng)
    items = [(b, {'source': 'seed', 'name': name, 'family': fam, 'rule': 'seed:' + fam, 'expected_ok': True}) for name, fam, b in ss]
    items = items[:limit]
    ordered = []
    for name, fam, b in ss:
        fixed, pool = variants(b, rng)
        ordered.append((name, fam, fixed, pool))
    # round robin: fixed-structure mutations first (truncation/flip/trailing), then the shuffled field pool
    gen = [(name, fam, iter(core.interleave_by_rule(fixed + pool, rng))) for name, fam, fixed, pool in ordered]
    active = list(range(len(gen)))
    while len(items) < limit and active:
        for i in active[:]:
            try: b, m = next(gen[i][2])
            except StopIteration: active.remove(i); continue
            m.update(source='mutation', seed=gen[i][0], family=gen[i][1]); items.append((b, m))
            if len(items) >= limit: break
    return items, len(ss)


def rgba_difference(cfg, path, results, timeout, folder):
    from leniency_core import engine
    good = [n for n, v in results.items() if v['status'] == 'ok']
    if len(good) < 2: return ''
    if len({(results[n]['width'], results[n]['height']) for n in good}) > 1: return 'dimensions'
    dumps = []
    for n in good:
        t = folder / f'pix-{n}.rgba'
        v = engine.invoke(path, cfg.runners[n], timeout, t)
        if v['status'] != 'ok' or not t.exists(): return 'rgba-dump-unavailable'
        dumps.append(t.read_bytes()); t.unlink()
    ref = dumps[0]; groups = {}
    for n, d in zip(good, dumps): groups.setdefault(d, []).append(n)
    if len(groups) == 1: return 'identical'
    parts = []
    others = [d for d in groups if d != ref]
    for d in others:
        if len(d) != len(ref): parts.append('length'); continue
        rgb_same = all(ref[i:i + 3] == d[i:i + 3] for i in range(0, len(d), 4)); a_same = all(ref[i + 3] == d[i + 3] for i in range(0, len(d), 4))
        swap = all(ref[i] == d[i + 2] and ref[i + 1] == d[i + 1] and ref[i + 2] == d[i] and ref[i + 3] == d[i + 3] for i in range(0, len(d), 4))
        mx = max(abs(x - y) for x, y in zip(ref, d))
        mrgb = max(abs(ref[i + k] - d[i + k]) for i in range(0, len(d), 4) for k in range(3)); ma = max(abs(ref[i + 3] - d[i + 3]) for i in range(0, len(d), 4))
        parts.append(f'alpha-only(max {ma})' if rgb_same else ('red-blue-swap' if swap else (f'rgb-only(max {mrgb})' if a_same else f'rgb(max {mrgb})+alpha(max {ma})')))
    return '/'.join(sorted(set(parts)))


class DdsConfig(core.Config):
    fmt = 'dds'; ext = 'dds'; adapter = HERE
    alt_hash = True
    alt_description = 'decoders whose RGBA8 outputs differ by at most 1 on every channel (BCn endpoint-expansion / rounding convention) are treated as equal; shows what remains once +-1 rounding is set aside'

    def refine(self, path, results, timeout):
        ok = [n for n, v in results.items() if v['status'] == 'ok']
        results = {k: dict(v) for k, v in results.items()}
        for n in ok: results[n]['alt_sha256'] = results[n]['rgba_sha256']
        if len({results[n]['rgba_sha256'] for n in ok}) <= 1 or len({(results[n]['width'], results[n]['height']) for n in ok}) > 1: return results
        from leniency_core import engine as eng
        tmp = Path(tempfile.mkdtemp(prefix='dds-', dir='/tmp/claude-0')); d = {}
        try:
            for n in ok:
                t = tmp / f'{n}.rgba'; v = eng.invoke(path, self.runners[n], timeout, t)
                if v['status'] != 'ok' or not t.exists(): return results
                d[n] = t.read_bytes()
        finally:
            for f in tmp.glob('*'): f.unlink()
            tmp.rmdir()
        clusters = []
        for n in ok:
            for rep, mem in clusters:
                if len(d[rep]) == len(d[n]) and max(abs(x - y) for x, y in zip(d[rep], d[n])) <= 1: mem.append(n); break
            else: clusters.append((n, [n]))
        for rep, mem in clusters:
            for n in mem: results[n]['alt_sha256'] = results[rep]['rgba_sha256']
        for n in ok: results[n]['max_abs_diff_vs_ref'] = max(abs(x - y) for x, y in zip(d[ok[0]], d[n])) if len(d[ok[0]]) == len(d[n]) else None
        return results

    meta_keys = ('mips', 'layers', 'cube', 'depth')
    runners = {n: (EXE, [n]) for n in ('image', 'glycin', 'image-dds', 'ddsfile-bcdec', 'ddsfile-texpresso')}
    unavailable = {'gdk-pixbuf DDS module': 'not installed (gdk-pixbuf-query-loaders lists none)', 'DirectXTex': 'C++ library not fetched; would need a CMake build',
                   'stb_image': 'has no DDS support'}
    def generate(self, limit, seed): return generate(limit, seed)
    def pixel_difference(self, path, results, timeout, folder): return rgba_difference(self, path, results, timeout, folder)
    def intro(self):
        return ('Decoded surface compared: **first mip level, array layer 0, volume slice 0, straight top-first RGBA8** (SHA-256 of the RGBA bytes). '
                '`image` and `glycin` are the same implementation (image::DdsDecoder) and count as one vote. `image` supports only DXT1/3/5 (and DX10 BC1-3) and returns RGB for DXT1. '
                '`ddsfile-*` columns are our glue around the ddsfile container parser: BC4 is shown as (r,0,0,255), BC5 as (r,g,0,255), alpha-only as (0,0,0,a), luminance as (l,l,l,a), bitmask channels scaled to 8 bits by truncating division; texpresso covers BC1-5 only (others show `UnsupportedByCandidate`, which is a coverage gap of that column, not a rejection judgement). '
                'The mip/cube/array/volume handling of each decoder is stored per input in the results JSONL (`mips`, `layers`, `cube`, `depth`, `policy`).')
    def notes(self):
        return ('Seeds: legacy FourCC DXT1/2/3/4/5 and ATI1/ATI2/BC4U/BC5U, 13 uncompressed bitmask layouts (RGB, RGBA, luminance, alpha, 8/16/24/32-bit), DX10 extended header (BC1-5,7, RGBA8, BGRA8, R8, A8, BGRX8), '
                'mipmap chains, cube maps (legacy caps2 and DX10), volume textures, DX10 arrays. Mutations: truncation at structural boundaries, trailing bytes, byte flips, every header field at boundary values, flag-bit toggles. '
                'Provenance (seed name, rule, field, old and new value) per input is in the manifest.')


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('--inputs', type=int, default=10000); ap.add_argument('--seed', type=int, default=731)
    ap.add_argument('--jobs', type=int, default=4); ap.add_argument('--timeout', type=float, default=4.0); ap.add_argument('--min-checks', type=int, default=24)
    core.run(DdsConfig(), ap.parse_args())
