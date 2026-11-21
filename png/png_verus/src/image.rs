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
            && p.pw==layout::span(w as nat,p.x0 as nat,p.dx as nat)
            && p.ph==layout::span(h as nat,p.y0 as nat,p.dy as nat)
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

verus! {

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
    proof {assert(0<255*(sa as int)+(da as int)*(255-sa as int)<=130050) by (nonlinear_arith)
        requires 1<=sa<255,da<=255;}
    let a = 255 * sa + da * (255 - sa);
    let mut o = [0u8; 4];
    let mut i = 0;
    while i < 3
        invariant i<=3,1<=sa<255,da<=255,a==255*sa+da*(255-sa),0<a<=130050,
        decreases 3-i,
    {
        let sc=s[i] as u32;
        let dc=d[i] as u32;
        proof {
            assert(255*(sa as int)*(sc as int)+(da as int)*(255-sa as int)*(dc as int)<=33162750) by (nonlinear_arith)
                requires 1<=sa<255,da<=255,sc<=255,dc<=255;
        }
        let num = 255 * sa * sc + da * (255 - sa) * dc;
        o[i] = ((2 * num + a) / (2 * a)) as u8;
        i += 1;
    }
    o[3] = ((2 * a + 255) / 510) as u8;
    o
}

proof fn pixel_offset(w: nat,h: nat,x: nat,y: nat,bytes: nat,n: nat)
    requires x<w,y<h,bytes>=1,bytes*w*h<=n<=crate::alloc_guard::MAX_BYTES,
    ensures y*w<=usize::MAX,y*w+x<=usize::MAX,bytes*(y*w+x)+bytes<=n<=usize::MAX,
{
    crate::limits::caps_ok();
    assert(y*w+x+1<=w*h) by (nonlinear_arith) requires 0<=y<h,0<=x<w;
    assert(bytes*(y*w+x+1)<=bytes*w*h) by (nonlinear_arith)
        requires y*w+x+1<=w*h,bytes>=1;
    assert(bytes*(y*w+x)+bytes==bytes*(y*w+x+1)) by (nonlinear_arith);
    assert(y*w+x<=bytes*(y*w+x)) by (nonlinear_arith) requires bytes>=1,0<=y,0<=w,0<=x;
}

impl<'o> Sink<'o> {
    /// The final contents of the original destination loan, not current pixels.
    #[verifier::prophetic]
    pub(crate) closed spec fn loan(self) -> Seq<u8> {
        match self {Self::Rgba8(out)|Self::Rgba16(out)=>final(out)@,Self::Canvas {canvas,..}=>final(canvas)@}
    }
    /// End the nested destination loan so its owner recovers the final length.
    pub(crate) fn finish(self)
        ensures self.loan().len()==self.shape().1,
            match self {
            Self::Rgba8(out)|Self::Rgba16(out)=>final(out)@.len()==self.shape().1,
            Self::Canvas {canvas,..}=>final(canvas)@.len()==self.shape().1,
        },
    {match self {Self::Rgba8(_)|Self::Rgba16(_)|Self::Canvas {..}=>{}}}
    pub(crate) fn image(out: &'o mut [u8],w: u32,h: u32,wide: bool) -> (r: Self)
        requires 1<=w<=crate::limits::PNG_WIRE_MAX,1<=h<=crate::limits::PNG_WIRE_MAX,
            out.len()==(if wide {8nat}else{4nat})*(w as nat)*(h as nat),out.len()<=crate::alloc_guard::MAX_BYTES,
        ensures r.wf(w,h),r.wide_view()==wide,r.shape().1==old(out).len(),r.loan()==final(out)@,
            match r {Self::Rgba8(s)|Self::Rgba16(s)=>s@==old(out)@ && final(s)@==final(out)@,_=>false},
    {if wide {Self::Rgba16(out)}else{Self::Rgba8(out)}}
    pub(crate) fn canvas(canvas: &'o mut [u8],cw: u32,fx: u32,fy: u32,w: u32,h: u32,over: bool) -> (r: Self)
        requires 1<=cw<=crate::limits::PNG_WIRE_MAX,1<=w<=crate::limits::PNG_WIRE_MAX,1<=h<=crate::limits::PNG_WIRE_MAX,
            fx as int+w as int<=cw,fy as int+h as int<=crate::limits::PNG_WIRE_MAX,
            4*(fy as int+h as int)*(cw as int)<=canvas.len()<=crate::alloc_guard::MAX_BYTES,
        ensures r.wf(w,h),!r.wide_view(),r.shape().1==old(canvas).len(),r.loan()==final(canvas)@,
    {Self::Canvas {canvas,cw,fx,fy,over}}

    pub(crate) closed spec fn wide_view(self) -> bool {matches!(self,Self::Rgba16(_))}
    pub(crate) closed spec fn shape(self) -> (nat,nat,nat,nat,nat,bool) {
        match self {
            Self::Rgba8(out)=>(0,out@.len(),0,0,0,false),
            Self::Rgba16(out)=>(1,out@.len(),0,0,0,false),
            Self::Canvas {canvas,cw,fx,fy,over}=>(2,canvas@.len(),cw as nat,fx as nat,fy as nat,over),
        }
    }
    pub(crate) closed spec fn wf(self,w: u32,h: u32) -> bool {
        1<=w<=crate::limits::PNG_WIRE_MAX && 1<=h<=crate::limits::PNG_WIRE_MAX
        && match self {
            Self::Rgba8(out)=>out@.len()==4*(w as nat)*(h as nat) && out@.len()<=crate::alloc_guard::MAX_BYTES,
            Self::Rgba16(out)=>out@.len()==8*(w as nat)*(h as nat) && out@.len()<=crate::alloc_guard::MAX_BYTES,
            Self::Canvas {canvas,cw,fx,fy,..}=>1<=cw<=crate::limits::PNG_WIRE_MAX
                && fx as int+w as int<=cw && fy as int+h as int<=crate::limits::PNG_WIRE_MAX
                && 4*(fy as int+h as int)*(cw as int)<=canvas@.len()<=crate::alloc_guard::MAX_BYTES,
        }
    }
    /// Place pixel `(x, y)` of the image being decoded (16-bit channel values;
    /// the RGBA8 sinks receive values already converted to 8 bits).
    #[inline]
    fn put(&mut self, width: u32,height: u32, x: u32, y: u32, px: [u16; 4])
        requires old(self).wf(width,height),x<width,y<height,
            !old(self).wide_view() ==> forall|i: int| 0<=i<4 ==> (#[trigger] px@[i])<=u8::MAX,
        ensures final(self).shape()==old(self).shape(),final(self).wf(width,height),final(self).loan()==old(self).loan(),
    {
        proof {crate::limits::caps_ok();}
        match self {
            Sink::Rgba8(out) => {
                proof {pixel_offset(width as nat,height as nat,x as nat,y as nat,4,out.len() as nat);}
                let o = 4 * (y as usize * width as usize + x as usize);
                let ghost n=out.len();
                let mut i = 0;
                while i < 4
                    invariant i<=4,o+4<=out.len(),out.len()==n,n==4*(width as nat)*(height as nat),n<=crate::alloc_guard::MAX_BYTES,forall|j: int| 0<=j<4 ==> (#[trigger] px@[j])<=u8::MAX,
                    decreases 4-i,
                {
                    out[o + i] = px[i] as u8;
                    i += 1;
                }
            }
            Sink::Rgba16(out) => {
                proof {pixel_offset(width as nat,height as nat,x as nat,y as nat,8,out.len() as nat);}
                let o = 8 * (y as usize * width as usize + x as usize);
                let ghost n=out.len();
                let mut i = 0;
                while i < 4
                    invariant i<=4,o+8<=out.len(),out.len()==n,n==8*(width as nat)*(height as nat),n<=crate::alloc_guard::MAX_BYTES,
                    decreases 4-i,
                {
                    let word=px[i];
                    proof {assert(word>>8<=u8::MAX as u16 && (word&255)<=u8::MAX as u16) by (bit_vector);}
                    out[o+2*i]=(word>>8) as u8;
                    out[o+2*i+1]=(word&255) as u8;
                    i += 1;
                }
            }
            Sink::Canvas { canvas, cw, fx, fy, over: ov } => {
                proof {
                    let stride=*cw as nat;let rows=*fy as nat+height as nat;
                    assert(4*stride*rows==4*rows*stride) by (nonlinear_arith);
                    pixel_offset(*cw as nat,*fy as nat+height as nat,*fx as nat+x as nat,*fy as nat+y as nat,4,canvas.len() as nat);
                }
                let o = 4 * ((*fy + y) as usize * *cw as usize + (*fx + x) as usize);
                let s = [px[0] as u8, px[1] as u8, px[2] as u8, px[3] as u8];
                let r = if *ov { over(s, [canvas[o], canvas[o + 1], canvas[o + 2], canvas[o + 3]]) } else { s };
                canvas[o..o + 4].copy_from_slice(&r);
            }
        }
    }

    fn wide(&self) -> (r: bool)
        ensures r==self.wide_view(),
    {
        matches!(self, Sink::Rgba16(_))
    }
}

} // verus!

/// Map an inflate status to the image result (P9–P11). `produced` bytes were
/// written of a limit of `r`.
verus! {

pub(crate) open spec fn inflate_error(z: Seq<u8>,limit: nat,pol: Policy) -> Option<Error> {
    match inflate_verus::model::Inflate(z,limit,pol.window==crate::policy::Window::Enforce,
        pol.zlib_tail==crate::policy::ZlibTail::IgnoreTail).status {
        inflate_verus::model::InflateStatus::Error(f)=>Some(crate::icc_model::fault(f)),_=>None,
    }
}
pub(crate) proof fn two_byte_zlib_error(limit: nat,pol: Policy)
    ensures inflate_error(seq![120u8,1u8],limit,pol)==Some(Error::TruncatedZlib),
{
    hide(inflate_verus::model::Inflate);hide(inflate_verus::model::MEnvelope);
    hide(inflate_verus::model::MBlk);hide(inflate_verus::model::MOneBlock);
    hide(inflate_verus::model::window_size);
    let z=seq![120u8,1u8];
    let empty=Seq::<u8>::empty();
    let fault=inflate_verus::model::StoredEffect::Error {
        error:inflate_verus::model::InflateFault::TruncatedZlib,out:empty,position:16nat};
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);
    assert(inflate_verus::model::pow2(15)==32768) by (compute);
    assert(inflate_verus::model::window_size(7,pol.window==crate::policy::Window::Enforce)==32768) by {
        reveal(inflate_verus::model::window_size);
    }
    assert(inflate_verus::model::MOneBlock(z,16,empty,limit,32768,true)==(false,fault)) by {
        reveal(inflate_verus::model::MOneBlock);
    }
    assert(inflate_verus::model::MBlk(z,16,empty,limit,32768,true)==fault) by {
        reveal(inflate_verus::model::MBlk);
    }
    assert(inflate_verus::model::MEnvelope(z,limit,pol.window==crate::policy::Window::Enforce,
        pol.zlib_tail==crate::policy::ZlibTail::IgnoreTail)==fault) by {
        reveal(inflate_verus::model::MEnvelope);
    }
    assert(inflate_verus::model::Inflate(z,limit,pol.window==crate::policy::Window::Enforce,
        pol.zlib_tail==crate::policy::ZlibTail::IgnoreTail).status
        ==inflate_verus::model::InflateStatus::Error(inflate_verus::model::InflateFault::TruncatedZlib)) by {
        reveal(inflate_verus::model::Inflate);
    }
}
fn image_status(status: Status, produced: u64, r: u64) -> (result: Result<(), Error>)
    ensures result.is_ok() ==> produced>=r,
        match status {Status::Err(e)=>result==Err(crate::icc_model::inflate_error(e)),_=>true},
{
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

/// Absolute value only in the proved Paeth arithmetic domain.
fn abs_small(v: i16) -> (r: i16)
    requires -510<=v<=510,
    ensures 0<=r<=510,
{if v<0 {-v}else{v}}

#[inline]
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i16 + b as i16 - c as i16;
    let pa = abs_small(p - a as i16);
    let pb = abs_small(p - b as i16);
    let pc = abs_small(p - c as i16);
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
fn unfilter(ft: u8, cur: &mut [u8], prev: Option<&[u8]>, bpp: usize)
    requires 1<=bpp,match prev {Some(p)=>old(cur).len()<=p.len(),None=>true},
    ensures final(cur).len()==old(cur).len(),
{
    let n = cur.len();
    let mut i = 0;
    while i < n
        invariant i<=n,n==cur.len(),1<=bpp,match prev {Some(p)=>n<=p.len(),None=>true},
        decreases n-i,
    {
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

proof fn packed_access(n: usize,i: usize,depth: u8)
    requires depth==1 || depth==2 || depth==4,(i as int+1)*(depth as int)<=8*(n as int),
    ensures 1<=8int/(depth as int),(i as int)/(8int/(depth as int))<n,
        1<=(depth as int)*((i as int)%(8int/(depth as int))+1)<=8,
{
    if depth==1 {assert(i/8<n);}
    else if depth==2 {assert(i/4<n);}
    else {assert(depth==4);assert(i/2<n);}
}

/// P13.1: sample `i` of a reconstructed scanline.
#[inline]
fn sample(line: &[u8], depth: u8, i: usize) -> (r: u16)
    requires depth==1 || depth==2 || depth==4 || depth==8 || depth==16,
        (i as int+1)*(depth as int)<=8*(line.len() as int),
    ensures r as u32<=(1u32<<depth)-1,
{
    proof {
        assert((depth==16 ==> (1u32<<depth)==u16::MAX as u32+1) && (depth==8 ==> (1u32<<depth)==256u32)) by (bit_vector);
    }
    match depth {
        16 => be16(line, 2 * i),
        8 => line[i] as u16,
        _ => {
            let per = 8 / depth as usize;
            proof {
                assert(depth==1 || depth==2 || depth==4);
                packed_access(line.len(),i,depth);
                assert(per>=1);
                assert(i/per<line.len());
                assert(1<=depth as int*((i%per) as int+1)<=8);
                assert((1u8<<depth)>0) by (bit_vector) requires 1<=depth<=4;
                assert(((1u8<<depth)-1) as u32==(1u32<<depth)-1) by (bit_vector) requires 1<=depth<=4;
            }
            let byte = line[i / per];
            let shift = 8 - depth as usize * (i % per + 1);
            let mask=(1u8<<depth)-1;
            let value=(byte>>shift)&mask;
            proof {assert(value<=mask) by (bit_vector) requires value==((byte>>shift)&mask),shift<8;}
            value as u16
        }
    }
}

/// Safety-only channel-index factoring; no sample-value equation is claimed.
fn sample_channel(line: &[u8],depth: u8,x: usize,channels: usize,channel: usize) -> (r: u16)
    requires depth==1 || depth==2 || depth==4 || depth==8 || depth==16,
        1<=channels<=4,channel<channels,4*(x as int+1)<=crate::alloc_guard::MAX_BYTES,
        (x as int+1)*(channels as int)*(depth as int)<=8*(line.len() as int),
    ensures r as u32<=(1u32<<depth)-1,
{
    proof {
        crate::limits::caps_ok();
        assert((x as int)*(channels as int)+(channel as int)<4*(x as int+1)) by (nonlinear_arith)
            requires 0<=x,1<=channels<=4,0<=channel<channels;
        assert((x as int*channels as int+channel as int+1)*(depth as int)
            <=(x as int+1)*(channels as int)*(depth as int)) by (nonlinear_arith)
            requires 0<=x,0<=depth,0<=channel<channels;
    }
    sample(line,depth,channels*x+channel)
}

/// P13.2 `s8` and P13.3 `s16`.
#[inline]
fn scale(v: u16, depth: u8, wide: bool) -> (r: u16)
    requires 1<=depth<=16,v as u32<=(1u32<<depth)-1,
    ensures r<=if wide {u16::MAX}else{255},
{
    proof {assert(2<=(1u32<<depth)) by (bit_vector) requires 1<=depth<=16;}
    let v = v as u32;
    let denominator=(1u32<<depth)-1;
    proof {
        let top=if wide {65535int}else{255int};
        assert(v as int*top<=denominator as int*top) by (nonlinear_arith) requires 0<=v<=denominator,0<=top;
        vstd::arithmetic::div_mod::lemma_div_is_ordered(v as int*top,denominator as int*top,denominator as int);
        vstd::arithmetic::div_mod::lemma_div_multiples_vanish(top,denominator as int);
    }
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
    closed spec fn wf(self) -> bool {crate::model::valid_depth(self.ct,self.depth)}
    closed spec fn channels(self) -> nat {if self.ct==0 || self.ct==3 {1nat}else if self.ct==2 {3nat}else if self.ct==4 {2nat}else{4nat}}
    fn new(doc: &Doc, b: &'b [u8]) -> (r: Colour<'b>)
        requires doc.wf(b@),
        ensures r.wf(),r.ct==doc.ihdr.ct,r.depth==doc.ihdr.depth,
    {
        let plte = match doc.plte {
            Some((o, n)) => {
                assert(o as int+n as int<=b.len());
                &b[o..o+n]
            },
            None => &b[0..0],
        };
        let trns=match doc.trns {Some((o,n))=>{
            assert(o as int+n as int<=b.len());Some(&b[o..o+n])
        },None=>None};
        let mut key = [0u16; 3];
        if let Some(t) = trns {
            if doc.ihdr.ct == 0 || doc.ihdr.ct == 2 {
                let depth=doc.ihdr.depth;
                proof {assert(2<=(1u32<<depth)) by (bit_vector) requires 1<=depth<=16;}
                let mask: u32 = (1u32 << depth) - 1;
                let mut j = 0;
                while j + 1 < t.len() && j / 2 < 3
                    invariant j<=6,j%2==0,
                    decreases 6-j,
                {
                    let word=be16(t,j);
                    let masked=word as u32&mask;
                    proof {assert(masked<=word as u32) by (bit_vector) requires masked==(word as u32&mask);}
                    key[j/2]=masked as u16;
                    j += 2;
                }
            }
        }
        Colour { ct: doc.ihdr.ct, depth: doc.ihdr.depth, plte, trns, key }
    }

    /// P13.2/P13.3: pixel `x` of a reconstructed scanline.
    #[inline]
    fn pixel(&self, line: &[u8], x: usize, wide: bool, pol: &Policy, ev: &mut Events) -> (r: Result<[u16; 4], Error>)
        requires self.wf(),4*(x as int+1)<=crate::alloc_guard::MAX_BYTES,
            (x as int+1)*self.channels()*(self.depth as int)<=8*(line.len() as int),
        ensures final(ev).r1==old(ev).r1,final(ev).r4==old(ev).r4,
            match r {Ok(px)=>forall|i: int| 0<=i<4 ==> (#[trigger] px@[i])<=if wide {u16::MAX}else{255},Err(_)=>true},
    {
        proof {crate::limits::caps_ok();}

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
                let r = sample_channel(line,d,x,3,0);
                let g = sample_channel(line,d,x,3,1);
                let bl = sample_channel(line,d,x,3,2);
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
                let g = scale(sample_channel(line,d,x,2,0), d, wide);
                let a = scale(sample_channel(line,d,x,2,1), d, wide);
                Ok([g, g, g, a])
            }
            _ => Ok([
                scale(sample_channel(line,d,x,4,0), d, wide),
                scale(sample_channel(line,d,x,4,1), d, wide),
                scale(sample_channel(line,d,x,4,2), d, wide),
                scale(sample_channel(line,d,x,4,3), d, wide),
            ]),
        }
    }
}

fn pass_coordinate(n: u32,origin: u32,step: u32,index: u32) -> (r: u32)
    requires step>=1,index<layout::span(n as nat,origin as nat,step as nat),
    ensures r<n,
{
    proof {layout::span_index(n as nat,origin as nat,step as nat,index as nat);}
    origin+index*step
}

/// Read a reconstructed row and write only within the selected destination.
fn draw_row(colour: &Colour<'_>,line: &[u8],w: u32,h: u32,x0: u32,dx: u32,pw: u32,y: u32,
    wide: bool,pol: &Policy,ev: &mut Events,sink: &mut Sink<'_>) -> (result: Result<(),Error>)
    requires colour.wf(),old(sink).wf(w,h),wide==old(sink).wide_view(),
        1<=pw<=w,1<=dx<=8,y<h,4*(w as int)<=crate::alloc_guard::MAX_BYTES,
        pw==layout::span(w as nat,x0 as nat,dx as nat),
        line.len()==layout::scan_bytes(colour.channels()*(colour.depth as nat),pw as nat),
    ensures final(sink).shape()==old(sink).shape(),final(sink).wf(w,h),final(sink).loan()==old(sink).loan(),
        final(ev).r1==old(ev).r1,final(ev).r4==old(ev).r4,
{
    let mut c=0u32;
    while c<pw
        invariant c<=pw,colour.wf(),sink.wf(w,h),sink.shape()==old(sink).shape(),sink.loan()==old(sink).loan(),wide==sink.wide_view(),
            1<=pw<=w,1<=dx<=8,y<h,4*(w as int)<=crate::alloc_guard::MAX_BYTES,
            pw==layout::span(w as nat,x0 as nat,dx as nat),
            line.len()==layout::scan_bytes(colour.channels()*(colour.depth as nat),pw as nat),
            ev.r1==old(ev).r1,ev.r4==old(ev).r4,
        decreases pw-c,
    {
        proof {
            assert((pw as int)*(colour.channels()*(colour.depth as nat))<=8*(line.len() as int));
            assert((c as int+1)*colour.channels()*(colour.depth as int)<=8*(line.len() as int)) by (nonlinear_arith)
                requires c<pw,(pw as int)*(colour.channels()*(colour.depth as nat))<=8*(line.len() as int),
                    colour.channels()>=1,colour.depth>=1;
            layout::span_index(w as nat,x0 as nat,dx as nat,c as nat);
            assert((x0 as int)+(c as int)*(dx as int)<w);
        }
        let px=match colour.pixel(line,c as usize,wide,pol,ev) {Ok(px)=>px,Err(e)=>return Err(e)};
        sink.put(w,h,x0+c*dx,y,px);
        c+=1;
    }
    Ok(())
}

proof fn row_extent(base: nat,rows: nat,bytes: nat,row: nat,limit: nat)
    requires row<rows,base+rows*(1+bytes)<=limit,
    ensures base+row*(1+bytes)+1+bytes<=limit,
        row>0 ==> bytes<=base+row*(1+bytes),
{
    assert((row+1)*(1+bytes)<=rows*(1+bytes)) by (nonlinear_arith) requires row<rows,bytes>=0;
    assert(row*(1+bytes)+1+bytes==(row+1)*(1+bytes)) by (nonlinear_arith);
    assert(row>0 ==> bytes<=base+row*(1+bytes)) by (nonlinear_arith)
        requires row>=0,base>=0,bytes>=0;
}

/// Weak safety/termination continuation; no filter/sample/pixel equation.
fn decode_exact<S: Segments>(
    doc: &Doc,b: &[u8],src: &mut S,raw: &mut [u8],w: u32,h: u32,
    pol: &Policy,ev: &mut Events,sink: &mut Sink<'_>,
) -> (result: Result<(),Error>)
    requires doc.wf(b@),doc.ihdr.wf(),1<=w<=doc.ihdr.w,1<=h<=doc.ihdr.h,
        old(src).wf(),old(src).pos()==0,old(sink).wf(w,h),
        layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat)==old(raw).len(),
    ensures final(src).wf(),final(src).concat()==old(src).concat(),
        final(src).source_frame()==old(src).source_frame(),final(src).seg_count()==old(src).seg_count(),
        final(src).pos()<=final(src).concat().len(),final(raw).len()==old(raw).len(),
        final(sink).shape()==old(sink).shape(),final(sink).wf(w,h),final(sink).loan()==old(sink).loan(),
        final(ev).r1==old(ev).r1,final(ev).r4==old(ev).r4,
        match inflate_error(old(src).concat(),layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat),*pol) {
            Some(e)=>result==Err(e),None=>true},
{
    hide(Doc::wf);hide(inflate_verus::model::Inflate);
    let ihdr=&doc.ihdr;
    let r=raw_size(ihdr,w,h);
    proof {crate::limits::caps_ok();}
    let ghost data_fault=inflate_error(old(src).concat(),layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat),*pol);
    let res=inflate_zlib_into(src,raw,pol.inflate_options());
    let checked=image_status(res.status,res.produced,r);
    proof {
        let expected=inflate_verus::model::Inflate(old(src).concat(),r as nat,
            pol.window==crate::policy::Window::Enforce,pol.zlib_tail==crate::policy::ZlibTail::IgnoreTail);
        if let inflate_verus::model::InflateStatus::Error(f)=expected.status {
            match res.status {Status::Err(e)=>{crate::icc_model::fault_agrees(e,f);},_=>{assert(false);},}
            assert(checked==Err(crate::icc_model::fault(f)));
        }
    }
    match checked {Err(e)=>return Err(e),Ok(())=>{}}
    assert(data_fault==None);
    proof {
        assert(4*(w as int)<=crate::alloc_guard::MAX_BYTES) by (nonlinear_arith)
            requires 1<=w<=ihdr.w,1<=ihdr.h,4*(ihdr.w as int)*(ihdr.h as int)<=crate::limits::MAX_OUTPUT,
                crate::limits::MAX_OUTPUT<=crate::alloc_guard::MAX_BYTES;
    }
    let bpp=ihdr.bpp();
    let wide=sink.wide();
    let colour=Colour::new(doc,b);
    let mut pos=0usize;
    let mut k=0usize;
    while k<pass_count(ihdr.interlace)
        invariant doc.wf(b@),ihdr==&doc.ihdr,ihdr.wf(),1<=w<=ihdr.w,1<=h<=ihdr.h,
            k<=layout::count(ihdr.interlace),raw.len()==r,r==old(raw).len(),
            r==layout::raw(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat),r<=crate::alloc_guard::MAX_BYTES,
            pos==layout::prefix(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat,k as nat),
            colour.wf(),colour.ct==ihdr.ct,colour.depth==ihdr.depth,bpp>=1,4*(w as int)<=crate::alloc_guard::MAX_BYTES,
            sink.wf(w,h),sink.shape()==old(sink).shape(),sink.loan()==old(sink).loan(),wide==sink.wide_view(),
            ev.r1==old(ev).r1,ev.r4==old(ev).r4,
            data_fault==None,
            data_fault==inflate_error(old(src).concat(),layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat),*pol),
            src.wf(),src.concat()==old(src).concat(),src.source_frame()==old(src).source_frame(),
            src.seg_count()==old(src).seg_count(),src.pos()<=src.concat().len(),
        decreases layout::count(ihdr.interlace)-k,
    {
        if let Some(p)=pass(ihdr.interlace,w,h,k) {
            let bytes=row_bytes(ihdr,p.pw);
            let ghost base=pos as nat;
            proof {
                reveal_with_fuel(layout::prefix,2);
                layout::monotone(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat,(k+1) as nat,layout::count(ihdr.interlace));
                assert(layout::contribution(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat,k as nat)
                    ==p.ph as int*(1+bytes as int));
                assert(base+p.ph as int*(1+bytes as int)<=r);
                assert(bytes<r) by (nonlinear_arith) requires base>=0,p.ph>=1,base+p.ph as int*(1+bytes as int)<=r;
            }
            let rb=bytes as usize;
            let mut row=0u32;
            while row<p.ph
                invariant doc.wf(b@),ihdr==&doc.ihdr,ihdr.wf(),1<=w<=ihdr.w,1<=h<=ihdr.h,
                    raw.len()==r,r==old(raw).len(),r<=crate::alloc_guard::MAX_BYTES,k<layout::count(ihdr.interlace),
                    base==layout::prefix(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat,k as nat),
                    row<=p.ph,1<=p.pw<=w,1<=p.ph<=h,1<=p.dx<=8,1<=p.dy<=8,
                    p.pw==layout::span(w as nat,p.x0 as nat,p.dx as nat),p.ph==layout::span(h as nat,p.y0 as nat,p.dy as nat),
                    rb==layout::scan_bytes(layout::bits(ihdr.view()),p.pw as nat),
                    base+p.ph as int*(1+rb as int)<=r,pos==base+row as int*(1+rb as int),
                    layout::prefix(ihdr.interlace,layout::bits(ihdr.view()),w as nat,h as nat,(k+1) as nat)==base+p.ph as int*(1+rb as int),
                    ihdr.interlace==0 ==> (p.x0==0 && p.y0==0 && p.dx==1 && p.dy==1 && p.pw==w && p.ph==h),
                    ihdr.interlace!=0 ==> ((p.x0 as nat,p.y0 as nat,p.dx as nat,p.dy as nat)==layout::geometry(k as nat)
                        && p.pw==layout::cols(w as nat,k as nat) && p.ph==layout::rows(h as nat,k as nat)),
                    colour.wf(),colour.ct==ihdr.ct,colour.depth==ihdr.depth,bpp>=1,4*(w as int)<=crate::alloc_guard::MAX_BYTES,
                    sink.wf(w,h),sink.shape()==old(sink).shape(),sink.loan()==old(sink).loan(),wide==sink.wide_view(),
                    ev.r1==old(ev).r1,ev.r4==old(ev).r4,
                    data_fault==None,
                    data_fault==inflate_error(old(src).concat(),layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat),*pol),
                    src.wf(),src.concat()==old(src).concat(),src.source_frame()==old(src).source_frame(),
                    src.seg_count()==old(src).seg_count(),src.pos()<=src.concat().len(),
                decreases p.ph-row,
            {
                proof {row_extent(base,p.ph as nat,rb as nat,row as nat,r as nat);}
                let ft=raw[pos];
                if ft>4 {return Err(Error::InvalidFilterType);}
                let (before,rest)=raw.split_at_mut(pos+1);
                let cur=&mut rest[..rb];
                let prev=if row>0 {Some(&before[pos-rb..pos])}else{None};
                unfilter(ft,cur,prev,bpp);
                proof {
                    assert(p.ph==layout::span(h as nat,p.y0 as nat,p.dy as nat));
                    assert(p.pw==layout::span(w as nat,p.x0 as nat,p.dx as nat));
                }
                let y=pass_coordinate(h,p.y0,p.dy,row);
                let row_result=draw_row(&colour,cur,w,h,p.x0,p.dx,p.pw,y,wide,pol,ev,sink);
                match row_result {Err(e)=>return Err(e),Ok(())=>{}}
                proof {
                    assert(base+row as int*(1+rb as int)+1+rb as int==base+(row as int+1)*(1+rb as int)) by (nonlinear_arith);
                }
                pos+=1+rb;
                row+=1;
            }
        } else {
            proof {reveal_with_fuel(layout::prefix,2);}
        }
        k+=1;
    }
    Ok(())
}

/// Restrict writes to the proved Raw prefix without changing the caller's buffer length.
pub(crate) fn decode_image<S: Segments>(
    doc: &Doc,b: &[u8],src: &mut S,raw: &mut [u8],w: u32,h: u32,
    pol: &Policy,ev: &mut Events,sink: &mut Sink<'_>,
) -> (result: Result<(),Error>)
    requires doc.wf(b@),doc.ihdr.wf(),1<=w<=doc.ihdr.w,1<=h<=doc.ihdr.h,
        old(src).wf(),old(src).pos()==0,old(sink).wf(w,h),
        layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat)<=old(raw).len(),
    ensures final(src).wf(),final(src).concat()==old(src).concat(),
        final(src).source_frame()==old(src).source_frame(),final(src).seg_count()==old(src).seg_count(),
        final(src).pos()<=final(src).concat().len(),final(raw).len()==old(raw).len(),
        final(sink).shape()==old(sink).shape(),final(sink).wf(w,h),final(sink).loan()==old(sink).loan(),
        final(ev).r1==old(ev).r1,final(ev).r4==old(ev).r4,
        match inflate_error(old(src).concat(),layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr.view()),w as nat,h as nat),*pol) {
            Some(e)=>result==Err(e),None=>true},
{
    hide(Doc::wf);hide(inflate_verus::model::Inflate);
    let r=raw_size(&doc.ihdr,w,h);
    proof {crate::limits::caps_ok();}
    decode_exact(doc,b,src,&mut raw[..r as usize],w,h,pol,ev,sink)
}

} // verus!
