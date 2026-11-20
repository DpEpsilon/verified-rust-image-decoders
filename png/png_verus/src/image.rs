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
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::layout_model as layout;

verus! {

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
pub(crate) fn pass(interlace: u8, w: u32, h: u32, k: usize) -> (r: Option<Pass>)
    requires 1<=w<=crate::limits::PNG_WIRE_MAX,1<=h<=crate::limits::PNG_WIRE_MAX,interlace<=1,
    ensures match r {
        None=>if interlace==0 {k!=0}else{k>=7 || layout::cols(w as nat,k as nat)==0 || layout::rows(h as nat,k as nat)==0},
        Some(p)=>1<=p.pw<=w && 1<=p.ph<=h && 1<=p.dx<=8 && 1<=p.dy<=8
            && if interlace==0 {k==0 && p.x0==0 && p.y0==0 && p.dx==1 && p.dy==1 && p.pw==w && p.ph==h}
            else{k<7 && (p.x0 as nat,p.y0 as nat,p.dx as nat,p.dy as nat)==layout::geometry(k as nat)
                && p.pw==layout::cols(w as nat,k as nat) && p.ph==layout::rows(h as nat,k as nat)},
    },
 {
    if interlace == 0 {
        return if k == 0 { Some(Pass { x0: 0, y0: 0, dx: 1, dy: 1, pw: w, ph: h }) } else { None };
    }
    if k >= 7 {
        return None;
    }
    let (x0, y0, dx, dy) = ADAM7[k];
    proof {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {}
        assert((x0 as nat,y0 as nat,dx as nat,dy as nat)==layout::geometry(k as nat));
        layout::span_bounds(w as nat,x0 as nat,dx as nat);
        layout::span_bounds(h as nat,y0 as nat,dy as nat);
    }

    let pw = if w > x0 { (w - x0 + dx - 1) / dx } else { 0 };
    let ph = if h > y0 { (h - y0 + dy - 1) / dy } else { 0 };
    if pw == 0 || ph == 0 {
        return None;
    }
    Some(Pass { x0, y0, dx, dy, pw, ph })
}

pub(crate) fn pass_count(interlace: u8) -> (r: usize)
    ensures r==layout::count(interlace),1<=r<=7,
{
    if interlace == 0 { 1 } else { 7 }
}

/// Bytes of one scanline of `pw` pixels after its filter byte, `⌈pw·B/8⌉`.
pub(crate) fn row_bytes(ihdr: &Ihdr, pw: u32) -> (r: u64)
    requires ihdr.wf(),pw<=crate::limits::PNG_WIRE_MAX,
    ensures r==layout::scan_bytes(layout::bits(ihdr.view()),pw as nat),r<=8*(pw as int),
{
    proof {layout::scan_bound(layout::bits(ihdr.view()),pw as nat);}
    (pw as u64 * ihdr.bits_pp() as u64 + 7) / 8
}

/// `R(w,h)`, P11: the sum over the non-empty passes of `ph·(1 + row bytes)`.
/// Callers pass `w ≤ W`, `h ≤ H` with `4·W·H ≤ MAX_OUTPUT`, so the sum is at
/// most `2·MAX_OUTPUT + MAX_OUTPUT/2 + 2` and fits `u64` (and one reservation).
pub(crate) fn raw_size(ihdr: &Ihdr, w: u32, h: u32) -> (r: u64)
    requires ihdr.wf(),1<=w<=ihdr.w,1<=h<=ihdr.h,
    ensures r==layout::raw(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat),
        1<=r<=crate::alloc_guard::MAX_BYTES,
        r<=layout::raw(ihdr.interlace,layout::bits(ihdr.view()),ihdr.w as nat,ihdr.h as nat),
{
    let bits=ihdr.bits_pp();
    proof {raw_budget(*ihdr,w,h);layout::raw_dimensions(ihdr.interlace,bits as nat,w as nat,h as nat,ihdr.w as nat,ihdr.h as nat);}
    let mut r=0u64;
    let mut k=0usize;
    while k<pass_count(ihdr.interlace)
        invariant ihdr.wf(),1<=w<=ihdr.w,1<=h<=ihdr.h,bits==layout::bits(ihdr.view()),
            bits<=64,k<=layout::count(ihdr.interlace),
            r==layout::prefix(ihdr.interlace,bits as nat,w as nat,h as nat,k as nat),
            layout::raw(ihdr.interlace,bits as nat,w as nat,h as nat)<=crate::alloc_guard::MAX_BYTES,
        decreases layout::count(ihdr.interlace)-k,
    {
        if let Some(p)=pass(ihdr.interlace,w,h,k) {
            let rb=row_bytes(ihdr,p.pw);
            proof {
                reveal_with_fuel(layout::prefix,2);
                layout::monotone(ihdr.interlace,bits as nat,w as nat,h as nat,(k+1) as nat,layout::count(ihdr.interlace));
                assert(layout::contribution(ihdr.interlace,bits as nat,w as nat,h as nat,k as nat)
                    ==p.ph as int*(1+rb as int));
            }
            r+=p.ph as u64*(1+rb);
        } else {
            proof {assert(layout::contribution(ihdr.interlace,bits as nat,w as nat,h as nat,k as nat)==0);}
        }
        k+=1;
    }
    r
}

/// Byte-counted Output request for RGBA8 or big-endian RGBA16.
pub(crate) fn output_size(ihdr: &Ihdr, wide: bool) -> (r: usize)
    requires ihdr.wf(),
    ensures r==(if wide {8int}else{4int})*(ihdr.w as int)*(ihdr.h as int),
        1<=r<=crate::alloc_guard::MAX_BYTES,
{
    proof {
        crate::limits::caps_ok();
        assert(1<=8*(ihdr.w as int)*(ihdr.h as int)<=2*crate::limits::MAX_OUTPUT) by (nonlinear_arith)
            requires 1<=ihdr.w,1<=ihdr.h,4*(ihdr.w as int)*(ihdr.h as int)<=crate::limits::MAX_OUTPUT;
        assert(1<=4*(ihdr.w as int)*(ihdr.h as int)<=crate::limits::MAX_OUTPUT) by (nonlinear_arith)
            requires 1<=ihdr.w,1<=ihdr.h,4*(ihdr.w as int)*(ihdr.h as int)<=crate::limits::MAX_OUTPUT;
    }
    proof {
        assert(1<=(ihdr.w as int)*(ihdr.h as int)<=crate::alloc_guard::MAX_BYTES) by (nonlinear_arith)
            requires 1<=ihdr.w,1<=ihdr.h,8*(ihdr.w as int)*(ihdr.h as int)<=2*crate::limits::MAX_OUTPUT,
                2*crate::limits::MAX_OUTPUT<=crate::alloc_guard::MAX_BYTES;
    }
    let n=ihdr.w as u64*ihdr.h as u64;
    proof {
        assert(1<=8*(n as int)<=crate::alloc_guard::MAX_BYTES) by (nonlinear_arith)
            requires n==(ihdr.w as int)*(ihdr.h as int),1<=ihdr.w,1<=ihdr.h,
                8*(ihdr.w as int)*(ihdr.h as int)<=2*crate::limits::MAX_OUTPUT,
                2*crate::limits::MAX_OUTPUT<=crate::alloc_guard::MAX_BYTES;
    }
    let bytes=if wide {8*n}else{4*n};
    proof {
        assert(bytes==(if wide {8int}else{4int})*(n as int));
        assert(bytes==(if wide {8int}else{4int})*(ihdr.w as int)*(ihdr.h as int)) by (nonlinear_arith)
            requires n==(ihdr.w as int)*(ihdr.h as int),bytes==(if wide {8int}else{4int})*(n as int);
    }
    bytes as usize
}

proof fn raw_budget(ihdr: Ihdr,w: u32,h: u32)
    requires ihdr.wf(),1<=w<=ihdr.w,1<=h<=ihdr.h,
    ensures 1<=layout::raw(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat)<=crate::alloc_guard::MAX_BYTES,
        w<=crate::limits::PNG_WIRE_MAX,h<=crate::limits::PNG_WIRE_MAX,
{
    crate::limits::caps_ok();
    assert(layout::bits(ihdr.view())<=64);
    layout::raw_bounds(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat);
    assert(4*(w as int)*(h as int)<=crate::limits::MAX_OUTPUT) by (nonlinear_arith)
        requires 1<=w<=ihdr.w,1<=h<=ihdr.h,4*(ihdr.w as int)*(ihdr.h as int)<=crate::limits::MAX_OUTPUT;
    assert(10*(w as int)*(h as int)+2<=5*(crate::limits::MAX_OUTPUT as int)/2+2) by (nonlinear_arith)
        requires 4*(w as int)*(h as int)<=crate::limits::MAX_OUTPUT;
}

} // verus!

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
