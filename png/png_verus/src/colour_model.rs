//! Independent P13 sample-scaling equations, over mathematical integers.
//! Legal PNG depths are a domain restriction, not an executable dispatch table.
use vstd::prelude::*;
use vstd::arithmetic::power2::{pow2,lemma2_to64};
use vstd::arithmetic::div_mod::*;

verus! {

pub open spec fn depth(d: u8) -> bool {d==1 || d==2 || d==4 || d==8 || d==16}
pub open spec fn maximum(d: u8) -> int {pow2(d as nat) as int-1}
pub open spec fn s8(v: int,d: u8) -> int {
    if d==16 {(v+128)/257}else{v*255/maximum(d)}
}
pub open spec fn s16(v: int,d: u8) -> int {v*(u16::MAX as int)/maximum(d)}

/// Connect the mathematical power with each legal wire depth and its bit mask.
pub proof fn depth_values(d: u8)
    requires depth(d),
    ensures maximum(d)==if d==1 {1int}else if d==2 {3int}else if d==4 {15int}else if d==8 {255int}else{u16::MAX as int},
        maximum(d)==((1u32<<d)-1) as int,
        1<=maximum(d)<=u16::MAX,
{
    lemma2_to64();
    assert(((1u32<<d)-1)==if d==1 {1u32}else if d==2 {3u32}else if d==4 {15u32}else if d==8 {255u32}else{u16::MAX as u32}) by (bit_vector)
        requires d==1 || d==2 || d==4 || d==8 || d==16;
}

/// An integral scale is multiplication by its exact, nonnegative factor.
proof fn integral_scale(v: int,top: int,den: int,factor: int)
    requires den>0,factor>=0,top==den*factor,0<=v<=den,
    ensures (v*top)%den==0,(v*top)/den==v*factor,
        ((v*top)/den)*den==v*top,0<=(v*top)/den<=top,
{
    assert(v*top==(v*factor)*den && v*top==den*(v*factor)) by (nonlinear_arith)
        requires top==den*factor;
    lemma_div_multiples_vanish(v*factor,den);
    lemma_mod_multiples_basic(v*factor,den);
    assert(0<=v*factor<=top) by (nonlinear_arith)
        requires den>0,factor>=0,top==den*factor,0<=v<=den;
}

/// P13.2: every low-depth scale is integral, including zero remainder.
pub proof fn scale8_exact(v: int,d: u8)
    requires depth(d),d<=8,0<=v<=maximum(d),
    ensures (v*255)%maximum(d)==0,
        s8(v,d)*maximum(d)==v*255,0<=s8(v,d)<=255,
{
    depth_values(d);
    if d==1 {integral_scale(v,255,maximum(d),255);}
    else if d==2 {integral_scale(v,255,maximum(d),85);}
    else if d==4 {integral_scale(v,255,maximum(d),17);}
    else {assert(d==8);integral_scale(v,255,maximum(d),1);}
}

/// P13.3: scaling to sixteen bits is integral at all five PNG depths.
pub proof fn scale16_exact(v: int,d: u8)
    requires depth(d),0<=v<=maximum(d),
    ensures (v*(u16::MAX as int))%maximum(d)==0,
        s16(v,d)*maximum(d)==v*(u16::MAX as int),0<=s16(v,d)<=u16::MAX,
{
    depth_values(d);
    if d==1 {integral_scale(v,u16::MAX as int,maximum(d),u16::MAX as int);}
    else if d==2 {integral_scale(v,u16::MAX as int,maximum(d),21845);}
    else if d==4 {integral_scale(v,u16::MAX as int,maximum(d),4369);}
    else if d==8 {integral_scale(v,u16::MAX as int,maximum(d),257);}
    else {assert(d==16);integral_scale(v,u16::MAX as int,maximum(d),1);}
}

/// P13.2: half-up rounding equals the simplified 257-denominator identity.
pub proof fn round16(v: int)
    requires 0<=v<=u16::MAX,
    ensures (510*v+u16::MAX as int)/131070==(v+128)/257,
        0<=s8(v,16)<=255,
{}

/// L-16-8's low-depth channel relation, not yet its integrated still theorem.
pub proof fn scale16_257(v: int,d: u8)
    requires depth(d),d<=8,0<=v<=maximum(d),
    ensures s16(v,d)==257*s8(v,d),
{
    depth_values(d);scale8_exact(v,d);scale16_exact(v,d);
    assert(s16(v,d)==257*s8(v,d)) by (nonlinear_arith)
        requires maximum(d)>0,s16(v,d)*maximum(d)==v*(u16::MAX as int),
            s8(v,d)*maximum(d)==v*255;
}


/// Borrowed P13 conversion inputs, independent of the executable's cached key.
pub struct Context {
    pub ct: u8,
    pub depth: u8,
    pub plte: Seq<u8>,
    pub trns: Option<Seq<u8>>,
}
pub open spec fn from_doc(doc: crate::document_model::Doc,b: Seq<u8>) -> Context {
    Context {ct:doc.ihdr.color_type,depth:doc.ihdr.depth,
        plte:match doc.plte {Some(r)=>b.subrange(r.offset as int,(r.offset+r.len) as int),None=>Seq::empty()},
        trns:match doc.trns {Some(r)=>Some(b.subrange(r.offset as int,(r.offset+r.len) as int)),None=>None}}
}
pub open spec fn channels(c: Context) -> nat {
    if c.ct==0 || c.ct==3 {1} else if c.ct==2 {3} else if c.ct==4 {2} else {4}
}

/// P13.1: big-endian words or most-significant-bit-first packed samples.
pub open spec fn sample(line: Seq<u8>,d: u8,i: nat) -> u16 {
    if d==16 {crate::model::be16(line,2*i as int)}
    else {crate::raster_model::palette_sample(line,d,i) as u16}
}

/// The tRNS comparison uses the low d bits of each complete, unscaled key.
pub open spec fn key(c: Context,j: nat) -> u16 {
    if c.ct==0 || c.ct==2 {
        match c.trns {
            Some(t) if j<3 && 2*j+1<t.len()=>
                (crate::model::be16(t,2*j as int) as u32 & (((1u32<<c.depth)-1) as u32)) as u16,
            _=>0,
        }
    } else {0}
}
pub open spec fn top(wide: bool) -> u16 {if wide {u16::MAX}else{u8::MAX as u16}}
pub open spec fn scaled(v: u16,d: u8,wide: bool) -> u16 {
    (if wide {s16(v as int,d)}else{s8(v as int,d)}) as u16
}

/// MC8/MC16's single-pixel equations; all reads are from immutable inputs.
pub open spec fn pixel(c: Context,line: Seq<u8>,x: nat,wide: bool,black: bool)
    -> Result<Seq<u16>,crate::png::Error>
{
    if c.ct==0 {
        let g=sample(line,c.depth,x);
        let a=if c.trns.is_some() && g==key(c,0) {0u16}else{top(wide)};
        let s=scaled(g,c.depth,wide);
        Ok(seq![s,s,s,a])
    } else if c.ct==2 {
        let r=sample(line,c.depth,3*x);
        let g=sample(line,c.depth,3*x+1);
        let b=sample(line,c.depth,3*x+2);
        let a=if c.trns.is_some() && r==key(c,0) && g==key(c,1) && b==key(c,2) {0u16}else{top(wide)};
        Ok(seq![scaled(r,c.depth,wide),scaled(g,c.depth,wide),scaled(b,c.depth,wide),a])
    } else if c.ct==3 {
        let i=sample(line,c.depth,x) as nat;
        if i>=c.plte.len()/3 {
            if black {Ok(seq![0u16,0u16,0u16,top(wide)])}
            else {Err(crate::png::Error::PaletteIndexOutOfRange)}
        } else {
            let a=match c.trns {Some(t) if i<t.len()=>t[i as int],_=>255u8};
            let m=if wide {257u16}else{1u16};
            Ok(seq![(c.plte[(3*i) as int] as u16*m) as u16,(c.plte[(3*i+1) as int] as u16*m) as u16,
                (c.plte[(3*i+2) as int] as u16*m) as u16,(a as u16*m) as u16])
        }
    } else if c.ct==4 {
        let g=scaled(sample(line,c.depth,2*x),c.depth,wide);
        let a=scaled(sample(line,c.depth,2*x+1),c.depth,wide);
        Ok(seq![g,g,g,a])
    } else {
        Ok(seq![scaled(sample(line,c.depth,4*x),c.depth,wide),
            scaled(sample(line,c.depth,4*x+1),c.depth,wide),
            scaled(sample(line,c.depth,4*x+2),c.depth,wide),
            scaled(sample(line,c.depth,4*x+3),c.depth,wide)])
    }
}

/// Destination geometry: kind (RGBA8, RGBA16, canvas), length, canvas width,
/// frame x/y offsets and blend-over flag. This is an abstract mathematical
/// destination, not the executable Sink enum or a mutable output buffer.
pub open spec fn destination_offset(shape: (nat,nat,nat,nat,nat,bool),width: nat,x: nat,y: nat) -> nat {
    if shape.0==2 {4*((shape.4+y)*shape.2+shape.3+x)}
    else {(if shape.0==1 {8nat}else{4nat})*(y*width+x)}
}
/// The two physical bytes preserve every bit of an MC16 channel.
pub proof fn physical_word(w: u16)
    ensures w>>8<=u8::MAX as u16,(w&255)<=u8::MAX as u16,
        w as nat==256*((w>>8) as u8 as nat)+(w&255) as u8 as nat,
{
    assert(w>>8<=u8::MAX as u16 && (w&255)<=u8::MAX as u16) by (bit_vector);
    assert(w as u32==256u32*((w>>8) as u32)+(w&255) as u32) by (bit_vector);
}
pub open spec fn physical(px: Seq<u16>,wide: bool) -> Seq<u8> {
    if wide {Seq::new(8,|i: int| if i%2==0 {(px[i/2]>>8) as u8}else{(px[i/2]&255) as u8})}
    else {Seq::new(4,|i: int| px[i] as u8)}
}
pub open spec fn place(shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x: nat,y: nat,px: Seq<u16>) -> Seq<u8> {
    let o=destination_offset(shape,width,x,y);
    let data=if shape.0==2 && shape.5 {
        crate::composite_model::over(physical(px,false),before.subrange(o as int,(o+4) as int))
    }else{physical(px,shape.0==1)};
    Seq::new(before.len(),|j: int| if o<=j<o+data.len() {data[j-o]}else{before[j]})
}

/// Every byte of a placed canvas pixel lies inside the frame rectangle.
pub proof fn place_drawing_frame(shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,w: nat,h: nat,x: nat,y: nat,px: Seq<u16>)
    requires x<w,y<h,px.len()==4,before.len()==shape.1,
        shape.0==2 ==> shape.2>=1 && shape.3+w<=shape.2,
    ensures crate::composite_model::drawing_frame(before,place(shape,before,w,x,y,px),shape,w,h),
{
    let after=place(shape,before,w,x,y,px);
    let o=destination_offset(shape,w,x,y);
    assert forall|j: int| 0<=j<before.len() && !crate::composite_model::drawing_region(shape,w,h,j)
        implies #[trigger] after[j]==before[j] by {
        if shape.0==2 && o<=j<o+4 {
            let r=crate::composite_model::Rect {x:shape.3,y:shape.4,w,h};
            let col=(4*x+j-o) as nat;
            assert(col<4*w) by (nonlinear_arith) requires x<w,o<=j<o+4,col==4*x+j-o;
            let base=crate::composite_model::row_base(shape.2,r,y);
            assert(base==4*((shape.4+y)*shape.2+shape.3));
            assert(base+col==j) by (nonlinear_arith)
                requires o==4*((shape.4+y)*shape.2+shape.3+x),col==4*x+j-o,
                    base==4*((shape.4+y)*shape.2+shape.3);
            crate::composite_model::rectangle_access(shape.2,r,y,col);
        }
    }
    crate::composite_model::drawing_frame_intro(before,after,shape,w,h);
}

/// The mathematical row conversion: first palette error, or one RGBA value
/// per pixel. The input sample width decides which bits are pixels; trailing
/// padding never becomes another sample.
pub open spec fn MC(c: Context,line: Seq<u8>,width: nat,wide: bool,black: bool)
    -> Result<Seq<Seq<u16>>,crate::png::Error>
{
    if c.ct==3 && !black && !crate::raster_model::palette_valid(line,c.depth,width,c.plte.len()/3) {
        Err(crate::png::Error::PaletteIndexOutOfRange)
    } else {
        Ok(Seq::new(width,|x: int| match pixel(c,line,x as nat,wide,black) {Ok(px)=>px,_=>Seq::empty()}))
    }
}

/// Place a prefix of MC's row in scanline order, using immutable canvas values
/// at each blend. Error rows have no successful output equation.
pub closed spec fn draw_prefix(c: Context,line: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x0: nat,dx: nat,y: nat,n: nat,wide: bool,black: bool) -> Seq<u8>
    decreases n,
{
    if n==0 {before} else {
        let k=(n-1) as nat;
        let p=draw_prefix(c,line,shape,before,width,x0,dx,y,k,wide,black);
        match pixel(c,line,k,wide,black) {
            Ok(px)=>place(shape,p,width,x0+k*dx,y,px),
            _=>p,
        }
    }
}
pub proof fn draw_start(c: Context,line: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x0: nat,dx: nat,y: nat,wide: bool,black: bool)
    ensures draw_prefix(c,line,shape,before,width,x0,dx,y,0,wide,black)==before,
{reveal(draw_prefix);}
pub proof fn draw_step(c: Context,line: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x0: nat,dx: nat,y: nat,n: nat,wide: bool,black: bool,px: Seq<u16>)
    requires pixel(c,line,n,wide,black)==Ok(px),
    ensures draw_prefix(c,line,shape,before,width,x0,dx,y,n+1,wide,black)
        ==place(shape,draw_prefix(c,line,shape,before,width,x0,dx,y,n,wide,black),width,x0+n*dx,y,px),
{reveal_with_fuel(draw_prefix,2);}


/// Every cell of a successful MC row is exactly its independent pixel result.
pub proof fn MC_pixel(c: Context,line: Seq<u8>,count: nat,wide: bool,black: bool,x: nat,rows: Seq<Seq<u16>>)
    requires crate::model::valid_depth(c.ct,c.depth),x<count,MC(c,line,count,wide,black)==Ok(rows),
    ensures rows.len()==count,pixel(c,line,x,wide,black)==Ok(rows[x as int]),rows[x as int].len()==4,
{
    if c.ct==3 && !black {
        assert(c.depth!=16);
        assert(crate::raster_model::palette_valid(line,c.depth,count,c.plte.len()/3));
        assert((crate::raster_model::palette_sample(line,c.depth,x) as nat)<c.plte.len()/3);
    }
    assert(pixel(c,line,x,wide,black).is_ok());
}

/// Apply the successful MC row values, independently of sample conversion.
pub closed spec fn render_row(rows: Seq<Seq<u16>>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x0: nat,dx: nat,y: nat,n: nat) -> Seq<u8>
    decreases n,
{
    if n==0 {before} else {
        let k=(n-1) as nat;
        place(shape,render_row(rows,shape,before,width,x0,dx,y,k),width,x0+k*dx,y,rows[k as int])
    }
}

/// The actual row loop's prefix equation is the rendering of MC8/MC16.
pub proof fn draw_matches_MC(c: Context,line: Seq<u8>,count: nat,rows: Seq<Seq<u16>>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,width: nat,x0: nat,dx: nat,y: nat,n: nat,wide: bool,black: bool)
    requires crate::model::valid_depth(c.ct,c.depth),n<=count,MC(c,line,count,wide,black)==Ok(rows),
    ensures draw_prefix(c,line,shape,before,width,x0,dx,y,n,wide,black)==render_row(rows,shape,before,width,x0,dx,y,n),
    decreases n,
{
    reveal_with_fuel(draw_prefix,2);reveal_with_fuel(render_row,2);
    if n>0 {
        let k=(n-1) as nat;
        MC_pixel(c,line,count,wide,black,k,rows);
        draw_matches_MC(c,line,count,rows,shape,before,width,x0,dx,y,k,wide,black);
    }
}

}
