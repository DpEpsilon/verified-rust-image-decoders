//! Independent per-position compositor history at public iterator boundaries.
//! Before-draw references are derived from earlier steps and canonical controls,
//! never chosen from the executable's Saved contents or snapshot field.
use vstd::prelude::*;
use crate::{document_model as d,iterator_data_model as data,paint_model as paint,
    composite_model as comp,colour_model as colour,raster_model as raster,policy::Policy};
verus! {
pub struct Trace {pub initial: Seq<u8>,pub before: Seq<Seq<u8>>,pub after: Seq<Seq<u8>>}
pub open spec fn empty(initial: Seq<u8>) -> Trace {Trace {initial,before:Seq::empty(),after:Seq::empty()}}
pub open spec fn effective(doc: d::Doc,k: nat) -> u8 {
    if doc.actl.is_none() {0}else{let op=doc.frames[k as int].dispose;if k==0 && op==2 {1}else{op}}
}
pub open spec fn rectangle(doc: d::Doc,k: nat) -> comp::Rect {
    if doc.actl.is_none() {comp::rect(0,0,doc.ihdr.width,doc.ihdr.height)}else{
        let f=doc.frames[k as int];comp::rect(f.x,f.y,f.width,f.height)}
}
pub open spec fn blend(doc: d::Doc,k: nat) -> bool {doc.actl.is_some() && doc.frames[k as int].blend==1}
pub open spec fn reuse(doc: d::Doc,k: nat) -> bool {k==0 && (doc.actl.is_none() || doc.info.default_is_frame)}
pub open spec fn previous(doc: d::Doc,k: nat) -> Option<((u32,u32,u32,u32),u8)> {
    if doc.actl.is_some() && 0<k<=doc.frames.len() {
        let j=(k-1) as nat;let f=doc.frames[j as int];Some(((f.x,f.y,f.width,f.height),effective(doc,j)))
    }else{None}
}
pub open spec fn regions(prev: Option<((u32,u32,u32,u32),u8)>) -> Option<(comp::Rect,u8)> {
    match prev {Some(((x,y,w,h),op))=>Some((comp::rect(x,y,w,h),op)),None=>None}
}
pub open spec fn stream(b: Seq<u8>,doc: d::Doc,k: nat) -> Option<Seq<u8>> {
    if doc.actl.is_none() {d::default_stream(b,doc)}else{d::frame_stream(b,doc,k)}
}
pub open spec fn shape(doc: d::Doc,p: Policy,k: nat) -> raster::Raster {
    if doc.actl.is_none() {paint::default_raster(doc,p)}else{data::frame_shape(doc,p,k)}
}
pub open spec fn canvas_before(t: Trace,k: nat) -> Seq<u8> {
    if k==0 {t.initial}else{t.after[(k-1) as int]}
}
pub open spec fn reference(t: Trace,k: nat) -> Option<Seq<u8>> {
    if 0<k<=t.before.len() {Some(t.before[(k-1) as int])}else{None}
}
pub open spec fn before_draw(doc: d::Doc,t: Trace,k: nat) -> Seq<u8> {
    let cv=canvas_before(t,k);
    if reuse(doc,k) {Seq::new(cv.len(),|j: int|0u8)}else{
        paint::disposed_reference(cv,reference(t,k),doc.ihdr.width as nat,regions(previous(doc,k)))}
}
#[verifier::opaque]
pub open spec fn step(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,before: Seq<u8>,ref_before_previous: Option<Seq<u8>>,after: Seq<u8>) -> bool {
    k<data::count(doc) && match stream(b,doc,k) {
        Some(z)=>paint::MK(z,shape(doc,p,k),colour::from_doc(doc,b),p,doc.ihdr.width as nat,
            rectangle(doc,k),blend(doc,k),regions(previous(doc,k)),ref_before_previous,before,after,reuse(doc,k)),
        None=>false,
    }
}
pub proof fn step_intro(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,z: Seq<u8>,before: Seq<u8>,reference: Option<Seq<u8>>,after: Seq<u8>)
    requires k<data::count(doc),stream(b,doc,k)==Some(z),
        paint::MK(z,shape(doc,p,k),colour::from_doc(doc,b),p,doc.ihdr.width as nat,
            rectangle(doc,k),blend(doc,k),regions(previous(doc,k)),reference,before,after,reuse(doc,k)),
    ensures step(b,doc,p,k,before,reference,after),
{reveal(step);}
pub proof fn step_context(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,z: Seq<u8>,before: Seq<u8>,reference: Option<Seq<u8>>,after: Seq<u8>)
    requires step(b,doc,p,k,before,reference,after),stream(b,doc,k)==Some(z),
    ensures k<data::count(doc),paint::MK(z,shape(doc,p,k),colour::from_doc(doc,b),p,doc.ihdr.width as nat,
        rectangle(doc,k),blend(doc,k),regions(previous(doc,k)),reference,before,after,reuse(doc,k)),
{reveal(step);}
pub proof fn reference_irrelevant(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,before: Seq<u8>,a: Option<Seq<u8>>,c: Option<Seq<u8>>,after: Seq<u8>)
    requires step(b,doc,p,k,before,a,after),
        match previous(doc,k) {Some((_,2))=>a==c,_=>true},
    ensures step(b,doc,p,k,before,c,after),
{
    reveal(step);
    if let Some(z)=stream(b,doc,k) {
        paint::mk_reference(z,shape(doc,p,k),colour::from_doc(doc,b),p,doc.ihdr.width as nat,
            rectangle(doc,k),blend(doc,k),regions(previous(doc,k)),a,c,before,after,reuse(doc,k));
    }
}
#[verifier::opaque]
pub open spec fn wf(b: Seq<u8>,doc: d::Doc,p: Policy,t: Trace) -> bool {
    t.initial.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat)
    && paint::initial_canvas(b,doc,p,t.initial)
    && t.before.len()==t.after.len() && t.after.len()<=data::count(doc)
    && forall|j: int| 0<=j<t.after.len() ==>
        (#[trigger] t.before[j])==before_draw(doc,t,j as nat)
        && t.after[j].len()==t.initial.len()
        && step(b,doc,p,j as nat,canvas_before(t,j as nat),reference(t,j as nat),t.after[j])
}
pub proof fn context(b: Seq<u8>,doc: d::Doc,p: Policy,t: Trace)
    requires wf(b,doc,p,t),
    ensures t.initial.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),paint::initial_canvas(b,doc,p,t.initial),
        t.before.len()==t.after.len(),t.after.len()<=data::count(doc),
        forall|j: int| 0<=j<t.after.len() ==> (#[trigger] t.before[j])==before_draw(doc,t,j as nat)
            && t.after[j].len()==t.initial.len()
            && step(b,doc,p,j as nat,canvas_before(t,j as nat),reference(t,j as nat),t.after[j]),
{reveal(wf);}
pub proof fn current_len(b: Seq<u8>,doc: d::Doc,p: Policy,t: Trace)
    requires wf(b,doc,p,t),
    ensures canvas_before(t,t.after.len()).len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),
{
    context(b,doc,p,t);
    if t.after.len()>0 {
        let j=t.after.len()-1;
        assert(t.before[j]==before_draw(doc,t,j as nat));
        assert(t.after[j].len()==t.initial.len());
    }
}
pub proof fn initial(b: Seq<u8>,doc: d::Doc,p: Policy,pixels: Seq<u8>)
    requires pixels.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),paint::initial_canvas(b,doc,p,pixels),
    ensures wf(b,doc,p,empty(pixels)),
{reveal(wf);}
pub proof fn static_step(b: Seq<u8>,doc: d::Doc,p: Policy,before: Seq<u8>)
    requires doc.actl.is_none(),paint::domain(paint::default_raster(doc,p)),
        1<=doc.ihdr.width,before.len()==4*(doc.ihdr.width as nat)*(doc.ihdr.height as nat),
        paint::initial_canvas(b,doc,p,before),
    ensures step(b,doc,p,0,before,None,before),
{
    paint::initial_default_source(b,doc,p,before);
    let z=d::default_stream(b,doc).unwrap();let a=shape(doc,p,0);let black=p.palette_index==crate::policy::PaletteIndex::Black;
    let zero=Seq::new(before.len(),|j: int|0u8);
    paint::reuse_progress(raster::inflated(z,a,p).out,a,colour::from_doc(doc,b),before,false,black);
    comp::full_drawing_frame(zero,before,doc.ihdr.width as nat,doc.ihdr.height as nat,false);
    paint::mk_intro(z,a,colour::from_doc(doc,b),p,doc.ihdr.width as nat,rectangle(doc,0),false,None,None,before,before,true);
    step_intro(b,doc,p,0,z,before,None,before);
}
pub open spec fn append(doc: d::Doc,t: Trace,after: Seq<u8>) -> Trace {
    Trace {initial:t.initial,before:t.before.push(before_draw(doc,t,t.after.len())),after:t.after.push(after)}
}
pub proof fn append_wf(b: Seq<u8>,doc: d::Doc,p: Policy,t: Trace,after: Seq<u8>)
    requires wf(b,doc,p,t),t.after.len()<data::count(doc),after.len()==t.initial.len(),
        step(b,doc,p,t.after.len(),canvas_before(t,t.after.len()),reference(t,t.after.len()),after),
    ensures wf(b,doc,p,append(doc,t,after)),
        append(doc,t,after).before.last()==before_draw(doc,t,t.after.len()),
        append(doc,t,after).after.last()==after,
        append(doc,t,after).after.len()==t.after.len()+1,
{
    reveal(wf);
    let n=t.after.len();let next=append(doc,t,after);
    assert forall|j: int| 0<=j<next.after.len() implies
        (#[trigger] next.before[j])==before_draw(doc,next,j as nat)
        && next.after[j].len()==next.initial.len()
        && step(b,doc,p,j as nat,canvas_before(next,j as nat),reference(next,j as nat),next.after[j]) by {
        if j<n {
            assert(canvas_before(next,j as nat)==canvas_before(t,j as nat));
            assert(reference(next,j as nat)==reference(t,j as nat));
        }else{assert(j==n);}
    }
}
} // verus!
