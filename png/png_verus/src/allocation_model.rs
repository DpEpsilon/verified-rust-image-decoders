//! Independent allocation-prefix composition over the mathematical walks.
//! Pixel/default-data and ICC inflation are deliberately outside this prefix.
use vstd::prelude::*;
use crate::{document_model as walk,layout_model as layout,alloc_guard::{AllocSite,sites::{TryError,log_tied}}};

verus! {

pub open spec fn image_walk(b: Seq<u8>,pol: walk::Policy,frames: bool) -> walk::Outcome {
    if frames {walk::MWF_p(b,pol)}else{walk::MW_p(b,pol)}
}
pub open spec fn image_requests(doc: walk::Doc,wide: bool) -> Seq<(AllocSite,nat)> {
    seq![(AllocSite::Raw,layout::raw(doc.ihdr.interlace,layout::bits(doc.ihdr),doc.ihdr.width as nat,doc.ihdr.height as nat)),
        (AllocSite::Output,layout::output_count(doc.ihdr,wide))]
}
pub open spec fn image_prepared(b: Seq<u8>,pol: walk::Policy,frames: bool,wide: bool,
    result: Result<walk::Doc,TryError>,events: walk::Events,log: Seq<(AllocSite,nat,bool)>) -> bool {
    match image_walk(b,pol,frames) {
        walk::Outcome::Error(e,ev)=>result==Err(TryError::Format(e)) && events==ev && log.len()==0,
        walk::Outcome::Done(doc,ev)=>events==ev && match result {
            Ok(got)=>got==doc && log==seq![(AllocSite::Raw,image_requests(doc,wide)[0].1,true),
                (AllocSite::Output,image_requests(doc,wide)[1].1,true)],
            Err(TryError::OutOfMemory(_))=>log_tied(result,log)
                && crate::alloc_guard::req_of_log(log).is_prefix_of(image_requests(doc,wide))
                && 1<=log.len()<=2,
            Err(TryError::Format(_))=>false,
        },
        _=>false,
    }
}

pub open spec fn icc_prepared(b: Seq<u8>,result: Result<Option<walk::Doc>,TryError>,
    events: walk::Events,log: Seq<(AllocSite,nat,bool)>) -> bool {
    match walk::MW_p(b,walk::strict()) {
        walk::Outcome::Error(e,ev)=>result==Err(TryError::Format(e)) && events==ev && log.len()==0,
        walk::Outcome::Done(doc,ev)=>events==ev && if doc.iccp.is_none() {result==Ok(None) && log.len()==0}
            else {match result {
                Ok(Some(got))=>got==doc && log==seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat,true)],
                Err(TryError::OutOfMemory(_))=>log_tied(result,log)
                    && log==seq![(AllocSite::IccWindow,inflate_verus::limits::WINDOW_MAX as nat,false)],
                _=>false,
            }},
        _=>false,
    }
}

/// Exact PreAlloc consequence of the image-prefix relation.
pub proof fn image_error_first(b: Seq<u8>,pol: walk::Policy,frames: bool,wide: bool,
    result: Result<walk::Doc,TryError>,events: walk::Events,log: Seq<(AllocSite,nat,bool)>,e: crate::png::Error,ev: walk::Events)
    requires image_prepared(b,pol,frames,wide,result,events,log),image_walk(b,pol,frames)==walk::Outcome::Error(e,ev),
    ensures result==Err(TryError::Format(e)),events==ev,log.len()==0,
{}

pub proof fn icc_error_first(b: Seq<u8>,result: Result<Option<walk::Doc>,TryError>,
    events: walk::Events,log: Seq<(AllocSite,nat,bool)>,e: crate::png::Error,ev: walk::Events)
    requires icc_prepared(b,result,events,log),walk::MW_p(b,walk::strict())==walk::Outcome::Error(e,ev),
    ensures result==Err(TryError::Format(e)),events==ev,log.len()==0,
{}

pub proof fn icc_absent(b: Seq<u8>,result: Result<Option<walk::Doc>,TryError>,events: walk::Events,
    log: Seq<(AllocSite,nat,bool)>,doc: walk::Doc,ev: walk::Events)
    requires icc_prepared(b,result,events,log),walk::MW_p(b,walk::strict())==walk::Outcome::Done(doc,ev),doc.iccp.is_none(),
    ensures result==Ok(None),events==ev,log.len()==0,
{}

} // verus!
