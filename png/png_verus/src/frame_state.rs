//! Checked source-state transitions used by the public frame iterator.
//! Pixel decoding, disposal and allocation remain in the public caller.
use vstd::prelude::*;
use crate::{walk::Doc,apng::Fctl,reader::{DataReader,next_selected_fctl},framing::frame_at};
#[cfg(verus_only)]
use crate::{model,document_model as m,frame_transport_model as transport};
#[cfg(verus_only)]
use inflate_verus::Segments;

verus! {

pub(crate) type Previous = Option<((u32,u32,u32,u32),u8)>;
pub(crate) open spec fn previous_needs_saved(previous: Previous) -> bool {
    match previous {Some((_,op))=>op==2,None=>false}
}
pub(crate) open spec fn previous_fits(previous: Previous,header: crate::model::Ihdr) -> bool {
    match previous {
        None=>true,
        Some(((x,y,w,h),dispose))=>1<=w && 1<=h && x as int+w as int<=header.width
            && y as int+h as int<=header.height && dispose<=2,
    }
}

#[derive(Clone,Copy)]
pub(crate) enum Kind {
    Single,
    Anim { index: u32, cursor: usize, previous: Previous },
    Done,
}

#[derive(Clone,Copy)]
pub(crate) struct FrameState { pub(crate) kind: Kind }

pub(crate) struct FrameJob<'a> {
    pub index: u32,
    pub control: Fctl,
    pub previous: Previous,
    pub effective_dispose: u8,
    pub reader: Option<DataReader<'a>>,
    pub resume: FrameState,
}

pub(crate) enum Step<'a> { Done, Single, Frame(FrameJob<'a>) }

#[verifier::opaque]
pub(crate) open spec fn source_document(b: Seq<u8>, doc: Doc) -> bool {
    doc.wf(b) && doc.source_partition() && b.len()<=crate::limits::MAX_INPUT
    && doc.info.frame_count==doc.frame_ranges@.len() && model::mh(b).is_ok()
    && (doc.actl.is_some() ==> transport::result(b,doc.view(),transport::header_end(b)))
}

/// Carry a concrete num_plays-only update through the source predicates.
pub(crate) proof fn plays_preserve_source(b: Seq<u8>, before: Doc, after: Doc, plays: Option<u32>)
    requires source_document(b,before),after==(Doc {info:before.info.with_plays(plays),..before}),
    ensures source_document(b,after),after.view()==(m::Doc {info:m::Info {num_plays:plays,..before.view().info},..before.view()}),
{
    hide(crate::png::Info::view);
    hide(transport::result);
    reveal(source_document);
    crate::png::Info::plays_view(before.info,plays);
    transport::result_same_frames(b,before.view(),after.view(),transport::header_end(b));
}

pub(crate) fn constructor_info(b: &[u8], doc: &mut Doc)
    requires source_document(b@,*old(doc)),
    ensures source_document(b@,*final(doc)),
        final(doc).info.view()==m::constructor_info(old(doc).view()),
        *final(doc)==(Doc {info:old(doc).info.with_plays(match old(doc).actl {Some(a)=>Some(a.num_plays),None=>None}),..*old(doc)}),
{
    let ghost before=*doc;
    let plays=match doc.actl {Some(a)=>Some(a.num_plays),None=>None};
    doc.info.set_num_plays(plays);
    proof {plays_preserve_source(b@,before,*doc,plays);}
}

impl FrameState {
    pub(crate) open spec fn index(self) -> Option<nat> {
        match self.kind {Kind::Anim {index,..}=>Some(index as nat),_=>None}
    }
    pub(crate) open spec fn previous(self) -> Previous {
        match self.kind {Kind::Anim {previous,..}=>previous,_=>None}
    }
    pub(crate) open spec fn done(self) -> bool {matches!(self.kind,Kind::Done)}
    pub(crate) open spec fn single(self) -> bool {matches!(self.kind,Kind::Single)}
    pub(crate) open spec fn wf(self, b: Seq<u8>, doc: Doc) -> bool {
        source_document(b,doc) && match self.kind {
            Kind::Done=>true,Kind::Single=>doc.actl.is_none(),
            Kind::Anim {index,cursor,previous}=>doc.actl.is_some() && index<=doc.info.frame_count
                && previous_fits(previous,doc.ihdr.view())
                && cursor==transport::cursor_for(b,doc.frame_ranges@,transport::header_end(b),index as nat),
        }
    }
    pub(crate) fn new(b: &[u8], doc: &Doc) -> (r: Self)
        requires source_document(b@,*doc),
        ensures r.wf(b@,*doc),r.single()==doc.actl.is_none(),
            doc.actl.is_some() ==> r.index()==Some(0nat),!r.done(),r.previous()==None,
    {
        hide(Doc::wf);hide(Doc::source_partition);hide(transport::result);
        proof {reveal(source_document);}
        if doc.actl.is_none() {Self {kind:Kind::Single}} else {
            let c=match frame_at(b,8) {Ok(c)=>c,Err(_)=>{assert(false);return Self {kind:Kind::Done};}};
            Self {kind:Kind::Anim {index:0,cursor:c.next,previous:None}}
        }
    }
    /// Select before any pixel work and fuse the receiver immediately. Only
    /// the returned resume state can continue after successful image work.
    #[verifier::spinoff_prover]
    pub(crate) fn begin_next<'a>(&mut self, b: &'a [u8], doc: &Doc) -> (r: Step<'a>)
        requires old(self).wf(b@,*doc),
        ensures final(self).wf(b@,*doc),final(self).done(),
            match r {
                Step::Done=>old(self).done() || (old(self).index()==Some(doc.info.frame_count as nat)),
                Step::Single=>old(self).single(),
                Step::Frame(job)=>job.wf(b@,*doc) && old(self).index()==Some(job.index as nat)
                    && job.previous==old(self).previous(),
            },
            old(self).done() ==> matches!(r,Step::Done),
            old(self).single() ==> matches!(r,Step::Single),
            match old(self).index() {
                Some(k)=>if k<doc.info.frame_count {matches!(r,Step::Frame(_))}else{matches!(r,Step::Done)},
                None=>true,
            },
    {
        let prior=self.kind;
        self.kind=Kind::Done;
        match prior {
            Kind::Done=>Step::Done,
            Kind::Single=>Step::Single,
            Kind::Anim {index,cursor,previous}=> {
                if index>=doc.info.frame_count {return Step::Done;}
                Step::Frame(FrameJob::select(b,doc,index,cursor,previous))
            },
        }
    }
}

impl<'a> FrameJob<'a> {
    #[verifier::spinoff_prover]
    fn select(b: &'a [u8], doc: &Doc, index: u32, cursor: usize, previous: Previous) -> (r: Self)
        requires source_document(b@,*doc),doc.actl.is_some(),index<doc.info.frame_count,
            previous_fits(previous,doc.ihdr.view()),
            cursor==transport::cursor_for(b@,doc.frame_ranges@,transport::header_end(b@),index as nat),
        ensures r.wf(b@,*doc),r.index==index,r.previous==previous,
    {
        hide(m::frame_stream);hide(transport::control_record);hide(transport::frame_start);hide(model::mh);
        hide(Doc::wf);hide(Doc::source_partition);hide(transport::result);
        proof {reveal(source_document);}
        let (control,start)=select_control(b,doc,index,cursor);
        proof {
            doc.selected_frame_fits(b@,index as nat);
            if index==0 && doc.info.default_is_frame {doc.selected_reuse_geometry(b@);}
        }

        let reader=if index==0 && doc.info.default_is_frame {None} else {
            Some(DataReader::for_frame(b,doc,index,start))
        };
        proof {reveal(transport::control_record);}
        let effective_dispose=if index==0 && control.dispose==2 {1} else {control.dispose};
        let resume=resume_state(b,doc,index,start,control,effective_dispose);
        proof {reveal(FrameJob::wf);reveal(transport::control_record);}
        Self {index,control,previous,effective_dispose,reader,resume}
    }

    pub(crate) proof fn reused_source(self,b: Seq<u8>,doc: Doc)
        requires self.wf(b,doc),self.reader.is_none(),
        ensures m::frame_stream(b,doc.view(),self.index as nat)==m::default_stream(b,doc.view()),
    {
        reveal(FrameJob::wf);reveal(source_document);
        doc.selected_reuse_geometry(b);
        m::reused_stream(b,doc.view());
    }

    /// Relate the selected source and dimensions to the independent data model.
    pub(crate) proof fn data_context(self,b: Seq<u8>,doc: Doc,p: crate::policy::Policy)
        requires self.wf(b,doc),
        ensures doc.actl.is_some(),self.index<crate::iterator_data_model::count(doc.view()),
            crate::iterator_data_model::frame_data(b,doc.view(),p,self.index as nat)==match m::frame_stream(b,doc.view(),self.index as nat) {
                Some(z)=>crate::raster_model::data(z,crate::image::raster_context(doc,self.control.w,self.control.h,p),p),None=>None},
            self.reader.is_none() ==> crate::iterator_data_model::frame_data(b,doc.view(),p,self.index as nat)==crate::raster_model::document_data(b,doc.view(),p),
    {
        hide(crate::raster_model::data);
        reveal(FrameJob::wf);
        crate::iterator_data_model::frame_rule(b,doc.view(),p,self.index as nat);
        if self.reader.is_none() {self.reused_source(b,doc);}
    }

    pub(crate) proof fn reuse_data_context(self,b: Seq<u8>,doc: Doc,p: crate::policy::Policy)
        requires self.wf(b,doc),crate::iterator_data_model::default_success(b,doc.view(),p),
        ensures doc.actl.is_some(),self.index<crate::iterator_data_model::count(doc.view()),
            self.reader.is_none() ==> crate::iterator_data_model::frame_ok(b,doc.view(),p,self.index as nat),
    {
        self.data_context(b,doc,p);crate::iterator_data_model::default_context(b,doc.view(),p);
    }

    /// Export disposal/storage metadata without exposing frame-stream models.
    pub(crate) proof fn storage_context(self,b: Seq<u8>,doc: Doc)
        requires self.wf(b,doc),
        ensures self.index<doc.frame_ranges@.len(),self.resume.wf(b,doc),
            self.resume.index()==Some(self.index as nat+1),
            self.resume.previous()==Some(((self.control.x,self.control.y,self.control.w,self.control.h),self.effective_dispose)),
            previous_fits(self.previous,doc.ihdr.view()),
            1<=self.control.w,1<=self.control.h,
            self.control.x as int+self.control.w as int<=doc.ihdr.w,
            self.control.y as int+self.control.h as int<=doc.ihdr.h,
            self.effective_dispose<=2,
            self.effective_dispose==(if self.index==0 && self.control.dispose==2 {1}else{self.control.dispose}),
            doc.frame_ranges@[self.index as int].dispose==self.control.dispose,
            self.reader.is_none() ==> doc.info.default_is_frame && self.index==0 && self.control.x==0 && self.control.y==0
                && self.control.w==doc.ihdr.w && self.control.h==doc.ihdr.h,
    {
        reveal(FrameJob::wf);
        transport::control_dispose(doc.frame_ranges@[self.index as int],self.control.view());
    }

    #[verifier::opaque]
    pub(crate) open spec fn wf(self, b: Seq<u8>, doc: Doc) -> bool {
        doc.actl.is_some() && self.index<doc.frame_ranges@.len() && self.resume.wf(b,doc)
        && previous_fits(self.previous,doc.ihdr.view())
        && self.resume.index()==Some(self.index as nat+1)
        && self.resume.previous()==Some(((self.control.x,self.control.y,self.control.w,self.control.h),self.effective_dispose))
        && self.effective_dispose<=2
        && transport::control_record(doc.frame_ranges@[self.index as int],self.control.view())
        && crate::walk_state::frame_fits(doc.frame_ranges@[self.index as int],doc.ihdr.view())
        && 1<=self.control.w<=doc.ihdr.w && 1<=self.control.h<=doc.ihdr.h
        && self.control.x as int+self.control.w as int<=doc.ihdr.w
        && self.control.y as int+self.control.h as int<=doc.ihdr.h
        && self.effective_dispose==(if self.index==0 && self.control.dispose==2 {1}else{self.control.dispose})
        && match self.reader {
            None=>self.index==0 && doc.info.default_is_frame
                && self.control.x==0 && self.control.y==0 && self.control.w==doc.ihdr.w && self.control.h==doc.ihdr.h,
            Some(reader)=>!(self.index==0 && doc.info.default_is_frame)
                && reader.wf() && reader.pos()==0 && reader.seg_steps()==0 && reader.core_steps()==0
                && m::frame_stream(b,doc.view(),self.index as nat)==Some(reader.concat()),
        }
    }
}

/// Parse only after the checked selector supplies the validated wire site.
#[verifier::spinoff_prover]
fn select_control(b: &[u8], doc: &Doc, index: u32, cursor: usize) -> (r: (Fctl,usize))
    requires source_document(b@,*doc),doc.actl.is_some(),index<doc.info.frame_count,
        cursor==transport::cursor_for(b@,doc.frame_ranges@,transport::header_end(b@),index as nat),
    ensures transport::control_record(doc.frame_ranges@[index as int],r.0.view()),
        r.1==transport::frame_start(b@,doc.frame_ranges@[index as int]),
{
    proof {reveal(source_document);}
    let fpos=match next_selected_fctl(b,doc,index,cursor) {
        Some(p)=>p,None=>{assert(false);return (Fctl {w:0,h:0,x:0,y:0,delay_num:0,delay_den:0,dispose:0,blend:0},0);},
    };
    read_control(b,fpos)
}

fn read_control(b: &[u8], offset: usize) -> (r: (Fctl,usize))
    requires match model::frame(b@,offset as nat) {Ok(c)=>c.len==26,Err(_)=>false},
    ensures match model::frame(b@,offset as nat) {
        Ok(c)=>r.0.view()==crate::apng_model::parse_fctl(b@.subrange(c.data as int,(c.data+c.len) as int))
            && r.1==c.next,Err(_)=>false,
    },
{
    let c=match frame_at(b,offset) {Ok(c)=>c,Err(_)=>{assert(false);return (Fctl {w:0,h:0,x:0,y:0,delay_num:0,delay_den:0,dispose:0,blend:0},0);}};
    let control=Fctl::parse(&b[c.data..c.data+26]);
    (control,c.next)
}

fn resume_state(b: &[u8], doc: &Doc, index: u32, start: usize, control: Fctl, dispose: u8) -> (r: FrameState)
    requires source_document(b@,*doc),doc.actl.is_some(),index<doc.info.frame_count,
        start==transport::frame_start(b@,doc.frame_ranges@[index as int]),
        1<=control.w,1<=control.h,control.x as int+control.w as int<=doc.ihdr.w,
        control.y as int+control.h as int<=doc.ihdr.h,dispose<=2,
    ensures r.wf(b@,*doc),r.index()==Some(index as nat+1),
        r.previous()==Some(((control.x,control.y,control.w,control.h),dispose)),
{
    FrameState {kind:Kind::Anim {index:index+1,cursor:start,
        previous:Some(((control.x,control.y,control.w,control.h),dispose))}}
}

} // verus!
