//! Checked complete chunk step: P3, phase advance, CRC, P5, then own values.
use vstd::prelude::*;
use crate::{walk::{Doc,State,WalkFail,Entry}, png::Events, kind::Kind,
    walk_policy::Rules, critical::{self,CrcAction}, ordering::{self,OrderAction}};
#[cfg(verus_only)]
use crate::{document_model as m, walk_model as names};

verus! {

pub(crate) fn run(b: &[u8], rules: Rules, entry: Entry, apng_off: bool,
    pos: &mut usize, doc: &mut Doc, state: &mut State, ev: &mut Events) -> (r: Result<bool,WalkFail>)
    requires old(state).wf(*old(doc),b@),*old(pos)<=b.len(),
    ensures rules.view()==crate::document_model::strict() ==> final(ev).r1==old(ev).r1,
        *final(pos)<=b.len(),
        crate::walk_own::result_view(r,*final(doc),*final(state),*final(pos),*final(ev))
            ==m::step(b@,rules.view(),matches!(entry,Entry::Frames),apng_off,
                old(state).view(*old(doc),*old(pos),*old(ev))),
        (!matches!(entry,Entry::Frames) || !rules.static_apng_errors) ==> !matches!(r,Err(WalkFail::Fallback)),
        final(ev).r3==old(ev).r3,final(ev).r4==old(ev).r4,
        r==Ok(true) ==> final(doc).info.frame_count==final(doc).frame_ranges@.len()
            && final(doc).source_partition() && final(state).idat!=0,
        r.is_ok() ==> final(doc).wf(b@),
        r.is_ok() ==> final(state).wf(*final(doc),b@),
        r==Ok(false) ==> *old(pos)+12<=*final(pos),
{
    let ghost original=state.view(*doc,*pos,*ev);
    proof {m::step_progress(b@,rules.view(),matches!(entry,Entry::Frames),apng_off,original);reveal(m::step);}
    let candidate=match critical::read(b,*pos,state.idat,rules.allow_eof_iend) {
        Err(e)=>return Err(WalkFail::Format(e)),Ok(c)=>c,
    };
    let c=candidate.chunk;
    let wire=candidate.kind;
    let chunk_offset=*pos;
    ordering::advance_phase(&mut state.idat,wire);
    *pos=c.next;
    let frames_entry=matches!(entry,Entry::Frames);
    let apng_on=!apng_off && !(!frames_entry && rules.ignore_apng_in_still);
    let static_frames=frames_entry && rules.static_apng_errors;
    let (kind,anc)=critical::effective(wire,wire.ancillary(&c.ty),apng_on);
    proof {
        assert(state.wf(*doc,b@));
        assert(state.view(*doc,*pos,*ev)==(m::State {pos:c.next as nat,
            phase:names::advance_phase(original.phase,wire.view()),..original}));
    }
    match critical::crc_rule(wire,kind,anc,c.crc_ok,rules.skip_ancillary_crc,rules.order.lenient_iend,frames_entry) {
        CrcAction::Fail(e,s)=>return Err(critical::route(e,s,static_frames)),
        CrcAction::Drop=>{crate::walk_state::discard(doc,kind,ev);return Ok(false);},
        CrcAction::Keep=>{},
    }
    proof {if matches!(kind,Kind::Idat) {assert(chunk_offset+8==c.data);}}
    ordered(b,rules,static_frames,kind,anc,c.data,c.len,chunk_offset,*pos,doc,state,ev)
}

fn ordered(b: &[u8], rules: Rules, static_frames: bool, kind: Kind, anc: bool,
    base: usize, length: usize, chunk_offset: usize, pos: usize,
    doc: &mut Doc, state: &mut State, ev: &mut Events) -> (r: Result<bool,WalkFail>)
    requires old(state).wf(*old(doc),b@),pos<=b.len(),base<=b.len(),length<=b.len()-base,
        matches!(kind,Kind::Idat) ==> chunk_offset+8==base,
    ensures rules.view()==crate::document_model::strict() ==> final(ev).r1==old(ev).r1,
        crate::walk_own::result_view(r,*final(doc),*final(state),pos,*final(ev))
        ==m::ordered(b@,rules.view(),static_frames,old(state).view(*old(doc),pos,*old(ev)),
            kind.view(),anc,base as nat,length as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        final(ev).r3==old(ev).r3,final(ev).r4==old(ev).r4,
        r==Ok(true) ==> final(doc).info.frame_count==final(doc).frame_ranges@.len()
            && final(doc).source_partition() && final(state).idat!=0,
        r.is_ok() ==> final(doc).wf(b@),
        r.is_ok() ==> final(state).wf(*final(doc),b@),
{
    proof {reveal(m::ordered);}
    let size=match ordering::preamble(state.seen,state.idat,doc.plte.is_some(),doc.ihdr.ct,kind,anc,length,rules.order) {
        OrderAction::Fail(e,s)=>return Err(critical::route(e,s,static_frames)),
        OrderAction::Drop=>{crate::walk_state::discard(doc,kind,ev);return Ok(false);},
        OrderAction::Keep(n)=>n,
    };
    crate::kind::mark_seen(&mut state.seen,kind);
    proof {
        assert(state.wf(*doc,b@));
        if matches!(kind,Kind::Idat) {assert(chunk_offset+8==base);}
    }
    crate::walk_own::run(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev)
}

} // verus!
