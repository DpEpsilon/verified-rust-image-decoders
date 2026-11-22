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

}
