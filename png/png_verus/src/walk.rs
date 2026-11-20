//! The structural pass, SPEC P2–P8 and P14.1, under a policy record
//! (`../../policy-draft/policy_oracle.py` `_walk`, restricted to the values
//! Strict and RecommendedBeta use). No pixel is decoded and no zlib stream is
//! inflated here. The result is a fixed-size record: no chunk list and no frame
//! list (SPEC P20); data chunks are re-walked by offset (`reader.rs`).
//!
//! Check order per chunk: P3 (framing, letters), the IDAT phase, the CRC rule,
//! P5.1 duplicates, P5.2 ordering, P5.3 fixed lengths, then the chunk's own
//! section, exactly as the oracle's `_walk`.

use crate::apng::Control;
use vstd::prelude::*;
use crate::png::{Error, Events, Info};
use crate::policy::Policy;

pub(crate) use crate::framing::{be16, be32, frame_at};

pub(crate) use crate::kind::{kind_of, Kind};

pub(crate) use crate::header::Ihdr;

verus! {

/// acTL (P14.1).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Actl {
    pub num_frames: u32,
    pub num_plays: u32,
}

/// The walk's result: the document record used by P11–P17.
pub(crate) struct Doc {
    pub ihdr: Ihdr,
    /// PLTE data `(offset, length)` in the input.
    pub plte: Option<(usize, usize)>,
    /// tRNS data `(offset, length)`.
    pub trns: Option<(usize, usize)>,
    /// The compressed profile field of iCCP `(offset, length)`.
    pub iccp: Option<(usize, usize)>,
    /// Offset of the first IDAT chunk (the re-walk starts there, L-REWALK).
    pub idat_first: usize,
    pub actl: Option<Actl>,
    pub info: Info,
    #[cfg(verus_only)]
    pub(crate) idat_ranges: Ghost<Seq<crate::walk_model::Range>>,
    #[cfg(verus_only)]
    pub(crate) frame_ranges: Ghost<Seq<crate::walk_model::Frame>>,
}

impl Doc {
    pub(crate) fn palette_entries(&self) -> (r: usize)
        ensures r==crate::document_model::entries(self.view()),
    {
        match self.plte {
            Some((_, n)) => n / 3,
            None => 0,
        }
    }
}

/// Which entry point runs the walk (`policy_oracle.py` `entry`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Entry {
    Still,
    Frames,
}

/// Why the walk stopped without a document.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WalkFail {
    Format(Error),
    /// `apng_errors = Static` on the frames entry: an APNG structural error
    /// (`sfail`, `apng_fail`); the caller re-walks with APNG off (R4).
    Fallback,
}

impl WalkFail {
    pub(crate) open spec fn view(self) -> crate::critical_model::Failure {
        match self {
            Self::Format(e) => crate::critical_model::Failure::Format(e),
            Self::Fallback => crate::critical_model::Failure::Fallback,
        }
    }
}

/// The per-walk state (P5): a fixed-size record.
pub(crate) struct State {
    pub(crate) seen: u32,
    /// 0 before the first IDAT, 1 in the IDAT run, 2 after it.
    pub(crate) idat: u8,
    pub(crate) apng: Control,
}

} // verus!

/// P2–P8, P14.1 under `pol`. `apng_off` is the re-walk of the R4 fallback, with
/// acTL, fcTL and fdAT treated as unknown ancillary chunks (`apng = Off`).
verus! {

pub(crate) open spec fn outcome_view(result: Result<Doc,WalkFail>, events: Events) -> crate::document_model::Outcome {
    match result {
        Ok(doc)=>crate::document_model::Outcome::Done(doc.view(),events.view()),
        Err(WalkFail::Format(e))=>crate::document_model::Outcome::Error(e,events.view()),
        Err(WalkFail::Fallback)=>crate::document_model::Outcome::Fallback(events.view()),
    }
}

pub(crate) fn walk(b: &[u8], pol: &Policy, entry: Entry, apng_off: bool, ev: &mut Events) -> (r: Result<Doc,WalkFail>)
    ensures final(ev).r3==old(ev).r3,final(ev).r4==old(ev).r4,
        outcome_view(r,*final(ev))==crate::document_model::start(b@,pol.walk_view(),
        matches!(entry,Entry::Frames),apng_off,old(ev).view()),
        crate::document_model::start(b@,pol.walk_view(),matches!(entry,Entry::Frames),apng_off,old(ev).view())
            != crate::document_model::Outcome::OutsideDomain,
        (!matches!(entry,Entry::Frames) || !pol.walk_view().static_apng_errors) ==> !matches!(r,Err(WalkFail::Fallback)),
        match r {Ok(doc)=>doc.wf(b@) && crate::transport_model::idat_result(b@,doc.view())
            && b.len()<=crate::limits::MAX_INPUT && doc.source_partition()
            && doc.info.frame_count==doc.frame_ranges@.len()
            && (matches!(entry,Entry::Frames) && !apng_off ==> crate::frame_transport_model::result(b@,doc.view(),crate::frame_transport_model::header_end(b@)))
            && (apng_off ==> crate::frame_transport_model::default_only(doc.view())),Err(_)=>true},
{
    let (ihdr,after_ihdr)=match crate::header::prefix(b) {
        Err(e)=>return Err(WalkFail::Format(e)),Ok(h)=>h,
    };
    let mut doc=Doc::new(ihdr,b);
    let mut state=State::new(&doc,b);
    let rules=pol.walk_rules();
    let mut pos=after_ihdr;
    loop
        invariant state.wf(doc,b@),pos<=b.len(),rules.view()==pol.walk_view(),
            ev.r3==old(ev).r3,ev.r4==old(ev).r4,
            crate::document_model::start(b@,pol.walk_view(),matches!(entry,Entry::Frames),apng_off,old(ev).view())
                ==crate::document_model::drive(b@,rules.view(),matches!(entry,Entry::Frames),apng_off,state.view(doc,pos,*ev)),
        decreases b.len()-pos,
    {
        proof {crate::document_model::drive_step(b@,rules.view(),matches!(entry,Entry::Frames),apng_off,state.view(doc,pos,*ev));}
        match crate::walk_step::run(b,rules,entry,apng_off,&mut pos,&mut doc,&mut state,ev) {
            Err(e)=>return Err(e),Ok(true)=> {
                proof {
                    crate::transport_model::start_idat(b@,pol.walk_view(),matches!(entry,Entry::Frames),apng_off,old(ev).view());
                    if matches!(entry,Entry::Frames) && !apng_off {crate::frame_transport_model::start_frames(b@,pol.walk_view(),old(ev).view());}
                    if apng_off {crate::frame_transport_model::start_off(b@,pol.walk_view(),matches!(entry,Entry::Frames),old(ev).view());}
                }
                return Ok(doc);
            },Ok(false)=>{},
        }
    }
}

pub(crate) open spec fn data_outcome(result: Result<Doc,Error>, events: Events) -> crate::document_model::Outcome {
    match result {Ok(doc)=>crate::document_model::Outcome::Done(doc.view(),events.view()),
        Err(e)=>crate::document_model::Outcome::Error(e,events.view())}
}

/// The actual still/ICC walk entry, with fresh recovery events.
pub(crate) fn still(b: &[u8], pol: &Policy, ev: &mut Events) -> (r: Result<Doc,Error>)
    requires old(ev).view()==crate::document_model::no_events(),
    ensures data_outcome(r,*final(ev))==crate::model::MW_p(b@,pol.walk_view()),
        crate::model::MW_p(b@,pol.walk_view()) != crate::document_model::Outcome::OutsideDomain,
        match r {Ok(doc)=>doc.wf(b@) && crate::transport_model::idat_result(b@,doc.view())
            && b.len()<=crate::limits::MAX_INPUT && doc.source_partition()
            && doc.info.frame_count==doc.frame_ranges@.len(),Err(_)=>true},
{
    match walk(b,pol,Entry::Still,false,ev) {
        Ok(doc)=>Ok(doc),Err(WalkFail::Format(e))=>Err(e),Err(WalkFail::Fallback)=>{assert(false);Err(Error::MissingAnimationControl)},
    }
}

/// The actual constructor walk, including a fresh APNG-off R4 re-walk.
pub(crate) fn frames(b: &[u8], pol: &Policy, ev: &mut Events) -> (r: Result<Doc,Error>)
    requires old(ev).view()==crate::document_model::no_events(),
    ensures data_outcome(r,*final(ev))==crate::model::MWF_p(b@,pol.walk_view()),
        crate::model::MWF_p(b@,pol.walk_view()) != crate::document_model::Outcome::OutsideDomain,
        match r {Ok(doc)=>doc.wf(b@) && crate::transport_model::idat_result(b@,doc.view())
            && b.len()<=crate::limits::MAX_INPUT && doc.source_partition()
            && doc.info.frame_count==doc.frame_ranges@.len()
            && (doc.actl.is_some() ==> crate::frame_transport_model::result(b@,doc.view(),crate::frame_transport_model::header_end(b@)))
            && (final(ev).r4 ==> crate::frame_transport_model::default_only(doc.view())),Err(_)=>true},
{
    match walk(b,pol,Entry::Frames,false,ev) {
        Ok(doc)=>Ok(doc),Err(WalkFail::Format(e))=>Err(e),Err(WalkFail::Fallback)=> {
            ev.r4=true;
            match walk(b,pol,Entry::Frames,true,ev) {
                Ok(doc)=>Ok(doc),Err(WalkFail::Format(e))=>Err(e),
                Err(WalkFail::Fallback)=>Err(Error::MissingAnimationControl),
            }
        },
    }
}

} // verus!

pub(crate) use crate::apng::Fctl;
