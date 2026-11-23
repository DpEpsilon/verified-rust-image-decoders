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
/// Length rules retained by every validated policy. PaletteSize::Allow can
/// exceed 2^depth entries, but never 256; MC has no palette-size policy input.
pub open spec fn chunk_lengths(ct: u8,depth: u8,palette: nat,trns: Option<nat>) -> bool {
    crate::model::valid_depth(ct,depth)
    && (if ct==0 || ct==4 {palette==0}else{palette<=768 && palette%3==0})
    && match trns {
        None=>true,
        Some(n)=>if ct==0 {n==2}else if ct==2 {n==6}else if ct==3 {1<=n<=palette/3}else{false},
    }
}
pub open spec fn context_domain(c: Context) -> bool {
    chunk_lengths(c.ct,c.depth,c.plte.len(),match c.trns {Some(t)=>Some(t.len()),None=>None})
    && (c.ct==3 ==> c.plte.len()>0)
}
pub open spec fn row_domain(c: Context,line: Seq<u8>,width: nat) -> bool {
    context_domain(c) && 1<=width<=crate::limits::PNG_WIRE_MAX
        && line.len()==crate::layout_model::scan_bytes(channels(c)*c.depth as nat,width)
}
/// None is the internal OutsideDomain marker, not a PNG format error.
pub open spec fn MC(c: Context,line: Seq<u8>,width: nat,wide: bool,black: bool)
    -> Option<Result<Seq<Seq<u16>>,crate::png::Error>>
{
    if !row_domain(c,line,width) {None}
    else if c.ct==3 && !black && !crate::raster_model::palette_valid(line,c.depth,width,c.plte.len()/3) {
        Some(Err(crate::png::Error::PaletteIndexOutOfRange))
    } else {
        Some(Ok(Seq::new(width,|x: int| match pixel(c,line,x as nat,wide,black) {Ok(px)=>px,_=>Seq::empty()})))
    }
}
pub proof fn MC_empty_rgba_outside()
    ensures MC(Context {ct:6,depth:8,plte:Seq::empty(),trns:None},Seq::empty(),1,false,false)==None,
{}

pub proof fn MC_invalid_context_outside(c: Context,line: Seq<u8>,width: nat,wide: bool,black: bool)
    requires !context_domain(c),
    ensures MC(c,line,width,wide,black)==None,
{}
pub proof fn MC_oversized_width_outside(c: Context,line: Seq<u8>,width: nat,wide: bool,black: bool)
    requires width>crate::limits::PNG_WIRE_MAX,
    ensures MC(c,line,width,wide,black)==None,
{}
pub proof fn MC_empty_grey_trns_outside()
    ensures MC(Context {ct:0,depth:8,plte:Seq::empty(),trns:Some(Seq::empty())},seq![0u8],1,false,false)==None,
        crate::metadata_model::transparency(0,0,0,false)==crate::metadata_model::Decision::Error(crate::png::Error::InvalidTransparency),
{}
pub proof fn MC_matching_oversized_rgba_outside()
    ensures MC(Context {ct:6,depth:8,plte:Seq::empty(),trns:None},
        Seq::new(4*(crate::limits::PNG_WIRE_MAX as nat+1),|i: int|0u8),
        crate::limits::PNG_WIRE_MAX as nat+1,false,false)==None,
{}
pub proof fn MC_policy_large_palette_domain()
    ensures context_domain(Context {ct:3,depth:1,plte:Seq::new(12,|i: int|0u8),trns:None}),
        crate::metadata_model::palette(12,3,1,false,false,false,true)==crate::metadata_model::Decision::Keep(4),
        crate::metadata_model::palette(12,3,1,false,false,false,false)==crate::metadata_model::Decision::Error(crate::png::Error::InvalidPalette),
{
    assert((1u32<<1u8)==2u32) by (compute);
    assert(crate::metadata_model::palette(12,3,1,false,false,false,false)==crate::metadata_model::Decision::Error(crate::png::Error::InvalidPalette)) by (compute);
}
pub proof fn MC_invalid_transparency_lengths(ct: u8,depth: u8,palette: Seq<u8>,trns: Seq<u8>,line: Seq<u8>,width: nat)
    requires (ct==0 && trns.len()!=2) || (ct==2 && trns.len()!=6)
        || (ct==3 && !(1<=trns.len()<=palette.len()/3)) || ct==4 || ct==6,
    ensures MC(Context {ct,depth,plte:palette,trns:Some(trns)},line,width,false,false)==None,
{}
pub proof fn MC_invalid_palette_lengths(ct: u8,depth: u8,palette: Seq<u8>,trns: Option<Seq<u8>>,line: Seq<u8>,width: nat)
    requires ((ct==0 || ct==4) && palette.len()!=0)
        || ((ct==2 || ct==3 || ct==6) && (palette.len()>768 || palette.len()%3!=0))
        || (ct==3 && palette.len()==0),
    ensures MC(Context {ct,depth,plte:palette,trns},line,width,false,false)==None,
{}

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
    requires crate::model::valid_depth(c.ct,c.depth),x<count,MC(c,line,count,wide,black)==Some(Ok(rows)),
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
    requires crate::model::valid_depth(c.ct,c.depth),n<=count,MC(c,line,count,wide,black)==Some(Ok(rows)),
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

/// Logical destination geometry, independent of the executable Sink enum.
pub open spec fn is_wide(shape: (nat,nat,nat,nat,nat,bool)) -> bool {shape.0==1}
pub open spec fn pixel_bytes(shape: (nat,nat,nat,nat,nat,bool)) -> nat {if shape.0==1 {8}else{4}}
pub open spec fn grid_width(shape: (nat,nat,nat,nat,nat,bool),w: nat) -> nat {if shape.0==2 {shape.2}else{w}}
pub open spec fn grid_x(shape: (nat,nat,nat,nat,nat,bool)) -> nat {if shape.0==2 {shape.3}else{0}}
pub open spec fn grid_y(shape: (nat,nat,nat,nat,nat,bool)) -> nat {if shape.0==2 {shape.4}else{0}}
pub open spec fn geometry(shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat,n: nat) -> bool {
    w>=1 && h>=1 && shape.0<=2 && shape.1==n
    && grid_width(shape,w)>=1 && grid_x(shape)+w<=grid_width(shape,w)
    && pixel_bytes(shape)*(grid_y(shape)+h)*grid_width(shape,w)<=n
}
pub open spec fn observe(shape: (nat,nat,nat,nat,nat,bool),bytes: Seq<u8>,w: nat,x: nat,y: nat) -> Seq<u8> {
    let o=destination_offset(shape,w,x,y);
    bytes.subrange(o as int,(o+pixel_bytes(shape)) as int)
}
pub open spec fn converted(shape: (nat,nat,nat,nat,nat,bool),px: Seq<u16>,dest: Seq<u8>) -> Seq<u8> {
    if shape.0==2 && shape.5 {crate::composite_model::over(physical(px,false),dest)}else{physical(px,shape.0==1)}
}
pub proof fn destination_grid(shape: (nat,nat,nat,nat,nat,bool),w: nat,x: nat,y: nat)
    ensures destination_offset(shape,w,x,y)==pixel_bytes(shape)*((grid_y(shape)+y)*grid_width(shape,w)+grid_x(shape)+x),
{
    if shape.0==2 {}else if shape.0==1 {}else{}
}
pub proof fn observed_bounds(shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat,n: nat,x: nat,y: nat)
    requires geometry(shape,w,h,n),x<w,y<h,
    ensures destination_offset(shape,w,x,y)+pixel_bytes(shape)<=n,
{
    let W=grid_width(shape,w);let fx=grid_x(shape);let fy=grid_y(shape);let b=pixel_bytes(shape);
    let index=(fy+y)*W+fx+x;
    assert(index+1<=(fy+h)*W) by (nonlinear_arith)
        requires fx+w<=W,x<w,y<h,W>=1,index==(fy+y)*W+fx+x;
    assert(b*(index+1)<=n) by (nonlinear_arith)
        requires index+1<=(fy+h)*W,b*(fy+h)*W<=n,b>=1;
    destination_grid(shape,w,x,y);
}

/// Distinct pixels have disjoint physical byte intervals, not merely
/// different starting addresses; this covers both four- and eight-byte pixels.
pub proof fn pixel_intervals(shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat,n: nat,x: nat,y: nat,X: nat,Y: nat)
    requires geometry(shape,w,h,n),x<w,y<h,X<w,Y<h,x!=X || y!=Y,
    ensures destination_offset(shape,w,x,y)+pixel_bytes(shape)<=destination_offset(shape,w,X,Y)
        || destination_offset(shape,w,X,Y)+pixel_bytes(shape)<=destination_offset(shape,w,x,y),
{
    let W=grid_width(shape,w);let fx=grid_x(shape);let fy=grid_y(shape);let b=pixel_bytes(shape);
    let i=(fy+y)*W+fx+x;let j=(fy+Y)*W+fx+X;
    if i==j {
        assert(crate::layout_model::canvas_offset(W,fx,fy,x,y)==crate::layout_model::canvas_offset(W,fx,fy,X,Y));
        crate::layout_model::canvas_offset_injective(W,fx,fy,w,h,x,y,X,Y);
        assert(false);
    }
    destination_grid(shape,w,x,y);
    destination_grid(shape,w,X,Y);
    if i<j {assert(b*i+b<=b*j) by (nonlinear_arith) requires i<j,b>=1;}
    else {assert(b*j+b<=b*i) by (nonlinear_arith) requires j<i,b>=1;}
}

/// Observe an actual placement at any image pixel. OVER reads this pixel's
/// old destination; every different pixel is unchanged.
pub proof fn place_observe(shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,w: nat,h: nat,x: nat,y: nat,X: nat,Y: nat,px: Seq<u16>)
    requires geometry(shape,w,h,before.len()),x<w,y<h,X<w,Y<h,px.len()==4,
    ensures observe(shape,place(shape,before,w,x,y,px),w,X,Y)
        ==if x==X && y==Y {converted(shape,px,observe(shape,before,w,X,Y))}else{observe(shape,before,w,X,Y)},
{
    observed_bounds(shape,w,h,before.len(),x,y);observed_bounds(shape,w,h,before.len(),X,Y);
    let after=place(shape,before,w,x,y,px);
    let o=destination_offset(shape,w,x,y);let q=destination_offset(shape,w,X,Y);let b=pixel_bytes(shape);
    if x==X && y==Y {
        assert(observe(shape,after,w,X,Y)=~=converted(shape,px,observe(shape,before,w,X,Y)));
    }else{
        pixel_intervals(shape,w,h,before.len(),x,y,X,Y);
        assert(observe(shape,after,w,X,Y)=~=observe(shape,before,w,X,Y));
    }
}

pub open spec fn row_contains(x0: nat,dx: nat,y: nat,n: nat,X: nat,Y: nat) -> bool {
    Y==y && X%dx==x0 && X/dx<n
}
/// A row places each covered pixel once. In particular its OVER destination
/// is the row's initial pixel, not the result of an earlier row-column write.
pub proof fn render_row_observe(rows: Seq<Seq<u16>>,shape: (nat,nat,nat,nat,nat,bool),before: Seq<u8>,w: nat,h: nat,x0: nat,dx: nat,y: nat,n: nat,X: nat,Y: nat)
    requires geometry(shape,w,h,before.len()),dx>=1,x0<dx,y<h,X<w,Y<h,
        n<=rows.len(),n<=crate::layout_model::span(w,x0,dx),
        forall|c: int| 0<=c<n ==> (#[trigger] rows[c]).len()==4,
    ensures render_row(rows,shape,before,w,x0,dx,y,n).len()==before.len(),
        observe(shape,render_row(rows,shape,before,w,x0,dx,y,n),w,X,Y)
            ==if row_contains(x0,dx,y,n,X,Y) {converted(shape,rows[(X/dx) as int],observe(shape,before,w,X,Y))}
                else{observe(shape,before,w,X,Y)},
    decreases n,
{
    reveal_with_fuel(render_row,2);
    if n>0 {
        let k=(n-1) as nat;
        render_row_observe(rows,shape,before,w,h,x0,dx,y,k,X,Y);
        crate::layout_model::span_index(w,x0,dx,k);
        crate::layout_model::axis_forward(x0,dx,k);
        let prior=render_row(rows,shape,before,w,x0,dx,y,k);
        place_observe(shape,prior,w,h,x0+k*dx,y,X,Y,rows[k as int]);
        if X==x0+k*dx && Y==y {
            assert(!row_contains(x0,dx,y,k,X,Y));
        }else if row_contains(x0,dx,y,n,X,Y) {
            crate::layout_model::axis_forward(x0,dx,X/dx);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(X as int,dx as int);
            assert(X/dx<k);
        }
    }
}

pub(crate) proof fn observed_slice(shape: (nat,nat,nat,nat,nat,bool),state: Seq<u8>,w: nat,x: nat,y: nat)
    ensures observe(shape,state,w,x,y)==state.subrange(destination_offset(shape,w,x,y) as int,
        (destination_offset(shape,w,x,y)+pixel_bytes(shape)) as int),
{}

}
