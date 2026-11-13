//! Seeded synthetic generator for the large benchmark inputs. Writes only under OUTDIR
//! (intended: /tmp/claude-0/bench-large). Content is generated from the seed; no third-party bytes.
//! Usage: gen OUTDIR [SEED] [--probes]   (--probes also writes the 8192x8192 and 16384x16384 files)
use std::{fs::File, io::{BufWriter, Write}, path::Path};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 { self.0 ^= self.0 >> 12; self.0 ^= self.0 << 25; self.0 ^= self.0 >> 27; self.0.wrapping_mul(0x2545F4914F6CDD1D) }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}
/// Gradient with a noise block every few 8x8 tiles: compressible enough for RLE, not constant.
fn px(x: u32, y: u32, w: u32, h: u32, seed: u64) -> [u8; 3] {
    let (bx, by) = ((x >> 3) as u64, (y >> 3) as u64);
    let hsh = (bx.wrapping_mul(0x9E3779B97F4A7C15) ^ by.wrapping_mul(0xC2B2AE3D27D4EB4F) ^ seed).wrapping_mul(0x165667B19E3779F9);
    if (hsh >> 60) < 4 { let v = (hsh >> 8) as u32 ^ (x * 2654435761u32).rotate_left(y & 15); [v as u8, (v >> 8) as u8, (v >> 16) as u8] }
    else { [(x as u64 * 255 / w as u64) as u8, (y as u64 * 255 / h as u64) as u8, ((x + y) & 255) as u8] }
}
fn le16(v: u16) -> [u8; 2] { v.to_le_bytes() }
fn le32(v: u32) -> [u8; 4] { v.to_le_bytes() }
fn out(dir: &Path, name: &str) -> BufWriter<File> { BufWriter::with_capacity(1 << 20, File::create(dir.join(name)).unwrap()) }

fn bmp_header(w: &mut impl Write, wd: u32, ht: u32, bpp: u16, comp: u32, image: u32, pal: u32) {
    let off = 54 + pal * 4;
    w.write_all(b"BM").unwrap(); w.write_all(&le32(off + image)).unwrap(); w.write_all(&[0; 4]).unwrap(); w.write_all(&le32(off)).unwrap();
    w.write_all(&le32(40)).unwrap(); w.write_all(&le32(wd)).unwrap(); w.write_all(&le32(ht)).unwrap();
    w.write_all(&le16(1)).unwrap(); w.write_all(&le16(bpp)).unwrap(); w.write_all(&le32(comp)).unwrap(); w.write_all(&le32(image)).unwrap();
    w.write_all(&le32(2835)).unwrap(); w.write_all(&le32(2835)).unwrap(); w.write_all(&le32(if pal > 0 { pal } else { 0 })).unwrap(); w.write_all(&le32(0)).unwrap();
}
fn bmp_rgb(dir: &Path, name: &str, wd: u32, ht: u32, bpp: u16, seed: u64) {
    let bytes = if bpp == 24 { (wd * 3 + 3) & !3 } else { wd * 4 };
    let mut w = out(dir, name);
    bmp_header(&mut w, wd, ht, bpp, 0, bytes * ht, 0);
    let mut row = vec![0u8; bytes as usize];
    for y in (0..ht).rev() {
        for x in 0..wd {
            let [r, g, b] = px(x, y, wd, ht, seed);
            if bpp == 24 { row[x as usize * 3..x as usize * 3 + 3].copy_from_slice(&[b, g, r]); }
            else { row[x as usize * 4..x as usize * 4 + 4].copy_from_slice(&[b, g, r, 255]); }
        }
        w.write_all(&row).unwrap();
    }
}
fn palette(w: &mut impl Write) { for i in 0..256u32 { w.write_all(&[(i * 7) as u8, (i * 3) as u8, i as u8, 0]).unwrap(); } }
fn bmp_8raw(dir: &Path, name: &str, wd: u32, ht: u32, seed: u64) {
    let stride = (wd + 3) & !3;
    let mut w = out(dir, name);
    bmp_header(&mut w, wd, ht, 8, 0, stride * ht, 256); palette(&mut w);
    let mut row = vec![0u8; stride as usize];
    for y in (0..ht).rev() { for x in 0..wd { row[x as usize] = px(x, y, wd, ht, seed)[2]; } w.write_all(&row).unwrap(); }
}
/// RLE8: per row a seeded mix of encoded runs and absolute runs, EOL after each row, EOB at the end.
fn bmp_rle8(dir: &Path, name: &str, wd: u32, ht: u32, seed: u64) {
    let mut rng = Rng(seed | 1);
    let mut body = Vec::new();
    for _ in 0..ht {
        let mut left = wd as u64;
        while left > 0 {
            let abs = rng.below(10) < 3 && left >= 3;
            let n = 1 + rng.below(left.min(255));
            if abs && n >= 3 { body.extend([0, n as u8]); for _ in 0..n { body.push(rng.next() as u8); } if n % 2 == 1 { body.push(0); } }
            else { body.extend([n as u8, rng.next() as u8]); }
            left -= n;
        }
        body.extend([0, 0]);
    }
    body.extend([0, 1]);
    let mut w = out(dir, name);
    bmp_header(&mut w, wd, ht, 8, 1, body.len() as u32, 256); palette(&mut w);
    w.write_all(&body).unwrap();
}
fn tga_header(w: &mut impl Write, ty: u8, wd: u32, ht: u32, bpp: u8) {
    let mut h = [0u8; 18]; h[2] = ty; h[12..14].copy_from_slice(&le16(wd as u16)); h[14..16].copy_from_slice(&le16(ht as u16));
    h[16] = bpp; h[17] = if bpp == 32 { 8 } else { 0 }; w.write_all(&h).unwrap();
}
fn tga_raw(dir: &Path, name: &str, wd: u32, ht: u32, bpp: u8, seed: u64) {
    let mut w = out(dir, name); tga_header(&mut w, 2, wd, ht, bpp);
    let n = bpp as usize / 8; let mut row = vec![0u8; wd as usize * n];
    for y in (0..ht).rev() {
        for x in 0..wd { let [r, g, b] = px(x, y, wd, ht, seed); row[x as usize * n..x as usize * n + 3].copy_from_slice(&[b, g, r]); if n == 4 { row[x as usize * n + 3] = 255; } }
        w.write_all(&row).unwrap();
    }
}
fn tga_rle(dir: &Path, name: &str, wd: u32, ht: u32, bpp: u8, seed: u64) {
    let mut rng = Rng(seed | 1);
    let mut w = out(dir, name); tga_header(&mut w, 10, wd, ht, bpp);
    let n = bpp as usize / 8;
    let pixel = |rng: &mut Rng| { let v = rng.next(); let mut p = [255u8; 4]; p[..3].copy_from_slice(&v.to_le_bytes()[..3]); p };
    for _ in 0..ht {
        let mut left = wd as u64;
        while left > 0 {
            let k = 1 + rng.below(left.min(128));
            if rng.below(10) < 3 { w.write_all(&[(k - 1) as u8]).unwrap(); for _ in 0..k { w.write_all(&pixel(&mut rng)[..n]).unwrap(); } }
            else { w.write_all(&[0x80 | (k - 1) as u8]).unwrap(); w.write_all(&pixel(&mut rng)[..n]).unwrap(); }
            left -= k;
        }
    }
}
fn crc32(d: &[u8]) -> u32 { let mut c = !0u32; for &b in d { c ^= b as u32; for _ in 0..8 { c = if c & 1 != 0 { 0xEDB88320 ^ (c >> 1) } else { c >> 1 }; } } !c }
fn adler(d: &[u8]) -> u32 { let (mut a, mut b) = (1u32, 0u32); for &x in d { a = (a + x as u32) % 65521; b = (b + a) % 65521; } (b << 16) | a }
/// PNG with stored (uncompressed) deflate blocks: valid, no compressor dependency.
fn png(side: u32, seed: u64) -> Vec<u8> {
    let mut raw = Vec::new();
    for y in 0..side { raw.push(0); for x in 0..side { let [r, g, b] = px(x, y, side, side, seed); raw.extend([r, g, b, 255]); } }
    let mut z = vec![0x78, 0x01];
    let mut chunks = raw.chunks(65535).peekable();
    while let Some(c) = chunks.next() { z.push(chunks.peek().is_none() as u8); z.extend(le16(c.len() as u16)); z.extend(le16(!(c.len() as u16))); z.extend(c); }
    z.extend(adler(&raw).to_be_bytes());
    let mut o = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut chunk = |t: &[u8; 4], d: &[u8]| { o.extend((d.len() as u32).to_be_bytes()); let mut td = t.to_vec(); td.extend(d); o.extend(&td); o.extend(crc32(&td).to_be_bytes()); };
    let mut ihdr = Vec::new(); ihdr.extend(side.to_be_bytes()); ihdr.extend(side.to_be_bytes()); ihdr.extend([8, 6, 0, 0, 0]);
    chunk(b"IHDR", &ihdr); chunk(b"IDAT", &z); chunk(b"IEND", &[]);
    o
}
fn icns_elem(code: &[u8; 4], payload: &[u8]) -> Vec<u8> { let mut e = code.to_vec(); e.extend((payload.len() as u32 + 8).to_be_bytes()); e.extend(payload); e }
fn icns_file(elems: &[Vec<u8>]) -> Vec<u8> { let body: Vec<u8> = elems.concat(); let mut f = b"icns".to_vec(); f.extend((body.len() as u32 + 8).to_be_bytes()); f.extend(body); f }
/// Legacy RLE channel streams ("is32" family): three planes, each a sequence of literal packets
/// (byte < 0x80: n+1 literals) and run packets (byte >= 0x80: n-0x80+3 copies of one byte).
fn legacy_rle(side: usize, seed: u64, with_header: bool) -> Vec<u8> {
    let mut rng = Rng(seed | 1); let n = side * side; let mut o = Vec::new();
    if with_header { o.extend([0u8; 4]); }
    for _ in 0..3 {
        let mut left = n;
        while left > 0 {
            if left >= 3 && rng.below(10) < 5 {
                let k = 3 + rng.below(left.min(130) as u64 - 2) as usize;
                o.extend([0x80 + (k - 3) as u8, rng.next() as u8]); left -= k;
            } else {
                let k = 1 + rng.below(left.min(128) as u64) as usize;
                o.push((k - 1) as u8); for _ in 0..k { o.push(rng.next() as u8); } left -= k;
            }
        }
    }
    o
}
fn ico_dib(side: u32, seed: u64) -> Vec<u8> {
    let mut d = Vec::new();
    d.extend(le32(40)); d.extend(le32(side)); d.extend(le32(side * 2)); d.extend(le16(1)); d.extend(le16(32));
    d.extend(le32(0)); d.extend(le32(0)); d.extend([0u8; 16]);
    for y in (0..side).rev() { for x in 0..side { let [r, g, b] = px(x, y, side, side, seed); d.extend([b, g, r, 255]); } }
    d.extend(vec![0u8; (((side + 31) / 32) * 4 * side) as usize]);
    d
}
fn ico_file(entries: &[(u32, Vec<u8>)], dir_side_override: Option<u8>) -> Vec<u8> {
    let mut f = vec![0, 0, 1, 0]; f.extend(le16(entries.len() as u16));
    let mut off = 6 + 16 * entries.len() as u32;
    for (side, d) in entries {
        let s = dir_side_override.unwrap_or(if *side >= 256 { 0 } else { *side as u8 });
        f.extend([s, s, 0, 0]); f.extend(le16(1)); f.extend(le16(32)); f.extend(le32(d.len() as u32)); f.extend(le32(off)); off += d.len() as u32;
    }
    for (_, d) in entries { f.extend(d); }
    f
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let dir = Path::new(a.get(1).expect("usage: gen OUTDIR [SEED] [--probes]"));
    let seed: u64 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(20260101);
    std::fs::create_dir_all(dir).unwrap();
    bmp_rgb(dir, "bmp24_4096x4096.bmp", 4096, 4096, 24, seed);
    bmp_rgb(dir, "bmp24_8192x2048.bmp", 8192, 2048, 24, seed);
    bmp_rgb(dir, "bmp32_4096x3840.bmp", 4096, 3840, 32, seed);
    bmp_8raw(dir, "bmp8_4096x4096.bmp", 4096, 4096, seed);
    bmp_rle8(dir, "bmp8rle_4096x4096.bmp", 4096, 4096, seed);
    tga_raw(dir, "tga24_4096x4096.tga", 4096, 4096, 24, seed);
    tga_raw(dir, "tga32_4096x3840.tga", 4096, 3840, 32, seed);
    tga_rle(dir, "tga24rle_4096x4096.tga", 4096, 4096, 24, seed);
    tga_rle(dir, "tga32rle_4096x3840.tga", 4096, 3840, 32, seed);
    // ICNS: legacy 128x128 colour+mask pair; PNG-in-ICNS 1024x1024 (ic10); legacy pair plus a 32 MiB unrelated element.
    let it32 = icns_elem(b"it32", &legacy_rle(128, seed, true));
    let t8mk = icns_elem(b"t8mk", &(0..128 * 128).map(|i| (i * 31 % 251) as u8).collect::<Vec<u8>>());
    std::fs::write(dir.join("icns_legacy128.icns"), icns_file(&[it32.clone(), t8mk.clone()])).unwrap();
    std::fs::write(dir.join("icns_png1024.icns"), icns_file(&[icns_elem(b"ic10", &png(1024, seed))])).unwrap();
    std::fs::write(dir.join("icns_legacy128_pad32M.icns"), icns_file(&[it32, t8mk, icns_elem(b"TOC ", &vec![7u8; 32 << 20])])).unwrap();
    // ICO: one 256x256 32bpp DIB entry; six entries 16..256; and 1024x1024 DIB under a 0x0 (=256) directory record.
    std::fs::write(dir.join("ico_dib256.ico"), ico_file(&[(256, ico_dib(256, seed))], None)).unwrap();
    let multi: Vec<(u32, Vec<u8>)> = [16u32, 32, 48, 64, 128, 256].iter().map(|&s| (s, ico_dib(s, seed))).collect();
    std::fs::write(dir.join("ico_multi6.ico"), ico_file(&multi, None)).unwrap();
    std::fs::write(dir.join("probe_ico_dib1024_dir256.ico"), ico_file(&[(256, ico_dib(1024, seed))], None)).unwrap();
    if a.iter().any(|s| s == "--probes") {
        bmp_rgb(dir, "probe_bmp24_8192x8192.bmp", 8192, 8192, 24, seed);
        bmp_rgb(dir, "probe_bmp24_16384x16384.bmp", 16384, 16384, 24, seed);
        tga_raw(dir, "probe_tga24_8192x8192.tga", 8192, 8192, 24, seed);
    }
}
