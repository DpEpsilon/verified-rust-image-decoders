//! Independent wire-frame cursor selection and transport to the modeled
//! animation frames. The runtime still scans lengths and types, never CRCs.
use vstd::prelude::*;
use crate::{model,document_model as m,walk_model::{self as names,Name,Phase,Frame,Source,Range},critical_model as critical};
use crate::transport_model::{self as idat,trace_bytes,valid_trace};

verus! {

pub open spec fn next_frame(b: Seq<u8>, pos: nat) -> Option<nat>
    decreases b.len()-pos,
{
    match model::frame(b,pos) {
        Err(_)=>None,Ok(c)=> {
            if c.ty==seq![b'I',b'E',b'N',b'D'] {None}
            else if c.ty==seq![b'f',b'c',b'T',b'L'] {Some(pos)}
            else {next_frame(b,c.next)}
        },
    }
}
pub open spec fn frame_start(b: Seq<u8>, frame: Frame) -> nat {
    match model::frame(b,frame.control_offset) {Ok(c)=>c.next,Err(_)=>b.len()}
}
pub open spec fn control_record(frame: Frame, control: crate::apng_model::FrameControl) -> bool {
    frame.width==control.width && frame.height==control.height && frame.x==control.x && frame.y==control.y
    && frame.delay_num==control.delay_num && frame.delay_den==control.delay_den
    && frame.dispose==control.dispose && frame.blend==control.blend
}
pub open spec fn valid_site(b: Seq<u8>, frame: Frame) -> bool {
    match model::frame(b,frame.control_offset) {
        Err(_)=>false,Ok(c)=>c.ty==seq![b'f',b'c',b'T',b'L'] && c.len==26
            && model::crc(b.subrange(c.data as int-4,(c.data+c.len) as int))==model::be32(b,(c.data+c.len) as int)
            && control_record(frame,crate::apng_model::parse_fctl(b.subrange(c.data as int,(c.data+c.len) as int))),
    }
}
pub open spec fn frame_stream(b: Seq<u8>, frame: Frame) -> Seq<u8> {model::reader_bytes(b,frame_start(b,frame),true)}
pub open spec fn header_end(b: Seq<u8>) -> nat {match model::mh(b) {Ok((_,p))=>p,Err(_)=>0}}

pub open spec fn linked(b: Seq<u8>, frames: Seq<Frame>, root: nat, pos: nat) -> bool {
    if frames.len()==0 {next_frame(b,root)==next_frame(b,pos)} else {
        next_frame(b,root)==Some(frames[0].control_offset)
        && (forall|i: int| 0<i<frames.len() ==> next_frame(b,frame_start(b,frames[i-1]))==Some((#[trigger] frames[i]).control_offset))
        && next_frame(b,frame_start(b,frames.last()))==next_frame(b,pos)
    }
}
pub open spec fn closed_frame(b: Seq<u8>, frame: Frame) -> bool {
    match frame.source {Source::Idat=>true,Source::Fdat(ranges)=>valid_trace(b,ranges)
        && frame_stream(b,frame)==trace_bytes(b,ranges)
        && model::reader_count(b,frame_start(b,frame),true)>=ranges.len()}
}
pub open spec fn open_frame(b: Seq<u8>, frame: Frame, pos: nat) -> bool {
    match frame.source {Source::Idat=>true,Source::Fdat(ranges)=>valid_trace(b,ranges)
        && frame_stream(b,frame)==trace_bytes(b,ranges)+model::reader_bytes(b,pos,true)
        && model::reader_count(b,frame_start(b,frame),true)>=ranges.len()+model::reader_count(b,pos,true)}
}
pub open spec fn span_inv(b: Seq<u8>, state: m::State, root: nat) -> bool {
    linked(b,state.doc.frames,root,state.pos)
    && forall|i: int| 0<=i<state.doc.frames.len() ==> valid_site(b,#[trigger] state.doc.frames[i])
        && if i+1==state.doc.frames.len() {open_frame(b,state.doc.frames[i],state.pos)}
            else{closed_frame(b,state.doc.frames[i])}
}
pub open spec fn result(b: Seq<u8>, doc: m::Doc, root: nat) -> bool {
    linked(b,doc.frames,root,b.len())
    && forall|i: int| 0<=i<doc.frames.len() ==> valid_site(b,#[trigger] doc.frames[i]) && closed_frame(b,doc.frames[i])
}

/// Raw re-walking charges every non-boundary chunk, even when its byte
/// contribution is empty. An fdAT contributes only bytes after its sequence.
pub proof fn read_frame(b: Seq<u8>, pos: nat, phase: Phase, allow_eof: bool)
    ensures match critical::read(b,pos,phase,allow_eof) {
        Err(_)=>true,Ok(c)=> {
            let name=names::classify(c.ty);
            c.data+c.len<=b.len() && c.next<=b.len()
            && (if c.synthetic {name==Name::Iend} else {
                c.data==pos+8 && model::frame(b,pos)==Ok(model::WireChunk {ty:c.ty,data:c.data,len:c.len,next:c.next})
                && c.crc_ok==(model::crc(b.subrange(c.data as int-4,(c.data+c.len) as int))==model::be32(b,(c.data+c.len) as int))
            })
            && if name==Name::Iend || name==Name::Fctl {
                model::reader_bytes(b,pos,true)==Seq::<u8>::empty() && model::reader_count(b,pos,true)==0
                && next_frame(b,pos)==if name==Name::Fctl {Some(pos)}else{None}
            }else{
                next_frame(b,pos)==next_frame(b,c.next)
                && model::reader_count(b,pos,true)==1+model::reader_count(b,c.next,true)
                && model::reader_bytes(b,pos,true)==(if name==Name::Fdat && c.len>=4 {
                    b.subrange(c.data as int+4,(c.data+c.len) as int)
                }else{Seq::<u8>::empty()})+model::reader_bytes(b,c.next,true)
            }
        },
    },
{}

pub proof fn next_frame_bounds(b: Seq<u8>, pos: nat)
    ensures match next_frame(b,pos) {
        None=>true,Some(q)=>pos<=q && q<b.len() && match model::frame(b,q) {
            Err(_)=>false,Ok(c)=>c.ty==seq![b'f',b'c',b'T',b'L'],
        },
    },
    decreases b.len()-pos,
{
    match model::frame(b,pos) {
        Err(_)=>{},Ok(c)=>if c.ty!=seq![b'I',b'E',b'N',b'D'] && c.ty!=seq![b'f',b'c',b'T',b'L'] {
            next_frame_bounds(b,c.next);
        },
    }
}

pub open spec fn frames_effect(b: Seq<u8>, state: m::State, next: m::State, name: Name, data: nat, length: nat) -> bool {
    if name==Name::Fctl {
        length==26 && next.doc.frames.len()==state.doc.frames.len()+1
        && next.doc.frames.drop_last()==state.doc.frames
        && next.doc.frames.last().control_offset==(data-8) as nat
        && control_record(next.doc.frames.last(),crate::apng_model::parse_fctl(b.subrange(data as int,(data+length) as int)))
        && match next.doc.frames.last().source {
            Source::Idat=>matches!(state.phase,Phase::Before),
            Source::Fdat(ranges)=>!matches!(state.phase,Phase::Before) && ranges.len()==0,
        }
    }else if name==Name::Fdat {
        length>=4 && state.doc.frames.len()>0 && match state.doc.frames.last().source {
            Source::Idat=>false,Source::Fdat(ranges)=>next.doc.frames==state.doc.frames.update(state.doc.frames.len()-1,
                Frame {source:Source::Fdat(ranges.push(Range {offset:data+4,len:(length-4) as nat})),..state.doc.frames.last()}),
        }
    }else{next.doc.frames==state.doc.frames}
}

pub proof fn own_frame_effect(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, data: nat, length: nat)
    ensures match m::own(b,policy,static_frames,state,name,data,length) {
        m::Step::Next(next)=>name!=Name::Iend && frames_effect(b,state,next,name,data,length),
        m::Step::Done(doc,_)=>doc.frames==state.doc.frames,_=>true,
    },
{
    reveal(m::own);
    match m::own(b,policy,static_frames,state,name,data,length) {
        m::Step::Next(next)=>if name==Name::Fctl {
            assert(next.doc.frames.drop_last() =~= state.doc.frames);
        }else if name==Name::Fdat {
            assert(length>=4);
        },_=>{},
    }
}

pub proof fn ordered_frame_effect(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, ancillary: bool, data: nat, length: nat)
    ensures match m::ordered(b,policy,static_frames,state,name,ancillary,data,length) {
        m::Step::Next(next)=>(name!=Name::Iend || ancillary) && frames_effect(b,state,next,name,data,length),
        m::Step::Done(doc,_)=>doc.frames==state.doc.frames,_=>true,
    },
{
    reveal(m::ordered);
    match names::order(state.seen,state.phase,state.doc.plte.is_some(),state.doc.ihdr.color_type,
        name,ancillary,length,policy.order) {
        names::OrderAction::Keep(len)=>own_frame_effect(b,policy,static_frames,m::State {
            seen:if names::singleton(name) {state.seen.insert(name)}else{state.seen},..state},name,data,len),
        _=>{},
    }
}

/// Accepted enabled-frame steps cannot cross an unchecked fcTL boundary.
pub proof fn active_step_shape(b: Seq<u8>, policy: m::Policy, state: m::State)
    ensures match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(_)=>true,Ok(c)=>match m::step(b,policy,true,false,state) {
            m::Step::Next(next)=>next.pos==c.next && names::classify(c.ty)!=Name::Iend
                && frames_effect(b,state,next,names::classify(c.ty),c.data,c.len)
                && if names::classify(c.ty)==Name::Fctl {
                c.crc_ok && c.len==26 && next.doc.frames.len()==state.doc.frames.len()+1
                && next.doc.frames.drop_last()==state.doc.frames
                && next.doc.frames.last().control_offset==(c.data-8) as nat
                && match next.doc.frames.last().source {
                    Source::Idat=>matches!(state.phase,Phase::Before),
                    Source::Fdat(ranges)=>!matches!(state.phase,Phase::Before) && ranges.len()==0,
                }
            }else{true},
            m::Step::Done(doc,_)=>names::classify(c.ty)==Name::Iend && doc.frames==state.doc.frames,
            _=>true,
        },
    },
{
    reveal(m::step);
    match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(_)=>{},Ok(c)=> {
            let wire=names::classify(c.ty);
            let ancillary=matches!(wire,Name::Unknown(true)) || (c.ty.len()==4 && c.ty[0]&0x20u8!=0);
            let advanced=m::State {pos:c.next,phase:names::advance_phase(state.phase,wire),..state};
            if wire==Name::Iend {
                assert(c.ty==seq![b'I',b'E',b'N',b'D']);
                assert(b'I'&0x20u8==0u8) by (bit_vector);
                assert(!ancillary);
            }
            ordered_frame_effect(b,policy,policy.static_apng_errors,advanced,wire,ancillary,c.data,c.len);
            m::ordered_position(b,policy,policy.static_apng_errors,advanced,wire,ancillary,c.data,c.len);
            idat::ordered_idat_effect(b,policy,policy.static_apng_errors,advanced,wire,ancillary,c.data,c.len);
        },
    }
}

pub proof fn close_frame(b: Seq<u8>, frame: Frame, pos: nat)
    requires open_frame(b,frame,pos),model::reader_bytes(b,pos,true)==Seq::<u8>::empty(),model::reader_count(b,pos,true)==0,
    ensures closed_frame(b,frame),
{}

pub proof fn append_frame(b: Seq<u8>, state: m::State, next: m::State, root: nat, frame: Frame)
    requires span_inv(b,state,root),next.doc.frames==state.doc.frames.push(frame),
        frame.control_offset==state.pos,valid_site(b,frame),next.pos==frame_start(b,frame),
        next_frame(b,state.pos)==Some(state.pos),model::reader_bytes(b,state.pos,true)==Seq::<u8>::empty(),
        model::reader_count(b,state.pos,true)==0,
        match frame.source {Source::Idat=>true,Source::Fdat(ranges)=>ranges.len()==0},
    ensures span_inv(b,next,root),
{
    assert forall|i: int| 0<=i<next.doc.frames.len() implies valid_site(b,#[trigger] next.doc.frames[i])
        && if i+1==next.doc.frames.len() {open_frame(b,next.doc.frames[i],next.pos)}else{closed_frame(b,next.doc.frames[i])} by {
        if i<state.doc.frames.len() && i+1==state.doc.frames.len() {close_frame(b,state.doc.frames[i],state.pos);}
    }
}

pub proof fn advance_frame(b: Seq<u8>, state: m::State, next: m::State, root: nat)
    requires span_inv(b,state,root),next.doc.frames==state.doc.frames,
        next_frame(b,state.pos)==next_frame(b,next.pos),
        model::reader_bytes(b,state.pos,true)==model::reader_bytes(b,next.pos,true),
        model::reader_count(b,state.pos,true)==1+model::reader_count(b,next.pos,true),
    ensures span_inv(b,next,root),
{}

pub proof fn update_frame(b: Seq<u8>, state: m::State, next: m::State, root: nat, ranges: Seq<Range>, range: Range)
    requires span_inv(b,state,root),state.doc.frames.len()>0,state.doc.frames.last().source==Source::Fdat(ranges),
        next.doc.frames==state.doc.frames.update(state.doc.frames.len()-1,
            Frame {source:Source::Fdat(ranges.push(range)),..state.doc.frames.last()}),
        range.offset+range.len<=b.len(),next_frame(b,state.pos)==next_frame(b,next.pos),
        model::reader_bytes(b,state.pos,true)==b.subrange(range.offset as int,(range.offset+range.len) as int)
            +model::reader_bytes(b,next.pos,true),
        model::reader_count(b,state.pos,true)==1+model::reader_count(b,next.pos,true),
    ensures span_inv(b,next,root),
{
    idat::concat_append(b,ranges,range);
    let prefix=trace_bytes(b,ranges);
    let payload=b.subrange(range.offset as int,(range.offset+range.len) as int);
    let tail=model::reader_bytes(b,next.pos,true);
    assert(prefix+(payload+tail) =~= (prefix+payload)+tail);
    assert forall|i: int| 0<=i<next.doc.frames.len() implies valid_site(b,#[trigger] next.doc.frames[i])
        && if i+1==next.doc.frames.len() {open_frame(b,next.doc.frames[i],next.pos)}else{closed_frame(b,next.doc.frames[i])} by {}
}

pub proof fn finish_frames(b: Seq<u8>, state: m::State, doc: m::Doc, root: nat)
    requires span_inv(b,state,root),doc.frames==state.doc.frames,next_frame(b,state.pos).is_none(),
        model::reader_bytes(b,state.pos,true)==Seq::<u8>::empty(),model::reader_count(b,state.pos,true)==0,
    ensures result(b,doc,root),
{
    assert(next_frame(b,b.len()).is_none());
    assert forall|i: int| 0<=i<doc.frames.len() implies valid_site(b,#[trigger] doc.frames[i]) && closed_frame(b,doc.frames[i]) by {
        if i+1==doc.frames.len() {close_frame(b,doc.frames[i],state.pos);}
    }
}

pub proof fn next_fctl_step(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat, c: critical::Candidate, next: m::State)
    requires span_inv(b,state,root),critical::read(b,state.pos,state.phase,policy.allow_eof_iend)==Ok(c),
        m::step(b,policy,true,false,state)==m::Step::Next(next),names::classify(c.ty)==Name::Fctl,
    ensures span_inv(b,next,root),
{
    active_step_shape(b,policy,state);read_frame(b,state.pos,state.phase,policy.allow_eof_iend);
    let frame=next.doc.frames.last();
    assert(c.ty==seq![b'f',b'c',b'T',b'L']);
    assert(frame.control_offset==state.pos);
    assert(next.doc.frames =~= state.doc.frames.push(frame));
    append_frame(b,state,next,root,frame);
}

pub proof fn next_fdat_step(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat, c: critical::Candidate, next: m::State)
    requires span_inv(b,state,root),critical::read(b,state.pos,state.phase,policy.allow_eof_iend)==Ok(c),
        m::step(b,policy,true,false,state)==m::Step::Next(next),names::classify(c.ty)==Name::Fdat,
    ensures span_inv(b,next,root),
{
    active_step_shape(b,policy,state);read_frame(b,state.pos,state.phase,policy.allow_eof_iend);
    let range=Range {offset:c.data+4,len:(c.len-4) as nat};
    match state.doc.frames.last().source {
        Source::Idat=>assert(false),Source::Fdat(ranges)=>update_frame(b,state,next,root,ranges,range),
    }
}

pub proof fn next_other_step(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat, c: critical::Candidate, next: m::State)
    requires span_inv(b,state,root),critical::read(b,state.pos,state.phase,policy.allow_eof_iend)==Ok(c),
        m::step(b,policy,true,false,state)==m::Step::Next(next),names::classify(c.ty)!=Name::Fctl,names::classify(c.ty)!=Name::Fdat,
    ensures span_inv(b,next,root),
{
    active_step_shape(b,policy,state);read_frame(b,state.pos,state.phase,policy.allow_eof_iend);
    advance_frame(b,state,next,root);
}

pub proof fn done_step(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat, c: critical::Candidate, doc: m::Doc, events: m::Events)
    requires span_inv(b,state,root),critical::read(b,state.pos,state.phase,policy.allow_eof_iend)==Ok(c),
        m::step(b,policy,true,false,state)==m::Step::Done(doc,events),
    ensures result(b,doc,root),
{
    active_step_shape(b,policy,state);read_frame(b,state.pos,state.phase,policy.allow_eof_iend);
    finish_frames(b,state,doc,root);
}

pub proof fn step_frames(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat)
    requires span_inv(b,state,root),
    ensures match m::step(b,policy,true,false,state) {
        m::Step::Next(next)=>span_inv(b,next,root),m::Step::Done(doc,_)=>result(b,doc,root),_=>true,
    },
{
    match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(_)=>{reveal(m::step);},Ok(c)=>match m::step(b,policy,true,false,state) {
            m::Step::Next(next)=> {
                let wire=names::classify(c.ty);
                if wire==Name::Fctl {next_fctl_step(b,policy,state,root,c,next);}
                else if wire==Name::Fdat {next_fdat_step(b,policy,state,root,c,next);}
                else{next_other_step(b,policy,state,root,c,next);}
            },
            m::Step::Done(doc,events)=>done_step(b,policy,state,root,c,doc,events),_=>{},
        },
    }
}

pub proof fn drive_frames(b: Seq<u8>, policy: m::Policy, state: m::State, root: nat)
    requires span_inv(b,state,root),state.pos<=b.len(),
    ensures match m::drive(b,policy,true,false,state) {m::Outcome::Done(doc,_)=>result(b,doc,root),_=>true},
    decreases b.len()-state.pos,
{
    m::drive_step(b,policy,true,false,state);m::step_progress(b,policy,true,false,state);
    step_frames(b,policy,state,root);
    match m::step(b,policy,true,false,state) {m::Step::Next(next)=>drive_frames(b,policy,next,root),_=>{}}
}

pub proof fn start_frames(b: Seq<u8>, policy: m::Policy, events: m::Events)
    ensures match m::start(b,policy,true,false,events) {m::Outcome::Done(doc,_)=>result(b,doc,header_end(b)),_=>true},
{
    match model::mh(b) {Err(_)=>{},Ok((h,pos))=>drive_frames(b,policy,m::initial(h,pos,events),pos)}
}

pub open spec fn default_only(doc: m::Doc) -> bool {
    doc.actl.is_none() && doc.frames.len()==0 && !doc.info.default_is_frame
}
pub open spec fn off_state(state: m::State) -> bool {
    default_only(state.doc) && state.control==crate::apng_model::initial()
}

pub proof fn own_off(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, data: nat, length: nat)
    requires off_state(state),!names::apng(name),
    ensures match m::own(b,policy,static_frames,state,name,data,length) {
        m::Step::Next(next)=>off_state(next),m::Step::Done(doc,_)=>default_only(doc),_=>true,
    },
{
    reveal(m::own);
}

pub proof fn ordered_off(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, ancillary: bool, data: nat, length: nat)
    requires off_state(state),!names::apng(name),
    ensures match m::ordered(b,policy,static_frames,state,name,ancillary,data,length) {
        m::Step::Next(next)=>off_state(next),m::Step::Done(doc,_)=>default_only(doc),_=>true,
    },
{
    reveal(m::ordered);
    match names::order(state.seen,state.phase,state.doc.plte.is_some(),state.doc.ihdr.color_type,
        name,ancillary,length,policy.order) {
        names::OrderAction::Keep(len)=>own_off(b,policy,static_frames,m::State {
            seen:if names::singleton(name) {state.seen.insert(name)}else{state.seen},..state},name,data,len),_=>{},
    }
}

pub proof fn step_off(b: Seq<u8>, policy: m::Policy, frames_entry: bool, state: m::State)
    requires off_state(state),
    ensures match m::step(b,policy,frames_entry,true,state) {
        m::Step::Next(next)=>off_state(next),m::Step::Done(doc,_)=>default_only(doc),_=>true,
    },
{
    reveal(m::step);
    match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(_)=>{},Ok(c)=> {
            let wire=names::classify(c.ty);
            let (name,anc)=critical::effective(wire,matches!(wire,Name::Unknown(true)) || (c.ty.len()==4 && c.ty[0]&0x20u8!=0),false);
            let advanced=m::State {pos:c.next,phase:names::advance_phase(state.phase,wire),..state};
            ordered_off(b,policy,frames_entry && policy.static_apng_errors,advanced,name,anc,c.data,c.len);
        },
    }
}

pub proof fn drive_off(b: Seq<u8>, policy: m::Policy, frames_entry: bool, state: m::State)
    requires off_state(state),state.pos<=b.len(),
    ensures match m::drive(b,policy,frames_entry,true,state) {m::Outcome::Done(doc,_)=>default_only(doc),_=>true},
    decreases b.len()-state.pos,
{
    m::drive_step(b,policy,frames_entry,true,state);m::step_progress(b,policy,frames_entry,true,state);
    step_off(b,policy,frames_entry,state);
    match m::step(b,policy,frames_entry,true,state) {m::Step::Next(next)=>drive_off(b,policy,frames_entry,next),_=>{}}
}

pub proof fn start_off(b: Seq<u8>, policy: m::Policy, frames_entry: bool, events: m::Events)
    ensures match m::start(b,policy,frames_entry,true,events) {m::Outcome::Done(doc,_)=>default_only(doc),_=>true},
{
    match model::mh(b) {Err(_)=>{},Ok((h,pos))=>drive_off(b,policy,frames_entry,m::initial(h,pos,events))}
}

pub proof fn constructor_frames(b: Seq<u8>, policy: m::Policy)
    ensures match m::MWF_p(b,policy) {m::Outcome::Done(doc,_)=>default_only(doc) || result(b,doc,header_end(b)),_=>true},
{
    start_frames(b,policy,m::no_events());
    match m::start(b,policy,true,false,m::no_events()) {
        m::Outcome::Fallback(ev)=>start_off(b,policy,true,m::Events {r4:true,..ev}),_=>{},
    }
}

pub open spec fn cursor_for(b: Seq<u8>, frames: Seq<Frame>, root: nat, index: nat) -> nat {
    if index==0 {root}else{frame_start(b,frames[index as int-1])}
}

pub proof fn select_frame(b: Seq<u8>, doc: m::Doc, root: nat, index: nat)
    requires result(b,doc,root),index<doc.frames.len(),
    ensures next_frame(b,cursor_for(b,doc.frames,root,index))==Some(doc.frames[index as int].control_offset),
        valid_site(b,doc.frames[index as int]),closed_frame(b,doc.frames[index as int]),
{}

/// Frame transport depends on the frame trace, not the public Info projection.
pub proof fn result_same_frames(b: Seq<u8>, before: m::Doc, after: m::Doc, root: nat)
    requires before.frames==after.frames,
    ensures result(b,before,root)==result(b,after,root),
{
    hide(valid_site);hide(closed_frame);hide(linked);
}

} // verus!
