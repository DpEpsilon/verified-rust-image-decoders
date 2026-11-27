//! T10 error-range observations over the unchanged walk primitives.
use vstd::prelude::*;
use crate::{model as h,metadata_model as m,apng_model as a,critical_model as c,
    walk_model::{self as w,Name,Phase},png::Error};
verus! {
pub proof fn framing(b: Seq<u8>,pos: nat)
    ensures match h::frame(b,pos) {Err(e)=>e!=Error::IccProfileTooLarge,_=>true},
{}
pub proof fn header(b: Seq<u8>)
    ensures match h::mh(b) {Err(e)=>e!=Error::IccProfileTooLarge,_=>true},
{
    hide(h::crc);hide(h::be32);
    framing(b,8);
}
pub proof fn keyword(b: Seq<u8>,e: Error)
    ensures match h::split_keyword(b,e) {Err(f)=>f==e,_=>true},
        match m::keyword_method(b,e) {Err(f)=>f==e,_=>true},
{hide(h::first_nul);}
pub proof fn metadata(name: Name,b: Seq<u8>,ct: u8,depth: u8,entries: nat)
    ensures match m::parse(name,b,ct,depth,entries) {m::Outcome::Error(e)=>e!=Error::IccProfileTooLarge,_=>true},
{
    hide(h::first_nul);hide(h::be32);hide(h::be16);
    keyword(b,Error::InvalidIccp);keyword(b,Error::InvalidSplt);keyword(b,Error::InvalidText);
}
pub proof fn palette(length: nat,ct: u8,depth: u8,has_idat: bool,after_plte: bool,ignore: bool,large: bool)
    ensures match m::palette(length,ct,depth,has_idat,after_plte,ignore,large) {
        m::Decision::Error(e)=>e!=Error::IccProfileTooLarge,_=>true},
{}
pub proof fn transparency(length: nat,ct: u8,entries: nat,drop: bool)
    ensures match m::transparency(length,ct,entries,drop) {m::Decision::Error(e)=>e!=Error::IccProfileTooLarge,_=>true},
{}
pub proof fn animation(b: Seq<u8>,present: bool,before: bool,width: u32,height: u32,state: a::Control,declared: Option<u32>)
    ensures match a::actl(b) {Err((e,_))=>e!=Error::IccProfileTooLarge,_=>true},
        match a::fctl(b,present,before,width,height,state) {Err((e,_))=>e!=Error::IccProfileTooLarge,_=>true},
        match a::fdat(b,present,before,state) {Err((e,_))=>e!=Error::IccProfileTooLarge,_=>true},
        match a::finish(state,declared) {Err((e,_))=>e!=Error::IccProfileTooLarge,_=>true},
{hide(h::be32);hide(h::be16);}
pub proof fn critical(b: Seq<u8>,pos: nat,phase: Phase,allow_eof: bool,ct: u8,plte: bool,
    state: a::Control,declared: Option<u32>,next: nat,ignore: bool,ancillary: bool)
    ensures match c::read(b,pos,phase,allow_eof) {Err(e)=>e!=Error::IccProfileTooLarge,_=>true},
        match c::idat(phase,ct,plte) {Err(e)=>e!=Error::IccProfileTooLarge,_=>true},
        match c::iend(phase,state,declared,next,b.len(),ignore) {Err((e,_))=>e!=Error::IccProfileTooLarge,_=>true},
        match c::unknown(ancillary) {Err(e)=>e!=Error::IccProfileTooLarge,_=>true},
{
    hide(h::crc);framing(b,pos);
    animation(b,false,false,0,0,state,declared);
}
pub proof fn crc(wire: Name,name: Name,ancillary: bool,ok: bool,skip: bool,lenient: bool,frames: bool)
    ensures match c::crc_rule(wire,name,ancillary,ok,skip,lenient,frames) {
        c::CrcAction::Fail(e,_)=>e!=Error::IccProfileTooLarge,_=>true},
{}
pub proof fn ordering(seen: Set<Name>,phase: Phase,plte: bool,ct: u8,name: Name,ancillary: bool,length: nat,rules: w::Rules)
    ensures match w::order(seen,phase,plte,ct,name,ancillary,length,rules) {
        w::OrderAction::Fail(e,_)=>e!=Error::IccProfileTooLarge,_=>true},
{}
}
