//! Independent P14.4 straight-alpha OVER; integer equations precede byte casts.
use vstd::prelude::*;
use vstd::arithmetic::div_mod::*;

verus! {

/// THEOREMS.md's fixed format constant, not a resource cap.
pub const BLEND_MAX: u32 = 255;

pub open spec fn alpha(sa: u8,da: u8) -> int {255*(sa as int)+(da as int)*(255-sa as int)}
pub open spec fn numerator(sa: u8,da: u8,sc: u8,dc: u8) -> int {
    255*(sa as int)*(sc as int)+(da as int)*(255-sa as int)*(dc as int)
}

/// Half-up division cannot raise a bounded exact rational above its bound.
proof fn half_up_bound(n: int,d: int,top: int)
    requires d>0,top>=0,0<=n<=top*d,
    ensures 0<=(2*n+d)/(2*d)<=top,
{
    assert(2*n+d<=2*top*d+d) by (nonlinear_arith)
        requires d>0,top>=0,0<=n<=top*d;
    assert(2*top*d+d==top*(2*d)+d) by (nonlinear_arith);
    lemma_fundamental_div_mod_converse_div(2*top*d+d,2*d,top,d);
    lemma_div_is_ordered(2*n+d,2*top*d+d,2*d);
    lemma_div_is_ordered(0,2*n+d,2*d);
}

/// T-OVER's complete intermediate and rounded-result bounds for all bytes.
pub proof fn over_bounds(sa: u8,da: u8,sc: u8,dc: u8)
    requires 1<=sa<255,
    ensures 255<=alpha(sa,da)<=65025,
        0<=numerator(sa,da,sc,dc)<=255*alpha(sa,da),
        0<=2*numerator(sa,da,sc,dc)+alpha(sa,da)<=33227775<u32::MAX,
        0<=2*alpha(sa,da)+255<u32::MAX,
        0<=(2*numerator(sa,da,sc,dc)+alpha(sa,da))/(2*alpha(sa,da))<=BLEND_MAX,
        0<=(2*alpha(sa,da)+255)/510<=BLEND_MAX,
{
    let a=alpha(sa,da);
    let n=numerator(sa,da,sc,dc);
    assert(255<=a<=65025) by (nonlinear_arith)
        requires 1<=sa<255,0<=da<=255,a==255*(sa as int)+(da as int)*(255-sa as int);
    assert(0<=n<=255*a) by (nonlinear_arith)
        requires 1<=sa<255,0<=da<=255,0<=sc<=255,0<=dc<=255,
            a==255*(sa as int)+(da as int)*(255-sa as int),
            n==255*(sa as int)*(sc as int)+(da as int)*(255-sa as int)*(dc as int);
    half_up_bound(n,a,255);
    half_up_bound(a,255,255);
}

/// MOver's channel equation includes both non-dividing alpha endpoints.
pub open spec fn over_component(s: Seq<u8>,d: Seq<u8>,c: int) -> int {
    if s[3]==255 {s[c] as int} else if s[3]==0 {d[c] as int}
    else if c==3 {(2*alpha(s[3],d[3])+255)/510}
    else {(2*numerator(s[3],d[3],s[c],d[c])+alpha(s[3],d[3]))/(2*alpha(s[3],d[3]))}
}
pub open spec fn over(s: Seq<u8>,d: Seq<u8>) -> Seq<u8> {
    Seq::new(4,|c: int| over_component(s,d,c) as u8)
}

pub proof fn over_equations(s: Seq<u8>,d: Seq<u8>)
    requires s.len()==4,d.len()==4,
    ensures over(s,d).len()==4,
        forall|c: int| 0<=c<4 ==> 0<=(#[trigger] over_component(s,d,c))<=255,
        forall|c: int| 0<=c<4 ==> #[trigger] over(s,d)[c] as int==over_component(s,d,c),
        s[3]==255 ==> over(s,d)==s,
        s[3]==0 ==> over(s,d)==d,
{
    assert forall|c: int| 0<=c<4 implies 0<=(#[trigger] over_component(s,d,c))<=255 by {
        if s[3]!=0 && s[3]!=255 {over_bounds(s[3],d[3],s[c],d[c]);}
    }
    if s[3]==255 {assert(over(s,d)=~=s);}
    if s[3]==0 {assert(over(s,d)=~=d);}
}

/// Canvas-byte geometry used to prove the concrete rectangle implementation.
/// This is not a specification of Saved's otherwise unspecified layout.
pub struct Rect {pub x: nat,pub y: nat,pub w: nat,pub h: nat}
pub open spec fn rect(x: u32,y: u32,w: u32,h: u32) -> Rect {
    Rect {x:x as nat,y:y as nat,w:w as nat,h:h as nat}
}
pub open spec fn row_base(cw: nat,r: Rect,row: nat) -> nat {4*((r.y+row)*cw+r.x)}
pub open spec fn inside_rows(cw: nat,r: Rect,rows: nat,j: int) -> bool {
    0<=j && r.y<=j/((4*cw) as int)<r.y+rows && 4*r.x<=j%((4*cw) as int)<4*(r.x+r.w)
}
pub open spec fn inside(cw: nat,r: Rect,j: int) -> bool {inside_rows(cw,r,r.h,j)}
pub open spec fn copy_rows(before: Seq<u8>,source: Seq<u8>,cw: nat,r: Rect,rows: nat) -> Seq<u8> {
    Seq::new(before.len(),|j: int| if inside_rows(cw,r,rows,j) {source[j]}else{before[j]})
}
pub open spec fn copy_rect(before: Seq<u8>,source: Seq<u8>,cw: nat,r: Rect) -> Seq<u8> {
    copy_rows(before,source,cw,r,r.h)
}
pub open spec fn clear_rows(before: Seq<u8>,cw: nat,r: Rect,rows: nat) -> Seq<u8> {
    Seq::new(before.len(),|j: int| if inside_rows(cw,r,rows,j) {0u8}else{before[j]})
}
pub open spec fn clear_rect(before: Seq<u8>,cw: nat,r: Rect) -> Seq<u8> {
    clear_rows(before,cw,r,r.h)
}

/// An executed rectangle byte load/store has this quotient and remainder.
pub proof fn rectangle_access(cw: nat,r: Rect,row: nat,col: nat)
    requires cw>=1,r.x+r.w<=cw,col<4*r.w,
    ensures (row_base(cw,r,row)+col)/(4*cw)==r.y+row,
        (row_base(cw,r,row)+col)%(4*cw)==4*r.x+col,
        row<r.h ==> inside(cw,r,(row_base(cw,r,row)+col) as int),
{
    let at=row_base(cw,r,row)+col;
    let den=4*cw;
    let rem=4*r.x+col;
    assert(0<=rem<den) by (nonlinear_arith)
        requires r.x+r.w<=cw,col<4*r.w,rem==4*r.x+col,den==4*cw;
    assert(at==(r.y+row)*den+rem) by (nonlinear_arith)
        requires at==4*((r.y+row)*cw+r.x)+col,den==4*cw,rem==4*r.x+col;
    lemma_fundamental_div_mod_converse(at as int,den as int,(r.y+row) as int,rem as int);
}

/// One physical row interval is exactly one quotient and its column range.
proof fn row_interval(cw: nat,r: Rect,row: nat,j: int)
    requires cw>=1,r.x+r.w<=cw,0<=j,
    ensures (row_base(cw,r,row)<=j<row_base(cw,r,row)+4*r.w)
        == (j/((4*cw) as int)==r.y+row && 4*r.x<=j%((4*cw) as int)<4*(r.x+r.w)),
{
    let base=row_base(cw,r,row);
    if base<=j<base+4*r.w {
        rectangle_access(cw,r,row,(j-base) as nat);
    }else if j/((4*cw) as int)==r.y+row && 4*r.x<=j%((4*cw) as int)<4*(r.x+r.w) {
        lemma_fundamental_div_mod(j,(4*cw) as int);
        assert(j==base+(j%((4*cw) as int)-4*r.x)) by (nonlinear_arith)
            requires j==(4*cw)*(r.y+row)+j%((4*cw) as int),base==4*((r.y+row)*cw+r.x);
    }
}

pub proof fn rows_step(cw: nat,r: Rect,row: nat,j: int)
    requires cw>=1,r.x+r.w<=cw,0<=j,
    ensures inside_rows(cw,r,row+1,j)==(inside_rows(cw,r,row,j)
        || row_base(cw,r,row)<=j<row_base(cw,r,row)+4*r.w),
{
    row_interval(cw,r,row,j);
}

/// Finish a row after an implementation has copied precisely its byte span.
pub proof fn copy_row_finished(before: Seq<u8>,source: Seq<u8>,after: Seq<u8>,cw: nat,r: Rect,row: nat)
    requires cw>=1,r.x+r.w<=cw,after.len()==before.len(),
        forall|j: int| 0<=j<before.len() ==> #[trigger] after[j]
            == if inside_rows(cw,r,row,j) || row_base(cw,r,row)<=j<row_base(cw,r,row)+4*r.w
                {source[j]}else{before[j]},
    ensures after==copy_rows(before,source,cw,r,row+1),
{
    assert forall|j: int| 0<=j<before.len() implies after[j]==copy_rows(before,source,cw,r,row+1)[j] by {
        rows_step(cw,r,row,j);
    }
    assert(after=~=copy_rows(before,source,cw,r,row+1));
}

pub proof fn clear_row_finished(before: Seq<u8>,prior: Seq<u8>,after: Seq<u8>,cw: nat,r: Rect,row: nat)
    requires cw>=1,r.x+r.w<=cw,after.len()==before.len(),prior==clear_rows(before,cw,r,row),
        forall|j: int| 0<=j<before.len() ==> #[trigger] after[j]
            == if row_base(cw,r,row)<=j<row_base(cw,r,row)+4*r.w {0u8}else{prior[j]},
    ensures after==clear_rows(before,cw,r,row+1),
{
    assert forall|j: int| 0<=j<before.len() implies after[j]==clear_rows(before,cw,r,row+1)[j] by {
        rows_step(cw,r,row,j);
    }
    assert(after=~=clear_rows(before,cw,r,row+1));
}

/// E1 form: arbitrary prior Saved bytes cannot affect a matching restore.
/// The conclusion describes only the restore effect, not Saved's layout.
pub proof fn matching_restore(drawn: Seq<u8>,before: Seq<u8>,arbitrary: Seq<u8>,cw: nat,r: Rect)
    requires drawn.len()==before.len(),arbitrary.len()==before.len(),
    ensures copy_rect(drawn,copy_rect(arbitrary,before,cw,r),cw,r)==copy_rect(drawn,before,cw,r),
{
    assert(copy_rect(drawn,copy_rect(arbitrary,before,cw,r),cw,r)=~=copy_rect(drawn,before,cw,r));
}

/// A matching restore recovers the entire old canvas when drawing touched
/// only that rectangle; Saved outside the read footprint is irrelevant.
pub proof fn restore_before(drawn: Seq<u8>,before: Seq<u8>,arbitrary: Seq<u8>,cw: nat,r: Rect)
    requires drawn.len()==before.len(),arbitrary.len()==before.len(),
        forall|j: int| 0<=j<before.len() && !inside(cw,r,j) ==> #[trigger] drawn[j]==before[j],
    ensures copy_rect(drawn,copy_rect(arbitrary,before,cw,r),cw,r)==before,
{
    matching_restore(drawn,before,arbitrary,cw,r);
    assert(copy_rect(drawn,before,cw,r)=~=before);
}

/// Auxiliary pre-paint region semantics. Absence of a required snapshot is
/// outside the restore domain; the receiver invariant proves its presence.
pub open spec fn dispose_regions(before: Seq<u8>,saved: Option<Seq<u8>>,cw: nat,previous: Option<(Rect,u8)>) -> Option<Seq<u8>> {
    match previous {
        Some((r,1))=>Some(clear_rect(before,cw,r)),
        Some((r,2))=>match saved {Some(s)=>Some(copy_rect(before,s,cw,r)),None=>None},
        _=>Some(before),
    }
}

/// State a snapshot solely through a restore observation, not a storage layout.
pub open spec fn snapshot_effect(saved: Seq<u8>,reference: Seq<u8>,cw: nat,r: Rect) -> bool {
    copy_rect(reference,saved,cw,r)==reference
}
pub proof fn snapshot_effect_on_drawn(drawn: Seq<u8>,saved: Seq<u8>,reference: Seq<u8>,cw: nat,r: Rect)
    requires drawn.len()==reference.len(),saved.len()==reference.len(),snapshot_effect(saved,reference,cw,r),
    ensures copy_rect(drawn,saved,cw,r)==copy_rect(drawn,reference,cw,r),
{
    assert(copy_rect(drawn,saved,cw,r)=~=copy_rect(drawn,reference,cw,r));
}

/// Byte positions writable by a decoded image. Canvas destinations use the
/// frame rectangle; a still destination owns its entire output slice.
pub open spec fn drawing_region(shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat,j: int) -> bool {
    if shape.0==2 {inside(shape.2,Rect {x:shape.3,y:shape.4,w,h},j)}else{0<=j<shape.1}
}
#[verifier::opaque]
pub open spec fn drawing_frame(before: Seq<u8>,after: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat) -> bool {
    before.len()==after.len() && forall|j: int| 0<=j<before.len() && !drawing_region(shape,w,h,j)
        ==> #[trigger] after[j]==before[j]
}
pub proof fn drawing_frame_intro(before: Seq<u8>,after: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat)
    requires before.len()==after.len(),forall|j: int| 0<=j<before.len() && !drawing_region(shape,w,h,j)
        ==> #[trigger] after[j]==before[j],
    ensures drawing_frame(before,after,shape,w,h),
{reveal(drawing_frame);}
pub proof fn drawing_frame_start(before: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat)
    ensures drawing_frame(before,before,shape,w,h),
{reveal(drawing_frame);}
pub proof fn drawing_frame_transitive(before: Seq<u8>,middle: Seq<u8>,after: Seq<u8>,shape: (nat,nat,nat,nat,nat,bool),w: nat,h: nat)
    requires drawing_frame(before,middle,shape,w,h),drawing_frame(middle,after,shape,w,h),
    ensures drawing_frame(before,after,shape,w,h),
{reveal(drawing_frame);}

/// The IDAT reuse path applies OVER to the already-decoded source on an
/// initially transparent canvas. Transparent source RGB must become zero.
pub open spec fn zero_alpha(before: Seq<u8>) -> Seq<u8> {
    Seq::new(before.len(),|j: int| if 4*(j/4)+3<before.len() && j%4<3 && before[4*(j/4)+3]==0 {0u8}else{before[j]})
}
pub proof fn over_transparent(source: Seq<u8>)
    requires source.len()==4,
    ensures over(source,seq![0u8,0u8,0u8,0u8])==if source[3]==0 {seq![0u8,0u8,0u8,0u8]}else{source},
{
    let empty=seq![0u8,0u8,0u8,0u8];
    over_equations(source,empty);
    if source[3]!=0 && source[3]!=255 {
        let a=alpha(source[3],0);
        assert(a==255*(source[3] as int));
        assert forall|c: int| 0<=c<3 implies over(source,empty)[c]==source[c] by {
            over_bounds(source[3],0,source[c],0);
            assert(numerator(source[3],0,source[c],0)==a*(source[c] as int)) by (nonlinear_arith)
                requires a==255*(source[3] as int);
            assert(2*numerator(source[3],0,source[c],0)+a==(source[c] as int)*(2*a)+a) by (nonlinear_arith)
                requires numerator(source[3],0,source[c],0)==a*(source[c] as int);
            lemma_fundamental_div_mod_converse_div(2*numerator(source[3],0,source[c],0)+a,2*a,source[c] as int,a);
        }
        assert(2*a+255==(source[3] as int)*510+255) by (nonlinear_arith)
            requires a==255*(source[3] as int);
        lemma_fundamental_div_mod_converse_div(2*a+255,510,source[3] as int,255);
        assert(over(source,empty)=~=source);
    }
}

pub proof fn pixel_block(base: nat,j: int)
    requires base%4==0,base<=j<base+4,
    ensures 4*(j/4)==base,j%4==j-base,
{
    lemma_fundamental_div_mod(base as int,4);
    lemma_fundamental_div_mod_converse(j,4,(base/4) as int,j-base);
}
/// A full-canvas frame has no valid byte outside its drawing region.
pub proof fn full_drawing_frame(before: Seq<u8>,after: Seq<u8>,cw: nat,h: nat,blend: bool)
    requires cw>=1,before.len()==4*cw*h,after.len()==before.len(),
    ensures drawing_frame(before,after,(2,before.len(),cw,0,0,blend),cw,h),
{
    assert forall|j: int| 0<=j<before.len() implies inside(cw,Rect {x:0,y:0,w:cw,h},j) by {
        let den=(4*cw) as int;
        lemma_fundamental_div_mod(j,den);
        lemma_mod_pos_bound(j,den);
        lemma_div_pos_is_pos(j,den);
        assert(j/den<h) by (nonlinear_arith)
            requires j==den*(j/den)+j%den,0<=j%den<den,den>0,j<den*h;
    }
    drawing_frame_intro(before,after,(2,before.len(),cw,0,0,blend),cw,h);
}

/// A previous-frame reference is related to concrete Saved only by the
/// canvas that restoration produces. No Saved-layout equation is asserted.
#[verifier::opaque]
pub open spec fn saved_before_frame(canvas: Seq<u8>,saved: Option<Seq<u8>>,reference: Option<Seq<u8>>,cw: nat,r: Rect) -> bool {
    match (saved,reference) {(Some(s),Some(before))=>copy_rect(canvas,s,cw,r)==before,_=>false}
}
pub proof fn saved_before_frame_restore(canvas: Seq<u8>,saved: Option<Seq<u8>>,reference: Option<Seq<u8>>,cw: nat,r: Rect)
    requires saved_before_frame(canvas,saved,reference,cw,r),
    ensures saved.is_some(),reference.is_some(),copy_rect(canvas,saved.unwrap(),cw,r)==reference.unwrap(),
{reveal(saved_before_frame);}

/// Whole-frame footprint plus the actual pre-paint snapshot restores the
/// observed canvas to the actual canvas from before this frame's first write.
pub proof fn drawing_restores_before(before: Seq<u8>,after: Seq<u8>,saved: Seq<u8>,cw: nat,r: Rect,blend: bool)
    requires saved.len()==before.len(),snapshot_effect(saved,before,cw,r),
        drawing_frame(before,after,(2,before.len(),cw,r.x,r.y,blend),r.w,r.h),
    ensures saved_before_frame(after,Some(saved),Some(before),cw,r),
{
    reveal(drawing_frame);reveal(saved_before_frame);
    snapshot_effect_on_drawn(after,saved,before,cw,r);
    assert(copy_rect(after,before,cw,r)=~=before);
}

/// Every complete normalized pixel is exactly OVER on transparent black.
pub proof fn zero_alpha_pixel(before: Seq<u8>,p: nat)
    requires 4*p+4<=before.len(),
    ensures zero_alpha(before).subrange((4*p) as int,(4*p+4) as int)
        ==over(before.subrange((4*p) as int,(4*p+4) as int),seq![0u8,0u8,0u8,0u8]),
{
    let source=before.subrange((4*p) as int,(4*p+4) as int);
    over_transparent(source);
    assert((4*p)%4==0);
    assert forall|c: int| 0<=c<4 implies #[trigger] zero_alpha(before)[4*p+c]
        ==if source[3]==0 {0u8}else{source[c]} by {
        pixel_block(4*p,4*p+c);
    }
    assert(zero_alpha(before).subrange((4*p) as int,(4*p+4) as int)=~=
        if source[3]==0 {seq![0u8,0u8,0u8,0u8]}else{source});
}

}
