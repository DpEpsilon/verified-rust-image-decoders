//! Successful reverse-emission drawing: retained addresses, commutation and
//! pixel preservation. These lemmas make no claim about a canvas on error.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::mc::*;

verus! {

/// A retained cell is in the screen even when the frame rectangle is clipped.
pub proof fn lemma_target_shape_clip(fr:FrameSpec,sw:u32,sh:u32,q:int,cell:int)
    requires 1<=fr.rect.width,1<=fr.rect.height<=crate::limits::MAX_DIMENSION,
        0<=q<(fr.rect.width as int)*(fr.rect.height as int),
        target_cell(fr,sw,sh,q)==Some(cell),
    ensures cell==target_y(fr,q)*(sw as int)+target_x(fr,q),
        0<=target_x(fr,q)<sw as int,0<=target_y(fr,q)<sh as int,
{
    let w=fr.rect.width as int;
    crate::compose_new::lemma_row_below(q,w,fr.rect.height as int);
    vstd::arithmetic::div_mod::lemma_mod_bound(q,w);
    if fr.interlaced {crate::interlace::lemma_ir_bijective(fr.rect.height as int);}
}

/// Clipping removes cells; it does not identify two retained raster positions.
pub proof fn lemma_retained_cells_distinct(fr:FrameSpec,sw:u32,sh:u32,p:int,q:int,c1:int,c2:int)
    requires 1<=fr.rect.width,1<=fr.rect.height<=crate::limits::MAX_DIMENSION,
        0<=p<(fr.rect.width as int)*(fr.rect.height as int),
        0<=q<(fr.rect.width as int)*(fr.rect.height as int),p!=q,
        target_cell(fr,sw,sh,p)==Some(c1),target_cell(fr,sw,sh,q)==Some(c2),
    ensures c1!=c2,
{
    lemma_target_shape_clip(fr,sw,sh,p,c1);
    lemma_target_shape_clip(fr,sw,sh,q,c2);
    if c1==c2 {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(c1,sw as int,target_y(fr,p),target_x(fr,p));
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(c2,sw as int,target_y(fr,q),target_x(fr,q));
        assert(target_x(fr,p)==target_x(fr,q));
        assert(target_y(fr,p)==target_y(fr,q));
        crate::interlace::lemma_frame_address_injective(
            fr.rect.left as int+fr.rect.width as int,fr.rect.top as int+fr.rect.height as int,
            fr.rect.top as int,fr.rect.left as int,fr.rect.width as int,fr.rect.height as int,fr.interlaced,p,q);
    }
}

/// The generalized T13 per-put clause: transparency precedes lookup, Clip is an
/// identity step, and only an actual opaque write needs a table address.
pub proof fn lemma_draw_step_pixels_clip(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,q:int,v:u8,ts:Set<Seq<u8>>)
    requires pixels_well_formed(cv,ts),ts.contains(table),
        0<=fr.c,3*fr.c<=table.len(),
        cv.len()==4*(sw as int)*(sh as int),1<=sw,1<=sh,
        1<=fr.rect.width,1<=fr.rect.height<=crate::limits::MAX_DIMENSION,
        0<=q<(fr.rect.width as int)*(fr.rect.height as int),
    ensures match draw_step(cv,fr,table,sw,sh,q,v) {
        Ok(cv2)=>cv2.len()==cv.len() && pixels_well_formed(cv2,ts),
        Err(_)=>true,
    },
{
    if fr.transparent==Some(v) {
        assert(draw_step(cv,fr,table,sw,sh,q,v)==Ok(cv));
    } else if (v as int)>=fr.c {
        assert(draw_step(cv,fr,table,sw,sh,q,v) is Err);
    } else {
        match target_cell(fr,sw,sh,q) {
            None=>{},
            Some(cell)=> {
                lemma_target_shape_clip(fr,sw,sh,q,cell);
                crate::compose_new::lemma_cell_in_canvas(target_y(fr,q),target_x(fr,q),sw as int,sh as int,cv.len());
                crate::compose_new::lemma_write_pixel_pixels(cv,table,sw,sh,cell,v as int,fr,q,ts);
            },
        }
    }
}

pub proof fn lemma_write_pixels_commute(cv:Seq<u8>,table:Seq<u8>,c1:int,c2:int,v1:int,v2:int)
    requires 0<=c1,0<=c2,c1!=c2,4*c1+3<cv.len(),4*c2+3<cv.len(),
        0<=v1,0<=v2,3*v1+2<table.len(),3*v2+2<table.len(),
    ensures write_pixel(write_pixel(cv,4*c1,table,v1),4*c2,table,v2)
        ==write_pixel(write_pixel(cv,4*c2,table,v2),4*c1,table,v1),
{
    let a=write_pixel(write_pixel(cv,4*c1,table,v1),4*c2,table,v2);
    let b=write_pixel(write_pixel(cv,4*c2,table,v2),4*c1,table,v1);
    assert(a =~= b) by {
        assert forall|i:int| 0<=i<cv.len() implies #[trigger] a[i]==b[i] by {
            assert(4*c1+3<4*c2 || 4*c2+3<4*c1);
        }
    }
}

/// Adjacent writes at distinct raster positions commute on successful drawing,
/// including transparency, off-screen Clip, and interlace.
pub proof fn lemma_draw_steps_commute(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,p:int,q:int,v1:u8,v2:u8)
    requires cv.len()==4*(sw as int)*(sh as int),1<=sw,1<=sh,
        1<=fr.rect.width,1<=fr.rect.height<=crate::limits::MAX_DIMENSION,
        0<=fr.c,3*fr.c<=table.len(),
        fr.transparent==Some(v1) || (v1 as int)<fr.c,
        fr.transparent==Some(v2) || (v2 as int)<fr.c,
        0<=p<(fr.rect.width as int)*(fr.rect.height as int),
        0<=q<(fr.rect.width as int)*(fr.rect.height as int),p!=q,
    ensures draw_step(cv,fr,table,sw,sh,p,v1) is Ok,
        draw_step(cv,fr,table,sw,sh,q,v2) is Ok,
        draw_step(draw_step(cv,fr,table,sw,sh,p,v1).unwrap(),fr,table,sw,sh,q,v2)
            ==draw_step(draw_step(cv,fr,table,sw,sh,q,v2).unwrap(),fr,table,sw,sh,p,v1),
{
    if fr.transparent!=Some(v1) && fr.transparent!=Some(v2) {
        match (target_cell(fr,sw,sh,p),target_cell(fr,sw,sh,q)) {
            (Some(c1),Some(c2))=> {
                lemma_retained_cells_distinct(fr,sw,sh,p,q,c1,c2);
                lemma_target_shape_clip(fr,sw,sh,p,c1);
                lemma_target_shape_clip(fr,sw,sh,q,c2);
                crate::compose_new::lemma_cell_in_canvas(target_y(fr,p),target_x(fr,p),sw as int,sh as int,cv.len());
                crate::compose_new::lemma_cell_in_canvas(target_y(fr,q),target_x(fr,q),sw as int,sh as int,cv.len());
                lemma_write_pixels_commute(cv,table,c1,c2,v1 as int,v2 as int);
            },
            _=>{},
        }
    }
}
} // verus!

verus! {
pub open spec fn draw_domain(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32)->bool {
    &&& cv.len()==4*(sw as int)*(sh as int) && 1<=sw && 1<=sh
    &&& 1<=fr.rect.width && 1<=fr.rect.height<=crate::limits::MAX_DIMENSION
    &&& 0<=fr.c && 3*fr.c<=table.len()
}
pub open spec fn calls_ok(h:Seq<(u32,u8)>,fr:FrameSpec)->bool {
    &&& forall|i:int| 0<=i<h.len() ==> (#[trigger] h[i].0 as int)<(fr.rect.width as int)*(fr.rect.height as int)
        && (fr.transparent==Some(h[i].1) || (h[i].1 as int)<fr.c)
    &&& forall|i:int,j:int| 0<=i<h.len() && 0<=j<h.len() && #[trigger] h[i].0==#[trigger] h[j].0 ==> h[i].1==h[j].1
}
pub open spec fn draw_calls(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,h:Seq<(u32,u8)>)->Result<Seq<u8>,crate::compose_new::ComposeError>
    decreases h.len(),
{
    if h.len()==0 {Ok(cv)} else {
        match draw_step(cv,fr,table,sw,sh,h[0].0 as int,h[0].1) {
            Err(e)=>Err(e),
            Ok(next)=>draw_calls(next,fr,table,sw,sh,h.drop_first()),
        }
    }
}

pub proof fn lemma_calls_remove(h:Seq<(u32,u8)>,fr:FrameSpec,k:int)
    requires calls_ok(h,fr),0<=k<h.len(),
    ensures calls_ok(h.drop_first(),fr),calls_ok(h.remove(k),fr),
{
    h.remove_ensures(k);
    assert forall|i:int,j:int| 0<=i<h.remove(k).len() && 0<=j<h.remove(k).len()
        && #[trigger] h.remove(k)[i].0==#[trigger] h.remove(k)[j].0 implies h.remove(k)[i].1==h.remove(k)[j].1 by {
        let ii=if i<k {i} else {i+1};
        let jj=if j<k {j} else {j+1};
        assert(h.remove(k)[i]==h[ii]);assert(h.remove(k)[j]==h[jj]);
    }
}

pub proof fn lemma_draw_success_len(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,p:u32,v:u8)
    requires draw_domain(cv,fr,table,sw,sh),(p as int)<(fr.rect.width as int)*(fr.rect.height as int),
        fr.transparent==Some(v) || (v as int)<fr.c,
    ensures draw_step(cv,fr,table,sw,sh,p as int,v) is Ok,
        draw_domain(draw_step(cv,fr,table,sw,sh,p as int,v).unwrap(),fr,table,sw,sh),
{
    if fr.transparent!=Some(v) {
        match target_cell(fr,sw,sh,p as int) {
            None=>{},Some(c)=> {
                lemma_target_shape_clip(fr,sw,sh,p as int,c);
                crate::compose_new::lemma_cell_in_canvas(target_y(fr,p as int),target_x(fr,p as int),sw as int,sh as int,cv.len());
            },
        }
    }
}

/// Bring the selected put to the front by commuting it across preceding puts.
pub proof fn lemma_bubble_call(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,h:Seq<(u32,u8)>,k:int)
    requires draw_domain(cv,fr,table,sw,sh),calls_ok(h,fr),0<=k<h.len(),
    ensures draw_calls(cv,fr,table,sw,sh,h)==draw_calls(draw_step(cv,fr,table,sw,sh,h[k].0 as int,h[k].1).unwrap(),fr,table,sw,sh,h.remove(k)),
    decreases k,
{
    lemma_draw_success_len(cv,fr,table,sw,sh,h[0].0,h[0].1);
    lemma_draw_success_len(cv,fr,table,sw,sh,h[k].0,h[k].1);
    lemma_calls_remove(h,fr,k);
    if k==0 {
        assert(h.remove(0)==h.drop_first());
    } else {
        let tail=h.drop_first();
        let firstcv=draw_step(cv,fr,table,sw,sh,h[0].0 as int,h[0].1).unwrap();
        lemma_bubble_call(firstcv,fr,table,sw,sh,tail,k-1);
        if h[0].0!=h[k].0 {lemma_draw_steps_commute(cv,fr,table,sw,sh,h[0].0 as int,h[k].0 as int,h[0].1,h[k].1);}
        else {assert(h[0]==h[k]);}
        assert(h.remove(k).drop_first() =~= tail.remove(k-1));
        assert(h.remove(k)[0]==h[0]);
    }
}

/// Successful-canvas permutation theorem. Error canvases are intentionally not
/// compared; `calls_ok` requires each index to pass the colour check.
pub proof fn lemma_successful_canvas_permutation(cv:Seq<u8>,fr:FrameSpec,table:Seq<u8>,sw:u32,sh:u32,h:Seq<(u32,u8)>,k:Seq<(u32,u8)>)
    requires draw_domain(cv,fr,table,sw,sh),calls_ok(h,fr),calls_ok(k,fr),h.to_multiset()==k.to_multiset(),
    ensures draw_calls(cv,fr,table,sw,sh,h) is Ok,
        draw_calls(cv,fr,table,sw,sh,h)==draw_calls(cv,fr,table,sw,sh,k),
    decreases h.len(),
{
    broadcast use vstd::seq_lib::group_to_multiset_ensures;
    vstd::seq_lib::to_multiset_len(h);vstd::seq_lib::to_multiset_len(k);
    if h.len()==0 {assert(k.len()==0);} else {
        assert(h.contains(h[0]));
        vstd::seq_lib::to_multiset_contains(h,h[0]);
        vstd::seq_lib::to_multiset_contains(k,h[0]);
        assert(k.contains(h[0]));
        let i=k.lemma_contains_to_index(h[0]);
        lemma_bubble_call(cv,fr,table,sw,sh,k,i);
        lemma_calls_remove(h,fr,0);lemma_calls_remove(k,fr,i);
        lemma_draw_success_len(cv,fr,table,sw,sh,h[0].0,h[0].1);
        let nc=draw_step(cv,fr,table,sw,sh,h[0].0 as int,h[0].1).unwrap();
        assert(h.drop_first()==h.remove(0));
        assert(h.drop_first().to_multiset()==k.remove(i).to_multiset());
        lemma_successful_canvas_permutation(nc,fr,table,sw,sh,h.drop_first(),k.remove(i));
    }
}
} // verus!
