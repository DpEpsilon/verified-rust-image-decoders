//! T10 applications to real logged public entries and an arbitrary reachable step.
use vstd::prelude::*;
use crate::{png::{self,Error,Frames,FrameView,Still,Still16},alloc_guard::{AllocLog,sites::TryError},
    precedence as t,still_equivalence as se,still_model as s,frame_model as f,
    frame_equivalence as fe,iterator_data_model as it,icc_model as icc};
verus! {
pub fn still8(b: &[u8],Ghost(e): Ghost<Error>) -> (out: (Result<Still,TryError>,Tracked<AllocLog>))
    ensures se::G(b@,false,se::native8(out.0),out.1@.view()),
        !matches!(out.0,Err(TryError::OutOfMemory(_))) ==>
            ((out.0==Err(TryError::Format(e)))==(exists|point: t::ImagePoint| t::still_at(b@,point,e))),
        out.0!=Err(TryError::Format(Error::IccProfileTooLarge)),
{
    let out=png::try_decode_still_logged(b);
    proof {
        if let Err(TryError::OutOfMemory(o))=out.0 {t::still_oom(b@,false,o,out.1@.view());}
        else {t::still_error_iff(b@,false,se::native8(out.0),e);}
    }
    out
}
pub fn still16(b: &[u8],Ghost(e): Ghost<Error>) -> (out: (Result<Still16,TryError>,Tracked<AllocLog>))
    ensures se::G(b@,true,se::native16(out.0),out.1@.view()),
        !matches!(out.0,Err(TryError::OutOfMemory(_))) ==>
            ((out.0==Err(TryError::Format(e)))==(exists|point: t::ImagePoint| t::still_at(b@,point,e))),
        out.0!=Err(TryError::Format(Error::IccProfileTooLarge)),
{
    let out=png::try_decode_still16_logged(b);
    proof {
        if let Err(TryError::OutOfMemory(o))=out.0 {t::still_oom(b@,true,o,out.1@.view());}
        else {t::still_error_iff(b@,true,se::native16(out.0),e);}
    }
    out
}
pub fn constructor<'a>(b: &'a [u8],Ghost(e): Ghost<Error>)
    -> (out: (Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
    ensures fe::G_new(b@,out.0,png::constructor_log(&out)),
        !matches!(out.0,Err(TryError::OutOfMemory(_))) ==>
            ((out.0==Err(TryError::Format(e)))==(exists|point: t::ImagePoint| t::constructor_at(b@,point,e))),
        out.0!=Err(TryError::Format(Error::IccProfileTooLarge)),
{
    let out=Frames::try_new_logged(b);
    proof {
        if let Err(TryError::OutOfMemory(o))=out.0 {t::constructor_oom::<Frames<'a>>(b@,o,png::constructor_log(&out));}
        else {t::constructor_error_iff(b@,out.0,e);}
    }
    out
}
pub fn iterator<'a,'b>(frames: &'b mut Frames<'a>,Ghost(e): Ghost<Error>,Ghost(n): Ghost<nat>)
    -> (out: Result<Option<FrameView<'b>>,TryError>)
    requires old(frames).strict_iterator_wf(),
    ensures final(frames).strict_iterator_wf(),
        out!=Err(TryError::Format(Error::IccProfileTooLarge)),
        (matches!(old(frames).history_view().phase,it::Phase::Active) && !matches!(out,Err(TryError::OutOfMemory(_)))) ==>
            ((out==Err(TryError::Format(e)))==
                (it::frame_data(old(frames).model_input(),old(frames).model_document(),crate::policy::STRICT,old(frames).history_view().returned)==Some(Err(e)))),
        out==Err(TryError::Format(e)) ==> exists|point: crate::precedence_model::DataPoint|
            t::frame_at(old(frames).model_input(),old(frames).model_document(),old(frames).history_view().returned,point,e),
        !matches!(old(frames).history_view().phase,it::Phase::Active) ==>
            matches!(out,Ok(None)) && final(frames).log_view()==old(frames).log_view()
            && crate::frame_activity::idle(final(frames).last_work()),
        matches!(out,Err(TryError::OutOfMemory(_))) ==>
            (it::frame_data(old(frames).model_input(),old(frames).model_document(),crate::policy::STRICT,n)==Some(Err(e))
                ==> old(frames).history_view().returned<=n),
{
    proof {frames.strict_iterator_context();frames.precedence_context();frames.fused_state();}
    let ghost b=frames.model_input();let ghost doc=frames.model_document();let ghost h=frames.history_view();
    let out=frames.try_next();
    proof {
        fe::observe_call(b,doc,h,fe::native_call(out));
        if matches!(h.phase,it::Phase::Active) {
            if let Err(TryError::OutOfMemory(o))=out {t::saved_oom(b,doc,h,o,n,e);}
            else {t::iterator_error(b,doc,h,fe::native_call(out),e);}
        }
    }
    out
}
pub fn profile(b: &[u8],Ghost(e): Ghost<Error>)
    -> (out: (Result<Option<alloc::vec::Vec<u8>>,TryError>,Tracked<AllocLog>))
    ensures icc::G(b@,icc::result_view(out.0),out.1@.view()),
        !matches!(out.0,Err(TryError::OutOfMemory(_))) ==>
            ((out.0==Err(TryError::Format(e)))==(exists|point: t::IccPoint| t::icc_at(b@,point,e))),
{
    let out=png::try_icc_profile_logged(b);
    proof {
        if let Err(TryError::OutOfMemory(o))=out.0 {t::icc_oom(b@,o,out.1@.view());}
        else {t::icc_error_iff(b@,icc::result_view(out.0),out.1@.view(),e);}
    }
    out
}
}
