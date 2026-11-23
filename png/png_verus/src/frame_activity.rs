//! Erased observations at actual iterator work-entry sites.
use vstd::prelude::*;
verus! {
/// These observations describe entering a work path, not its output or log.
pub enum Site {Select,ReserveSaved,DisposeSave,Decode,NormalizeReused,Resume}
pub struct Work {pub select: nat,pub reserve: nat,pub dispose: nat,pub decode: nat,pub normalize: nat,pub resume: nat}
pub open spec fn zero() -> Work {Work {select:0,reserve:0,dispose:0,decode:0,normalize:0,resume:0}}
pub open spec fn recorded(w: Work,site: Site) -> Work {
    match site {
        Site::Select=>Work {select:w.select+1,..w},
        Site::ReserveSaved=>Work {reserve:w.reserve+1,..w},
        Site::DisposeSave=>Work {dispose:w.dispose+1,..w},
        Site::Decode=>Work {decode:w.decode+1,..w},
        Site::NormalizeReused=>Work {normalize:w.normalize+1,..w},
        Site::Resume=>Work {resume:w.resume+1,..w},
    }
}
/// Only this module can create a trace or change its counters; each recording
/// increments one counter. The iterator threads one trace through each call.
pub tracked struct Trace {ghost work: Work}
impl Trace {
    pub closed spec fn view(&self) -> Work {self.work}
    pub proof fn new() -> (tracked r: Self)
        ensures r.view()==zero(),
    {Self {work:zero()}}
    pub(crate) proof fn record(tracked &mut self,site: Site)
        ensures final(self).view()==recorded(old(self).view(),site),
    {self.work=recorded(self.work,site);}
}
pub open spec fn idle(work: Work) -> bool {work==zero()}
pub proof fn idle_excludes_work(work: Work)
    requires idle(work),
    ensures work.select==0,work.reserve==0,work.dispose==0,
        work.decode==0,work.normalize==0,work.resume==0,
{}
}
