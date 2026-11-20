//! Independent P11/P12 layout arithmetic. Natural-number pass dimensions and
//! filter-byte overhead, with no executable parser or allocation operations.
use vstd::prelude::*;

verus! {

pub open spec fn bits(h: crate::model::Ihdr) -> nat {
    (if h.color_type==0 || h.color_type==3 {1nat}else if h.color_type==2 {3nat}else if h.color_type==4 {2nat}else{4nat}) * h.depth as nat
}

pub open spec fn output_count(h: crate::model::Ihdr,wide: bool) -> nat {
    (if wide {8nat}else{4nat})*h.width as nat*h.height as nat
}
pub open spec fn image_buffers(h: crate::model::Ihdr,wide: bool,raw_bytes: Seq<u8>,output: Seq<u8>) -> bool {
    raw_bytes.len()==raw(h.interlace,bits(h),h.width as nat,h.height as nat)
    && output.len()==output_count(h,wide)
}

pub open spec fn span(n: nat, origin: nat, step: nat) -> nat {
    if n>origin {((n-origin+step-1)/(step as int)) as nat}else{0}
}
pub open spec fn geometry(k: nat) -> (nat,nat,nat,nat) {
    if k==0 {(0,0,8,8)} else if k==1 {(4,0,8,8)} else if k==2 {(0,4,4,8)}
    else if k==3 {(2,0,4,4)} else if k==4 {(0,2,2,4)} else if k==5 {(1,0,2,2)} else {(0,1,1,2)}
}
pub open spec fn cols(w: nat,k: nat) -> nat {let g=geometry(k);span(w,g.0,g.2)}
pub open spec fn rows(h: nat,k: nat) -> nat {let g=geometry(k);span(h,g.1,g.3)}
pub open spec fn scan_bytes(bits: nat,w: nat) -> nat {(w*bits+7)/8}
pub open spec fn contribution(interlace: u8,bits: nat,w: nat,h: nat,k: nat) -> nat {
    if interlace==0 {if k==0 {h*(1+scan_bytes(bits,w))}else{0}}
    else if k<7 {let pw=cols(w,k);let ph=rows(h,k);
        if pw==0 || ph==0 {0}else{ph*(1+scan_bytes(bits,pw))}}
    else{0}
}
pub open spec fn count(interlace: u8) -> nat {if interlace==0 {1}else{7}}
pub open spec fn prefix(interlace: u8,bits: nat,w: nat,h: nat,n: nat) -> nat
    decreases n,
{if n==0 {0}else{prefix(interlace,bits,w,h,(n-1) as nat)+contribution(interlace,bits,w,h,(n-1) as nat)}}
pub open spec fn raw(interlace: u8,bits: nat,w: nat,h: nat) -> nat {prefix(interlace,bits,w,h,count(interlace))}
pub open spec fn area_prefix(w: nat,h: nat,n: nat) -> nat
    decreases n,
{if n==0 {0}else{area_prefix(w,h,(n-1) as nat)+cols(w,(n-1) as nat)*rows(h,(n-1) as nat)}}
pub open spec fn row_prefix(h: nat,n: nat) -> nat
    decreases n,
{if n==0 {0}else{row_prefix(h,(n-1) as nat)+rows(h,(n-1) as nat)}}

pub proof fn span_bounds(n: nat,origin: nat,step: nat)
    requires step>=1,
    ensures span(n,origin,step)<=n,
{
    if n>origin {
        vstd::arithmetic::div_mod::lemma_div_plus_one(n-origin-1,step as int);
        vstd::arithmetic::div_mod::lemma_div_basics_5(n-origin-1,step as int);
    }
}

/// The even/odd splits telescope the seven two-dimensional pass areas.
#[verifier::spinoff_prover]
pub proof fn axis_splits(n: nat)
    ensures span(n,0,8)+span(n,4,8)==span(n,0,4),
        span(n,0,4)+span(n,2,4)==span(n,0,2),
        span(n,0,2)+span(n,1,2)==n,
        8*span(n,0,8)<=n+7,4*span(n,0,4)<=n+3,2*span(n,0,2)<=n+1,
{
    assert(span(n,0,8)==(n+7)/8);
    assert(span(n,4,8)==(n+3)/8);
    assert(span(n,0,4)==(n+3)/4);
    assert(span(n,2,4)==(n+1)/4);
    assert(span(n,0,2)==(n+1)/2);
    assert(span(n,1,2)==n/2);
    assert((n+7)/8+(n+3)/8==(n+3)/4);
    assert((n+3)/4+(n+1)/4==(n+1)/2);
    assert((n+1)/2+n/2==n);
}

pub proof fn coverage(w: nat,h: nat)
    requires h>=1,
    ensures area_prefix(w,h,7)==w*h,row_prefix(h,7)<=2*h+2,
{
    reveal(cols);reveal(rows);reveal(geometry);reveal(span);
    axis_splits(w);axis_splits(h);
    reveal_with_fuel(area_prefix,10);reveal_with_fuel(row_prefix,10);
    let a=span(w,0,8);let b=span(w,4,8);let c=span(w,0,4);
    let d=span(w,2,4);let e=span(w,0,2);let f=span(w,1,2);
    let p=span(h,0,8);let q=span(h,4,8);let r=span(h,0,4);
    let s=span(h,2,4);let t=span(h,0,2);let u=span(h,1,2);
    assert(geometry(6)==(0nat,1nat,1nat,2nat)) by (compute);
    assert(cols(w,6)==span(w,0,1));
    if w>0 {assert(span(w,0,1)==w);} else {assert(span(w,0,1)==0);}
    assert(cols(w,6)==w);
    assert(area_prefix(w,h,0)==0);
    assert(area_prefix(w,h,7)==a*p+b*p+c*q+d*r+e*s+f*t+w*u);
    assert(row_prefix(h,7)==2*p+q+r+s+t+u);
    assert(a*p+b*p==c*p) by (nonlinear_arith) requires a+b==c;
    assert(c*p+c*q==c*r) by (nonlinear_arith) requires p+q==r;
    assert(c*r+d*r==e*r) by (nonlinear_arith) requires c+d==e;
    assert(e*r+e*s==e*t) by (nonlinear_arith) requires r+s==t;
    assert(e*t+f*t==w*t) by (nonlinear_arith) requires e+f==w;
    assert(w*t+w*u==w*h) by (nonlinear_arith) requires t+u==h;
    assert(a*p+b*p+c*q+d*r+e*s+f*t+w*u==w*h);
    assert(2*p+q+r+s+t+u==h+t+r+p);
    assert(t+r+p<=h+2);
}

pub proof fn scan_bound(bits: nat,w: nat)
    requires bits<=64,
    ensures scan_bytes(bits,w)<=8*w,
{
    assert(w*bits<=w*64) by (nonlinear_arith) requires bits<=64;
}

pub proof fn interlaced_prefix_bound(bits: nat,w: nat,h: nat,n: nat)
    requires bits<=64,n<=7,
    ensures prefix(1,bits,w,h,n)<=8*area_prefix(w,h,n)+row_prefix(h,n),
    decreases n,
{
    reveal_with_fuel(prefix,3);reveal_with_fuel(area_prefix,3);reveal_with_fuel(row_prefix,3);
    if n>0 {
        let k=(n-1) as nat;
        interlaced_prefix_bound(bits,w,h,k);scan_bound(bits,cols(w,k));
        let pw=cols(w,k);let ph=rows(h,k);
        assert(ph*(1+scan_bytes(bits,pw))<=8*pw*ph+ph) by (nonlinear_arith)
            requires scan_bytes(bits,pw)<=8*pw;
        assert(contribution(1,bits,w,h,k)<=8*pw*ph+ph);
        assert(8*area_prefix(w,h,k)+8*pw*ph==8*(area_prefix(w,h,k)+pw*ph)) by (nonlinear_arith);
    }
}

pub proof fn monotone(interlace: u8,bits: nat,w: nat,h: nat,lo: nat,hi: nat)
    requires lo<=hi,
    ensures prefix(interlace,bits,w,h,lo)<=prefix(interlace,bits,w,h,hi),
    decreases hi-lo,
{
    if lo<hi {monotone(interlace,bits,w,h,lo,(hi-1) as nat);}
}

pub proof fn raw_bounds(interlace: u8,bits: nat,w: nat,h: nat)
    requires interlace<=1,bits<=64,w>=1,h>=1,
    ensures 1<=raw(interlace,bits,w,h)<=10*w*h+2,
{
    reveal(raw);reveal_with_fuel(prefix,3);
    if interlace==0 {
        assert(prefix(interlace,bits,w,h,1)==h*(1+scan_bytes(bits,w)));
        scan_bound(bits,w);
        assert(1<=h*(1+scan_bytes(bits,w))) by (nonlinear_arith) requires h>=1;
        assert(h*(1+scan_bytes(bits,w))<=10*w*h+2) by (nonlinear_arith)
            requires w>=1,h>=1,scan_bytes(bits,w)<=8*w;
    } else {
        coverage(w,h);interlaced_prefix_bound(bits,w,h,7);
        assert(prefix(1,bits,w,h,7)<=8*w*h+2*h+2) by (nonlinear_arith)
            requires prefix(1,bits,w,h,7)<=8*area_prefix(w,h,7)+row_prefix(h,7),
                area_prefix(w,h,7)==w*h,row_prefix(h,7)<=2*h+2;
        assert(8*w*h+2*h+2<=10*w*h+2) by (nonlinear_arith) requires w>=1,h>=1;
        assert(cols(w,0)>=1 && rows(h,0)>=1);
        assert(contribution(1,bits,w,h,0)>=1) by (nonlinear_arith)
            requires cols(w,0)>=1,rows(h,0)>=1;
        assert(prefix(1,bits,w,h,1)==contribution(1,bits,w,h,0));
        monotone(1,bits,w,h,1,7);
    }
}

/// Pass extents and rounded scanline lengths preserve dimension order.
pub proof fn span_monotone(lo: nat,hi: nat,origin: nat,step: nat)
    requires lo<=hi,step>=1,
    ensures span(lo,origin,step)<=span(hi,origin,step),
{
    if lo>origin {
        assert(lo-origin+step-1<=hi-origin+step-1);
        vstd::arithmetic::div_mod::lemma_div_is_ordered(lo-origin+step-1,hi-origin+step-1,step as int);
    }
}
pub proof fn contribution_monotone(interlace: u8,bits: nat,w: nat,h: nat,W: nat,H: nat,k: nat)
    requires w<=W,h<=H,
    ensures contribution(interlace,bits,w,h,k)<=contribution(interlace,bits,W,H,k),
{
    if interlace==0 {
        assert(w*bits<=W*bits) by (nonlinear_arith) requires w<=W;
        assert(scan_bytes(bits,w)<=scan_bytes(bits,W));
        assert(h*(1+scan_bytes(bits,w))<=H*(1+scan_bytes(bits,W))) by (nonlinear_arith)
            requires h<=H,scan_bytes(bits,w)<=scan_bytes(bits,W);
    } else if k<7 {
        let g=geometry(k);
        span_monotone(w,W,g.0,g.2);span_monotone(h,H,g.1,g.3);
        let pw=cols(w,k);let ph=rows(h,k);let PW=cols(W,k);let PH=rows(H,k);
        assert(pw*bits<=PW*bits) by (nonlinear_arith) requires pw<=PW;
        assert(scan_bytes(bits,pw)<=scan_bytes(bits,PW));
        if pw>0 && ph>0 {
            assert(ph*(1+scan_bytes(bits,pw))<=PH*(1+scan_bytes(bits,PW))) by (nonlinear_arith)
                requires ph<=PH,scan_bytes(bits,pw)<=scan_bytes(bits,PW);
        }
    }
}
pub proof fn prefix_dimensions(interlace: u8,bits: nat,w: nat,h: nat,W: nat,H: nat,n: nat)
    requires w<=W,h<=H,
    ensures prefix(interlace,bits,w,h,n)<=prefix(interlace,bits,W,H,n),
    decreases n,
{
    reveal_with_fuel(prefix,2);
    if n>0 {
        prefix_dimensions(interlace,bits,w,h,W,H,(n-1) as nat);
        contribution_monotone(interlace,bits,w,h,W,H,(n-1) as nat);
    }
}
/// L-R-MONO: every fitting subframe reuses a prefix of the full-image Raw.
pub proof fn raw_dimensions(interlace: u8,bits: nat,w: nat,h: nat,W: nat,H: nat)
    requires w<=W,h<=H,
    ensures raw(interlace,bits,w,h)<=raw(interlace,bits,W,H),
{prefix_dimensions(interlace,bits,w,h,W,H,count(interlace));}

/// Safety-only pass coordinate bound, not Adam7 coverage or bijection.
pub proof fn span_index(n: nat,origin: nat,step: nat,index: nat)
    requires step>=1,index<span(n,origin,step),
    ensures origin+index*step<n,
{
    assert(n>origin);
    vstd::arithmetic::div_mod::lemma_div_plus_one(n-origin-1,step as int);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(n-origin-1,step as int);
    let quotient=(n-origin-1)/(step as int);
    assert(index<=quotient);
    assert((index as int)*(step as int)<=quotient*(step as int)) by (nonlinear_arith)
        requires 0<=index<=quotient,step>=1;
}

} // verus!
