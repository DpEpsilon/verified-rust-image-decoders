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

/// These facts concern the independent P17 model, not an allocation trace.
pub open spec fn total(b: Seq<u8>) -> bool {
    !matches!(MI(b),Outcome::OutsideDomain)
}
pub open spec fn walk_context(b: Seq<u8>) -> bool {
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Done(doc,_)=>match doc.iccp {
            Some(r)=>r.offset+r.len<=b.len() && stream(b,doc).len()<=crate::limits::MAX_INPUT
                && !matches!(measured(stream(b,doc)).status,z::InflateStatus::OutsideDomain)
                && measured(stream(b,doc)).out.len()<=crate::limits::MAX_ICC,
            None=>true,
        },
        walk::Outcome::Error(_,_)=>true,
        _=>false,
    }
}
pub open spec fn resources(value: Outcome) -> bool {
    match value {Outcome::Profile(bytes)=>bytes.len()<=crate::limits::MAX_ICC,_=>true}
}
pub open spec fn request_bounds(b: Seq<u8>) -> bool {
    requests(b).len()<=2 && forall|j: int| 0<=j<requests(b).len()
        ==> site_bound((#[trigger] requests(b)[j]).0,requests(b)[j].1)
}
/// Walk failure and profile absence precede any measuring reservation.
pub proof fn model_prefix(b: Seq<u8>)
    ensures match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Error(e,_)=>MI(b)==Outcome::Error(e) && requests(b)==Seq::<(AllocSite,nat)>::empty(),
        walk::Outcome::Done(doc,_)=>doc.iccp.is_none()
            ==> MI(b)==Outcome::Absent && requests(b)==Seq::<(AllocSite,nat)>::empty(),
        _=>true,
    },
{}
/// Full is decided by the bounded measuring pass, before any Icc request.
pub proof fn model_payload(b: Seq<u8>,doc: walk::Doc,ev: walk::Events)
    requires walk::MW_p(b,walk::strict())==walk::Outcome::Done(doc,ev),doc.iccp.is_some(),
    ensures match measured(stream(b,doc)).status {
        z::InflateStatus::Done=>MI(b)==Outcome::Profile(measured(stream(b,doc)).out)
            && requests(b)==seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat),
                (AllocSite::Icc,measured(stream(b,doc)).out.len())],
        z::InflateStatus::Full=>MI(b)==Outcome::Error(Error::IccProfileTooLarge)
            && requests(b)==seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat)],
        z::InflateStatus::Error(e)=>MI(b)==Outcome::Error(fault(e))
            && requests(b)==seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat)],
        z::InflateStatus::OutsideDomain=>MI(b)==Outcome::OutsideDomain,
    },
{}
/// A zlib format fault is never the ICC output-limit result.
pub proof fn fault_not_profile_cap(e: z::InflateFault)
    ensures fault(e)!=Error::IccProfileTooLarge,
{}
pub proof fn model_cap_precedence(b: Seq<u8>,doc: walk::Doc,ev: walk::Events)
    requires walk::MW_p(b,walk::strict())==walk::Outcome::Done(doc,ev),doc.iccp.is_some(),
    ensures (MI(b)==Outcome::Error(Error::IccProfileTooLarge))
        == (measured(stream(b,doc)).status==z::InflateStatus::Full),
{
    if let z::InflateStatus::Error(e)=measured(stream(b,doc)).status {fault_not_profile_cap(e);}
}
pub proof fn model_resources(b: Seq<u8>)
    requires walk_context(b),
    ensures total(b),resources(MI(b)),request_bounds(b),
{
    hide(z::Inflate);hide(walk::MW_p);
    crate::limits::caps_ok();
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Done(doc,_)=>{
            assert forall|j: int| 0<=j<requests(b).len() implies
                site_bound((#[trigger] requests(b)[j]).0,requests(b)[j].1) by {
                if j==0 {}else{assert(j==1);}
            }
        },_=>{},
    }
}
/// Instantiate accepted M2 L-LIMIT-MONO at the exact successful count m.
/// No native writer, allocation-success premise or new inflate proof appears.
pub proof fn exact_second_pass(bytes: Seq<u8>)
    requires measured(bytes).status==z::InflateStatus::Done,
        measured(bytes).out.len()<=crate::limits::MAX_ICC,
    ensures z::Inflate(bytes,measured(bytes).out.len(),true,false).status==z::InflateStatus::Done,
        z::Inflate(bytes,measured(bytes).out.len(),true,false).out==measured(bytes).out,
{
    inflate_verus::limit_mono::lemma_limit_mono(bytes,crate::limits::MAX_ICC as nat,measured(bytes).out.len(),true,false);
}
/// Allocation-free totality certificate, separate from the model definition.
/// The actual Strict walk and M2 measurement use one fixed stack window;
/// neither this certificate nor MI calls the public ICC allocator pipeline.
#[verifier::spinoff_prover]
pub fn model_totality(b: &[u8])
    ensures total(b@),walk_context(b@),resources(MI(b@)),request_bounds(b@),
{
    hide(walk::MW_p);hide(z::Inflate);
    proof {crate::walk_policy::profiles_project_exactly();crate::limits::caps_ok();}
    let mut ev=crate::png::Events {r1:false,r3:false,r4:false};
    let result=crate::callers::still(b,&crate::policy::STRICT,&mut ev);
    match result {
        Err(_)=>{},
        Ok(doc)=>{
            if let Some((o,n))=doc.iccp {
                let bytes=&b[o..o+n];
                let parts: [&[u8];1]=[bytes];
                proof {
                    assert(inflate_verus::segments::parts_prefix(parts@,0)==Seq::<u8>::empty());
                    assert(inflate_verus::segments::parts_prefix(parts@,1)==bytes@);
                }
                let mut source=match inflate_verus::SegmentSlices::new(&parts) {
                    Some(s)=>s,None=>{assert(false);return;},
                };
                let mut window=[0u8;inflate_verus::limits::WINDOW_MAX];
                proof {
                    vstd::array::array_len_matches_n(&window);
                    assert(inflate_verus::limits::WINDOW_MAX==32768usize) by (compute);
                }
                let ring=vstd::array::ref_mut_array_unsizing_coercion(&mut window);
                let measured_result=inflate_verus::inflate_zlib_measure(&mut source,ring,crate::limits::MAX_ICC,inflate_verus::InflateOptions::STRICT);
                proof {
                    let bytes_view=bytes@;
                    assert(bytes_view==stream(b@,doc.view()));
                    assert(inflate_verus::status_eq(measured_result.status,measured(bytes_view).status));
                    assert(!matches!(measured(bytes_view).status,z::InflateStatus::OutsideDomain));
                    assert(measured(bytes_view).out.len()<=crate::limits::MAX_ICC);
                }
            }
        },
    }
    proof {model_resources(b@);}
}

} // verus!
