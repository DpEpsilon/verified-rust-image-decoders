//! Complete Strict MFr: independent control, first-error prefix and canvases.
use vstd::prelude::*;
use crate::{document_model as d,raster_model as r,iterator_data_model as data,
    iterator_canvas_model as controls,frame_canvas_model as canvas,still_model as still,
    composite_model as comp,policy::STRICT,png::Error};
verus! {
pub struct Frame {pub rgba: Seq<u8>,pub rect: comp::Rect,pub delay: Option<(u16,u16)>,pub dispose: u8,pub blend: u8}
pub struct Value {pub info: d::Info,pub num_plays: Option<u32>,pub frames: Seq<Frame>,pub end: data::End}
pub enum Outcome {Error(Error),Done(Value),OutsideDomain}
pub open spec fn record(b: Seq<u8>,doc: d::Doc,k: nat) -> Frame {
    Frame {rgba:canvas::fold(b,doc,k+1).canvas,rect:controls::rectangle(doc,k),
        delay:if doc.actl.is_none() {None}else{Some((doc.frames[k as int].delay_num,doc.frames[k as int].delay_den))},
        dispose:if doc.actl.is_none() {0}else{doc.frames[k as int].dispose},
        blend:if doc.actl.is_none() {0}else{doc.frames[k as int].blend}}
}
pub open spec fn from_document(b: Seq<u8>,doc: d::Doc) -> Outcome {
    match r::document_data(b,doc,STRICT) {
        Some(Err(e))=>Outcome::Error(e),
        Some(Ok(_))=>{
            let summary=data::MFr_data(b,doc,STRICT);
            Outcome::Done(Value {info:doc.info,num_plays:doc.info.num_plays,
                frames:Seq::new(summary.frames,|k: int|record(b,doc,k as nat)),end:summary.end})
        },None=>Outcome::OutsideDomain,
    }
}
pub open spec fn MFr(b: Seq<u8>) -> Outcome {
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Error(e,_)=>Outcome::Error(e),
        d::Outcome::Done(doc,_)=>from_document(b,data::constructor_doc(doc)),
        _=>Outcome::OutsideDomain,
    }
}
pub open spec fn document_total(b: Seq<u8>,doc: d::Doc) -> bool {
    r::document_data(b,doc,STRICT).is_some()
    && forall|k: nat| k<data::count(doc) ==> (#[trigger] data::frame_data(b,doc,STRICT,k)).is_some()
}
pub proof fn suffix_total(b: Seq<u8>,doc: d::Doc,start: nat)
    requires start<=data::count(doc),document_total(b,doc),
    ensures !matches!(data::suffix(b,doc,STRICT,start).end,data::End::OutsideDomain),
    decreases data::count(doc)-start,
{
    data::suffix_step(b,doc,STRICT,start);
    if start<data::count(doc) {
        assert(data::frame_data(b,doc,STRICT,start).is_some());
        if data::frame_ok(b,doc,STRICT,start) {suffix_total(b,doc,start+1);}
    }
}
/// Prefix success and its first failed data result, not a native call history.
pub proof fn summary_prefix(b: Seq<u8>,doc: d::Doc,start: nat)
    requires start<=data::count(doc),document_total(b,doc),
    ensures data::prefix_from(b,doc,STRICT,start,data::suffix(b,doc,STRICT,start).frames),
        match data::suffix(b,doc,STRICT,start).end {
            data::End::Iend=>data::suffix(b,doc,STRICT,start).frames==data::count(doc),
            data::End::Format(e)=>data::suffix(b,doc,STRICT,start).frames<data::count(doc)
                && data::frame_data(b,doc,STRICT,data::suffix(b,doc,STRICT,start).frames)==Some(Err(e)),
            data::End::OutsideDomain=>false,
        },
    decreases data::count(doc)-start,
{
    data::suffix_step(b,doc,STRICT,start);data::summary_bounds(b,doc,STRICT,start);
    if start<data::count(doc) {
        assert(data::frame_data(b,doc,STRICT,start).is_some());
        if data::frame_ok(b,doc,STRICT,start) {
            summary_prefix(b,doc,start+1);
            let n=data::suffix(b,doc,STRICT,start).frames;
            assert forall|j: nat| start<=j<n implies #[trigger] data::frame_ok(b,doc,STRICT,j) by {
                if j==start {}else{assert(start+1<=j);}
            }
        }
    }
}
pub open spec fn resources(value: Value) -> bool {
    still::checked_header(value.info.header) && value.num_plays==value.info.num_plays
    && forall|k: int| 0<=k<value.frames.len() ==> (#[trigger] value.frames[k]).rgba.len()
        ==4*(value.info.header.width as nat)*(value.info.header.height as nat)<=crate::limits::MAX_OUTPUT
}
pub proof fn complete_resources(b: Seq<u8>,doc: d::Doc)
    requires still::checked_header(doc.ihdr),doc.info.header==doc.ihdr,
    ensures match from_document(b,doc) {Outcome::Done(value)=>resources(value),_=>true},
{
    hide(r::document_data);
    if let Outcome::Done(value)=from_document(b,doc) {
        assert forall|k: int| 0<=k<value.frames.len() implies (#[trigger] value.frames[k]).rgba.len()
            ==4*(value.info.header.width as nat)*(value.info.header.height as nat)<=crate::limits::MAX_OUTPUT by {
            canvas::lengths(b,doc,k as nat+1);
        }
    }
}
/// Saved is reserved before frame k's data, hence the inclusive k <= length.
pub proof fn saved_at_complete(b: Seq<u8>,doc: d::Doc,k: nat)
    requires data::data_ok(r::document_data(b,doc,STRICT)),document_total(b,doc),k<data::count(doc),
    ensures data::is_saved_at(b,doc,STRICT,k)==match from_document(b,doc) {
        Outcome::Done(value)=>data::qualifies(doc,k) && k<=value.frames.len()
            && forall|j: nat| 1<=j<k ==> !#[trigger] data::qualifies(doc,j),
        _=>false,
    },
{
    suffix_total(b,doc,0);data::saved_at_projection(b,doc,STRICT,k);
}
pub proof fn frame_transparency(b: Seq<u8>,doc: d::Doc,k: nat,j: int)
    requires 0<=j<canvas::bytes(doc),!canvas::covered(doc,k+1,j),
    ensures record(b,doc,k).rgba[j]==0u8,
{canvas::untouched(b,doc,k+1,j);}
// Checked structural provenance and the fixed-window totality certificate.
pub open spec fn structure(b: Seq<u8>,doc: d::Doc) -> bool {
    still::checked_header(doc.ihdr) && doc.info.header==doc.ihdr
    && crate::colour_model::context_domain(crate::colour_model::from_doc(doc,b))
    && d::entries(doc)==crate::colour_model::from_doc(doc,b).plte.len()/3
    && data::count(doc)<=u32::MAX && canvas::document_geometry(doc) && canvas::streams(b,doc)
}
pub proof fn ranges_total(b: Seq<u8>,ranges: Seq<crate::walk_model::Range>)
    requires forall|i: int| 0<=i<ranges.len() ==> (#[trigger] ranges[i]).offset+ranges[i].len<=b.len(),
    ensures d::concat_ranges(b,ranges).is_some(),
    decreases ranges.len(),
{
    reveal_with_fuel(d::concat_ranges,2);
    if ranges.len()>0 {
        let rest=ranges.drop_first();
        assert forall|i: int| 0<=i<rest.len() implies (#[trigger] rest[i]).offset+rest[i].len<=b.len() by {
            assert(rest[i]==ranges[i+1]);
        }
        ranges_total(b,rest);
    }
}
pub(crate) proof fn document_context(b: Seq<u8>,doc: crate::walk::Doc)
    requires doc.wf(b),doc.source_partition(),doc.info.frame_count==doc.frame_ranges@.len(),
    ensures structure(b,doc.view()),
{
    doc.colour_context(b);
    ranges_total(b,doc.idat_ranges@);
    assert forall|k: nat| k<data::count(doc.view()) implies #[trigger] canvas::geometry(doc.view(),k) by {
        if doc.actl.is_some() {doc.selected_frame_fits(b,k);}
    }
    assert forall|k: nat| k<data::count(doc.view()) implies (#[trigger] controls::stream(b,doc.view(),k)).is_some() by {
        if doc.actl.is_some() {
            let f=doc.frame_ranges@[k as int];
            if let crate::walk_model::Source::Fdat(ranges)=f.source {ranges_total(b,ranges);}
        }
    }
}
pub proof fn normalized_structure(b: Seq<u8>,doc: d::Doc)
    requires structure(b,doc),
    ensures structure(b,data::constructor_doc(doc)),
{
    assert forall|k: nat| k<data::count(data::constructor_doc(doc)) implies
        #[trigger] canvas::geometry(data::constructor_doc(doc),k) by {assert(canvas::geometry(doc,k));}
    assert forall|k: nat| k<data::count(data::constructor_doc(doc)) implies
        (#[trigger] controls::stream(b,data::constructor_doc(doc),k)).is_some() by {
        assert(controls::stream(b,doc,k).is_some());
    }
}
#[verifier::spinoff_prover]
fn measure_data<S: inflate_verus::Segments>(src: &mut S,hdr: &crate::header::Ihdr,w: u32,h: u32,Ghost(entries): Ghost<nat>,ring: &mut [u8])
    requires old(src).wf(),old(src).pos()==0,hdr.wf(),1<=w<=hdr.w,1<=h<=hdr.h,
        old(ring).len()==inflate_verus::limits::WINDOW_MAX,
    ensures final(src).wf(),final(src).concat()==old(src).concat(),final(ring).len()==old(ring).len(),
        r::data(old(src).concat(),r::shape(hdr.view(),w as nat,h as nat,entries,STRICT),STRICT).is_some(),
{
    hide(inflate_verus::model::Inflate);hide(r::data);
    let limit=crate::image::raw_size(hdr,w,h);
    let result=inflate_verus::inflate_zlib_measure(src,ring,limit,inflate_verus::InflateOptions::STRICT);
    proof {
        let a=r::shape(hdr.view(),w as nat,h as nat,entries,STRICT);
        assert(inflate_verus::status_eq(result.status,r::inflated(old(src).concat(),a,STRICT).status));
        assert(!matches!(r::inflated(old(src).concat(),a,STRICT).status,inflate_verus::model::InflateStatus::OutsideDomain));
        reveal(r::data);
    }
}
/// Exclude both constructor and terminal internal domain markers. Keeping
/// the terminal marker in its projection avoids a constructor dependency on
/// executing any future frame; the certificate below excludes that marker.
pub open spec fn total(b: Seq<u8>) -> bool {
    match MFr(b) {Outcome::Error(_)=>true,Outcome::Done(value)=>!matches!(value.end,data::End::OutsideDomain),Outcome::OutsideDomain=>false}
}
pub open spec fn walk_context(b: Seq<u8>) -> bool {
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(original,_)=>{
            let doc=data::constructor_doc(original);
            structure(b,doc) && document_total(b,doc)
        },d::Outcome::Error(_,_)=>true,_=>false,
    }
}
/// Checked allocation-free certificate: select and measure sources, without
/// assuming successful decoding or constructing any native frame/canvas.
#[verifier::spinoff_prover]
pub fn model_totality(b: &[u8])
    ensures total(b@),walk_context(b@),match MFr(b@) {Outcome::Done(value)=>resources(value),_=>true},
{
    hide(d::MWF_p);hide(inflate_verus::model::Inflate);hide(r::document_data);
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut events=crate::png::Events {r1:false,r3:false,r4:false};
    let result=crate::callers::frames(b,&STRICT,&mut events);
    match result {
        Err(_)=>{},
        Ok(mut doc)=>{
            proof {
                crate::frame_state::source_context(b@,doc);
                document_context(b@,doc);
                normalized_structure(b@,doc.view());
            }
            crate::frame_state::constructor_info(b,&mut doc);
            proof {crate::frame_state::source_context(b@,doc);}
            let mut window=[0u8;inflate_verus::limits::WINDOW_MAX];
            proof {
                vstd::array::array_len_matches_n(&window);
                assert(inflate_verus::limits::WINDOW_MAX==32768usize) by (compute);
            }
            let ring=vstd::array::ref_mut_array_unsizing_coercion(&mut window);
            let mut source=crate::reader::IdatReader::for_doc(b,&doc);
            measure_data(&mut source,&doc.ihdr,doc.ihdr.w,doc.ihdr.h,Ghost(d::entries(doc.view())),ring);
            proof {reveal(r::document_data);assert(r::document_data(b@,doc.view(),STRICT).is_some());}
            if doc.actl.is_some() {
                let mut state=crate::frame_state::FrameState::new(b,&doc);
                let mut k=0u32;
                while k<doc.info.frame_count
                    invariant doc.wf(b@),doc.source_partition(),crate::frame_state::source_document(b@,doc),doc.actl.is_some(),
                        doc.info.frame_count==doc.frame_ranges@.len(),state.wf(b@,doc),state.index()==Some(k as nat),
                        k<=doc.info.frame_count,ring.len()==inflate_verus::limits::WINDOW_MAX,
                        structure(b@,doc.view()),r::document_data(b@,doc.view(),STRICT).is_some(),
                        forall|j: nat| j<k ==> (#[trigger] data::frame_data(b@,doc.view(),STRICT,j)).is_some(),
                        match d::MWF_p(b@,d::strict()) {d::Outcome::Done(original,_)=>doc.view()==data::constructor_doc(original),_=>false},
                    decreases doc.info.frame_count-k,
                {
                    let job=match state.begin_next(b,&doc) {crate::frame_state::Step::Frame(job)=>job,_=>{assert(false);return;}};
                    proof {job.measure_context(b@,doc);job.data_context(b@,doc,STRICT);}
                    let resume=job.resume;
                    match job.reader {
                        Some(mut src)=>{
                            measure_data(&mut src,&doc.ihdr,job.control.w,job.control.h,Ghost(d::entries(doc.view())),ring);
                            assert(data::frame_data(b@,doc.view(),STRICT,k as nat).is_some());
                        },None=>{assert(data::frame_data(b@,doc.view(),STRICT,k as nat).is_some());},
                    }
                    assert forall|j: nat| j<k as nat+1 implies (#[trigger] data::frame_data(b@,doc.view(),STRICT,j)).is_some() by {
                        if j<k {}else{assert(j==k);}
                    }
                    state=resume;k+=1;
                }
            }else{
                proof {data::frame_rule(b@,doc.view(),STRICT,0);}
            }
            proof {
                assert(document_total(b@,doc.view()));
                suffix_total(b@,doc.view(),0);complete_resources(b@,doc.view());
            }
        },
    }
}

pub open spec fn request_bounds(doc: d::Doc) -> bool {
    let raw=crate::layout_model::raw(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr),doc.ihdr.width as nat,doc.ihdr.height as nat);
    2<=raw<=2*(crate::limits::MAX_OUTPUT as nat)+crate::limits::MAX_OUTPUT as nat/2+2
    && raw<=crate::alloc_guard::MAX_BYTES
    && 1<=canvas::bytes(doc)<=crate::limits::MAX_OUTPUT
    && canvas::bytes(doc)<=crate::alloc_guard::MAX_BYTES
    && forall|k: nat| k<data::count(doc) ==> #[trigger] r::raw(controls::shape(doc,STRICT,k))<=raw
}
pub proof fn resource_bounds(b: Seq<u8>,doc: d::Doc)
    requires structure(b,doc),
    ensures request_bounds(doc),
{
    crate::still_equivalence::header_request_bounds(doc.ihdr,false);
    assert forall|k: nat| k<data::count(doc) implies #[trigger] r::raw(controls::shape(doc,STRICT,k))
        <=crate::layout_model::raw(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr),doc.ihdr.width as nat,doc.ihdr.height as nat) by {
        assert(canvas::geometry(doc,k));
        let a=controls::shape(doc,STRICT,k);
        crate::layout_model::raw_dimensions(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr),a.width,a.height,
            doc.ihdr.width as nat,doc.ihdr.height as nat);
    }
}
pub proof fn frame_source_ok(b: Seq<u8>,doc: d::Doc,k: nat,x: nat,y: nat)
    requires structure(b,doc),k<data::count(doc),data::frame_ok(b,doc,STRICT,k),
        x<controls::shape(doc,STRICT,k).width,y<controls::shape(doc,STRICT,k).height,
    ensures {
        let z=controls::stream(b,doc,k).get_Some_0();let a=controls::shape(doc,STRICT,k);
        crate::paint_model::source_pixel(r::inflated(z,a,STRICT).out,a,crate::colour_model::from_doc(doc,b),x,y,false,false).is_ok()
    },
{
    hide(inflate_verus::model::Inflate);
    data::frame_rule(b,doc,STRICT,k);
    assert(canvas::geometry(doc,k));
    let z=controls::stream(b,doc,k).get_Some_0();let a=controls::shape(doc,STRICT,k);
    let colour=crate::colour_model::from_doc(doc,b);
    assert(r::data(z,a,STRICT).is_some());
    assert(r::scan(r::inflated(z,a,STRICT).out,a,0).is_ok());
    still::source_pixel_ok(r::inflated(z,a,STRICT).out,a,colour,x,y);
}
pub proof fn terminal_saved(b: Seq<u8>,doc: d::Doc,k: nat,e: Error)
    requires document_total(b,doc),data::data_ok(r::document_data(b,doc,STRICT)),data::prefix(b,doc,STRICT,k),
        data::qualifies(doc,k),data::frame_data(b,doc,STRICT,k)==Some(Err(e)),
        forall|j: nat| 1<=j<k ==> !#[trigger] data::qualifies(doc,j),
    ensures match from_document(b,doc) {
        Outcome::Done(value)=>value.frames.len()==k && value.end==data::End::Format(e)
            && data::SavedAt(b,doc,STRICT)==Some(k),
        _=>false,
    },
{
    summary_prefix(b,doc,0);data::summary_reached(b,doc,STRICT,0,k);
    let n=data::MFr_data(b,doc,STRICT).frames;
    if k<n {assert(data::frame_ok(b,doc,STRICT,k));assert(false);}
    assert(k==n);
    assert(data::is_saved_at(b,doc,STRICT,k));data::saved_at_unique(b,doc,STRICT,k,k);
}
/// Unconditional checked model package, without a public T-ITER claim.
pub fn model_theorems(b: &[u8])
    ensures total(b@),walk_context(b@),metadata(b@),returned_sources(b@),
        match MFr(b@) {Outcome::Done(value)=>resources(value),_=>true},
        match d::MWF_p(b@,d::strict()) {d::Outcome::Done(original,_)=>request_bounds(data::constructor_doc(original)),_=>true},
{
    model_totality(b);
    proof {
        metadata_properties(b@);returned_source_properties(b@);
        match d::MWF_p(b@,d::strict()) {
            d::Outcome::Done(original,_)=>{resource_bounds(b@,data::constructor_doc(original));},_=>{},
        }
    }
}

pub proof fn successful_prefix_bound(b: Seq<u8>,doc: d::Doc,n: nat)
    requires document_total(b,doc),data::prefix(b,doc,STRICT,n),
    ensures n<=data::MFr_data(b,doc,STRICT).frames,
{
    summary_prefix(b,doc,0);data::summary_bounds(b,doc,STRICT,0);
    let limit=data::MFr_data(b,doc,STRICT).frames;
    if limit<n {
        assert(data::frame_ok(b,doc,STRICT,limit));
        match data::MFr_data(b,doc,STRICT).end {
            data::End::Iend=>{assert(false);},
            data::End::Format(e)=>{assert(data::frame_data(b,doc,STRICT,limit)==Some(Err(e)));assert(false);},
            _=>{assert(false);},
        }
    }
}
/// Compose independent data-prefix success and the accepted M5 pixel trace.
pub proof fn history_result_prefix(b: Seq<u8>,doc: d::Doc,t: controls::Trace)
    requires structure(b,doc),document_total(b,doc),data::data_ok(r::document_data(b,doc,STRICT)),
        controls::wf(b,doc,STRICT,t),data::prefix(b,doc,STRICT,t.after.len()),
    ensures match from_document(b,doc) {
        Outcome::Done(value)=>t.after.len()<=value.frames.len()
            && forall|k: int| 0<=k<t.after.len() ==> (#[trigger] t.after[k])==value.frames[k].rgba,
        _=>false,
    },
{
    canvas::history_complete(b,doc,t);successful_prefix_bound(b,doc,t.after.len());suffix_total(b,doc,0);
}

pub open spec fn metadata(b: Seq<u8>) -> bool {
    match MFr(b) {Outcome::Done(value)=>match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(original,_)=>value.info==d::constructor_info(original)
            && value.num_plays==match original.actl {Some((_,plays))=>Some(plays),None=>None}
            && forall|k: int| #![trigger value.frames[k]] 0<=k<value.frames.len() ==>{
                let frame=value.frames[k];
                frame.rect==controls::rectangle(original,k as nat)
                && frame.delay==(if original.actl.is_none() {None}else{Some((original.frames[k].delay_num,original.frames[k].delay_den))})
                && frame.dispose==(if original.actl.is_none() {0}else{original.frames[k].dispose})
                && frame.blend==(if original.actl.is_none() {0}else{original.frames[k].blend})
            },_=>false},_=>true}
}
pub proof fn metadata_properties(b: Seq<u8>)
    ensures metadata(b),
{hide(d::MWF_p);}
pub open spec fn covered_pixel(doc: d::Doc,n: nat,x: nat,y: nat) -> bool {
    exists|k: nat| #![trigger controls::rectangle(doc,k)] k<n && {
        let rect=controls::rectangle(doc,k);rect.x<=x<rect.x+rect.w && rect.y<=y<rect.y+rect.h
    }
}
pub proof fn untouched_pixel(b: Seq<u8>,doc: d::Doc,k: nat,x: nat,y: nat)
    requires x<doc.ihdr.width,y<doc.ihdr.height,!covered_pixel(doc,k+1,x,y),
    ensures forall|channel: int| 0<=channel<4 ==> (#[trigger] record(b,doc,k).rgba[4*(y*(doc.ihdr.width as nat)+x)+channel])==0u8,
{
    let width=doc.ihdr.width as nat;let height=doc.ihdr.height as nat;
    assert forall|channel: int| 0<=channel<4 implies (#[trigger] record(b,doc,k).rgba[4*(y*width+x)+channel])==0u8 by {
        let j=4*(y*width+x)+channel;
        comp::rectangle_access(width,comp::Rect {x:0,y:0,w:width,h:height},y,4*x+channel as nat);
        assert(0<=j<canvas::bytes(doc)) by (nonlinear_arith)
            requires x<width,y<height,0<=channel<4,j==4*(y*width+x)+channel,canvas::bytes(doc)==4*width*height;
        if canvas::covered(doc,k+1,j) {
            let index=choose|index: nat| index<k+1 && comp::inside(width,controls::rectangle(doc,index),j);
            let rect=controls::rectangle(doc,index);
            assert(rect.x<=x<rect.x+rect.w && rect.y<=y<rect.y+rect.h);
            assert(covered_pixel(doc,k+1,x,y));assert(false);
        }
        frame_transparency(b,doc,k,j);
    }
}

/// The frozen global SavedAt is projected onto the complete MFr prefix.
pub proof fn saved_at_global(b: Seq<u8>,value: Value,k: nat)
    requires walk_context(b),MFr(b)==Outcome::Done(value),
    ensures match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(original,_)=>{
            let doc=data::constructor_doc(original);
            (data::SavedAt_p(b,STRICT)==Some(k))==(data::qualifies(doc,k) && k<=value.frames.len()
                && forall|j: nat| 1<=j<k ==> !#[trigger] data::qualifies(doc,j))
            && ((data::SavedAt_p(b,STRICT)==Some(k) && k==value.frames.len()) ==> matches!(value.end,data::End::Format(_)))
        },_=>false,
    },
{
    hide(d::MWF_p);crate::walk_policy::profiles_project_exactly();
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(original,_)=>{
            let doc=data::constructor_doc(original);
            data::from_walk_rule(b,doc,STRICT);data::default_intro(b,doc,STRICT);
            data::canonical_saved_at(b,doc,STRICT);
            summary_prefix(b,doc,0);data::summary_bounds(b,doc,STRICT,0);
            if k<data::count(doc) {saved_at_complete(b,doc,k);}
            else {assert(!data::qualifies(doc,k));assert(!data::is_saved_at(b,doc,STRICT,k));}
            if data::is_saved_at(b,doc,STRICT,k) {data::saved_at_unique(b,doc,STRICT,k,k);}
            if data::SavedAt(b,doc,STRICT)==Some(k) {assert(data::is_saved_at(b,doc,STRICT,k));}
            if data::SavedAt_p(b,STRICT)==Some(k) && k==value.frames.len() {
                assert(data::qualifies(doc,k));assert(k<data::count(doc));
                assert(!matches!(value.end,data::End::Iend));
            }
        },_=>{},
    }
}

pub open spec fn pixel_ready(b: Seq<u8>,doc: d::Doc,k: nat,x: nat,y: nat) -> bool {
    let z=controls::stream(b,doc,k).get_Some_0();let a=controls::shape(doc,STRICT,k);
    crate::paint_model::source_pixel(r::inflated(z,a,STRICT).out,a,crate::colour_model::from_doc(doc,b),x,y,false,false).is_ok()
}
pub open spec fn returned_sources(b: Seq<u8>) -> bool {
    match MFr(b) {Outcome::Done(value)=>match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(original,_)=>{
            let doc=data::constructor_doc(original);
            forall|k: nat,x: nat,y: nat| k<value.frames.len() && x<controls::shape(doc,STRICT,k).width
                && y<controls::shape(doc,STRICT,k).height ==> #[trigger] pixel_ready(b,doc,k,x,y)
        },_=>false},_=>true}
}
pub proof fn returned_source_properties(b: Seq<u8>)
    requires walk_context(b),
    ensures returned_sources(b),
{
    hide(d::MWF_p);hide(inflate_verus::model::Inflate);
    match MFr(b) {
        Outcome::Done(value)=>match d::MWF_p(b,d::strict()) {
            d::Outcome::Done(original,_)=>{
                let doc=data::constructor_doc(original);
                summary_prefix(b,doc,0);data::summary_bounds(b,doc,STRICT,0);
                assert forall|k: nat,x: nat,y: nat| k<value.frames.len() && x<controls::shape(doc,STRICT,k).width
                    && y<controls::shape(doc,STRICT,k).height implies #[trigger] pixel_ready(b,doc,k,x,y) by {
                    assert(data::frame_ok(b,doc,STRICT,k));
                    frame_source_ok(b,doc,k,x,y);
                }
            },_=>{},
        },_=>{},
    }
}

}
