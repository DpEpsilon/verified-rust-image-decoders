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

}
