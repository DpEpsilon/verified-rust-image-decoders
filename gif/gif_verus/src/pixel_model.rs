//! T13 pixel invariants for the independent compositor and frame models.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::mc::*;
#[cfg(verus_only)]
use crate::model::{HeaderM,WalkM,FrameHeaderM,GceM,MbOut};
#[cfg(verus_only)]
use crate::policy::Policy;

verus! {

pub proof fn lemma_L_length(m:int,d:Seq<u8>,a:int,ps:crate::lzw::LzwParams,st:crate::lzw_model::St)
    requires 0<=st.q<=a,st.o.len()==st.q,
    ensures crate::lzw_model::L(m,d,a,ps,st).idx.len()<=a,
        crate::lzw_model::L(m,d,a,ps,st).error is None ==> crate::lzw_model::L(m,d,a,ps,st).idx.len()==a,
    decreases crate::lzw_model::nbits(d)-st.pos,
{
    hide(crate::lzw_model::L);hide(crate::lzw_model::semit);hide(crate::lzw_model::first_of);
    crate::lzw_model::lemma_L_unfold(m,d,a,ps,st);
    match crate::lzw_model::verdict(m,d,a,ps,st) {
        crate::lzw_model::Vd::Halt(r)=>{},
        crate::lzw_model::Vd::Step(ns)=>if ns.pos>st.pos {lemma_L_length(m,d,a,ps,ns);},
    }
}
pub proof fn lemma_ML_length(m:int,d:Seq<u8>,a:int,ps:crate::lzw::LzwParams)
    requires a>=0,
    ensures crate::lzw_model::ML(m,d,a,ps).idx.len()<=a,
        crate::lzw_model::ML(m,d,a,ps).error is None ==> crate::lzw_model::ML(m,d,a,ps).idx.len()==a,
{
    hide(crate::lzw_model::L);
    if crate::lzw_model::m_ok(ps,m,a) {lemma_L_length(m,d,a,ps,crate::lzw_model::init_st(m));}
}

proof fn lemma_pixel_transfer(dst:Seq<u8>,src:Seq<u8>,i:int,ts:Set<Seq<u8>>)
    requires pixels_well_formed(src,ts),0<=i<src.len()/4,4*i+3<dst.len(),
        dst[4*i]==src[4*i],dst[4*i+1]==src[4*i+1],dst[4*i+2]==src[4*i+2],dst[4*i+3]==src[4*i+3],
    ensures pixel_is_clear(dst,i) || exists|table:Seq<u8>,v:int| ts.contains(table) && 0<=v && 3*v+2<table.len() && #[trigger] pixel_is_entry(dst,table,i,v),
{
    if !pixel_is_clear(src,i) {
        let (table,v)=choose|table:Seq<u8>,v:int| ts.contains(table) && 0<=v && 3*v+2<table.len() && pixel_is_entry(src,table,i,v);
        assert(pixel_is_entry(dst,table,i,v));
    }
}
proof fn lemma_cell_clip(i:int,j:int,sw:u32,sh:u32,r:Rect)
    requires i>=0,0<=j<4,sw>=1,
    ensures in_clipped(4*i+j,sw,sh,r)==in_clipped(4*i,sw,sh,r),
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(4*i+j,4,i,j);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(4*i,4,i,0);
}
pub proof fn lemma_clear_pixels(cv:Seq<u8>,sw:u32,sh:u32,r:Rect,ts:Set<Seq<u8>>)
    requires sw>=1,pixels_well_formed(cv,ts),
    ensures clear_rect(cv,sw,sh,r).len()==cv.len(),pixels_well_formed(clear_rect(cv,sw,sh,r),ts),
{
    let out=clear_rect(cv,sw,sh,r);
    assert forall|i:int| 0<=i<out.len()/4 implies pixel_is_clear(out,i) ||
        exists|table:Seq<u8>,v:int| ts.contains(table) && 0<=v && 3*v+2<table.len() && #[trigger] pixel_is_entry(out,table,i,v) by {
        lemma_cell_clip(i,1,sw,sh,r);lemma_cell_clip(i,2,sw,sh,r);lemma_cell_clip(i,3,sw,sh,r);
        if in_clipped(4*i,sw,sh,r) {assert(pixel_is_clear(out,i));}
        else {lemma_pixel_transfer(out,cv,i,ts);}
    }
}
pub proof fn lemma_copy_pixels(dst:Seq<u8>,src:Seq<u8>,sw:u32,sh:u32,r:Rect,ts:Set<Seq<u8>>)
    requires sw>=1,dst.len()==src.len(),pixels_well_formed(dst,ts),pixels_well_formed(src,ts),
    ensures copy_rect(dst,src,sw,sh,r).len()==dst.len(),pixels_well_formed(copy_rect(dst,src,sw,sh,r),ts),
{
    let out=copy_rect(dst,src,sw,sh,r);
    assert forall|i:int| 0<=i<out.len()/4 implies pixel_is_clear(out,i) ||
        exists|table:Seq<u8>,v:int| ts.contains(table) && 0<=v && 3*v+2<table.len() && #[trigger] pixel_is_entry(out,table,i,v) by {
        lemma_cell_clip(i,1,sw,sh,r);lemma_cell_clip(i,2,sw,sh,r);lemma_cell_clip(i,3,sw,sh,r);
        if in_clipped(4*i,sw,sh,r) {lemma_pixel_transfer(out,src,i,ts);}
        else {lemma_pixel_transfer(out,dst,i,ts);}
    }
}

pub proof fn lemma_draw_pixels(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,idx:Seq<u8>,q:int,ts:Set<Seq<u8>>)
    requires pixels_well_formed(cv,ts),ts.contains(table),0<=fr.c,3*fr.c<=table.len(),
        cv.len()==4*(sw as int)*(sh as int),1<=sw,1<=sh,
        1<=fr.rect.width,1<=fr.rect.height<=crate::limits::MAX_DIMENSION,
        0<=q<=idx.len()<=(fr.rect.width as int)*(fr.rect.height as int),
    ensures draw_fold(cv,fr,table,sw,sh,idx,q) is Ok ==> draw_fold(cv,fr,table,sw,sh,idx,q).unwrap().len()==cv.len()
        && pixels_well_formed(draw_fold(cv,fr,table,sw,sh,idx,q).unwrap(),ts),
    decreases idx.len()-q,
{
    reveal_with_fuel(draw_fold,1);
    if q<idx.len() {
        crate::emission_compose::lemma_draw_step_pixels_clip(cv,fr,table,sw,sh,q,idx[q],ts);
        if draw_step(cv,fr,table,sw,sh,q,idx[q]) is Ok {
            lemma_draw_pixels(draw_step(cv,fr,table,sw,sh,q,idx[q]).unwrap(),fr,table,sw,sh,idx,q+1,ts);
        }
    }
}

pub open spec fn table_at(b:Seq<u8>,h:HeaderM,p:Policy,k:nat)->Option<Seq<u8>> {
    match crate::frame_model::after_calls(b,h,p,k) {
        Some(s)=>if s.status==crate::frame_model::Status::Running {
            match crate::model::mb(b,h,p,s.walk) {
                Ok(MbOut::Image(fh,_,_,_,_))=>Some(b.subrange(fh.table.off,fh.table.off+3*fh.table.entries)),
                _=>None,
            }
        } else {None},
        None=>None,
    }
}
/// Only tables active at a model image step belong to this set.
pub open spec fn active_tables(b:Seq<u8>,h:HeaderM,p:Policy,k:nat)->Set<Seq<u8>>
    decreases k,
{
    if k==0 {Set::empty()} else {
        let prev=active_tables(b,h,p,(k-1) as nat);
        match table_at(b,h,p,(k-1) as nat) {Some(t)=>prev.insert(t),None=>prev}
    }
}
pub open spec fn state_pixels(s:crate::frame_model::State,sw:u32,sh:u32,ts:Set<Seq<u8>>)->bool {
    &&& s.canvas.len()==4*(sw as int)*(sh as int) && pixels_well_formed(s.canvas,ts)
    &&& match s.saved {None=>true,Some(v)=>v.len()==s.canvas.len() && pixels_well_formed(v,ts)}
}
proof fn lemma_pixels_subset(cv:Seq<u8>,small:Set<Seq<u8>>,big:Set<Seq<u8>>)
    requires pixels_well_formed(cv,small),forall|t:Seq<u8>| #[trigger] small.contains(t) ==> big.contains(t),
    ensures pixels_well_formed(cv,big),
{
    assert forall|i:int| 0<=i<cv.len()/4 implies pixel_is_clear(cv,i) ||
        exists|t:Seq<u8>,v:int| big.contains(t) && 0<=v && 3*v+2<t.len() && #[trigger] pixel_is_entry(cv,t,i,v) by {
        if !pixel_is_clear(cv,i) {
            let (t,v)=choose|t:Seq<u8>,v:int| small.contains(t) && 0<=v && 3*v+2<t.len() && pixel_is_entry(cv,t,i,v);
            assert(big.contains(t));
        }
    }
}
proof fn lemma_state_pixels_subset(s:crate::frame_model::State,sw:u32,sh:u32,small:Set<Seq<u8>>,big:Set<Seq<u8>>)
    requires state_pixels(s,sw,sh,small),forall|t:Seq<u8>| #[trigger] small.contains(t) ==> big.contains(t),
    ensures state_pixels(s,sw,sh,big),
{
    lemma_pixels_subset(s.canvas,small,big);
    if s.saved is Some {lemma_pixels_subset(s.saved.unwrap(),small,big);}
}
proof fn lemma_disposed_pixels(s:crate::frame_model::State,sw:u32,sh:u32,ts:Set<Seq<u8>>)
    requires sw>=1,state_pixels(s,sw,sh,ts),
    ensures crate::frame_model::disposed(s.canvas,s.saved,s.prev,sw,sh).len()==s.canvas.len(),
        pixels_well_formed(crate::frame_model::disposed(s.canvas,s.saved,s.prev,sw,sh),ts),
{
    if s.prev is Some {
        let prev=s.prev.unwrap();
        if prev.disposal==2 {lemma_clear_pixels(s.canvas,sw,sh,prev.rect,ts);}
        else if prev.disposal==3 && s.saved is Some {lemma_copy_pixels(s.canvas,s.saved.unwrap(),sw,sh,prev.rect,ts);}
    }
}
pub proof fn lemma_image_pixels(b:Seq<u8>,h:HeaderM,p:Policy,s:crate::frame_model::State,fh:FrameHeaderM,g:Option<GceM>,m:int,data:Seq<u8>,walk:WalkM,still:bool,ts:Set<Seq<u8>>)
    requires crate::frame_model::image_domain(b,h,fh),state_pixels(s,h.width as u32,h.height as u32,ts),
        ts.contains(b.subrange(fh.table.off,fh.table.off+3*fh.table.entries)),
    ensures crate::frame_model::image_mode(b,h,p,s,fh,g,m,data,walk,still) is Some,
        state_pixels(crate::frame_model::image_mode(b,h,p,s,fh,g,m,data,walk,still).unwrap().after,h.width as u32,h.height as u32,ts),
{
    hide(crate::lzw_model::ML);hide(draw_fold);hide(clear_rect);hide(copy_rect);
    crate::frame_model::lemma_image_defined(b,h,p,s,fh,g,m,data,walk,still);
    let sw=h.width as u32;let sh=h.height as u32;
    let fr=crate::frame_model::frame(fh,g);
    let table=b.subrange(fh.table.off,fh.table.off+3*fh.table.entries);
    lemma_disposed_pixels(s,sw,sh,ts);
    let cv=crate::frame_model::disposed(s.canvas,s.saved,s.prev,sw,sh);
    if fr.disposal==3 && !still {
        let base=match s.saved {Some(v)=>v,None=>Seq::new(s.canvas.len(),|_i:int|0u8)};
        if s.saved is None {crate::compose_new::lemma_initial_pixels_well_formed(sw,sh,ts);}
        lemma_copy_pixels(base,cv,sw,sh,fr.rect,ts);
    }
    assert(fh.w*fh.h>=1) by(nonlinear_arith) requires fh.w>=1,fh.h>=1;
    lemma_ML_length(m,data,fh.w*fh.h,crate::frame_model::params(p));
    let ml=crate::lzw_model::ML(m,data,fh.w*fh.h,crate::frame_model::params(p));
    lemma_draw_pixels(cv,fr,table,sw,sh,ml.idx,0,ts);
}
pub proof fn lemma_call_pixels(b:Seq<u8>,h:HeaderM,p:Policy,s:crate::frame_model::State,still:bool,ts:Set<Seq<u8>>)
    requires crate::model::mh_bounds(b,h),0<=s.walk.p<=b.len(),state_pixels(s,h.width as u32,h.height as u32,ts),
        match crate::model::mb(b,h,p,s.walk) {Ok(MbOut::Image(fh,_,_,_,_))=>ts.contains(b.subrange(fh.table.off,fh.table.off+3*fh.table.entries)),_=>true},
    ensures crate::frame_model::call_mode(b,h,p,s,still) is Some,
        state_pixels(crate::frame_model::call_mode(b,h,p,s,still).unwrap().after,h.width as u32,h.height as u32,ts),
{
    hide(crate::model::mb);hide(crate::frame_model::image_mode);
    if s.status==crate::frame_model::Status::Running {
        match crate::model::mb(b,h,p,s.walk) {
            Ok(MbOut::Image(fh,g,m,data,walk))=>{
                assert(crate::model::mb_fuel(b,h,p,s.walk,(b.len() as int-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,data,walk))) by {reveal(crate::model::mb);}
                crate::frame_model::lemma_mb_image_progress(b,h,p,s.walk,(b.len() as int-s.walk.p) as nat,fh,g,m,data,walk);
                lemma_image_pixels(b,h,p,s,fh,g,m,data,walk,still,ts);
            },
            _=>{},
        }
    }
}
pub proof fn lemma_initial_pixels(b:Seq<u8>,h:HeaderM,p:Policy,ts:Set<Seq<u8>>)
    requires crate::model::mh(b)==Ok(h),crate::model::mp(b,p) is Ok,
    ensures crate::frame_model::initial(b,p) is Some,
        state_pixels(crate::frame_model::initial(b,p).unwrap(),h.width as u32,h.height as u32,ts),
{
    hide(crate::model::mp_fuel);
    crate::frame_model::lemma_initial_domain(b,p);
    crate::compose_new::lemma_initial_pixels_well_formed(h.width as u32,h.height as u32,ts);
}
pub proof fn lemma_after_pixels(b:Seq<u8>,h:HeaderM,p:Policy,k:nat)
    requires crate::model::mh(b)==Ok(h),crate::model::mp(b,p) is Ok,
    ensures crate::frame_model::after_calls(b,h,p,k) is Some,
        state_pixels(crate::frame_model::after_calls(b,h,p,k).unwrap(),h.width as u32,h.height as u32,active_tables(b,h,p,k)),
        0<=crate::frame_model::after_calls(b,h,p,k).unwrap().walk.p<=b.len(),
    decreases k,
{
    hide(crate::model::mp);hide(crate::model::mb);hide(crate::frame_model::call);hide(crate::frame_model::call_mode);
    let ts=active_tables(b,h,p,k);
    crate::frame_model::lemma_initial_domain(b,p);
    reveal_with_fuel(crate::frame_model::after_calls,1);
    if k==0 {lemma_initial_pixels(b,h,p,ts);}
    else {
        lemma_after_pixels(b,h,p,(k-1) as nat);
        let s=crate::frame_model::after_calls(b,h,p,(k-1) as nat).unwrap();
        let prev=active_tables(b,h,p,(k-1) as nat);
        assert forall|t:Seq<u8>| #[trigger] prev.contains(t) implies ts.contains(t) by {};
        lemma_state_pixels_subset(s,h.width as u32,h.height as u32,prev,ts);
        crate::frame_model::lemma_call_progress(b,h,p,s);
        if s.status==crate::frame_model::Status::Running {
            match crate::model::mb(b,h,p,s.walk) {
                Ok(MbOut::Image(fh,_,_,_,_))=>{
                    let table=b.subrange(fh.table.off,fh.table.off+3*fh.table.entries);
                    assert(table_at(b,h,p,(k-1) as nat)==Some(table));
                    assert(ts.contains(table));
                },
                _=>{},
            }
        }
        // A terminal call does not consult MB; permit its unused membership
        // premise by handling it separately instead of inventing a table.
        if s.status==crate::frame_model::Status::Running {lemma_call_pixels(b,h,p,s,false,ts);}
        else {crate::frame_model::lemma_terminal_call(b,h,p,s);}
        reveal(crate::frame_model::call);
    }
}
pub proof fn lemma_ms_pixels(b:Seq<u8>,p:Policy)
    ensures match crate::frame_model::ms(b,p) {
        Some(Ok(s))=>1<=s.width<=crate::limits::MAX_DIMENSION && 1<=s.height<=crate::limits::MAX_DIMENSION
            && s.rgba.len()==4*s.width*s.height<=crate::limits::MAX_OUTPUT
            && pixels_well_formed(s.rgba,active_tables(b,crate::model::mh(b).unwrap(),p,1)),
        _=>true,
    },
{
    hide(crate::model::mp_fuel);hide(crate::model::mb);hide(crate::frame_model::call_mode);hide(crate::frame_model::image_mode);
    if crate::model::mp(b,p) is Ok {
        let h=crate::model::mh(b).unwrap();
        let ts=active_tables(b,h,p,1);
        crate::frame_model::lemma_initial_domain(b,p);
        lemma_initial_pixels(b,h,p,ts);
        let s=crate::frame_model::initial(b,p).unwrap();
        if s.status==crate::frame_model::Status::Running {
            match crate::model::mb(b,h,p,s.walk) {
                Ok(MbOut::Image(fh,_,_,_,_))=>{
                    let table=b.subrange(fh.table.off,fh.table.off+3*fh.table.entries);
                    assert(table_at(b,h,p,0)==Some(table));assert(ts.contains(table));
                },
                _=>{},
            }
            lemma_call_pixels(b,h,p,s,true,ts);
        }
        reveal(crate::frame_model::call_mode);
        reveal(crate::frame_model::image_mode);
    }
}

} // verus!
