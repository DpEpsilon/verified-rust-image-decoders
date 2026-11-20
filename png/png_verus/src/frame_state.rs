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

impl FrameState {
    pub(crate) open spec fn index(self) -> Option<nat> {
        match self.kind {Kind::Anim {index,..}=>Some(index as nat),_=>None}
    }
    pub(crate) open spec fn done(self) -> bool {matches!(self.kind,Kind::Done)}
    pub(crate) open spec fn single(self) -> bool {matches!(self.kind,Kind::Single)}
    pub(crate) open spec fn wf(self, b: Seq<u8>, doc: Doc) -> bool {
        source_document(b,doc) && match self.kind {
            Kind::Done=>true,Kind::Single=>doc.actl.is_none(),
            Kind::Anim {index,cursor,..}=>doc.actl.is_some() && index<=doc.info.frame_count
                && cursor==transport::cursor_for(b,doc.frame_ranges@,transport::header_end(b),index as nat),
        }
    }
    pub(crate) fn new(b: &[u8], doc: &Doc) -> (r: Self)
        requires source_document(b@,*doc),
        ensures r.wf(b@,*doc),r.single()==doc.actl.is_none(),
            doc.actl.is_some() ==> r.index()==Some(0nat),!r.done(),
    {
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
                Step::Frame(job)=>job.wf(b@,*doc) && old(self).index()==Some(job.index as nat),
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
            cursor==transport::cursor_for(b@,doc.frame_ranges@,transport::header_end(b@),index as nat),
        ensures r.wf(b@,*doc),r.index==index,
    {
        hide(m::frame_stream);hide(transport::control_record);hide(transport::frame_start);hide(model::mh);
        hide(Doc::wf);hide(Doc::source_partition);hide(transport::result);
        proof {reveal(source_document);}
        let (control,start)=select_control(b,doc,index,cursor);
        let reader=if index==0 && doc.info.default_is_frame {None} else {
            Some(DataReader::for_frame(b,doc,index,start))
        };
        let effective_dispose=if index==0 && control.dispose==2 {1} else {control.dispose};
        let resume=resume_state(b,doc,index,start,control,effective_dispose);
        proof {reveal(FrameJob::wf);}
        Self {index,control,previous,effective_dispose,reader,resume}
    }

    #[verifier::opaque]
    pub(crate) open spec fn wf(self, b: Seq<u8>, doc: Doc) -> bool {
        self.index<doc.frame_ranges@.len() && self.resume.wf(b,doc)
        && self.resume.index()==Some(self.index as nat+1)
        && transport::control_record(doc.frame_ranges@[self.index as int],self.control.view())
        && self.effective_dispose==(if self.index==0 && self.control.dispose==2 {1}else{self.control.dispose})
        && match self.reader {
            None=>self.index==0 && doc.info.default_is_frame,
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
    ensures r.wf(b@,*doc),r.index()==Some(index as nat+1),
{
    FrameState {kind:Kind::Anim {index:index+1,cursor:start,
        previous:Some(((control.x,control.y,control.w,control.h),dispose))}}
}

} // verus!
