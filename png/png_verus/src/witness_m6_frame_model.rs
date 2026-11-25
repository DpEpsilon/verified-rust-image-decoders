//! Proof-only instantiation of the independent canonical canvas in accepted MK.
use vstd::prelude::*;
use crate::{frame_canvas_model as fc,colour_model as c,paint_model as p,
    composite_model as comp,raster_model as r,document_model as d,
    iterator_canvas_model as old,policy::STRICT};
verus! {
pub(crate) proof fn extend_member<T>(small: Seq<(crate::alloc_guard::AllocSite,nat)>,
    large: Seq<(crate::alloc_guard::AllocSite,nat)>,result: Result<T,crate::alloc_guard::TryError>)
    requires small.is_prefix_of(large),crate::still_equivalence::oom_member(small,result),
    ensures crate::still_equivalence::oom_member(large,result),
{
    if let Err(crate::alloc_guard::TryError::OutOfMemory(o))=result {
        let pair=(o.site_spec(),o.elements_spec() as nat);
        let i=choose|i: int| 0<=i<small.len() && small[i]==pair;
        assert(large[i]==pair);
        assert(large.contains(pair));
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn draw_observe(input: Seq<u8>,a: r::Raster,colour: c::Context,cw: nat,
    rect: comp::Rect,blend: bool,before: Seq<u8>,x: nat,y: nat)
    requires p::domain(a),crate::model::valid_depth(colour.ct,colour.depth),
        a.width==rect.w,a.height==rect.h,cw>=1,rect.x+rect.w<=cw,
        4*(rect.y+rect.h)*cw<=before.len(),x<a.width,y<a.height,
        p::source_pixel(input,a,colour,x,y,false,false).is_ok(),
    ensures c::observe((2nat,before.len(),cw,rect.x,rect.y,blend),
        fc::draw(input,a,colour,cw,rect,blend,before),a.width,x,y)
        ==c::converted((2nat,before.len(),cw,rect.x,rect.y,blend),
            p::source_pixel(input,a,colour,x,y,false,false).get_Ok_0(),
            c::observe((2nat,before.len(),cw,rect.x,rect.y,blend),before,a.width,x,y)),
{
    hide(c::pixel);
    let shape=(2nat,before.len(),cw,rect.x,rect.y,blend);
    let after=fc::draw(input,a,colour,cw,rect,blend,before);
    assert(c::geometry(shape,a.width,a.height,before.len()));
    c::observed_bounds(shape,a.width,a.height,before.len(),x,y);
    c::destination_grid(shape,a.width,x,y);
    let offset=c::destination_offset(shape,a.width,x,y);
    let at=p::source_cell(a,x,y);
    crate::still_samples::pixel_pair(colour,r::line(input,a,at.0,at.2),at.1);
    let px=p::source_pixel(input,a,colour,x,y,false,false).get_Ok_0();
    let value=c::converted(shape,px,c::observe(shape,before,a.width,x,y));
    assert(px.len()==4 && value.len()==4);
    let actual=c::observe(shape,after,a.width,x,y);
    assert(actual.len()==4);
    assert forall|channel: int| 0<=channel<4 implies #[trigger] actual[channel]==value[channel] by {
        let j=offset+channel;
        assert(0<=4*(rect.x+x)+channel<4*cw) by (nonlinear_arith)
            requires rect.x+x<cw,0<=channel<4,cw>=1;
        assert(j==4*cw*(rect.y+y)+(4*(rect.x+x)+channel)) by (nonlinear_arith)
            requires offset==4*((rect.y+y)*cw+rect.x+x),j==offset+channel;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(j,(4*cw) as int,
            (rect.y+y) as int,4*(rect.x+x)+channel);
        assert(comp::inside(cw,rect,j));
        let col=(j%((4*cw) as int)-4*rect.x) as nat;
        assert(col==4*x+channel);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(col as int,4,x as int,channel);
        assert(col/4==x && col%4==channel);
        assert((j/((4*cw) as int)-rect.y) as nat==y);
        assert(j-channel==offset);
        assert(after[j]==value[channel]);
    }
    assert(actual=~=value);
}
#[verifier::spinoff_prover]
pub(crate) proof fn canonical_step(b: Seq<u8>,doc: d::Doc,k: nat)
    requires k<crate::iterator_data_model::count(doc),fc::geometry(doc,k),doc.ihdr.width>=1,
        crate::model::valid_depth(c::from_doc(doc,b).ct,c::from_doc(doc,b).depth),
        doc.ihdr.color_type==c::from_doc(doc,b).ct,doc.ihdr.depth==c::from_doc(doc,b).depth,
        old::shape(doc,STRICT,k).entries==c::from_doc(doc,b).plte.len()/3,
        old::stream(b,doc,k).is_some(),
        r::scan(r::inflated(old::stream(b,doc,k).get_Some_0(),old::shape(doc,STRICT,k),STRICT).out,
            old::shape(doc,STRICT,k),0).is_ok(),
    ensures old::step(b,doc,STRICT,k,
        if k==0 {fc::zero(doc)}else{fc::fold(b,doc,k).canvas},
        if k==0 {None}else{Some(fc::fold(b,doc,k).before_draw)},
        fc::fold(b,doc,k+1).canvas),
{
    hide(c::pixel);hide(r::inflated);
    fc::fold_step(b,doc,k+1);fc::lengths(b,doc,k);
    let prior=fc::fold(b,doc,k);
    let before=if k==0 {fc::zero(doc)}else{prior.canvas};
    let reference=if k==0 {None}else{Some(prior.before_draw)};
    let initial=fc::prepared(doc,k,prior);
    let z=old::stream(b,doc,k).get_Some_0();let a=old::shape(doc,STRICT,k);
    let colour=c::from_doc(doc,b);let rect=old::rectangle(doc,k);let cw=doc.ihdr.width as nat;
    let blend=old::blend(doc,k);let after=fc::fold(b,doc,k+1).canvas;
    let shape=(2nat,before.len(),cw,rect.x,rect.y,blend);
    assert(initial.len()==before.len());
    assert(initial==if old::reuse(doc,k) {Seq::new(before.len(),|j: int|0u8)}else{
        p::disposed_reference(before,reference,cw,old::regions(old::previous(doc,k)))});
    assert(4*(rect.y+rect.h)*cw<=before.len()) by (nonlinear_arith)
        requires rect.y+rect.h<=doc.ihdr.height,before.len()==4*cw*(doc.ihdr.height as nat),cw>=1;
    let input=r::inflated(z,a,STRICT).out;
    assert(after==fc::draw(input,a,colour,cw,rect,blend,initial));
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        (#[trigger] p::source_pixel(input,a,colour,x,y,false,false)).is_ok()
        && c::observe(shape,after,a.width,x,y)==c::converted(shape,
            p::source_pixel(input,a,colour,x,y,false,false).get_Ok_0(),c::observe(shape,initial,a.width,x,y)) by {
        crate::still_model::source_pixel_ok(input,a,colour,x,y);
        draw_observe(input,a,colour,cw,rect,blend,initial,x,y);
    }
    p::complete_progress_intro(input,a,colour,shape,initial,after,false);
    assert forall|j: int| 0<=j<initial.len() && !comp::drawing_region(shape,a.width,a.height,j)
        implies #[trigger] after[j]==initial[j] by {}
    comp::drawing_frame_intro(initial,after,shape,a.width,a.height);
    p::mk_intro(z,a,colour,STRICT,cw,rect,blend,old::regions(old::previous(doc,k)),reference,before,after,old::reuse(doc,k));
    old::step_intro(b,doc,STRICT,k,z,before,reference,after);
}

pub struct Observation {pub info: Option<d::Info>,pub frames: Seq<crate::frame_model::Frame>}
pub open spec fn observed_result(value: crate::frame_model::Value,
    result: Result<(),crate::alloc_guard::TryError>,observed: Observation) -> bool {
    match result {
        Err(crate::alloc_guard::TryError::OutOfMemory(_))=>observed.frames.is_prefix_of(value.frames)
            && match observed.info {Some(info)=>info==value.info,None=>observed.frames.len()==0},
        _=>observed.info==Some(value.info) && observed.frames==value.frames
            && match result {
                Ok(())=>value.end==crate::iterator_data_model::End::Iend,
                Err(crate::alloc_guard::TryError::Format(e))=>value.end==crate::iterator_data_model::End::Format(e),
                _=>false,
            },
    }
}
/// A real ordinary constructor and call sequence, retaining ghost observations
/// of every native frame, not constructing an executable expected canvas.
#[verifier::spinoff_prover]
pub(crate) fn exercise(b: &[u8],count: usize,Ghost(value): Ghost<crate::frame_model::Value>)
    -> (result: (Result<(),crate::alloc_guard::TryError>,Ghost<Observation>))
    requires crate::frame_model::MFr(b@)==crate::frame_model::Outcome::Done(value),
        value.frames.len()==count,
        crate::frame_equivalence::requests(b@).is_prefix_of(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1)),
    ensures observed_result(value,result.0,result.1@),
        crate::still_equivalence::oom_member(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1),result.0),
{
    hide(crate::frame_model::MFr);hide(crate::frame_equivalence::requests);
    let constructor=crate::png::Frames::try_new(b);
    proof {
        crate::frame_equivalence::observe_constructor(b@,constructor);
        extend_member(crate::frame_equivalence::requests(b@),crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1),constructor);
    }
    let mut dec=match constructor {
        Ok(dec)=>dec,
        Err(crate::alloc_guard::TryError::OutOfMemory(o))=>return (Err(crate::alloc_guard::TryError::OutOfMemory(o)),Ghost(Observation {info:None,frames:Seq::empty()})),
        _=>{assert(false);return (Ok(()),Ghost(Observation {info:None,frames:Seq::empty()}));},
    };
    proof {
        dec.strict_iterator_context();dec.strict_context();
        crate::frame_equivalence::context(b@,dec.model_document(),dec.info_value().model_value(),dec.canvas_trace(),dec.history_view());
        crate::frame_equivalence::canonical_requests(b@,dec.model_document(),count as nat+1);
    }
    let ghost doc=dec.model_document();
    let info=dec.info();
    proof {info.public_observations(b.len() as nat);}
    let ghost actual_info=info.model_value();
    assert(actual_info==value.info);
    assert(info.width==value.info.header.width && info.height==value.info.header.height
        && info.num_plays==value.info.num_plays);
    let ghost mut seen=Seq::<crate::frame_model::Frame>::empty();
    let mut index=0usize;
    while index<count
        invariant index<=count,value.frames.len()==count,
            crate::frame_model::MFr(b@)==crate::frame_model::Outcome::Done(value),
            dec.strict_iterator_wf(),dec.model_input()==b@,dec.model_document()==doc,
            dec.history_view().returned==index,
            matches!(dec.history_view().phase,crate::iterator_data_model::Phase::Active)
                || matches!(dec.history_view().phase,crate::iterator_data_model::Phase::Iend),
            seen==value.frames.take(index as int),actual_info==value.info,
            crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1)
                ==crate::frame_equivalence::requests(b@)+if match crate::iterator_data_model::SavedAt_p(b@,STRICT) {Some(k)=>k<count as nat+1,None=>false}
                    {seq![(crate::alloc_guard::AllocSite::Saved,fc::bytes(doc))]}else{Seq::empty()},
        decreases count-index,
    {
        proof {
            dec.strict_iterator_context();dec.strict_context();
            crate::frame_equivalence::context(b@,doc,dec.info_value().model_value(),dec.canvas_trace(),dec.history_view());
        }
        assert(matches!(dec.history_view().phase,crate::iterator_data_model::Phase::Active));
        assert(dec.history_view().calls==index);
        let ghost before=dec.history_view();
        let next=dec.try_next();
        proof {crate::frame_equivalence::observe_call(b@,doc,before,crate::frame_equivalence::native_call(next));}
        match next {
            Ok(Some(view))=>{
                assert(view.model_value()==value.frames[index as int]);
                assert(!view.is_recovered());
                proof {seen=seen.push(view.model_value());}
            },
            Err(crate::alloc_guard::TryError::OutOfMemory(o))=>{
                assert(o.site_spec()==crate::alloc_guard::AllocSite::Saved);
                assert(o.elements_spec() as nat==fc::bytes(doc));
                assert(crate::iterator_data_model::SavedAt_p(b@,STRICT)==Some(index as nat));
                assert(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1)[crate::frame_equivalence::requests(b@).len() as int]
                    ==(o.site_spec(),o.elements_spec() as nat));
                assert(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1).contains((o.site_spec(),o.elements_spec() as nat)));
                return (Err(crate::alloc_guard::TryError::OutOfMemory(o)),Ghost(Observation {info:Some(actual_info),frames:seen}));
            },
            _=>{assert(false);return (Ok(()),Ghost(Observation {info:Some(actual_info),frames:seen}));},
        }
        assert(seen=~=value.frames.take(index as int+1));
        index+=1;
    }
    assert(seen=~=value.frames);
    proof {
        dec.strict_iterator_context();dec.strict_context();
        crate::frame_equivalence::context(b@,doc,dec.info_value().model_value(),dec.canvas_trace(),dec.history_view());
    }
    let ghost before=dec.history_view();
    let terminal=dec.try_next();
    proof {crate::frame_equivalence::observe_call(b@,doc,before,crate::frame_equivalence::native_call(terminal));}
    let result=match terminal {
        Ok(None)=>{assert(value.end==crate::iterator_data_model::End::Iend);Ok(())},
        Err(crate::alloc_guard::TryError::Format(error))=>{assert(value.end==crate::iterator_data_model::End::Format(error));Err(crate::alloc_guard::TryError::Format(error))},
        Err(crate::alloc_guard::TryError::OutOfMemory(o))=>{
            assert(before.calls==count);
            assert(o.site_spec()==crate::alloc_guard::AllocSite::Saved);
            assert(o.elements_spec() as nat==fc::bytes(doc));
            assert(crate::iterator_data_model::SavedAt_p(b@,STRICT)==Some(count as nat));
            assert(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1)[crate::frame_equivalence::requests(b@).len() as int]
                ==(o.site_spec(),o.elements_spec() as nat));
            assert(crate::iterator_data_model::requests_p(b@,STRICT,count as nat+1).contains((o.site_spec(),o.elements_spec() as nat)));
            return (Err(crate::alloc_guard::TryError::OutOfMemory(o)),Ghost(Observation {info:Some(actual_info),frames:seen}));
        },
        _=>{assert(false);return (Ok(()),Ghost(Observation {info:Some(actual_info),frames:seen}));},
    };
    let ghost terminal_log=dec.log_view();
    proof {dec.strict_iterator_context();}
    let fused=dec.try_next();
    assert(matches!(fused,Ok(None)));
    assert(dec.log_view()==terminal_log);
    proof {dec.terminal_no_work();}
    (result,Ghost(Observation {info:Some(actual_info),frames:seen}))
}
}
