//! Independent canonical canvas fold; native histories occur only in proofs.
use vstd::prelude::*;
use crate::{document_model as d,raster_model as r,paint_model as p,colour_model as c,
    composite_model as comp,iterator_canvas_model as old,policy::STRICT};
verus! {
pub struct State {pub canvas: Seq<u8>,pub before_draw: Seq<u8>}
pub open spec fn bytes(doc: d::Doc) -> nat {4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat)}
pub open spec fn zero(doc: d::Doc) -> Seq<u8> {Seq::new(bytes(doc),|j: int|0u8)}
/// P14 pointwise paint from the independent P11/P13 source equation.
pub open spec fn draw(input: Seq<u8>,a: r::Raster,colour: c::Context,cw: nat,rect: comp::Rect,blend: bool,before: Seq<u8>) -> Seq<u8> {
    Seq::new(before.len(),|j: int| if comp::inside(cw,rect,j) {
        let col=(j%((4*cw) as int)-4*rect.x) as nat;
        let x=col/4;let y=(j/((4*cw) as int)-rect.y) as nat;let channel=(col%4) as int;
        let px=c::physical(p::source_pixel(input,a,colour,x,y,false,false).get_Ok_0(),false);
        let offset=j-channel;
        (if blend {comp::over(px,before.subrange(offset,offset+4))}else{px})[channel]
    }else{before[j]})
}
pub open spec fn prepared(doc: d::Doc,k: nat,previous: State) -> Seq<u8> {
    if k==0 {zero(doc)}else{
        p::disposed_reference(previous.canvas,Some(previous.before_draw),doc.ihdr.width as nat,old::regions(old::previous(doc,k)))
    }
}
pub open spec fn paint(b: Seq<u8>,doc: d::Doc,k: nat,before: Seq<u8>) -> Seq<u8> {
    let z=old::stream(b,doc,k).get_Some_0();let a=old::shape(doc,STRICT,k);
    draw(r::inflated(z,a,STRICT).out,a,c::from_doc(doc,b),doc.ihdr.width as nat,old::rectangle(doc,k),old::blend(doc,k),before)
}
#[verifier::opaque]
pub open spec fn fold(b: Seq<u8>,doc: d::Doc,n: nat) -> State
    decreases n,
{
    if n==0 {State {canvas:zero(doc),before_draw:zero(doc)}}else{
        let k=(n-1) as nat;let prior=fold(b,doc,k);let before=prepared(doc,k,prior);
        State {canvas:paint(b,doc,k,before),before_draw:before}
    }
}
pub proof fn fold_step(b: Seq<u8>,doc: d::Doc,n: nat)
    ensures fold(b,doc,n)==if n==0 {State {canvas:zero(doc),before_draw:zero(doc)}}else{
        let k=(n-1) as nat;let prior=fold(b,doc,k);let before=prepared(doc,k,prior);
        State {canvas:paint(b,doc,k,before),before_draw:before}},
{reveal_with_fuel(fold,2);}
pub open spec fn geometry(doc: d::Doc,k: nat) -> bool {
    let rect=old::rectangle(doc,k);let a=old::shape(doc,STRICT,k);
    p::domain(a) && a.width==rect.w && a.height==rect.h
    && rect.x+rect.w<=doc.ihdr.width && rect.y+rect.h<=doc.ihdr.height
}
/// Unique whole-canvas bytes, not a choice from the MK relation.
#[verifier::spinoff_prover]
pub proof fn mk_deterministic(z: Seq<u8>,a: r::Raster,colour: c::Context,cw: nat,rect: comp::Rect,blend: bool,
    previous: Option<(comp::Rect,u8)>,reference: Option<Seq<u8>>,before: Seq<u8>,after: Seq<u8>,reuse: bool)
    requires p::domain(a),a.width==rect.w,a.height==rect.h,cw>=1,rect.x+rect.w<=cw,
        p::MK(z,a,colour,STRICT,cw,rect,blend,previous,reference,before,after,reuse),
        (if reuse {before.len()}else{p::disposed_reference(before,reference,cw,previous).len()})==before.len(),
        4*(rect.y+rect.h)*cw<=before.len(),
    ensures after==draw(r::inflated(z,a,STRICT).out,a,colour,cw,rect,blend,
        if reuse {Seq::new(before.len(),|j: int|0u8)}else{p::disposed_reference(before,reference,cw,previous)}),
{
    hide(c::pixel);hide(r::inflated);
    p::mk_context(z,a,colour,STRICT,cw,rect,blend,previous,reference,before,after,reuse);
    let initial=if reuse {Seq::new(before.len(),|j: int|0u8)}else{p::disposed_reference(before,reference,cw,previous)};
    let shape=(2nat,before.len(),cw,rect.x,rect.y,blend);
    let input=r::inflated(z,a,STRICT).out;let expected=draw(input,a,colour,cw,rect,blend,initial);
    comp::drawing_frame_context(initial,after,shape,rect.w,rect.h);
    assert(c::geometry(shape,a.width,a.height,initial.len()));
    assert forall|j: int| 0<=j<after.len() implies #[trigger] after[j]==expected[j] by {
        if comp::inside(cw,rect,j) {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j,(4*cw) as int);
            vstd::arithmetic::div_mod::lemma_mod_pos_bound(j,(4*cw) as int);
            let col=(j%((4*cw) as int)-4*rect.x) as nat;
            let x=col/4;let y=(j/((4*cw) as int)-rect.y) as nat;let channel=col%4;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(col as int,4);
            assert(x<rect.w);assert(y<rect.h);
            p::pixel_observe(input,a,colour,shape,initial,after,false,x,y);
            c::observed_bounds(shape,a.width,a.height,initial.len(),x,y);
            let offset=c::destination_offset(shape,a.width,x,y);
            assert(offset+channel==j) by (nonlinear_arith)
                requires j==(4*cw)*(rect.y+y)+4*rect.x+col,col==4*x+channel,
                    offset==4*((rect.y+y)*cw+rect.x+x);
            assert(c::observe(shape,after,a.width,x,y)[channel as int]==after[j]);
            assert(c::observe(shape,initial,a.width,x,y)==initial.subrange(j-channel,j-channel+4));
        }
    }
    assert(after=~=expected);
}
pub proof fn lengths(b: Seq<u8>,doc: d::Doc,n: nat)
    ensures fold(b,doc,n).canvas.len()==bytes(doc),fold(b,doc,n).before_draw.len()==bytes(doc),
    decreases n,
{
    fold_step(b,doc,n);
    if n>0 {lengths(b,doc,(n-1) as nat);}
}
pub open spec fn covered(doc: d::Doc,n: nat,j: int) -> bool {
    exists|k: nat| k<n && comp::inside(doc.ihdr.width as nat,old::rectangle(doc,k),j)
}
pub proof fn covered_step(doc: d::Doc,n: nat,j: int)
    ensures covered(doc,n+1,j)==(covered(doc,n,j) || comp::inside(doc.ihdr.width as nat,old::rectangle(doc,n),j)),
{
    if covered(doc,n+1,j) {
        let k=choose|k: nat| k<n+1 && comp::inside(doc.ihdr.width as nat,old::rectangle(doc,k),j);
        assert(k<n || k==n);
    }
    if covered(doc,n,j) {
        let k=choose|k: nat| k<n && comp::inside(doc.ihdr.width as nat,old::rectangle(doc,k),j);
        assert(k<n+1);
    }
}
pub proof fn untouched(b: Seq<u8>,doc: d::Doc,n: nat,j: int)
    requires 0<=j<bytes(doc),!covered(doc,n,j),
    ensures fold(b,doc,n).canvas[j]==0u8,fold(b,doc,n).before_draw[j]==0u8,
    decreases n,
{
    fold_step(b,doc,n);
    if n>0 {
        let k=(n-1) as nat;covered_step(doc,k,j);
        untouched(b,doc,k,j);lengths(b,doc,k);
        let prior=fold(b,doc,k);let initial=prepared(doc,k,prior);
        assert(initial.len()==bytes(doc));
        assert(initial[j]==0u8);
        assert(!comp::inside(doc.ihdr.width as nat,old::rectangle(doc,k),j));
    }
}
pub open spec fn document_geometry(doc: d::Doc) -> bool {
    doc.ihdr.width>=1 && doc.ihdr.height>=1
    && forall|k: nat| k<crate::iterator_data_model::count(doc) ==> #[trigger] geometry(doc,k)
}
pub open spec fn streams(b: Seq<u8>,doc: d::Doc) -> bool {
    forall|k: nat| k<crate::iterator_data_model::count(doc) ==> (#[trigger] old::stream(b,doc,k)).is_some()
}
pub open spec fn agrees(b: Seq<u8>,doc: d::Doc,t: old::Trace,n: nat) -> bool {
    n<=t.after.len() && forall|j: nat| j<n ==> (#[trigger] t.after[j as int])==fold(b,doc,j+1).canvas
        && t.before[j as int]==fold(b,doc,j+1).before_draw
}
#[verifier::spinoff_prover]
pub proof fn history_prefix(b: Seq<u8>,doc: d::Doc,t: old::Trace,n: nat)
    requires old::wf(b,doc,STRICT,t),document_geometry(doc),streams(b,doc),n<=t.after.len(),
    ensures agrees(b,doc,t,n),
    decreases n,
{
    hide(c::pixel);hide(r::inflated);
    old::context(b,doc,STRICT,t);
    if n>0 {
        let k=(n-1) as nat;
        history_prefix(b,doc,t,k);lengths(b,doc,k);fold_step(b,doc,k+1);
        assert(k<t.after.len());
        assert(t.before[k as int]==old::before_draw(doc,t,k));
        assert(old::step(b,doc,STRICT,k,old::canvas_before(t,k),old::reference(t,k),t.after[k as int]));
        assert(geometry(doc,k));
        let before=old::canvas_before(t,k);let reference=old::reference(t,k);
        let expected_before=prepared(doc,k,fold(b,doc,k));
        let actual_before=old::before_draw(doc,t,k);
        if k==0 {
            if !old::reuse(doc,k) {
                p::initial_transparent_context(b,doc,STRICT,t.initial);
                assert(t.initial=~=zero(doc));
            }
        }else{
            assert(t.after[(k-1) as int]==fold(b,doc,k).canvas);
            assert(t.before[(k-1) as int]==fold(b,doc,k).before_draw);
        }
        assert(actual_before==expected_before);
        let z=old::stream(b,doc,k).get_Some_0();
        old::step_context(b,doc,STRICT,k,z,before,reference,t.after[k as int]);
        let rect=old::rectangle(doc,k);let a=old::shape(doc,STRICT,k);let width=doc.ihdr.width as nat;
        assert(before.len()==bytes(doc));
        assert(4*(rect.y+rect.h)*width<=bytes(doc)) by (nonlinear_arith)
            requires rect.y+rect.h<=doc.ihdr.height,bytes(doc)==4*width*(doc.ihdr.height as nat),width>=1;
        assert((if old::reuse(doc,k) {before.len()}else{p::disposed_reference(before,reference,width,old::regions(old::previous(doc,k))).len()})==before.len());
        mk_deterministic(z,a,c::from_doc(doc,b),width,rect,old::blend(doc,k),old::regions(old::previous(doc,k)),reference,before,t.after[k as int],old::reuse(doc,k));
        assert(t.after[k as int]==fold(b,doc,k+1).canvas);
        assert(t.before[k as int]==fold(b,doc,k+1).before_draw);
        assert forall|j: nat| j<n implies (#[trigger] t.after[j as int])==fold(b,doc,j+1).canvas
            && t.before[j as int]==fold(b,doc,j+1).before_draw by {
            if j<k {}else{assert(j==k);}
        }
    }
}
pub proof fn history_complete(b: Seq<u8>,doc: d::Doc,t: old::Trace)
    requires old::wf(b,doc,STRICT,t),document_geometry(doc),streams(b,doc),
    ensures agrees(b,doc,t,t.after.len()),
{history_prefix(b,doc,t,t.after.len());}

}
