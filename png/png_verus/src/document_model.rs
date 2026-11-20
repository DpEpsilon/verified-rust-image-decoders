//! Independent complete P2–P8/P14.1 walk. Lists of payload ranges and frames
//! are mathematical sequences, never executable chunk/frame allocations.
//! Defining MW is not a proof that the production walk refines it.
use vstd::prelude::*;
use crate::{model, metadata_model as meta, apng_model as apng, critical_model as critical};
use crate::png::Error;
use crate::walk_model::{self as order, Name, Phase, Range, Frame, Source};

verus! {

pub enum Slot<T> { Absent, Discarded, Present(T) }
pub enum Stored { Unit, Raw(meta::Value), Bytes(Range) }
pub struct Info {
    pub header: model::Ihdr,
    pub palette_entries: nat,
    pub slots: Map<Name,Slot<Stored>>,
    pub num_plays: Option<u32>,
    pub frame_count: nat,
    pub default_is_frame: bool,
}
pub struct Doc {
    pub ihdr: model::Ihdr,
    pub plte: Option<Range>, pub trns: Option<Range>, pub iccp: Option<Range>,
    pub actl: Option<(u32,u32)>,
    pub info: Info,
    pub idat: Seq<Range>, pub frames: Seq<Frame>,
}
/// P16's public Info projection after the constructor validates its default image.
pub open spec fn constructor_info(doc: Doc) -> Info {
    Info {num_plays:match doc.actl {Some((_,plays))=>Some(plays),None=>None},..doc.info}
}
pub struct Events { pub r1: bool, pub r3: bool, pub r4: bool }
/// Only choices used by the structural walk. Pixel/inflate choices have no
/// effect here. The executable policy projection needs its own refinement.
pub struct Policy {
    pub order: order::Rules,
    pub skip_ancillary_crc: bool,
    pub allow_eof_iend: bool,
    pub ignore_trailing: bool,
    pub allow_large_palette: bool,
    pub drop_invalid_trns: bool,
    pub ignore_apng_in_still: bool,
    pub static_apng_errors: bool,
}
pub struct State {
    pub pos: nat, pub seen: Set<Name>, pub phase: Phase,
    pub control: apng::Control, pub doc: Doc, pub events: Events,
}
pub enum Step { Next(State), Done(Doc,Events), Error(Error,Events), Fallback(Events), OutsideDomain }
pub enum Outcome { Done(Doc,Events), Error(Error,Events), Fallback(Events), OutsideDomain }

pub open spec fn metadata_names() -> Set<Name> {
    set![Name::Trns,Name::Gama,Name::Chrm,Name::Srgb,Name::Iccp,Name::Sbit,Name::Cicp,
        Name::Mdcv,Name::Clli,Name::Bkgd,Name::Phys,Name::Exif,Name::Time]
}
pub open spec fn initial_info(h: model::Ihdr) -> Info {
    Info { header:h, palette_entries:0, slots:Map::new(metadata_names(), |n: Name| Slot::Absent),
        num_plays:None, frame_count:0, default_is_frame:false }
}
pub open spec fn initial(h: model::Ihdr, pos: nat, events: Events) -> State {
    State {pos, seen:set![Name::Ihdr], phase:Phase::Before, control:apng::initial(), events,
        doc:Doc {ihdr:h,plte:None,trns:None,iccp:None,actl:None,info:initial_info(h),
            idat:Seq::empty(),frames:Seq::empty()} }
}
pub open spec fn no_events() -> Events { Events {r1:false,r3:false,r4:false} }
pub open spec fn strict() -> Policy {
    Policy {order:order::Rules {first_duplicate_wins:false,drop_misordered:false,drop_invalid:false,
        ignore_unexpected_plte:false,lenient_iend:false}, skip_ancillary_crc:false,allow_eof_iend:false,
        ignore_trailing:false,allow_large_palette:false,drop_invalid_trns:false,
        ignore_apng_in_still:false,static_apng_errors:false}
}
pub open spec fn recommended_beta() -> Policy {
    Policy {order:order::Rules {first_duplicate_wins:true,drop_misordered:true,drop_invalid:true,
        ignore_unexpected_plte:true,lenient_iend:true}, skip_ancillary_crc:true,allow_eof_iend:false,
        ignore_trailing:true,allow_large_palette:true,drop_invalid_trns:false,
        ignore_apng_in_still:true,static_apng_errors:true}
}
pub open spec fn discarded<T>(slot: Slot<T>) -> Slot<T> {
    match slot {Slot::Absent => Slot::Discarded, _ => slot}
}
pub open spec fn discard(state: State, name: Name) -> State {
    let slots = state.doc.info.slots;
    let info = Info {slots:if metadata_names().contains(name) {
        slots.insert(name,discarded(slots[name]))
    } else {slots}, ..state.doc.info};
    let ct=state.doc.ihdr.color_type;
    State {doc:Doc {info,..state.doc},events:Events {
        r1:state.events.r1 || (name==Name::Trns && (ct==0 || ct==2 || ct==3)),..state.events},..state}
}
pub open spec fn failure(state: State, error: Error, structural: bool, static_frames: bool) -> Step {
    if structural && static_frames {Step::Fallback(state.events)} else {Step::Error(error,state.events)}
}
pub open spec fn entries(doc: Doc) -> nat {
    match doc.plte {Some(r)=>r.len/3,None=>0}
}
pub open spec fn stored(name: Name, value: Stored, info: Info) -> Info {
    Info {slots:info.slots.insert(name,Slot::Present(value)),..info}
}
/// Rebase payload-relative descriptors without retaining or copying a payload.
pub open spec fn store(doc: Doc, value: meta::Value, base: nat) -> Doc {
    match value {
        meta::Value::Ignored => doc,
        meta::Value::Iccp {name_len,profile_start,profile_len} => Doc {
            iccp:Some(Range {offset:base+profile_start,len:profile_len}),
            info:stored(Name::Iccp,Stored::Bytes(Range {offset:base,len:name_len}),doc.info),..doc},
        meta::Value::Exif(len) => Doc {
            info:stored(Name::Exif,Stored::Bytes(Range {offset:base,len}),doc.info),..doc},
        _ => {
            let name=match value {
                meta::Value::Gama(_)=>Name::Gama,meta::Value::Chrm(_)=>Name::Chrm,
                meta::Value::Srgb(_)=>Name::Srgb,meta::Value::Sbit(_)=>Name::Sbit,
                meta::Value::Cicp(_)=>Name::Cicp,meta::Value::Mdcv(_)=>Name::Mdcv,
                meta::Value::Clli(_,_)=>Name::Clli,meta::Value::Bkgd(_)=>Name::Bkgd,
                meta::Value::Phys(_,_,_)=>Name::Phys,meta::Value::Time(_,_,_,_,_,_)=>Name::Time,
                _=>Name::Unknown(true),
            };
            Doc {info:stored(name,Stored::Raw(value),doc.info),..doc}
        },
    }
}
#[verifier::opaque]
pub open spec fn own(b: Seq<u8>, policy: Policy, static_frames: bool,
    state: State, name: Name, data: nat, length: nat) -> Step
{
    if data+length>b.len() {Step::OutsideDomain}
    else {
        let d=b.subrange(data as int,(data+length) as int);
        let doc=state.doc;
        let ct=doc.ihdr.color_type;
        match name {
            Name::Ihdr=>Step::Error(Error::DuplicateChunk,state.events),
            Name::Plte=>match meta::palette(length,ct,doc.ihdr.depth,!matches!(state.phase,Phase::Before),
                state.seen.contains(Name::Trns)||state.seen.contains(Name::Bkgd)||state.seen.contains(Name::Hist),
                policy.order.ignore_unexpected_plte,policy.allow_large_palette) {
                meta::Decision::OutsideDomain=>Step::OutsideDomain,
                meta::Decision::Error(e)=>Step::Error(e,state.events),
                meta::Decision::Drop=>Step::Next(state),
                meta::Decision::Keep(count)=>Step::Next(State {doc:Doc {
                    plte:Some(Range {offset:data,len:length}),info:Info {palette_entries:count,..doc.info},..doc},..state}),
            },
            Name::Trns=>match meta::transparency(length,ct,entries(doc),policy.drop_invalid_trns) {
                meta::Decision::OutsideDomain=>Step::OutsideDomain,
                meta::Decision::Error(e)=>Step::Error(e,state.events),
                meta::Decision::Drop=>Step::Next(discard(state,name)),
                meta::Decision::Keep(_)=>Step::Next(State {doc:Doc {
                    trns:Some(Range {offset:data,len:length}),info:stored(Name::Trns,Stored::Unit,doc.info),..doc},..state}),
            },
            Name::Idat=>match critical::idat(state.phase,ct,doc.plte.is_some()) {
                Err(e)=>Step::Error(e,state.events),
                Ok(first)=> {
                    let default_is_frame=doc.info.default_is_frame || (first && !matches!(state.control.current,apng::Current::None));
                    Step::Next(State {phase:Phase::During,
                        control:if first {apng::begin_idat(state.control)} else {state.control},
                        doc:Doc {idat:doc.idat.push(Range {offset:data,len:length}),
                            info:Info {default_is_frame,..doc.info},..doc},..state})
                },
            },
            Name::Iend=>match critical::iend(state.phase,state.control,
                match doc.actl {None=>None,Some(a)=>Some(a.0)},state.pos,b.len(),policy.ignore_trailing) {
                Err((e,s))=>failure(state,e,s,static_frames),
                Ok(())=>Step::Done(Doc {info:Info {frame_count:state.control.frame_count,..doc.info},..doc},state.events),
            },
            Name::Actl=>match apng::actl(d) {
                Err((e,s))=>failure(state,e,s,static_frames),
                Ok(a)=>Step::Next(State {doc:Doc {actl:Some(a),..doc},..state}),
            },
            Name::Fctl=>match apng::fctl(d,doc.actl.is_some(),matches!(state.phase,Phase::Before),
                doc.ihdr.width,doc.ihdr.height,state.control) {
                Err((e,s))=>failure(state,e,s,static_frames),
                Ok((f,c))=>Step::Next(State {control:c,doc:Doc {frames:doc.frames.push(Frame {
                    control_offset:(data-8) as nat,width:f.width,height:f.height,x:f.x,y:f.y,delay_num:f.delay_num,delay_den:f.delay_den,
                    dispose:f.dispose,blend:f.blend,source:if matches!(state.phase,Phase::Before) {
                        Source::Idat
                    } else {Source::Fdat(Seq::empty())}}),..doc},..state}),
            },
            Name::Fdat=>match apng::fdat(d,doc.actl.is_some(),matches!(state.phase,Phase::Before),state.control) {
                Err((e,s))=>failure(state,e,s,static_frames),
                Ok(c)=>if doc.frames.len()==0 {Step::OutsideDomain} else {
                    let i=doc.frames.len()-1;
                    let frame=doc.frames[i];
                    match frame.source {
                        Source::Idat=>Step::OutsideDomain,
                        Source::Fdat(ranges)=>Step::Next(State {control:c,doc:Doc {
                            frames:doc.frames.update(i,Frame {source:Source::Fdat(ranges.push(Range {
                                offset:data+4,len:(length-4) as nat})),..frame}),..doc},..state}),
                    }
                },
            },
            Name::Unknown(anc)=>match critical::unknown(anc) {
                Err(e)=>Step::Error(e,state.events),Ok(())=>Step::Next(state),
            },
            _=>match meta::parse(name,d,ct,doc.ihdr.depth,entries(doc)) {
                meta::Outcome::OutsideDomain=>Step::OutsideDomain,
                meta::Outcome::Error(e)=>if policy.order.drop_invalid {Step::Next(discard(state,name))}
                    else {Step::Error(e,state.events)},
                meta::Outcome::Value(v)=>Step::Next(State {doc:store(doc,v,data),..state}),
            },
        }
    }
}

/// P5 and own-value composition, kept separate from framing/CRC for proof budgets.
#[verifier::opaque]
pub open spec fn ordered(b: Seq<u8>, policy: Policy, static_frames: bool, state: State,
    name: Name, ancillary: bool, data: nat, length: nat) -> Step
{
    match order::order(state.seen,state.phase,state.doc.plte.is_some(),state.doc.ihdr.color_type,name,ancillary,length,policy.order) {
        order::OrderAction::Drop=>Step::Next(discard(state,name)),
        order::OrderAction::Fail(e,s)=>failure(state,e,s,static_frames),
        order::OrderAction::Keep(length)=>own(b,policy,static_frames,State {
            seen:if order::singleton(name) {state.seen.insert(name)} else {state.seen},..state},name,data,length),
    }
}

/// One whole chunk: framing, phase advance, CRC, P5, seen insertion, values.
#[verifier::opaque]
pub open spec fn step(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, state: State) -> Step {
    let static_frames=frames_entry && policy.static_apng_errors;
    match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
        Err(e)=>Step::Error(e,state.events),
        Ok(c)=> {
            let wire=order::classify(c.ty);
            let apng_on=!apng_off && !(!frames_entry && policy.ignore_apng_in_still);
            let (name,anc)=critical::effective(wire,matches!(wire,Name::Unknown(true)) ||
                (c.ty.len()==4 && c.ty[0]&0x20u8!=0),apng_on);
            let next=State {pos:c.next,phase:order::advance_phase(state.phase,wire),..state};
            match critical::crc_rule(wire,name,anc,c.crc_ok,policy.skip_ancillary_crc,policy.order.lenient_iend,frames_entry) {
                critical::CrcAction::Drop=>Step::Next(discard(next,name)),
                critical::CrcAction::Fail(e,s)=>failure(next,e,s,static_frames),
                critical::CrcAction::Keep=>ordered(b,policy,static_frames,next,name,anc,c.data,c.len),
            }
        },
    }
}

/// The progress guard only excludes malformed internal states. The proof of
/// step_progress below rules it out for every Next, including synthetic EOF.
pub open spec fn drive(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, state: State) -> Outcome
    decreases b.len()-state.pos,
{
    if state.pos>b.len() {Outcome::OutsideDomain}
    else {match step(b,policy,frames_entry,apng_off,state) {
        Step::Error(e,ev)=>Outcome::Error(e,ev),Step::Fallback(ev)=>Outcome::Fallback(ev),
        Step::Done(doc,ev)=>Outcome::Done(doc,ev),Step::OutsideDomain=>Outcome::OutsideDomain,
        Step::Next(next)=>if state.pos<next.pos && next.pos<=b.len() {
            drive(b,policy,frames_entry,apng_off,next)
        } else {Outcome::OutsideDomain},
    }}
}
pub open spec fn start(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, events: Events) -> Outcome {
    match model::mh(b) {Err(e)=>Outcome::Error(e,events),Ok((h,p))=>drive(b,policy,frames_entry,apng_off,initial(h,p,events))}
}
#[allow(non_snake_case)]
pub open spec fn MW_p(b: Seq<u8>, policy: Policy) -> Outcome {start(b,policy,false,false,no_events())}
#[allow(non_snake_case)]
pub open spec fn MW(b: Seq<u8>) -> Outcome {MW_p(b,strict())}
#[allow(non_snake_case)]
pub open spec fn MWF_p(b: Seq<u8>, policy: Policy) -> Outcome {
    match start(b,policy,true,false,no_events()) {
        Outcome::Fallback(ev)=>match start(b,policy,true,true,Events {r4:true,..ev}) {
            Outcome::Fallback(ev)=>Outcome::Error(Error::MissingAnimationControl,ev),other=>other,
        },other=>other,
    }
}

pub open spec fn range_ok(b: Seq<u8>, range: Range) -> bool {range.offset+range.len<=b.len()}

/// None is the internal OutsideDomain marker for an out-of-bounds descriptor.
/// Empty ranges remain in the trace, including an empty fdAT data field.
pub open spec fn concat_ranges(b: Seq<u8>, ranges: Seq<Range>) -> Option<Seq<u8>>
    decreases ranges.len(),
{
    if ranges.len()==0 {Some(Seq::empty())}
    else {
        let r=ranges[0];
        if !range_ok(b,r) {None} else {
            match concat_ranges(b,ranges.drop_first()) {
                None=>None,Some(tail)=>Some(b.subrange(r.offset as int,(r.offset+r.len) as int)+tail),
            }
        }
    }
}
pub open spec fn default_stream(b: Seq<u8>, doc: Doc) -> Option<Seq<u8>> {concat_ranges(b,doc.idat)}
pub open spec fn frame_stream(b: Seq<u8>, doc: Doc, index: nat) -> Option<Seq<u8>> {
    if index>=doc.frames.len() {None} else {match doc.frames[index as int].source {
        Source::Idat=>default_stream(b,doc),Source::Fdat(ranges)=>concat_ranges(b,ranges),
    }}
}

pub proof fn drive_step(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, state: State)
    requires state.pos<=b.len(),
    ensures drive(b,policy,frames_entry,apng_off,state)==match step(b,policy,frames_entry,apng_off,state) {
        Step::Next(next)=>drive(b,policy,frames_entry,apng_off,next),Step::Error(e,ev)=>Outcome::Error(e,ev),
        Step::Fallback(ev)=>Outcome::Fallback(ev),Step::Done(doc,ev)=>Outcome::Done(doc,ev),Step::OutsideDomain=>Outcome::OutsideDomain,
    },
{
    step_progress(b,policy,frames_entry,apng_off,state);
}

pub proof fn own_position(b: Seq<u8>, policy: Policy, static_frames: bool,
    state: State, name: Name, data: nat, length: nat)
    ensures match own(b,policy,static_frames,state,name,data,length) {
        Step::Next(next)=>next.pos==state.pos,_=>true,
    },
{
    reveal(own);
}

pub proof fn eof_terminal(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, state: State)
    requires state.pos==b.len(),
    ensures !matches!(step(b,policy,frames_entry,apng_off,state),Step::Next(_)),
{
    reveal(step);
    reveal(ordered);
    reveal(own);
}

pub proof fn ordered_position(b: Seq<u8>, policy: Policy, static_frames: bool, state: State,
    name: Name, ancillary: bool, data: nat, length: nat)
    ensures match ordered(b,policy,static_frames,state,name,ancillary,data,length) {
        Step::Next(next)=>next.pos==state.pos,_=>true,
    },
{
    reveal(ordered);
    match order::order(state.seen,state.phase,state.doc.plte.is_some(),state.doc.ihdr.color_type,
        name,ancillary,length,policy.order) {
        order::OrderAction::Keep(length)=>own_position(b,policy,static_frames,
            State {seen:if order::singleton(name) {state.seen.insert(name)}else{state.seen},..state},name,data,length),
        _=>{},
    }
}

pub proof fn step_progress(b: Seq<u8>, policy: Policy, frames_entry: bool, apng_off: bool, state: State)
    ensures match step(b,policy,frames_entry,apng_off,state) {
        Step::Next(next)=>state.pos+12<=next.pos && next.pos<=b.len(),_=>true,
    },
{
    if state.pos==b.len() {eof_terminal(b,policy,frames_entry,apng_off,state);}
    else {
        reveal(step);
        match critical::read(b,state.pos,state.phase,policy.allow_eof_iend) {
            Err(_)=>{},
            Ok(c)=> {
                let wire=order::classify(c.ty);
                let (name,anc)=critical::effective(wire,matches!(wire,Name::Unknown(true)) ||
                    (c.ty.len()==4 && c.ty[0]&0x20u8!=0),!apng_off && !(!frames_entry && policy.ignore_apng_in_still));
                let next=State {pos:c.next,phase:order::advance_phase(state.phase,wire),..state};
                ordered_position(b,policy,frames_entry && policy.static_apng_errors,next,name,anc,c.data,c.len);
            },
        }
    }
}

} // verus!
