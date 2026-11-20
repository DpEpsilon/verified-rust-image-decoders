//! One image's decode, SPEC P11–P13: inflate the image data into `Raw` with
//! limit `R(w,h)`, then, pass by pass and scanline by scanline in stream order,
//! check the filter byte, reconstruct the scanline in place (its predecessor in
//! the same pass is the reconstructed bytes just before it in `Raw`), convert
//! its samples (P13) and place its pixels through the Adam7 map (P12) into a
//! `Sink`. No row, palette-expansion or frame buffer is allocated.

use inflate_verus::{inflate_zlib_into, Segments, Status};

use crate::png::{Error, Events};
use crate::policy::{PaletteIndex, Policy};
use crate::walk::{be16, Doc, Ihdr};

/// Adam7 passes `(x0, y0, dx, dy)` (P12).
pub(crate) const ADAM7: [(u32, u32, u32, u32); 7] =
    [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)];

/// One non-empty pass: origin, step and size in pixels.
#[derive(Clone, Copy)]
pub(crate) struct Pass {
    pub x0: u32,
    pub y0: u32,
    pub dx: u32,
    pub dy: u32,
    pub pw: u32,
    pub ph: u32,
}

/// The `k`-th pass of a `w × h` image (`k = 0` only without interlace), or
/// `None` when it is empty: `pw = ⌈(w − x0)/dx⌉` if `w > x0`, else 0 (P12).
pub(crate) fn pass(interlace: u8, w: u32, h: u32, k: usize) -> Option<Pass> {
    if interlace == 0 {
        return if k == 0 { Some(Pass { x0: 0, y0: 0, dx: 1, dy: 1, pw: w, ph: h }) } else { None };
    }
    if k >= 7 {
        return None;
    }
    let (x0, y0, dx, dy) = ADAM7[k];
    let pw = if w > x0 { (w - x0 + dx - 1) / dx } else { 0 };
    let ph = if h > y0 { (h - y0 + dy - 1) / dy } else { 0 };
    if pw == 0 || ph == 0 {
        return None;
    }
    Some(Pass { x0, y0, dx, dy, pw, ph })
}

pub(crate) fn pass_count(interlace: u8) -> usize {
    if interlace == 0 { 1 } else { 7 }
}

/// Bytes of one scanline of `pw` pixels after its filter byte, `⌈pw·B/8⌉`.
pub(crate) fn row_bytes(ihdr: &Ihdr, pw: u32) -> u64 {
    (pw as u64 * ihdr.bits_pp() as u64 + 7) / 8
}

/// `R(w,h)`, P11: the sum over the non-empty passes of `ph·(1 + row bytes)`.
/// Callers pass `w ≤ W`, `h ≤ H` with `4·W·H ≤ MAX_OUTPUT`, so the sum is at
/// most `2·MAX_OUTPUT + MAX_OUTPUT/2 + 2` and fits `u64` (and one reservation).
pub(crate) fn raw_size(ihdr: &Ihdr, w: u32, h: u32) -> u64 {
    let mut r = 0u64;
    let mut k = 0;
    while k < pass_count(ihdr.interlace) {
        if let Some(p) = pass(ihdr.interlace, w, h, k) {
            r += p.ph as u64 * (1 + row_bytes(ihdr, p.pw));
        }
        k += 1;
    }
    r
}

/// Where converted pixels go.
pub(crate) enum Sink<'o> {
    /// RGBA8 still: `out` is `4·W·H` bytes.
    Rgba8(&'o mut [u8]),
    /// RGBA16 still: `out` is `8·W·H` bytes, samples big-endian.
    Rgba16(&'o mut [u8]),
    /// An APNG frame drawn onto the `4·W·H` canvas at `(fx, fy)` with
    /// `blend_op` 0 (source) or 1 (over, P14.4).
    Canvas { canvas: &'o mut [u8], cw: u32, fx: u32, fy: u32, over: bool },
}

/// P14.4: exact straight-alpha OVER on 8-bit samples with half-up rounding.
#[inline]
pub(crate) fn over(s: [u8; 4], d: [u8; 4]) -> [u8; 4] {
    let sa = s[3] as u32;
    if sa == 255 {
        return s;
    }
    if sa == 0 {
        return d;
    }
    let da = d[3] as u32;
    // A ∈ [255, 65025]; each numerator N ≤ 255·A, so every result is ≤ 255.
    let a = 255 * sa + da * (255 - sa);
    let mut o = [0u8; 4];
    let mut i = 0;
    while i < 3 {
        let num = 255 * sa * s[i] as u32 + da * (255 - sa) * d[i] as u32;
        o[i] = ((2 * num + a) / (2 * a)) as u8;
        i += 1;
    }
    o[3] = ((2 * a + 255) / 510) as u8;
    o
}

impl<'o> Sink<'o> {
    /// Place pixel `(x, y)` of the image being decoded (16-bit channel values;
    /// the RGBA8 sinks receive values already converted to 8 bits).
    #[inline]
    fn put(&mut self, width: u32, x: u32, y: u32, px: [u16; 4]) {
        match self {
            Sink::Rgba8(out) => {
                let o = 4 * (y as usize * width as usize + x as usize);
                let mut i = 0;
                while i < 4 {
                    out[o + i] = px[i] as u8;
                    i += 1;
                }
            }
            Sink::Rgba16(out) => {
                let o = 8 * (y as usize * width as usize + x as usize);
                let mut i = 0;
                while i < 4 {
                    out[o + 2 * i] = (px[i] >> 8) as u8;
                    out[o + 2 * i + 1] = px[i] as u8;
                    i += 1;
                }
            }
            Sink::Canvas { canvas, cw, fx, fy, over: ov } => {
                let o = 4 * ((*fy + y) as usize * *cw as usize + (*fx + x) as usize);
                let s = [px[0] as u8, px[1] as u8, px[2] as u8, px[3] as u8];
                let r = if *ov { over(s, [canvas[o], canvas[o + 1], canvas[o + 2], canvas[o + 3]]) } else { s };
                canvas[o..o + 4].copy_from_slice(&r);
            }
        }
    }

    fn wide(&self) -> bool {
        matches!(self, Sink::Rgba16(_))
    }
}

/// Map an inflate status to the image result (P9–P11). `produced` bytes were
/// written of a limit of `r`.
fn image_status(status: Status, produced: u64, r: u64) -> Result<(), Error> {
    match status {
        Status::Done => {
            if produced < r {
                Err(Error::TruncatedImageData)
            } else {
                Ok(())
            }
        }
        Status::Full => Err(Error::ExcessImageData),
        // No inflate error is renamed (council M1BQ-Q1 (b)): under IgnoreTail
        // `inflate_verus` ignores bytes after a matching Adler-32 only when the
        // output is exactly R bytes, and a short stream followed by bytes stays
        // TrailingZlibData, as under Strict.
        Status::Err(e) => Err(Error::from_inflate(e)),
    }
}

#[inline]
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i16 + b as i16 - c as i16;
    let pa = (p - a as i16).abs();
    let pb = (p - b as i16).abs();
    let pc = (p - c as i16).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

/// Reconstruct one scanline in place (P11): `cur` is the filtered row after its
/// filter byte, `prev` the reconstructed previous row of the same pass (`None`
/// for the pass's first scanline, read as zeros).
fn unfilter(ft: u8, cur: &mut [u8], prev: Option<&[u8]>, bpp: usize) {
    let n = cur.len();
    let mut i = 0;
    while i < n {
        let a = if i >= bpp { cur[i - bpp] } else { 0 };
        let b = match prev {
            Some(p) => p[i],
            None => 0,
        };
        let c = match prev {
            Some(p) if i >= bpp => p[i - bpp],
            _ => 0,
        };
        let x = cur[i];
        cur[i] = match ft {
            0 => x,
            1 => x.wrapping_add(a),
            2 => x.wrapping_add(b),
            3 => x.wrapping_add(((a as u16 + b as u16) >> 1) as u8),
            _ => x.wrapping_add(paeth(a, b, c)),
        };
        i += 1;
    }
}

/// P13.1: sample `i` of a reconstructed scanline.
#[inline]
fn sample(line: &[u8], depth: u8, i: usize) -> u16 {
    match depth {
        16 => be16(line, 2 * i),
        8 => line[i] as u16,
        _ => {
            let per = 8 / depth as usize;
            let byte = line[i / per];
            let shift = 8 - depth as usize * (i % per + 1);
            ((byte >> shift) & ((1u8 << depth) - 1)) as u16
        }
    }
}

/// P13.2 `s8` and P13.3 `s16`.
#[inline]
fn scale(v: u16, depth: u8, wide: bool) -> u16 {
    let v = v as u32;
    if wide {
        (v * 65535 / ((1u32 << depth) - 1)) as u16
    } else if depth == 16 {
        ((v * 510 + 65535) / 131070) as u16
    } else {
        (v * 255 / ((1u32 << depth) - 1)) as u16
    }
}

/// The colour context of one image (P13): palette, tRNS and the masked key.
struct Colour<'b> {
    ct: u8,
    depth: u8,
    plte: &'b [u8],
    trns: Option<&'b [u8]>,
    key: [u16; 3],
}

impl<'b> Colour<'b> {
    fn new(doc: &Doc, b: &'b [u8]) -> Colour<'b> {
        let plte = match doc.plte {
            Some((o, n)) => &b[o..o + n],
            None => &b[0..0],
        };
        let trns = doc.trns.map(|(o, n)| &b[o..o + n]);
        let mut key = [0u16; 3];
        if let Some(t) = trns {
            if doc.ihdr.ct == 0 || doc.ihdr.ct == 2 {
                let mask: u32 = (1u32 << doc.ihdr.depth) - 1;
                let mut j = 0;
                while j + 1 < t.len() && j / 2 < 3 {
                    key[j / 2] = (be16(t, j) as u32 & mask) as u16;
                    j += 2;
                }
            }
        }
        Colour { ct: doc.ihdr.ct, depth: doc.ihdr.depth, plte, trns, key }
    }

    /// P13.2/P13.3: pixel `x` of a reconstructed scanline.
    #[inline]
    fn pixel(&self, line: &[u8], x: usize, wide: bool, pol: &Policy, ev: &mut Events) -> Result<[u16; 4], Error> {
        let top: u16 = if wide { 65535 } else { 255 };
        let d = self.depth;
        match self.ct {
            0 => {
                let g = sample(line, d, x);
                let a = if self.trns.is_some() && self.key[0] == g { 0 } else { top };
                let s = scale(g, d, wide);
                Ok([s, s, s, a])
            }
            2 => {
                let r = sample(line, d, 3 * x);
                let g = sample(line, d, 3 * x + 1);
                let bl = sample(line, d, 3 * x + 2);
                let a = if self.trns.is_some() && self.key == [r, g, bl] { 0 } else { top };
                Ok([scale(r, d, wide), scale(g, d, wide), scale(bl, d, wide), a])
            }
            3 => {
                let i = sample(line, d, x) as usize;
                if i >= self.plte.len() / 3 {
                    return match pol.palette_index {
                        PaletteIndex::Black => {
                            ev.r3 = true;
                            Ok([0, 0, 0, top])
                        }
                        PaletteIndex::Reject => Err(Error::PaletteIndexOutOfRange),
                    };
                }
                let a = match self.trns {
                    Some(t) if i < t.len() => t[i] as u16,
                    _ => 255,
                };
                let m = if wide { 257 } else { 1 };
                Ok([
                    self.plte[3 * i] as u16 * m,
                    self.plte[3 * i + 1] as u16 * m,
                    self.plte[3 * i + 2] as u16 * m,
                    a * m,
                ])
            }
            4 => {
                let g = scale(sample(line, d, 2 * x), d, wide);
                let a = scale(sample(line, d, 2 * x + 1), d, wide);
                Ok([g, g, g, a])
            }
            _ => Ok([
                scale(sample(line, d, 4 * x), d, wide),
                scale(sample(line, d, 4 * x + 1), d, wide),
                scale(sample(line, d, 4 * x + 2), d, wide),
                scale(sample(line, d, 4 * x + 3), d, wide),
            ]),
        }
    }
}

/// Decode one `w × h` image (P11–P13) whose zlib stream `src` yields, using
/// `raw` (at least `R(w,h)` bytes, P20 `Raw`) and drawing into `sink`.
pub(crate) fn decode_image<S: Segments>(
    doc: &Doc,
    b: &[u8],
    src: &mut S,
    raw: &mut [u8],
    w: u32,
    h: u32,
    pol: &Policy,
    ev: &mut Events,
    sink: &mut Sink<'_>,
) -> Result<(), Error> {
    let ihdr = &doc.ihdr;
    let r = raw_size(ihdr, w, h);
    let raw = &mut raw[..r as usize];
    let res = inflate_zlib_into(src, raw, pol.inflate_options());
    image_status(res.status, res.produced, r)?;
    let bpp = ihdr.bpp();
    let wide = sink.wide();
    let colour = Colour::new(doc, b);
    let mut pos = 0usize;
    let mut k = 0;
    while k < pass_count(ihdr.interlace) {
        if let Some(p) = pass(ihdr.interlace, w, h, k) {
            let rb = row_bytes(ihdr, p.pw) as usize;
            let mut row = 0u32;
            while row < p.ph {
                let ft = raw[pos];
                if ft > 4 {
                    return Err(Error::InvalidFilterType);
                }
                let (before, rest) = raw.split_at_mut(pos + 1);
                let cur = &mut rest[..rb];
                let prev = if row > 0 { Some(&before[pos - rb..pos]) } else { None };
                unfilter(ft, cur, prev, bpp);
                let y = p.y0 + row * p.dy;
                let mut c = 0u32;
                while c < p.pw {
                    let px = colour.pixel(cur, c as usize, wide, pol, ev)?;
                    sink.put(w, p.x0 + c * p.dx, y, px);
                    c += 1;
                }
                pos += 1 + rb;
                row += 1;
            }
        }
        k += 1;
    }
    Ok(())
}
