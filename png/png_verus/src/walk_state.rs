//! Checked walk-state representation: executable fields plus erased payload
//! and frame traces. No runtime chunk/frame list is introduced.
use vstd::prelude::*;
use crate::{walk::{Doc,Actl,State}, header::Ihdr, png::{Info,Events}, apng::Control};
#[cfg(verus_only)]
use crate::{document_model as m, walk_model::{self as names,Range,Source}, ordering::phase_view};

verus! {

pub(crate) open spec fn optional_range(r: Option<(usize,usize)>) -> Option<Range> {
    match r {None=>None,Some((p,n))=>Some(Range {offset:p as nat,len:n as nat})}
}
pub(crate) open spec fn bounded(r: Option<(usize,usize)>, len: nat) -> bool {
    match r {None=>true,Some((p,n))=>p+n<=len}
}
pub(crate) open spec fn ranges_valid(ranges: Seq<Range>, len: nat) -> bool {
    forall|i: int| 0<=i<ranges.len() ==> (#[trigger] ranges[i]).offset+ranges[i].len<=len
}
pub(crate) open spec fn frames_valid(frames: Seq<names::Frame>, len: nat) -> bool {
    forall|i: int| 0<=i<frames.len() ==> match (#[trigger] frames[i]).source {
        Source::Idat=>true,Source::Fdat(ranges)=>ranges_valid(ranges,len),
    }
}
pub(crate) open spec fn frame_fits(f: names::Frame,h: crate::model::Ihdr) -> bool {
    1<=f.width && 1<=f.height && f.x as int+f.width as int<=h.width
    && f.y as int+f.height as int<=h.height && f.dispose<=2 && f.blend<=1
    && (matches!(f.source,Source::Idat) ==> f.x==0 && f.y==0 && f.width==h.width && f.height==h.height)
}
pub(crate) open spec fn frames_fit(frames: Seq<names::Frame>,h: crate::model::Ihdr) -> bool {
    forall|i: int| 0<=i<frames.len() ==> frame_fits(#[trigger] frames[i],h)
}
pub(crate) proof fn fit_push(frames: Seq<names::Frame>,f: names::Frame,h: crate::model::Ihdr)
    requires frames_fit(frames,h),frame_fits(f,h),
    ensures frames_fit(frames.push(f),h),
{}
pub(crate) proof fn fit_update(frames: Seq<names::Frame>,f: names::Frame,h: crate::model::Ihdr)
    requires frames.len()>0,frames_fit(frames,h),frame_fits(f,h),
    ensures frames_fit(frames.update(frames.len()-1,f),h),
{}
impl Actl {
    pub(crate) open spec fn view(self) -> (u32,u32) {(self.num_frames,self.num_plays)}
}
impl Doc {
    pub(crate) open spec fn view(self) -> m::Doc {
        m::Doc {ihdr:self.ihdr.view(),plte:optional_range(self.plte),trns:optional_range(self.trns),
            iccp:optional_range(self.iccp),actl:match self.actl {None=>None,Some(a)=>Some(a.view())},
            info:self.info.view(),idat:self.idat_ranges@,frames:self.frame_ranges@}
    }
    pub(crate) open spec fn source_partition(self) -> bool {
        let frames=self.frame_ranges@;
        (self.info.default_is_frame ==> frames.len()>0)
        && forall|i: int| 0<=i<frames.len() ==> ((match (#[trigger] frames[i]).source {Source::Idat=>true,_=>false})
            == (i==0 && self.info.default_is_frame))
    }
    pub(crate) open spec fn wf(self, b: Seq<u8>) -> bool {
        self.ihdr.wf() && self.ihdr.w<=crate::limits::PNG_WIRE_MAX && self.ihdr.h<=crate::limits::PNG_WIRE_MAX
        && self.info.view().header==self.ihdr.view()
        && self.info.palette_entries==m::entries(self.view()) && m::entries(self.view())<=256
        && bounded(self.plte,b.len()) && bounded(self.trns,b.len()) && bounded(self.iccp,b.len())
        && self.info.ranges_wf(b.len()) && self.info.repr_wf()
        && ranges_valid(self.idat_ranges@,b.len()) && frames_valid(self.frame_ranges@,b.len())
        && frames_fit(self.frame_ranges@,self.ihdr.view())
        && if self.idat_ranges@.len()==0 {self.idat_first==0} else {
            8<=self.idat_ranges@[0].offset && self.idat_first+8==self.idat_ranges@[0].offset
        }
    }
    pub(crate) proof fn selected_frame_fits(self,b: Seq<u8>,index: nat)
        requires self.wf(b),index<self.frame_ranges@.len(),
        ensures frame_fits(self.frame_ranges@[index as int],self.ihdr.view()),
    {}
    /// The executed before-IDAT fcTL guard established full-canvas geometry.
    pub(crate) proof fn selected_reuse_geometry(self,b: Seq<u8>)
        requires self.wf(b),self.source_partition(),self.info.default_is_frame,
        ensures self.frame_ranges@.len()>0,self.frame_ranges@[0].x==0,self.frame_ranges@[0].y==0,
            self.frame_ranges@[0].width==self.ihdr.w,self.frame_ranges@[0].height==self.ihdr.h,
    {
        self.selected_frame_fits(b,0);
    }
    pub(crate) fn new(ihdr: Ihdr, b: &[u8]) -> (r: Self)
        requires ihdr.wf(),
        ensures r.wf(b@),r.view()==m::initial(ihdr.view(),0,m::no_events()).doc,
    {
        proof {crate::limits::caps_ok();}
        let r=Self {ihdr,plte:None,trns:None,iccp:None,idat_first:0,actl:None,info:Info::new(&ihdr),
            #[cfg(verus_only)] idat_ranges:Ghost(Seq::empty()),
            #[cfg(verus_only)] frame_ranges:Ghost(Seq::empty())};
        r
    }
}
impl State {
    pub(crate) open spec fn view(self, doc: Doc, pos: usize, events: Events) -> m::State {
        m::State {pos:pos as nat,seen:crate::kind::seen_view(self.seen),phase:phase_view(self.idat),
            control:self.apng.view(),doc:doc.view(),events:events.view()}
    }
    pub(crate) open spec fn source_roles(self, doc: Doc) -> bool {
        let frames=doc.frame_ranges@;
        self.apng.cur.is_some()==(frames.len()>0)
        && (self.apng.fctl_before_idat ==> frames.len()>0)
        && (self.idat==0 && frames.len()>0 ==> self.apng.fctl_before_idat)
        && doc.info.default_is_frame==(self.idat!=0 && self.apng.fctl_before_idat)
        && (forall|i: int| 0<=i<frames.len() ==> ((match (#[trigger] frames[i]).source {Source::Idat=>true,_=>false})
            == (i==0 && self.apng.fctl_before_idat)))
    }
    pub(crate) open spec fn wf(self, doc: Doc, b: Seq<u8>) -> bool {
        doc.wf(b) && self.idat<=2 && self.apng.wf() && self.source_roles(doc)
        && (self.idat==0)==(doc.idat_ranges@.len()==0)
        && self.apng.frames==doc.frame_ranges@.len()
        && (doc.actl.is_none() ==> self.apng.frames==0)
        && match self.apng.view().current {
            crate::apng_model::Current::Frame(_)=>doc.frame_ranges@.len()>0 && match doc.frame_ranges@.last().source {
                Source::Idat=>self.idat==0,Source::Fdat(_)=>self.idat!=0,
            },_=>true,
        }
    }
    pub(crate) fn new(doc: &Doc, b: &[u8]) -> (r: Self)
        requires doc.wf(b@),doc.view()==m::initial(doc.ihdr.view(),0,m::no_events()).doc,
        ensures r.wf(*doc,b@),forall|p: usize,ev: Events| r.view(*doc,p,ev)==m::initial(doc.ihdr.view(),p as nat,ev.view()),
    {
        Self {seen:crate::kind::initial_seen(),idat:0,apng:Control::new()}
    }
}

pub(crate) fn discard(doc: &mut Doc, kind: crate::kind::Kind, ev: &mut Events)
    ensures final(doc).view()==(m::Doc {info:m::Info {slots:if m::metadata_names().contains(kind.view()) {
            old(doc).view().info.slots.insert(kind.view(),m::discarded(old(doc).view().info.slots[kind.view()]))
        }else{old(doc).view().info.slots},..old(doc).view().info},..old(doc).view()}),
        final(ev).view()==(m::Events {r1:old(ev).r1 || (kind.view()==names::Name::Trns &&
            (old(doc).ihdr.ct==0 || old(doc).ihdr.ct==2 || old(doc).ihdr.ct==3)),..old(ev).view()}),
        forall|b: Seq<u8>| old(doc).wf(b) ==> final(doc).wf(b),
        final(doc).idat_first==old(doc).idat_first,
{
    ev.discard_trns(kind,doc.ihdr.ct);
    crate::info::discard(&mut doc.info,kind);
}

pub(crate) fn ancillary_values(kind: crate::kind::Kind, b: &[u8], base: usize, size: usize, doc: &mut Doc)
    -> (r: Result<(),crate::png::Error>)
    requires old(doc).wf(b@),base<=b.len(),size<=b.len()-base,
    ensures final(doc).ihdr==old(doc).ihdr,final(doc).idat_first==old(doc).idat_first,
        match r {
            Err(e)=>*final(doc)==*old(doc) && crate::metadata_model::parse(kind.view(),
                b@.subrange(base as int,base as int+size as int),old(doc).ihdr.ct,old(doc).ihdr.depth,
                m::entries(old(doc).view()))==crate::metadata_model::Outcome::Error(e),
            Ok(())=>final(doc).wf(b@) && exists|value: crate::metadata_model::Value|
                crate::metadata_model::parse(kind.view(),b@.subrange(base as int,base as int+size as int),
                    old(doc).ihdr.ct,old(doc).ihdr.depth,m::entries(old(doc).view()))==crate::metadata_model::Outcome::Value(value)
                && final(doc).view()==m::store(old(doc).view(),value,base as nat),
        },
{
    let ghost original=*doc;
    let ghost parsed=crate::metadata_model::parse(kind.view(),b@.subrange(base as int,base as int+size as int),
        doc.ihdr.ct,doc.ihdr.depth,m::entries(doc.view()));
    let entries=doc.palette_entries();
    let profile=match crate::info::parse_and_store(&mut doc.info,kind,b,base,size,entries) {
        Err(e)=>return Err(e),Ok(r)=>r,
    };
    if let Some(profile)=profile {doc.iccp=Some(profile);}
    proof {
        match parsed {
            crate::metadata_model::Outcome::Value(value)=> {
                assert(doc.view()==m::store(original.view(),value,base as nat));
            },_=>assert(false),
        }
    }
    Ok(())
}

pub(crate) proof fn range_push(ranges: Seq<Range>, range: Range, len: nat)
    requires ranges_valid(ranges,len),range.offset+range.len<=len,
    ensures ranges_valid(ranges.push(range),len),
{}

pub(crate) proof fn frame_push(frames: Seq<names::Frame>, frame: names::Frame, len: nat)
    requires frames_valid(frames,len),match frame.source {Source::Idat=>true,Source::Fdat(ranges)=>ranges_valid(ranges,len)},
    ensures frames_valid(frames.push(frame),len),
{}

pub(crate) proof fn last_frame_update(frames: Seq<names::Frame>, frame: names::Frame, len: nat)
    requires frames.len()>0,frames_valid(frames,len),match frame.source {Source::Idat=>true,Source::Fdat(ranges)=>ranges_valid(ranges,len)},
    ensures frames_valid(frames.update(frames.len()-1,frame),len),
{}

} // verus!
