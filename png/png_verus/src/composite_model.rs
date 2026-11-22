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

}
