//! P13 sample-pair and colour properties used by the complete Strict models.
use vstd::prelude::*;
use crate::{colour_model as c,document_model as d,raster_model as r,paint_model as p};

verus! {

pub proof fn sample_bound(line: Seq<u8>,depth: u8,i: nat)
    requires c::depth(depth),
    ensures 0<=c::sample(line,depth,i)<=c::maximum(depth),
{
    c::depth_values(depth);
    if depth==1 {
        let v=line[(i/8) as int];let shift=(7-i%8) as u8;
        assert((v>>shift)&1u8<=1u8) by (bit_vector);
    }else if depth==2 {
        let v=line[(i/4) as int];let shift=(6-2*(i%4)) as u8;
        assert((v>>shift)&3u8<=3u8) by (bit_vector);
    }else if depth==4 {
        let v=line[(i/2) as int];let shift=(4-4*(i%2)) as u8;
        assert((v>>shift)&15u8<=15u8) by (bit_vector);
    }
}
pub proof fn rounded_multiple(v: nat)
    ensures (257*v+128)/257==v,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse((257*v+128) as int,257,v as int,128);
}
pub proof fn scaled_pair(v: u16,depth: u8)
    requires c::depth(depth),v<=c::maximum(depth),
    ensures c::scaled(v,depth,false)<=255,
        c::scaled(v,depth,false) as nat==(c::scaled(v,depth,true) as nat+128)/257,
        depth<=8 ==> c::scaled(v,depth,true) as nat==257*(c::scaled(v,depth,false) as nat),
{
    c::depth_values(depth);c::scale16_exact(v as int,depth);
    if depth<=8 {
        c::scale8_exact(v as int,depth);c::scale16_257(v as int,depth);
        rounded_multiple(c::s8(v as int,depth) as nat);
    }else{
        assert(depth==16);c::round16(v as int);
        assert(c::s16(v as int,depth)==v) by (nonlinear_arith)
            requires c::s16(v as int,depth)*65535==(v as int)*65535;
    }
}
pub proof fn palette_pair(v: u8)
    ensures v as nat==((v as u16*257) as nat+128)/257,
        (v as u16*257) as nat==257*(v as nat),
{
    rounded_multiple(v as nat);
}

/// One channel keeps the default arithmetic queries independent.
#[verifier::spinoff_prover]
pub proof fn pixel_channel_pair(colour: c::Context,line: Seq<u8>,x: nat,j: nat)
    requires crate::model::valid_depth(colour.ct,colour.depth),j<4,
    ensures c::pixel(colour,line,x,false,false).is_ok()==c::pixel(colour,line,x,true,false).is_ok(),
        c::pixel(colour,line,x,false,false).is_ok() ==> {
            let a=c::pixel(colour,line,x,false,false).get_Ok_0();
            let b=c::pixel(colour,line,x,true,false).get_Ok_0();
            a.len()==4 && b.len()==4 && a[j as int]<=255
                && a[j as int] as nat==(b[j as int] as nat+128)/257
                && (colour.depth<=8 ==> b[j as int] as nat==257*(a[j as int] as nat))
        },
{
    let depth=colour.depth;
    if colour.ct==3 {
        let i=c::sample(line,depth,x) as nat;
        if i<colour.plte.len()/3 {
            if j<3 {palette_pair(colour.plte[(3*i+j) as int]);}
            else {let alpha=match colour.trns {Some(t) if i<t.len()=>t[i as int],_=>255u8};palette_pair(alpha);}
        }
    }else if (colour.ct==0 || colour.ct==2) && j==3 {
    }else{
        let i=if colour.ct==0 {x}else if colour.ct==2 {3*x+j}
            else if colour.ct==4 {if j<3 {2*x}else{2*x+1}}else{4*x+j};
        sample_bound(line,depth,i);scaled_pair(c::sample(line,depth,i),depth);
    }
    if j==0 {}else if j==1 {}else if j==2 {}else{assert(j==3);}
}

pub proof fn pixel_pair(colour: c::Context,line: Seq<u8>,x: nat)
    requires crate::model::valid_depth(colour.ct,colour.depth),
    ensures c::pixel(colour,line,x,false,false).is_ok()==c::pixel(colour,line,x,true,false).is_ok(),
        c::pixel(colour,line,x,false,false).is_ok() ==> {
            let a=c::pixel(colour,line,x,false,false).get_Ok_0();
            let b=c::pixel(colour,line,x,true,false).get_Ok_0();
            a.len()==4 && b.len()==4 && forall|j: int| 0<=j<4 ==>
                #[trigger] a[j]<=255 && a[j] as nat==(b[j] as nat+128)/257
                && (colour.depth<=8 ==> b[j] as nat==257*(a[j] as nat))
        },
{
    hide(c::pixel);
    pixel_channel_pair(colour,line,x,0);
    let a=c::pixel(colour,line,x,false,false);let b=c::pixel(colour,line,x,true,false);
    if a.is_ok() {
        assert forall|j: int| 0<=j<4 implies #[trigger] a.get_Ok_0()[j]<=255
            && a.get_Ok_0()[j] as nat==(b.get_Ok_0()[j] as nat+128)/257
            && (colour.depth<=8 ==> b.get_Ok_0()[j] as nat==257*(a.get_Ok_0()[j] as nat)) by {
            pixel_channel_pair(colour,line,x,j as nat);
        }
    }
}

pub open spec fn colour_properties(colour: c::Context,line: Seq<u8>,x: nat,wide: bool,px: Seq<u16>) -> bool {
    px.len()==4
    && (colour.ct==0 || colour.ct==4 ==> px[0]==px[1] && px[1]==px[2])
    && ((colour.ct==0 || colour.ct==2 || colour.ct==3) && colour.trns.is_none() ==> px[3]==c::top(wide))
    && ((colour.ct==0 || colour.ct==2) && colour.trns.is_some() ==> px[3]==0 || px[3]==c::top(wide))
    && (colour.ct==3 ==> {
        let i=c::sample(line,colour.depth,x) as nat;let scale=if wide {257nat}else{1nat};
        i<colour.plte.len()/3
        && (forall|j: int| 0<=j<3 ==> #[trigger] px[j] as nat==(colour.plte[3*i as int+j] as nat)*scale)
        && px[3] as nat==(match colour.trns {Some(t) if i<t.len()=>t[i as int] as nat,_=>255nat})*scale
    })
}
/// T13's format-specific channel package follows directly from P13's model.
#[verifier::spinoff_prover]
pub proof fn pixel_properties(colour: c::Context,line: Seq<u8>,x: nat,wide: bool)
    requires crate::model::valid_depth(colour.ct,colour.depth),c::pixel(colour,line,x,wide,false).is_ok(),
    ensures colour_properties(colour,line,x,wide,c::pixel(colour,line,x,wide,false).get_Ok_0()),
        forall|j: int| 0<=j<4 ==> (#[trigger] c::pixel(colour,line,x,wide,false).get_Ok_0()[j])<=c::top(wide),
{
    pixel_pair(colour,line,x);
    if colour.ct==3 {
        let i=c::sample(line,colour.depth,x) as nat;
        assert(i<colour.plte.len()/3);
        assert forall|j: int| 0<=j<3 implies
            #[trigger] c::pixel(colour,line,x,wide,false).get_Ok_0()[j] as nat
                ==(colour.plte[3*i as int+j] as nat)*(if wide {257nat}else{1nat}) by {
            if j==0 {}else if j==1 {}else {assert(j==2);}
        }
    }
}

}
