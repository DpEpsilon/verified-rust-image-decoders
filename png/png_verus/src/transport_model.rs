//! Transport from the independent structural walk's IDAT range trace to the
//! lengths-and-types reader. Byte equality and segment count are separate:
//! empty data fields still contribute one segment.
use vstd::prelude::*;
use crate::{document_model as m,model,walk_model::{self as names,Name,Phase,Range},critical_model as critical};

verus! {

pub open spec fn trace_bytes(b: Seq<u8>, ranges: Seq<Range>) -> Seq<u8> {
    match m::concat_ranges(b,ranges) {Some(bytes)=>bytes,None=>Seq::empty()}
}
pub open spec fn valid_trace(b: Seq<u8>, ranges: Seq<Range>) -> bool {
    m::concat_ranges(b,ranges).is_some()
}
pub open spec fn first_cursor(ranges: Seq<Range>) -> nat {
    if ranges.len()==0 {0} else {(ranges[0].offset-8) as nat}
}
pub open spec fn idat_result(b: Seq<u8>, doc: m::Doc) -> bool {
    doc.idat.len()>0 && doc.idat[0].offset>=8 && valid_trace(b,doc.idat)
    && model::reader_bytes(b,first_cursor(doc.idat),false)==trace_bytes(b,doc.idat)
    && model::reader_count(b,first_cursor(doc.idat),false)==doc.idat.len()
}
pub open spec fn idat_invariant(b: Seq<u8>, state: m::State) -> bool {
    if matches!(state.phase,Phase::Before) {state.doc.idat.len()==0}
    else {
        let ranges=state.doc.idat;
        ranges.len()>0 && ranges[0].offset>=8 && valid_trace(b,ranges)
        && if matches!(state.phase,Phase::During) {
            model::reader_bytes(b,first_cursor(ranges),false)
                ==trace_bytes(b,ranges)+model::reader_bytes(b,state.pos,false)
            && model::reader_count(b,first_cursor(ranges),false)
                ==ranges.len()+model::reader_count(b,state.pos,false)
        }else{idat_result(b,state.doc)}
    }
}

pub proof fn concat_append(b: Seq<u8>, ranges: Seq<Range>, range: Range)
    requires valid_trace(b,ranges),range.offset+range.len<=b.len(),
    ensures m::concat_ranges(b,ranges.push(range))==Some(trace_bytes(b,ranges)
        +b.subrange(range.offset as int,(range.offset+range.len) as int)),
    decreases ranges.len(),
{
    reveal_with_fuel(m::concat_ranges,2);
    let payload=b.subrange(range.offset as int,(range.offset+range.len) as int);
    if ranges.len()>0 {
        concat_append(b,ranges.drop_first(),range);
        assert(ranges.push(range).drop_first()==ranges.drop_first().push(range));
        let head=b.subrange(ranges[0].offset as int,(ranges[0].offset+ranges[0].len) as int);
        let tail=trace_bytes(b,ranges.drop_first());
        assert(head+(tail+payload) =~= (head+tail)+payload);
    }else{
        assert(ranges==Seq::<Range>::empty());
        assert(ranges.push(range)==seq![range]);
    }
}

/// A valid candidate either contributes one IDAT segment, or stops that
/// reader immediately. A synthetic IEND at exact EOF also stops the reader.
pub proof fn read_idat(b: Seq<u8>, pos: nat, phase: Phase, allow_eof: bool)
    ensures match critical::read(b,pos,phase,allow_eof) {
        Err(_)=>true,Ok(c)=> {
            if names::classify(c.ty)==Name::Idat {
                c.data==pos+8 && c.data+c.len<=b.len() && c.next<=b.len()
                && model::reader_bytes(b,pos,false)==b.subrange(c.data as int,(c.data+c.len) as int)
                    +model::reader_bytes(b,c.next,false)
                && model::reader_count(b,pos,false)==1+model::reader_count(b,c.next,false)
            }else{model::reader_bytes(b,pos,false)==Seq::<u8>::empty() && model::reader_count(b,pos,false)==0}
        },
    },
{}

/// Metadata/control transitions do not silently change the IDAT trace.
pub proof fn own_idat_effect(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, data: nat, length: nat)
    ensures match m::own(b,policy,static_frames,state,name,data,length) {
        m::Step::Next(next)=>if name==Name::Idat {
            !matches!(state.phase,Phase::After) && matches!(next.phase,Phase::During)
            && next.doc.idat==state.doc.idat.push(Range {offset:data,len:length})
        }else{next.phase==state.phase && next.doc.idat==state.doc.idat},
        m::Step::Done(doc,_)=>name==Name::Iend && !matches!(state.phase,Phase::Before) && doc.idat==state.doc.idat,
        _=>true,
    },
{
    reveal(m::own);
}

pub proof fn ordered_idat_effect(b: Seq<u8>, policy: m::Policy, static_frames: bool, state: m::State,
    name: Name, ancillary: bool, data: nat, length: nat)
    ensures match m::ordered(b,policy,static_frames,state,name,ancillary,data,length) {
        m::Step::Next(next)=>if name==Name::Idat {
            !matches!(state.phase,Phase::After) && matches!(next.phase,Phase::During)
            && next.doc.idat==state.doc.idat.push(Range {offset:data,len:length})
        }else{next.phase==state.phase && next.doc.idat==state.doc.idat},
        m::Step::Done(doc,_)=>name==Name::Iend && !matches!(state.phase,Phase::Before) && doc.idat==state.doc.idat,
        _=>true,
    },
{
    reveal(m::ordered);
    match names::order(state.seen,state.phase,state.doc.plte.is_some(),state.doc.ihdr.color_type,
        name,ancillary,length,policy.order) {
        names::OrderAction::Keep(len)=>own_idat_effect(b,policy,static_frames,m::State {
            seen:if names::singleton(name) {state.seen.insert(name)}else{state.seen},..state},name,data,len),
        _=>{},
    }
}

pub proof fn stopped(b: Seq<u8>, state: m::State, next: m::State, wire: Name)
    requires idat_invariant(b,state),wire!=Name::Idat,next.doc.idat==state.doc.idat,
        next.phase==names::advance_phase(state.phase,wire),
        model::reader_bytes(b,state.pos,false)==Seq::<u8>::empty(),model::reader_count(b,state.pos,false)==0,
    ensures idat_invariant(b,next),
{}

pub proof fn appended(b: Seq<u8>, state: m::State, next: m::State, data: nat, length: nat)
    requires idat_invariant(b,state),!matches!(state.phase,Phase::After),
        matches!(next.phase,Phase::During),next.doc.idat==state.doc.idat.push(Range {offset:data,len:length}),
        data==state.pos+8,data+length<=b.len(),
        model::reader_bytes(b,state.pos,false)==b.subrange(data as int,(data+length) as int)+model::reader_bytes(b,next.pos,false),
        model::reader_count(b,state.pos,false)==1+model::reader_count(b,next.pos,false),
    ensures idat_invariant(b,next),
{
    concat_append(b,state.doc.idat,Range {offset:data,len:length});
    assert(valid_trace(b,next.doc.idat));
    assert(trace_bytes(b,next.doc.idat)==trace_bytes(b,state.doc.idat)+b.subrange(data as int,(data+length) as int));
    if state.doc.idat.len()==0 {
        assert(first_cursor(next.doc.idat)==state.pos);
        assert(trace_bytes(b,state.doc.idat)==Seq::<u8>::empty());
    }else{
        assert(first_cursor(next.doc.idat)==first_cursor(state.doc.idat));
        let prefix=trace_bytes(b,state.doc.idat);
        let payload=b.subrange(data as int,(data+length) as int);
        let tail=model::reader_bytes(b,next.pos,false);
        assert(prefix+(payload+tail) =~= (prefix+payload)+tail);
    }
}

pub proof fn step_idat(b: Seq<u8>, policy: m::Policy, frames_entry: bool, apng_off: bool, state: m::State)
    requires idat_invariant(b,state),
    ensures match m::step(b,policy,frames_entry,apng_off,state) {
        m::Step::Next(next)=>idat_invariant(b,next),m::Step::Done(doc,_)=>idat_result(b,doc),_=>true,
    },
{
    reveal(m::step);
    read_idat(b,state.pos,state.phase,policy.allow_eof_iend);
    match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(_)=>{},Ok(c)=> {
            let wire=names::classify(c.ty);
            let (name,anc)=critical::effective(wire,matches!(wire,Name::Unknown(true)) ||
                (c.ty.len()==4 && c.ty[0]&0x20u8!=0),!apng_off && !(!frames_entry && policy.ignore_apng_in_still));
            let advanced=m::State {pos:c.next,phase:names::advance_phase(state.phase,wire),..state};
            assert((name==Name::Idat)==(wire==Name::Idat));
            if wire==Name::Idat {
                assert(c.ty==seq![b'I',b'D',b'A',b'T']);
                assert(b'I'&0x20u8==0u8) by (bit_vector);
                assert(!anc);
                assert(!matches!(critical::crc_rule(wire,name,anc,c.crc_ok,policy.skip_ancillary_crc,
                    policy.order.lenient_iend,frames_entry),critical::CrcAction::Drop));
            }
            ordered_idat_effect(b,policy,frames_entry && policy.static_apng_errors,advanced,name,anc,c.data,c.len);
            m::ordered_position(b,policy,frames_entry && policy.static_apng_errors,advanced,name,anc,c.data,c.len);
            match m::step(b,policy,frames_entry,apng_off,state) {
                m::Step::Next(next)=> {
                    if wire==Name::Idat {appended(b,state,next,c.data,c.len);}
                    else{stopped(b,state,next,wire);}
                },
                m::Step::Done(doc,_)=> {
                    assert(wire==Name::Iend);
                    stopped(b,state,advanced,wire);
                },_=>{},
            }
        },
    }
}

pub proof fn drive_idat(b: Seq<u8>, policy: m::Policy, frames_entry: bool, apng_off: bool, state: m::State)
    requires idat_invariant(b,state),state.pos<=b.len(),
    ensures match m::drive(b,policy,frames_entry,apng_off,state) {
        m::Outcome::Done(doc,_)=>idat_result(b,doc),_=>true,
    },
    decreases b.len()-state.pos,
{
    m::drive_step(b,policy,frames_entry,apng_off,state);
    m::step_progress(b,policy,frames_entry,apng_off,state);
    step_idat(b,policy,frames_entry,apng_off,state);
    match m::step(b,policy,frames_entry,apng_off,state) {
        m::Step::Next(next)=>drive_idat(b,policy,frames_entry,apng_off,next),_=>{},
    }
}

pub proof fn start_idat(b: Seq<u8>, policy: m::Policy, frames_entry: bool, apng_off: bool, events: m::Events)
    ensures match m::start(b,policy,frames_entry,apng_off,events) {
        m::Outcome::Done(doc,_)=>idat_result(b,doc) && b.len()<=crate::limits::MAX_INPUT,_=>true,
    },
{
    match model::mh(b) {
        Err(_)=>{},Ok((h,pos))=> {
            assert(pos<=b.len());
            drive_idat(b,policy,frames_entry,apng_off,m::initial(h,pos,events));
        },
    }
}

} // verus!
