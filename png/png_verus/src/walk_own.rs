//! Checked own-value stage of the actual walk, including erased IDAT/fdAT
//! and frame-trace updates. Structural failure sites use checked constructors.
use vstd::prelude::*;
use crate::{walk::{Doc,State,WalkFail,Actl}, png::{Error,Events,Meta}, kind::Kind,
    walk_policy::Rules, critical, apng, walk_state};
#[cfg(verus_only)]
use crate::{document_model as m, walk_model::{self as names,Range,Source}};

verus! {

pub(crate) open spec fn result_view(result: Result<bool,WalkFail>, doc: Doc, state: State,
    pos: usize, events: Events) -> m::Step
{
    match result {
        Ok(false)=>m::Step::Next(state.view(doc,pos,events)),
        Ok(true)=>m::Step::Done(doc.view(),events.view()),
        Err(WalkFail::Format(e))=>m::Step::Error(e,events.view()),
        Err(WalkFail::Fallback)=>m::Step::Fallback(events.view()),
    }
}

/// Ok(true) finishes at IEND; Ok(false) continues with a preserved invariant.
pub(crate) fn run(b: &[u8], base: usize, size: usize, chunk_offset: usize, pos: usize,
    kind: Kind, rules: Rules, static_frames: bool, doc: &mut Doc, state: &mut State, ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires old(state).wf(*old(doc),b@),pos<=b.len(),base<=b.len(),size<=b.len()-base,
        matches!(kind,Kind::Idat) ==> chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(state),pos,*final(ev))==m::own(b@,rules.view(),static_frames,
        old(state).view(*old(doc),pos,*old(ev)),kind.view(),base as nat,size as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(state).wf(*final(doc),b@),
{
    match kind {
        Kind::Ihdr=>run_ihdr(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Plte=>run_plte(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Idat=>run_idat(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Iend=>run_iend(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Trns=>run_trns(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Actl=>run_actl(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Fctl=>run_fctl(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Fdat=>run_fdat(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        Kind::Unknown(_)=>run_unknown(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
        _=>run_ancillary(b,base,size,chunk_offset,pos,kind,rules,static_frames,doc,state,ev),
    }
}

fn run_ihdr(_b: &[u8], _base: usize, _size: usize, _chunk_offset: usize, _pos: usize,
    _kind: Kind, _rules: Rules, _static_frames: bool, _doc: &mut Doc, _state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Ihdr),old(_state).wf(*old(_doc),_b@),_pos<=_b.len(),_base<=_b.len(),_size<=_b.len()-_base,
        matches!(_kind,Kind::Idat) ==> _chunk_offset+8==_base,
    ensures result_view(r,*final(_doc),*final(_state),_pos,*final(_ev))==m::own(_b@,_rules.view(),_static_frames,
        old(_state).view(*old(_doc),_pos,*old(_ev)),_kind.view(),_base as nat,_size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(_doc).wf(_b@),
        r==Ok(false) ==> final(_state).wf(*final(_doc),_b@),
{
    let ghost original_doc=*_doc;
    let ghost original_state=*_state;
    let ghost original_ev=*_ev;
    proof {reveal(m::own);}
    return Err(WalkFail::Format(Error::DuplicateChunk))
}

fn run_plte(_b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    kind: Kind, rules: Rules, _static_frames: bool, doc: &mut Doc, state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(kind,Kind::Plte),old(state).wf(*old(doc),_b@),_pos<=_b.len(),base<=_b.len(),size<=_b.len()-base,
        matches!(kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(state),_pos,*final(_ev))==m::own(_b@,rules.view(),_static_frames,
        old(state).view(*old(doc),_pos,*old(_ev)),kind.view(),base as nat,size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(_b@),
        r==Ok(false) ==> final(state).wf(*final(doc),_b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*state;
    let ghost original_ev=*_ev;
    let ct=doc.ihdr.ct;
    let depth=doc.ihdr.depth;
    proof {reveal(m::own);}
    let entries=match crate::metadata::palette(size,ct,depth,state.idat!=0,
        crate::kind::after_plte_seen(state.seen),rules.order.ignore_unexpected_plte,rules.allow_large_palette) {
        Err(e)=>return Err(WalkFail::Format(e)),Ok(None)=>return Ok(false),Ok(Some(n))=>n,
    };
    doc.plte=Some((base,size));
    doc.info.palette_entries=entries;
    proof {
        assert(result_view(Ok(false),*doc,*state,_pos,*_ev)==m::own(_b@,rules.view(),_static_frames,
            original_state.view(original_doc,_pos,original_ev),kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

fn run_idat(_b: &[u8], _base: usize, _size: usize, chunk_offset: usize, _pos: usize,
    _kind: Kind, _rules: Rules, _static_frames: bool, doc: &mut Doc, state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Idat),old(state).wf(*old(doc),_b@),_pos<=_b.len(),_base<=_b.len(),_size<=_b.len()-_base,
        matches!(_kind,Kind::Idat) ==> chunk_offset+8==_base,
    ensures result_view(r,*final(doc),*final(state),_pos,*final(_ev))==m::own(_b@,_rules.view(),_static_frames,
        old(state).view(*old(doc),_pos,*old(_ev)),_kind.view(),_base as nat,_size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(_b@),
        r==Ok(false) ==> final(state).wf(*final(doc),_b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*state;
    let ghost original_ev=*_ev;
    let ct=doc.ihdr.ct;
    proof {reveal(m::own);}
    let first=match critical::idat(&mut state.idat,ct,doc.plte.is_some()) {
        Err(e)=>return Err(WalkFail::Format(e)),Ok(first)=>first,
    };
    if first {
        if state.apng.begin_idat() {doc.info.default_is_frame=true;}
        doc.idat_first=chunk_offset;
    }
    proof {
        let range=Range {offset:_base as nat,len:_size as nat};
        walk_state::range_push(doc.idat_ranges@,range,_b@.len());
        doc.idat_ranges=Ghost(doc.idat_ranges@.push(range));
    }
    proof {
        assert(result_view(Ok(false),*doc,*state,_pos,*_ev)==m::own(_b@,_rules.view(),_static_frames,
            original_state.view(original_doc,_pos,original_ev),_kind.view(),_base as nat,_size as nat));
    }
    Ok(false)
}

fn run_iend(b: &[u8], _base: usize, _size: usize, _chunk_offset: usize, pos: usize,
    _kind: Kind, rules: Rules, static_frames: bool, doc: &mut Doc, state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Iend),old(state).wf(*old(doc),b@),pos<=b.len(),_base<=b.len(),_size<=b.len()-_base,
        matches!(_kind,Kind::Idat) ==> _chunk_offset+8==_base,
    ensures result_view(r,*final(doc),*final(state),pos,*final(_ev))==m::own(b@,rules.view(),static_frames,
        old(state).view(*old(doc),pos,*old(_ev)),_kind.view(),_base as nat,_size as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(state).wf(*final(doc),b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*state;
    let ghost original_ev=*_ev;
    proof {reveal(m::own);}
    let declared=match doc.actl {None=>None,Some(a)=>Some(a.num_frames)};
    if let Err((e,structural))=critical::iend(state.idat,state.apng,declared,pos,b.len(),rules.ignore_trailing) {
        return Err(critical::route(e,structural,static_frames));
    }
    proof {assert(state.apng.frames<=u32::MAX);}
    doc.info.frame_count=state.apng.frames as u32;
    return Ok(true);
}

fn run_trns(_b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    kind: Kind, rules: Rules, _static_frames: bool, doc: &mut Doc, _state: &mut State, ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(kind,Kind::Trns),old(_state).wf(*old(doc),_b@),_pos<=_b.len(),base<=_b.len(),size<=_b.len()-base,
        matches!(kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(_state),_pos,*final(ev))==m::own(_b@,rules.view(),_static_frames,
        old(_state).view(*old(doc),_pos,*old(ev)),kind.view(),base as nat,size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(_b@),
        r==Ok(false) ==> final(_state).wf(*final(doc),_b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*_state;
    let ghost original_ev=*ev;
    let ct=doc.ihdr.ct;
    proof {reveal(m::own);}
    let keep=match crate::metadata::transparency(size,ct,doc.palette_entries(),rules.drop_invalid_trns) {
        Err(e)=>return Err(WalkFail::Format(e)),Ok(keep)=>keep,
    };
    if !keep {walk_state::discard(doc,kind,ev);return Ok(false);}
    doc.trns=Some((base,size));
    doc.info.trns=Meta::Present(());
    proof {
        assert(doc.info.view().slots =~= original_doc.info.view().slots.insert(names::Name::Trns,m::Slot::Present(m::Stored::Unit)));
        assert(result_view(Ok(false),*doc,*_state,_pos,*ev)==m::own(_b@,rules.view(),_static_frames,
            original_state.view(original_doc,_pos,original_ev),kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

fn run_actl(b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    _kind: Kind, _rules: Rules, static_frames: bool, doc: &mut Doc, _state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Actl),old(_state).wf(*old(doc),b@),_pos<=b.len(),base<=b.len(),size<=b.len()-base,
        matches!(_kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(_state),_pos,*final(_ev))==m::own(b@,_rules.view(),static_frames,
        old(_state).view(*old(doc),_pos,*old(_ev)),_kind.view(),base as nat,size as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(_state).wf(*final(doc),b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*_state;
    let ghost original_ev=*_ev;
    let d=&b[base..base+size];
    proof {reveal(m::own);}
    let (num_frames,num_plays)=match apng::actl(d) {
        Err((e,s))=>return Err(critical::route(e,s,static_frames)),Ok(a)=>a,
    };
    doc.actl=Some(Actl {num_frames,num_plays});
    proof {
        assert(result_view(Ok(false),*doc,*_state,_pos,*_ev)==m::own(b@,_rules.view(),static_frames,
            original_state.view(original_doc,_pos,original_ev),_kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

fn run_fctl(b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    _kind: Kind, _rules: Rules, static_frames: bool, doc: &mut Doc, state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Fctl),old(state).wf(*old(doc),b@),_pos<=b.len(),base<=b.len(),size<=b.len()-base,
        matches!(_kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(state),_pos,*final(_ev))==m::own(b@,_rules.view(),static_frames,
        old(state).view(*old(doc),_pos,*old(_ev)),_kind.view(),base as nat,size as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(state).wf(*final(doc),b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*state;
    let ghost original_ev=*_ev;
    let d=&b[base..base+size];
    proof {reveal(m::own);}
    let (_f,next)=match apng::fctl(d,doc.actl.is_some(),state.idat==0,doc.ihdr.w,doc.ihdr.h,state.apng) {
        Err((e,s))=>return Err(critical::route(e,s,static_frames)),Ok(a)=>a,
    };
    state.apng=next;
    proof {
        let frame=names::Frame {width:_f.w,height:_f.h,x:_f.x,y:_f.y,delay_num:_f.delay_num,delay_den:_f.delay_den,
            dispose:_f.dispose,blend:_f.blend,source:if state.idat==0 {Source::Idat}else{Source::Fdat(Seq::empty())}};
        walk_state::frame_push(doc.frame_ranges@,frame,b@.len());
        doc.frame_ranges=Ghost(doc.frame_ranges@.push(frame));
    }
    proof {
        assert(result_view(Ok(false),*doc,*state,_pos,*_ev)==m::own(b@,_rules.view(),static_frames,
            original_state.view(original_doc,_pos,original_ev),_kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

fn run_fdat(b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    _kind: Kind, _rules: Rules, static_frames: bool, doc: &mut Doc, state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(_kind,Kind::Fdat),old(state).wf(*old(doc),b@),_pos<=b.len(),base<=b.len(),size<=b.len()-base,
        matches!(_kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(state),_pos,*final(_ev))==m::own(b@,_rules.view(),static_frames,
        old(state).view(*old(doc),_pos,*old(_ev)),_kind.view(),base as nat,size as nat),
        !static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(state).wf(*final(doc),b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*state;
    let ghost original_ev=*_ev;
    let d=&b[base..base+size];
    proof {reveal(m::own);}
    let next=match apng::fdat(d,doc.actl.is_some(),state.idat==0,state.apng) {
        Err((e,s))=>return Err(critical::route(e,s,static_frames)),Ok(a)=>a,
    };
    proof {
        assert(size>=4);
        let frames=doc.frame_ranges@;
        let frame=frames.last();
        match frame.source {
            Source::Idat=>assert(false),
            Source::Fdat(ranges)=> {
                let range=Range {offset:base as nat+4,len:(size-4) as nat};
                walk_state::range_push(ranges,range,b@.len());
                let changed=names::Frame {source:Source::Fdat(ranges.push(range)),..frame};
                walk_state::last_frame_update(frames,changed,b@.len());
                doc.frame_ranges=Ghost(frames.update(frames.len()-1,changed));
            },
        }
    }
    state.apng=next;
    proof {
        assert(result_view(Ok(false),*doc,*state,_pos,*_ev)==m::own(b@,_rules.view(),static_frames,
            original_state.view(original_doc,_pos,original_ev),_kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

fn run_unknown(_b: &[u8], _base: usize, _size: usize, _chunk_offset: usize, _pos: usize,
    kind: Kind, _rules: Rules, _static_frames: bool, _doc: &mut Doc, _state: &mut State, _ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires matches!(kind,Kind::Unknown(_)),old(_state).wf(*old(_doc),_b@),_pos<=_b.len(),_base<=_b.len(),_size<=_b.len()-_base,
        matches!(kind,Kind::Idat) ==> _chunk_offset+8==_base,
    ensures result_view(r,*final(_doc),*final(_state),_pos,*final(_ev))==m::own(_b@,_rules.view(),_static_frames,
        old(_state).view(*old(_doc),_pos,*old(_ev)),kind.view(),_base as nat,_size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(_doc).wf(_b@),
        r==Ok(false) ==> final(_state).wf(*final(_doc),_b@),
{
    let ghost original_doc=*_doc;
    let ghost original_state=*_state;
    let ghost original_ev=*_ev;
    proof {reveal(m::own);}
    let anc=match kind {Kind::Unknown(a)=>a,_=>false};
    match critical::unknown(anc) {
    Err(e)=>return Err(WalkFail::Format(e)),Ok(())=>{},
        }
    proof {
        assert(result_view(Ok(false),*_doc,*_state,_pos,*_ev)==m::own(_b@,_rules.view(),_static_frames,
            original_state.view(original_doc,_pos,original_ev),kind.view(),_base as nat,_size as nat));
    }
    Ok(false)
}

fn run_ancillary(b: &[u8], base: usize, size: usize, _chunk_offset: usize, _pos: usize,
    kind: Kind, rules: Rules, _static_frames: bool, doc: &mut Doc, _state: &mut State, ev: &mut Events)
    -> (r: Result<bool,WalkFail>)
    requires !matches!(kind,Kind::Ihdr|Kind::Plte|Kind::Idat|Kind::Iend|Kind::Trns|Kind::Actl|Kind::Fctl|Kind::Fdat|Kind::Unknown(_)),old(_state).wf(*old(doc),b@),_pos<=b.len(),base<=b.len(),size<=b.len()-base,
        matches!(kind,Kind::Idat) ==> _chunk_offset+8==base,
    ensures result_view(r,*final(doc),*final(_state),_pos,*final(ev))==m::own(b@,rules.view(),_static_frames,
        old(_state).view(*old(doc),_pos,*old(ev)),kind.view(),base as nat,size as nat),
        !_static_frames ==> !matches!(r,Err(WalkFail::Fallback)),
        r.is_ok() ==> final(doc).wf(b@),
        r==Ok(false) ==> final(_state).wf(*final(doc),b@),
{
    let ghost original_doc=*doc;
    let ghost original_state=*_state;
    let ghost original_ev=*ev;
    proof {reveal(m::own);}
    match walk_state::ancillary_values(kind,b,base,size,doc) {
    Ok(())=>{},Err(e)=> {
        if rules.order.drop_invalid {walk_state::discard(doc,kind,ev);return Ok(false);}
        return Err(WalkFail::Format(e));
    },
        }
    proof {
        assert(result_view(Ok(false),*doc,*_state,_pos,*ev)==m::own(b@,rules.view(),_static_frames,
            original_state.view(original_doc,_pos,original_ev),kind.view(),base as nat,size as nat));
    }
    Ok(false)
}

} // verus!
