//! P17 ICC mathematics: Strict walk, bounded inflate, and allocation equations.
use vstd::prelude::*;
use crate::{document_model as walk,png::Error,alloc_guard::{AllocSite,sites::{TryError,log_tied}}};
use inflate_verus::{InflateError,model as z};

verus! {

pub open spec fn inflate_error(e: InflateError) -> Error {
    match e {
        InflateError::TruncatedZlib=>Error::TruncatedZlib,
        InflateError::InvalidZlibHeader=>Error::InvalidZlibHeader,
        InflateError::UnsupportedZlibMethod=>Error::UnsupportedZlibMethod,
        InflateError::ZlibPresetDictionary=>Error::ZlibPresetDictionary,
        InflateError::InvalidBlockType=>Error::InvalidBlockType,
        InflateError::StoredLengthMismatch=>Error::StoredLengthMismatch,
        InflateError::InvalidCodeLengths=>Error::InvalidCodeLengths,
        InflateError::InvalidSymbol=>Error::InvalidSymbol,
        InflateError::DistanceTooFar=>Error::DistanceTooFar,
        InflateError::AdlerMismatch=>Error::AdlerMismatch,
        InflateError::TrailingZlibData=>Error::TrailingZlibData,
    }
}
pub open spec fn fault(e: z::InflateFault) -> Error {
    match e {
        z::InflateFault::TruncatedZlib=>Error::TruncatedZlib,
        z::InflateFault::InvalidZlibHeader=>Error::InvalidZlibHeader,
        z::InflateFault::UnsupportedZlibMethod=>Error::UnsupportedZlibMethod,
        z::InflateFault::ZlibPresetDictionary=>Error::ZlibPresetDictionary,
        z::InflateFault::InvalidBlockType=>Error::InvalidBlockType,
        z::InflateFault::StoredLengthMismatch=>Error::StoredLengthMismatch,
        z::InflateFault::InvalidCodeLengths=>Error::InvalidCodeLengths,
        z::InflateFault::InvalidSymbol=>Error::InvalidSymbol,
        z::InflateFault::DistanceTooFar=>Error::DistanceTooFar,
        z::InflateFault::AdlerMismatch=>Error::AdlerMismatch,
        z::InflateFault::TrailingZlibData=>Error::TrailingZlibData,
    }
}
pub proof fn fault_agrees(e: InflateError,f: z::InflateFault)
    requires inflate_verus::fault_eq(f,e),
    ensures inflate_error(e)==fault(f),
{}

pub open spec fn measured(bytes: Seq<u8>) -> z::Inflated {
    z::Inflate(bytes,crate::limits::MAX_ICC as nat,true,false)
}
pub enum Outcome {Error(Error),Absent,Profile(Seq<u8>),OutsideDomain}
pub open spec fn data(bytes: Seq<u8>) -> Outcome {
    let r=measured(bytes);
    match r.status {
        z::InflateStatus::Done=>Outcome::Profile(r.out),
        z::InflateStatus::Full=>Outcome::Error(Error::IccProfileTooLarge),
        z::InflateStatus::Error(e)=>Outcome::Error(fault(e)),
        z::InflateStatus::OutsideDomain=>Outcome::OutsideDomain,
    }
}
pub open spec fn stream(b: Seq<u8>,doc: walk::Doc) -> Seq<u8> {
    match doc.iccp {Some(r)=>b.subrange(r.offset as int,(r.offset+r.len) as int),None=>Seq::empty()}
}
pub open spec fn MI(b: Seq<u8>) -> Outcome {
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Error(e,_)=>Outcome::Error(e),
        walk::Outcome::Done(doc,_)=>if doc.iccp.is_none() {Outcome::Absent}else{data(stream(b,doc))},
        _=>Outcome::OutsideDomain,
    }
}
pub open spec fn data_requests(bytes: Seq<u8>) -> Seq<(AllocSite,nat)> {
    let first=seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat)];
    if measured(bytes).status==z::InflateStatus::Done {
        first.push((AllocSite::Icc,measured(bytes).out.len()))
    }else{first}
}
pub open spec fn requests(b: Seq<u8>) -> Seq<(AllocSite,nat)> {
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Done(doc,_)=>if doc.iccp.is_none() {Seq::empty()}else{data_requests(stream(b,doc))},
        _=>Seq::empty(),
    }
}
pub open spec fn result_view(result: Result<Option<alloc::vec::Vec<u8>>,TryError>) -> Result<Option<Seq<u8>>,TryError> {
    match result {Ok(Some(v))=>Ok(Some(v@)),Ok(None)=>Ok(None),Err(e)=>Err(e)}
}
pub open spec fn eq(result: Result<Option<Seq<u8>>,TryError>,expected: Outcome) -> bool {
    match result {
        Ok(None)=>expected==Outcome::Absent,
        Ok(Some(bytes))=>expected==Outcome::Profile(bytes),
        Err(TryError::Format(e))=>expected==Outcome::Error(e),
        Err(TryError::OutOfMemory(_))=>false,
    }
}
/// G result/log/cap clauses and Prog, with the actual sealed log.
pub open spec fn site_bound(site: AllocSite,n: nat) -> bool {
    n<=crate::alloc_guard::MAX_BYTES && match site {
        AllocSite::IccWindow=>n==inflate_verus::limits::WINDOW_MAX,
        AllocSite::Icc=>n<=crate::limits::MAX_ICC,
        _=>false,
    }
}
pub open spec fn complete(b: Seq<u8>,result: Result<Option<Seq<u8>>,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    log_tied(result,log) && crate::alloc_guard::req_of_log(log).is_prefix_of(requests(b))
    && (forall|i: int| 0<=i<log.len() ==> site_bound((#[trigger] log[i]).0,log[i].1))
    && match result {
        Err(TryError::OutOfMemory(o))=>o.elements_spec()>0,
        _=>eq(result,MI(b)) && crate::alloc_guard::req_of_log(log)==requests(b),
    }
}

/// The fixed-size call log has at most two ICC requests.
pub proof fn short_log(log: Seq<(AllocSite,nat,bool)>)
    requires log.len()<=2,
    ensures crate::alloc_guard::req_of_log(log)==if log.len()==0 {Seq::empty()}
        else if log.len()==1 {seq![(log[0].0,log[0].1)]}
        else {seq![(log[0].0,log[0].1),(log[1].0,log[1].1)]},
{
    let expected=if log.len()==0 {Seq::empty()}
        else if log.len()==1 {seq![(log[0].0,log[0].1)]}
        else {seq![(log[0].0,log[0].1),(log[1].0,log[1].1)]};
    assert(crate::alloc_guard::req_of_log(log) =~= expected);
}

pub proof fn prefix_exit(b: Seq<u8>,prefix: Result<Option<walk::Doc>,TryError>,ev: walk::Events,log: Seq<(AllocSite,nat,bool)>)
    requires crate::allocation_model::icc_prepared(b,prefix,ev,log),
    ensures match prefix {Err(e)=>complete(b,Err(e),log),Ok(None)=>complete(b,Ok(None),log),_=>true},
{
    hide(z::Inflate);hide(walk::MW_p);
    crate::limits::caps_ok();
    assert(0<inflate_verus::limits::WINDOW_MAX) by (bit_vector);
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Error(_,_)=>{short_log(log);},
        walk::Outcome::Done(doc,_)=>{
            if doc.iccp.is_none() {short_log(log);}
            else if prefix.is_err() {
                short_log(log);
                let req=requests(b);
                assert(crate::alloc_guard::req_of_log(log) =~= req.take(1));
            }
        },_=>{},
    }
}

/// PreAlloc for the complete ICC entry, not only its preparation.
pub proof fn walk_error_first(b: Seq<u8>,result: Result<Option<Seq<u8>>,TryError>,
    log: Seq<(AllocSite,nat,bool)>,e: Error,ev: walk::Events)
    requires complete(b,result,log),walk::MW_p(b,walk::strict())==walk::Outcome::Error(e,ev),
    ensures result==Err(TryError::Format(e)),log==Seq::<(AllocSite,nat,bool)>::empty(),
{
    hide(z::Inflate);hide(walk::MW_p);
    assert(log.len()==0);
    assert(log =~= Seq::empty());
}

pub proof fn oom_requires_profile(b: Seq<u8>,result: Result<Option<Seq<u8>>,TryError>,log: Seq<(AllocSite,nat,bool)>)
    requires complete(b,result,log),matches!(result,Err(TryError::OutOfMemory(_))),
    ensures match walk::MW_p(b,walk::strict()) {walk::Outcome::Done(doc,_)=>doc.iccp.is_some(),_=>false},
{
    hide(z::Inflate);hide(walk::MW_p);
}

pub proof fn absent_profile(b: Seq<u8>,result: Result<Option<Seq<u8>>,TryError>,
    log: Seq<(AllocSite,nat,bool)>,doc: walk::Doc,ev: walk::Events)
    requires complete(b,result,log),walk::MW_p(b,walk::strict())==walk::Outcome::Done(doc,ev),doc.iccp.is_none(),
    ensures result==Ok(None),log==Seq::<(AllocSite,nat,bool)>::empty(),
{
    hide(z::Inflate);hide(walk::MW_p);
    assert(log.len()==0);
    assert(log =~= Seq::empty());
}

} // verus!
